//! Core runtime building blocks for Harmonius.
//!
//! This crate currently hosts the main-thread I/O request protocol (`io`).
//!
//! The `io` dispatcher uses synchronous [`std::fs`] in this milestone for bring-up; see
//! [`crate::io`] module documentation for backend and channel limitations.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod error;
pub mod io;
pub mod primitives;
