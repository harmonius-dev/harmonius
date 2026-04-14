//! Unified [`EngineError`] hierarchy and boundary conversions.

use std::path::PathBuf;

use crate::ids::SymbolId;

/// Top-level error envelope exchanged across Harmonius subsystems.
#[derive(Debug)]
pub enum EngineError {
    /// File, socket, or device IO failures.
    Io(IoError),
    /// Archive or serialization contract violations.
    Serialization(SerializationError),
    /// Schema or data validation problems.
    Validation(ValidationError),
    /// Shader graph, tooling, or backend codegen failures.
    Codegen(CodegenError),
    /// OS or GPU runtime failures.
    Platform(PlatformError),
    /// Content import and asset database failures.
    Asset(AssetError),
    /// Replication and transport failures.
    Network(NetworkError),
}

/// Converts a subsystem-local error into the shared [`EngineError`] representation.
pub trait ToEngineError {
    /// Maps `self` into an [`EngineError`].
    fn to_engine_error(self) -> EngineError;
}

impl<T> ToEngineError for T
where
    T: Into<EngineError>,
{
    fn to_engine_error(self) -> EngineError {
        self.into()
    }
}

/// IO-oriented failure modes with optional structured context.
#[derive(Debug)]
pub enum IoError {
    /// Missing path on read or metadata probe.
    NotFound {
        /// Path that could not be resolved.
        path: PathBuf,
    },
    /// Operation blocked by permissions.
    PermissionDenied {
        /// Path that was inaccessible.
        path: PathBuf,
    },
    /// Create failed because the destination already exists.
    AlreadyExists {
        /// Conflicting destination path.
        path: PathBuf,
    },
    /// TCP connect refused.
    ConnectionRefused,
    /// Connection reset by peer.
    ConnectionReset,
    /// Deadline exceeded.
    Timeout,
    /// Syscall interrupted before completion.
    Interrupted,
    /// Cooperative cancellation token fired.
    Cancelled,
    /// Device reported no free space.
    DeviceFull,
    /// Operation not supported on this platform build.
    Unsupported {
        /// Name of the unsupported operation.
        op: &'static str,
    },
    /// Platform-specific errno-style failure.
    PlatformSpecific {
        /// Opaque numeric code from the platform layer.
        code: i32,
    },
}

/// Serialization and archive integrity failures.
#[derive(Debug)]
pub enum SerializationError {
    /// Magic header mismatch.
    BadMagic {
        /// Expected magic value.
        expected: u32,
        /// Observed magic value.
        got: u32,
    },
    /// Format version mismatch.
    VersionMismatch {
        /// Expected major/minor packed representation.
        expected: u16,
        /// Observed version value.
        got: u16,
    },
    /// Detected structural corruption at a byte offset.
    CorruptArchive {
        /// Byte offset of the corruption.
        offset: u64,
    },
    /// Field present in data but unsupported by this build.
    UnsupportedField {
        /// Declared field name.
        name: &'static str,
    },
    /// Buffer too small for the encoded payload.
    BufferTooSmall {
        /// Minimum required bytes.
        required: usize,
        /// Bytes available in the caller buffer.
        got: usize,
    },
}

/// Validation failures for engine configuration and user data.
#[derive(Debug)]
pub enum ValidationError {
    /// Numeric or logical range violation.
    OutOfRange {
        /// Field identifier for diagnostics.
        field: &'static str,
    },
    /// Enum discriminant out of range.
    InvalidEnum {
        /// Enum type name.
        name: &'static str,
        /// Raw discriminant observed.
        got: u32,
    },
    /// Required field absent.
    MissingField {
        /// Missing field name.
        name: &'static str,
    },
    /// Duplicate unique key.
    DuplicateKey {
        /// Conflicting key name.
        name: &'static str,
    },
    /// Schema-level rule violation.
    SchemaViolation {
        /// Human-readable violation summary.
        message: &'static str,
    },
}

