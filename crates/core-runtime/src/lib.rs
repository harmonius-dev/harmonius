#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

//! Core runtime primitives shared across Harmonius subsystems.

pub mod error;
pub mod ids;

pub use error::{
    AssetError, CodegenError, CompileError, Diagnostic, EngineError, IoError, NetworkError,
    PlatformError, SerializationError, Severity, SourceSpan, ToEngineError, ValidationError,
};
pub use ids::SymbolId;
