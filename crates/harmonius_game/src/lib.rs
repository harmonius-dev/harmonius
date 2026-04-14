//! Game-facing engine modules such as spatial awareness and selection queries.
//!
//! This crate is introduced by [`PLAN-simulation-spatial-awareness`](https://github.com/cjhowe/harmonius/blob/main/docs/plans/simulation/spatial-awareness.md)
//! and follows `docs/design/simulation/spatial-awareness.md`.

#![forbid(unsafe_code)]
#![deny(clippy::all)]

pub mod spatial_awareness;
