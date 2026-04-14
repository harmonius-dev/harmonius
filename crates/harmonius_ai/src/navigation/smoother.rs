//! Path smoothing after funneling.

use glam::Vec3;

use super::tile::raycast_poly_xz;
use super::tile_map::NavMeshTileMap;
use super::types::Waypoint;

/// Smoothing algorithm selection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SmoothingMode {
    /// Raw funnel polyline.
    None,
    /// Linear interpolation with ray-shortcutting.
    Linear,
    /// Catmull-Rom spline between waypoints.
    CatmullRom,
    /// Cubic Bezier (reserved).
    Bezier,
}

/// Mobile vs desktop quality targets.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SmoothingTarget {
    /// Mobile profile: CatmullRom coerced to Linear.
    Mobile,
    /// Desktop profile: honor requested mode.
    Desktop,
}

/// Post-processes funnel output into denser samples.
pub struct PathSmoother;

impl PathSmoother {
    /// Samples `waypoints` into `segment_count` segments per span.
    pub fn smooth(
        tile_map: &NavMeshTileMap,
        waypoints: &[Waypoint],
        mode: SmoothingMode,
        segment_count: u32,
        target: SmoothingTarget,
    ) -> Vec<Waypoint> {
        let mode = match (target, mode) {
            (SmoothingTarget::Mobile, SmoothingMode::CatmullRom | SmoothingMode::Bezier) => {
                SmoothingMode::Linear
            }
            (_, m) => m,
        };
        if waypoints.len() < 2 || segment_count == 0 {
            return waypoints.to_vec();
        }
        match mode {
            SmoothingMode::None => waypoints.to_vec(),
            SmoothingMode::Linear => {
                let mut out = Vec::new();
                for w in waypoints.windows(2) {
                    for i in 0..=segment_count {
                        let t = i as f32 / segment_count as f32;
                        let p = w[0].position.lerp(w[1].position, t);
                        let pref = if t < 0.5 {
                            w[0].poly_ref
                        } else {
                            w[1].poly_ref
                        };
                        out.push(Waypoint {
                            position: p,
                            poly_ref: pref,
                            link: None,
                        });
                    }
                }
                out
            }
            SmoothingMode::CatmullRom => catmull_rom(tile_map, waypoints, segment_count),
            SmoothingMode::Bezier => PathSmoother::smooth(
                tile_map,
                waypoints,
                SmoothingMode::Linear,
                segment_count,
                target,
            ),
        }
    }

    /// Removes collinear intermediate turns using NavMesh raycasts on XZ.
    pub fn remove_redundant_turns(tile_map: &NavMeshTileMap, waypoints: &mut Vec<Waypoint>) {
        if waypoints.len() < 3 {
            return;
        }
        let mut i = 0;
        while i + 2 < waypoints.len() {
            let a = waypoints[i].position;
            let c = waypoints[i + 2].position;
            let tile = tile_map
                .get_tile(
                    waypoints[i].poly_ref.tile.coord,
                    waypoints[i].poly_ref.tile.layer,
                )
                .expect("tile");
            let hit = raycast_poly_xz(tile, waypoints[i].poly_ref.poly_index, a, c);
            if let Some(t) = hit {
                if t >= 0.999 {
                    waypoints.remove(i + 1);
                    continue;
                }
            }
            i += 1;
        }
    }
}

fn catmull_rom(
    tile_map: &NavMeshTileMap,
    waypoints: &[Waypoint],
    segment_count: u32,
) -> Vec<Waypoint> {
    let mut out = Vec::new();
    let n = waypoints.len();
    for i in 0..n - 1 {
        let p0 = if i == 0 {
            waypoints[0].position
        } else {
            waypoints[i - 1].position
        };
        let p1 = waypoints[i].position;
        let p2 = waypoints[i + 1].position;
        let p3 = if i + 2 < n {
            waypoints[i + 2].position
        } else {
            waypoints[i + 1].position
        };
        for s in 0..=segment_count {
            let t = s as f32 / segment_count as f32;
            let p = 0.5
                * (2.0 * p1
                    + (-p0 + p2) * t
                    + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t * t
                    + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t * t * t);
            let pref = if t < 0.5 {
                waypoints[i].poly_ref
            } else {
                waypoints[i + 1].poly_ref
            };
            let tile = tile_map
                .get_tile(pref.tile.coord, pref.tile.layer)
                .expect("tile");
            let mut q = p;
            if !tile.point_in_polygon(pref.poly_index, q) {
                q = project_to_poly(tile, pref.poly_index, q);
            }
            out.push(Waypoint {
                position: q,
                poly_ref: pref,
                link: None,
            });
        }
    }
    out
}

