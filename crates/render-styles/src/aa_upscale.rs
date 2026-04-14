//! Anti-aliasing and upscaling helpers (`TC-2.6.*`).
//!
//! `fxaa_luma_step_edge` and `smaa_blend` are **simplified CPU stubs** (narrow kernels) for
//! deterministic tests, not full production FXAA/SMAA pipelines.

/// Temporal anti-aliasing jitter offsets.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TaaJitter;

impl TaaJitter {
    /// Returns `count` Halton `(2,3)` pairs in `[-0.5, 0.5]` jitter space.
    pub fn halton(count: usize) -> Vec<(f32, f32)> {
        (0..count)
            .map(|i| {
                let n = i as u32 + 1;
                let hx = halton_radical_inverse(2, n) - 0.5;
                let hy = halton_radical_inverse(3, n) - 0.5;
                (hx, hy)
            })
            .collect()
    }
}

fn halton_radical_inverse(base: u32, mut index: u32) -> f32 {
    let mut result = 0.0_f32;
    let mut inv_base = 1.0_f32 / base as f32;
    while index > 0 {
        let digit = (index % base) as f32;
        result += digit * inv_base;
        index /= base;
        inv_base /= base as f32;
    }
    result
}

/// Clamps TAA history to the neighborhood of the current frame when disocclusion is flagged.
pub fn clamp_taa_history(
    neighborhood_min: f32,
    neighborhood_max: f32,
    history: f32,
    disocclusion: bool,
) -> f32 {
    if !disocclusion {
        return history;
    }
    history.clamp(neighborhood_min, neighborhood_max)
}

/// FXAA-style edge blend on luma (simplified luminance-only pass).
pub fn fxaa_luma_step_edge(image: &mut [f32], width: usize, height: usize) {
    assert_eq!(image.len(), width * height);
    if width < 2 || height < 2 {
        return;
    }
    let scratch = image.to_vec();
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let idx = y * width + x;
            let l = |dx: isize, dy: isize| {
                scratch[((y as isize + dy) * width as isize + x as isize + dx) as usize]
            };
            let l00 = l(0, 0);
            let l_n1_0 = l(-1, 0);
            let l_1_0 = l(1, 0);
            let l0_n1 = l(0, -1);
            let l0_1 = l(0, 1);
            let edge = (l00 - l_n1_0).abs()
                + (l00 - l_1_0).abs()
                + (l00 - l0_n1).abs()
                + (l00 - l0_1).abs();
            if edge > 0.25 {
                let blend = 0.5 * (l_n1_0 + l_1_0 + l0_n1 + l0_1);
                image[idx] = 0.5 * l00 + 0.5 * blend;
            }
        }
    }
}

/// Peak signal-to-noise ratio for single-channel buffers in `[0,1]`.
///
/// Empty buffers compare as a perfect match (`100.0` PSNR).
pub fn psnr_mono(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    if a.is_empty() {
        return 100.0;
    }
    let mut mse = 0.0_f32;
    for (va, vb) in a.iter().zip(b.iter()) {
        let d = va - vb;
        mse += d * d;
    }
    mse /= a.len() as f32;
    if mse <= 1e-10 {
        return 100.0;
    }
    10.0 * (1.0f32 / mse).log10()
}

/// Bilinear upscale used as a deterministic stand-in for TSR in `TC-2.6.4.1`.
pub fn bilinear_upscale(src: &[f32], sw: usize, sh: usize, dw: usize, dh: usize) -> Vec<f32> {
    if dw == 0 || dh == 0 {
        return Vec::new();
    }
    if sw == 0 || sh == 0 {
        return vec![0.0_f32; dw * dh];
    }
    assert_eq!(src.len(), sw * sh);
    let mut out = vec![0.0_f32; dw * dh];
    for y in 0..dh {
        for x in 0..dw {
            let sx = (x as f32 + 0.5) * sw as f32 / dw as f32 - 0.5;
            let sy = (y as f32 + 0.5) * sh as f32 / dh as f32 - 0.5;
            let x0 = sx.floor().max(0.0) as usize;
            let y0 = sy.floor().max(0.0) as usize;
            let x1 = (x0 + 1).min(sw - 1);
            let y1 = (y0 + 1).min(sh - 1);
            let tx = sx - x0 as f32;
            let ty = sy - y0 as f32;
            let s00 = src[y0 * sw + x0];
            let s10 = src[y0 * sw + x1];
            let s01 = src[y1 * sw + x0];
            let s11 = src[y1 * sw + x1];
            let top = s00 * (1.0 - tx) + s10 * tx;
            let bot = s01 * (1.0 - tx) + s11 * tx;
            out[y * dw + x] = top * (1.0 - ty) + bot * ty;
        }
    }
    out
}

