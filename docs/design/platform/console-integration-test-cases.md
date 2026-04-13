# Console Integration -- Test Cases

Companion to [console-integration.md](console-integration.md).

Test case IDs use `TC-14.6.Z.N` format.

## Unit Tests

| ID              | Name                                          | Req       |
|-----------------|-----------------------------------------------|-----------|
| TC-14.6.1.1     | `test_stub_init_returns_ok`                   | R-14.6.1  |
| TC-14.6.1.2     | `test_stub_account_current_user_none`         | R-14.6.1  |
| TC-14.6.1.3     | `test_stub_account_sign_in_returns_fake`      | R-14.6.1  |
| TC-14.6.1.4     | `test_stub_storage_list_empty`                | R-14.6.1  |
| TC-14.6.1.5     | `test_stub_storage_write_and_read_roundtrip`  | R-14.6.1  |
| TC-14.6.1.6     | `test_stub_trophies_unlock_ok`                | R-14.6.1  |
| TC-14.6.1.7     | `test_stub_online_session_create_ok`          | R-14.6.1  |
| TC-14.6.1.8     | `test_stub_cert_all_methods_no_panic`         | R-14.6.1  |
| TC-14.6.2.1     | `test_build_script_sets_console_cfg_ps5`      | R-14.6.2  |
| TC-14.6.2.2     | `test_build_script_sets_console_cfg_xbox`     | R-14.6.2  |
| TC-14.6.2.3     | `test_build_script_sets_console_cfg_switch`   | R-14.6.2  |
| TC-14.6.2.4     | `test_desktop_target_has_no_console_cfg`      | R-14.6.2  |
| TC-14.6.3.1     | `test_nda_symbol_scan_rejects_ps5_header`     | R-14.6.3  |
| TC-14.6.3.2     | `test_nda_symbol_scan_rejects_gdk_header`     | R-14.6.3  |
| TC-14.6.3.3     | `test_public_crates_have_no_cdylib`           | R-14.6.3  |
| TC-14.6.4.1     | `test_cert_handle_suspend_idempotent`         | R-14.6.4  |
| TC-14.6.4.2     | `test_cert_handle_resume_idempotent`          | R-14.6.4  |
| TC-14.6.4.3     | `test_cert_handle_controller_disconnect`      | R-14.6.4  |
| TC-14.6.4.4     | `test_cert_rating_lock_toggles`               | R-14.6.4  |

1. **TC-14.6.1.1** `test_stub_init_returns_ok` -- `StubConsoleSdk::init(default())` returns Ok.
2. **TC-14.6.1.2** `test_stub_account_current_user_none` -- Stub `current_user()` returns `None`.
3. **TC-14.6.1.3** `test_stub_account_sign_in_returns_fake` -- Stub `sign_in(false)` returns a
   deterministic fake user with known id.
4. **TC-14.6.1.4** `test_stub_storage_list_empty` -- No slots initially.
5. **TC-14.6.1.5** `test_stub_storage_write_and_read_roundtrip` -- Write slot 0, read back equals.
6. **TC-14.6.1.6** `test_stub_trophies_unlock_ok` -- Unlock returns Ok and marks in-memory state.
7. **TC-14.6.1.7** `test_stub_online_session_create_ok` -- Create returns a deterministic
   `SessionId`.
8. **TC-14.6.1.8** `test_stub_cert_all_methods_no_panic` -- Every `CertHandle` method runs without
   panic.
9. **TC-14.6.2.1** `test_build_script_sets_console_cfg_ps5` -- TARGET containing `ps5` sets cfg
   `target_platform_console="ps5"`.
10. **TC-14.6.2.2** `test_build_script_sets_console_cfg_xbox` -- Same for `xbox-gdk`.
11. **TC-14.6.2.3** `test_build_script_sets_console_cfg_switch` -- Same for `nintendo`.
12. **TC-14.6.2.4** `test_desktop_target_has_no_console_cfg` -- Linux / macOS / Windows targets
    produce no console cfg.
13. **TC-14.6.3.1** `test_nda_symbol_scan_rejects_ps5_header` -- Pre-commit scan flags a file
    containing a known PS5 header name.
14. **TC-14.6.3.2** `test_nda_symbol_scan_rejects_gdk_header` -- Scan flags a file with a known GDK
    header name.
15. **TC-14.6.3.3** `test_public_crates_have_no_cdylib` -- Static check over workspace refuses
    `crate-type = ["cdylib"]` in public console crates.
16. **TC-14.6.4.1** `test_cert_handle_suspend_idempotent` -- Calling suspend twice is safe.
17. **TC-14.6.4.2** `test_cert_handle_resume_idempotent` -- Resume twice is safe.
18. **TC-14.6.4.3** `test_cert_handle_controller_disconnect` -- Disconnect fires input pause
    side-effect.
19. **TC-14.6.4.4** `test_cert_rating_lock_toggles` -- Enforce enabled then disabled; state
    observable.

## Integration Tests

| ID              | Name                                              | Req       |
|-----------------|---------------------------------------------------|-----------|
| TC-14.6.1.I1    | `test_contract_all_backends_pass_same_suite`      | R-14.6.1  |
| TC-14.6.2.I1    | `test_cargo_build_selects_correct_crate`          | R-14.6.2  |
| TC-14.6.5.I1    | `test_cert_test_harness_runs_under_ci`            | R-14.6.5  |
| TC-14.6.5.I2    | `test_rating_lock_enforced_end_to_end`            | R-14.6.5  |

1. **TC-14.6.1.I1** `test_contract_all_backends_pass_same_suite` -- A trait-contract test suite runs
   against every backend (stub + each private fork's impl) and all pass.
2. **TC-14.6.2.I1** `test_cargo_build_selects_correct_crate` -- Running `cargo build` with the PS5
   target links `console-ps5`, not the stub.
3. **TC-14.6.5.I1** `test_cert_test_harness_runs_under_ci` -- CI workflow in the private fork runs
   the cert test battery to completion.
4. **TC-14.6.5.I2** `test_rating_lock_enforced_end_to_end` -- Enable rating lock; subsequent online
   session creation fails with `RatingLocked`.

## Benchmarks

| ID              | Name                                 | Target       |
|-----------------|--------------------------------------|--------------|
| TC-14.6.4.B1    | `bench_handle_resume_latency`        | < 100 ms     |
| TC-14.6.4.B2    | `bench_handle_controller_disconnect` | < 50 ms      |
| TC-14.6.1.B1    | `bench_stub_call_overhead`           | < 20 ns      |

1. **TC-14.6.4.B1** -- Resume callback runs and returns. < 100 ms (cert requirement).
2. **TC-14.6.4.B2** -- Controller disconnect path. < 50 ms.
3. **TC-14.6.1.B1** -- Overhead of a single stub method call. < 20 ns.
