//! Data contracts shared by rendering and geometry for meshlet integration.

use glam::Vec4;

/// Per-meshlet cluster uploaded to GPU for culling.
///
/// Immutable after bake -- produced offline, uploaded once per asset.
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpuMeshletCluster {
    /// World-space bounding sphere center (xyz) + radius (w).
    pub bounding_sphere: Vec4,
    /// Cone axis (xyz) + cos(half-angle) (w).
    pub normal_cone: Vec4,
    /// Parent LOD error bound from the meshlet DAG.
    pub parent_error: f32,
    /// Cluster LOD screen-space error bound.
    pub lod_error: f32,
    /// Byte or vertex offset into the mesh vertex buffer.
    pub vertex_offset: u32,
    /// Index or triangle offset into mesh index data.
    pub triangle_offset: u32,
    /// Number of vertices referenced by this cluster.
    pub vertex_count: u8,
    /// Number of triangles in this cluster.
    pub triangle_count: u8,
    /// Padding to satisfy `#[repr(C, align(16))]` layout.
    pub _pad: [u8; 2],
}
