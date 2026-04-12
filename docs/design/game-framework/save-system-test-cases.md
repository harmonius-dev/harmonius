# Save System Test Cases

Companion test cases for [save-system.md](save-system.md).

## Unit Tests

### TC-13.3.1.1 Full-World Serialization

| # | Requirement |
|---|-------------|
| 1 | R-13.3.1    |

1. **#1** — World with 50 `Saveable` entities (Character context); call
   `serialize_world(Character, arena)`
   - **Expected:** Returns `Box<[u8]>` containing all 50 entity snapshots; no HashMap used

### TC-13.3.1.2 Dirty-Only Serialization

| # | Requirement |
|---|-------------|
| 1 | R-13.3.1    |

1. **#1** — 50 entities; mark 10 with `SaveDirty`; call `serialize_incremental`
   - **Expected:** Payload contains exactly 10 entity snapshots

### TC-13.3.1.3 Codegen Serialize Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-13.3.1    |

1. **#1** — Codegen'd `serialize_component` + `deserialize_component` for a test component
   - **Expected:** Roundtrip produces identical field values; rkyv bytes are valid

### TC-13.3.1.4 No HashMap in Serialize Path

| # | Requirement |
|---|-------------|
| 1 | R-13.3.1    |

1. **#1** — Run serialization with deterministic component type ordering
   - **Expected:** Output byte order is stable across runs; no HashMap iteration involved

### TC-13.3.1.5 Arena Reset After Save

| # | Requirement |
|---|-------------|
| 1 | R-13.3.1    |

1. **#1** — Run two consecutive saves; measure arena allocations
   - **Expected:** Arena is reset between saves; no unbounded growth

### TC-13.3.1.6 Save 2D Transform

| # | Requirement |
|---|-------------|
| 1 | R-13.3.1    |

1. **#1** — Entity with `Transform2D`; serialize and deserialize
   - **Expected:** `Transform2D` fields (position, rotation, scale) roundtrip correctly

### TC-13.3.2.1 Migration v1 to v3

| # | Requirement |
|---|-------------|
| 1 | R-13.3.2    |

1. **#1** — Register steps v1→v2 and v2→v3; call `migrate` with saved=v1, current=v3
   - **Expected:** Both steps applied in order; output matches v3 schema

### TC-13.3.2.2 Migration Failure Preserves Data

| # | Requirement |
|---|-------------|
| 1 | R-13.3.2    |

1. **#1** — Step v2→v3 returns error; original bytes unchanged
   - **Expected:** Original save bytes unmodified; `MigrationError::StepFailed` returned

### TC-13.3.2.3 Migration No Path

| # | Requirement |
|---|-------------|
| 1 | R-13.3.2    |

1. **#1** — No steps registered for `type_hash`; call `migrate`
   - **Expected:** Returns `MigrationError::NoPath`

### TC-13.3.2.4 Golden Save v1

| # | Requirement |
|---|-------------|
| 1 | R-13.3.2    |

1. **#1** — Load golden save fixture from v1 schema with current engine
   - **Expected:** Migration succeeds; game state matches expected post-migration values

### TC-13.3.2.5 Data Loss Warning

| # | Requirement |
|---|-------------|
| 1 | R-13.3.2    |

1. **#1** — `RemoveField` step on a field with non-default value
   - **Expected:** `MigrationError::DataLossWarning` emitted; migration still completes

### TC-13.3.2.6 Per-Component Version Tracking

| # | Requirement |
|---|-------------|
| 1 | R-13.3.2    |

1. **#1** — Save header contains version table; one component unchanged, one changed
   - **Expected:** Only changed component runs migration; unchanged component is zero-copy loaded

### TC-13.3.2.7 Split Component Migration

| # | Requirement |
|---|-------------|
| 1 | R-13.3.2    |

1. **#1** — `CharacterStats` splits into `HealthComponent` + `ManaComponent` via
   `MigrationTransform::SplitComponent`
   - **Expected:** Both new components present with correct values after migration

### TC-13.3.2.8 Lazy Migration on Access

| # | Requirement |
|---|-------------|
| 1 | R-13.3.2    |

1. **#1** — Open-world save with 10 000 entities; access 5 entities at load time
   - **Expected:** Only 5 entities migrated at load; remaining migrate on first access

### TC-13.3.2.9 Forward Compatibility Error

