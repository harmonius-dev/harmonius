# Telemetry -- Test Cases

Companion to [telemetry.md](telemetry.md).

Test case IDs use `TC-14.5.Z.N` and `TC-14.4.5.N` format.

## Unit Tests

| ID              | Name                                             | Req       |
|-----------------|--------------------------------------------------|-----------|
| TC-14.4.5.1     | `test_counter_registration_unique_names`         | R-14.4.5  |
| TC-14.4.5.2     | `test_counter_read_returns_current_value`        | R-14.4.5  |
| TC-14.5.1.1     | `test_first_run_defaults_both_scopes_off`        | R-14.5.1  |
| TC-14.5.1.2     | `test_first_run_prompt_once_per_install`         | R-14.5.1  |
| TC-14.5.1.3     | `test_consent_persists_across_restart`           | R-14.5.1  |
| TC-14.5.2.1     | `test_engine_scope_drops_game_events`            | R-14.5.2  |
| TC-14.5.2.2     | `test_game_scope_drops_engine_events`            | R-14.5.2  |
| TC-14.5.2.3     | `test_both_scopes_records_all`                   | R-14.5.2  |
| TC-14.5.2.4     | `test_neither_scope_records_none`                | R-14.5.2  |
| TC-14.5.3.1     | `test_ringbuffer_rejects_push_when_full`         | R-14.5.3  |
| TC-14.5.3.2     | `test_ringbuffer_spills_to_disk_at_75pct`        | R-14.5.3  |
| TC-14.5.3.3     | `test_disk_spill_roundtrip_preserves_records`    | R-14.5.3  |
| TC-14.5.3.4     | `test_offline_buffer_survives_restart`           | R-14.5.3  |
| TC-14.5.4.1     | `test_batch_drains_buffer_atomically`            | R-14.5.4  |
| TC-14.5.4.2     | `test_failed_upload_retains_batch`               | R-14.5.4  |
| TC-14.5.4.3     | `test_retry_uses_exponential_backoff`            | R-14.5.4  |
| TC-14.5.5.1     | `test_export_bundle_contains_records`            | R-14.5.5  |
| TC-14.5.5.2     | `test_delete_clears_local_buffer`                | R-14.5.5  |
| TC-14.5.5.3     | `test_delete_rotates_anonymous_id`               | R-14.5.5  |
| TC-14.5.6.1     | `test_schema_catalog_lookup_by_id`               | R-14.5.6  |
| TC-14.5.6.2     | `test_codegen_rejects_sensitive_field`           | R-14.5.6  |
| TC-14.5.6.3     | `test_schema_required_fields_enforced`           | R-14.5.6  |
| TC-14.5.6.4     | `test_pii_class_tag_preserved_in_archive`        | R-14.5.6  |

1. **TC-14.4.5.1** `test_counter_registration_unique_names` -- Duplicate registration returns error.
2. **TC-14.4.5.2** `test_counter_read_returns_current_value` -- Write 42 to counter, read returns
   42.
3. **TC-14.5.1.1** `test_first_run_defaults_both_scopes_off` -- Fresh `ConsentState` has both scopes
   false.
4. **TC-14.5.1.2** `test_first_run_prompt_once_per_install` -- `is_first_run()` returns true once,
   then false after `set_consent` is called.
5. **TC-14.5.1.3** `test_consent_persists_across_restart` -- Save consent, create new client, state
   preserved.
6. **TC-14.5.2.1** `test_engine_scope_drops_game_events` -- With only engine consent, a game-logic
   event is dropped.
7. **TC-14.5.2.2** `test_game_scope_drops_engine_events` -- Inverse: with only game consent, an
   engine event is dropped.
8. **TC-14.5.2.3** `test_both_scopes_records_all` -- Both on, all events buffered.
9. **TC-14.5.2.4** `test_neither_scope_records_none` -- Both off, nothing buffered.
10. **TC-14.5.3.1** `test_ringbuffer_rejects_push_when_full` -- Full buffer returns `Err`.
11. **TC-14.5.3.2** `test_ringbuffer_spills_to_disk_at_75pct` -- Reaching 75% triggers a spill
    operation.
