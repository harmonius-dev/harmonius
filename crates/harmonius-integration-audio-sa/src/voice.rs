//! Voice-level filtering driven by propagation results.

use crate::PropagationResult;

/// Minimum low-pass cutoff when fully occluded (Hz).
pub const MIN_VOICE_LPF_HZ: f32 = 800.0;
/// Maximum low-pass cutoff at full line-of-sight (Hz).
pub const MAX_VOICE_LPF_HZ: f32 = 20_000.0;

/// Simple one-pole low-pass state used by tests in place of a full voice graph.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VoiceFilterState {
    /// Effective low-pass cutoff in Hz.
    pub lpf_hz: f32,
}

impl VoiceFilterState {
    /// Starts at wide-open bandwidth.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            lpf_hz: MAX_VOICE_LPF_HZ,
        }
    }

    /// Applies occlusion to the cutoff: full occlusion snaps to [`MIN_VOICE_LPF_HZ`].
    pub fn apply_occlusion(&mut self, occlusion: f32) {
        self.lpf_hz = MAX_VOICE_LPF_HZ * occlusion + MIN_VOICE_LPF_HZ * (1.0 - occlusion);
    }
}

impl Default for VoiceFilterState {
    fn default() -> Self {
        Self::new()
    }
}

/// Applies propagation to a voice filter (occlusion drives low-pass).
pub fn apply_propagation_to_voice(filter: &mut VoiceFilterState, pr: &PropagationResult) {
    filter.apply_occlusion(pr.occlusion);
}
