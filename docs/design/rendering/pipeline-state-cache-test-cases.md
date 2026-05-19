# Pipeline State Object Cache -- Test Cases

Companion to [pipeline-state-cache.md](pipeline-state-cache.md).

Test case IDs use `TC-2.3.9.Z.N` format.

## Unit Tests

| ID            | Name                                         | Req       |
|---------------|----------------------------------------------|-----------|
| TC-2.3.9.2.1  | `test_pso_key_deterministic_hash`            | R-2.3.9.2 |
| TC-2.3.9.2.2  | `test_pso_key_shader_hash_affects_key`       | R-2.3.9.2 |
| TC-2.3.9.2.3  | `test_pso_key_signature_hash_affects_key`    | R-2.3.9.2 |
| TC-2.3.9.2.4  | `test_pso_key_driver_version_affects_key`    | R-2.3.9.2 |
| TC-2.3.9.1.1  | `test_memory_cache_insert_and_get`           | R-2.3.9.1 |
| TC-2.3.9.1.2  | `test_memory_cache_last_used_tick_bumps`     | R-2.3.9.1 |
| TC-2.3.9.1.3  | `test_memory_cache_lookup_is_sorted_vec`     | R-2.3.9.1 |
| TC-2.3.9.3.1  | `test_disk_layout_versioned_directory`       | R-2.3.9.3 |
| TC-2.3.9.3.2  | `test_disk_layout_fingerprint_subdirectory`  | R-2.3.9.3 |
| TC-2.3.9.4.1  | `test_invalidate_on_shader_hash_change`      | R-2.3.9.4 |
| TC-2.3.9.4.2  | `test_invalidate_on_driver_upgrade`          | R-2.3.9.4 |
| TC-2.3.9.4.3  | `test_invalidate_on_device_change`           | R-2.3.9.4 |
| TC-2.3.9.4.4  | `test_invalidate_all_clears_memory`          | R-2.3.9.4 |
| TC-2.3.9.5.1  | `test_gc_drops_oldest_by_tick`               | R-2.3.9.5 |
| TC-2.3.9.5.2  | `test_gc_compacts_blob_file`                 | R-2.3.9.5 |
| TC-2.3.9.5.3  | `test_gc_respects_disk_cap`                  | R-2.3.9.5 |
| TC-2.3.9.6.1  | `test_corrupt_blob_isolated`                 | R-2.3.9.6 |
| TC-2.3.9.6.2  | `test_corrupt_index_resets_directory`        | R-2.3.9.6 |
| TC-2.3.9.6.3  | `test_bad_magic_resets_directory`            | R-2.3.9.6 |
| TC-2.3.9.8.1  | `test_infer_descriptor_layout_dxil`          | R-2.3.9.8 |
| TC-2.3.9.8.2  | `test_infer_descriptor_layout_spirv`         | R-2.3.9.8 |
| TC-2.3.9.8.3  | `test_infer_descriptor_layout_vulkan`         | R-2.3.9.8 |

1. **TC-2.3.9.2.1** `test_pso_key_deterministic_hash` -- Construct a key twice from the same inputs.
   Assert equal.
2. **TC-2.3.9.2.2** `test_pso_key_shader_hash_affects_key` -- Change only shader hash. Assert keys
   differ.
3. **TC-2.3.9.2.3** `test_pso_key_signature_hash_affects_key` -- Change only signature.
4. **TC-2.3.9.2.4** `test_pso_key_driver_version_affects_key` -- Change only driver version.
5. **TC-2.3.9.1.1** `test_memory_cache_insert_and_get` -- Insert entry, retrieve by key.
6. **TC-2.3.9.1.2** `test_memory_cache_last_used_tick_bumps` -- Successful lookup bumps tick.
7. **TC-2.3.9.1.3** `test_memory_cache_lookup_is_sorted_vec` -- Cache uses `SortedVecMap`, not
   `HashMap`. Assert type equality via type assertion macro.
8. **TC-2.3.9.3.1** `test_disk_layout_versioned_directory` -- Format version directory exists.
9. **TC-2.3.9.3.2** `test_disk_layout_fingerprint_subdirectory` -- Fingerprint subdirectory exists
   and name matches fingerprint format.
10. **TC-2.3.9.4.1** `test_invalidate_on_shader_hash_change` -- Shader reload produces new key. Old
    entry can be invalidated.
11. **TC-2.3.9.4.2** `test_invalidate_on_driver_upgrade` -- Simulate driver upgrade by changing
    `DeviceFingerprint::driver_version`. Assert new cache opens in new directory.
