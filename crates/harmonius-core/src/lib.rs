//! Core runtime primitives for typed events and plugin composition.
//!
//! This crate implements the contracts described in `docs/design/core-runtime/events-plugins.md`.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod events;
pub mod plugins;
mod world;

pub use world::{ComponentEvent, Entity, World, WorldId};
