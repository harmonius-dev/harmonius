//! Central tile storage for loaded NavMesh chunks.

use std::collections::HashMap;

use glam::Vec3;

use super::tile::NavMeshTile;
use super::types::{LayerId, PolyRef, TileCoord, TileKey};

/// Loading state for a tile slot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TileStatus {
    /// Tile data is current.
    Loaded,
    /// Rebuild in progress; stale data may still be read as fallback.
    Pending,
    /// Slot empty (outside streaming window or never loaded).
    Unloaded,
}

/// Configuration for the in-memory tile window.
#[derive(Clone, Debug)]
pub struct TileMapConfig {
    /// World-space edge length of each square tile (XZ).
    pub tile_size: f32,
    /// Maximum resident tiles (soft budget).
    pub max_loaded_tiles: u32,
    /// Preload radius in tile units (unused in minimal build).
    pub preload_radius: u32,
}

impl Default for TileMapConfig {
    fn default() -> Self {
        Self {
            tile_size: 64.0,
            max_loaded_tiles: 1024,
            preload_radius: 2,
        }
    }
}

#[derive(Debug)]
struct TileSlot {
    tile: NavMeshTile,
    status: TileStatus,
}

/// Runtime NavMesh storage shared by pathfinding and rebuild systems.
#[derive(Debug)]
pub struct NavMeshTileMap {
    config: TileMapConfig,
    tiles: HashMap<TileKey, TileSlot>,
    /// Incremented when geometry-backed tiles are rebuilt (not area-cost edits).
    pub geometry_rebuild_counter: u64,
    /// Monotonic counter bumped on every successful `swap_tile`.
    pub tile_swap_generation: u64,
}

impl NavMeshTileMap {
    /// Creates an empty map.
    #[must_use]
    pub fn new(config: TileMapConfig) -> Self {
        Self {
            config,
            tiles: HashMap::new(),
            geometry_rebuild_counter: 0,
            tile_swap_generation: 0,
        }
    }

    /// Borrows a tile if present.
    #[must_use]
    pub fn get_tile(&self, coord: TileCoord, layer: LayerId) -> Option<&NavMeshTile> {
        let key = TileKey { coord, layer };
        self.tiles.get(&key).map(|s| &s.tile)
    }

    /// Tile status for the slot.
    #[must_use]
    pub fn tile_status(&self, coord: TileCoord, layer: LayerId) -> TileStatus {
        let key = TileKey { coord, layer };
        self.tiles
            .get(&key)
            .map(|s| s.status)
            .unwrap_or(TileStatus::Unloaded)
    }

    /// Inserts or replaces a tile as [`TileStatus::Loaded`].
    pub fn insert_tile(&mut self, tile: NavMeshTile) {
        let key = TileKey {
            coord: tile.coord,
            layer: tile.layer,
        };
        self.tiles.insert(
            key,
            TileSlot {
                tile,
                status: TileStatus::Loaded,
            },
        );
    }

    /// Marks a tile pending without removing geometry.
    pub fn mark_pending(&mut self, coord: TileCoord, layer: LayerId) {
        let key = TileKey { coord, layer };
        if let Some(slot) = self.tiles.get_mut(&key) {
            slot.status = TileStatus::Pending;
        }
    }

    /// Swaps in rebuilt tile data and clears pending state.
    pub fn swap_tile(&mut self, key: TileKey, tile: NavMeshTile) {
        self.geometry_rebuild_counter = self.geometry_rebuild_counter.saturating_add(1);
        self.tile_swap_generation = self.tile_swap_generation.saturating_add(1);
        self.tiles.insert(
            key,
            TileSlot {
                tile,
                status: TileStatus::Loaded,
            },
        );
    }

    /// Removes a tile from the resident set.
    pub fn remove_tile(&mut self, coord: TileCoord, layer: LayerId) {
        self.tiles.remove(&TileKey { coord, layer });
    }

    /// Returns tile coordinates overlapping `bounds` on XZ (layer filter only).
    #[must_use]
    pub fn tiles_in_bounds(
        &self,
        bounds_min: Vec3,
        bounds_max: Vec3,
        layer: LayerId,
    ) -> Vec<TileCoord> {
        let ts = self.config.tile_size;
        let min_x = (bounds_min.x / ts).floor() as i32;
        let max_x = (bounds_max.x / ts).floor() as i32;
        let min_z = (bounds_min.z / ts).floor() as i32;
        let max_z = (bounds_max.z / ts).floor() as i32;
        let mut out = Vec::new();
        for x in min_x..=max_x {
            for z in min_z..=max_z {
                let c = TileCoord { x, z };
                if self.get_tile(c, layer).is_some() {
                    out.push(c);
                }
            }
        }
        out
    }

