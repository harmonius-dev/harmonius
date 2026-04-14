//! Bone picking overlay visibility rules (TC-IR-5.3.F3).

/// Emitted when a bone index is outside the evaluated skeleton.
pub const WARN_BONE_INDEX_OUT_OF_RANGE: &str = "bone index out of range for overlay";

/// Returns whether a bone overlay should be drawn for `bone_index`.
#[must_use]
pub fn bone_overlay_visible(bone_index: u32, bone_count: u32) -> bool {
    bone_index < bone_count
}
