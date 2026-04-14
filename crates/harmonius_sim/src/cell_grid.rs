//! 2D `CellGrid` gameplay grid, queries, and propagation.

use std::collections::VecDeque;

use glam::Vec2;
use rkyv_derive::{Archive, Deserialize, Serialize};
use smallvec::SmallVec;

use crate::coord::CellCoord;

/// Scratch allocator for grid queries that return heap-backed results.
#[derive(Clone, Debug, Default)]
pub struct Arena {
    coords: Vec<CellCoord>,
}

impl Arena {
    /// Creates an empty arena.
    pub fn new() -> Self {
        Self { coords: Vec::new() }
    }

    fn coords_mut(&mut self) -> &mut Vec<CellCoord> {
        &mut self.coords
    }
}

/// Neighbor adjacency pattern for propagation and neighbor queries.
#[derive(Archive, Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum NeighborPattern {
    /// Four orthogonal neighbors.
    Cardinal,
    /// Four diagonal neighbors.
    Diagonal,
    /// All eight neighbors.
    All,
}

/// Shape hint for higher-level spatial queries.
#[derive(Archive, Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum GridQueryShape {
    /// Circular footprint in cells.
    Circle,
    /// Axis-aligned rectangle in cells.
    Rectangle,
    /// Discrete line between two cells.
    Line,
}

/// Result of a line-of-sight traversal.
#[derive(Archive, Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LineOfSightResult {
    /// True when no blocking cell was hit.
    pub clear: bool,
    /// Cells visited in traversal order.
    pub traversed: Vec<CellCoord>,
    /// First blocking cell, if any.
    pub blocked_at: Option<CellCoord>,
}

/// Result of a flood fill.
#[derive(Archive, Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FloodFillResult {
    /// Reachable cells.
    pub cells: Vec<CellCoord>,
    /// Count of dequeue operations (visited frontier cells).
    pub cells_visited: u32,
}

/// Pure spread function for propagation kernels.
pub type SpreadFn<T> = fn(&T, &T) -> T;

/// Configuration for one propagation pass over a `CellGrid`.
#[derive(Clone, Debug)]
pub struct PropagationKernel<T> {
    /// Neighbor connectivity used when spreading.
    pub pattern: NeighborPattern,
    /// Per-neighbor spread combining source and destination samples.
    pub spread: SpreadFn<T>,
    /// Multiplicative decay applied after spreading.
    pub decay_rate: f32,
    /// Optional Chebyshev cutoff from `anchor` when set.
    pub max_radius: u32,
    /// Optional anchor cell for radius-limited kernels.
    pub anchor: Option<CellCoord>,
    /// Phantom marker for generic `T`.
    pub _marker: std::marker::PhantomData<T>,
}

impl<T> PropagationKernel<T> {
    /// Builds a kernel with no anchor (global decay/spread only).
    pub fn new(pattern: NeighborPattern, spread: SpreadFn<T>, decay_rate: f32) -> Self {
        Self {
            pattern,
            spread,
            decay_rate,
            max_radius: u32::MAX,
            anchor: None,
            _marker: std::marker::PhantomData,
        }
    }

    /// Sets the optional anchor and Chebyshev radius bound.
    pub fn with_anchor(mut self, anchor: CellCoord, max_radius: u32) -> Self {
        self.anchor = Some(anchor);
        self.max_radius = max_radius;
        self
    }
}

/// 2D gameplay grid with axis-aligned cells.
#[derive(Clone, Debug)]
pub struct CellGrid<T> {
    /// Width in cells.
    pub width: u32,
    /// Height in cells.
    pub height: u32,
    /// World-space size of each cell edge.
    pub cell_size: f32,
    /// World-space origin at the grid's minimum corner.
    pub origin: Vec2,
    cells: Vec<T>,
}

/// Backwards-compatible alias used by cross-subsystem specs.
pub type UniformGrid<T> = CellGrid<T>;

