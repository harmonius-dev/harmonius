//! CPU-side helpers and types for the GPU particle system.
//!
//! Deterministic spawn and simulation stepping functions support test cases `TC-11.1.1.*` and
//! `TC-11.1.2.*` without a GPU.

pub mod budget;
pub mod dispatch;
pub mod emitter_shape;
pub mod fluid_grid;
pub mod freelist;
pub mod lod;
pub mod particle_lights;
pub mod radix_sort_cpu;
pub mod render_cpu;
pub mod simulation_cpu;
pub mod sub_emitter;
pub mod warmup;

pub use budget::{allocate_by_priority, EmitterBudgetRequest, ParticleBudgetManager};
pub use dispatch::compute_thread_group_count_u32;
pub use emitter_shape::{spawn_positions, EmitterShape};
pub use fluid_grid::FluidDensityGrid;
pub use freelist::{FreelistAllocator, FreelistStats};
pub use lod::{lod_tier_after_distance, LodConfig, LodTier};
pub use particle_lights::particle_light_write_count;
pub use radix_sort_cpu::radix_sort_indices_by_u32_keys;
pub use render_cpu::{
    camera_basis, catmull_rom_segment, flipbook_frame_index, mesh_particle_instance_matrices,
    ribbon_segment_lengths, soft_depth_fade_alpha, sprite_billboard_corners,
};
pub use simulation_cpu::{
    apply_drag, apply_gravity, curl_noise_velocity_sample, depth_buffer_bounce_velocity,
    divergence_estimate_3d, sample_color_over_life_linear, sample_size_over_life,
    sdf_collision_velocity, FusedSimDescriptor,
};
pub use sub_emitter::{
    sub_emit_inherit_velocity, sub_emit_spawn_position_on_collision,
    sub_emit_spawn_position_on_death, sub_emitter_max_depth,
};
pub use warmup::warmup_estimated_alive;
