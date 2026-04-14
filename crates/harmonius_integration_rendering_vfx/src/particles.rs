//! CPU-side helpers for particle draw + sort policy (IR-3.7.2).

use glam::Vec3;

/// Single particle instance for ordering tests.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParticleInstance {
    /// World position.
    pub position: Vec3,
}

/// Builds indirect draw instance count for sprite billboards.
///
/// TC-IR-3.7.2.1 — `DrawIndirect` count matches alive billboard count.
pub fn sprite_indirect_instance_count(alive_sprites: u32) -> u32 {
    alive_sprites
}

/// Builds indirect draw vertex/instance count for ribbon strips.
///
/// TC-IR-3.7.2.2 — ribbon path issues non-zero indirect args for trail strips.
pub fn ribbon_indirect_vertex_count(control_points: u32) -> u32 {
    control_points.saturating_mul(2).saturating_sub(1).max(1)
}

/// Sorts particle indices back-to-front using squared distance to `camera_position`.
///
/// TC-IR-3.7.2.3 — farthest particles draw first for alpha correctness.
pub fn sort_particle_indices_back_to_front(
    camera_position: Vec3,
    particles: &[ParticleInstance],
) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..particles.len()).collect();
    idx.sort_by(|&a, &b| {
        let da = particles[a].position.distance_squared(camera_position);
        let db = particles[b].position.distance_squared(camera_position);
        db.partial_cmp(&da).unwrap_or(std::cmp::Ordering::Equal)
    });
    idx
}

/// TC-IR-3.7.2.F1 — skip GPU radix sort when scratch is undersized; log warning.
pub fn radix_sort_skipped(particle_count: usize, scratch_capacity: usize) -> bool {
    scratch_capacity < particle_count
}
