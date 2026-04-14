//! Core primitives shared across Harmonius subsystems.
//!
//! # Handle type safety (TC-1.7.4.4)
//!
//! `Handle<T>` is branded by `T`; a `Handle<Mesh>` must not be used with
//! `HandleMap<Texture>`.
//!
//! ```compile_fail
//! use harmonius_core::handle::{Handle, HandleMap};
//! struct Mesh;
//! struct Texture;
//! let mut map: HandleMap<Texture> = HandleMap::new();
//! let h: Handle<Mesh> = Handle::NULL;
//! let _ = map.get(h);
//! ```
#![deny(clippy::all)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

pub mod handle;
