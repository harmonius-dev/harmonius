//! Test cases from `docs/design/rendering/meshlets-test-cases.md`.

use glam::Mat3;
use glam::Vec2;
use glam::Vec3;
use harmonius_meshlets::meshlet_asset_gpu_layout_valid;
use harmonius_meshlets::AssetId;
use harmonius_meshlets::BuildError;
use harmonius_meshlets::Meshlet;
use harmonius_meshlets::MeshletAsset;
use harmonius_meshlets::MeshletBuilder;
use harmonius_meshlets::NormalizedMesh;
use meshopt::clusterize;
use meshopt::optimize;
use meshopt::remap;
use meshopt::typed_to_bytes;
use meshopt::Vertex;
use meshopt::VertexDataAdapter;
use rkyv::from_bytes;
use rkyv::rancor::Error;
use rkyv::to_bytes;
use std::mem::size_of;

fn unit_cube_mesh() -> NormalizedMesh {
    let p = [
        [0.0f32, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
        [1.0, 1.0, 1.0],
        [0.0, 1.0, 1.0],
    ];
    let positions: Vec<Vec3> = p.iter().map(|a| Vec3::from_array(*a)).collect();
    let normals = vec![Vec3::Z; 8];
    let uvs = vec![Vec2::ZERO; 8];
    let indices: Vec<u32> = vec![
        0, 1, 2, 0, 2, 3, // bottom
        4, 6, 5, 4, 7, 6, // top
        0, 4, 5, 0, 5, 1, // front
        1, 5, 6, 1, 6, 2, // right
        2, 6, 7, 2, 7, 3, // back
        3, 7, 4, 3, 4, 0, // left
    ];
    NormalizedMesh {
        positions,
        normals,
        uvs,
        indices,
    }
}

fn non_manifold_mesh() -> NormalizedMesh {
    let positions = vec![
        Vec3::ZERO,
        Vec3::X,
        Vec3::Y,
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 1.0, 1.0),
    ];
    let normals = vec![Vec3::Z; 5];
    let uvs = vec![Vec2::ZERO; 5];
    let indices = vec![0u32, 1, 2, 0, 1, 3, 0, 1, 4];
    NormalizedMesh {
        positions,
        normals,
        uvs,
        indices,
    }
}

fn uv_sphere_mesh(stacks: u32, slices: u32) -> NormalizedMesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    for iy in 0..=stacks {
        let v = iy as f32 / stacks as f32;
        let phi = v * std::f32::consts::PI;
        for ix in 0..=slices {
            let u = ix as f32 / slices as f32;
            let theta = u * std::f32::consts::TAU;
            let x = phi.sin() * theta.cos();
            let y = phi.cos();
            let z = phi.sin() * theta.sin();
            let n = Vec3::new(x, y, z);
            positions.push(n);
            normals.push(n);
            uvs.push(Vec2::new(u, v));
        }
    }
    let mut indices = Vec::new();
    for iy in 0..stacks {
        for ix in 0..slices {
            let a = iy * (slices + 1) + ix;
            let b = a + slices + 1;
            indices.extend_from_slice(&[a, b, a + 1, b, b + 1, a + 1]);
        }
    }
    NormalizedMesh {
        positions,
        normals,
        uvs,
        indices,
    }
}

fn grid_mesh(w: u32, h: u32) -> NormalizedMesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    for j in 0..h {
        for i in 0..w {
            let x = i as f32 / (w - 1) as f32;
            let z = j as f32 / (h - 1) as f32;
            positions.push(Vec3::new(x, 0.0, z));
            normals.push(Vec3::Y);
            uvs.push(Vec2::new(x, z));
        }
    }
    let mut indices = Vec::new();
    for j in 0..h - 1 {
        for i in 0..w - 1 {
            let i0 = j * w + i;
            let i1 = i0 + 1;
            let i2 = i0 + w;
            let i3 = i2 + 1;
            indices.extend_from_slice(&[i0, i2, i1, i1, i2, i3]);
        }
    }
    NormalizedMesh {
        positions,
        normals,
        uvs,
        indices,
    }
}

