# Plugin Marketplace -- Test Cases

Companion to [plugin-marketplace.md](plugin-marketplace.md).

Test case IDs use `TC-15.17.Z.N` format.

## Unit Tests

| ID               | Name                                          | Req         |
|------------------|-----------------------------------------------|-------------|
| TC-15.17.1.1     | `test_catalog_query_roundtrip`                | R-15.17.1   |
| TC-15.17.1.2     | `test_catalog_pagination_cursor`              | R-15.17.1   |
| TC-15.17.2.1     | `test_install_fetches_missing_blobs`          | R-15.17.2   |
| TC-15.17.2.2     | `test_install_skips_cached_blobs`             | R-15.17.2   |
| TC-15.17.2.3     | `test_install_atomic_on_failure`              | R-15.17.2   |
| TC-15.17.2.4     | `test_uninstall_removes_symlink`              | R-15.17.2   |
| TC-15.17.7.1     | `test_manifest_rkyv_roundtrip`                | R-15.17.7   |
| TC-15.17.7.2     | `test_blob_integrity_mismatch_rejected`       | R-15.17.7   |
| TC-15.17.7.3     | `test_file_role_counts_by_kind`               | R-15.17.7   |
| TC-15.17.8.1     | `test_signature_valid_accepted`               | R-15.17.8   |
| TC-15.17.8.2     | `test_signature_invalid_rejected`             | R-15.17.8   |
| TC-15.17.8.3     | `test_unknown_publisher_unsigned_level`       | R-15.17.8   |
| TC-15.17.8.4     | `test_official_key_trust_level`               | R-15.17.8   |
| TC-15.17.12.1    | `test_resolver_picks_highest_compatible`      | R-15.17.12  |
| TC-15.17.12.2    | `test_resolver_backtracks_on_conflict`        | R-15.17.12  |
| TC-15.17.12.3    | `test_resolver_detects_cycle`                 | R-15.17.12  |
| TC-15.17.12.4    | `test_resolver_topological_order`             | R-15.17.12  |
| TC-15.17.12.5    | `test_resolver_engine_range_filter`           | R-15.17.12  |
| TC-15.17.12.6    | `test_resolver_deterministic_with_equal_set`  | R-15.17.12  |
| TC-15.17.30.1    | `test_update_check_reports_higher_version`    | R-15.17.30  |
| TC-15.17.30.2    | `test_update_check_honors_pinning`            | R-15.17.30  |
| TC-15.17.30.3    | `test_bulk_update_resolves_all_non_pinned`    | R-15.17.30  |

1. **TC-15.17.1.1** `test_catalog_query_roundtrip` -- Encode `CatalogQuery`, decode, fields equal.
2. **TC-15.17.1.2** `test_catalog_pagination_cursor` -- Paginated list returns distinct pages across
   sequential cursors.
3. **TC-15.17.2.1** `test_install_fetches_missing_blobs` -- With an empty CAS, install requests all
   blobs from the client and writes them to CAS.
4. **TC-15.17.2.2** `test_install_skips_cached_blobs` -- With a populated CAS, install issues zero
   network fetches.
5. **TC-15.17.2.3** `test_install_atomic_on_failure` -- Injected codegen failure rolls back symlink
   and leaves no partial installed state.
6. **TC-15.17.2.4** `test_uninstall_removes_symlink` -- After uninstall, `installed/<id>/current` is
   absent.
7. **TC-15.17.7.1** `test_manifest_rkyv_roundtrip` -- Manifest archive round-trips bit-identical.
8. **TC-15.17.7.2** `test_blob_integrity_mismatch_rejected` -- Corrupted blob hash triggers
   `InstallError::IntegrityFailure`.
9. **TC-15.17.7.3** `test_file_role_counts_by_kind` -- Manifest with 2 codegen, 3 assets, 1 license
   reports the expected role counts.
10. **TC-15.17.8.1** `test_signature_valid_accepted` -- Signed manifest verified under publisher key
    returns `TrustLevel::Verified`.
11. **TC-15.17.8.2** `test_signature_invalid_rejected` -- Tampered manifest fails verification.
12. **TC-15.17.8.3** `test_unknown_publisher_unsigned_level` -- Unknown publisher yields
    `TrustLevel::Unsigned`.
