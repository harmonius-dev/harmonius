//! Error types for the telemetry client.

use core::fmt;

/// Top-level error for [`crate::TelemetryClient`] operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TelemetryError {
    /// User-facing configuration or environment error.
    Config(&'static str),
    /// Disk or serialization failure on the buffer path.
    Io(String),
    /// Upload or backend transport failure.
    Upload(String),
}

impl fmt::Display for TelemetryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TelemetryError::Config(msg) => write!(f, "{msg}"),
            TelemetryError::Io(msg) => write!(f, "{msg}"),
            TelemetryError::Upload(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for TelemetryError {}

/// Counter registry failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CounterError {
    /// Registering a name that already exists.
    DuplicateName,
    /// Unknown counter name for a read/write.
    UnknownName,
}

impl fmt::Display for CounterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CounterError::DuplicateName => write!(f, "duplicate counter name"),
            CounterError::UnknownName => write!(f, "unknown counter name"),
        }
    }
}

impl std::error::Error for CounterError {}

/// Bounded in-memory buffer failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BufferError {
    /// In-memory ring is full and cannot accept another record without spill/drain.
    Full,
    /// Fewer records than requested for an exact drain.
    NotEnoughRecords,
}

impl fmt::Display for BufferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BufferError::Full => write!(f, "telemetry ring buffer is full"),
            BufferError::NotEnoughRecords => write!(f, "not enough records for exact drain"),
        }
    }
}

impl std::error::Error for BufferError {}

/// Upload failures surfaced to the client.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UploadError(pub String);

impl fmt::Display for UploadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for UploadError {}
