//! Crash reporting and diagnostics helpers (`R-14.4`).

mod aggregate;
mod gpu;
mod handler;
mod logger;
mod perf;
mod symbols;

pub use aggregate::{CrashAggregator, StackSignature};
pub use gpu::{GpuBreadcrumbs, GpuDevice, PassId};
pub use handler::{CrashHandler, CrashHandlerConfig};
pub use logger::{LogFilter, LogRecord, LogSink, Logger, Severity};
pub use perf::{CounterName, PerfCounters, Snapshot};
pub use symbols::{SymbolFormat, SymbolUploader};

/// Errors surfaced by crash-path helpers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrashError {
    /// Crash handler helper binary could not be located.
    HandlerNotFound {
        /// Path that was missing.
        path: String,
    },
    /// Helper failed to launch.
    HandlerLaunchFailed {
        /// Host status code.
        code: i32,
    },
    /// Symbol upload failed.
    UploadFailed {
        /// HTTP status when applicable.
        status: u16,
        /// Server body or message.
        message: String,
    },
    /// Build id missing from the binary.
    BuildIdNotFound,
    /// GPU buffer for breadcrumbs could not be allocated.
    GpuBufferAllocationFailed,
    /// Catch-all platform error.
    Platform {
        /// errno-style code.
        code: i32,
        /// Message text.
        message: String,
    },
}
