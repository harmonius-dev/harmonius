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

/// Triangles whose XZ bounds intersect any tile in `affected_tiles` (TC-7.1.8.2).
///
/// Models an incremental builder that only receives geometry overlapping the dirty region instead of
/// the full world triangle soup.
#[must_use]
pub fn input_geometry_for_tiles(
    geometry: &InputGeometry,
    tile_size: f32,
    affected_tiles: &HashSet<TileCoord>,
) -> InputGeometry {
    if affected_tiles.is_empty() || geometry.indices.len() < 3 {
        return InputGeometry::default();
    }
    let mut verts = Vec::new();
    let mut indices = Vec::new();
    let mut area_types = Vec::new();
    let mut remap = vec![u32::MAX; geometry.vertices.len()];
    let mut next = 0u32;
    for (tri_index, tri) in geometry.indices.chunks_exact(3).enumerate() {
        let i0 = tri[0] as usize;
        let i1 = tri[1] as usize;
        let i2 = tri[2] as usize;
        let Some(v0) = geometry.vertices.get(i0) else {
            continue;
        };
        let Some(v1) = geometry.vertices.get(i1) else {
            continue;
        };
        let Some(v2) = geometry.vertices.get(i2) else {
            continue;
        };
        let xmin = v0.x.min(v1.x).min(v2.x);
        let xmax = v0.x.max(v1.x).max(v2.x);
        let zmin = v0.z.min(v1.z).min(v2.z);
        let zmax = v0.z.max(v1.z).max(v2.z);
        if !triangle_intersects_tile_set(xmin, xmax, zmin, zmax, tile_size, affected_tiles) {
            continue;
        }
        let Some(at) = geometry.area_types.get(tri_index) else {
            continue;
        };
        for &vi in &[i0, i1, i2] {
            if remap[vi] == u32::MAX {
                remap[vi] = next;
                verts.push(geometry.vertices[vi]);
                next += 1;
            }
        }
        indices.extend_from_slice(&[remap[i0], remap[i1], remap[i2]]);
        area_types.push(*at);
    }
    InputGeometry {
        vertices: verts,
        indices,
        area_types,
    }
}

fn triangle_intersects_tile_set(
    xmin: f32,
    xmax: f32,
    zmin: f32,
    zmax: f32,
    tile_size: f32,
    tiles: &HashSet<TileCoord>,
) -> bool {
    if tile_size <= 0.0 {
        return false;
    }
    tiles.iter().any(|c| {
        let x0 = c.x as f32 * tile_size;
        let z0 = c.z as f32 * tile_size;
        let x1 = x0 + tile_size;
        let z1 = z0 + tile_size;
        !(xmax < x0 || xmin > x1 || zmax < z0 || zmin > z1)
    })
}

/// Compares regenerating `coord` from geometry clipped to `affected_tiles` against full geometry
/// (TC-7.1.8.2 #1 — local regen matches full inputs for that tile).
#[must_use]
pub fn compare_incremental_to_full_tile(
    gen: &NavMeshGenerator,
    coord: TileCoord,
    layer: LayerId,
    affected_tiles: &HashSet<TileCoord>,
    geometry: &InputGeometry,
) -> bool {
    if !affected_tiles.contains(&coord) {
        return false;
    }
    let local = input_geometry_for_tiles(geometry, gen.tile_size(), affected_tiles);
    let full = gen.generate_tile(coord, layer, geometry);
    let inc = gen.generate_tile(coord, layer, &local);
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

    /// TC-7.1.8.2 — geometry clipped to the dirty tile union yields the same tile as the full soup.
    #[test]
    fn tc_7_1_8_2_incremental_equals_full() {
        let gen = NavMeshGenerator::new(NavMeshBuildConfig::default());
        let geom = InputGeometry {
            vertices: vec![
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(10.0, 0.0, 0.0),
                Vec3::new(10.0, 0.0, 10.0),
                Vec3::new(0.0, 0.0, 10.0),
                // Far-away triangle that must not change tile (0,0) when clipped out.
                Vec3::new(500.0, 0.0, 500.0),
                Vec3::new(510.0, 0.0, 500.0),
                Vec3::new(510.0, 0.0, 510.0),
            ],
            indices: vec![0, 1, 2, 0, 2, 3, 4, 5, 6],
            area_types: vec![crate::navigation::types::AreaType::Ground; 3],
        };
        let dirty = dirty_tiles_for_local_edit(TileCoord { x: 0, z: 0 }, false);
        assert!(compare_incremental_to_full_tile(
            &gen,
            TileCoord { x: 0, z: 0 },
            LayerId(0),
            &dirty,
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
