# Error Hierarchy Test Cases

Companion test cases for [error.md](error.md).

## Unit Tests

### TC-1.12.1.1 IoError Converts To EngineError

| # | Requirement |
|---|-------------|
| 1 | F-1.12.1    |

1. **#1** — `EngineError::from(IoError::Timeout)`
   - **Expected:** `EngineError::Io(IoError::Timeout)`

### TC-1.12.1.2 SerializationError Converts

| # | Requirement |
|---|-------------|
| 1 | F-1.12.1    |

1. **#1** — `EngineError::from(SerializationError::BadMagic { expected: 1, got: 2 })`
   - **Expected:** Matches `EngineError::Serialization(...)`

### TC-1.12.1.3 ValidationError Converts

| # | Requirement |
|---|-------------|
| 1 | F-1.12.1    |

1. **#1** — `EngineError::from(ValidationError::OutOfRange { field: "x" })`
   - **Expected:** `EngineError::Validation(...)` variant

### TC-1.12.1.4 CodegenError Converts

| # | Requirement |
|---|-------------|
| 1 | F-1.12.1    |

1. **#1** — `EngineError::from(CodegenError::SymbolConflict { .. })`
   - **Expected:** `EngineError::Codegen(...)` variant

### TC-1.12.1.5 CompileError Converts Through CodegenError

| # | Requirement |
|---|-------------|
| 1 | F-1.12.4    |

1. **#1** — `EngineError::from(CompileError::Link { .. })`
   - **Expected:** `EngineError::Codegen(CodegenError::Compile(CompileError::Link ..))`

### TC-1.12.1.6 PlatformError Converts

| # | Requirement |
|---|-------------|
| 1 | F-1.12.1    |

1. **#1** — `EngineError::from(PlatformError::DeviceLost)`
   - **Expected:** `EngineError::Platform(PlatformError::DeviceLost)`

### TC-1.12.1.7 AssetError Converts

| # | Requirement |
|---|-------------|
| 1 | F-1.12.1    |

1. **#1** — `EngineError::from(AssetError::NotFound { asset_id: 42 })`
   - **Expected:** `EngineError::Asset(...)` variant

### TC-1.12.1.8 NetworkError Converts

| # | Requirement |
|---|-------------|
| 1 | F-1.12.1    |

1. **#1** — `EngineError::from(NetworkError::ConnectionDropped)`
   - **Expected:** `EngineError::Network(NetworkError::ConnectionDropped)`

### TC-1.12.2.1 ToEngineError Trait

| # | Requirement |
|---|-------------|
| 1 | F-1.12.2    |

1. **#1** — `IoError::Cancelled.to_engine_error()`
   - **Expected:** `EngineError::Io(IoError::Cancelled)`

### TC-1.12.3.1 IoError NotFound Carries Path

| # | Requirement |
|---|-------------|
| 1 | F-1.12.3    |

1. **#1** — `IoError::NotFound { path: "/tmp/foo" }` via Debug
   - **Expected:** String contains `"/tmp/foo"`

### TC-1.12.4.1 CompileError Syntax Diagnostic

| # | Requirement |
|---|-------------|
| 1 | F-1.12.4    |
| 2 | F-1.12.5    |

1. **#1** — `CompileError::Syntax { diagnostics: vec![diag] }`, inspect diag
   - **Expected:** Has `severity == Severity::Error`
2. **#2** — Diagnostic has `span` set
   - **Expected:** `span.line > 0`

### TC-1.12.5.1 Diagnostic Severity Ordering

| # | Requirement |
|---|-------------|
| 1 | F-1.12.5    |

1. **#1** — Construct Diagnostic with each Severity variant
   - **Expected:** All four variants constructible; Debug output differs

### TC-1.12.1.9 EngineError Debug Discriminates

| # | Requirement |
|---|-------------|
| 1 | F-1.12.1    |

1. **#1** — `Debug::fmt` of `EngineError::Io(IoError::NotFound ..)` and `EngineError::Network(..)`
   - **Expected:** Different strings

## Integration Tests

### TC-1.12.1.10 Asset Pipeline Bubbles To EngineError

| # | Requirement |
|---|-------------|
| 1 | F-1.12.1    |

1. **#1** — Asset pipeline reports `AssetError::HashMismatch`
   - **Expected:** Converted to `EngineError::Asset(...)` at editor boundary

### TC-1.12.1.11 Save System Produces Serialization Variant

| # | Requirement |
|---|-------------|
| 1 | F-1.12.1    |

1. **#1** — Save system encounters version mismatch
   - **Expected:** EngineError::Serialization(SerializationError::VersionMismatch)

### TC-1.12.4.2 Shader Compile Error Surfaces As CompileError

| # | Requirement |
|---|-------------|
| 1 | F-1.12.4    |

1. **#1** — naga in-process compilation exits with non-zero; shader compile error raised
   - **Expected:** `EngineError::Codegen(CodegenError::Compile(CompileError::Subprocess ..))`

## Benchmarks

### TC-1.12.1.12 EngineError Conversion Under 50 ns

| # | Requirement |
|---|-------------|
| 1 | R-1.12.1a   |

1. **#1** — `EngineError::from(IoError::Timeout)` average over 1M iterations
   - **Expected:** Under 50 ns per call

### TC-1.12.5.2 Diagnostic Allocation Avoided

| # | Requirement |
|---|-------------|
| 1 | R-1.12.5a   |

1. **#1** — Construct Diagnostic with `message: &'static str` and path=None
   - **Expected:** Zero heap allocation
