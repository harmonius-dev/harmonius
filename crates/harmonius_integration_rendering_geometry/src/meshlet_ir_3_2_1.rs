//! IR-3.2.1 meshlet DAG → GPU culling contracts (CPU-side validation).

use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};

use crate::contracts::GpuMeshletCluster;

/// Six frustum planes in Hessian form (xyz unit normal, w offset).
pub type FrustumPlanes = [Vec4; 6];

/// Axis-aligned depth grid for HZB-style occlusion tests (tests only).
#[derive(Clone, Debug)]
pub struct DepthGrid {
    /// Per-cell closest occluder Z in view space (+Z forward from the camera origin).
    pub occluder_z: Vec<f32>,
    /// Horizontal resolution of the grid (cells).
    pub width: usize,
    /// Vertical resolution of the grid (cells).
    pub height: usize,
}

/// Errors when validating meshlet GPU payloads at load time.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MeshletLoadError {
    /// Byte length does not match a whole number of clusters.
    InvalidLayout,
    /// Declared cluster count disagrees with buffer size.
    InvalidClusterCount,
}

/// Packs clusters into a GPU upload buffer using native `#[repr(C)]` layout.
#[must_use]
pub fn encode_meshlet_clusters_gpu_buffer(clusters: &[GpuMeshletCluster]) -> Vec<u8> {
    let sz = std::mem::size_of::<GpuMeshletCluster>();
    let mut out = Vec::with_capacity(clusters.len().saturating_mul(sz));
    for c in clusters {
        let bytes = unsafe {
            std::slice::from_raw_parts(
                std::ptr::from_ref::<GpuMeshletCluster>(c).cast::<u8>(),
                sz,
            )
        };
        out.extend_from_slice(bytes);
    }
    out
}

fn normalize_plane(mut p: Vec4) -> Vec4 {
    let n = p.xyz();
    let len = n.length();
    if len < 1e-8 {
        return p;
    }
    let inv = 1.0 / len;
    p.x *= inv;
    p.y *= inv;
    p.z *= inv;
    p.w *= inv;
    p
}

/// Builds frustum planes from a column-major view-projection matrix (clip = VP * world).
#[must_use]
pub fn frustum_planes_from_view_proj(vp: Mat4) -> FrustumPlanes {
    let c0 = vp.col(0);
    let c1 = vp.col(1);
    let c2 = vp.col(2);
    let c3 = vp.col(3);
    let raw = [
        c3 + c0,
        c3 - c0,
        c3 + c1,
        c3 - c1,
        c3 + c2,
        c3 - c2,
    ];
    raw.map(normalize_plane)
}

/// Returns `true` when the cluster's bounding sphere is inside or crossing the frustum.
#[must_use]
pub fn cluster_visible_frustum(cluster: &GpuMeshletCluster, planes: &FrustumPlanes) -> bool {
    let c = cluster.bounding_sphere;
    let center = Vec3::new(c.x, c.y, c.z);
    let r = c.w;
    for p in planes {
        let n = p.xyz();
        let d = p.w;
        if n.dot(center) + d < -r {
            return false;
        }
    }
    true
}

/// Returns `true` when the cluster is fully occluded by the depth grid (conservative test).
#[must_use]
pub fn cluster_fully_occluded_hzb(
    cluster: &GpuMeshletCluster,
    planes: &FrustumPlanes,
    hzb: &DepthGrid,
) -> bool {
    if !cluster_visible_frustum(cluster, planes) {
        return false;
    }
    let c = cluster.bounding_sphere;
    let z_front = c.z - c.w;
    let (ix, iy) = ndc_cell(c.x, c.y, hzb.width, hzb.height);
    let idx = iy * hzb.width + ix;
    let wall = hzb.occluder_z.get(idx).copied().unwrap_or(f32::NEG_INFINITY);
    z_front > wall
}

/// Maps approximate NDC-like coordinates in x,y to a grid cell (clamped).
fn ndc_cell(x: f32, y: f32, w: usize, h: usize) -> (usize, usize) {
    let fx = (x + 1.0) * 0.5 * w as f32;
    let fy = (y + 1.0) * 0.5 * h as f32;
    let ix = fx.floor() as isize;
    let iy = fy.floor() as isize;
    let ix = ix.clamp(0, w.saturating_sub(1) as isize) as usize;
    let iy = iy.clamp(0, h.saturating_sub(1) as isize) as usize;
    (ix, iy)
}

/// `view_dir` is the normalized direction **from the camera position toward the cluster
/// center** (camera → surface).
///
/// Returns `true` when the cone test rejects the cluster for backface culling.
#[must_use]
pub fn cluster_culled_backface(cone_axis_cos: Vec4, view_dir: Vec3) -> bool {
    let axis = cone_axis_cos.xyz().normalize_or_zero();
    let v = view_dir.normalize_or_zero();
    if axis.length_squared() < 1e-12 || v.length_squared() < 1e-12 {
        return false;
    }
    v.dot(axis) < cone_axis_cos.w
}

/// Two-phase HZB visibility: phase-2 is authoritative when it runs.
#[must_use]
pub fn two_phase_hzb_final_visible(phase1_visible: bool, phase2_visible: bool) -> bool {
    let _ = phase1_visible;
    phase2_visible
}

