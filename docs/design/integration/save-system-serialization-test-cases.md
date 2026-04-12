# Save System ↔ Serialization Integration Test Cases

All tests in this file are CI-runnable: no physical disk fixtures, no external services, no manual
steps. Integration tests use an in-memory `VirtualFileSystem` fake that implements the same trait as
the real platform-native VFS. Benchmarks run under `cargo bench` with a fixed RNG seed.

## Integration Tests

Columns refer to IDs only; Input, Expected, and Notes appear in the detail list below the table.

| ID | Test | Req |
|----|------|-----|
| TC-IR-5.10.1.1 | Serialize single Saveable component | IR-5.10.1 |
| TC-IR-5.10.1.2 | Serialize entity with 5 components | IR-5.10.1 |
| TC-IR-5.10.1.3 | Non-Saveable components excluded | IR-5.10.1 |
| TC-IR-5.10.1.4 | Round-trip serialize/deserialize | IR-5.10.1 |
| TC-IR-5.10.1.5 | Arena overflow retry | IR-5.10.1 |
| TC-IR-5.10.1.6 | Transform2D Saveable (2D dimension) | IR-5.10.1 |
| TC-IR-5.10.1.7 | Transform2D + depth Saveable (2.5D) | IR-5.10.1 |
| TC-IR-5.10.2.1 | Zero-copy mmap load | IR-5.10.2 |
| TC-IR-5.10.2.2 | Mmap load validates checksum | IR-5.10.2 |
| TC-IR-5.10.2.3 | Mmap rejects corrupted file | IR-5.10.2 |
| TC-IR-5.10.2.4 | Full load round-trip (end-to-end) | IR-5.10.2 |
| TC-IR-5.10.2.5 | Invalid header length prefix | IR-5.10.2 |
| TC-IR-5.10.3.1 | Schema version stored in header | IR-5.10.3 |
| TC-IR-5.10.3.2 | Migration chain v1 to v3 | IR-5.10.3 |
| TC-IR-5.10.3.3 | Migration adds new field | IR-5.10.3 |
| TC-IR-5.10.3.4 | Migration renames field | IR-5.10.3 |
| TC-IR-5.10.3.5 | Migration step error propagates | IR-5.10.3 |
| TC-IR-5.10.4.1 | Incremental save dirty entities | IR-5.10.4 |
| TC-IR-5.10.4.2 | SaveDirty tick tracking | IR-5.10.4 |
| TC-IR-5.10.4.3 | Clean entities not re-serialized | IR-5.10.4 |
| TC-IR-5.10.5.1 | Codegen produces serialize fn | IR-5.10.5 |
| TC-IR-5.10.5.2 | Codegen produces deserialize fn | IR-5.10.5 |
| TC-IR-5.10.6.1 | LZ4 compression applied | IR-5.10.6 |
| TC-IR-5.10.6.2 | Zstd compression applied | IR-5.10.6 |
| TC-IR-5.10.6.3 | Encryption round-trip | IR-5.10.6 |
| TC-IR-5.10.6.4 | Wrong key fails decryption | IR-5.10.6 |
| TC-IR-5.10.6.5 | I/O write failure retry | IR-5.10.6 |
| TC-IR-5.10.7.1 | Entity IDs remapped on load | IR-5.10.7 |
| TC-IR-5.10.7.2 | Parent-child refs survive remap | IR-5.10.7 |
| TC-IR-5.10.8.1 | Cloud slot upload round-trip | IR-5.10.8 |
| TC-IR-5.10.8.2 | Cloud conflict detected and resolved | IR-5.10.8 |

Detail list:

1. TC-IR-5.10.1.1 — Input: entity with `Transform` (Saveable). Expected: `ComponentSnapshot` with
   `type_hash == Transform::TYPE_HASH` and rkyv-archived bytes.
2. TC-IR-5.10.1.2 — Input: entity with 5 Saveable components. Expected: `EntitySnapshot` with
   exactly 5 `ComponentSnapshot` entries in deterministic order.
