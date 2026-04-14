//! Crash reporting primitives: symbol indexing, stack fingerprints, dump rotation, and monitor IPC.
//!
//! This crate implements the platform-agnostic portions of the crash reporting design. Platform
//! signal handlers remain in dedicated `unsafe` platform modules (not covered by the initial unit
//! slice).

#![deny(clippy::all)]
#![deny(missing_docs)]

pub mod alerting;
pub mod cluster;
pub mod fingerprint;
pub mod handler;
pub mod metadata_sidecar;
pub mod minidump;
pub mod monitor;
pub mod rotation;
pub mod stub_fault_path;
pub mod symbol_server;
pub mod symbols;

pub use handler::{BreadcrumbHandle, CrashConfig, CrashHandler, CrashInstallError, UserId};
pub use monitor::{FaultNotification, MonitorArgs};
pub use symbols::{BuildId, SymbolBundleEntry};
