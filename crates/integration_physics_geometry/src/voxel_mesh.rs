//! Collision meshes produced from voxel volumes.

use glam::Vec3;

use crate::{ChunkCoord, PhysicsMaterialHandle, TriMeshData};

/// Collision mesh extracted from a single voxel chunk (IR-3.8.5).
#[derive(Clone, Debug, PartialEq)]
pub struct VoxelCollisionMesh {
    /// Vertex positions in chunk-local space.
    pub vertices: Vec<Vec3>,
    /// Triangle triplets compatible with [`TriMeshData`].
    pub indices: Vec<[u32; 3]>,
    /// Owning chunk coordinate in world voxel space.
    pub chunk_coord: ChunkCoord,
    /// Material handle resolved later by the physics runtime.
    pub material: PhysicsMaterialHandle,
}

impl VoxelCollisionMesh {
    /// Projects this mesh into the shared [`TriMeshData`] contract (IR-3.8.5.4).
    #[must_use]
    pub fn as_trimesh_data(&self) -> TriMeshData {
        TriMeshData {
            vertices: self.vertices.clone(),
            indices: self.indices.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_3_8_5_4_index_triplet_layout_verified() {
        let mesh = VoxelCollisionMesh {
            vertices: vec![Vec3::ZERO, Vec3::X, Vec3::Y],
            indices: vec![[0, 1, 2]],
            chunk_coord: ChunkCoord { x: 0, y: 0, z: 0 },
            material: PhysicsMaterialHandle { id: 1 },
        };
        let data = mesh.as_trimesh_data();
        assert_eq!(data.indices, vec![[0, 1, 2]]);
    }
}
