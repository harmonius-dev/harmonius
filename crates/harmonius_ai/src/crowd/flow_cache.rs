//! LRU cache for tiled flow fields.

use std::collections::{HashMap, VecDeque};

use glam::IVec2;

use super::flow_field::FlowField;

/// Key identifying a cached flow field slice.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FlowFieldKey {
    /// Goal cell in tile-local coordinates.
    pub goal_cell: glam::UVec2,
    /// Cost layer version.
    pub cost_version: u64,
    /// Tile coordinate in world tiling.
    pub tile_coord: IVec2,
}

/// LRU-backed cache of [`FlowField`] values.
#[derive(Debug)]
pub struct FlowFieldCache {
    entries: HashMap<FlowFieldKey, FlowField>,
    lru_order: VecDeque<FlowFieldKey>,
    /// Maximum retained fields.
    pub max_entries: u32,
}

impl FlowFieldCache {
    /// Create an empty cache with the given capacity.
    pub fn new(max_entries: u32) -> Self {
        Self {
            entries: HashMap::new(),
            lru_order: VecDeque::new(),
            max_entries,
        }
    }

    /// Borrow a field and mark it most-recently used.
    pub fn get(&mut self, key: &FlowFieldKey) -> Option<&FlowField> {
        if self.entries.contains_key(key) {
            self.lru_order.retain(|k| k != key);
            self.lru_order.push_front(*key);
            self.entries.get(key)
        } else {
            None
        }
    }

    /// Insert `field`, evicting LRU when over capacity.
    pub fn insert(&mut self, key: FlowFieldKey, field: FlowField) {
        if self.entries.contains_key(&key) {
            self.lru_order.retain(|k| k != &key);
        } else if self.entries.len() as u32 >= self.max_entries {
            if let Some(old) = self.lru_order.pop_back() {
                self.entries.remove(&old);
            }
        }
        self.entries.insert(key, field);
        self.lru_order.push_front(key);
    }

    /// Drop entries whose `cost_version` is strictly older than `current_version`.
    pub fn invalidate_stale(&mut self, current_version: u64) {
        self.entries
            .retain(|k, _| k.cost_version >= current_version);
        self.lru_order.retain(|k| self.entries.contains_key(k));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::{UVec2, Vec2};

    fn dummy_field() -> FlowField {
        FlowField {
            grid: vec![Vec2::X; 4],
            origin: Vec2::ZERO,
            cell_size: 1.0,
            width: 2,
            height: 2,
            goal_cell: UVec2::ZERO,
            cost_version: 1,
        }
    }

    #[test]
    fn tc_7_7_3_1_flow_cache_hit() {
        let mut c = FlowFieldCache::new(8);
        let k = FlowFieldKey {
            goal_cell: UVec2::new(3, 4),
            cost_version: 1,
            tile_coord: IVec2::ZERO,
        };
        c.insert(k, dummy_field());
        assert!(c.get(&k).is_some());
        assert!(c.get(&k).is_some());
    }

    #[test]
    fn tc_7_7_3_2_flow_cache_invalidation() {
        let mut c = FlowFieldCache::new(8);
        let k = FlowFieldKey {
            goal_cell: UVec2::new(1, 1),
            cost_version: 1,
            tile_coord: IVec2::ZERO,
        };
        c.insert(k, dummy_field());
        c.invalidate_stale(2);
        assert!(c.get(&k).is_none());
    }

    #[test]
    fn tc_7_7_3_3_flow_cache_lru_eviction() {
        let mut c = FlowFieldCache::new(4);
        for i in 0..5_u32 {
            let k = FlowFieldKey {
                goal_cell: UVec2::new(i, 0),
                cost_version: 1,
                tile_coord: IVec2::ZERO,
            };
            c.insert(k, dummy_field());
        }
        let oldest = FlowFieldKey {
            goal_cell: UVec2::new(0, 0),
            cost_version: 1,
            tile_coord: IVec2::ZERO,
        };
        assert!(c.get(&oldest).is_none());
    }
}
