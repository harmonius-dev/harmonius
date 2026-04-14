//! Enumerated collider shapes shared by physics and geometry producers.

use glam::Vec3;

use crate::HeightfieldCollider;

/// Triangle mesh payload consumed by the narrowphase (IR-3.8.1 / IR-3.8.5).
#[derive(Clone, Debug, PartialEq)]
pub struct TriMeshData {
    /// Vertex positions in collider local space.
    pub vertices: Vec<Vec3>,
    /// Triangle corner indices (IR-3.8.5.4 triplet layout).
    pub indices: Vec<[u32; 3]>,
}

/// Physics collider shape variants referenced by the integration design.
#[derive(Clone, Debug, PartialEq)]
pub enum ColliderShape {
    /// Sphere primitive.
    Sphere {
        /// Radius in local units.
        radius: f32,
    },
    /// Axis-aligned box primitive.
    Box {
        /// Half-extents along each axis in local units.
        half_extents: Vec3,
    },
    /// Triangle mesh collider backed by [`TriMeshData`].
    TriangleMesh {
        /// Mesh payload.
        data: TriMeshData,
    },
    /// Heightfield collider (terrain tiles).
    Heightfield {
        /// Heightfield payload.
        field: HeightfieldCollider,
    },
}

impl ColliderShape {
    /// Returns `true` when this shape is a triangle mesh variant.
    #[must_use]
    pub const fn is_triangle_mesh(&self) -> bool {
        matches!(self, Self::TriangleMesh { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_3_8_1_1_unit_cube_collider_is_triangle_mesh() {
        let mut vertices = Vec::new();
        for x in [0.0_f32, 1.0] {
            for y in [0.0_f32, 1.0] {
                for z in [0.0_f32, 1.0] {
                    vertices.push(Vec3::new(x, y, z));
                }
            }
        }
        let indices = vec![
            [0, 1, 2],
            [2, 1, 3],
            [4, 5, 6],
            [6, 5, 7],
            [0, 2, 4],
            [4, 2, 6],
            [1, 3, 5],
            [5, 3, 7],
            [0, 4, 1],
            [1, 4, 5],
            [2, 6, 3],
            [3, 6, 7],
        ];
        let shape = ColliderShape::TriangleMesh {
            data: TriMeshData { vertices, indices },
        };
        let ColliderShape::TriangleMesh { data } = shape else {
            panic!("expected triangle mesh");
        };
        assert_eq!(data.indices.len(), 12);
    }
}
