//! Tactical grid occupancy updates driven from physics body positions.

use smallvec::SmallVec;

use crate::types::{CellCoord, Entity};

/// Grid occupancy updated from physics bodies (IR-3.10.4).
#[derive(Clone, Debug, PartialEq)]
pub struct OccupancyUpdate {
    /// Affected cell.
    pub cell: CellCoord,
    /// Whether the cell should be treated as occupied.
    pub occupied: bool,
    /// Entities overlapping the cell (inline capacity 4).
    pub entities: SmallVec<[Entity; 4]>,
}

#[cfg(test)]
#[allow(missing_docs)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_3_10_u3_occupancy_inline_four_entities() {
        let mut u = OccupancyUpdate {
            cell: CellCoord { x: 0, y: 0 },
            occupied: true,
            entities: SmallVec::new(),
        };
        for id in 1..=4_u64 {
            u.entities.push(id);
        }
        assert!(!u.entities.spilled());
    }

    #[test]
    fn tc_ir_3_10_u4_occupancy_spills_at_five_entities() {
        let mut u = OccupancyUpdate {
            cell: CellCoord { x: 0, y: 0 },
            occupied: true,
            entities: SmallVec::new(),
        };
        for id in 1..=5_u64 {
            u.entities.push(id);
        }
        assert_eq!(u.entities.len(), 5);
        assert!(u.entities.spilled());
    }
}