fn single_triangle() -> NormalizedMesh {
    NormalizedMesh {
        positions: vec![Vec3::ZERO, Vec3::X, Vec3::Y],
        normals: vec![Vec3::Z; 3],
        uvs: vec![Vec2::ZERO; 3],
        indices: vec![0, 1, 2],
    }
}

/// TC-2.4.1.1
#[test]
fn test_meshlet_asset_roundtrip_rkyv() {
    let mesh = uv_sphere_mesh(24, 24);
    let asset = MeshletBuilder::new(mesh)
        .id(AssetId(7))
        .build()
        .expect("build");
    assert!(
        asset.lod_groups[0].meshlets.len() >= 10,
        "fixture should yield >=10 meshlets"
    );
    let bytes = to_bytes::<Error>(&asset).expect("serialize");
    let round: MeshletAsset =
        from_bytes::<MeshletAsset, Error>(bytes.as_slice()).expect("deserialize");
    assert_eq!(round, asset);
}

/// TC-2.4.1.2
#[test]
fn test_source_hash_stable() {
    let mesh = unit_cube_mesh();
    let a = MeshletBuilder::new(mesh.clone()).build().expect("a");
    let b = MeshletBuilder::new(mesh).build().expect("b");
    assert_eq!(a.source_hash, b.source_hash);
}

/// TC-2.4.2.1
#[test]
fn test_meshlet_vertex_cap_64() {
    let mesh = grid_mesh(100, 100);
    let asset = MeshletBuilder::new(mesh).build().expect("build");
    for lg in &asset.lod_groups {
        for m in &lg.meshlets {
            assert!(m.vertex_count <= 64, "vertex_count {}", m.vertex_count);
        }
    }
}

/// TC-2.4.2.2
#[test]
fn test_meshlet_triangle_cap_124() {
    let mesh = grid_mesh(100, 100);
    let asset = MeshletBuilder::new(mesh).build().expect("build");
    for lg in &asset.lod_groups {
        for m in &lg.meshlets {
            assert!(
                m.triangle_count <= 124,
                "triangle_count {}",
                m.triangle_count
            );
        }
    }
}

/// TC-2.4.2.3
#[test]
fn test_meshlet_struct_size() {
    assert_eq!(size_of::<Meshlet>(), 64);
}

/// TC-2.4.3.1
#[test]
fn test_lod_group_hierarchy_monotonic() {
    let mesh = unit_cube_mesh();
    let asset = MeshletBuilder::new(mesh)
        .simplify_lods(1)
        .build()
        .expect("build");
    let levels: Vec<u8> = asset.lod_groups.iter().map(|g| g.level).collect();
    for (i, &lv) in levels.iter().enumerate() {
        assert_eq!(lv, i as u8);
    }
}

/// TC-2.4.3.2
#[test]
fn test_lod_screen_error_increases() {
    let mesh = unit_cube_mesh();
    let asset = MeshletBuilder::new(mesh)
        .simplify_lods(2)
        .build()
        .expect("build");
    for w in asset.lod_groups.windows(2) {
        assert!(w[1].screen_error > w[0].screen_error);
    }
}

/// TC-2.4.4.1
#[test]
fn test_builder_determinism_cube() {
    let mesh = unit_cube_mesh();
    let a = MeshletBuilder::new(mesh.clone()).build().expect("a");
    let b = MeshletBuilder::new(mesh).build().expect("b");
    assert_eq!(a.vertex_data, b.vertex_data);
    assert_eq!(a.index_data, b.index_data);
    assert_eq!(a.meshlet_data, b.meshlet_data);
    assert_eq!(a.meshlet_triangle_data, b.meshlet_triangle_data);
}

/// TC-2.4.4.2
#[test]
fn test_builder_determinism_sphere() {
    let mesh = uv_sphere_mesh(32, 32);
    let a = MeshletBuilder::new(mesh.clone()).build().expect("a");
    let b = MeshletBuilder::new(mesh).build().expect("b");
    assert_eq!(a.vertex_data, b.vertex_data);
    assert_eq!(a.index_data, b.index_data);
    assert_eq!(a.meshlet_data, b.meshlet_data);
}

/// TC-2.4.4.3
#[test]
fn test_builder_rejects_non_manifold() {
    let mesh = non_manifold_mesh();
    let err = MeshletBuilder::new(mesh).build().expect_err("non-manifold");
    assert_eq!(err, BuildError::InvalidTopology);
}