12. **TC-14.5.3.3** `test_disk_spill_roundtrip_preserves_records` -- Spill and reload equals
    original.
13. **TC-14.5.3.4** `test_offline_buffer_survives_restart` -- Spilled files found on next run and
    re-queued for upload.
14. **TC-14.5.4.1** `test_batch_drains_buffer_atomically` -- Batch drain pulls exactly N records or
    fails leaving buffer unchanged.
15. **TC-14.5.4.2** `test_failed_upload_retains_batch` -- Simulated 5xx leaves batch on disk for
    retry.
16. **TC-14.5.4.3** `test_retry_uses_exponential_backoff` -- Second retry interval is 2x first,
    bounded by max.
17. **TC-14.5.5.1** `test_export_bundle_contains_records` -- Export returns every in-memory and
    on-disk record for the current `AnonId`.
18. **TC-14.5.5.2** `test_delete_clears_local_buffer` -- After `delete()`, buffer empty and disk dir
    empty.
19. **TC-14.5.5.3** `test_delete_rotates_anonymous_id` -- New `AnonId` differs from previous.
20. **TC-14.5.6.1** `test_schema_catalog_lookup_by_id` -- Lookup returns the expected schema.
21. **TC-14.5.6.2** `test_codegen_rejects_sensitive_field` -- Running the codegen on a schema with a
    `Sensitive` field fails the build.
22. **TC-14.5.6.3** `test_schema_required_fields_enforced` -- Missing required field rejects
    archive.
23. **TC-14.5.6.4** `test_pii_class_tag_preserved_in_archive` -- Round trip retains pii class
    metadata.

## Integration Tests

| ID              | Name                                              | Req       |
|-----------------|---------------------------------------------------|-----------|
| TC-14.5.3.I1    | `test_offline_72h_scenario`                       | R-14.5.3  |
| TC-14.5.4.I1    | `test_end_to_end_upload_ack`                      | R-14.5.4  |
| TC-14.5.5.I1    | `test_export_endpoint_round_trip`                 | R-14.5.5  |
| TC-14.5.5.I2    | `test_delete_endpoint_round_trip`                 | R-14.5.5  |
| TC-14.5.1.I1    | `test_first_run_dialog_updates_consent_state`     | R-14.5.1  |

1. **TC-14.5.3.I1** `test_offline_72h_scenario` -- Simulate 72 hours offline with continuous
   emission; on reconnect all batches upload in order.
2. **TC-14.5.4.I1** `test_end_to_end_upload_ack` -- Real HTTP/3 server acks a batch; client marks
   records consumed.
3. **TC-14.5.5.I1** `test_export_endpoint_round_trip` -- Export returns same records the server
   stored for the anon id.
4. **TC-14.5.5.I2** `test_delete_endpoint_round_trip` -- Delete, then export: server returns empty
   set.
5. **TC-14.5.1.I1** `test_first_run_dialog_updates_consent_state` -- Consent dialog integration
   triggers persistence.

## Benchmarks

| ID              | Name                                 | Target       |
|-----------------|--------------------------------------|--------------|
| TC-14.5.2.B1    | `bench_record_event_hot_path`        | < 500 ns     |
| TC-14.5.4.B1    | `bench_serialize_batch_1000`         | < 2 ms       |
| TC-14.5.4.B2    | `bench_upload_batch_1mib`            | < 1500 ms    |
| TC-14.5.3.B1    | `bench_disk_spill_16mib`             | < 50 ms      |

1. **TC-14.5.2.B1** -- Record an event with consent on. < 500 ns.
2. **TC-14.5.4.B1** -- Serialize 1000 records into a single batch. < 2 ms.
3. **TC-14.5.4.B2** -- Upload a 1 MiB batch over loopback HTTP/3. < 1500 ms.
4. **TC-14.5.3.B1** -- Spill a full 16 MiB in-memory buffer to disk. < 50 ms.
