//! CPU-side procedural animation primitives used by Harmonius procedural animation plans.
//!
//! **Scope (Tier 0):** This crate is a deterministic **reference and math layer** for TDD against
//! `docs/design/animation/procedural-test-cases.md`. It is **not** the full ECS-first subsystem in
//! `docs/design/animation/procedural.md` (components, plugins, spatial queries, and `glam` are
//! wired in follow-up integration work).
//!
//! Solvers favor pure, deterministic functions so automated tests can assert numeric contracts
//! from `docs/design/animation/procedural-test-cases.md`. Internal [`math`](crate::math) types
//! avoid an external math dependency in this minimal workspace slice.

#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(missing_docs)]

pub mod math;
pub mod procedural;
