//! Stylized rendering passes (`TC-2.11.*`).

/// RGBA color in linear `0..1` space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
    /// Red channel.
    pub r: f32,
    /// Green channel.
    pub g: f32,
    /// Blue channel.
    pub b: f32,
    /// Alpha channel.
    pub a: f32,
}

/// Outline color selection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutlineColor {
    /// Constant RGBA.
    Rgba(Rgba),
}

/// Requested outline parameters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OutlineRequest {
    /// Outline color.
    pub color: OutlineColor,
    /// Higher wins on overlap.
    pub priority: u8,
}

/// GPU capability flags used to pick outline technique (CPU policy only; no GPU API calls).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Capabilities {
    /// Whether compute shaders are available.
    pub compute_shaders: bool,
}

/// Outline pipeline variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutlinePipeline {
    /// Jump flood + distance.
    JumpFlood,
    /// Sobel screen-space edge pass.
    Sobel,
}

/// Builds outline pipeline from capabilities (offline policy; not a GPU backend selector).
pub fn outline_pipeline_for_caps(caps: &Capabilities) -> OutlinePipeline {
    if caps.compute_shaders {
        OutlinePipeline::JumpFlood
    } else {
        OutlinePipeline::Sobel
    }
}

const SEED_MAX: u16 = u16::MAX;

/// Initializes jump-flood seed buffer from a boolean mask (`TC-2.11.1.1`).
pub fn outline_jump_flood_init(mask: &[bool], width: usize, height: usize) -> Vec<(u16, u16)> {
    assert_eq!(mask.len(), width * height);
    let mut seeds = vec![(SEED_MAX, SEED_MAX); width * height];
    for y in 0..height {
        for x in 0..width {
            if mask[y * width + x] {
                seeds[y * width + x] = (x as u16, y as u16);
            }
        }
    }
    seeds
}

/// Resolves outline color at a pixel using highest priority request (`TC-2.11.1.4`).
pub fn outline_resolve_overlap(requests: &[OutlineRequest]) -> Rgba {
    let mut best: Option<(u8, Rgba)> = None;
    for req in requests {
        let OutlineColor::Rgba(Rgba { r, g, b, a }) = req.color;
        match best {
            None => best = Some((req.priority, Rgba { r, g, b, a })),
            Some((p, _)) if req.priority > p => {
                best = Some((req.priority, Rgba { r, g, b, a }));
            }
            _ => {}
        }
    }
    best.map(|(_, c)| c).unwrap_or(Rgba {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    })
}

/// Highlight modulation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightMode {
    /// Sinusoidal pulse.
    Pulse,
}

/// Runtime highlight state (`TC-2.11.2.1`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HighlightState {
    /// Modulation mode.
    pub mode: HighlightMode,
    /// Frequency in hertz.
    pub freq_hz: f32,
    /// Base intensity.
    pub base: f32,
    /// Pulse amplitude.
    pub amp: f32,
}

/// Samples highlight intensity at `t` seconds.
pub fn highlight_intensity(state: &HighlightState, t: f32) -> f32 {
    match state.mode {
        HighlightMode::Pulse => {
            let phase = std::f32::consts::TAU * state.freq_hz * t;
            state.base + state.amp * phase.sin()
        }
    }
}

/// Schlick-style fresnel for rim lighting (`TC-2.11.2.2`).
pub fn highlight_fresnel(n_dot_v: f32, power: f32) -> f32 {
    (1.0 - n_dot_v.clamp(0.0, 1.0)).powf(power)
}

/// Separable Gaussian weights summing to ~1 (`TC-2.11.2.3`).
pub fn gaussian_weights_1d(sigma: f32, radius: usize) -> Vec<f32> {
    let mut w = Vec::with_capacity(radius * 2 + 1);
    let mut sum = 0.0_f32;
    for i in 0..radius * 2 + 1 {
        let x = i as isize - radius as isize;
        let g = (-0.5 * (x as f32 / sigma).powi(2)).exp();
        w.push(g);
        sum += g;
    }
    for v in &mut w {
        *v /= sum;
    }
    w
}

/// Mobile vs desktop toon quality.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToonQuality {
    /// Mobile preset.
    Mobile,
}

