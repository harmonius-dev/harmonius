# Crash Reporting -- Test Cases

Companion to [crash-reporting.md](crash-reporting.md).

Test case IDs use `TC-14.4.Z.N` format.

## Unit Tests

| ID              | Name                                          | Req       |
|-----------------|-----------------------------------------------|-----------|
| TC-14.4.1.1     | `test_stub_install_spawns_monitor`            | R-14.4.1  |
| TC-14.4.1.2     | `test_stub_is_signal_safe_noalloc`            | R-14.4.1  |
| TC-14.4.1.3     | `test_breadcrumb_handle_write`                | R-14.4.1  |
| TC-14.4.1.4     | `test_update_scene_metadata`                  | R-14.4.1  |
| TC-14.4.1.5     | `test_update_player_metadata`                 | R-14.4.1  |
| TC-14.4.2.1     | `test_pdb_guid_age_index_key`                 | R-14.4.2  |
| TC-14.4.2.2     | `test_mac_lc_uuid_index_key`                  | R-14.4.2  |
| TC-14.4.2.3     | `test_linux_build_id_index_key`               | R-14.4.2  |
| TC-14.4.2.4     | `test_symbol_server_put_and_get`              | R-14.4.2  |
| TC-14.4.3.1     | `test_fingerprint_normalizes_line_numbers`    | R-14.4.3  |
| TC-14.4.3.2     | `test_fingerprint_strips_param_lists`         | R-14.4.3  |
| TC-14.4.3.3     | `test_fingerprint_strips_template_args`       | R-14.4.3  |
| TC-14.4.3.4     | `test_fingerprint_drops_handler_frames`       | R-14.4.3  |
| TC-14.4.3.5     | `test_cluster_upsert_increments_count`        | R-14.4.3  |
| TC-14.4.3.6     | `test_alerting_rule_new_cluster_high_volume`  | R-14.4.3  |
| TC-14.4.7.1     | `test_monitor_reads_notification`             | R-14.4.7  |
| TC-14.4.7.2     | `test_monitor_writes_minidump_file`           | R-14.4.7  |
| TC-14.4.7.3     | `test_monitor_attaches_metadata_stream`       | R-14.4.7  |
| TC-14.4.8.1     | `test_rotation_caps_count`                    | R-14.4.8  |
| TC-14.4.8.2     | `test_rotation_removes_oldest_first`          | R-14.4.8  |
| TC-14.4.8.3     | `test_sidecar_metadata_written`               | R-14.4.8  |

1. **TC-14.4.1.1** `test_stub_install_spawns_monitor` -- Calling `install()` starts a child process
   whose pid is stored in the handle.
2. **TC-14.4.1.2** `test_stub_is_signal_safe_noalloc` -- Code-gen lint asserts the fault path calls
   no heap allocator symbols.
3. **TC-14.4.1.3** `test_breadcrumb_handle_write` -- Writing a breadcrumb value is visible to the
   monitor through shared memory.
4. **TC-14.4.1.4** `test_update_scene_metadata` -- After `update_scene("level_1")`, the monitor's
   shared buffer contains "level_1".
5. **TC-14.4.1.5** `test_update_player_metadata` -- After `update_player`, the monitor reads the
   expected `UserId`.
6. **TC-14.4.2.1** `test_pdb_guid_age_index_key` -- GUID `{1234...}` and age 2 produce
   `game.pdb/12345678...2/game.pdb`.
7. **TC-14.4.2.2** `test_mac_lc_uuid_index_key` -- UUID `ABCD...` maps to
   `game.dSYM/Contents/Resources/DWARF/ABCD...`.
8. **TC-14.4.2.3** `test_linux_build_id_index_key` -- Build id hex maps to `game/<hex>`.
9. **TC-14.4.2.4** `test_symbol_server_put_and_get` -- PUT a symfile then GET; bytes equal.
10. **TC-14.4.3.1** `test_fingerprint_normalizes_line_numbers` -- Two dumps differing only in line
    number produce the same fingerprint.