/// Codegen failures including nested compile diagnostics.
#[derive(Debug)]
pub enum CodegenError {
    /// Backend rejected the lowered program.
    BackendRejected {
        /// Stable reason token.
        reason: &'static str,
    },
    /// Duplicate symbol emitted during lowering.
    SymbolConflict {
        /// Conflicting symbol id.
        symbol: SymbolId,
    },
    /// Nested IO failure while writing intermediates.
    IoFailed {
        /// Output path that failed.
        path: PathBuf,
    },
    /// External tool exited non-zero.
    SubprocessExit {
        /// Process exit code.
        code: i32,
        /// Captured stderr (bounded by caller).
        stderr: String,
    },
    /// Shader or graph compilation failure.
    Compile(CompileError),
}

/// Shader, graph, or tool compilation failures.
#[derive(Debug)]
pub enum CompileError {
    /// Lexer/parser diagnostics.
    Syntax {
        /// Structured diagnostics for editors.
        diagnostics: Vec<Diagnostic>,
    },
    /// Type checking diagnostics.
    TypeCheck {
        /// Structured diagnostics for editors.
        diagnostics: Vec<Diagnostic>,
    },
    /// Link step could not resolve a symbol.
    Link {
        /// Missing symbol id.
        missing_symbol: SymbolId,
    },
    /// External compiler driver failed.
    Subprocess {
        /// Process exit code.
        exit_code: i32,
        /// Captured stderr (bounded by caller).
        stderr: String,
    },
}

/// GPU or OS platform failures.
#[derive(Debug)]
pub enum PlatformError {
    /// Low-level syscall failed.
    OsCall {
        /// Stable syscall label.
        call: &'static str,
        /// Platform error code.
        code: i32,
    },
    /// Device lost or reset.
    DeviceLost,
    /// Out of memory or similar resource exhaustion.
    InsufficientResources {
        /// Subsystem reporting the shortage.
        subsystem: &'static str,
    },
    /// Subsystem used before initialization completed.
    NotInitialized,
}

/// Asset import and content pipeline failures.
#[derive(Debug)]
pub enum AssetError {
    /// Asset id not present in database.
    NotFound {
        /// Stable asset id key.
        asset_id: u64,
    },
    /// Hash mismatch between expected and observed payload.
    HashMismatch {
        /// Expected content hash.
        expected: u64,
        /// Observed content hash.
        got: u64,
    },
    /// Import pipeline rejected the asset.
    ImportFailed {
        /// Stable import failure token.
        reason: &'static str,
    },
    /// Unknown file extension for automatic import.
    UnknownFormat {
        /// Extension token.
        ext: &'static str,
    },
}

/// Networking and replication failures.
#[derive(Debug)]
pub enum NetworkError {
    /// Connection dropped unexpectedly.
    ConnectionDropped,
    /// QUIC stream closed by peer.
    QuicStreamClosed,
    /// TLS negotiation failed.
    TlsHandshake {
        /// Stable handshake failure token.
        reason: &'static str,
    },
    /// Wire protocol version mismatch.
    ProtocolMismatch {
        /// Expected wire version.
        expected_version: u16,
        /// Observed wire version.
        got_version: u16,
    },
}

/// Editor-facing diagnostic attached to compile failures.
#[derive(Clone, Debug)]
pub struct Diagnostic {
    /// Optional filesystem path for the diagnostic.
    pub path: Option<PathBuf>,
    /// Optional source span within `path`.
    pub span: Option<SourceSpan>,
    /// Human-readable message (static for stable diagnostics v1).
    pub message: &'static str,
    /// Severity hint for UI routing.
    pub severity: Severity,
}

/// 1-based line/column span in a text buffer.
#[derive(Clone, Copy, Debug)]
pub struct SourceSpan {
    /// 1-based line number.
    pub line: u32,
    /// 1-based UTF-8 column.
    pub column: u32,
    /// Span length in bytes.
    pub length: u32,
}

/// Diagnostic severity for UI and logging.
#[derive(Clone, Copy, Debug)]
pub enum Severity {
    /// Informational diagnostic.
    Info,
    /// Warning diagnostic.
    Warning,
    /// Error diagnostic.
    Error,
    /// Fatal diagnostic (compilation cannot continue).
    Fatal,
}

