use bytemuck::Pod;
use bytemuck::Zeroable;
use rkyv_derive::Archive;
use rkyv_derive::Deserialize;
use rkyv_derive::Serialize;

/// Stable asset identifier (content-addressable in the full engine; here a numeric id).
#[derive(
    Archive, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct AssetId(
    /// Stable numeric identifier.
    pub u64,
);

/// Byte range into an rkyv-mapped archive or GPU upload staging blob.
#[derive(Archive, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferView {
    /// Byte offset from the start of the containing buffer.
    pub offset: u64,
    /// Span in bytes.
    pub size: u64,
    /// Stride for structured views (vertex buffer uses 32 bytes).
    pub stride: u32,
}

/// One LOD level: a full meshlet set plus aggregate bounds and screen-space error metadata.
#[derive(Archive, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LodGroup {
    /// Monotonic LOD level (`0` is highest detail).
    pub level: u8,
    /// Screen-space error metric for this level (must increase with `level`).
    pub screen_error: f32,
    /// Byte offset into [`MeshletAsset::index_data`] where this LOD's packed `u32` indices begin.
    pub index_byte_start: u64,
    /// Span in bytes of this LOD's packed `u32` triangle indices (`index_byte_count % 12 == 0`).
    pub index_byte_count: u64,
    /// Meshlets referencing the shared vertex/index streams for this LOD.
    pub meshlets: Vec<Meshlet>,
    /// Conservative bounds for the entire LOD level.
    pub bounds_center: [f32; 3],
    /// Bounding sphere radius for the LOD aggregate.
    pub bounds_radius: f32,
}

/// Per-meshlet header stored in the meshlet SSBO (`R-2.4.2`, `R-2.4.5`, `R-2.4.8`, `R-2.4.9`).
#[repr(C)]
#[derive(Archive, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct Meshlet {
    /// Byte offset into [`MeshletAsset::meshlet_vertex_index_data`] for this meshlet's packed
    /// global vertex indices (`u32` LE, [`Meshlet::vertex_count`] entries). Micro-triangles index
    /// this list (meshoptimizer layout).
    pub vertex_start: u32,
    /// Number of vertices referenced by this meshlet (≤ 64).
    pub vertex_count: u8,
    /// Padding to keep `triangle_start` four-byte aligned.
    pub pad_vertex: [u8; 3],
    /// Byte offset into the packed u8 index stream for this meshlet's micro triangles.
    pub triangle_start: u32,
    /// Number of triangles in this meshlet (≤ 124).
    pub triangle_count: u8,
    /// Padding before `Vec3` fields.
    pub pad_triangle: [u8; 3],
    /// Normal-cone apex used for perspective correct culling tests.
    pub cone_apex: [f32; 3],
    /// Unit axis for the baked normal cone (`R-2.4.8`).
    pub cone_axis: [f32; 3],
    /// Opening angle in radians derived from meshoptimizer's cutoff (`R-2.4.8`).
    pub cone_angle: f32,
    /// World-space bounds center for frustum/occlusion tests (`R-2.4.9`).
    pub bounds_center: [f32; 3],
    /// Bounding sphere radius (`R-2.4.9`).
    pub bounds_radius: f32,
    /// Padding to 64 bytes for GPU struct layout.
    pub pad_tail: [u8; 4],
}

/// Root mesh representation: shared vertex/index buffers plus per-LOD meshlet metadata (`R-2.4.1`).
#[derive(Archive, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MeshletAsset {
    /// Asset id assigned by the caller.
    pub id: AssetId,
    /// Monotonic authoring version.
    pub version: u32,
    /// Total vertices in the packed vertex buffer.
    pub vertex_count: u32,
    /// Total u32 indices across every packed LOD (`index_data.len() / 4`).
    pub index_count: u32,
    /// LOD levels, ordered by increasing `level`.
    pub lod_groups: Vec<LodGroup>,
    /// Packed SoA vertex bytes (see [`crate::vertex_stride_bytes`]).
    pub vertex_buffer: BufferView,
    /// Global index buffer (little-endian `u32` indices).
    pub index_buffer: BufferView,
    /// Packed [`Meshlet`] headers.
    pub meshlet_buffer: BufferView,
    /// Packed global vertex indices per meshlet (`u32` LE; see each [`Meshlet::vertex_start`]).
    pub meshlet_vertex_index_buffer: BufferView,
    /// Packed meshoptimizer micro-triangle indices (`u8`; see each [`Meshlet::triangle_start`]).
    pub meshlet_triangle_buffer: BufferView,
    /// Deterministic fingerprint of the authored source mesh (`R-2.4.1`).
    pub source_hash: [u8; 32],
    /// Bytes backing `vertex_buffer` (32-byte stride vertices).
    pub vertex_data: Vec<u8>,
    /// Bytes backing `index_buffer` (`u32` triangle corners).
    pub index_data: Vec<u8>,
    /// Bytes backing `meshlet_buffer` (array of [`Meshlet`]).
    pub meshlet_data: Vec<u8>,
    /// Bytes backing `meshlet_vertex_index_buffer` (global indices into the compact vertex buffer).
    pub meshlet_vertex_index_data: Vec<u8>,
    /// Bytes backing `meshlet_triangle_buffer` (`u8` locals into each meshlet vertex list).
    pub meshlet_triangle_data: Vec<u8>,
}