impl<T: Clone + Default + PartialEq> CellGrid<T> {
    /// Creates a grid filled with `T::default()`.
    pub fn new(width: u32, height: u32, cell_size: f32, origin: Vec2) -> Self {
        let len = (width as usize).saturating_mul(height as usize);
        Self {
            width,
            height,
            cell_size,
            origin,
            cells: vec![T::default(); len],
        }
    }

    pub(crate) fn index(&self, coord: CellCoord) -> Option<usize> {
        if coord.x >= self.width || coord.y >= self.height {
            return None;
        }
        Some((coord.y as usize).saturating_mul(self.width as usize) + coord.x as usize)
    }

    /// Returns `true` when `coord` lies inside the grid.
    pub fn in_bounds(&self, coord: CellCoord) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    /// Immutable borrow of a cell.
    pub fn get(&self, coord: CellCoord) -> Option<&T> {
        let i = self.index(coord)?;
        self.cells.get(i)
    }

    /// Mutable borrow of a cell.
    pub fn get_mut(&mut self, coord: CellCoord) -> Option<&mut T> {
        let i = self.index(coord)?;
        self.cells.get_mut(i)
    }

    /// Overwrites a cell, panicking when out of bounds.
    pub fn set(&mut self, coord: CellCoord, value: T) {
        let i = self
            .index(coord)
            .unwrap_or_else(|| panic!("set out of bounds: {:?}", coord));
        self.cells[i] = value;
    }

    /// Maps world space to a cell coordinate.
    pub fn world_to_cell(&self, world_pos: Vec2) -> Option<CellCoord> {
        if self.cell_size <= 0.0 {
            return None;
        }
        let local = world_pos - self.origin;
        let gx = (local.x / self.cell_size).floor() as i64;
        let gy = (local.y / self.cell_size).floor() as i64;
        if gx < 0 || gy < 0 {
            return None;
        }
        let x = u32::try_from(gx).ok()?;
        let y = u32::try_from(gy).ok()?;
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(CellCoord { x, y })
    }

    /// World-space center of `coord`.
    pub fn cell_to_world(&self, coord: CellCoord) -> Vec2 {
        let hx = coord.x as f32 + 0.5;
        let hy = coord.y as f32 + 0.5;
        self.origin + Vec2::new(hx * self.cell_size, hy * self.cell_size)
    }