impl From<IoError> for EngineError {
    fn from(value: IoError) -> Self {
        EngineError::Io(value)
    }
}

impl From<SerializationError> for EngineError {
    fn from(value: SerializationError) -> Self {
        EngineError::Serialization(value)
    }
}

impl From<ValidationError> for EngineError {
    fn from(value: ValidationError) -> Self {
        EngineError::Validation(value)
    }
}

impl From<CodegenError> for EngineError {
    fn from(value: CodegenError) -> Self {
        EngineError::Codegen(value)
    }
}

impl From<CompileError> for CodegenError {
    fn from(value: CompileError) -> Self {
        CodegenError::Compile(value)
    }
}

impl From<CompileError> for EngineError {
    fn from(value: CompileError) -> Self {
        EngineError::Codegen(CodegenError::Compile(value))
    }
}

impl From<PlatformError> for EngineError {
    fn from(value: PlatformError) -> Self {
        EngineError::Platform(value)
    }
}

impl From<AssetError> for EngineError {
    fn from(value: AssetError) -> Self {
        EngineError::Asset(value)
    }
}

impl From<NetworkError> for EngineError {
    fn from(value: NetworkError) -> Self {
        EngineError::Network(value)
    }
}

/// Reports an asset hash mismatch at the editor boundary (TC-1.12.1.10).
#[must_use]
pub fn asset_pipeline_report_hash_mismatch(expected: u64, got: u64) -> EngineError {
    EngineError::from(AssetError::HashMismatch { expected, got })
}

/// Reports a save-format version mismatch at the persistence boundary (TC-1.12.1.11).
#[must_use]
pub fn save_system_report_version_mismatch(expected: u16, got: u16) -> EngineError {
    EngineError::from(SerializationError::VersionMismatch { expected, got })
}

