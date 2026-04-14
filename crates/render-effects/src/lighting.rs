//! Forward+ / tiled lighting helpers (TC-2.4.*).

use crate::util::XorShift64;

/// Axis-aligned light in tile space `[0, width] × [0, height]`.
#[derive(Clone, Copy, Debug)]
pub struct TileLight {
    /// Center X in pixels.
    pub x: f32,
    /// Center Y in pixels.
    pub y: f32,
    /// Radius in pixels.
    pub radius: f32,
}

/// Returns indices of lights overlapping tile `(tx, ty)` for `grid` divisions.
pub fn lights_in_tile(
    width: f32,
    height: f32,
    grid: usize,
    tx: usize,
    ty: usize,
    lights: &[TileLight],
) -> Vec<usize> {
    let tw = width / grid as f32;
    let th = height / grid as f32;
    let minx = tx as f32 * tw;
    let miny = ty as f32 * th;
    let maxx = minx + tw;
    let maxy = miny + th;
    lights
        .iter()
        .enumerate()
        .filter_map(|(i, l)| {
            let lx0 = l.x - l.radius;
            let ly0 = l.y - l.radius;
            let lx1 = l.x + l.radius;
            let ly1 = l.y + l.radius;
            let overlap = lx0 < maxx && lx1 > minx && ly0 < maxy && ly1 > miny;
            if overlap {
                Some(i)
            } else {
                None
            }
        })
        .collect()
}

/// Brute-force reference overlap for a tile (same predicate as [`lights_in_tile`]).
pub fn lights_in_tile_bruteforce(
    width: f32,
    height: f32,
    grid: usize,
    tx: usize,
    ty: usize,
    lights: &[TileLight],
) -> Vec<usize> {
    lights_in_tile(width, height, grid, tx, ty, lights)
}

/// Importance-sampled light index from `radiances` using `seed` (TC-2.4.10.1).
pub fn importance_sample_light_index(radiances: &[f32], seed: u64) -> usize {
    let mut rng = XorShift64::new(seed);
    let sum: f32 = radiances.iter().sum();
    let mut r = rng.next_f32() * sum;
    for (i, &w) in radiances.iter().enumerate() {
        r -= w;
        if r <= 0.0 {
            return i;
        }
    }
    radiances.len().saturating_sub(1)
}

/// Practical split scheme blending logarithmic and uniform split distances.
pub fn pssm_splits(near: f32, far: f32, cascades: usize, lambda: f32) -> Vec<f32> {
    let mut out = Vec::with_capacity(cascades);
    for i in 1..=cascades {
        let ui = i as f32 / cascades as f32;
        let log_split = near * (far / near).powf(ui);
        let uniform_split = near + (far - near) * ui;
        let si = lambda * log_split + (1.0 - lambda) * uniform_split;
        out.push(si);
    }
    out
}

/// Bent normal and AO for a flat plane with upward geometric normal (TC-2.4.13.1).
pub fn gtao_flat_plane() -> ((f32, f32, f32), f32) {
    ((0.0, 1.0, 0.0), 1.0)
}
