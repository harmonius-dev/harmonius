use core::fmt;

/// Opaque slot identifier returned when inserting into a BVH.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BvhHandle {
    /// Stable slot index for removal and updates.
    pub slot: u32,
}

/// Integer chunk coordinate for voxel volumes (geometry subsystem contract).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ChunkCoord {
    /// Chunk index along +X.
    pub x: i32,
    /// Chunk index along +Y.
    pub y: i32,
    /// Chunk index along +Z.
    pub z: i32,
}

/// Indirection to a baked physics material asset.
///
/// Stand-in for the future `AssetId` newtype from the asset pipeline; keep `id` opaque to
/// callers outside tests.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PhysicsMaterialHandle {
    /// Stable asset identifier; `0` is reserved as invalid / unresolved in tests.
    pub id: u64,
}

/// Per-tile hole mask mirroring the geometry `TerrainHole` component layout.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerrainHole {
    /// Packed bits for the `resolution × resolution` hole grid.
    pub mask: Vec<u8>,
    /// Hole grid width and height in cells (square grid).
    pub resolution: u32,
}

impl TerrainHole {
    /// Returns `true` when the cell at `(row, col)` is a hole.
    ///
    /// Bit selection follows the integration design: linear index `row * resolution + col`
    /// indexes a single bit stream laid out in little-endian bit order within each byte (bit 0 of
    /// `mask[0]` is cell `(0, 0)`, bit 1 is `(0, 1)` when it fits in the same byte, etc.).
    /// Geometry `TerrainHole` producers must match this layout (FM-4) before any mmap or shared
    /// buffer handoff.
    #[must_use]
    pub fn cell_is_hole(&self, row: u32, col: u32) -> bool {
        if self.resolution == 0 {
            return false;
        }
        if row >= self.resolution || col >= self.resolution {
            return false;
        }
        let idx = (row * self.resolution + col) as usize;
        let byte = idx / 8;
        let bit = idx % 8;
        self.mask
            .get(byte)
            .is_some_and(|value| (value >> bit) & 1 == 1)
    }
}

impl fmt::Display for PhysicsMaterialHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PhysicsMaterialHandle({})", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_3_8_4_1_hole_cell_flags_fallthrough_region() {
        let resolution = 8_u32;
        let row = 5_u32;
        let col = 5_u32;
        let mut mask = vec![0_u8; 8];
        let bit_index = (row * resolution + col) as usize;
        mask[bit_index / 8] |= 1 << (bit_index % 8);
        let holes = TerrainHole { mask, resolution };
        assert!(holes.cell_is_hole(row, col));
    }

    #[test]
    fn tc_ir_3_8_4_2_adjacent_cell_not_hole() {
        let resolution = 8_u32;
        let row = 5_u32;
        let col = 5_u32;
        let mut mask = vec![0_u8; 8];
        let bit_index = (row * resolution + col) as usize;
        mask[bit_index / 8] |= 1 << (bit_index % 8);
        let holes = TerrainHole { mask, resolution };
        assert!(holes.cell_is_hole(5, 5));
        assert!(!holes.cell_is_hole(6, 5));
    }
}