3. TC-IR-5.10.1.3 — Input: entity with 2 Saveable and 1 non-Saveable. Expected: snapshot contains
   only the 2 Saveable entries.
4. TC-IR-5.10.1.4 — Input: `World` with 10 entities, save then load into a fresh `World`. Expected:
   all Saveable components byte-identical after round-trip.
5. TC-IR-5.10.1.5 — Input: arena initialized at 256 KiB, world requires 700 KiB. Expected:
   serializer returns `SaveError::ArenaOverflow` on first try, retries with 512 KiB, then 1 MiB,
   succeeds; final buffer is complete. Negative sub-case: 4th overflow returns
   `SaveError::ArenaOverflow` with no retry.
6. TC-IR-5.10.1.6 — Input: 2D world with `Transform2D` components only. Expected:
   `SaveFileHeader.dimension == WorldDimension::D2`, round-trip succeeds.
7. TC-IR-5.10.1.7 — Input: 2.5D world with `Transform2D` + depth sort component. Expected:
   `SaveFileHeader.dimension == WorldDimension::D2_5`, round-trip succeeds.
8. TC-IR-5.10.2.1 — Input: valid save file on in-memory VFS. Expected:
   `check_archived_root::<SaveFileHeader>` returns without copying the file body.
9. TC-IR-5.10.2.2 — Input: valid save file. Expected: CRC-32C matches, load returns `Ok`.
10. TC-IR-5.10.2.3 — Negative. Input: save file with one flipped bit in the payload. Expected:
    `LoadError::ChecksumMismatch`.
11. TC-IR-5.10.2.4 — Full end-to-end load round-trip: save a 100-entity world, clear the live world,
    call `load_world`, verify every entity re-appears with all components identical and parent/child
    links intact.
12. TC-IR-5.10.2.5 — Negative. Input: save file with header length prefix larger than file size.
    Expected: `LoadError::InvalidHeader`.
13. TC-IR-5.10.3.1 — Input: save with `config.current_schema == 1.2.0`. Expected:
    `SaveFileHeader.schema_version == SchemaVersion { major: 1, minor: 2, patch: 0 }`.
14. TC-IR-5.10.3.2 — Input: save at v1.0.0, load with current schema v3.0.0, 2 migration steps
    registered. Expected: both steps applied in topological order.
15. TC-IR-5.10.3.3 — Input: v1 component archive missing field `hp`. Expected: migration fills `hp`
    with `Default::default()`.
16. TC-IR-5.10.3.4 — Input: v1 component archive with field `hp`, v2 renames to `health`. Expected:
    after migration the field is readable as `health`.
17. TC-IR-5.10.3.5 — Negative. Input: migration step returns `Err`. Expected:
    `LoadError::MigrationFailed { step_index, type_hash }`, original file untouched.
18. TC-IR-5.10.4.1 — Input: 1000 entities, 5 with `SaveDirty`. Expected: exactly 5 `EntitySnapshot`
    entries written.
19. TC-IR-5.10.4.2 — Input: modify entity at tick 42. Expected: `SaveDirty.dirty_tick == 42`.
20. TC-IR-5.10.4.3 — Input: 1000 entities, none dirty. Expected: zero `ComponentSnapshot` entries
    written, save file still has valid header.
21. TC-IR-5.10.5.1 — Input: apply `#[derive(Saveable)]` to a struct via codegen. Expected:
    `serialize_component::<MyType>` is a defined symbol in the middleman `.dylib`.
22. TC-IR-5.10.5.2 — Input: same struct. Expected: `deserialize_component::<MyType>` is a defined
    symbol in the middleman `.dylib`.
23. TC-IR-5.10.6.1 — Input: full save with `CompressionKind::Lz4`. Expected: compressed size <
    uncompressed size for a typical world.
24. TC-IR-5.10.6.2 — Input: full save with `CompressionKind::Zstd`. Expected: compressed size < LZ4
    compressed size for the same world.
