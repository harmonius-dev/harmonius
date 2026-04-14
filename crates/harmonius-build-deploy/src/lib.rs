//! Build, packaging, and deploy pipeline helpers for Harmonius.
//!
//! This crate hosts deterministic, test-first implementations for the build and deploy design
//! (`docs/design/tools/build-deploy.md`), including shared cache keys and packaging helpers.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]
// `cargo clippy` (without `--all-targets`) builds this library with `cfg(test)` disabled, so
// unit-test-only call graphs would otherwise trip `dead_code` under `-D warnings`.
#![cfg_attr(not(test), allow(dead_code))]

mod cache_key;
mod console;
mod infra;
mod launcher;
mod marketplace;
mod mod_support;
mod packaging;
mod tiered_cache;

pub use cache_key::{
    compute_logic_graph_cache_key, compute_shader_cache_key, compute_shared_cache_key,
    CacheKeyInputs,
};
