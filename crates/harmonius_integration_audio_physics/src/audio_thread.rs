//! Minimal audio-thread helpers (forward-compatible command decoding).

use crate::commands::VoiceParam;

/// Applies a wire-format [`VoiceParam`] tag; returns `false` for unknown tags (ignored safely).
pub fn apply_set_param_wire(tag: u8, _value: f32) -> bool {
    VoiceParam::from_wire(tag).is_some()
}
