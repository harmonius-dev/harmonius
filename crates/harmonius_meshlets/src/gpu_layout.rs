use crate::MeshletAsset;

/// GPU vertex stride: `vec3` position + `vec3` normal + `vec2` UV (`R-2.4.5`).
#[must_use]
pub const fn vertex_stride_bytes() -> u32 {
    32
}

/// Validates buffer views for alignment and vertex stride invariants (`TC-2.4.5.1`, `TC-2.4.5.2`).
#[must_use]
pub fn meshlet_asset_gpu_layout_valid(asset: &MeshletAsset) -> bool {
    let v = asset.vertex_buffer;
    let i = asset.index_buffer;
    let m = asset.meshlet_buffer;
    v.stride == vertex_stride_bytes()
        && v.offset % 16 == 0
        && i.offset % 16 == 0
        && m.offset % 16 == 0
        && m.stride as usize == core::mem::size_of::<crate::Meshlet>()
}
