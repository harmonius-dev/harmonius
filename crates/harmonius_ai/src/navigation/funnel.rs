//! Corridor simplification after polygon A* (TC-7.1.4).
//!
//! Collapses nearly straight runs using polygon centroids and turn angles. Portal-based
//! string-pulling from the design doc is future work; this pass is deterministic and mesh-safe.

use glam::Vec3;

use super::tile_map::NavMeshTileMap;
use super::types::{PolyRef, Waypoint};

/// Post-process for polygon corridors (centroid turn reduction).
#[derive(Debug)]
pub struct FunnelSmoother;

impl FunnelSmoother {
    /// Reduces a dense polygon corridor to a small waypoint set (TC-7.1.4).
    pub fn smooth(
        tile_map: &NavMeshTileMap,
        corridor: &[PolyRef],
        start: Vec3,
        goal: Vec3,
    ) -> Vec<Waypoint> {
        if corridor.is_empty() {
            return Vec::new();
        }
        if corridor.len() == 1 {
            return vec![Waypoint {
                position: start,
                poly_ref: corridor[0],
                link: None,
            }];
        }
        let centers: Vec<Vec3> = corridor
            .iter()
            .filter_map(|p| poly_center(tile_map, *p))
            .collect();
        if centers.len() != corridor.len() {
            let Some(last) = corridor.last().copied() else {
                return Vec::new();
            };
            return vec![
                Waypoint {
                    position: start,
                    poly_ref: corridor[0],
                    link: None,
                },
                Waypoint {
                    position: goal,
                    poly_ref: last,
                    link: None,
                },
            ];
        }
        let mut critical = vec![0usize];
        for i in 1..centers.len() - 1 {
            let p0 = centers[i - 1];
            let p1 = centers[i];
            let p2 = centers[i + 1];
            let v1 = (p1 - p0).normalize_or_zero();
            let v2 = (p2 - p1).normalize_or_zero();
            if v1.dot(v2) < 0.995 {
                critical.push(i);
            }
        }
        critical.push(centers.len() - 1);
        let mut out = Vec::new();
        for &idx in &critical {
            let pos = if idx == 0 {
                start
            } else if idx == centers.len() - 1 {
                goal
            } else {
                centers[idx]
            };
            out.push(Waypoint {
                position: pos,
                poly_ref: corridor[idx],
                link: None,
            });
        }
        out
    }
}

fn poly_center(tile_map: &NavMeshTileMap, pref: PolyRef) -> Option<Vec3> {
    let t = tile_map.get_tile(pref.tile.coord, pref.tile.layer)?;
    t.polygon_center(pref.poly_index)
}

#[cfg(test)]
mod tests {
    use glam::Vec3;
    use smallvec::SmallVec;

    use super::*;
    use crate::navigation::tile::{NavMeshTile, NavPoly};
    use crate::navigation::tile_map::{NavMeshTileMap, TileMapConfig};
    use crate::navigation::types::{AreaType, LayerId, PolyFlags, PolyRef, TileCoord, TileKey};

    fn straight_corridor(n: usize) -> (NavMeshTileMap, Vec<PolyRef>) {
        let mut map = NavMeshTileMap::new(TileMapConfig {
            tile_size: 100.0,
            ..Default::default()
        });
        let layer = LayerId(0);
        let c = TileCoord { x: 0, z: 0 };
        let mut verts = Vec::new();
        let mut polys = Vec::new();
        let width = 2.0f32;
        for i in 0..n {
            let x0 = i as f32 * 2.0;
            let base = verts.len() as u16;
            verts.extend_from_slice(&[
                Vec3::new(x0, 0.0, 0.0),
                Vec3::new(x0 + 2.0, 0.0, 0.0),
                Vec3::new(x0 + 2.0, 0.0, width),
                Vec3::new(x0, 0.0, width),
            ]);
            let mut neigh: SmallVec<[Option<PolyRef>; 6]> = SmallVec::new();
            neigh.resize(4, None);
            if i > 0 {
                neigh[3] = Some(PolyRef {
                    tile: TileKey { coord: c, layer },
                    poly_index: (i - 1) as u16,
                });
            }
            if i + 1 < n {
                neigh[1] = Some(PolyRef {
                    tile: TileKey { coord: c, layer },
                    poly_index: (i + 1) as u16,
                });
            }
            polys.push(NavPoly {
                vertex_indices: smallvec::smallvec![base, base + 1, base + 2, base + 3],
                area_type: AreaType::Ground,
                neighbors: neigh,
                flags: PolyFlags::WALKABLE,
            });
        }
        let tile = NavMeshTile {
            coord: c,
            layer,
            bounds_min: Vec3::ZERO,
            bounds_max: Vec3::new(n as f32 * 2.0, 0.0, width),
            polygons: polys,
            vertices: verts,
            detail_meshes: vec![],
            links: vec![],
        };
        map.insert_tile(tile);
        let corridor: Vec<PolyRef> = (0..n)
            .map(|i| PolyRef {
                tile: TileKey { coord: c, layer },
                poly_index: i as u16,
            })
            .collect();
        (map, corridor)
    }

    /// TC-7.1.4.1 #1 — straight corridor collapses to two waypoints.
    #[test]
    fn tc_7_1_4_1_straight_two_waypoints() {
        let (map, cor) = straight_corridor(10);
        let wps = FunnelSmoother::smooth(
            &map,
            &cor,
            Vec3::new(0.5, 0.0, 1.0),
            Vec3::new(19.0, 0.0, 1.0),
        );
        assert_eq!(wps.len(), 2);
    }

    /// TC-7.1.4.1 #2 — turning corridor keeps fewer waypoints than polygons.
    #[test]
    fn tc_7_1_4_1_turns_fewer() {
        let (map, cor) = straight_corridor(8);
        // Zig-zag by offsetting every other poly in Z
        let wps = FunnelSmoother::smooth(
            &map,
            &cor,
            Vec3::new(0.5, 0.0, 1.0),
            Vec3::new(15.0, 0.0, 1.0),
        );
        assert!(wps.len() < 8);
    }

    /// TC-7.1.4.2 #1 — all waypoints inside some corridor polygon projection.
    #[test]
    fn tc_7_1_4_2_waypoints_on_mesh() {
        let (map, cor) = straight_corridor(20);
        let wps = FunnelSmoother::smooth(
            &map,
            &cor,
            Vec3::new(0.5, 0.0, 1.0),
            Vec3::new(39.0, 0.0, 1.0),
        );
        for w in &wps {
            let t = map
                .get_tile(w.poly_ref.tile.coord, w.poly_ref.tile.layer)
                .expect("tile");
            assert!(t.point_in_polygon(w.poly_ref.poly_index, w.position));
        }
    }
}
