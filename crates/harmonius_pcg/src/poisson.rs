//! Poisson disk sampling (Bridson) on a flat 2D plane for vegetation scatter and surface points.

use std::collections::HashMap;

/// Deterministic RNG (xorshift64*) for sampling without pulling in a full RNG crate.
#[derive(Clone, Copy)]
struct XorShift64 {
    s: u64,
}

impl XorShift64 {
    fn new(seed: u64) -> Self {
        let s = if seed == 0 {
            0x9E37_79B9_7F4A_7C15
        } else {
            seed
        };
        Self { s }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.s;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.s = x;
        x
    }

    fn unit_f32(&mut self) -> f32 {
        (self.next_u64() as f32) / (u64::MAX as f32)
    }
}

fn cell_coords(p: (f32, f32), cell: f32) -> (i32, i32) {
    ((p.0 / cell).floor() as i32, (p.1 / cell).floor() as i32)
}

fn dist2(a: (f32, f32), b: (f32, f32)) -> f32 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    dx * dx + dy * dy
}

/// Bridson-style Poisson disk on `[0, width) × [0, height)` until `target` samples or exhaustion.
///
/// `min_distance` is the minimum pairwise separation (same units as width/height).
pub fn poisson_disk_bridson_2d(
    min_distance: f32,
    width: f32,
    height: f32,
    seed: u64,
    target: usize,
) -> Vec<(f32, f32)> {
    let mut rng = XorShift64::new(seed);
    let cell = min_distance / std::f32::consts::SQRT_2;
    let mut grid: HashMap<(i32, i32), usize> = HashMap::new();
    let mut points: Vec<(f32, f32)> = Vec::new();
    let mut active: Vec<usize> = Vec::new();

    let first = (rng.unit_f32() * width, rng.unit_f32() * height);
    points.push(first);
    active.push(0);
    grid.insert(cell_coords(first, cell), 0);

    let mut guard = 0usize;
    while !active.is_empty() && points.len() < target && guard < target.saturating_mul(200) {
        guard += 1;
        let ai = (rng.next_u64() as usize) % active.len();
        let base_i = active[ai];
        let base = points[base_i];
        let mut accepted = false;
        for _ in 0..30 {
            let radius = min_distance * (1.0 + rng.unit_f32());
            let theta = rng.unit_f32() * std::f32::consts::TAU;
            let candidate = (base.0 + radius * theta.cos(), base.1 + radius * theta.sin());
            if candidate.0 < 0.0
                || candidate.0 >= width
                || candidate.1 < 0.0
                || candidate.1 >= height
            {
                continue;
            }
            let (gx, gy) = cell_coords(candidate, cell);
            let mut ok = true;
            for ox in -2..=2 {
                for oy in -2..=2 {
                    if let Some(&pi) = grid.get(&(gx + ox, gy + oy)) {
                        if dist2(candidate, points[pi]) < min_distance * min_distance {
                            ok = false;
                            break;
                        }
                    }
                }
                if !ok {
                    break;
                }
            }
            if ok {
                let idx = points.len();
                points.push(candidate);
                active.push(idx);
                grid.insert((gx, gy), idx);
                accepted = true;
                break;
            }
        }
        if !accepted {
            active.swap_remove(ai);
        }
    }
    points
}

/// Verifies all pairs closer than `min_distance` are caught by neighborhood grid (O(n)).
pub fn verify_min_distance(points: &[(f32, f32)], min_distance: f32) -> bool {
    let cell = min_distance;
    let mut buckets: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
    for (i, p) in points.iter().enumerate() {
        let (gx, gy) = cell_coords(*p, cell);
        buckets.entry((gx, gy)).or_default().push(i);
    }
    let r2 = min_distance * min_distance;
    for (i, p) in points.iter().enumerate() {
        let (gx, gy) = cell_coords(*p, cell);
        for ox in -1..=1 {
            for oy in -1..=1 {
                if let Some(idxs) = buckets.get(&(gx + ox, gy + oy)) {
                    for &j in idxs {
                        if i == j {
                            continue;
                        }
                        if dist2(*p, points[j]) < r2 {
                            return false;
                        }
                    }
                }
            }
        }
    }
    true
}
