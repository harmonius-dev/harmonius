//! Hierarchical multi-resolution grids.

use glam::Vec2;

use crate::cell_grid::CellGrid;
use crate::coord::CellCoord;

/// Coarse + fine `CellGrid` pair with deterministic aggregation.
#[derive(Clone, Debug)]
pub struct HierarchicalGrid<T: Clone + Default + PartialEq> {
    fine: CellGrid<T>,
    coarse: CellGrid<T>,
    fine_per_coarse: u32,
}

impl<T: Clone + Default + PartialEq> HierarchicalGrid<T> {
    /// Builds a hierarchy where each coarse cell covers `fine_per_coarse` fine cells per axis.
    pub fn new(
        coarse_width: u32,
        coarse_height: u32,
        fine_per_coarse: u32,
        fine_cell_size: f32,
        fine_origin: Vec2,
    ) -> Self {
        assert!(fine_per_coarse > 0);
        let fine_w = coarse_width * fine_per_coarse;
        let fine_h = coarse_height * fine_per_coarse;
        Self {
            fine: CellGrid::new(fine_w, fine_h, fine_cell_size, fine_origin),
            coarse: CellGrid::new(coarse_width, coarse_height, fine_cell_size, fine_origin),
            fine_per_coarse,
        }
    }

    /// Writes a fine cell and refreshes the owning coarse aggregate.
    pub fn set_fine(&mut self, coord: CellCoord, value: T) {
        self.fine.set(coord, value.clone());
        let cx = coord.x / self.fine_per_coarse;
        let cy = coord.y / self.fine_per_coarse;
        self.refresh_coarse(CellCoord { x: cx, y: cy });
    }

    /// Reads the coarse aggregate cell.
    pub fn coarse_get(&self, coord: CellCoord) -> Option<&T> {
        self.coarse.get(coord)
    }

    fn refresh_coarse(&mut self, coarse_cell: CellCoord) {
        let mut acc = T::default();
        let base_x = coarse_cell.x * self.fine_per_coarse;
        let base_y = coarse_cell.y * self.fine_per_coarse;
        for dy in 0..self.fine_per_coarse {
            for dx in 0..self.fine_per_coarse {
                let c = CellCoord {
                    x: base_x + dx,
                    y: base_y + dy,
                };
                if let Some(v) = self.fine.get(c) {
                    if *v != T::default() {
                        acc = v.clone();
                    }
                }
            }
        }
        self.coarse.set(coarse_cell, acc);
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use super::*;

    #[test]
    fn test_hierarchical_grid_multires() {
        let mut h = HierarchicalGrid::<u8>::new(16, 16, 16, 1.0, Vec2::ZERO);
        h.set_fine(CellCoord { x: 10, y: 10 }, 1);
        let cx = 10 / 16;
        let cy = 10 / 16;
        assert_eq!(h.coarse_get(CellCoord { x: cx, y: cy }), Some(&1u8));
    }
}
