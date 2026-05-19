# Shader Variants -- Test Cases

Companion to [shader-variants.md](shader-variants.md).

Test case IDs use `TC-2.3.10.Z.N` format.

## Unit Tests

| ID             | Name                                     | Req        |
|----------------|------------------------------------------|------------|
| TC-2.3.10.1.1  | `test_permutation_key_hash_deterministic`| R-2.3.10.1 |
| TC-2.3.10.1.2  | `test_permutation_key_distinct_features` | R-2.3.10.1 |
| TC-2.3.10.1.3  | `test_permutation_key_ordering`          | R-2.3.10.1 |
| TC-2.3.10.2.1  | `test_dimension_shading_model_count`     | R-2.3.10.2 |
| TC-2.3.10.2.2  | `test_dimension_features_16_bits_used`   | R-2.3.10.2 |
| TC-2.3.10.2.3  | `test_dimension_render_path_5_variants`  | R-2.3.10.2 |
| TC-2.3.10.2.4  | `test_dimension_lod_3_levels`            | R-2.3.10.2 |
| TC-2.3.10.3.1  | `test_budget_per_material_64`            | R-2.3.10.3 |
| TC-2.3.10.3.2  | `test_budget_per_shading_model_256`      | R-2.3.10.3 |
| TC-2.3.10.3.3  | `test_budget_per_project_4096`           | R-2.3.10.3 |
| TC-2.3.10.3.4  | `test_over_budget_fails_build`           | R-2.3.10.3 |
| TC-2.3.10.4.1  | `test_precompile_list_subset`            | R-2.3.10.4 |
| TC-2.3.10.4.2  | `test_precompile_respects_caps`          | R-2.3.10.4 |
| TC-2.3.10.5.1  | `test_on_demand_compile_missing_key`     | R-2.3.10.5 |
| TC-2.3.10.5.2  | `test_on_demand_updates_miss_log`        | R-2.3.10.5 |
| TC-2.3.10.6.1  | `test_bundle_roundtrip_mmap`             | R-2.3.10.6 |
| TC-2.3.10.6.2  | `test_bundle_format_version_check`       | R-2.3.10.6 |
| TC-2.3.10.7.1  | `test_coverage_report_counts_variants`   | R-2.3.10.7 |
| TC-2.3.10.7.2  | `test_coverage_report_reports_miss`      | R-2.3.10.7 |
| TC-2.3.10.8.1  | `test_metrics_miss_log_bounded`          | R-2.3.10.8 |
| TC-2.3.10.8.2  | `test_metrics_feeds_next_precompile`     | R-2.3.10.8 |
| TC-2.3.10.1.4  | `test_features_contains`                 | R-2.3.10.1 |
| TC-2.3.10.1.5  | `test_features_union`                    | R-2.3.10.1 |

1. **TC-2.3.10.1.1** `test_permutation_key_hash_deterministic` -- Same key produces same hash across
   two hasher instances.
2. **TC-2.3.10.1.2** `test_permutation_key_distinct_features` -- Toggling `NORMAL_MAP` changes the
   hash.
3. **TC-2.3.10.1.3** `test_permutation_key_ordering` -- `Ord` implementation is stable and
   consistent with serialized byte order.
4. **TC-2.3.10.2.1** `test_dimension_shading_model_count` -- `ShadingModel::VARIANTS.len() == 8`.
5. **TC-2.3.10.2.2** `test_dimension_features_16_bits_used` -- Bits 0..16 are named, 16..32
   reserved.
6. **TC-2.3.10.2.3** `test_dimension_render_path_5_variants` -- `RenderPath::VARIANTS.len() == 5`.
7. **TC-2.3.10.2.4** `test_dimension_lod_3_levels` -- `LodLevel::VARIANTS.len() == 3`.
8. **TC-2.3.10.3.1** `test_budget_per_material_64` -- Material graph with 65 variants errors.
9. **TC-2.3.10.3.2** `test_budget_per_shading_model_256` -- 257 variants under one shading model
   errors.
