//! Bone picking overlay visibility rules (TC-IR-5.3.F3).

/// Emitted when a bone index is outside the evaluated skeleton.
pub const WARN_BONE_INDEX_OUT_OF_RANGE: &str = "bone index out of range for overlay";

/// Overlay evaluation for one bone index (matches preview outcome pattern).
#[derive(Debug, PartialEq)]
pub struct BoneOverlayOutcome {
    /// Whether the overlay should draw for this bone.
    pub visible: bool,
    /// Human-readable warnings mirroring production logging text.
    pub warnings: Vec<&'static str>,
}

/// Applies TC-IR-5.3.F3 policy: skip out-of-range indices with a stable warning token.
#[must_use]
pub fn bone_overlay_outcome(bone_index: u32, bone_count: u32) -> BoneOverlayOutcome {
    if bone_index < bone_count {
        BoneOverlayOutcome {
            visible: true,
            warnings: Vec::new(),
        }
    } else {
        BoneOverlayOutcome {
            visible: false,
            warnings: vec![WARN_BONE_INDEX_OUT_OF_RANGE],
        }
    }
}

/// Returns whether a bone overlay should be drawn for `bone_index`.
#[must_use]
pub fn bone_overlay_visible(bone_index: u32, bone_count: u32) -> bool {
    bone_overlay_outcome(bone_index, bone_count).visible
}