/// TC-2.4.4.4
#[test]
fn test_builder_simplify_failure() {
    let mesh = single_triangle();
    let err = MeshletBuilder::new(mesh)
        .simplify_lods(1)
        .build()
        .expect_err("simplify");
    match err {
        BuildError::SimplifyFailed { .. } => {}
        other => panic!("unexpected {other:?}"),
    }
}

/// TC-2.4.5.1
#[test]
fn test_gpu_layout_alignment() {
    let mesh = unit_cube_mesh();
    let asset = MeshletBuilder::new(mesh).build().expect("build");
    assert!(meshlet_asset_gpu_layout_valid(&asset));
}

/// TC-2.4.5.2
#[test]
fn test_buffer_view_stride_matches() {
    let mesh = unit_cube_mesh();
    let asset = MeshletBuilder::new(mesh).build().expect("build");
    assert_eq!(asset.vertex_buffer.stride, 32);
}

/// TC-2.4.8.1
#[test]
fn test_cone_axis_unit_length() {
    let mesh = uv_sphere_mesh(12, 12);
    let asset = MeshletBuilder::new(mesh).build().expect("build");
    for lg in &asset.lod_groups {
        for m in &lg.meshlets {
            let ax = Vec3::from_array(m.cone_axis);
            assert!((ax.length() - 1.0).abs() < 1e-5);
        }
    }
}

/// TC-2.4.8.2
#[test]
fn test_cone_angle_in_range() {
    let mesh = uv_sphere_mesh(12, 12);
    let asset = MeshletBuilder::new(mesh).build().expect("build");
    for lg in &asset.lod_groups {
        for m in &lg.meshlets {
            assert!(m.cone_angle >= 0.0 && m.cone_angle <= std::f32::consts::PI);
        }
    }
}

fn meshopt_lod0_session(mesh: &NormalizedMesh) -> (Vec<Vertex>, meshopt::clusterize::Meshlets) {
    let verts: Vec<Vertex> = mesh
        .positions
        .iter()
        .enumerate()
        .map(|(i, p)| Vertex {
            p: p.to_array(),
            n: mesh.normals[i].to_array(),
            t: mesh.uvs[i].to_array(),
        })
        .collect();
    let (unique_count, remap) = remap::generate_vertex_remap(&verts, Some(&mesh.indices));
    let remapped = remap::remap_index_buffer(Some(&mesh.indices), verts.len(), &remap);
    let compact: Vec<Vertex> = remap::remap_vertex_buffer(&verts, unique_count, &remap);
    let optimized = optimize::optimize_vertex_cache(&remapped, unique_count);
    let adapter =
        VertexDataAdapter::new(typed_to_bytes(&compact), size_of::<Vertex>(), 0).expect("adapter");
    let meshlets = clusterize::build_meshlets(&optimized, &adapter, 64, 124, 0.5);
    (compact, meshlets)
}

/// TC-2.4.9.1
#[test]
fn test_bounds_sphere_encloses_triangles() {
    let mesh = unit_cube_mesh();
    let asset = MeshletBuilder::new(mesh.clone()).build().expect("build");
    let (compact, meshlets) = meshopt_lod0_session(&mesh);
    let adapter =
        VertexDataAdapter::new(typed_to_bytes(&compact), size_of::<Vertex>(), 0).expect("adapter");
    for i in 0..meshlets.len() {
        let mh = meshlets.get(i);
        let mm = &asset.lod_groups[0].meshlets[i];
        let c = Vec3::from_array(mm.bounds_center);
        let r = mm.bounds_radius;
        for &gv in mh.vertices {
            let p = Vec3::from(compact[gv as usize].p);
            assert!(
                (p - c).length() <= r + 2e-3,
                "vertex outside baked bounds sphere"
            );
        }
        let b = clusterize::compute_meshlet_bounds(mh, &adapter);
        assert!(
            (Vec3::from_array(b.center) - c).length() < 1e-2,
            "center drift vs meshopt"
        );
        assert!((b.radius - r).abs() < 1e-2, "radius drift vs meshopt");
    }
}

