//! Dynamic obstacle carving into NavMesh tiles.

use glam::Vec3;

use super::tile_map::NavMeshTileMap;
use super::types::{PolyFlags, PolyRef, TileKey};

/// Geometric shape submitted for carving.
#[derive(Clone, Debug, PartialEq)]
pub enum CarveShape {
    /// Axis-aligned box in world space.
    Box {
        /// Half extents along each axis (meters).
        half_extents: Vec3,
    },
}

/// Marks polygons overlapped by obstacles as disabled.
#[derive(Clone, Copy, Debug, Default)]
pub struct ObstacleCarver;

impl ObstacleCarver {
    /// Disables polygons whose centroids fall inside the carve AABB.
    ///
    /// Until the shared spatial index is wired, this scans all loaded tiles whose bounds overlap
    /// the carve region.
    pub fn carve(
        &self,
        tile_map: &mut NavMeshTileMap,
        shape: &CarveShape,
        center: Vec3,
    ) -> Vec<PolyRef> {
        let CarveShape::Box { half_extents } = shape;
        let carve_min = center - *half_extents;
        let carve_max = center + *half_extents;
        let mut touched = Vec::new();
        let keys: Vec<TileKey> = tile_map.loaded_keys().collect();
        for key in keys {
            let Some(tile) = tile_map.get_tile(key.coord, key.layer) else {
                continue;
            };
            if !aabb_intersects(tile.bounds_min, tile.bounds_max, carve_min, carve_max) {
                continue;
            }
            let mut tile_clone = tile.clone();
            let mut changed = false;
            for pi in 0..tile_clone.polygons.len() {
                let Some(pc) = tile_clone.polygon_center(pi as u16) else {
                    continue;
                };
                if aabb_contains(carve_min, carve_max, pc) {
                    tile_clone.polygons[pi].flags.insert(PolyFlags::DISABLED);
                    touched.push(PolyRef {
                        tile: key,
                        poly_index: pi as u16,
                    });
                    changed = true;
                }
            }
            if changed {
                tile_map.swap_tile(key, tile_clone);
            }
        }
        touched
    }

    /// Restores polygons disabled by a prior carve.
    pub fn uncarve(&self, tile_map: &mut NavMeshTileMap, affected: &[PolyRef]) {
        for pref in affected {
            let Some(tile) = tile_map.get_tile(pref.tile.coord, pref.tile.layer) else {
                continue;
            };
            let mut t = tile.clone();
            if let Some(poly) = t.polygons.get_mut(usize::from(pref.poly_index)) {
                poly.flags.remove(PolyFlags::DISABLED);
            }
            tile_map.swap_tile(
                TileKey {
                    coord: pref.tile.coord,
                    layer: pref.tile.layer,
                },
                t,
            );
        }
    }
}

fn aabb_intersects(a_min: Vec3, a_max: Vec3, b_min: Vec3, b_max: Vec3) -> bool {
    a_min.x <= b_max.x
        && a_max.x >= b_min.x
        && a_min.y <= b_max.y
        && a_max.y >= b_min.y
        && a_min.z <= b_max.z
        && a_max.z >= b_min.z
}

fn aabb_contains(min: Vec3, max: Vec3, p: Vec3) -> bool {
    p.x >= min.x && p.x <= max.x && p.y >= min.y && p.y <= max.y && p.z >= min.z && p.z <= max.z
}

#[cfg(test)]
mod tests {
    use glam::Vec3;
    use smallvec::SmallVec;

    use super::*;
    use crate::navigation::pathfinder::heuristic_euclidean;
    use crate::navigation::pathfinder::{Pathfinder, QueryFilter};
    use crate::navigation::tile::{NavMeshTile, NavPoly};
    use crate::navigation::tile_map::{NavMeshTileMap, TileMapConfig};
    use crate::navigation::types::{AreaType, LayerId, PolyFlags, PolyRef, TileCoord, TileKey};
    use crate::navigation::AreaCostTable;

    fn three_poly_line() -> (NavMeshTileMap, PolyRef, PolyRef) {
        let mut map = NavMeshTileMap::new(TileMapConfig {
            tile_size: 30.0,
            ..Default::default()
        });
        let layer = LayerId(0);
        let c = TileCoord { x: 0, z: 0 };
        let mut verts = Vec::new();
        let mut polys = Vec::new();
        for i in 0..3 {
            let x0 = i as f32 * 4.0;
            let b = verts.len() as u16;
            verts.extend_from_slice(&[
                Vec3::new(x0, 0.0, 0.0),
                Vec3::new(x0 + 4.0, 0.0, 0.0),
                Vec3::new(x0 + 4.0, 0.0, 4.0),
                Vec3::new(x0, 0.0, 4.0),
            ]);
            let mut neigh: SmallVec<[Option<PolyRef>; 6]> = SmallVec::new();
            neigh.resize(4, None);
            if i > 0 {
                neigh[3] = Some(PolyRef {
                    tile: TileKey { coord: c, layer },
                    poly_index: (i - 1) as u16,
                });
            }
            if i + 1 < 3 {
                neigh[1] = Some(PolyRef {
                    tile: TileKey { coord: c, layer },
                    poly_index: (i + 1) as u16,
                });
            }
            polys.push(NavPoly {
                vertex_indices: smallvec::smallvec![b, b + 1, b + 2, b + 3],
                area_type: AreaType::Ground,
                neighbors: neigh,
                flags: PolyFlags::WALKABLE,
            });
        }
        let tile = NavMeshTile {
            coord: c,
            layer,
            bounds_min: Vec3::ZERO,
            bounds_max: Vec3::new(14.0, 0.0, 6.0),
            polygons: polys,
            vertices: verts,
            detail_meshes: vec![],
            links: vec![],
        };
        map.insert_tile(tile);
        let s = PolyRef {
            tile: TileKey { coord: c, layer },
            poly_index: 0,
        };
        let g = PolyRef {
            tile: TileKey { coord: c, layer },
            poly_index: 2,
        };
        (map, s, g)
    }

    /// TC-7.1.6.1 — carve blocks choke poly.
    #[test]
    fn tc_7_1_6_1_carve_blocks() {
        let (mut map, s, g) = three_poly_line();
        let pf = Pathfinder::new(heuristic_euclidean);
        let table = AreaCostTable::new();
        let filter = QueryFilter::default();
        assert!(pf.find_corridor(&map, &table, s, g, &filter).is_some());
        let carver = ObstacleCarver;
        let shape = CarveShape::Box {
            half_extents: Vec3::new(2.0, 1.0, 2.0),
        };
        carver.carve(&mut map, &shape, Vec3::new(6.0, 0.0, 2.0));
        assert!(pf.find_corridor(&map, &table, s, g, &filter).is_none());
    }

    /// TC-7.1.6.2 — uncarve restores connectivity.
    #[test]
    fn tc_7_1_6_2_uncarve_restores() {
        let (mut map, s, g) = three_poly_line();
        let pf = Pathfinder::new(heuristic_euclidean);
        let table = AreaCostTable::new();
        let filter = QueryFilter::default();
        let carver = ObstacleCarver;
        let shape = CarveShape::Box {
            half_extents: Vec3::new(2.0, 1.0, 2.0),
        };
        let aff = carver.carve(&mut map, &shape, Vec3::new(6.0, 0.0, 2.0));
        carver.uncarve(&mut map, &aff);
        assert!(pf.find_corridor(&map, &table, s, g, &filter).is_some());
    }
}