| # | Requirement |
|---|-------------|
| 1 | R-13.3.2    |

1. **#1** — Save created with `saved_version > current_version`; attempt to load
   - **Expected:** Returns `LoadError::ForwardCompatError` with clear version info

### TC-13.3.2.10 Cross-Platform Migration

| # | Requirement |
|---|-------------|
| 1 | R-13.3.2    |

1. **#1** — Save created on Windows; load on macOS (via cloud sync fixture)
   - **Expected:** Asset references resolve via asset ID; no path-separator errors

### TC-13.3.3.1 Checkpoint Trigger

| # | Requirement |
|---|-------------|
| 1 | R-13.3.3    |

1. **#1** — Call `checkpoint_save` on slot 3
   - **Expected:** `SaveStarted` event fired; save written to slot 3 path

### TC-13.3.3.2 Autosave Rotation

| # | Requirement |
|---|-------------|
| 1 | R-13.3.3    |

1. **#1** — `autosave_rotation=3`; trigger autosave 5 times
   - **Expected:** Slots rotate 0→1→2→0→1; no slot 3 created

### TC-13.3.3.3 Autosave Crash Mid-Write

| # | Requirement |
|---|-------------|
| 1 | R-13.3.3    |
| 2 | R-13.3.NF3  |

1. **#1** — Simulate process kill during temp file write
   - **Expected:** Final save path unchanged; temp file cleaned up on restart
2. **#2** — Load after crash
   - **Expected:** Previous valid save loads successfully; no data loss

### TC-13.3.4.1 Save Slot Metadata Extended

| # | Requirement |
|---|-------------|
| 1 | R-13.3.4    |

1. **#1** — Create slot; verify all `SaveSlotMeta` fields populated correctly
   - **Expected:** `real_world_date`, `engine_version`, `save_type`, `platform` all set

### TC-13.3.4.2 Meta File Separate from Save Archive

| # | Requirement |
|---|-------------|
| 1 | R-13.3.4    |

1. **#1** — Save slot 1; list directory
   - **Expected:** `slot_01.meta` and `slot_01.save` exist as separate files

### TC-13.3.4.3 Thumbnail Loaded on Demand

| # | Requirement |
|---|-------------|
| 1 | R-13.3.4    |

1. **#1** — Load `.meta` file; do not request thumbnail
   - **Expected:** `slot_01.thumb` not read; I/O request count = 1 (meta only)

### TC-13.3.4.4 Slot Copy Transactional

| # | Requirement |
|---|-------------|
| 1 | R-13.3.4    |

1. **#1** — Copy slot 1 → slot 2; simulate I/O failure mid-copy
   - **Expected:** Slot 2 either fully written or not created; slot 1 unchanged

### TC-13.3.4.5 Slot Delete

| # | Requirement |
|---|-------------|
| 1 | R-13.3.4    |

1. **#1** — Delete slot 2; list slots
   - **Expected:** Slot 2 absent from `list_slots()`; files removed from disk

### TC-13.3.4.6 Slot Export and Import

| # | Requirement |
|---|-------------|
| 1 | R-13.3.4    |

1. **#1** — Export slot 1 to `export.save`; import to slot 3
   - **Expected:** Slot 3 content identical to slot 1

### TC-13.3.5.1 Cloud Sync Upload

| # | Requirement |
|---|-------------|
| 1 | R-13.3.5    |

1. **#1** — Local save newer than remote; call `sync`
   - **Expected:** `SyncResult::Uploaded`; upload I/O submitted

### TC-13.3.5.2 Cloud Sync Conflict

| # | Requirement |
|---|-------------|
| 1 | R-13.3.5    |

1. **#1** — Both local and remote changed; call `sync`
   - **Expected:** `SyncResult::Conflict` with both `SaveSlotMeta` values; no data overwritten

### TC-13.3.6.1 Pipeline Compress and Encrypt

| # | Requirement |
|---|-------------|
| 1 | R-13.3.6    |

1. **#1** — Write 1 MB payload with LZ4 + AES-256-GCM
   - **Expected:** Output smaller than input; decrypted + decompressed matches original

### TC-13.3.6.2 Pipeline Atomic Rename

| # | Requirement |
|---|-------------|
| 1 | R-13.3.6    |
| 2 | R-13.3.NF3  |

