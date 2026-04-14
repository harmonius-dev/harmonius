//! CPU-side helpers and types for the GPU particle system.
//!
//! Deterministic spawn and simulation stepping functions support test cases `TC-11.1.1.*` and
//! `TC-11.1.2.*` without a GPU.

pub mod dispatch;
pub mod emitter_shape;
pub mod freelist;
pub mod simulation_cpu;

pub use dispatch::compute_thread_group_count_u32;
pub use emitter_shape::{EmitterShape, spawn_positions};
pub use freelist::{FreelistAllocator, FreelistStats};
pub use simulation_cpu::{apply_drag, apply_gravity};
