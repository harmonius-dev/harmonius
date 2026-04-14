#![forbid(unsafe_code)]
#![allow(missing_docs)]
//! Harmonius save system primitives.
//!
//! This crate hosts the first executable slice of
//! [`PLAN-game-framework-save-system`](https://github.com/cjhowe-us/harmonius/blob/main/docs/plans/game-framework/save-system.md):
//! deterministic serialization helpers, migration planning, slot layout, and the on-disk pipeline
//! (CRC → compress → encrypt → atomic write) described in `docs/design/game-framework/save-system.md`.

pub mod arena;
pub mod error;
pub mod lazy;
pub mod manager;
pub mod migration;
pub mod pipeline;
pub mod procedural;
pub mod schema;
pub mod serialize;
pub mod slots;
pub mod sync;
pub mod types;
pub mod vfs;
