//! CPU-side procedural animation primitives used by Harmonius procedural animation plans.
//!
//! Solvers favor pure, deterministic functions so automated tests can assert numeric contracts
//! from `docs/design/animation/procedural-test-cases.md`.

#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(missing_docs)]

pub mod math;
pub mod procedural;
