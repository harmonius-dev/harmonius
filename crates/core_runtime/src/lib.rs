#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

//! Core runtime primitives for Harmonius.
//!
//! The [`ids`] module owns engine-wide ID taxonomy, [`StableId`](ids::StableId) policies, and
//! network entity mapping helpers described in `docs/design/core-runtime/ids.md`.

pub mod ids;

mod sorted_vec_map;
