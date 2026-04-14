//! Tile-based dirty region tracking (`DirtyRegionSet`).

use fixedbitset::FixedBitSet;

/// Tracks which fixed-size tiles changed for incremental GPU upload.
#[derive(Debug)]
pub struct DirtyRegionSet {
    /// One bit per tile, row-major over the logical tile grid.
    pub tile_bits: FixedBitSet,
    /// Edge length of a tile in cells (default **16** per design).
    pub tile_side: u16,
    /// Scratch buffer filled by [`Self::drain_sorted`].
    pub sorted_indices: Vec<u32>,
}

impl DirtyRegionSet {
    /// Builds a tracker for `tile_count` tiles using `tile_side` cells per edge.
    #[must_use]
    pub fn new(tile_count: usize, tile_side: u16) -> Self {
        Self {
            tile_bits: FixedBitSet::with_capacity(tile_count),
            tile_side,
            sorted_indices: Vec::new(),
        }
    }

    /// Marks `tile_index` dirty when in range.
    pub fn mark(&mut self, tile_index: u32) {
        let Ok(i) = usize::try_from(tile_index) else {
            return;
        };
        if i < self.tile_bits.len() {
            self.tile_bits.insert(i);
        }
    }

    /// Fills [`Self::sorted_indices`] from set bits in ascending index order.
    pub fn drain_sorted(&mut self) -> &[u32] {
        self.sorted_indices.clear();
        for bit in self.tile_bits.ones() {
            self.sorted_indices
                .push(u32::try_from(bit).expect("tile index fits u32"));
        }
        &self.sorted_indices
    }

    /// Clears all dirty bits and scratch state.
    pub fn clear(&mut self) {
        self.tile_bits.clear();
        self.sorted_indices.clear();
    }

    /// Counts raster cells covered by `dirty_tile_count` full tiles.
    ///
    /// Used to size staging uploads (one tile covers `tile_side²` cells on a 2D grid).
    #[must_use]
    pub fn cell_upload_count_2d(tile_side: u16, dirty_tile_count: u32) -> u32 {
        let s = u32::from(tile_side);
        s.saturating_mul(s).saturating_mul(dirty_tile_count)
    }
}

#[cfg(test)]
mod tests {
    use super::DirtyRegionSet;

    /// TC-IR-3.3.4.3 — tile indices marked in random order drain ascending.
    #[test]
    fn drain_sorted_is_deterministic_ascending() {
        let mut dr = DirtyRegionSet::new(64, 16);
        for idx in [41u32, 3, 17, 3] {
            dr.mark(idx);
        }
        let tiles = dr.drain_sorted().to_vec();
        assert_eq!(tiles, vec![3, 17, 41]);
    }

    /// TC-IR-3.3.4.2 — clean grid yields no dirty tiles.
    #[test]
    fn no_dirty_tiles_when_unmarked() {
        let mut dr = DirtyRegionSet::new(32, 16);
        assert!(dr.drain_sorted().is_empty());
    }

    /// Marks out of range are ignored.
    #[test]
    fn mark_out_of_range_no_panic() {
        let mut dr = DirtyRegionSet::new(4, 16);
        dr.mark(999);
        assert!(dr.drain_sorted().is_empty());
    }

    /// After [`DirtyRegionSet::clear`], a subsequent drain is empty.
    #[test]
    fn clear_resets_dirty_state() {
        let mut dr = DirtyRegionSet::new(8, 16);
        dr.mark(1);
        assert_eq!(dr.drain_sorted(), &[1]);
        dr.clear();
        assert!(dr.drain_sorted().is_empty());
    }

    /// TC-IR-3.3.4.1 — one 16×16 dirty tile covers 256 cells.
    #[test]
    fn one_tile_partial_upload_cell_count() {
        assert_eq!(DirtyRegionSet::cell_upload_count_2d(16, 1), 256);
    }
}
