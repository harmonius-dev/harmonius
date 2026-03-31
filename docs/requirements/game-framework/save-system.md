# R-13.3 — Save System Requirements

## Serialization

1. **R-13.3.1** — The engine **SHALL** serialize game state (character data, inventory, quest
   progress, ability loadout, achievements) into a versioned binary format using reflection to
   capture ECS components without hand-written serializers, supporting partial serialization of
   dirty fields only.
   - **Rationale:** Reflection-based serialization ensures new component types are saved
     automatically; partial writes reduce I/O cost.
   - **Verification:** Save a character with full inventory and quest state. Load and verify every
     field matches. Add a new component type and verify it serializes without custom code.

2. **R-13.3.2** — The engine **SHALL** stamp each save with a schema version and apply ordered
   migration steps (add, remove, rename, reshape) when loading older saves, running automatically
   without user intervention.
   - **Rationale:** Automatic migration enables live-service patches without invalidating player
     data.
   - **Verification:** Save with schema v1. Apply a migration adding a field. Load and verify the
     migration runs and the field is defaulted. Chain 3 migrations (v1 to v4) and verify all apply
     in order.

## Save Triggers

3. **R-13.3.3** — The engine **SHALL** trigger automatic saves at designer-placed checkpoints and
   configurable time intervals, writing to a rotating slot to prevent corruption from interrupted
   writes.
   - **Rationale:** Rotating slots ensure a previous save survives if a write is interrupted.
   - **Verification:** Configure a 5-minute autosave interval with 3 rotating slots. Verify saves
     cycle through slots. Interrupt a write and verify the previous slot is intact.

4. **R-13.3.4** — The engine **SHALL** provide save slots with metadata (character name, level,
   playtime, screenshot thumbnail, timestamp) and support copy, delete, and export/import operations
   with transactional safety.
   - **Rationale:** Metadata enables informed slot selection; transactional ops prevent data loss
     from concurrent access.
   - **Verification:** Save to a slot and verify metadata is populated. Copy to a new slot and
     verify independent state. Delete a slot and verify removal. Export and import and verify data
     integrity.

## Cloud and Platform

5. **R-13.3.5** — The engine **SHALL** synchronize save data with platform cloud storage (Steam
   Cloud, iCloud, PlayStation Plus Cloud, Xbox Cloud Saves) using each platform's native async API,
   with conflict resolution when local and cloud diverge.
   - **Rationale:** Cloud sync enables cross-device continuity; async prevents blocking gameplay.
   - **Verification:** Save locally and to cloud. Modify only the local save. Sync and verify
     conflict resolution prompts. Verify sync completes without blocking the game thread.

6. **R-13.3.6** — The engine **SHALL** compress (LZ4/Zstd), encrypt (AES-256-GCM), and checksum
   (CRC-32) all save data through an async I/O pipeline using atomic rename for consistency, with
   platform-native async I/O (IOCP, GCD, io_uring).
   - **Rationale:** Compression reduces size; encryption prevents tampering; atomic rename
     guarantees consistency even on crash.
   - **Verification:** Save and verify the file is compressed and encrypted. Corrupt a byte and
     verify the checksum detects it. Kill the process mid-write and verify the previous save is
     intact via atomic rename.
