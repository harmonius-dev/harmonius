//! Area-of-interest helpers for relevancy-style queries.

use std::collections::HashSet;

use crate::cell_grid::CellGrid;
use crate::coord::CellCoord;

/// Unordered set of replicated entity identifiers stored in a grid cell.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EntitySet {
    ids: Vec<u32>,
}

impl EntitySet {
    /// Inserts `id` when missing.
    pub fn insert(&mut self, id: u32) {
        if !self.ids.contains(&id) {
            self.ids.push(id);
        }
    }

    /// Returns the contained ids.
    pub fn ids(&self) -> &[u32] {
        &self.ids
    }
}

/// Returns every entity id stored in cells within Chebyshev `radius` of `center`.
pub fn entities_in_radius(grid: &CellGrid<EntitySet>, center: CellCoord, radius: u32) -> Vec<u32> {
    let mut out = HashSet::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            let c = CellCoord { x, y };
            let d = chebyshev(c, center);
            if d > radius {
                continue;
            }
            if let Some(set) = grid.get(c) {
                for id in set.ids() {
                    out.insert(*id);
                }
            }
        }
    }
    out.into_iter().collect()
}

fn chebyshev(a: CellCoord, b: CellCoord) -> u32 {
    a.x.abs_diff(b.x).max(a.y.abs_diff(b.y))
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use super::*;

    #[test]
    fn test_aoi_grid_replication_filter() {
        let mut grid = CellGrid::<EntitySet>::new(64, 64, 1.0, Vec2::ZERO);
        let center = CellCoord { x: 32, y: 32 };
        let mut expected = HashSet::new();
        for id in 0u32..100 {
            let dx = (id % 11).saturating_sub(5);
            let dy = (id / 11).saturating_sub(4);
            let x = center.x.saturating_add(dx);
            let y = center.y.saturating_add(dy);
            if x >= grid.width || y >= grid.height {
                continue;
            }
            let c = CellCoord { x, y };
            if chebyshev(c, center) <= 5 {
                grid.get_mut(c).unwrap().insert(id);
                expected.insert(id);
            }
        }
        let mut got: Vec<u32> = entities_in_radius(&grid, center, 5);
        got.sort_unstable();
        let mut exp: Vec<u32> = expected.into_iter().collect();
        exp.sort_unstable();
        assert_eq!(got, exp);
    }

    #[test]
    fn test_relevancy_grid_entities_in_radius() {
        let mut grid = CellGrid::<EntitySet>::new(32, 32, 1.0, Vec2::ZERO);
        for id in 0..10 {
            let c = CellCoord { x: id, y: id };
            grid.get_mut(c).unwrap().insert(id);
        }
        let out = entities_in_radius(&grid, CellCoord { x: 5, y: 5 }, 2);
        assert!(!out.is_empty());
    }
}
