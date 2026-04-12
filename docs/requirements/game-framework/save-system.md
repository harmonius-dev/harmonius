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
   Tokio async I/O.
   - **Rationale:** Compression reduces size; encryption prevents tampering; atomic rename
     guarantees consistency even on crash.
   - **Verification:** Save and verify the file is compressed and encrypted. Corrupt a byte and
     verify the checksum detects it. Kill the process mid-write and verify the previous save is
     intact via atomic rename.

7. **R-13.3.7** — The engine **SHALL** snapshot procedural asset blobs (terrain modifications,
   generated textures) alongside entity component state in the save archive.
   - **Rationale:** Procedural modifications are part of game state and must persist across
     save/load cycles.
   - **Verification:** Modify terrain procedurally, save, load, and verify the terrain matches the
     saved modification.

8. **R-13.3.8** — The engine **SHALL** track schema versions per-component type in the save header,
   migrating only changed components on load.
   - **Rationale:** Per-component versioning avoids migrating unchanged data and enables independent
     component evolution.
   - **Verification:** Change one component's schema, save with the old schema, load, and verify
     only that component migrates while others load directly.

9. **R-13.3.9** — The engine **SHALL** support save migration transforms including split component,
   merge components, reparent entity, create/delete entity, cross-component field move, and data
   table rekeying.
   - **Rationale:** Complex schema evolution (splitting components, reparenting entities) requires
     rich migration primitives beyond add/remove/rename.
   - **Verification:** Register a split-component migration, save with the old schema, load, and
     verify the component is split into two with correct field values.

10. **R-13.3.10** — The engine **SHALL** support save context filtering (character, world, instance,
    settings) allowing selective serialization of entity subsets.
    - **Rationale:** Different save contexts (character vs. world) enable separate save files for
      different game state domains.
    - **Verification:** Tag entities with character and world contexts, save with character context
      only, and verify world entities are excluded.

11. **R-13.3.11** — The engine **SHALL** emit typed events for all save lifecycle stages (start,
    complete, fail) including cloud sync and conflict events, enabling UI feedback and game logic
    hooks.
    - **Rationale:** Save lifecycle events enable loading screens, progress bars, and conflict
      resolution UI.
    - **Verification:** Register an event listener, trigger a save, and verify SaveStarted and
      SaveComplete events fire in order.

12. **R-13.3.12** — The engine **SHALL** store rich per-slot metadata including character class,
    party composition, quest progress summary, death count, currency, completion percentage,
    difficulty, and extensible custom fields.
    - **Rationale:** Rich metadata enables informed save slot selection without loading full game
      state.
    - **Verification:** Save with custom metadata fields, read slot metadata without loading the
      save, and verify all fields are present.

13. **R-13.3.13** — The engine **SHALL** support cloud save sync with Nintendo cloud storage and
    Google Play Games cloud saves in addition to Steam, iCloud, PlayStation, and Xbox platforms.
    - **Rationale:** Nintendo Switch and Android are target platforms that require their own cloud
      save integration.
    - **Verification:** Save on a Nintendo Switch, sync to cloud, load on a second device, and
      verify state matches.
