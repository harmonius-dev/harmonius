//! Steering behaviors, ORCA avoidance, flocking, flow fields, LOD scheduling, and density
//! enforcement aligned with `docs/design/ai/steering-crowds.md`.
//!
//! ECS components, `Reflect`, and runtime systems from the design diagrams are intentionally
//! out of scope for this crate; it ships deterministic math helpers and small data structures for
//! TDD against `TC-*` rows before engine integration wiring lands.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

pub mod crowd;
pub mod steering;
