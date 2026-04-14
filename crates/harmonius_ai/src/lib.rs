//! Artificial intelligence systems for Harmonius: navigation, pathfinding, and related data.
//!
//! This crate currently focuses on [`navigation`] — NavMesh tiles, A\*, funnel smoothing,
//! and supporting structures described in `docs/design/ai/navigation.md`.

#![forbid(unsafe_code)]
#![warn(clippy::all)]
#![warn(missing_docs)]

pub mod navigation;
