//! Dijkstra flow fields over `CellGrid` cost maps.

use std::cmp::Ordering;
use std::collections::BinaryHeap;

use glam::Vec2;

use crate::cell_grid::CellGrid;
use crate::coord::CellCoord;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct State {
    cost: u32,
    idx: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra_distances(walls: &CellGrid<bool>, target: CellCoord) -> Vec<u32> {
    let w = walls.width as usize;
    let h = walls.height as usize;
    let len = w * h;
    let mut dist = vec![u32::MAX; len];
    let mut heap = BinaryHeap::new();
    let Some(ti) = walls.index(target) else {
        return dist;
    };
    if *walls.get(target).unwrap_or(&true) {
        return dist;
    }
    dist[ti] = 0;
    heap.push(State { cost: 0, idx: ti });
    while let Some(State { cost, idx }) = heap.pop() {
        if cost != dist[idx] {
            continue;
        }
        let x = (idx % w) as u32;
        let y = (idx / w) as u32;
        let coord = CellCoord { x, y };
        for n in walls.neighbors(coord, crate::cell_grid::NeighborPattern::Cardinal) {
            let ni = walls.index(n).unwrap();
            if *walls.get(n).unwrap_or(&true) {
                continue;
            }
            let next = cost.saturating_add(1);
            if next < dist[ni] {
                dist[ni] = next;
                heap.push(State {
                    cost: next,
                    idx: ni,
                });
            }
        }
    }
    dist
}

/// Computes a normalized flow field pointing along decreasing Dijkstra cost.
pub fn compute_flow_field(target: CellCoord, walls: &CellGrid<bool>) -> CellGrid<Vec2> {
    let dist = dijkstra_distances(walls, target);
    let mut out = CellGrid::<Vec2>::new(walls.width, walls.height, walls.cell_size, walls.origin);
    for y in 0..walls.height {
        for x in 0..walls.width {
            let c = CellCoord { x, y };
            let idx = walls.index(c).unwrap();
            if dist[idx] == u32::MAX {
                continue;
            }
            let mut best_n: Option<CellCoord> = None;
            let mut best_d = dist[idx];
            for n in walls.neighbors(c, crate::cell_grid::NeighborPattern::Cardinal) {
                let ni = walls.index(n).unwrap();
                let d = dist[ni];
                if d < best_d {
                    best_d = d;
                    best_n = Some(n);
                }
            }
            if let Some(n) = best_n {
                let cur = walls.cell_to_world(c);
                let nxt = walls.cell_to_world(n);
                let mut v = nxt - cur;
                if v.length_squared() > 0.0 {
                    v = v.normalize();
                }
                out.set(c, v);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use super::*;
    use crate::cell_grid::{NeighborPattern, PropagationKernel};

    #[test]
    fn test_flow_field_dijkstra() {
        let mut walls = CellGrid::<bool>::new(32, 32, 1.0, Vec2::ZERO);
        for y in 0..32 {
            walls.set(CellCoord { x: 15, y }, true);
        }
        let goal = CellCoord { x: 31, y: 31 };
        let dist = dijkstra_distances(&walls, goal);
        let flow = compute_flow_field(goal, &walls);
        for y in 0..32 {
            for x in 0..32 {
                let c = CellCoord { x, y };
                if x == 15 {
                    continue;
                }
                let idx = walls.index(c).unwrap();
                if dist[idx] == u32::MAX {
                    continue;
                }
                let v = *flow.get(c).unwrap();
                if v == Vec2::ZERO {
                    continue;
                }
                let mut best: Option<CellCoord> = None;
                let mut best_dot = -1.0f32;
                for n in walls.neighbors(c, NeighborPattern::Cardinal) {
                    if *walls.get(n).unwrap_or(&true) {
                        continue;
                    }
                    let dir = walls.cell_to_world(n) - walls.cell_to_world(c);
                    if dir.length_squared() == 0.0 {
                        continue;
                    }
                    let dir = dir.normalize();
                    let dot = v.dot(dir);
                    if dot > best_dot {
                        best_dot = dot;
                        best = Some(n);
                    }
                }
                let Some(n) = best else {
                    continue;
                };
                let ni = walls.index(n).unwrap();
                assert!(dist[ni] < dist[idx]);
            }
        }
    }

    #[test]
    fn test_influence_decay_block() {
        let mut grid = CellGrid::<f32>::new(16, 16, 1.0, Vec2::ZERO);
        grid.set(CellCoord { x: 0, y: 0 }, 1.0);
        let kernel =
            PropagationKernel::<f32>::new(NeighborPattern::Cardinal, spread_influence, 0.95);
        for _ in 0..10 {
            grid.propagate_masked(&kernel, |c| c.y == 4);
        }
        for y in 5..16 {
            for x in 0..16 {
                assert!(
                    grid.get(CellCoord { x, y }).copied().unwrap_or(0.0) <= f32::EPSILON,
                    "expected zero past wall row at {:?}",
                    CellCoord { x, y }
                );
            }
        }
        let mut any = false;
        for y in 0..4 {
            for x in 0..16 {
                if grid.get(CellCoord { x, y }).copied().unwrap_or(0.0) > 0.0 {
                    any = true;
                }
            }
        }
        assert!(any, "expected non-zero before wall row");
    }

    fn spread_influence(src: &f32, dst: &f32) -> f32 {
        (*src * 0.5).max(*dst)
    }
}
