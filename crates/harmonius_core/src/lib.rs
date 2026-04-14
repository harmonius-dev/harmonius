//! Harmonius core runtime primitives.
//!
//! This crate hosts the scene and transform stack described in
//! `docs/design/core-runtime/scene-transforms.md`.

#![deny(clippy::all)]
#![deny(unsafe_code)]
#![warn(missing_docs)]

pub mod job;
pub mod scene;
