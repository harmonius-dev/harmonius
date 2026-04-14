//! Runtime decal atlas packing with LRU eviction.

use std::collections::{HashMap, VecDeque};

/// Stable asset identifier used by atlas tests.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct AssetId(pub u64);

/// UV rectangle in normalized \[0,1\] atlas space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AtlasRegion {
    /// Origin U.
    pub u0: f32,
    /// Origin V.
    pub v0: f32,
    /// Extent U.
    pub u1: f32,
    /// Extent V.
    pub v1: f32,
}

/// Fixed-capacity atlas with LRU eviction when full.
#[derive(Debug)]
pub struct DecalAtlas {
    capacity: u32,
    regions: HashMap<AssetId, AtlasRegion>,
    lru: VecDeque<AssetId>,
}

impl DecalAtlas {
    /// Creates an atlas that can hold at most `capacity` distinct textures.
    pub fn new(capacity: u32) -> Self {
        Self {
            capacity,
            regions: HashMap::new(),
            lru: VecDeque::new(),
        }
    }

    /// Packs `id` into a grid cell; evicts LRU entries if needed (**TC-11.2.3.2**).
    pub fn pack(&mut self, id: AssetId) -> Option<AtlasRegion> {
        if self.regions.contains_key(&id) {
            self.touch(id);
            return self.regions.get(&id).copied();
        }
        while self.regions.len() as u32 >= self.capacity {
            let victim = self.lru.pop_front()?;
            self.regions.remove(&victim);
        }
        let idx = self.regions.len() as u32;
        let n = self.capacity.max(1);
        let cols = (n as f32).sqrt().ceil() as u32;
        let rows = n.div_ceil(cols);
        let row = idx / cols;
        let col = idx % cols;
        let du = 1.0 / cols as f32;
        let dv = 1.0 / rows as f32;
        let region = AtlasRegion {
            u0: col as f32 * du,
            v0: row as f32 * dv,
            u1: (col + 1) as f32 * du,
            v1: (row + 1) as f32 * dv,
        };
        self.regions.insert(id, region);
        self.lru.push_back(id);
        Some(region)
    }

    fn touch(&mut self, id: AssetId) {
        if let Some(pos) = self.lru.iter().position(|x| *x == id) {
            self.lru.remove(pos);
        }
        self.lru.push_back(id);
    }

    /// Returns the region for `id` if resident.
    pub fn lookup(&self, id: AssetId) -> Option<AtlasRegion> {
        self.regions.get(&id).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::{AssetId, DecalAtlas};

    /// **TC-11.2.3.1** — many textures pack to valid regions with stable lookup.
    #[test]
    fn tc_11_2_3_1_atlas_pack_and_lookup() {
        let mut a = DecalAtlas::new(64);
        for i in 0..50 {
            let id = AssetId(i + 1);
            let r = a.pack(id).expect("pack");
            assert!(r.u1 > r.u0);
            assert!(r.v1 > r.v0);
            assert_eq!(a.lookup(id), Some(r));
        }
    }

    /// **TC-11.2.3.2** — exceeding capacity evicts LRU entries.
    #[test]
    fn tc_11_2_3_2_atlas_lru_eviction() {
        let mut a = DecalAtlas::new(4);
        for i in 1..=4 {
            assert!(a.pack(AssetId(i)).is_some());
        }
        assert!(a.lookup(AssetId(1)).is_some());
        assert!(a.pack(AssetId(5)).is_some());
        assert!(a.lookup(AssetId(1)).is_none());
        assert!(a.lookup(AssetId(5)).is_some());
    }
}
