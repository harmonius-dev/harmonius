//! Froxel density injection helpers (IR-3.7.3).

use glam::Vec3;

use crate::types::Aabb;
use crate::types::FroxelInjection;
use crate::types::MobileVolumetricCaps;

/// Marks whether a froxel cell center lies inside `bounds`.
pub fn cell_center_inside_aabb(cell_center: Vec3, bounds: &Aabb) -> bool {
    cell_center.cmpge(bounds.min).all() && cell_center.cmple(bounds.max).all()
}

/// Writes baseline density into `grid` for cells whose centers fall inside `injection.world_aabb`.
///
/// TC-IR-3.7.3.1 — fog volume increases density for enclosed cells.
pub fn apply_fog_density_injection(
    grid: &mut [f32],
    cell_centers: &[Vec3],
    injection: &FroxelInjection,
) {
    for (i, c) in cell_centers.iter().enumerate().take(grid.len()) {
        if cell_center_inside_aabb(*c, &injection.world_aabb) {
            grid[i] += injection.density;
        }
    }
}

/// TC-IR-3.7.3.2 — weather increases scattering versus baseline (scalar bump on channel sum).
pub fn apply_weather_scattering_bump(base: Vec3, intensity: f32) -> Vec3 {
    base + Vec3::splat(0.05 * intensity.clamp(0.0, 1.0))
}

/// TC-IR-3.7.3.F1 — clamp per-voxel density to a load-time maximum.
pub fn clamp_froxel_density(value: f32, max_density: f32) -> f32 {
    value.min(max_density)
}

/// TC-IR-3.7.3.S1 — CPU screen-tile fallback buffer (16×9) population for mobile caps.
pub fn populate_mobile_screen_tile_fog(
    caps: MobileVolumetricCaps,
    fog_present: bool,
    out_tiles: &mut [f32; 144],
) -> bool {
    if caps.gpu_froxel_supported || !fog_present {
        return false;
    }
    for t in out_tiles.iter_mut() {
        *t = 0.1;
    }
    true
}
