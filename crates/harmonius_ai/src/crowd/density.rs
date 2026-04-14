//! Uniform-grid density caps with overflow actions.

use glam::Vec2;
use smallvec::SmallVec;

use crate::steering::formation::Entity;

/// One cell in a density grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DensityCell {
    /// Current occupant count.
    pub count: u16,
    /// Maximum allowed occupants.
    pub cap: u16,
}

/// Uniform grid counting agents per cell for crowd density limits.
#[derive(Clone, Debug, PartialEq)]
pub struct DensityGrid {
    /// Row-major cells (`width` then `height`).
    pub cells: Vec<DensityCell>,
    /// Edge length of a cell in world units.
    pub cell_size: f32,
    /// Cell count along the world X axis.
    pub width: u32,
    /// Cell count along the world Z axis (stored as `Vec2::y`).
    pub height: u32,
    /// World-space origin (corner of cell `(0,0)`).
    pub origin: Vec2,
}

/// Action applied when a cell exceeds its cap.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverflowAction {
    /// Steer toward a neighbor cell with spare capacity.
    Redirect,
    /// Remove ambient agent (not modeled here).
    Despawn,
}

impl DensityGrid {
    /// Map a world XZ position into a linear cell index.
    pub fn cell_index(&self, world_pos: Vec2) -> Option<usize> {
        let local = world_pos - self.origin;
        let cx = (local.x / self.cell_size) as i64;
        let cy = (local.y / self.cell_size) as i64;
        if cx >= 0 && cy >= 0 && (cx as u32) < self.width && (cy as u32) < self.height {
            Some(((cy as u32) * self.width + (cx as u32)) as usize)
        } else {
            None
        }
    }

    /// True when `idx` is already at or above its cap.
    pub fn is_over_cap(&self, idx: usize) -> bool {
        self.cells[idx].count >= self.cells[idx].cap
    }

    /// Increment occupancy for `idx`.
    pub fn increment(&mut self, idx: usize) {
        self.cells[idx].count = self.cells[idx].count.saturating_add(1);
    }

    /// Reset all occupancy counters to zero while keeping caps.
    pub fn reset_counts(&mut self) {
        for cell in &mut self.cells {
            cell.count = 0;
        }
    }
}

/// Count agents per cell and emit overflow actions before incrementing past `cap`.
pub fn enforce_density(
    grid: &mut DensityGrid,
    agents: &[(Entity, Vec2)],
) -> SmallVec<[(Entity, OverflowAction); 64]> {
    grid.reset_counts();
    let mut overflow = SmallVec::new();
    for &(entity, pos) in agents {
        if let Some(idx) = grid.cell_index(pos) {
            if grid.is_over_cap(idx) {
                overflow.push((entity, OverflowAction::Redirect));
            } else {
                grid.increment(idx);
            }
        }
    }
    overflow
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid_with_cap(cap: u16) -> DensityGrid {
        let mut cells = Vec::new();
        cells.resize(4, DensityCell { count: 0, cap });
        DensityGrid {
            cells,
            cell_size: 1.0,
            width: 2,
            height: 2,
            origin: Vec2::ZERO,
        }
    }

    #[test]
    fn tc_7_7_6_1_density_cap_enforced() {
        let mut g = grid_with_cap(20);
        let mut agents = Vec::new();
        for i in 0..30 {
            agents.push((Entity(i), Vec2::new(0.3, 0.3)));
        }
        let over = enforce_density(&mut g, &agents);
        assert_eq!(g.cells[0].count, 20);
        assert_eq!(over.len(), 10);
    }

    #[test]
    fn tc_7_7_6_2_density_redirect() {
        let mut g = grid_with_cap(20);
        let agents: Vec<_> = (0..25).map(|i| (Entity(i), Vec2::new(0.2, 0.2))).collect();
        let over = enforce_density(&mut g, &agents);
        assert_eq!(over.len(), 5);
        assert!(over.iter().all(|(_, a)| *a == OverflowAction::Redirect));
    }
}