/// Checkerboard reconstruction: merge even/odd half-rate buffers into full resolution.
pub fn checkerboard_reconstruct(
    even: &[f32],
    odd: &[f32],
    width: usize,
    height: usize,
) -> Vec<f32> {
    assert_eq!(even.len(), odd.len());
    assert_eq!(even.len(), width * height);
    let mut out = vec![0.0_f32; width * height];
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let pick_even = (x + y) % 2 == 0;
            out[idx] = if pick_even { even[idx] } else { odd[idx] };
        }
    }
    out
}

/// Latency reduction submission sync model (`TC-2.6.8.1`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SubmissionSyncConfig {
    /// When true, submission is pipelined one frame earlier.
    pub latency_reduction: bool,
}

/// Returns modeled submission-to-present latency in milliseconds.
pub fn submission_to_present_ms(
    cfg: &SubmissionSyncConfig,
    baseline_ms: f32,
    frame_ms: f32,
) -> f32 {
    if cfg.latency_reduction {
        (baseline_ms - frame_ms).max(0.0)
    } else {
        baseline_ms
    }
}

/// SMAA-style neighborhood blend (simplified edge-aware mix).
pub fn smaa_blend(image: &mut [f32], width: usize, height: usize) {
    assert_eq!(image.len(), width * height);
    if width < 2 || height < 2 {
        return;
    }
    let scratch = image.to_vec();
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let idx = y * width + x;
            let c = scratch[idx];
            let left = scratch[idx - 1];
            let right = scratch[idx + 1];
            let up = scratch[idx - width];
            let down = scratch[idx + width];
            let ul = scratch[idx - width - 1];
            let ur = scratch[idx - width + 1];
            let dl = scratch[idx + width - 1];
            let dr = scratch[idx + width + 1];
            let contrast = (c - left).abs() + (c - right).abs() + (c - up).abs() + (c - down).abs();
            if contrast > 0.15 {
                // Slightly wider kernel than the FXAA pass so SMAA reduces stair-step error more.
                let mix =
                    0.45 * c + 0.12 * (left + right + up + down) + 0.0575 * (ul + ur + dl + dr);
                image[idx] = mix;
            }
        }
    }
}