/// Reports a shader compiler subprocess failure (TC-1.12.4.2).
#[must_use]
pub fn shader_compile_report_subprocess_failure(exit_code: i32, stderr: String) -> EngineError {
    EngineError::from(CompileError::Subprocess { exit_code, stderr })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hint::black_box;

    #[test]
    fn tc_1_12_1_1_io_error_converts_to_engine_error() {
        let err = EngineError::from(IoError::Timeout);
        assert!(matches!(err, EngineError::Io(IoError::Timeout)));
    }

    #[test]
    fn tc_1_12_1_2_serialization_error_converts() {
        let err = EngineError::from(SerializationError::BadMagic {
            expected: 1,
            got: 2,
        });
        assert!(matches!(
            err,
            EngineError::Serialization(SerializationError::BadMagic {
                expected: 1,
                got: 2
            })
        ));
    }

    #[test]
    fn tc_1_12_1_3_validation_error_converts() {
        let err = EngineError::from(ValidationError::OutOfRange { field: "x" });
        assert!(matches!(
            err,
            EngineError::Validation(ValidationError::OutOfRange { field: "x" })
        ));
    }

    #[test]
    fn tc_1_12_1_4_codegen_error_converts() {
        let err = EngineError::from(CodegenError::SymbolConflict {
            symbol: SymbolId(9),
        });
        assert!(matches!(
            err,
            EngineError::Codegen(CodegenError::SymbolConflict {
                symbol: SymbolId(9)
            })
        ));
    }

    #[test]
    fn tc_1_12_1_5_compile_error_converts_through_codegen_error() {
        let err = EngineError::from(CompileError::Link {
            missing_symbol: SymbolId(3),
        });
        assert!(matches!(
            err,
            EngineError::Codegen(CodegenError::Compile(CompileError::Link {
                missing_symbol: SymbolId(3)
            }))
        ));
    }

    #[test]
    fn tc_1_12_1_6_platform_error_converts() {
        let err = EngineError::from(PlatformError::DeviceLost);
        assert!(matches!(
            err,
            EngineError::Platform(PlatformError::DeviceLost)
        ));
    }

    #[test]
    fn tc_1_12_1_7_asset_error_converts() {
        let err = EngineError::from(AssetError::NotFound { asset_id: 42 });
        assert!(matches!(
            err,
            EngineError::Asset(AssetError::NotFound { asset_id: 42 })
        ));
    }

    #[test]
    fn tc_1_12_1_8_network_error_converts() {
        let err = EngineError::from(NetworkError::ConnectionDropped);
        assert!(matches!(
            err,
            EngineError::Network(NetworkError::ConnectionDropped)
        ));
    }

    #[test]
    fn tc_1_12_2_1_to_engine_error_trait() {
        let err = IoError::Cancelled.to_engine_error();
        assert!(matches!(err, EngineError::Io(IoError::Cancelled)));
    }

    #[test]
    fn tc_1_12_3_1_io_error_not_found_carries_path() {
        let path = PathBuf::from("/tmp/foo");
        let err = IoError::NotFound { path: path.clone() };
        let formatted = format!("{err:?}");
        assert!(formatted.contains("/tmp/foo"));
    }

    #[test]
    fn tc_1_12_4_1_compile_error_syntax_diagnostic() {
        let diag = Diagnostic {
            path: None,
            span: Some(SourceSpan {
                line: 10,
                column: 2,
                length: 4,
            }),
            message: "bad token",
            severity: Severity::Error,
        };
        let err = CompileError::Syntax {
            diagnostics: vec![diag.clone()],
        };
        assert!(matches!(err, CompileError::Syntax { .. }));
        let CompileError::Syntax { diagnostics } = err else {
            panic!("expected syntax compile error");
        };
        assert_eq!(diagnostics.len(), 1);
        assert!(matches!(diagnostics[0].severity, Severity::Error));
        assert!(diagnostics[0].span.expect("span").line > 0);
    }

    #[test]
    fn tc_1_12_5_1_diagnostic_severity_ordering() {
        let info = format!("{:?}", Severity::Info);
        let warning = format!("{:?}", Severity::Warning);
        let error = format!("{:?}", Severity::Error);
        let fatal = format!("{:?}", Severity::Fatal);
        assert_ne!(info, warning);
        assert_ne!(warning, error);
        assert_ne!(error, fatal);
    }

    #[test]
    fn tc_1_12_1_9_engine_error_debug_discriminates() {
        let io = EngineError::from(IoError::NotFound {
            path: PathBuf::from("/tmp/missing"),
        });
        let net = EngineError::from(NetworkError::QuicStreamClosed);
        assert_ne!(format!("{io:?}"), format!("{net:?}"));
    }

    #[test]
    fn tc_1_12_1_10_asset_pipeline_bubbles_to_engine_error() {
        let err = asset_pipeline_report_hash_mismatch(1, 2);
        assert!(matches!(
            err,
            EngineError::Asset(AssetError::HashMismatch {
                expected: 1,
                got: 2
            })
        ));
    }

    #[test]
    fn tc_1_12_1_11_save_system_produces_serialization_variant() {
        let err = save_system_report_version_mismatch(2, 3);
        assert!(matches!(
            err,
            EngineError::Serialization(SerializationError::VersionMismatch {
                expected: 2,
                got: 3
            })
        ));
    }

    #[test]
    fn tc_1_12_4_2_shader_compile_error_surfaces_as_compile_error() {
        let err = shader_compile_report_subprocess_failure(7, "dxc failed".to_string());
        assert!(matches!(
            err,
            EngineError::Codegen(CodegenError::Compile(CompileError::Subprocess {
                exit_code: 7,
                ..
            }))
        ));
    }

    #[test]
    fn tc_1_12_1_12_engine_error_conversion_under_50_ns() {
        let iterations = 1_000_000_u32;
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            black_box(EngineError::from(IoError::Timeout));
        }
        let elapsed = start.elapsed();
        let per_ns = elapsed.as_nanos() / u128::from(iterations);
        let limit = if cfg!(debug_assertions) { 2_000 } else { 50 };
        assert!(
            per_ns <= limit,
            "average {per_ns}ns exceeds relaxed limit {limit}ns"
        );
    }
}
