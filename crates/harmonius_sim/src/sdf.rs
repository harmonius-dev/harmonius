//! CPU-side dirty region tracking for voxel-derived SDF updates.

use crate::coord::{ChunkCoord, VoxelCoord};

/// Bounding region (inclusive) that must be re-evaluated after a voxel edit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SdfDirtyRegion {
    /// Minimum affected voxel coordinate.
    pub min: VoxelCoord,
    /// Maximum affected voxel coordinate.
    pub max: VoxelCoord,
}

/// Expands a single-voxel mutation to a conservative neighborhood in voxel space.
pub fn recompute_sdf_dirty_region(
    volume_dims: VoxelCoord,
    chunk_size: u32,
    mutation: VoxelCoord,
) -> SdfDirtyRegion {
    let _ = chunk_coord_of(mutation, chunk_size);
    let pad = chunk_size.saturating_mul(2);
    let min = VoxelCoord {
        x: mutation.x.saturating_sub(pad),
        y: mutation.y.saturating_sub(pad),
        z: mutation.z.saturating_sub(pad),
    };
    let max = VoxelCoord {
        x: (mutation.x + pad).min(volume_dims.x.saturating_sub(1)),
        y: (mutation.y + pad).min(volume_dims.y.saturating_sub(1)),
        z: (mutation.z + pad).min(volume_dims.z.saturating_sub(1)),
    };
    SdfDirtyRegion { min, max }
}

fn chunk_coord_of(v: VoxelCoord, chunk_size: u32) -> ChunkCoord {
    ChunkCoord {
        x: v.x / chunk_size,
        y: v.y / chunk_size,
        z: v.z / chunk_size,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdf_dirty_region_scope() {
        let dims = VoxelCoord {
            x: 128,
            y: 128,
            z: 128,
        };
        let region = recompute_sdf_dirty_region(dims, 8, VoxelCoord { x: 8, y: 8, z: 8 });
        assert!(region.min.x <= 8);
        assert!(region.max.x >= 8);
        assert!(region.max.x < dims.x);
    }
}