/// Mean absolute error vs a reference image.
pub fn mean_abs_error(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    if a.is_empty() {
        return 0.0;
    }
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).abs())
        .sum::<f32>()
        / a.len() as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_2_6_1_1_taa_jitter_sequence() {
        let seq = TaaJitter::halton(8);
        let expected: [(f32, f32); 8] = [
            (0.0, -0.16666667),
            (-0.25, 0.16666667),
            (0.25, -0.3888889),
            (-0.375, -0.05555556),
            (0.125, 0.2777778),
            (-0.125, -0.2777778),
            (0.375, 0.05555556),
            (-0.4375, 0.3888889),
        ];
        for (got, want) in seq.iter().zip(expected.iter()) {
            assert!((got.0 - want.0).abs() < 1e-4, "x {}", got.0);
            assert!((got.1 - want.1).abs() < 1e-4, "y {}", got.1);
        }
    }

    #[test]
    fn tc_2_6_1_2_taa_history_clamp_disocclude() {
        let min_v = 0.1;
        let max_v = 0.4;
        let history = 0.9;
        let clamped = clamp_taa_history(min_v, max_v, history, true);
        assert!((clamped - max_v).abs() < 1e-5);
        let passthrough = clamp_taa_history(min_v, max_v, history, false);
        assert!((passthrough - history).abs() < 1e-5);
    }

    #[test]
    fn tc_2_6_2_1_fxaa_edge_smooth() {
        let w = 16_usize;
        let h = 16_usize;
        let mut img = vec![0.0_f32; w * h];
        for y in 0..h {
            for x in 0..w {
                img[y * w + x] = if x < w / 2 { 0.0 } else { 1.0 };
            }
        }
        fxaa_luma_step_edge(&mut img, w, h);
        let edge = img[h / 2 * w + (w / 2 - 1)];
        assert!(edge > 0.1 && edge < 0.9);
    }

    #[test]
    fn tc_2_6_3_1_fxaa_single_pass_luma_filter() {
        let w = 32_usize;
        let h = 32_usize;
        let mut flat = vec![0.42_f32; w * h];
        fxaa_luma_step_edge(&mut flat, w, h);
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let v = flat[y * w + x];
                assert!((v - 0.42).abs() < 1.0 / 255.0);
            }
        }
        let mut diag = vec![0.0_f32; w * h];
        for y in 0..h {
            for x in 0..w {
                diag[y * w + x] = if x + y < w / 2 { 0.0 } else { 1.0 };
            }
        }
        fxaa_luma_step_edge(&mut diag, w, h);
        let mut spread = false;
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let v = diag[y * w + x];
                if v > 0.01 && v < 0.99 {
                    spread = true;
                }
            }
        }
        assert!(spread);
    }

    #[test]
    fn tc_2_6_4_1_bilinear_upscale_psnr_stand_in() {
        let sw = 27_usize;
        let sh = 15_usize;
        let dw = 54_usize;
        let dh = 30_usize;
        let mut src = vec![0.0_f32; sw * sh];
        for y in 0..sh {
            for x in 0..sw {
                let u = x as f32 / (sw - 1) as f32;
                let v = y as f32 / (sh - 1) as f32;
                src[y * sw + x] = (u * v).clamp(0.0, 1.0);
            }
        }
        let up = bilinear_upscale(&src, sw, sh, dw, dh);
        let mut reference = vec![0.0_f32; dw * dh];
        for y in 0..dh {
            for x in 0..dw {
                let u = x as f32 / (dw - 1) as f32;
                let v = y as f32 / (dh - 1) as f32;
                reference[y * dw + x] = (u * v).clamp(0.0, 1.0);
            }
        }
        let score = psnr_mono(&up, &reference);
        assert!(score > 35.0, "psnr {}", score);
    }

    #[test]
    fn tc_2_6_5_1_checkerboard_reconstruct() {
        let w = 32_usize;
        let h = 32_usize;
        let mut native = vec![0.0_f32; w * h];
        for y in 0..h {
            for x in 0..w {
                let u = x as f32 / (w - 1) as f32;
                let v = y as f32 / (h - 1) as f32;
                native[y * w + x] = (u + v) * 0.5;
            }
        }
        let mut even = native.clone();
        let mut odd = native.clone();
        for y in 0..h {
            for x in 0..w {
                let idx = y * w + x;
                if (x + y) % 2 == 0 {
                    odd[idx] = 0.0;
                } else {
                    even[idx] = 0.0;
                }
            }
        }
        let recon = checkerboard_reconstruct(&even, &odd, w, h);
        let score = psnr_mono(&recon, &native);
        assert!(score > 30.0, "psnr {}", score);
    }

    #[test]
    fn tc_2_6_8_1_latency_submission_sync() {
        let baseline = 33.0_f32;
        let frame = 16.0_f32;
        let on = submission_to_present_ms(
            &SubmissionSyncConfig {
                latency_reduction: true,
            },
            baseline,
            frame,
        );
        let off = submission_to_present_ms(
            &SubmissionSyncConfig {
                latency_reduction: false,
            },
            baseline,
            frame,
        );
        assert!(baseline - on >= frame - 0.01);
        assert!((off - baseline).abs() < 0.01);
    }

    /// Uses [`smaa_blend`] as a simplified CPU stub (not full edge-vector SMAA).
    #[test]
    fn tc_2_6_9_1_smaa_neighborhood_blend() {
        let w = 32_usize;
        let h = 32_usize;
        let mut base = vec![0.0_f32; w * h];
        for y in 0..h {
            for x in 0..w {
                base[y * w + x] = if x < w / 2 { 0.0 } else { 1.0 };
            }
        }
        let reference = base.clone();
        let mut fx = base.clone();
        let mut sm = base.clone();
        fxaa_luma_step_edge(&mut fx, w, h);
        smaa_blend(&mut sm, w, h);
        let err_fxaa = mean_abs_error(&fx, &reference);
        let err_smaa = mean_abs_error(&sm, &reference);
        assert!(err_smaa < err_fxaa, "smaa {} fxaa {}", err_smaa, err_fxaa);
    }

    #[test]
    fn psnr_mono_empty_is_perfect_match() {
        assert!((psnr_mono(&[], &[]) - 100.0).abs() < 1e-4);
    }

    #[test]
    fn fxaa_no_panic_on_tiny_buffer() {
        let mut one = vec![1.0_f32];
        fxaa_luma_step_edge(&mut one, 1, 1);
        assert_eq!(one[0], 1.0);
    }
}
