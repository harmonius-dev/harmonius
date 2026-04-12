# Save System ↔ Serialization Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-5.10.1.1 | Serialize single Saveable component | Entity with Transform (Saveable) | ComponentSnapshot with rkyv bytes | IR-5.10.1 |
| TC-IR-5.10.1.2 | Serialize entity with 5 components | Entity with 5 Saveable components | EntitySnapshot with 5 ComponentSnapshots | IR-5.10.1 |
| TC-IR-5.10.1.3 | Non-Saveable components excluded | Entity with Saveable + non-Saveable | Only Saveable components in snapshot | IR-5.10.1 |
| TC-IR-5.10.1.4 | Round-trip serialize/deserialize | Save then load same world | All Saveable components identical | IR-5.10.1 |
| TC-IR-5.10.2.1 | Zero-copy mmap load | Save file on disk | Archived data accessed without copy | IR-5.10.2 |
| TC-IR-5.10.2.2 | Mmap load validates checksum | Valid save file | CRC32 matches, load succeeds | IR-5.10.2 |
| TC-IR-5.10.2.3 | Mmap rejects corrupted file | Flipped bit in save file | LoadError::ChecksumMismatch | IR-5.10.2 |
| TC-IR-5.10.3.1 | Schema version stored in header | Save with schema v1.2.0 | SaveFileHeader.schema_version == 1.2.0 | IR-5.10.3 |
| TC-IR-5.10.3.2 | Migration chain v1 to v3 | Save at v1.0.0, current v3.0.0 | 2 migrations applied in order | IR-5.10.3 |
| TC-IR-5.10.3.3 | Migration adds new field | v1 component missing field | Field added with default value | IR-5.10.3 |
| TC-IR-5.10.3.4 | Migration renames field | v1 component with old_name | Field accessible as new_name | IR-5.10.3 |
| TC-IR-5.10.4.1 | Incremental save dirty entities | 1000 entities, 5 dirty | Only 5 EntitySnapshots written | IR-5.10.4 |
| TC-IR-5.10.4.2 | SaveDirty tick tracking | Modify entity at tick 42 | SaveDirty.dirty_tick == 42 | IR-5.10.4 |
| TC-IR-5.10.4.3 | Clean entities not re-serialized | 1000 entities, 0 dirty | Zero ComponentSnapshots written | IR-5.10.4 |
| TC-IR-5.10.5.1 | Codegen produces serialize fn | Saveable derive on struct | serialize_component callable | IR-5.10.5 |
| TC-IR-5.10.5.2 | Codegen produces deserialize fn | Saveable derive on struct | deserialize_component callable | IR-5.10.5 |
| TC-IR-5.10.6.1 | LZ4 compression applied | Full save, LZ4 mode | File smaller than uncompressed | IR-5.10.6 |
| TC-IR-5.10.6.2 | Zstd compression applied | Full save, Zstd mode | File smaller than LZ4 | IR-5.10.6 |
| TC-IR-5.10.6.3 | Encryption round-trip | Save with encryption | Load decrypts successfully | IR-5.10.6 |
| TC-IR-5.10.6.4 | Wrong key fails decryption | Load with wrong key | LoadError::DecryptionFailed | IR-5.10.6 |
| TC-IR-5.10.7.1 | Entity IDs remapped on load | Save with IDs 1-100, load into populated world | No ID collisions, all refs valid | IR-5.10.7 |
| TC-IR-5.10.7.2 | Parent-child refs survive remap | Hierarchy with 3 levels | Parent/child relationships preserved | IR-5.10.7 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-5.10.1.B1 | Full save 10K entities | < 100 ms | IR-5.10.1 |
| TC-IR-5.10.2.B1 | Mmap load 10 MB save file | < 50 ms | IR-5.10.2 |
| TC-IR-5.10.4.B1 | Incremental save 100 dirty of 10K | < 10 ms | IR-5.10.4 |
| TC-IR-5.10.6.B1 | LZ4 compress 10 MB | < 5 ms | IR-5.10.6 |
| TC-IR-5.10.6.B2 | Compressed file size | < 10 MB for typical world | IR-5.10.6 |
| TC-IR-5.10.1.B2 | rkyv serialize throughput | >= 500 MB/s | IR-5.10.1 |
