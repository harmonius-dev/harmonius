#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

//! Build, packaging, and deploy pipeline helpers for Harmonius.
//!
//! Implements deterministic shared-cache keying described in `docs/design/tools/build-deploy.md`
//! (BLAKE3 over a canonical byte layout).

mod cache_key;

pub use cache_key::{compute_shared_cache_key, CacheKeyInputs};
