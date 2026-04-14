//! Platform layer: threading, topology, and channel-based I/O integration.
//!
//! Implements the API surface described in `docs/design/platform/threading.md`.

#![deny(clippy::all)]
#![deny(missing_docs)]
// Work-stealing deques and parking use `unsafe` only inside dependency crates; this crate stays safe.
#![deny(unsafe_code)]

pub mod threading;