/// Validates that a byte buffer can hold `expected_clusters` clusters.
pub fn validate_meshlet_cluster_buffer(
    byte_len: usize,
    expected_clusters: u32,
) -> Result<(), MeshletLoadError> {
    let sz = std::mem::size_of::<GpuMeshletCluster>();
    if sz == 0 || byte_len % sz != 0 {
        return Err(MeshletLoadError::InvalidLayout);
    }
    let n = byte_len / sz;
    if n != expected_clusters as usize {
        return Err(MeshletLoadError::InvalidClusterCount);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unit_cluster_at(origin: Vec3, radius: f32) -> GpuMeshletCluster {
        GpuMeshletCluster {
            bounding_sphere: Vec4::new(origin.x, origin.y, origin.z, radius),
            normal_cone: Vec4::new(0.0, 0.0, 1.0, 0.9),
            parent_error: 0.0,
            lod_error: 0.0,
            vertex_offset: 0,
            triangle_offset: 0,
            vertex_count: 0,
            triangle_count: 0,
            _pad: [0, 0],
        }
    }

    /// Open symmetric NDC cube [-1, 1] on x, y, z: inside iff all plane half-spaces hold.
    fn ndc_box_frustum() -> FrustumPlanes {
        [
            Vec4::new(1.0, 0.0, 0.0, 1.0),
            Vec4::new(-1.0, 0.0, 0.0, 1.0),
            Vec4::new(0.0, 1.0, 0.0, 1.0),
            Vec4::new(0.0, -1.0, 0.0, 1.0),
            Vec4::new(0.0, 0.0, 1.0, 1.0),
            Vec4::new(0.0, 0.0, -1.0, 1.0),
        ]
    }

    #[test]
    fn tc_ir_3_2_1_1_byte_identical_upload() {
        let clusters: Vec<GpuMeshletCluster> = (0..8)
            .map(|i| GpuMeshletCluster {
                bounding_sphere: Vec4::splat(i as f32),
                normal_cone: Vec4::ONE,
                parent_error: i as f32,
                lod_error: (i + 1) as f32,
                vertex_offset: i,
                triangle_offset: i + 10,
                vertex_count: (i % 255) as u8,
                triangle_count: ((i + 3) % 255) as u8,
                _pad: [0, 0],
            })
            .collect();
        let gpu = encode_meshlet_clusters_gpu_buffer(&clusters);
        assert_eq!(gpu.len(), clusters.len() * std::mem::size_of::<GpuMeshletCluster>());
        for (i, c) in clusters.iter().enumerate() {
            let off = i * std::mem::size_of::<GpuMeshletCluster>();
            let slice = &gpu[off..off + std::mem::size_of::<GpuMeshletCluster>()];
            let decoded = unsafe {
                std::ptr::read_unaligned(slice.as_ptr().cast::<GpuMeshletCluster>())
            };
            assert_eq!(*c, decoded);
        }
    }

    #[test]
    fn tc_ir_3_2_1_2_frustum_half_rejected() {
        let planes = ndc_box_frustum();
        let mut inside = 0u32;
        let mut outside = 0u32;
        for i in 0..100u32 {
            let x = if i < 50 {
                (i as f32 / 50.0) * 0.4
            } else {
                1.25 + (i as f32 - 50.0) / 50.0 * 0.8
            };
            let c = unit_cluster_at(Vec3::new(x, 0.0, 0.0), 0.02);
            if cluster_visible_frustum(&c, &planes) {
                inside += 1;
            } else {
                outside += 1;
            }
        }
        assert_eq!(inside, 50, "inside={inside}");
        assert_eq!(outside, 50, "outside={outside}");
    }

    #[test]
    fn tc_ir_3_2_1_3_hzb_occlusion() {
        let planes = ndc_box_frustum();
        let hzb = DepthGrid {
            occluder_z: vec![0.58; 16 * 16],
            width: 16,
            height: 16,
        };
        let mut occluded = 0u32;
        let mut visible = 0u32;
        for i in 0..100u32 {
            let z = 0.2 + (i as f32 / 99.0) * 0.6;
            let c = unit_cluster_at(Vec3::new(0.0, 0.0, z), 0.05);
            if cluster_fully_occluded_hzb(&c, &planes, &hzb) {
                occluded += 1;
            } else {
                visible += 1;
            }
        }
        assert!(
            (25..=38).contains(&occluded),
            "expected ~30 occluded, got {occluded}"
        );
        assert!(
            (62..=75).contains(&visible),
            "expected ~70 visible, got {visible}"
        );
    }

    #[test]
    fn tc_ir_3_2_1_4_backface_cone_rejects() {
        let cone = Vec4::new(0.0, 0.0, 1.0, 0.9);
        let camera_toward_cluster = Vec3::new(0.0, 0.0, -1.0);
        assert!(cluster_culled_backface(cone, camera_toward_cluster));
    }

    #[test]
    fn tc_ir_3_2_1_5_two_phase_phase2_wins() {
        assert!(two_phase_hzb_final_visible(false, true));
        assert!(!two_phase_hzb_final_visible(true, false));
        assert!(!two_phase_hzb_final_visible(false, false));
        assert!(two_phase_hzb_final_visible(true, true));
    }

    #[test]
    fn tc_ir_3_2_1_n1_invalid_buffer_rejected() {
        let sz = std::mem::size_of::<GpuMeshletCluster>();
        assert_eq!(
            validate_meshlet_cluster_buffer(sz + 1, 1),
            Err(MeshletLoadError::InvalidLayout)
        );
        assert_eq!(
            validate_meshlet_cluster_buffer(sz * 2, 1),
            Err(MeshletLoadError::InvalidClusterCount)
        );
        assert!(validate_meshlet_cluster_buffer(sz * 3, 3).is_ok());
    }

    #[test]
    fn frustum_planes_include_origin_for_identity_clip() {
        let vp = Mat4::IDENTITY;
        let planes = frustum_planes_from_view_proj(vp);
        let c = unit_cluster_at(Vec3::ZERO, 0.01);
        assert!(cluster_visible_frustum(&c, &planes));
    }
}
