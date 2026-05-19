//! Unified engine error hierarchy (bootstrap subset).

use thiserror::Error;

/// Top-level error type converging subsystem failures.
#[derive(Clone, Debug, Error, PartialEq)]
pub enum EngineError {
    /// Platform or windowing failure.
    #[error("platform error: {0}")]
    Platform(String),
    /// GPU or Vulkan failure.
    #[error("gpu error: {0}")]
    Gpu(String),
    /// Shader compilation or module creation failure.
    #[error("shader error: {0}")]
    Shader(String),
    /// Generic validation failure.
    #[error("validation error: {0}")]
    Validation(String),
}