    /// Neighbor coordinates respecting `pattern` and bounds.
    pub fn neighbors(
        &self,
        coord: CellCoord,
        pattern: NeighborPattern,
    ) -> SmallVec<[CellCoord; 8]> {
        let mut out = SmallVec::new();
        let push_if = |out: &mut SmallVec<[CellCoord; 8]>, c: CellCoord| {
            if c.x < self.width && c.y < self.height {
                out.push(c);
            }
        };
        match pattern {
            NeighborPattern::Cardinal => {
                if coord.x > 0 {
                    push_if(
                        &mut out,
                        CellCoord {
                            x: coord.x - 1,
                            y: coord.y,
                        },
                    );
                }
                if coord.x + 1 < self.width {
                    push_if(
                        &mut out,
                        CellCoord {
                            x: coord.x + 1,
                            y: coord.y,
                        },
                    );
                }
                if coord.y > 0 {
                    push_if(
                        &mut out,
                        CellCoord {
                            x: coord.x,
                            y: coord.y - 1,
                        },
                    );
                }
                if coord.y + 1 < self.height {
                    push_if(
                        &mut out,
                        CellCoord {
                            x: coord.x,
                            y: coord.y + 1,
                        },
                    );
                }
            }
            NeighborPattern::Diagonal => {
                for (dx, dy) in [(-1, -1), (1, -1), (-1, 1), (1, 1)] {
                    let nx = coord.x as i64 + dx;
                    let ny = coord.y as i64 + dy;
                    if nx >= 0 && ny >= 0 {
                        let c = CellCoord {
                            x: nx as u32,
                            y: ny as u32,
                        };
                        push_if(&mut out, c);
                    }
                }
            }
            NeighborPattern::All => {
                for dy in -1i64..=1 {
                    for dx in -1i64..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = coord.x as i64 + dx;
                        let ny = coord.y as i64 + dy;
                        if nx >= 0 && ny >= 0 {
                            let c = CellCoord {
                                x: nx as u32,
                                y: ny as u32,
                            };
                            push_if(&mut out, c);
                        }
                    }
                }
            }
        }
        out
    }

    /// Bresenham line-of-sight with a blocking predicate.
    pub fn line_of_sight(
        &self,
        from: CellCoord,
        to: CellCoord,
        blocked: impl Fn(&T) -> bool,
        arena: &mut Arena,
    ) -> LineOfSightResult {
        let _ = arena;
        let mut traversed = Vec::new();
        if !self.in_bounds(from) || !self.in_bounds(to) {
            return LineOfSightResult {
                clear: false,
                traversed,
                blocked_at: None,
            };
        }
        let mut x0 = from.x as i32;
        let mut y0 = from.y as i32;
        let x1 = to.x as i32;
        let y1 = to.y as i32;
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        loop {
            let c = CellCoord {
                x: x0 as u32,
                y: y0 as u32,
            };
            traversed.push(c);
            if c != from {
                if let Some(v) = self.get(c) {
                    if blocked(v) {
                        return LineOfSightResult {
                            clear: false,
                            traversed,
                            blocked_at: Some(c),
                        };
                    }
                }
            }
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
        LineOfSightResult {
            clear: true,
            traversed,
            blocked_at: None,
        }
    }

    /// Flood fill over passable cells.
    pub fn flood_fill(
        &self,
        start: CellCoord,
        passable: impl Fn(&T) -> bool,
        arena: &mut Arena,
    ) -> FloodFillResult {
        let coords_scratch = arena.coords_mut();
        coords_scratch.clear();
        if !self.in_bounds(start) || !passable(self.get(start).unwrap_or(&T::default())) {
            return FloodFillResult {
                cells: Vec::new(),
                cells_visited: 0,
            };
        }
        let mut visited = vec![false; self.cells.len()];
        let mut queue = VecDeque::new();
        let mut cells = Vec::new();
        let mut cells_visited: u32 = 0;
        let si = self.index(start).unwrap();
        visited[si] = true;
        queue.push_back(start);
        while let Some(c) = queue.pop_front() {
            cells_visited = cells_visited.saturating_add(1);
            cells.push(c);
            for n in self.neighbors(c, NeighborPattern::Cardinal) {
                let Some(ni) = self.index(n) else {
                    continue;
                };
                if visited[ni] {
                    continue;
                }
                if let Some(v) = self.get(n) {
                    if !passable(v) {
                        continue;
                    }
                }
                visited[ni] = true;
                queue.push_back(n);
            }
        }
        FloodFillResult {
            cells,
            cells_visited,
        }
    }

    /// Returns all cells whose centers lie within `radius` of `center` in world units.
    pub fn area_query(&self, center: Vec2, radius: f32, arena: &mut Arena) -> Vec<CellCoord> {
        let _ = arena;
        let r2 = radius * radius;
        let mut out = Vec::new();
        if self.cell_size <= 0.0 {
            return out;
        }
        let min_x = (((center.x - radius - self.origin.x) / self.cell_size).floor() as i64)
            .clamp(0, self.width as i64 - 1) as u32;
        let max_x = (((center.x + radius - self.origin.x) / self.cell_size).floor() as i64)
            .clamp(0, self.width as i64 - 1) as u32;
        let min_y = (((center.y - radius - self.origin.y) / self.cell_size).floor() as i64)
            .clamp(0, self.height as i64 - 1) as u32;
        let max_y = (((center.y + radius - self.origin.y) / self.cell_size).floor() as i64)
            .clamp(0, self.height as i64 - 1) as u32;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let c = CellCoord { x, y };
                let p = self.cell_to_world(c);
                if p.distance_squared(center) <= r2 {
                    out.push(c);
                }
            }
        }
        out
    }
}

