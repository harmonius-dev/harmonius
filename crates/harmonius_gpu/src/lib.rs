//! GPU abstraction traits and CPU-side helpers aligned with `docs/design/rendering/render-pipeline.md`.

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
