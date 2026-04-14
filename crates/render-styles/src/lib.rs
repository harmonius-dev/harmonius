//! Deterministic CPU-side helpers for Harmonius render-style passes. Submodule map:
//! `aa_upscale` (2.6), `environment` (2.7), `character` (2.8), `stylized` (2.11), `materials`
//! (2.12), `decals` (11.2). Each module pairs with `TC-*` rows in
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
