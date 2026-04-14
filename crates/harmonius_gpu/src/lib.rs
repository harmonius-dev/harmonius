//! GPU abstraction traits and CPU-side helpers aligned with
//! `docs/design/rendering/render-pipeline.md`.
//!
//! Runtime services the design names `harmonius_gpu_runtime` live in this crate for now. A split
//! crate can follow once the backend surface stabilizes.

#![deny(clippy::all)]
#![deny(unsafe_code)]
#![warn(missing_docs)]

pub mod allocator;
pub mod backend;
pub mod barrier;
pub mod command_buffer;
pub mod feature_emulation;
pub mod gpu_perf;
pub mod platform;
pub mod pso;
pub mod state_tracker;
pub mod work_graph;
