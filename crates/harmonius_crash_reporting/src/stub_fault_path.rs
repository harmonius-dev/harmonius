//! Signal-safe crash stub placeholder (`R-14.4.1`).
//!
//! The production fault path must remain allocator-free. This module is intentionally tiny so unit
//! tests can scan it for forbidden allocator entry points.

#![forbid(unsafe_code)]

/// No-op placeholder for the signal-safe fault stub surface.
pub const fn noop_fault_stub() {}