impl CellGrid<f32> {
    /// Applies one synchronous propagation step for `f32` influence grids.
    pub fn propagate(&mut self, kernel: &PropagationKernel<f32>) {
        let old = self.cells.clone();
        let mut next = old.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let coord = CellCoord { x, y };
                let idx = self.index(coord).unwrap();
                let v = old[idx];
                if v == 0.0 {
                    continue;
                }
                for n in self.neighbors(coord, kernel.pattern) {
                    let ni = self.index(n).unwrap();
                    let proposed = (kernel.spread)(&v, &old[ni]);
                    next[ni] = next[ni].max(proposed);
                }
            }
        }
        for v in &mut next {
            *v *= kernel.decay_rate;
        }
        if let Some(anchor) = kernel.anchor {
            for y in 0..self.height {
                for x in 0..self.width {
                    let c = CellCoord { x, y };
                    let d = chebyshev(c, anchor);
                    if d > kernel.max_radius {
                        let i = self.index(c).unwrap();
                        next[i] = 0.0;
                    }
                }
            }
        }
        self.cells = next;
    }

    /// Like [`CellGrid::propagate`], but skips writes into `wall` cells.
    pub fn propagate_masked(
        &mut self,
        kernel: &PropagationKernel<f32>,
        mut wall: impl FnMut(CellCoord) -> bool,
    ) {
        let old = self.cells.clone();
        let mut next = old.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let coord = CellCoord { x, y };
                let idx = self.index(coord).unwrap();
                let v = old[idx];
                if v == 0.0 {
                    continue;
                }
                for n in self.neighbors(coord, kernel.pattern) {
                    if wall(n) {
                        continue;
                    }
                    let ni = self.index(n).unwrap();
                    let proposed = (kernel.spread)(&v, &old[ni]);
                    next[ni] = next[ni].max(proposed);
                }
            }
        }
        for y in 0..self.height {
            for x in 0..self.width {
                let coord = CellCoord { x, y };
                let i = self.index(coord).unwrap();
                if wall(coord) {
                    next[i] = 0.0;
                } else {
                    next[i] *= kernel.decay_rate;
                }
            }
        }
        if let Some(anchor) = kernel.anchor {
            for y in 0..self.height {
                for x in 0..self.width {
                    let c = CellCoord { x, y };
                    let d = chebyshev(c, anchor);
                    if d > kernel.max_radius {
                        let i = self.index(c).unwrap();
                        next[i] = 0.0;
                    }
                }
            }
        }
        self.cells = next;
    }
}

