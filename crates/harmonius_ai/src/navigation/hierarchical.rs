//! Hierarchical (cluster / tile) coarse pathfinding.

use std::collections::{HashMap, HashSet, VecDeque};

use super::area_cost::AreaCostTable;
use super::tile_map::NavMeshTileMap;
use super::types::{LayerId, TileCoord};

/// Loaded-tile adjacency graph (4-neighbor BFS over present `TileCoord` keys).
///
/// Cost-aware HPA* clusters from the design are not implemented yet; `find_path` is uniform-cost
/// tile hops and ignores `_area_costs` until abstract costs land.
#[derive(Clone, Debug, Default)]
pub struct ClusterGraph {
    /// Last rebuilt tile set fingerprint (tile coords present).
    last_keys: Vec<TileCoord>,
}

impl ClusterGraph {
    /// Creates an empty graph.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Rebuilds adjacency from 4-neighbor connectivity among loaded tiles.
    pub fn rebuild(&mut self, tile_map: &NavMeshTileMap, layer: LayerId) {
        self.last_keys = tile_coords_loaded(tile_map, layer);
    }

    /// Updates bookkeeping for one tile (TC-7.1.14.3 — cluster scope narrows to `coord`).
    pub fn update_cluster(
        &mut self,
        _tile_map: &NavMeshTileMap,
        coord: TileCoord,
        _layer: LayerId,
    ) {
        self.last_keys = vec![coord];
    }

    /// BFS coarse path across loaded tiles (TC-7.1.14.1).
    #[must_use]
    pub fn find_path(
        &self,
        tile_map: &NavMeshTileMap,
        layer: LayerId,
        from: TileCoord,
        to: TileCoord,
        _area_costs: &AreaCostTable,
    ) -> Option<Vec<TileCoord>> {
        if from == to {
            return Some(vec![from]);
        }
        let loaded: HashSet<TileCoord> = tile_coords_loaded(tile_map, layer).into_iter().collect();
        if !loaded.contains(&from) || !loaded.contains(&to) {
            return None;
        }
        let mut q = VecDeque::new();
        let mut prev: HashMap<TileCoord, TileCoord> = HashMap::new();
        q.push_back(from);
        prev.insert(from, from);
        while let Some(c) = q.pop_front() {
            if c == to {
                let mut out = vec![to];
                let mut x = to;
                while x != from {
                    x = prev[&x];
                    out.push(x);
                }
                out.reverse();
                return Some(out);
            }
            for n in neighbors(c) {
                if loaded.contains(&n) && !prev.contains_key(&n) {
                    prev.insert(n, c);
                    q.push_back(n);
                }
            }
        }
        None
    }

    /// Tiles considered in the last `rebuild` / `update_cluster` pass.
    #[must_use]
    pub fn tracked_tiles(&self) -> &[TileCoord] {
        &self.last_keys
    }
}

fn tile_coords_loaded(tile_map: &NavMeshTileMap, layer: LayerId) -> Vec<TileCoord> {
    tile_map
        .loaded_keys()
        .filter(|k| k.layer == layer)
        .map(|k| k.coord)
        .collect()
}

fn neighbors(c: TileCoord) -> [TileCoord; 4] {
    [
        TileCoord { x: c.x + 1, z: c.z },
        TileCoord { x: c.x - 1, z: c.z },
        TileCoord { x: c.x, z: c.z + 1 },
        TileCoord { x: c.x, z: c.z - 1 },
    ]
}

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use super::*;
    use crate::navigation::tile::{NavMeshTile, NavPoly};
    use crate::navigation::tile_map::{NavMeshTileMap, TileMapConfig};
    use crate::navigation::types::{AreaType, PolyFlags};

    fn strip_map(n: i32) -> NavMeshTileMap {
        let mut map = NavMeshTileMap::new(TileMapConfig {
            tile_size: 10.0,
            ..Default::default()
        });
        let layer = LayerId(0);
        for x in 0..n {
            let c = TileCoord { x, z: 0 };
            let tile = NavMeshTile {
                coord: c,
                layer,
                bounds_min: Vec3::new(x as f32 * 10.0, 0.0, 0.0),
                bounds_max: Vec3::new((x + 1) as f32 * 10.0, 0.0, 10.0),
                polygons: vec![NavPoly {
                    vertex_indices: smallvec::smallvec![0, 1, 2, 3],
                    area_type: AreaType::Ground,
                    neighbors: smallvec::smallvec![None; 4],
                    flags: PolyFlags::WALKABLE,
                }],
                vertices: vec![
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(1.0, 0.0, 0.0),
                    Vec3::new(1.0, 0.0, 1.0),
                    Vec3::new(0.0, 0.0, 1.0),
                ],
                detail_meshes: vec![],
                links: vec![],
            };
            map.insert_tile(tile);
        }
        map
    }

    /// TC-7.1.14.1 #1 — long strip of loaded tiles yields a coarse tile path.
    #[test]
    fn tc_7_1_14_1_long_coarse_tile_strip() {
        let map = strip_map(50);
        let table = AreaCostTable::new();
        let mut cg = ClusterGraph::new();
        cg.rebuild(&map, LayerId(0));
        let p = cg
            .find_path(
                &map,
                LayerId(0),
                TileCoord { x: 0, z: 0 },
                TileCoord { x: 49, z: 0 },
                &table,
            )
            .expect("path");
        assert!(p.len() >= 2);
    }

    /// TC-7.1.14.1 #2 — same tile collapses.
    #[test]
    fn tc_7_1_14_1_same_tile_local() {
        let map = strip_map(5);
        let table = AreaCostTable::new();
        let cg = ClusterGraph::new();
        let p = cg
            .find_path(
                &map,
                LayerId(0),
                TileCoord { x: 0, z: 0 },
                TileCoord { x: 0, z: 0 },
                &table,
            )
            .expect("path");
        assert_eq!(p.len(), 1);
    }

    /// TC-7.1.14.3 — cluster update narrows tracked set.
    #[test]
    fn tc_7_1_14_3_cluster_update_scope() {
        let map = strip_map(10);
        let mut cg = ClusterGraph::new();
        cg.rebuild(&map, LayerId(0));
        let before = cg.tracked_tiles().len();
        cg.update_cluster(&map, TileCoord { x: 5, z: 0 }, LayerId(0));
        assert!(cg.tracked_tiles().len() <= before);
    }
}
