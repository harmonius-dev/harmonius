/// Errors returned by [`crate::DefinitionAsset::bind`](super::DefinitionAsset).
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BindError {
    /// ECS prerequisite missing.
    MissingComponent(&'static str),
    /// Handle was revoked or never issued.
    InvalidHandle,
    /// Handle version does not match the definition asset version.
    VersionMismatch {
        /// Expected version from the definition.
        expected: u32,
        /// Version carried by the handle.
        actual: u32,
    },
    /// Dependency resolution failed (placeholder for asset pipeline).
    ResolutionFailed,
}
