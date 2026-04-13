# Localization -- Test Cases

Companion to [localization.md](localization.md).

Test case IDs use `TC-10.1.9.N` and `TC-15.13.Z.N` format.

## Unit Tests

| ID                | Name                                             | Req         |
|-------------------|--------------------------------------------------|-------------|
| TC-10.1.9.1       | `test_lookup_returns_active_locale_string`       | R-10.1.9    |
| TC-10.1.9.2       | `test_lookup_missing_key_returns_source`         | R-10.1.9    |
| TC-10.1.9.3       | `test_lookup_missing_in_source_returns_placeholder`| R-10.1.9  |
| TC-10.1.9.4       | `test_active_locale_reported`                    | R-10.1.9    |
| TC-10.1.9.5       | `test_binary_search_on_sorted_keys`              | R-10.1.9    |
| TC-10.1.9.6       | `test_plural_selection_one`                      | R-10.1.9    |
| TC-10.1.9.7       | `test_plural_selection_many`                     | R-10.1.9    |
| TC-10.1.9.8       | `test_missing_log_dedupes_per_pair`              | R-10.1.9    |
| TC-10.1.9.9       | `test_missing_log_drain_returns_entries`         | R-10.1.9    |
| TC-15.13.1.1      | `test_table_archive_roundtrip`                   | R-15.13.1   |
| TC-15.13.1.2      | `test_table_keys_sorted_ascending`               | R-15.13.1   |
| TC-15.13.1.3      | `test_table_key_entries_unique`                  | R-15.13.1   |
| TC-15.13.1.4      | `test_bake_deterministic_across_platforms`       | R-15.13.1   |
| TC-15.13.2.1      | `test_locale_switch_event_changes_active`        | R-15.13.2   |
| TC-15.13.2.2      | `test_locale_switch_emits_locale_changed`        | R-15.13.2   |
| TC-15.13.2.3      | `test_locale_switch_unknown_locale_errors`       | R-15.13.2   |
| TC-15.13.3.1      | `test_fallback_to_source_logs_warning_once`      | R-15.13.3   |
| TC-15.13.3.2      | `test_fallback_returns_correct_source_text`      | R-15.13.3   |
| TC-15.13.3.3      | `test_placeholder_when_neither_has_key`          | R-15.13.3   |

1. **TC-10.1.9.1** `test_lookup_returns_active_locale_string` -- Load en and ja tables; set active =
   ja; lookup returns ja string for known id.
2. **TC-10.1.9.2** `test_lookup_missing_key_returns_source` -- ja table missing id; fallback returns
   en (source) string.
3. **TC-10.1.9.3** `test_lookup_missing_in_source_returns_placeholder` -- Id not in any table;
   returns `[missing:<id>]`.
4. **TC-10.1.9.4** `test_active_locale_reported` -- After `set_active(ja)`, `active_locale()` equals
   `ja`.
5. **TC-10.1.9.5** `test_binary_search_on_sorted_keys` -- Lookup uses binary search; instrumentation
   count equals O(log n).
6. **TC-10.1.9.6** `test_plural_selection_one` -- n=1 selects the `One` category text.
7. **TC-10.1.9.7** `test_plural_selection_many` -- n=5 selects the `Other` category text in en.
8. **TC-10.1.9.8** `test_missing_log_dedupes_per_pair` -- Same missing id logged twice produces one
   warning.
9. **TC-10.1.9.9** `test_missing_log_drain_returns_entries` -- Drain returns each unique entry once
   and clears the log.
10. **TC-15.13.1.1** `test_table_archive_roundtrip` -- Encode and decode a table; fields equal.
11. **TC-15.13.1.2** `test_table_keys_sorted_ascending` -- All `KeyEntry::id` ascending.
12. **TC-15.13.1.3** `test_table_key_entries_unique` -- No duplicate ids within a table.
13. **TC-15.13.1.4** `test_bake_deterministic_across_platforms` -- Running `loc-bake` on fixture
    produces identical bytes on Linux and macOS.
14. **TC-15.13.2.1** `test_locale_switch_event_changes_active` -- Sending `LocaleSwitch(target=ja)`
    updates the service's active locale.
15. **TC-15.13.2.2** `test_locale_switch_emits_locale_changed` -- Service emits `LocaleChanged`
    downstream.
16. **TC-15.13.2.3** `test_locale_switch_unknown_locale_errors` -- Switching to an unloaded locale
    returns `LocError::NotLoaded`.
17. **TC-15.13.3.1** `test_fallback_to_source_logs_warning_once` -- First fallback logs; second is
    silent.
18. **TC-15.13.3.2** `test_fallback_returns_correct_source_text` -- Result equals the source
    locale's string.
19. **TC-15.13.3.3** `test_placeholder_when_neither_has_key` -- Returns exact placeholder string.

## Integration Tests

| ID                | Name                                              | Req         |
|-------------------|---------------------------------------------------|-------------|
| TC-10.1.9.I1      | `test_ui_rerenders_on_locale_change`              | R-10.1.9    |
| TC-15.13.1.I1     | `test_loc_bake_end_to_end`                        | R-15.13.1   |
| TC-15.13.1.I2     | `test_editor_hot_reload_updates_strings`          | R-15.13.1   |
| TC-15.13.3.I1     | `test_shipping_runtime_reports_missing_log`       | R-15.13.3   |

1. **TC-10.1.9.I1** `test_ui_rerenders_on_locale_change` -- Text widget displays ja after
   `LocaleSwitch(ja)` without restart.
2. **TC-15.13.1.I1** `test_loc_bake_end_to_end` -- Feed a `StringTableDocument` through the content
   pipeline; resulting `.loctable` loads cleanly into the service.
3. **TC-15.13.1.I2** `test_editor_hot_reload_updates_strings` -- Edit a source string in the editor;
   after save, the service reloads the changed table and the UI updates.
4. **TC-15.13.3.I1** `test_shipping_runtime_reports_missing_log` -- Drain the missing log after a
   session; entries match fixture.

## Benchmarks

| ID              | Name                             | Target       |
|-----------------|----------------------------------|--------------|
| TC-10.1.9.B1    | `bench_lookup_hot_path`          | < 50 ns      |
| TC-10.1.9.B2    | `bench_locale_switch`            | < 10 us      |
| TC-15.13.1.B1   | `bench_table_load_10k_keys`      | < 2 ms       |
| TC-15.13.1.B2   | `bench_bake_10k_keys`            | < 50 ms      |

1. **TC-10.1.9.B1** -- Single lookup into a 10k-key table. < 50 ns.
2. **TC-10.1.9.B2** -- Switch active locale. < 10 us.
3. **TC-15.13.1.B1** -- Load a baked 10k-key table into memory. < 2 ms.
4. **TC-15.13.1.B2** -- Bake a 10k-key table from source. < 50 ms.
