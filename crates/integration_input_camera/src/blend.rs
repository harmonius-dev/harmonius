//! Blend state consumed by CIAC suppression.

/// Runtime blend controller state (subset for integration).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BlendState {
    /// True while `BlendSystem` is actively blending virtual cameras.
    pub is_blending: bool,
}