25. TC-IR-5.10.6.3 — Input: save with `EncryptionKind::Aes256Gcm`, load with the same key. Expected:
    decryption succeeds, payload bytes match pre-encryption.
26. TC-IR-5.10.6.4 — Negative. Input: load with wrong key. Expected: `LoadError::DecryptionFailed`,
    previous slot untouched.
27. TC-IR-5.10.6.5 — Negative. Input: fake VFS configured to fail the first 2 writes, succeed on the
    3rd. Expected: retry budget 3 is honored, final write succeeds, `SaveSlotManager::get(previous)`
    still returns the prior valid save unchanged.
28. TC-IR-5.10.7.1 — Input: save with IDs 1–100, load into a `World` that already has IDs 1–50.
    Expected: no collisions, all internal references remapped via the `stable_id` table.
29. TC-IR-5.10.7.2 — Input: hierarchy with 3 levels (root, mid, leaf). Expected: after load the
    parent/child relationships are preserved.
30. TC-IR-5.10.8.1 — Input: save a world to `SlotKind::Cloud`, simulate upload via fake cloud VFS,
    load from cloud into fresh `World`. Expected: round-trip succeeds, `CloudSyncState::InSync`.
31. TC-IR-5.10.8.2 — Input: local save at t=10, cloud save at t=20, load triggers conflict.
    Expected: `CloudSyncState::Conflict`, resolver picks the newer timestamp, loser preserved as
    `SlotKind::Auto` backup.

## Negative / Error Tests

| ID | Test | Expected | Req |
|----|------|----------|-----|
| TC-IR-5.10.2.3 | Flipped-bit payload | `LoadError::ChecksumMismatch` | IR-5.10.2 |
| TC-IR-5.10.2.5 | Oversize header length prefix | `LoadError::InvalidHeader` | IR-5.10.2 |
| TC-IR-5.10.3.5 | Migration step returns `Err` | `LoadError::MigrationFailed` | IR-5.10.3 |
| TC-IR-5.10.6.4 | Wrong decryption key | `LoadError::DecryptionFailed` | IR-5.10.6 |
| TC-IR-5.10.6.5 | I/O write fails first 2 | Retries succeed; prior slot intact | IR-5.10.6 |
| TC-IR-5.10.1.5 | Arena overflow past 1 MiB | `SaveError::ArenaOverflow` | IR-5.10.1 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-5.10.1.B1 | Full save 10K entities | < 100 ms | IR-5.10.1 |
| TC-IR-5.10.1.B2 | rkyv serialize throughput | >= 500 MB/s | IR-5.10.1 |
| TC-IR-5.10.2.B1 | Mmap load 10 MB save file | < 50 ms | IR-5.10.2 |
| TC-IR-5.10.2.B2 | Full load round-trip 10K entities | < 50 ms | IR-5.10.2 |
| TC-IR-5.10.4.B1 | Incremental save 100 dirty of 10K | < 10 ms | IR-5.10.4 |
| TC-IR-5.10.6.B1 | LZ4 compress 10 MB | < 5 ms | IR-5.10.6 |
| TC-IR-5.10.6.B2 | Compressed file size | < 10 MB for typical world | IR-5.10.6 |
| TC-IR-5.10.6.B3 | AES-256-GCM encrypt 10 MB | < 3 ms | IR-5.10.6 |
| TC-IR-5.10.6.B4 | AES-256-GCM decrypt 10 MB | < 3 ms | IR-5.10.6 |
| TC-IR-5.10.6.B5 | ChaCha20-Poly1305 encrypt 10 MB | < 5 ms | IR-5.10.6 |
| TC-IR-5.10.8.B1 | Cloud slot upload 10 MB (fake VFS) | < 20 ms | IR-5.10.8 |

All benchmarks run in CI under `cargo bench` with a fixed RNG seed so results are reproducible.
Variance ceilings are enforced via the shared benchmark harness.
