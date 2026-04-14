//! Headless editor ↔ core runtime integration harness.
//!
//! This crate provides deterministic stand-ins for [`EditorWorld`](world::EditorWorld),
//! [`GameWorld`](world::GameWorld), [`EventBridge`](bridge::EventBridge), and snapshot plumbing
//! described in `docs/design/integration/editor-core-runtime.md`. It exists so CI can exercise
//! `IR-9.1.x` ordering contracts before the full ECS stack lands.
//!
//! `EventBridge` is an in-process queue, not a cross-thread channel; see `bridge` module docs.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod bridge;
pub mod error;
pub mod harness;
pub mod mutation;
pub mod snapshot;
pub mod world;
