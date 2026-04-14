//! Pure, deterministic building blocks for render effects covered by
//! `docs/design/rendering/render-effects-test-cases.md`.

#![deny(unsafe_code)]
#![warn(clippy::all)]
#![allow(missing_docs)]

pub mod auto_exposure;
pub mod bloom;
pub mod cavity;
pub mod chromatic_aberration;
pub mod dof;
pub mod film_grain;
pub mod graph;
pub mod lighting;
pub mod local_exposure;
pub mod motion_blur;
pub mod post_chain;
pub mod quality;
pub mod rt;
pub mod tonemap;
pub mod vignette;
pub mod volume;

mod util;

#[cfg(test)]
mod tc_tests;