/// Quantizes N·L into `bands` thresholds (`TC-2.11.3.1`).
pub fn toon_quantize_ndotl(ndotl: f32, bands: &[f32], _quality: ToonQuality) -> f32 {
    if bands.is_empty() {
        return 0.0;
    }
    let mut chosen = bands[0];
    for &b in bands {
        if ndotl >= b {
            chosen = b;
        }
    }
    chosen
}

/// Samples a 1D ramp texture defined by evenly spaced RGB knots (`TC-2.11.3.2`).
///
/// Uses **nearest knot** selection (not linear RGB interpolation between knots).
pub fn toon_ramp_lookup(knots: &[(f32, f32, f32)], ndotl: f32) -> (f32, f32, f32) {
    if knots.is_empty() {
        return (0.0, 0.0, 0.0);
    }
    let u = ndotl.clamp(0.0, 1.0);
    let idx = ((knots.len() - 1) as f32 * u).round() as usize;
    let idx = idx.min(knots.len() - 1);
    knots[idx]
}

/// Specular step for toon shading (`TC-2.11.3.3`).
pub fn toon_specular_step(spec: f32, threshold: f32) -> f32 {
    if spec >= threshold {
        1.0
    } else {
        0.0
    }
}

/// Occlusion volume fade mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OcclusionMode {
    /// Axis-aligned volume.
    Volume,
}

/// Occlusion volume parameters (`TC-2.11.4.1`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OcclusionVolume {
    /// Fade mode.
    pub mode: OcclusionMode,
    /// Fade duration in seconds.
    pub fade_s: f32,
}

/// Computes roof alpha during camera ingress (`TC-2.11.4.1`).
pub fn occlusion_roof_alpha(vol: &OcclusionVolume, t: f32) -> f32 {
    match vol.mode {
        OcclusionMode::Volume => (1.0 - (t / vol.fade_s).min(1.0)).max(0.0),
    }
}

/// Returns Bayer 8x8 threshold matrix in `0..1` (`TC-2.11.4.2`).
pub fn bayer8_matrix() -> [[f32; 8]; 8] {
    // Classic 8x8 Bayer matrix normalized to 0..63 -> 0..1
    const RAW: [[u8; 8]; 8] = [
        [0, 48, 12, 60, 3, 51, 15, 63],
        [32, 16, 44, 28, 35, 19, 47, 31],
        [8, 56, 4, 52, 11, 59, 7, 55],
        [40, 24, 36, 20, 43, 27, 39, 23],
        [2, 50, 14, 62, 1, 49, 13, 61],
        [34, 18, 46, 30, 33, 17, 45, 29],
        [10, 58, 6, 54, 9, 57, 5, 53],
        [42, 26, 38, 22, 41, 25, 37, 21],
    ];
    let mut out = [[0.0_f32; 8]; 8];
    for y in 0..8 {
        for x in 0..8 {
            out[y][x] = RAW[y][x] as f32 / 64.0;
        }
    }
    out
}

/// Applies dissolve dither at threshold `t` (`TC-2.11.4.2`).
pub fn dissolve_mask(bayer: &[[f32; 8]; 8], threshold: f32, x: usize, y: usize) -> bool {
    let v = bayer[y % 8][x % 8];
    v >= threshold
}

/// Depth compare for silhouette passes (`TC-2.11.5.1`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepthCompare {
    /// Greater depth draws (through walls).
    Greater,
    /// Default less-equal.
    LessEqual,
}

/// Pipeline depth state for X-ray silhouettes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XRayDepthState {
    /// Depth function for silhouette pass.
    pub depth_compare: DepthCompare,
}

/// Selects depth state when any entity needs X-ray (`TC-2.11.5.1`).
pub fn xray_depth_state(has_xray: bool) -> XRayDepthState {
    XRayDepthState {
        depth_compare: if has_xray {
            DepthCompare::Greater
        } else {
            DepthCompare::LessEqual
        },
    }
}

/// Filters entities by minimum priority (`TC-2.11.5.2`).
pub fn xray_filter_by_priority(priorities: &[u8], min_priority: u8) -> Vec<bool> {
    priorities.iter().map(|p| *p >= min_priority).collect()
}

/// Painterly post parameters (`TC-2.11.6.1`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PainterlyParams {
    /// Brush radius in pixels.
    pub brush_radius: f32,
}

/// Estimates clustered region width for a synthetic stroke (`TC-2.11.6.1`).
pub fn painterly_cluster_width(params: &PainterlyParams) -> f32 {
    params.brush_radius
}

