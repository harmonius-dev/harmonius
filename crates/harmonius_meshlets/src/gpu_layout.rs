use crate::MeshletAsset;

/// GPU vertex stride: `vec3` position + `vec3` normal + `vec2` UV (`R-2.4.5`).
#[must_use]
pub const fn vertex_stride_bytes() -> u32 {
    32
}

/// Validates buffer views for alignment, strides, and backing slice sizes (`TC-2.4.5.1`,
/// `TC-2.4.5.2`).
#[must_use]
pub fn meshlet_asset_gpu_layout_valid(asset: &MeshletAsset) -> bool {
    let v = asset.vertex_buffer;
    let i = asset.index_buffer;
    let m = asset.meshlet_buffer;
    let mv = asset.meshlet_vertex_index_buffer;
    let mt = asset.meshlet_triangle_buffer;
    let meshlet_stride = core::mem::size_of::<crate::Meshlet>();
    let index_u32s = asset.index_data.len() / 4;
    let index_sum_ok = asset.lod_groups.iter().all(|g| {
        g.index_byte_start % 4 == 0
            && g.index_byte_count % 4 == 0
            && g.index_byte_start
                .checked_add(g.index_byte_count)
                .map(|end| end as usize <= asset.index_data.len())
                .unwrap_or(false)
    });
    let meshlet_vertices_ok = asset.lod_groups.iter().all(|g| {
        g.meshlets.iter().all(|ml| {
            let start = ml.vertex_start as usize;
            let need = (ml.vertex_count as usize).saturating_mul(4);
            start
                .checked_add(need)
                .map(|end| end <= asset.meshlet_vertex_index_data.len())
                .unwrap_or(false)
        })
    });
    let meshlet_tris_ok = asset.lod_groups.iter().all(|g| {
        g.meshlets
            .iter()
            .all(|ml| (ml.triangle_start as usize) <= asset.meshlet_triangle_data.len())
    });
    v.stride == vertex_stride_bytes()
        && v.offset % 16 == 0
        && i.offset % 16 == 0
        && m.offset % 16 == 0
        && mv.offset % 4 == 0
        && mt.offset % 4 == 0
        && m.stride as usize == meshlet_stride
        && mv.stride == 4
        && mt.stride == 1
        && v.size == asset.vertex_data.len() as u64
        && i.size == asset.index_data.len() as u64
        && m.size == asset.meshlet_data.len() as u64
        && mv.size == asset.meshlet_vertex_index_data.len() as u64
        && mt.size == asset.meshlet_triangle_data.len() as u64
        && asset.vertex_data.len() % usize::try_from(v.stride).unwrap_or(1) == 0
        && index_u32s * 4 == asset.index_data.len()
        && asset.index_count as usize == index_u32s
        && asset.meshlet_data.len() % meshlet_stride == 0
        && index_sum_ok
        && meshlet_vertices_ok
        && meshlet_tris_ok
}
