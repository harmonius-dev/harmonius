//! Grid flow fields built with Dijkstra on a cost surface.

use glam::{UVec2, Vec2};
use ordered_float::OrderedFloat;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Scalar traversal cost per cell.
pub trait CostGrid {
    /// Positive finite cost to enter cell `(x, y)`; `f32::MAX` blocks the cell.
    fn cost(&self, x: u32, y: u32) -> f32;
}

/// Uniform cost everywhere (open terrain).
#[derive(Clone, Copy, Debug)]
pub struct UniformCostGrid {
    /// Cost applied to every traversable cell.
    pub cell_cost: f32,
}

impl CostGrid for UniformCostGrid {
    fn cost(&self, _x: u32, _y: u32) -> f32 {
        self.cell_cost
    }
}

/// Direction grid toward a goal produced by `generate_flow_field`.
#[derive(Clone, Debug, PartialEq)]
pub struct FlowField {
    /// Unit direction per cell, row-major (x fastest).
    pub grid: Vec<Vec2>,
    /// World-space origin (corner of cell (0,0)).
    pub origin: Vec2,
    /// Edge length of one cell in world units.
    pub cell_size: f32,
    /// Cells in X.
    pub width: u32,
    /// Cells in Y.
    pub height: u32,
    /// Goal cell indices.
    pub goal_cell: UVec2,
    /// Cost layer version used when building this field.
    pub cost_version: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct State {
    dist: OrderedFloat<f32>,
    idx: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn neighbors_4(x: u32, y: u32, width: u32, height: u32) -> impl Iterator<Item = (u32, u32)> {
    let mut out = [(0_u32, 0_u32); 4];
    let mut n = 0_usize;
    if x > 0 {
        out[n] = (x - 1, y);
        n += 1;
    }
    if x + 1 < width {
        out[n] = (x + 1, y);
        n += 1;
    }
    if y > 0 {
        out[n] = (x, y - 1);
        n += 1;
    }
    if y + 1 < height {
        out[n] = (x, y + 1);
        n += 1;
    }
    out.into_iter().take(n)
}

fn neighbors_8(x: u32, y: u32, width: u32, height: u32) -> impl Iterator<Item = (u32, u32)> {
    let mut out = Vec::with_capacity(8);
    for dy in -1_i32..=1 {
        for dx in -1_i32..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 {
                out.push((nx as u32, ny as u32));
            }
        }
    }
    out.into_iter()
}

/// Run Dijkstra from `goal` and emit a unit direction field toward lower cost.
pub fn generate_flow_field(
    cost_grid: &impl CostGrid,
    goal: UVec2,
    origin: Vec2,
    cell_size: f32,
    width: u32,
    height: u32,
    cost_version: u64,
) -> FlowField {
    let total = (width * height) as usize;
    let mut dist = vec![f32::MAX; total];
    let mut dirs = vec![Vec2::ZERO; total];
    let goal_idx = (goal.y * width + goal.x) as usize;
    dist[goal_idx] = 0.0;
    let mut heap = BinaryHeap::new();
    heap.push(State {
        dist: OrderedFloat(0.0),
        idx: goal_idx,
    });

    while let Some(State {
        dist: OrderedFloat(d),
        idx,
    }) = heap.pop()
    {
        if d > dist[idx] {
            continue;
        }
        let x = (idx as u32) % width;
        let y = (idx as u32) / width;
        for (nx, ny) in neighbors_4(x, y, width, height) {
            let ni = (ny * width + nx) as usize;
            let step = cost_grid.cost(nx, ny);
            if step >= f32::MAX {
                continue;
            }
            let nd = d + step;
            if nd < dist[ni] {
                dist[ni] = nd;
                heap.push(State {
                    dist: OrderedFloat(nd),
                    idx: ni,
                });
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) as usize;
            if dist[idx] >= f32::MAX {
                continue;
            }
            let mut best_dir = Vec2::ZERO;
            let mut best_dist = dist[idx];
            for (nx, ny) in neighbors_8(x, y, width, height) {
                let ni = (ny * width + nx) as usize;
                if dist[ni] < best_dist {
                    best_dist = dist[ni];
                    best_dir = Vec2::new(nx as f32 - x as f32, ny as f32 - y as f32);
                }
            }
            dirs[idx] = best_dir.normalize_or_zero();
        }
    }

    FlowField {
        grid: dirs,
        origin,
        cell_size,
        width,
        height,
        goal_cell: goal,
        cost_version,
    }
}

/// Nearest-cell lookup of the flow direction at `world_pos`.
pub fn sample_flow_field(field: &FlowField, world_pos: Vec2) -> Vec2 {
    let local = world_pos - field.origin;
    let fx = local.x / field.cell_size;
    let fy = local.y / field.cell_size;
    let ix = (fx as u32).min(field.width.saturating_sub(1));
    let iy = (fy as u32).min(field.height.saturating_sub(1));
    let idx = (iy * field.width + ix) as usize;
    if idx < field.grid.len() {
        field.grid[idx]
    } else {
        Vec2::ZERO
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::UVec2;
    use std::time::Instant;

    #[test]
    fn tc_7_7_2_1_flow_field_reaches_goal() {
        let w = 64_u32;
        let h = 64_u32;
        let costs = UniformCostGrid { cell_cost: 1.0 };
        let goal = UVec2::new(63, 63);
        let field = generate_flow_field(&costs, goal, Vec2::ZERO, 1.0, w, h, 1);
        let mut rng = 7_u64;
        for _ in 0..100 {
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
            let x = (rng % w as u64) as u32;
            let y = (rng / w as u64 % h as u64) as u32;
            let world = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
            let mut p = world;
            for _ in 0..500 {
                let d = sample_flow_field(&field, p);
                if d.length_squared() < 1e-8 {
                    break;
                }
                p += d * 0.5;
                let gx = field.goal_cell.x as f32 + 0.5;
                let gy = field.goal_cell.y as f32 + 0.5;
                if p.distance(Vec2::new(gx, gy)) < 1.0 {
                    break;
                }
            }
            let gx = field.goal_cell.x as f32 + 0.5;
            let gy = field.goal_cell.y as f32 + 0.5;
            assert!(p.distance(Vec2::new(gx, gy)) < 2.5);
        }
    }

    #[test]
    fn tc_7_7_2_2_flow_sample_constant_cost() {
        let costs = UniformCostGrid { cell_cost: 1.0 };
        let field = generate_flow_field(&costs, UVec2::new(31, 31), Vec2::ZERO, 1.0, 64, 64, 1);
        let mut samples = Vec::new();
        for _ in 0..100 {
            let x = (samples.len() % 64) as f32;
            let y = (samples.len() / 64 % 64) as f32;
            samples.push(Vec2::new(x, y));
        }
        let t0 = Instant::now();
        for _ in 0..10_000 {
            for &p in &samples {
                let _ = sample_flow_field(&field, p);
            }
        }
        let t_large = t0.elapsed();
        let t1 = Instant::now();
        for _ in 0..100 {
            for &p in &samples {
                let _ = sample_flow_field(&field, p);
            }
        }
        let t_small = t1.elapsed();
        let per_large = t_large.as_secs_f64() / (10_000.0 * 100.0);
        let per_small = t_small.as_secs_f64() / (100.0 * 100.0);
        assert!(
            per_large <= per_small * 2.5,
            "per-sample large {per_large} vs small {per_small}"
        );
    }
}
