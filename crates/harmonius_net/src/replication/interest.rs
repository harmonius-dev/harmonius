//! Area-of-interest filtering.
//!
//! [`filter_aoi_radius_m`] keeps the scalar test harness from TC-8.2.3.1. Grid-based subscription
//! uses [`GridCell`] + [`filter_aoi_same_cell`] as the minimal 2D cell filter toward design RF-4.

/// Discrete grid cell for AOI subscription (meters per cell in tests).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GridCell(pub i32, pub i32);

/// Entity placed on the AOI grid.
#[derive(Clone, Copy, Debug)]
pub struct EntityGridPos {
    /// Stable id.
    pub id: u32,
    /// Listener cell bucket.
    pub cell: GridCell,
}

/// Entities sharing the listener's cell (tier-zero grid AOI).
pub fn filter_aoi_same_cell(entities: &[EntityGridPos], listener_cell: GridCell) -> Vec<u32> {
    let mut ids: Vec<u32> = entities
        .iter()
        .filter(|e| e.cell == listener_cell)
        .map(|e| e.id)
        .collect();
    ids.sort_unstable();
    ids
}

/// Test entity id + planar distance in meters from origin.
#[derive(Clone, Copy, Debug)]
pub struct EntityPos {
    /// Id.
    pub id: u32,
    /// Distance from listener along one axis (test uses 10..1000 step 10).
    pub distance_m: f32,
}

/// Returns entity ids within `radius_m` of origin (inclusive).
pub fn filter_aoi_radius_m(entities: &[EntityPos], radius_m: f32) -> Vec<u32> {
    entities
        .iter()
        .filter(|e| e.distance_m <= radius_m)
        .map(|e| e.id)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.2.3.1 — only entities within 50 m.
    #[test]
    fn test_aoi_radius_filter() {
        let ents: Vec<EntityPos> = (1..=100)
            .map(|i| EntityPos {
                id: i,
                distance_m: (i * 10) as f32,
            })
            .collect();
        let in_aoi = filter_aoi_radius_m(&ents, 50.0);
        assert_eq!(in_aoi, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_aoi_grid_same_cell() {
        let listener = GridCell(0, 0);
        let ents = vec![
            EntityGridPos {
                id: 1,
                cell: GridCell(0, 0),
            },
            EntityGridPos {
                id: 2,
                cell: GridCell(1, 0),
            },
            EntityGridPos {
                id: 3,
                cell: GridCell(0, 0),
            },
        ];
        assert_eq!(filter_aoi_same_cell(&ents, listener), vec![1, 3]);
    }
}
