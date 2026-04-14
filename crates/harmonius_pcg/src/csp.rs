//! Tiny constraint satisfaction helpers for placement tests.

use glam::Vec2;

/// Placement solver errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CspError {
    /// Constraints cannot be satisfied in bounded search.
    Unsatisfiable,
}

/// Assigns `count` sites in a large world with pairwise distance >= `min_distance`.
pub fn place_min_distance_2d(
    count: usize,
    min_distance: f32,
    world: (f32, f32),
    seed: u64,
) -> Result<Vec<Vec2>, CspError> {
    let mut out = Vec::new();
    let mut s = seed;
    let mut guard = 0usize;
    while out.len() < count && guard < count * 10_000 {
        guard += 1;
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1);
        let x = (s as f32 / u64::MAX as f32) * world.0;
        let y = ((s >> 32) as f32 / u64::MAX as f32) * world.1;
        let p = Vec2::new(x, y);
        if out.iter().all(|q: &Vec2| q.distance(p) >= min_distance) {
            out.push(p);
        }
    }
    if out.len() == count {
        Ok(out)
    } else {
        Err(CspError::Unsatisfiable)
    }
}

/// Attempts to pack `count` circles of radius `min_distance/2` into `area` rectangle — fast reject.
pub fn place_min_distance_bounded(
    count: usize,
    min_distance: f32,
    area: (f32, f32),
    seed: u64,
) -> Result<Vec<Vec2>, CspError> {
    let max_kiss = (area.0 / min_distance).floor() * (area.1 / min_distance).floor();
    if (max_kiss as usize) < count {
        return Err(CspError::Unsatisfiable);
    }
    place_min_distance_2d(count, min_distance, area, seed)
}
