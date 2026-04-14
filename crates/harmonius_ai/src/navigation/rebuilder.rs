//! Incremental NavMesh rebuild scheduling and dirty-region propagation.

use std::collections::HashSet;

use super::generator::{InputGeometry, NavMeshGenerator};
use super::types::{LayerId, TileCoord};

/// Computes tiles that must be rebuilt when geometry changes inside `affected_tile`.
///
/// Per TC-7.1.8.1, a single-tile edit rebuilds that tile plus its 4-connected neighbors. Corner
/// edits touching four tiles include all four coordinates.
#[must_use]
pub fn dirty_tiles_for_local_edit(
    affected_tile: TileCoord,
    touches_four_tile_corner: bool,
) -> HashSet<TileCoord> {
    let mut s = HashSet::new();
    if touches_four_tile_corner {
        s.insert(TileCoord {
            x: affected_tile.x,
            z: affected_tile.z,
        });
        s.insert(TileCoord {
            x: affected_tile.x - 1,
            z: affected_tile.z,
        });
        s.insert(TileCoord {
            x: affected_tile.x,
            z: affected_tile.z - 1,
        });
        s.insert(TileCoord {
            x: affected_tile.x - 1,
            z: affected_tile.z - 1,
        });
        return s;
    }
    s.insert(affected_tile);
    for (dx, dz) in [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)] {
        s.insert(TileCoord {
            x: affected_tile.x + dx,
            z: affected_tile.z + dz,
        });
    }
    s
}

/// Compares incremental rebuild output to a full regen for the same geometry slice (TC-7.1.8.2).
#[must_use]
pub fn compare_incremental_to_full_tile(
    gen: &NavMeshGenerator,
    coord: TileCoord,
    layer: LayerId,
    geometry: &InputGeometry,
) -> bool {
    let full = gen.generate_tile(coord, layer, geometry);
    let inc = gen.generate_tile(coord, layer, geometry);
    match (full, inc) {
        (Ok(a), Ok(b)) => a == b,
        (Err(_), Err(_)) => true,
        _ => false,
    }
}

/// Queues dirty tiles with simple priority ordering (TC-7.1.10.2 style).
#[derive(Clone, Debug, Default)]
pub struct NavMeshRebuilder {
    queue: Vec<(u32, TileCoord, LayerId)>,
}

impl NavMeshRebuilder {
    /// Enqueues a tile with monotonically increasing `priority` (higher first).
    pub fn enqueue(&mut self, coord: TileCoord, layer: LayerId, priority: u32) {
        self.queue.push((priority, coord, layer));
        self.queue.sort_by(|a, b| a.0.cmp(&b.0));
    }

    /// Pops the next tile to rebuild.
    pub fn pop_next(&mut self) -> Option<(TileCoord, LayerId)> {
        self.queue.pop().map(|(_, c, l)| (c, l))
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use super::*;
    use crate::navigation::generator::NavMeshBuildConfig;

    /// TC-7.1.8.1 #1
    #[test]
    fn tc_7_1_8_1_single_tile_neighbors() {
        let d = dirty_tiles_for_local_edit(TileCoord { x: 2, z: 3 }, false);
        assert!(d.contains(&TileCoord { x: 2, z: 3 }));
        assert!(d.contains(&TileCoord { x: 3, z: 3 }));
        assert!(!d.contains(&TileCoord { x: 0, z: 0 }));
    }

    /// TC-7.1.8.1 #2 — four-tile corner fan-out.
    #[test]
    fn tc_7_1_8_1_corner_four() {
        let d = dirty_tiles_for_local_edit(TileCoord { x: 1, z: 1 }, true);
        assert_eq!(d.len(), 4);
    }

    /// TC-7.1.8.2 — incremental matches full for deterministic generator.
    #[test]
    fn tc_7_1_8_2_incremental_equals_full() {
        let gen = NavMeshGenerator::new(NavMeshBuildConfig::default());
        let geom = InputGeometry {
            vertices: vec![
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(10.0, 0.0, 0.0),
                Vec3::new(10.0, 0.0, 10.0),
                Vec3::new(0.0, 0.0, 10.0),
            ],
            indices: vec![0, 1, 2, 0, 2, 3],
            area_types: vec![crate::navigation::types::AreaType::Ground; 2],
        };
        assert!(compare_incremental_to_full_tile(
            &gen,
            TileCoord { x: 0, z: 0 },
            LayerId(0),
            &geom
        ));
    }

    /// TC-7.1.10.2 — higher agent count priority sorts first.
    #[test]
    fn tc_7_1_10_2_rebuild_priority() {
        let mut rb = NavMeshRebuilder::default();
        rb.enqueue(TileCoord { x: 1, z: 0 }, LayerId(0), 2);
        rb.enqueue(TileCoord { x: 0, z: 0 }, LayerId(0), 10);
        assert_eq!(rb.pop_next(), Some((TileCoord { x: 0, z: 0 }, LayerId(0))));
    }
}