1. **#1** — Write save; verify temp file renamed atomically
   - **Expected:** Final path present; no temp file remnant

### TC-13.3.6.3 Pipeline Priority Ordering

| # | Requirement |
|---|-------------|
| 1 | R-13.3.6    |

1. **#1** — Queue `Critical` and `Background` writes simultaneously
   - **Expected:** `Critical` write completes first

### TC-13.3.6.4 LZ4 vs Zstd Compression

| # | Requirement |
|---|-------------|
| 1 | R-13.3.6    |

1. **#1** — Compress 1 MB with LZ4 and Zstd level 3; compare ratio and speed
   - **Expected:** LZ4 faster; Zstd smaller; both decompress to identical output

### TC-13.3.6.5 Wrong Decryption Key

| # | Requirement |
|---|-------------|
| 1 | R-13.3.6    |

1. **#1** — Encrypt save; attempt read with wrong key
   - **Expected:** Returns `LoadError::DecryptionFailed`

### TC-13.3.6.6 Procedural Voxel Delta Save

| # | Requirement |
|---|-------------|
| 1 | R-13.3.1    |

1. **#1** — Modify 100 voxel blocks; save and load
   - **Expected:** Delta contains only 100 changed blocks; base terrain restored + delta applied

### TC-13.3.6.7 Procedural Mesh Deform Save

| # | Requirement |
|---|-------------|
| 1 | R-13.3.1    |

1. **#1** — Apply destruction deformation to mesh; save and load
   - **Expected:** Displaced vertices match original deformation after restore

## Integration Tests

### TC-13.3.7.1 Save Load Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-13.3.1    |
| 2 | R-13.3.6    |

1. **#1** — Full world save + load; compare ECS world state before and after
   - **Expected:** All `Saveable` component values identical after roundtrip

### TC-13.3.7.2 Save Does Not Drop Frame

| # | Requirement |
|---|-------------|
| 1 | R-13.3.NF1  |

1. **#1** — Trigger save during active game loop; measure frame time
   - **Expected:** No frame exceeds 16.67 ms (60 Hz) during save submission

### TC-13.3.7.3 Save Under 100ms

| # | Requirement |
|---|-------------|
| 1 | R-13.3.NF1  |

1. **#1** — Full save of max character state (p99 across 100 runs)
   - **Expected:** p99 latency < 100 ms

### TC-13.3.7.4 Save File Under 10 MB

| # | Requirement |
|---|-------------|
| 1 | R-13.3.NF2  |

1. **#1** — Save max character + world state with LZ4 compression
   - **Expected:** Compressed save archive ≤ 10 MB

### TC-13.3.7.5 Crash Safety 10 Points

| # | Requirement |
|---|-------------|
| 1 | R-13.3.NF3  |

1. **#1** — Kill process at 10 different points during write pipeline
   - **Expected:** Previous valid save intact at each crash point

### TC-13.3.7.6 Cloud Sync No Block

| # | Requirement |
|---|-------------|
| 1 | R-13.3.5    |

1. **#1** — Trigger cloud sync during game loop; measure game thread stall
   - **Expected:** Game thread stall = 0 ms; sync runs on main thread I/O path

### TC-13.3.7.7 Frame Boundary SaveComplete Event

| # | Requirement |
|---|-------------|
| 1 | R-13.3.6    |

1. **#1** — Trigger save at frame N; record when `SaveComplete` event fires
   - **Expected:** `SaveComplete` delivered at frame N+1 or later; never same frame

### TC-13.3.7.8 Migration CI Golden Saves

| # | Requirement |
|---|-------------|
| 1 | R-13.3.2    |

1. **#1** — CI loads all golden save fixtures from versions v1.0 through current
   - **Expected:** All migrations succeed; post-migration game state matches expected values

## Benchmarks

| Benchmark | Target | Requirement |
|-----------|--------|-------------|
| Full save p99 (max character) | < 100 ms | R-13.3.NF1 |
| Incremental save (10 dirty entities) | < 10 ms p99 | R-13.3.1 |
| LZ4 compress 5 MB | < 5 ms | R-13.3.6 |
| AES-256-GCM encrypt 5 MB | < 10 ms | R-13.3.6 |
| Save archive size (max state) | ≤ 10 MB | R-13.3.NF2 |
| `SaveSlotMeta` read from disk | < 1 ms per slot | R-13.3.4 |
