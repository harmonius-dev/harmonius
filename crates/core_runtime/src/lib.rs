//! Core runtime building blocks for Harmonius.
//!
//! This crate currently hosts the main-thread I/O request protocol (`io`).

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod error;
pub mod io;
pub mod primitives;