fn sphere_from_two_points(a: Vec3, b: Vec3) -> (Vec3, f32) {
    let c = (a + b) * 0.5;
    let r = (a - c).length();
    (c, r)
}

fn sphere_from_three_points(a: Vec3, b: Vec3, c: Vec3) -> Option<(Vec3, f32)> {
    let ab = b - a;
    let ac = c - a;
    let n = ab.cross(ac);
    let denom = 2.0 * n.length_squared();
    if denom.abs() < 1e-12 {
        return None;
    }
    let center =
        a + (ac.length_squared() * ab.cross(n) - ab.length_squared() * ac.cross(n)) / denom;
    let r = (center - a).length();
    Some((center, r))
}

fn sphere_from_four_points(a: Vec3, b: Vec3, c: Vec3, d: Vec3) -> Option<(Vec3, f32)> {
    let m = Mat3::from_cols(2.0 * (a - b), 2.0 * (a - c), 2.0 * (a - d));
    let rhs = Vec3::new(
        a.dot(a) - b.dot(b),
        a.dot(a) - c.dot(c),
        a.dot(a) - d.dot(d),
    );
    if m.determinant().abs() < 1e-12 {
        return None;
    }
    let center = m.inverse() * rhs;
    let r = (center - a).length();
    Some((center, r))
}

fn combinatorial_min_enclosing_radius(pts: &[Vec3]) -> f32 {
    let n = pts.len();
    if n <= 1 {
        return 0.0;
    }
    let mut best = f32::INFINITY;
    let mut consider = |c: Vec3, r: f32| {
        if !(r.is_finite() && r >= 0.0) {
            return;
        }
        if pts.iter().all(|p| (*p - c).length() <= r + 2e-3) {
            best = best.min(r);
        }
    };
    for i in 0..n {
        for j in i + 1..n {
            let (c, r) = sphere_from_two_points(pts[i], pts[j]);
            consider(c, r);
        }
    }
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if let Some((c, r)) = sphere_from_three_points(pts[i], pts[j], pts[k]) {
                    consider(c, r);
                }
            }
        }
    }
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                for l in k + 1..n {
                    if let Some((c, r)) = sphere_from_four_points(pts[i], pts[j], pts[k], pts[l]) {
                        consider(c, r);
                    }
                }
            }
        }
    }
    assert!(best.is_finite(), "failed to find enclosing sphere");
    best
}

/// TC-2.4.9.2
#[test]
fn test_bounds_sphere_minimal() {
    let mesh = unit_cube_mesh();
    let asset = MeshletBuilder::new(mesh.clone()).build().expect("build");
    let (compact, meshlets) = meshopt_lod0_session(&mesh);
    for i in 0..meshlets.len() {
        let mh = meshlets.get(i);
        let mm = &asset.lod_groups[0].meshlets[i];
        let pts: Vec<Vec3> = mh
            .vertices
            .iter()
            .map(|&gv| Vec3::from(compact[gv as usize].p))
            .collect();
        let r_min = combinatorial_min_enclosing_radius(&pts);
        let br = mm.bounds_radius;
        assert!(
            br <= r_min * 1.05 + 2e-3,
            "bounds too loose: br={br} r_min={r_min}"
        );
        assert!(br + 2e-3 >= r_min, "bounds smaller than true minimum");
    }
}

/// TC-2.4.5.3 — requires GPU harness.
#[test]
#[ignore = "requires GPU readback harness"]
fn test_asset_upload_and_draw() {}

/// TC-2.4.6.1 — requires ray tracing device.
#[test]
#[ignore = "requires GpuDevice BLAS harness"]
fn test_blas_built_from_meshlet_asset() {}

/// TC-2.4.6.2
#[test]
#[ignore = "requires GpuDevice BLAS harness"]
fn test_blas_hits_match_raster() {}

/// TC-2.4.7.1
#[test]
#[ignore = "requires ECS + physics harness"]
fn test_collider_references_mesh_asset() {}

/// TC-2.4.7.2
#[test]
#[ignore = "requires physics harness"]
fn test_collision_bvh_from_lod0() {}

/// TC-2.4.7.3
#[test]
#[ignore = "requires hot reload harness"]
fn test_hot_reload_rebuilds_bvh_and_gpu() {}