    /// Nearest polygon across all loaded tiles on `layer`.
    #[must_use]
    pub fn find_nearest_poly(
        &self,
        pos: Vec3,
        extent: Vec3,
        layer: LayerId,
    ) -> Option<(PolyRef, Vec3)> {
        let mut best: Option<(PolyRef, Vec3, f32)> = None;
        for slot in self.tiles.values() {
            if slot.tile.layer != layer {
                continue;
            }
            if let Some((pref, p)) = slot.tile.find_nearest_poly(pos, extent) {
                let d = (p - pos).length_squared();
                match best {
                    None => best = Some((pref, p, d)),
                    Some((_, _, bd)) if d < bd => best = Some((pref, p, d)),
                    _ => {}
                }
            }
        }
        best.map(|(pr, p, _)| (pr, p))
    }

    /// Exposes tile size for planners.
    #[must_use]
    pub fn tile_size(&self) -> f32 {
        self.config.tile_size
    }

    /// Iterates all resident tile keys (for maintenance and carving sweeps).
    pub fn loaded_keys(&self) -> impl Iterator<Item = TileKey> + '_ {
        self.tiles.keys().copied()
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec3;
    use smallvec::SmallVec;

    use super::*;
    use crate::navigation::tile::{NavMeshTile, NavPoly};
    use crate::navigation::types::{AreaType, PolyFlags, TileKey};

    fn dummy_tile(coord: TileCoord, layer: LayerId) -> NavMeshTile {
        NavMeshTile {
            coord,
            layer,
            bounds_min: Vec3::ZERO,
            bounds_max: Vec3::ONE,
            polygons: vec![NavPoly {
                vertex_indices: SmallVec::new(),
                area_type: AreaType::Ground,
                neighbors: SmallVec::new(),
                flags: PolyFlags::WALKABLE,
            }],
            vertices: Vec::new(),
            detail_meshes: vec![],
            links: vec![],
        }
    }

    /// TC-7.1.9.1 — pending keeps stale tile readable.
    #[test]
    fn tc_7_1_9_1_pending_fallback() {
        let mut map = NavMeshTileMap::new(TileMapConfig::default());
        let c = TileCoord { x: 0, z: 0 };
        let layer = LayerId(0);
        map.insert_tile(dummy_tile(c, layer));
        map.mark_pending(c, layer);
        assert_eq!(map.tile_status(c, layer), TileStatus::Pending);
        assert!(map.get_tile(c, layer).is_some());
    }

    /// TC-7.1.9.2 — swap bumps generation monotonically.
    #[test]
    fn tc_7_1_9_2_swap_atomic_view() {
        let mut map = NavMeshTileMap::new(TileMapConfig::default());
        let c = TileCoord { x: 0, z: 0 };
        let layer = LayerId(0);
        let k = TileKey { coord: c, layer };
        map.insert_tile(dummy_tile(c, layer));
        let g0 = map.tile_swap_generation;
        map.swap_tile(k, dummy_tile(c, layer));
        assert!(map.tile_swap_generation > g0);
    }

    /// TC-7.1.12.2 — wrong layer misses tile.
    #[test]
    fn tc_7_1_12_2_layer_mismatch() {
        let mut map = NavMeshTileMap::new(TileMapConfig::default());
        let c = TileCoord { x: 0, z: 0 };
        map.insert_tile(dummy_tile(c, LayerId(0)));
        assert!(map.get_tile(c, LayerId(1)).is_none());
    }

    /// TC-7.1.13.3 — geometry rebuild counter not bumped by unrelated mutations (simulated).
    #[test]
    fn tc_7_1_13_3_geometry_counter_only_on_swap() {
        let mut map = NavMeshTileMap::new(TileMapConfig::default());
        let c = TileCoord { x: 0, z: 0 };
        let layer = LayerId(0);
        map.insert_tile(dummy_tile(c, layer));
        let n0 = map.geometry_rebuild_counter;
        map.mark_pending(c, layer);
        assert_eq!(map.geometry_rebuild_counter, n0);
    }
}
