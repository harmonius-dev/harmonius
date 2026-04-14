//! Summing mixer for many simultaneous mono voice streams (post-decode PCM).

/// Mixes equal-length mono `f32` PCM slices into `out` by summing samples.
///
/// All inputs must match `out.len()` (one 20 ms frame at 48 kHz is typically 960 samples). This
/// supports the documented cap of 32 simultaneous voice streams on the voice bus.
///
/// # Panics
///
/// Panics if any `pcm` slice length differs from `out.len()`.
pub fn mix_voice_streams<'a>(streams: impl IntoIterator<Item = &'a [f32]>, out: &mut [f32]) {
    out.fill(0.0);
    for pcm in streams {
        assert_eq!(
            pcm.len(),
            out.len(),
            "networking_audio_integration: mixed streams must share frame length"
        );
        for (dst, &s) in out.iter_mut().zip(pcm.iter()) {
            *dst += s;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sine_frame(phase: usize, len: usize) -> Vec<f32> {
        (0..len)
            .map(|n| {
                let t = (n + phase) as f32 / 48_000.0;
                (2.0 * std::f32::consts::PI * 440.0 * t).sin() * 0.01
            })
            .collect()
    }

    /// TC-IR-4.3.4.B1 (behavioral slice) — 32 streams summed without non-finite values.
    #[test]
    fn tc_ir_4_3_4_b1_thirty_two_voice_streams_mix() {
        const FRAME: usize = 960;
        let streams: Vec<Vec<f32>> = (0..32).map(|i| sine_frame(i, FRAME)).collect();
        let refs: Vec<&[f32]> = streams.iter().map(Vec::as_slice).collect();
        let mut out = vec![0.0f32; FRAME];
        mix_voice_streams(refs, &mut out);
        assert!(out.iter().all(|x| x.is_finite()));
        let peak = out.iter().map(|x| x.abs()).fold(0.0f32, f32::max);
        assert!(peak > 0.01 && peak < 1.0e6);
    }
}