10. **TC-2.3.10.3.3** `test_budget_per_project_4096` -- 4097 total variants errors.
11. **TC-2.3.10.3.4** `test_over_budget_fails_build` -- Build-time check returns the offending list.
12. **TC-2.3.10.4.1** `test_precompile_list_subset` -- Precompile list is a subset of declared
    variants.
13. **TC-2.3.10.4.2** `test_precompile_respects_caps` -- No bundle exceeds caps after precompile.
14. **TC-2.3.10.5.1** `test_on_demand_compile_missing_key` -- Resolver compiles a missing key via
    glslc and returns bytecode.
15. **TC-2.3.10.5.2** `test_on_demand_updates_miss_log` -- Miss log contains the key after.
16. **TC-2.3.10.6.1** `test_bundle_roundtrip_mmap` -- Build, mmap, read, assert equal.
17. **TC-2.3.10.6.2** `test_bundle_format_version_check` -- Bundle with wrong format version returns
    `VariantError::FormatMismatch`.
18. **TC-2.3.10.7.1** `test_coverage_report_counts_variants` -- Report sums match.
19. **TC-2.3.10.7.2** `test_coverage_report_reports_miss` -- Report includes miss records.
20. **TC-2.3.10.8.1** `test_metrics_miss_log_bounded` -- Miss log is a ring buffer cap 1024.
21. **TC-2.3.10.8.2** `test_metrics_feeds_next_precompile` -- Running the build again produces a
    precompile list that includes previous misses.
22. **TC-2.3.10.1.4** `test_features_contains` -- Subset check works.
23. **TC-2.3.10.1.5** `test_features_union` -- Union combines bits.

## Integration Tests

| ID             | Name                                         | Req        |
|----------------|----------------------------------------------|------------|
| TC-2.3.10.4.3  | `test_full_build_precompile_pak`             | R-2.3.10.4 |
| TC-2.3.10.5.3  | `test_runtime_resolve_precompiled_hit`       | R-2.3.10.5 |
| TC-2.3.10.5.4  | `test_runtime_resolve_on_demand_fallback`    | R-2.3.10.5 |
| TC-2.3.10.6.3  | `test_pak_cross_backend_build`               | R-2.3.10.6 |

1. **TC-2.3.10.4.3** `test_full_build_precompile_pak` -- Build a project with 5 materials, assert
   pak contains all resolved variants.
2. **TC-2.3.10.5.3** `test_runtime_resolve_precompiled_hit` -- Precompiled variant resolves without
   invoking glslc.
3. **TC-2.3.10.5.4** `test_runtime_resolve_on_demand_fallback` -- Resolve a key not in pak; assert
   on-demand compile succeeds, PSO cache hit next time.
4. **TC-2.3.10.6.3** `test_pak_cross_backend_build` -- Build Vulkan paks from the
   same sources.

## Benchmarks

| ID              | Name                                     | Target            |
|-----------------|------------------------------------------|-------------------|
| TC-2.3.10.4.B1  | `bench_precompile_4096_variants`         | < 5 min           |
| TC-2.3.10.5.B1  | `bench_resolve_bundle_hit`               | < 500 ns          |
| TC-2.3.10.5.B2  | `bench_on_demand_compile_single`         | < 200 ms          |
| TC-2.3.10.6.B1  | `bench_pak_mmap_open`                    | < 10 ms           |

1. **TC-2.3.10.4.B1** -- Precompile 4096 variants. Pass if < 5 minutes on dev hardware.
2. **TC-2.3.10.5.B1** -- Resolve a precompiled variant. Pass if < 500 ns.
3. **TC-2.3.10.5.B2** -- On-demand compile via glslc subprocess. Pass if < 200 ms.
4. **TC-2.3.10.6.B1** -- Open and mmap `shaders.pak`. Pass if < 10 ms.