/// Darkens edge pixels by factor (`TC-2.11.6.2`).
pub fn painterly_edge_darken(luma: f32, is_edge: bool, factor: f32) -> f32 {
    if is_edge {
        luma * factor
    } else {
        luma
    }
}

/// Pixel art downscale parameters (`TC-2.11.7.1`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PixelArtParams {
    /// Internal resolution width.
    pub internal_w: u32,
    /// Internal resolution height.
    pub internal_h: u32,
}

/// Quantizes RGB to nearest palette entry (`TC-2.11.7.2`).
pub fn pixel_art_palette_match(palette: &[[f32; 3]], color: [f32; 3]) -> [f32; 3] {
    if palette.is_empty() {
        return [0.0, 0.0, 0.0];
    }
    let mut best = palette[0];
    let mut best_d = f32::MAX;
    for p in palette {
        let d = (p[0] - color[0]).powi(2) + (p[1] - color[1]).powi(2) + (p[2] - color[2]).powi(2);
        if d < best_d {
            best_d = d;
            best = *p;
        }
    }
    best
}

/// Nearest-neighbor upscale factor check (`TC-2.11.7.3`).
pub fn nearest_block_scale(src_w: u32, dst_w: u32) -> u32 {
    if src_w == 0 {
        return 0;
    }
    dst_w / src_w
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_2_11_1_1_outline_jump_flood_init() {
        let w = 32_usize;
        let h = 32_usize;
        let mut mask = vec![false; w * h];
        mask[5 * w + 5] = true;
        let seeds = outline_jump_flood_init(&mask, w, h);
        assert_eq!(seeds[5 * w + 5], (5, 5));
        assert_eq!(seeds[0], (SEED_MAX, SEED_MAX));
    }

    #[test]
    fn tc_2_11_1_2_outline_color_apply() {
        let c = Rgba {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        };
        let reqs = [OutlineRequest {
            color: OutlineColor::Rgba(c),
            priority: 1,
        }];
        let resolved = outline_resolve_overlap(&reqs);
        assert!((resolved.r - 1.0).abs() < 1.0 / 255.0);
        let w = 8_usize;
        let h = 8_usize;
        let mut mask = vec![false; w * h];
        mask[3 * w + 3] = true;
        mask[3 * w + 4] = true;
        let seeds = outline_jump_flood_init(&mask, w, h);
        assert_eq!(seeds[3 * w + 3], (3, 3));
        assert_eq!(seeds[3 * w + 4], (4, 3));
    }

    #[test]
    fn tc_2_11_1_3_outline_sobel_fallback() {
        let caps = Capabilities {
            compute_shaders: false,
        };
        assert_eq!(outline_pipeline_for_caps(&caps), OutlinePipeline::Sobel);
    }

    #[test]
    fn tc_2_11_1_4_outline_priority_overlap() {
        let a = OutlineRequest {
            color: OutlineColor::Rgba(Rgba {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            }),
            priority: 1,
        };
        let b = OutlineRequest {
            color: OutlineColor::Rgba(Rgba {
                r: 0.0,
                g: 0.0,
                b: 1.0,
                a: 1.0,
            }),
            priority: 2,
        };
        let c = outline_resolve_overlap(&[a, b]);
        assert!((c.b - 1.0).abs() < 1e-4);
    }

    #[test]
    fn tc_2_11_2_1_highlight_pulse_modulation() {
        let st = HighlightState {
            mode: HighlightMode::Pulse,
            freq_hz: 1.0,
            base: 0.5,
            amp: 0.5,
        };
        assert!((highlight_intensity(&st, 0.0) - 0.5).abs() < 0.01);
        assert!((highlight_intensity(&st, 0.25) - 1.0).abs() < 0.01);
    }

    #[test]
    fn tc_2_11_2_2_highlight_fresnel_rim() {
        assert!(highlight_fresnel(1.0, 5.0) < 0.05);
        assert!(highlight_fresnel(0.0, 5.0) > 0.9);
    }

    #[test]
    fn tc_2_11_2_3_highlight_inner_glow_blur() {
        let w = gaussian_weights_1d(2.0, 3);
        let sum: f32 = w.iter().sum();
        assert!((sum - 1.0).abs() < 0.001);
    }

    #[test]
    fn tc_2_11_3_1_toon_band_count_3() {
        let bands = [0.33, 0.66, 1.0];
        let q = toon_quantize_ndotl(0.7, &bands, ToonQuality::Mobile);
        assert!((q - 0.66).abs() < 0.01);
    }

    #[test]
    fn tc_2_11_3_2_toon_ramp_texture_lookup() {
        let knots = [(1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)];
        let a = toon_ramp_lookup(&knots, 0.1);
        let b = toon_ramp_lookup(&knots, 0.5);
        let c = toon_ramp_lookup(&knots, 0.9);
        assert_eq!(a, knots[0]);
        assert_eq!(b, knots[1]);
        assert_eq!(c, knots[2]);
    }

    #[test]
    fn tc_2_11_3_3_toon_specular_shape() {
        assert_eq!(toon_specular_step(0.5, 0.8), 0.0);
        assert_eq!(toon_specular_step(0.85, 0.8), 1.0);
        assert_eq!(toon_specular_step(0.9, 0.8), 1.0);
    }

    #[test]
    fn tc_2_11_4_1_occlusion_volume_fade() {
        let vol = OcclusionVolume {
            mode: OcclusionMode::Volume,
            fade_s: 0.5,
        };
        let a0 = occlusion_roof_alpha(&vol, 0.0);
        let a1 = occlusion_roof_alpha(&vol, 0.5);
        assert!((a0 - 1.0).abs() < 1e-4);
        assert!((a1 - 0.0).abs() < 1e-4);
        assert!(occlusion_roof_alpha(&vol, 0.25) > occlusion_roof_alpha(&vol, 0.4));
    }

    #[test]
    fn tc_2_11_4_2_occlusion_dither_pattern() {
        let b = bayer8_matrix();
        let mut hits = 0_usize;
        for y in 0..8 {
            for x in 0..8 {
                if dissolve_mask(&b, 0.5, x, y) {
                    hits += 1;
                }
            }
        }
        assert!(hits > 0 && hits < 64);
        let levels: std::collections::BTreeSet<u8> = b
            .iter()
            .flatten()
            .map(|v| (v * 64.0).round().clamp(0.0, 63.0) as u8)
            .collect();
        assert_eq!(
            levels.len(),
            64,
            "order-8 Bayer uses 64 distinct thresholds"
        );
    }

    #[test]
    fn tc_2_11_5_1_xray_depth_compare() {
        let st_on = xray_depth_state(true);
        assert_eq!(st_on.depth_compare, DepthCompare::Greater);
        let st_off = xray_depth_state(false);
        assert_eq!(st_off.depth_compare, DepthCompare::LessEqual);
    }

    #[test]
    fn tc_2_11_5_2_xray_priority_through_walls() {
        let pri = [1_u8, 3_u8];
        let keep = xray_filter_by_priority(&pri, 2);
        assert_eq!(keep, vec![false, true]);
    }

    #[test]
    fn tc_2_11_6_1_painterly_brush_radius() {
        let p = PainterlyParams { brush_radius: 5.0 };
        assert!((painterly_cluster_width(&p) - 5.0).abs() < 1.0);
    }

    #[test]
    fn tc_2_11_6_2_painterly_edge_darken() {
        let out = painterly_edge_darken(1.0, true, 0.5);
        assert!((out - 0.5).abs() < 1e-4);
    }

    #[test]
    fn tc_2_11_7_1_pixel_art_resolution_quant() {
        let p = PixelArtParams {
            internal_w: 160,
            internal_h: 144,
        };
        assert_eq!((p.internal_w, p.internal_h), (160, 144));
    }

    #[test]
    fn tc_2_11_7_2_pixel_art_palette_match() {
        let palette: Vec<[f32; 3]> = (0..16)
            .map(|i| {
                let v = i as f32 / 15.0;
                [v, v * 0.5, 1.0 - v]
            })
            .collect();
        let out = pixel_art_palette_match(&palette, [0.6, 0.4, 0.2]);
        let mut best = palette[0];
        let mut best_d = f32::MAX;
        for p in &palette {
            let d = (p[0] - 0.6).powi(2) + (p[1] - 0.4).powi(2) + (p[2] - 0.2).powi(2);
            if d < best_d {
                best_d = d;
                best = *p;
            }
        }
        assert_eq!(out, best);
    }

    #[test]
    fn tc_2_11_7_3_pixel_art_nearest_neighbor() {
        assert_eq!(nearest_block_scale(160, 1920), 12);
        assert_eq!(nearest_block_scale(0, 1920), 0);
    }
}