11. **TC-14.4.3.2** `test_fingerprint_strips_param_lists` -- `foo(i32)` and `foo(u32)` yield the
    same fingerprint.
12. **TC-14.4.3.3** `test_fingerprint_strips_template_args` -- `Vec<i32>::push` and `Vec<u8>::push`
    yield the same fingerprint.
13. **TC-14.4.3.4** `test_fingerprint_drops_handler_frames` -- Frames in `harmonius::crash::*`
    removed before hashing.
14. **TC-14.4.3.5** `test_cluster_upsert_increments_count` -- Upserting same fingerprint twice
    leaves count 2.
15. **TC-14.4.3.6** `test_alerting_rule_new_cluster_high_volume` -- Injecting 11 reports in 60s
    triggers high alert.
16. **TC-14.4.7.1** `test_monitor_reads_notification` -- Write a fault record to the pipe; monitor
    loop returns it.
17. **TC-14.4.7.2** `test_monitor_writes_minidump_file` -- Monitor creates a valid `MDMP` file in
    the dump directory.
18. **TC-14.4.7.3** `test_monitor_attaches_metadata_stream` -- Resulting dump contains a
    `HarmoniusMetadataStream` with the expected fields.
19. **TC-14.4.8.1** `test_rotation_caps_count` -- With cap 10 and 15 dumps, 5 removed.
20. **TC-14.4.8.2** `test_rotation_removes_oldest_first` -- Oldest by `mtime` removed first.
21. **TC-14.4.8.3** `test_sidecar_metadata_written` -- Each dump has a JSON sidecar with the same
    stem and build id.

## Integration Tests

| ID              | Name                                               | Req       |
|-----------------|----------------------------------------------------|-----------|
| TC-14.4.1.I1    | `test_real_sigsegv_produces_dump_linux`            | R-14.4.1  |
| TC-14.4.1.I2    | `test_real_seh_produces_dump_windows`              | R-14.4.1  |
| TC-14.4.1.I3    | `test_real_mach_exception_produces_dump_macos`     | R-14.4.1  |
| TC-14.4.3.I1    | `test_end_to_end_upload_and_cluster`               | R-14.4.3  |
| TC-14.4.2.I1    | `test_symbol_upload_resolves_frame`                | R-14.4.2  |

1. **TC-14.4.1.I1** `test_real_sigsegv_produces_dump_linux` -- Spawn a child that dereferences a
   null pointer; monitor produces a valid minidump.
2. **TC-14.4.1.I2** `test_real_seh_produces_dump_windows` -- Child raises access violation; monitor
   produces a valid minidump.
3. **TC-14.4.1.I3** `test_real_mach_exception_produces_dump_macos` -- Child triggers bad access;
   monitor catches Mach message and writes dump.
4. **TC-14.4.3.I1** `test_end_to_end_upload_and_cluster` -- Dump uploads over HTTP/3; backend ingest
   groups it under the expected cluster id.
5. **TC-14.4.2.I1** `test_symbol_upload_resolves_frame` -- Upload symfiles for a test binary, then
   request resolve for a known address; server returns the expected function name.

## Benchmarks

| ID              | Name                                    | Target       |
|-----------------|-----------------------------------------|--------------|
| TC-14.4.1.B1    | `bench_breadcrumb_write_latency`        | < 50 ns      |
| TC-14.4.7.B1    | `bench_minidump_write_typical`          | < 500 ms     |
| TC-14.4.3.B1    | `bench_fingerprint_normalize_32_frames` | < 20 us      |
| TC-14.4.3.B2    | `bench_upload_dump_1mib`                | < 2000 ms    |

1. **TC-14.4.1.B1** -- Write a breadcrumb into the shared buffer. < 50 ns.
2. **TC-14.4.7.B1** -- Write a typical minidump (10 threads, 4 MiB memory regions). < 500 ms.
3. **TC-14.4.3.B1** -- Normalize and hash a 32-frame symbolicated stack. < 20 us.
4. **TC-14.4.3.B2** -- Upload a 1 MiB dump over HTTP/3 on loopback. < 2000 ms.
