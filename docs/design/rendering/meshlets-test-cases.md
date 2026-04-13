# Meshlets -- Test Cases

Companion to [meshlets.md](meshlets.md).

Test case IDs use `TC-2.4.Z.N` format.

## Unit Tests

| ID          | Name                                     | Req     |
|-------------|------------------------------------------|---------|
| TC-2.4.1.1  | `test_meshlet_asset_roundtrip_rkyv`      | R-2.4.1 |
| TC-2.4.1.2  | `test_source_hash_stable`                | R-2.4.1 |
| TC-2.4.2.1  | `test_meshlet_vertex_cap_64`             | R-2.4.2 |
| TC-2.4.2.2  | `test_meshlet_triangle_cap_124`          | R-2.4.2 |
| TC-2.4.2.3  | `test_meshlet_struct_size`               | R-2.4.2 |
| TC-2.4.3.1  | `test_lod_group_hierarchy_monotonic`     | R-2.4.3 |
| TC-2.4.3.2  | `test_lod_screen_error_increases`        | R-2.4.3 |
| TC-2.4.4.1  | `test_builder_determinism_cube`          | R-2.4.4 |
| TC-2.4.4.2  | `test_builder_determinism_sphere`        | R-2.4.4 |
| TC-2.4.4.3  | `test_builder_rejects_non_manifold`      | R-2.4.4 |
| TC-2.4.4.4  | `test_builder_simplify_failure`          | R-2.4.4 |
| TC-2.4.5.1  | `test_gpu_layout_alignment`              | R-2.4.5 |
| TC-2.4.5.2  | `test_buffer_view_stride_matches`        | R-2.4.5 |
| TC-2.4.8.1  | `test_cone_axis_unit_length`             | R-2.4.8 |
| TC-2.4.8.2  | `test_cone_angle_in_range`               | R-2.4.8 |
| TC-2.4.9.1  | `test_bounds_sphere_encloses_triangles`  | R-2.4.9 |
| TC-2.4.9.2  | `test_bounds_sphere_minimal`             | R-2.4.9 |

1. **TC-2.4.1.1** `test_meshlet_asset_roundtrip_rkyv` -- Serialize a `MeshletAsset` and deserialize
   via mmap. Assert field equality.
   - Input: built asset with 10 meshlets
   - Expected: zero-copy view equal to source
2. **TC-2.4.1.2** `test_source_hash_stable` -- Build the same fixture mesh twice. Assert
   `source_hash` is identical.
3. **TC-2.4.2.1** `test_meshlet_vertex_cap_64` -- Build a mesh with 10,000 vertices. Assert no
   meshlet has `vertex_count > 64`.
4. **TC-2.4.2.2** `test_meshlet_triangle_cap_124` -- Same; assert `triangle_count <= 124`.
5. **TC-2.4.2.3** `test_meshlet_struct_size` -- Assert `size_of::<Meshlet>() == 64`.
6. **TC-2.4.3.1** `test_lod_group_hierarchy_monotonic` -- LOD levels are 0..n, no gaps, no
   duplicates.
7. **TC-2.4.3.2** `test_lod_screen_error_increases` -- `screen_error[i+1] > screen_error[i]` for all
   i.
8. **TC-2.4.4.1** `test_builder_determinism_cube` -- Build a unit cube twice. Assert the two assets
   are byte-identical.
9. **TC-2.4.4.2** `test_builder_determinism_sphere` -- Same for a 1024-triangle sphere.
10. **TC-2.4.4.3** `test_builder_rejects_non_manifold` -- Input with a non-manifold edge.
    - Expected: `Err(BuildError::InvalidTopology)`
11. **TC-2.4.4.4** `test_builder_simplify_failure` -- Input where simplification can't hit the
    target ratio.
    - Expected: `Err(BuildError::SimplifyFailed { level, reason })`