13. **TC-15.17.8.4** `test_official_key_trust_level` -- Manifest signed with official key yields
    `TrustLevel::Official`.
14. **TC-15.17.12.1** `test_resolver_picks_highest_compatible` -- Given versions 1.0.0, 1.1.0, 2.0.0
    and range `^1.0`, resolver picks 1.1.0.
15. **TC-15.17.12.2** `test_resolver_backtracks_on_conflict` -- Two packages impose conflicting
    sub-dependency ranges; resolver picks the other branch.
16. **TC-15.17.12.3** `test_resolver_detects_cycle` -- A depends on B, B depends on A; returns
    `ResolveError::Cycle`.
17. **TC-15.17.12.4** `test_resolver_topological_order` -- Output order places dependencies before
    dependents.
18. **TC-15.17.12.5** `test_resolver_engine_range_filter` -- Plugin with `engine_range = ^1.2` is
    filtered when current engine is 1.1.
19. **TC-15.17.12.6** `test_resolver_deterministic_with_equal_set` -- Two runs with identical inputs
    produce identical output.
20. **TC-15.17.30.1** `test_update_check_reports_higher_version` -- Installed 1.0.0 and catalog
    offers 1.1.0; `UpdateCandidate` emitted.
21. **TC-15.17.30.2** `test_update_check_honors_pinning` -- Pinned plugins are reported but
    `pinned = true` and skipped by bulk update.
22. **TC-15.17.30.3** `test_bulk_update_resolves_all_non_pinned` -- Bulk update installs every
    non-pinned candidate in one resolve pass.

## Integration Tests

| ID               | Name                                             | Req         |
|------------------|--------------------------------------------------|-------------|
| TC-15.17.2.I1    | `test_install_drives_codegen_and_loads_dylib`    | R-15.17.2   |
| TC-15.17.29.I1   | `test_self_hosted_registry_same_protocol`        | R-15.17.29  |
| TC-15.17.29.I2   | `test_registry_fetch_offline_uses_cas`           | R-15.17.29  |
| TC-15.17.8.I1    | `test_trust_store_reject_unsigned_in_shipping`   | R-15.17.8   |
| TC-15.17.1.I1    | `test_end_to_end_install_and_enable`             | R-15.17.1   |

1. **TC-15.17.2.I1** `test_install_drives_codegen_and_loads_dylib` -- Install a real fixture plugin,
   middleman codegen runs, `.dylib` is produced and loadable.
2. **TC-15.17.29.I1** `test_self_hosted_registry_same_protocol` -- Bring up a local registry server
   and install against it with identical client code.
3. **TC-15.17.29.I2** `test_registry_fetch_offline_uses_cas` -- Disable network, run install with a
   fully cached CAS; install succeeds.
4. **TC-15.17.8.I1** `test_trust_store_reject_unsigned_in_shipping` -- Shipping editor build rejects
   installation of an unsigned plugin with a clear error.
5. **TC-15.17.1.I1** `test_end_to_end_install_and_enable` -- Browse, install, and enable a fixture
   plugin end-to-end; middleman dylib exports expected symbols.

## Benchmarks

| ID               | Name                                     | Target       |
|------------------|------------------------------------------|--------------|
| TC-15.17.1.B1    | `bench_catalog_query_round_trip`         | < 100 ms     |
| TC-15.17.12.B1   | `bench_resolve_100_plugins`              | < 20 ms      |
| TC-15.17.12.B2   | `bench_resolve_1000_plugins`             | < 200 ms     |
| TC-15.17.2.B1    | `bench_install_small_fixture`            | < 500 ms     |
| TC-15.17.8.B1    | `bench_signature_verification`           | < 1 ms       |

1. **TC-15.17.1.B1** -- Round trip a single catalog query over loopback HTTP/3. < 100 ms.
2. **TC-15.17.12.B1** -- Resolve 100 plugins against a catalog of 1000. < 20 ms.
3. **TC-15.17.12.B2** -- Resolve 1000 plugins against a catalog of 10k. < 200 ms.
4. **TC-15.17.2.B1** -- Install a small fixture plugin from a warm CAS. < 500 ms.
5. **TC-15.17.8.B1** -- Verify an ed25519 signature on a 4 KiB manifest. < 1 ms.