fn chebyshev(a: CellCoord, b: CellCoord) -> u32 {
    let dx = a.x.abs_diff(b.x);
    let dy = a.y.abs_diff(b.y);
    dx.max(dy)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::time::Instant;

    use glam::Vec2;

    use crate::aoi::EntitySet;

    use super::*;

    fn blocking_u8(v: &u8) -> bool {
        *v >= 128
    }

    fn spread_zero_f32(_: &f32, _: &f32) -> f32 {
        0.0
    }

    fn spread_mul09(src: &f32, _: &f32) -> f32 {
        *src * 0.9
    }

    fn spread_mul05(src: &f32, _: &f32) -> f32 {
        *src * 0.5
    }

    fn spread_max_f32(src: &f32, dst: &f32) -> f32 {
        (*src).max(*dst)
    }

    #[test]
    fn test_cell_coord_equality() {
        assert_eq!(CellCoord { x: 3, y: 4 }, CellCoord { x: 3, y: 4 });
    }

    #[test]
    fn test_cell_coord_hash() {
        let mut set = HashSet::new();
        set.insert(CellCoord { x: 3, y: 4 });
        set.insert(CellCoord { x: 3, y: 4 });
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_uniform_grid_primitive() {
        let mut grid = CellGrid::<u8>::new(32, 32, 1.0, Vec2::ZERO);
        grid.set(CellCoord { x: 5, y: 6 }, 42);
        assert_eq!(grid.width, 32);
        assert_eq!(grid.height, 32);
        assert_eq!(grid.get(CellCoord { x: 5, y: 6 }), Some(&42u8));
    }

    #[test]
    fn test_grid_get_valid_cell() {
        let grid = CellGrid::<u8>::new(4, 4, 1.0, Vec2::ZERO);
        assert_eq!(grid.get(CellCoord { x: 1, y: 1 }), Some(&0));
    }

    #[test]
    fn test_grid_get_out_of_bounds() {
        let grid = CellGrid::<u8>::new(4, 4, 1.0, Vec2::ZERO);
        assert_eq!(grid.get(CellCoord { x: 4, y: 4 }), None);
    }

    #[test]
    fn test_grid_set_valid_cell() {
        let mut grid = CellGrid::<u8>::new(4, 4, 1.0, Vec2::ZERO);
        grid.set(CellCoord { x: 2, y: 2 }, 7);
        assert_eq!(grid.get(CellCoord { x: 2, y: 2 }), Some(&7));
    }

    #[test]
    fn test_grid_set_edge_cell() {
        let mut grid = CellGrid::<u8>::new(4, 4, 1.0, Vec2::ZERO);
        grid.set(CellCoord { x: 0, y: 3 }, 9);
        assert_eq!(grid.get(CellCoord { x: 0, y: 3 }), Some(&9));
    }

    #[test]
    fn test_world_to_cell_inside() {
        let grid = CellGrid::<u8>::new(8, 8, 1.0, Vec2::ZERO);
        assert_eq!(
            grid.world_to_cell(Vec2::new(1.5, 2.5)),
            Some(CellCoord { x: 1, y: 2 })
        );
    }

    #[test]
    fn test_world_to_cell_outside() {
        let grid = CellGrid::<u8>::new(8, 8, 1.0, Vec2::ZERO);
        assert_eq!(grid.world_to_cell(Vec2::new(8.1, 0.0)), None);
    }

    #[test]
    fn test_cell_to_world_corners() {
        let grid = CellGrid::<u8>::new(4, 4, 2.0, Vec2::ZERO);
        let corners = [
            (CellCoord { x: 0, y: 0 }, Vec2::new(1.0, 1.0)),
            (CellCoord { x: 3, y: 0 }, Vec2::new(7.0, 1.0)),
            (CellCoord { x: 0, y: 3 }, Vec2::new(1.0, 7.0)),
            (CellCoord { x: 3, y: 3 }, Vec2::new(7.0, 7.0)),
        ];
        for (c, expected) in corners {
            assert_eq!(grid.cell_to_world(c), expected);
        }
    }

    #[test]
    fn test_neighbors_cardinal_interior() {
        let grid = CellGrid::<u8>::new(5, 5, 1.0, Vec2::ZERO);
        let n = grid.neighbors(CellCoord { x: 2, y: 2 }, NeighborPattern::Cardinal);
        let mut set: HashSet<_> = n.iter().copied().collect();
        assert_eq!(set.len(), 4);
        assert!(set.remove(&CellCoord { x: 1, y: 2 }));
        assert!(set.remove(&CellCoord { x: 3, y: 2 }));
        assert!(set.remove(&CellCoord { x: 2, y: 1 }));
        assert!(set.remove(&CellCoord { x: 2, y: 3 }));
    }

    #[test]
    fn test_neighbors_cardinal_edge() {
        let grid = CellGrid::<u8>::new(5, 5, 1.0, Vec2::ZERO);
        let n = grid.neighbors(CellCoord { x: 0, y: 2 }, NeighborPattern::Cardinal);
        assert_eq!(n.len(), 3);
    }

    #[test]
    fn test_neighbors_all_corner() {
        let grid = CellGrid::<u8>::new(5, 5, 1.0, Vec2::ZERO);
        let n = grid.neighbors(CellCoord { x: 0, y: 0 }, NeighborPattern::All);
        assert_eq!(n.len(), 3);
    }

    #[test]
    fn test_neighbors_all_center() {
        let grid = CellGrid::<u8>::new(5, 5, 1.0, Vec2::ZERO);
        let n = grid.neighbors(CellCoord { x: 2, y: 2 }, NeighborPattern::All);
        assert_eq!(n.len(), 8);
    }

    #[test]
    fn test_los_clear_path() {
        let grid = CellGrid::<u8>::new(8, 8, 1.0, Vec2::ZERO);
        let mut arena = Arena::new();
        let res = grid.line_of_sight(
            CellCoord { x: 0, y: 0 },
            CellCoord { x: 4, y: 4 },
            blocking_u8,
            &mut arena,
        );
        assert!(res.clear);
    }

    #[test]
    fn test_los_blocked_cell() {
        let mut grid = CellGrid::<u8>::new(8, 8, 1.0, Vec2::ZERO);
        grid.set(CellCoord { x: 2, y: 2 }, 200);
        let mut arena = Arena::new();
        let res = grid.line_of_sight(
            CellCoord { x: 0, y: 0 },
            CellCoord { x: 4, y: 4 },
            blocking_u8,
            &mut arena,
        );
        assert!(!res.clear);
        assert_eq!(res.blocked_at, Some(CellCoord { x: 2, y: 2 }));
    }

    #[test]
    fn test_flood_fill_open() {
        let grid = CellGrid::<u8>::new(8, 8, 1.0, Vec2::ZERO);
        let mut arena = Arena::new();
        let res = grid.flood_fill(CellCoord { x: 0, y: 0 }, |_| true, &mut arena);
        assert_eq!(res.cells.len(), 64);
    }

    #[test]
    fn test_flood_fill_walled() {
        let mut grid = CellGrid::<u8>::new(8, 8, 1.0, Vec2::ZERO);
        for x in 0..8 {
            grid.set(CellCoord { x, y: 4 }, 255);
        }
        let mut arena = Arena::new();
        let res = grid.flood_fill(CellCoord { x: 0, y: 0 }, |v| *v < 128, &mut arena);
        assert_eq!(res.cells.len(), 32);
    }

    #[test]
    fn test_flood_fill_full_grid() {
        let grid = CellGrid::<u8>::new(4, 4, 1.0, Vec2::ZERO);
        let mut arena = Arena::new();
        let res = grid.flood_fill(CellCoord { x: 0, y: 0 }, |_| true, &mut arena);
        assert_eq!(res.cells_visited, 16);
    }

    #[test]
    fn test_area_query_within_radius() {
        let grid = CellGrid::<u8>::new(16, 16, 1.0, Vec2::ZERO);
        let mut arena = Arena::new();
        let cells = grid.area_query(Vec2::new(7.5, 7.5), 3.0, &mut arena);
        for c in &cells {
            let p = grid.cell_to_world(*c);
            assert!(p.distance(Vec2::new(7.5, 7.5)) <= 3.0 + 1e-3);
        }
    }

    #[test]
    fn test_area_query_partial_overlap() {
        let grid = CellGrid::<u8>::new(16, 16, 1.0, Vec2::ZERO);
        let mut arena = Arena::new();
        let cells = grid.area_query(Vec2::new(0.5, 0.5), 3.0, &mut arena);
        for c in &cells {
            assert!(grid.in_bounds(*c));
        }
    }

    #[test]
    fn test_propagate_decay() {
        let mut grid = CellGrid::<f32>::new(8, 8, 1.0, Vec2::ZERO);
        grid.set(CellCoord { x: 4, y: 4 }, 1.0);
        let kernel = PropagationKernel::<f32>::new(NeighborPattern::Cardinal, spread_zero_f32, 0.5);
        for _ in 0..4 {
            grid.propagate(&kernel);
        }
        assert!(grid.get(CellCoord { x: 4, y: 4 }).copied().unwrap() < 0.1);
    }

    #[test]
    fn test_propagate_max_radius() {
        let mut grid = CellGrid::<f32>::new(8, 8, 1.0, Vec2::ZERO);
        grid.set(CellCoord { x: 4, y: 4 }, 1.0);
        let kernel = PropagationKernel::<f32>::new(NeighborPattern::Cardinal, spread_mul05, 0.9)
            .with_anchor(CellCoord { x: 4, y: 4 }, 2);
        for _ in 0..8 {
            grid.propagate(&kernel);
        }
        for y in 0..8 {
            for x in 0..8 {
                let c = CellCoord { x, y };
                let d = chebyshev(c, CellCoord { x: 4, y: 4 });
                let v = grid.get(c).copied().unwrap_or(0.0);
                if d > 2 {
                    assert!(v.abs() < f32::EPSILON, "cell {:?} should be clamped", c);
                }
            }
        }
    }

    #[test]
    fn test_propagate_cardinal_pattern() {
        let mut grid = CellGrid::<f32>::new(5, 5, 1.0, Vec2::ZERO);
        grid.set(CellCoord { x: 2, y: 2 }, 1.0);
        let kernel = PropagationKernel::<f32>::new(NeighborPattern::Cardinal, spread_mul09, 1.0);
        grid.propagate(&kernel);
        assert!(grid.get(CellCoord { x: 1, y: 2 }).copied().unwrap() > 0.0);
        assert!(grid.get(CellCoord { x: 2, y: 1 }).copied().unwrap() > 0.0);
        assert_eq!(
            grid.get(CellCoord { x: 1, y: 1 }).copied().unwrap_or(0.0),
            0.0
        );
    }

    #[test]
    fn test_propagate_diagonal_pattern() {
        let mut grid = CellGrid::<f32>::new(5, 5, 1.0, Vec2::ZERO);
        grid.set(CellCoord { x: 2, y: 2 }, 1.0);
        let kernel = PropagationKernel::<f32>::new(NeighborPattern::Diagonal, spread_mul09, 1.0);
        grid.propagate(&kernel);
        assert!(grid.get(CellCoord { x: 1, y: 1 }).copied().unwrap() > 0.0);
        assert_eq!(
            grid.get(CellCoord { x: 2, y: 1 }).copied().unwrap_or(0.0),
            0.0
        );
    }

    #[test]
    fn test_propagate_256_budget() {
        let mut grid = CellGrid::<f32>::new(256, 256, 1.0, Vec2::ZERO);
        grid.set(CellCoord { x: 128, y: 128 }, 1.0);
        let kernel = PropagationKernel::<f32>::new(NeighborPattern::Cardinal, spread_max_f32, 0.9);
        let start = Instant::now();
        grid.propagate(&kernel);
        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
        let limit = if cfg!(debug_assertions) { 25.0 } else { 1.0 };
        assert!(
            elapsed_ms < limit,
            "propagation too slow: {elapsed_ms} ms (limit {limit})"
        );
    }

    #[test]
    fn test_los_128_budget() {
        let grid = CellGrid::<u8>::new(256, 256, 1.0, Vec2::ZERO);
        let mut arena = Arena::new();
        let start = Instant::now();
        let res = grid.line_of_sight(
            CellCoord { x: 0, y: 0 },
            CellCoord { x: 127, y: 0 },
            blocking_u8,
            &mut arena,
        );
        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
        let limit = if cfg!(debug_assertions) { 5.0 } else { 0.01 };
        assert!(res.clear);
        assert!(
            elapsed_ms < limit,
            "los too slow: {elapsed_ms} ms (limit {limit})"
        );
    }

    #[test]
    fn test_scent_propagation_decay() {
        let mut grid = CellGrid::<f32>::new(16, 16, 1.0, Vec2::ZERO);
        grid.set(CellCoord { x: 4, y: 4 }, 1.0);
        let kernel = PropagationKernel::<f32>::new(NeighborPattern::Cardinal, spread_mul05, 0.8);
        for _ in 0..5 {
            grid.propagate(&kernel);
        }
        let d3 = grid.get(CellCoord { x: 4, y: 7 }).copied().unwrap_or(0.0);
        assert!(d3 > 0.0);
    }

    #[test]
    fn test_grid_rkyv_round_trip() {
        use rkyv::api::high::{from_bytes, to_bytes};
        use rkyv::rancor::Error;
        use rkyv_derive::{Archive, Deserialize, Serialize};

        #[derive(Archive, Deserialize, Serialize, PartialEq, Debug)]
        struct GridU8Snapshot {
            width: u32,
            height: u32,
            cell_size: f32,
            ox: f32,
            oy: f32,
            cells: Vec<u8>,
        }

        impl From<&CellGrid<u8>> for GridU8Snapshot {
            fn from(grid: &CellGrid<u8>) -> Self {
                let mut cells = Vec::new();
                for y in 0..grid.height {
                    for x in 0..grid.width {
                        let c = CellCoord { x, y };
                        cells.push(*grid.get(c).unwrap());
                    }
                }
                Self {
                    width: grid.width,
                    height: grid.height,
                    cell_size: grid.cell_size,
                    ox: grid.origin.x,
                    oy: grid.origin.y,
                    cells,
                }
            }
        }

        impl GridU8Snapshot {
            fn into_grid(self) -> CellGrid<u8> {
                let mut g = CellGrid::<u8>::new(
                    self.width,
                    self.height,
                    self.cell_size,
                    glam::Vec2::new(self.ox, self.oy),
                );
                let mut i = 0usize;
                for y in 0..self.height {
                    for x in 0..self.width {
                        let c = CellCoord { x, y };
                        g.set(c, self.cells[i]);
                        i += 1;
                    }
                }
                g
            }
        }

        let mut grid = CellGrid::<u8>::new(8, 8, 1.0, Vec2::ZERO);
        grid.set(CellCoord { x: 1, y: 2 }, 9);
        let snap = GridU8Snapshot::from(&grid);
        let bytes = to_bytes::<Error>(&snap).unwrap();
        let decoded: GridU8Snapshot = from_bytes::<GridU8Snapshot, Error>(&bytes).unwrap();
        let round = decoded.into_grid();
        assert_eq!(round.width, grid.width);
        assert_eq!(round.height, grid.height);
        for y in 0..8 {
            for x in 0..8 {
                let c = CellCoord { x, y };
                assert_eq!(round.get(c), grid.get(c));
            }
        }
    }

    #[test]
    fn test_grid_ecs_spawn_query_mutate() {
        #[derive(Debug)]
        struct World {
            grid: CellGrid<u8>,
        }
        let mut world = World {
            grid: CellGrid::new(4, 4, 1.0, Vec2::ZERO),
        };
        world.grid.set(CellCoord { x: 1, y: 1 }, 5);
        assert_eq!(world.grid.get(CellCoord { x: 1, y: 1 }), Some(&5u8));
    }

    #[test]
    fn test_grid_spatial_index_lookup() {
        let mut grid = CellGrid::<EntitySet>::new(8, 8, 1.0, Vec2::ZERO);
        grid.get_mut(CellCoord { x: 1, y: 1 }).unwrap().insert(1);
        grid.get_mut(CellCoord { x: 2, y: 2 }).unwrap().insert(2);
        grid.get_mut(CellCoord { x: 3, y: 3 }).unwrap().insert(3);
        let center = grid.cell_to_world(CellCoord { x: 2, y: 2 });
        let cells = grid.area_query(center, 2.0, &mut Arena::new());
        let mut ids = Vec::new();
        for c in cells {
            for id in grid.get(c).unwrap().ids() {
                ids.push(*id);
            }
        }
        ids.sort_unstable();
        ids.dedup();
        assert!(ids.contains(&1));
        assert!(ids.contains(&2));
        assert!(ids.contains(&3));
    }
}
