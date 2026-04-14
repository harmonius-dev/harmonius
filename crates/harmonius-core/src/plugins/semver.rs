//! Semantic version tuple used by descriptors and capabilities.

/// Major/minor/patch triple for compatibility checks.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct SemVer {
    /// Breaking compatibility boundary.
    pub major: u32,
    /// Additive feature boundary.
    pub minor: u32,
    /// Patch level.
    pub patch: u32,
}

impl SemVer {
    /// Constructs a version triple.
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}
