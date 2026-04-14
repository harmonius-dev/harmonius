//! Deterministic helpers for the rendering core pipeline (`rendering::core`).
//!
//! This crate hosts pure, testable building blocks for the rendering-core design
//! (`docs/design/rendering/rendering-core.md` in the Harmonius repository).
#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod batch_compact;
pub mod bindless_params;
pub mod draw_batch;
pub mod frame_ring;
pub mod layers;
pub mod light_buffer;
pub mod meshlet_frustum;
pub mod normal_cone;
pub mod pass_barrier;
pub mod projection;
pub mod proxy_dirty;
pub mod sort_key;
pub mod transform;
pub mod view_registry;
