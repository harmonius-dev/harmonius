//! Steering behaviors, ORCA avoidance, flocking, flow fields, LOD scheduling, and density
//! enforcement aligned with `docs/design/ai/steering-crowds.md`.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

pub mod crowd;
pub mod steering;
