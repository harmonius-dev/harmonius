//! Harmonius platform layer: windowing, filesystem, crash hooks, OS bridges,
//! storage helpers, and online service stubs.
//!
//! Host I/O in this crate uses synchronous `std::fs` shims so unit tests can
//! run without async runtimes; native backends replace these seams later.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod crash;
pub mod filesystem;
pub mod os;
pub mod sdk;
pub mod services;
pub mod storage;
pub mod windowing;