12. **TC-2.4.5.1** `test_gpu_layout_alignment` -- Assert all buffer offsets are 16-byte aligned.
13. **TC-2.4.5.2** `test_buffer_view_stride_matches` -- Vertex stride equals 32 bytes.
14. **TC-2.4.8.1** `test_cone_axis_unit_length` -- `cone_axis.length() == 1.0 +/- 1e-5`.
15. **TC-2.4.8.2** `test_cone_angle_in_range` -- `0 <= cone_angle <= pi`.
16. **TC-2.4.9.1** `test_bounds_sphere_encloses_triangles` -- Every meshlet vertex is inside
    `bounds`.
17. **TC-2.4.9.2** `test_bounds_sphere_minimal` -- Bounds radius is within 5% of the minimum
    enclosing sphere.

## Integration Tests

| ID          | Name                                     | Req     |
|-------------|------------------------------------------|---------|
| TC-2.4.5.3  | `test_asset_upload_and_draw`             | R-2.4.5 |
| TC-2.4.6.1  | `test_blas_built_from_meshlet_asset`     | R-2.4.6 |
| TC-2.4.6.2  | `test_blas_hits_match_raster`            | R-2.4.6 |
| TC-2.4.7.1  | `test_collider_references_mesh_asset`    | R-2.4.7 |
| TC-2.4.7.2  | `test_collision_bvh_from_lod0`           | R-2.4.7 |
| TC-2.4.7.3  | `test_hot_reload_rebuilds_bvh_and_gpu`   | R-2.4.7 |

1. **TC-2.4.5.3** `test_asset_upload_and_draw` -- Load a mesh, upload to GPU, draw one frame, read
   back a pixel. Assert non-clear color.
2. **TC-2.4.6.1** `test_blas_built_from_meshlet_asset` -- Build BLAS from a meshlet asset. Assert no
   device errors.
3. **TC-2.4.6.2** `test_blas_hits_match_raster` -- Raycast against BLAS. Assert hit distance matches
   rasterized depth within 1e-4.
4. **TC-2.4.7.1** `test_collider_references_mesh_asset` -- `Collider::mesh` holds a valid handle and
   physics reads it.
5. **TC-2.4.7.2** `test_collision_bvh_from_lod0` -- Physics BVH uses LOD 0 geometry; triangle count
   equals `lod_groups[0]` triangle count.
6. **TC-2.4.7.3** `test_hot_reload_rebuilds_bvh_and_gpu` -- Hot-reload the mesh. Assert both GPU
   buffers and physics BVH see the new geometry.

## Benchmarks

| ID           | Name                                     | Target            |
|--------------|------------------------------------------|-------------------|
| TC-2.4.4.B1  | `bench_build_100k_tri_mesh`              | < 200 ms          |
| TC-2.4.4.B2  | `bench_build_1m_tri_mesh`                | < 2.0 s           |
| TC-2.4.5.B1  | `bench_upload_10m_tri_scene`             | < 50 ms           |
| TC-2.4.8.B1  | `bench_cone_cull_100k_meshlets`          | < 0.2 ms          |
| TC-2.4.9.B1  | `bench_frustum_cull_100k_meshlets`       | < 0.15 ms         |
| TC-2.4.6.B1  | `bench_blas_build_500k_tri`              | < 100 ms          |

1. **TC-2.4.4.B1** -- Build meshlet asset for a 100k-triangle mesh. Pass if < 200 ms.
2. **TC-2.4.4.B2** -- Same for 1M triangles. Pass if < 2 s.
3. **TC-2.4.5.B1** -- Upload 10M triangles total across 1000 meshes. Pass if < 50 ms.
4. **TC-2.4.8.B1** -- GPU cone cull 100,000 meshlets. Pass if < 0.2 ms.
5. **TC-2.4.9.B1** -- GPU frustum cull 100,000 meshlets. Pass if < 0.15 ms.
6. **TC-2.4.6.B1** -- Build BLAS for 500k triangles. Pass if < 100 ms.
