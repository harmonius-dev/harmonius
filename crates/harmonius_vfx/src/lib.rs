//! Visual effects runtime: decals, screen effects, weather, destruction, and effect graphs.
//!
//! Behavior is traced to `docs/design/vfx/effects.md` and the `PLAN-vfx-effects` implementation
//! plan.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod effect_graph;
pub mod effects;
pub mod logic_eval;
pub mod particles;
