//! Deterministic CPU-side helpers for Harmonius render-style passes (TAA, stylized effects,
//! environment, character shading, materials, decals). Each submodule pairs with `TC-*` cases in
//! `docs/design/rendering/render-styles-test-cases.md`.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod aa_upscale;
pub mod character;
pub mod decals;
pub mod environment;
pub mod materials;
pub mod stylized;
