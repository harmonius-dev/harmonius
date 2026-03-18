# R-13.3 — Save System Requirements

## Serialization

| ID       | Derived From                                             |
|----------|----------------------------------------------------------|
| R-13.3.1 | [F-13.3.1](../../features/game-framework/save-system.md) |
| R-13.3.2 | [F-13.3.2](../../features/game-framework/save-system.md) |

1. **R-13.3.1** — The engine **SHALL** serialize the player's game state (character data, inventory,
   quest progress, ability loadout, map exploration, achievement progress) into a versioned binary
   format using the engine's reflection and serialization systems, supporting partial serialization
   of dirty fields only to reduce save size and I/O cost.
   - **Rationale:** Reflection-based serialization eliminates hand-written serializers for each
     type, and partial dirty-field writes minimize I/O cost for characters with extensive data.
   - **Verification:** Integration test: create a character with inventory, quest progress, and
     ability loadout. Serialize to binary, deserialize, and verify all fields match. Modify a single
     field, serialize with partial mode, and verify only the dirty field is written. Verify the
     binary format includes a version stamp.
2. **R-13.3.2** — The engine **SHALL** stamp each save file with a schema version and apply ordered
   migration transforms (version N to N+1) on load, supporting field addition, removal, renaming,
   and reshaping, such that older saves load correctly after schema evolution without data loss.
   - **Rationale:** Live-service games must evolve save schemas across patches and expansions
     without invalidating existing player data.
   - **Verification:** Unit test: create a save at schema version 1. Register migrations from v1 to
     v2 (add field) and v2 to v3 (rename field). Load the v1 save and verify both migrations apply
     in order, producing a valid v3 save with the added and renamed fields intact.

## Save Triggers

| ID       | Derived From                                             |
|----------|----------------------------------------------------------|
| R-13.3.3 | [F-13.3.3](../../features/game-framework/save-system.md) |
| R-13.3.4 | [F-13.3.4](../../features/game-framework/save-system.md) |

1. **R-13.3.3** — The engine **SHALL** trigger automatic saves at designer-placed checkpoints (zone
   transitions, quest milestones, boss kills) and at configurable time intervals, writing to a
   rotating slot to prevent corruption from interrupted writes, with instanced content checkpoints
   capturing instance progress (bosses defeated, lockout state) for session resumption.
   - **Rationale:** Automatic saves at meaningful checkpoints and time intervals protect player
     progress without manual intervention, and rotating slots guard against write corruption.
   - **Verification:** Integration test: place a checkpoint at a zone transition, cross it, and
     verify an autosave is written. Verify the rotating slot advances on each write. Simulate a
     crash mid-write and verify the previous slot remains intact. Verify instance checkpoints record
     boss-defeated state for session resumption.
2. **R-13.3.4** — The engine **SHALL** provide a save slot system with metadata (character name,
   level, playtime, screenshot thumbnail, timestamp, zone name) for UI display, supporting copy,
   delete, and export/import operations, with all slot management operations being transactional to
   prevent data loss from concurrent access or interrupted writes.
   - **Rationale:** Transactional slot management with rich metadata provides a safe and informative
     save UI that prevents accidental data loss.
   - **Verification:** Unit test: create a save slot and verify metadata (name, level, playtime,
     timestamp) is correctly stored and retrievable. Copy a slot, verify the copy is identical.
     Delete a slot and verify it is removed. Simulate an interrupted copy operation and verify
     neither source nor destination is corrupted.

## Cloud and Platform Integration

| ID       | Derived From                                             |
|----------|----------------------------------------------------------|
| R-13.3.5 | [F-13.3.5](../../features/game-framework/save-system.md) |
| R-13.3.6 | [F-13.3.6](../../features/game-framework/save-system.md) |

1. **R-13.3.5** — The engine **SHALL** synchronize save data with platform cloud storage (Steam
   Cloud, PlayStation Plus Cloud, Xbox Cloud Saves, iCloud, Epic Online Services) using each
   platform's native API, handling conflict resolution when local and cloud saves diverge, with
   fully async upload/download operations that never block the game thread.
   - **Rationale:** Cloud save integration ensures player progress persists across devices and
     protects against local data loss, while async I/O prevents frame stalls.
   - **Verification:** Integration test: save locally and upload to cloud storage. Modify the local
     save, trigger sync, and verify conflict resolution prompts the user. Verify upload and download
     operations complete without blocking the game thread (measure zero frame stalls exceeding 1
     ms). Verify platform-native async I/O is used (no stdlib file I/O).
2. **R-13.3.6** — The engine **SHALL** route all save reads and writes through an async I/O pipeline
   that compresses (LZ4 for speed, Zstd for cloud uploads), encrypts (AES-256-GCM), and checksums
   (CRC-32) save data, using atomic rename for crash-safe writes and priority ordering so autosaves
   yield to explicit user saves.
   - **Rationale:** Compression, encryption, and checksumming protect save integrity and privacy,
     while atomic rename and priority ordering ensure consistency under all conditions.
   - **Verification:** Integration test: write a save through the pipeline and verify the output is
     compressed, encrypted, and checksummed. Simulate a crash mid-write and verify the previous save
     file remains intact (atomic rename). Issue an autosave followed immediately by a user save and
     verify the user save takes priority. Verify platform-native async I/O is used (no stdlib file
     I/O).

## Non-Functional Requirements

| ID         | Derived From       |
|------------|--------------------|
| R-13.3.NF1 | F-13.3.1, F-13.3.6 |
| R-13.3.NF2 | F-13.3.1, F-13.3.5 |
| R-13.3.NF3 | F-13.3.6           |

1. **R-13.3.NF1** — The engine **SHALL** complete a full save operation (serialization, compression,
   encryption, and write) within 100 ms on target hardware, measured from save initiation to write
   completion, so that autosaves do not produce perceptible hitches during gameplay.
   - **Rationale:** Saves that exceed 100 ms risk visible frame hitches or input lag, degrading the
     player experience during autosave triggers.
   - **Verification:** Benchmark: serialize a character with 500 inventory items, 50 active quests,
     and 200 achievement entries. Measure end-to-end save time across 100 iterations and verify the
     99th percentile is under 100 ms on target hardware.
2. **R-13.3.NF2** — The engine **SHALL** produce save files no larger than 10 MB after compression
   for a single character with maximum progression data, ensuring reasonable cloud upload times and
   storage consumption.
   - **Rationale:** Large save files increase cloud sync times and storage costs, and may exceed
     platform cloud storage quotas on constrained platforms.
   - **Verification:** Create a character with maximum inventory (all slots filled), all quests
     completed, all achievements earned, and full customization. Serialize and compress; verify the
     output file size is under 10 MB.
3. **R-13.3.NF3** — The engine **SHALL** guarantee that no save data is lost or corrupted if the
   process terminates unexpectedly (crash, power loss, force quit) at any point during a save
   operation.
   - **Rationale:** Players losing hours of progress due to a crash during save is unacceptable;
     atomic writes ensure the previous valid save always remains intact.
   - **Verification:** Inject process termination at 10 random points during a save pipeline
     execution. After each termination, verify the most recent completed save loads without
     corruption and no partial save files remain on disk.
