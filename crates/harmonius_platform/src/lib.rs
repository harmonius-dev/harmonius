//! Harmonius platform layer: native windowing types and helpers.
//!
//! This crate hosts cross-platform windowing data structures. OS-specific
//! backends live in separate platform modules as the engine grows.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod windowing;