fn project_to_poly(tile: &super::tile::NavMeshTile, index: u16, p: Vec3) -> Vec3 {
    if let Some(c) = tile.polygon_center(index) {
        Vec3::new(p.x, c.y, p.z)
    } else {
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::navigation::tile::{NavMeshTile, NavPoly};
    use crate::navigation::tile_map::{NavMeshTileMap, TileMapConfig};
    use crate::navigation::types::{AreaType, LayerId, PolyFlags, PolyRef, TileCoord, TileKey};

    fn flat_tile() -> NavMeshTileMap {
        let mut map = NavMeshTileMap::new(TileMapConfig {
            tile_size: 50.0,
            ..Default::default()
        });
        let layer = LayerId(0);
        let c = TileCoord { x: 0, z: 0 };
        let tile = NavMeshTile {
            coord: c,
            layer,
            bounds_min: Vec3::ZERO,
            bounds_max: Vec3::new(50.0, 0.0, 50.0),
            polygons: vec![NavPoly {
                vertex_indices: smallvec::smallvec![0, 1, 2, 3],
                area_type: AreaType::Ground,
                neighbors: smallvec::smallvec![None; 4],
                flags: PolyFlags::WALKABLE,
            }],
            vertices: vec![
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(50.0, 0.0, 0.0),
                Vec3::new(50.0, 0.0, 50.0),
                Vec3::new(0.0, 0.0, 50.0),
            ],
            detail_meshes: vec![],
            links: vec![],
        };
        map.insert_tile(tile);
        map
    }

    /// TC-7.1.5.1 — Catmull-Rom samples stay on mesh (single large poly).
    #[test]
    fn tc_7_1_5_1_catmull_on_mesh() {
        let map = flat_tile();
        let pref = PolyRef {
            tile: TileKey {
                coord: TileCoord { x: 0, z: 0 },
                layer: LayerId(0),
            },
            poly_index: 0,
        };
        let wps: Vec<Waypoint> = (0..5)
            .map(|i| Waypoint {
                position: Vec3::new(i as f32 * 5.0, 0.0, 10.0),
                poly_ref: pref,
                link: None,
            })
            .collect();
        let sm = PathSmoother::smooth(
            &map,
            &wps,
            SmoothingMode::CatmullRom,
            20,
            SmoothingTarget::Desktop,
        );
        assert_eq!(sm.len(), 84);
        let tile = map
            .get_tile(TileCoord { x: 0, z: 0 }, LayerId(0))
            .expect("t");
        for w in &sm {
            assert!(tile.point_in_polygon(0, w.position));
        }
    }

    /// TC-7.1.5.2 — mobile forces linear.
    #[test]
    fn tc_7_1_5_2_mobile_linear() {
        let map = flat_tile();
        let pref = PolyRef {
            tile: TileKey {
                coord: TileCoord { x: 0, z: 0 },
                layer: LayerId(0),
            },
            poly_index: 0,
        };
        let wps = vec![
            Waypoint {
                position: Vec3::ZERO,
                poly_ref: pref,
                link: None,
            },
            Waypoint {
                position: Vec3::new(10.0, 0.0, 0.0),
                poly_ref: pref,
                link: None,
            },
        ];
        let m = PathSmoother::smooth(
            &map,
            &wps,
            SmoothingMode::CatmullRom,
            4,
            SmoothingTarget::Mobile,
        );
        let lin = PathSmoother::smooth(
            &map,
            &wps,
            SmoothingMode::Linear,
            4,
            SmoothingTarget::Desktop,
        );
        assert_eq!(m.len(), lin.len());
    }
}