12. **TC-2.3.9.4.3** `test_invalidate_on_device_change` -- Swap fingerprint. Old directory left
    intact but unused.
13. **TC-2.3.9.4.4** `test_invalidate_all_clears_memory` -- After call, `memory.len() == 0`.
14. **TC-2.3.9.5.1** `test_gc_drops_oldest_by_tick` -- GC removes entries in `last_used_tick` order.
15. **TC-2.3.9.5.2** `test_gc_compacts_blob_file` -- After GC, `blobs.bin` size == sum of remaining
    entries.
16. **TC-2.3.9.5.3** `test_gc_respects_disk_cap` -- After GC, total size <= cap.
17. **TC-2.3.9.6.1** `test_corrupt_blob_isolated` -- Corrupt one blob's CRC. Cache skips it. Other
    entries still retrievable.
18. **TC-2.3.9.6.2** `test_corrupt_index_resets_directory` -- Corrupt index checksum. Cache wipes
    and rebuilds.
19. **TC-2.3.9.6.3** `test_bad_magic_resets_directory` -- Wrong magic header. Directory reset.
20. **TC-2.3.9.8.1** `test_infer_descriptor_layout_dxil` -- Reflection produces correct binding
    count for a known SPIR-V fixture.
21. **TC-2.3.9.8.2** `test_infer_descriptor_layout_spirv` -- Same for SPIR-V.
22. **TC-2.3.9.8.3** `test_infer_descriptor_layout_vulkan` -- Same for Vulkan library.

## Integration Tests

| ID            | Name                                       | Req       |
|---------------|--------------------------------------------|-----------|
| TC-2.3.9.1.4  | `test_cold_start_miss_then_hit_disk`       | R-2.3.9.1 |
| TC-2.3.9.1.5  | `test_warm_start_hit_memory`               | R-2.3.9.1 |
| TC-2.3.9.7.1  | `test_hot_reload_invalidate_and_rebuild`   | R-2.3.9.7 |
| TC-2.3.9.7.2  | `test_hot_reload_mid_frame_safe`           | R-2.3.9.7 |
| TC-2.3.9.6.4  | `test_shutdown_flush_clean`                | R-2.3.9.6 |
| TC-2.3.9.3.3  | `test_editor_and_game_cache_isolated`      | R-2.3.9.3 |

1. **TC-2.3.9.1.4** `test_cold_start_miss_then_hit_disk` -- First frame misses everything, compiles.
   Second frame (new process) hits disk.
2. **TC-2.3.9.1.5** `test_warm_start_hit_memory` -- Within one process, second lookup hits memory.
3. **TC-2.3.9.7.1** `test_hot_reload_invalidate_and_rebuild` -- Reload a shader, draw, assert new
   PSO used.
4. **TC-2.3.9.7.2** `test_hot_reload_mid_frame_safe` -- Invalidation during frame defers actual drop
   until end-of-frame barrier. No use-after-free.
5. **TC-2.3.9.6.4** `test_shutdown_flush_clean` -- Tmp directories cleaned up after shutdown.
6. **TC-2.3.9.3.3** `test_editor_and_game_cache_isolated` -- Editor and game processes with
   different roots don't see each other's entries.

## Benchmarks

| ID             | Name                                     | Target             |
|----------------|------------------------------------------|--------------------|
| TC-2.3.9.1.B1  | `bench_memory_lookup`                    | < 200 ns           |
| TC-2.3.9.1.B2  | `bench_disk_lookup_hit`                  | < 1 ms             |
| TC-2.3.9.1.B3  | `bench_cold_compile`                     | < 500 ms           |
| TC-2.3.9.5.B1  | `bench_gc_100k_entries`                  | < 200 ms           |
| TC-2.3.9.6.B1  | `bench_checksum_validate_full_cache`     | < 50 ms            |

1. **TC-2.3.9.1.B1** -- Memory lookup with 10k entries. Pass if < 200 ns.
2. **TC-2.3.9.1.B2** -- Disk hit with revival. Pass if < 1 ms.
3. **TC-2.3.9.1.B3** -- Cold compile of a non-trivial graphics PSO. Pass if < 500 ms.
4. **TC-2.3.9.5.B1** -- GC run with 100k entries. Pass if < 200 ms.
5. **TC-2.3.9.6.B1** -- Validate checksums for a full 512 MiB cache. Pass if < 50 ms.
