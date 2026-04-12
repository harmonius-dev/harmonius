# 13.3 — Save System

## Serialization

| ID       | Feature                            |
|----------|------------------------------------|
| F-13.3.1 | Save Game Serialization            |
| F-13.3.2 | Save Data Migration and Versioning |

1. **F-13.3.1** — Serializes the player's game state (character data, inventory, quest progress,
   ability loadout, map exploration, achievement progress) into a versioned binary format. Uses the
   engine's reflection and serialization systems to capture component data without hand-written
   serializers for each type. Supports partial serialization where only dirty fields are written,
   reducing save size and I/O cost for MMO characters with extensive inventories and progression
   data.
   - **Deps:** F-1.4.1 (Binary Serialization), F-1.3.1 (Type Registry)
2. **F-13.3.2** — Stamps each save file with a schema version and applies migration transforms when
   loading older saves. Migrations are registered as ordered steps (version N to N+1) that add,
   remove, rename, or reshape fields. The migration pipeline runs automatically on load, allowing
   live-service MMO patches to evolve save schemas without invalidating existing player data across
   expansions and content updates.
   - **Deps:** F-13.3.1, F-1.3.3 (Property System)

## Save Triggers

| ID       | Feature                   |
|----------|---------------------------|
| F-13.3.3 | Checkpoint and Autosave   |
| F-13.3.4 | Save Slots and Management |

1. **F-13.3.3** — Triggers automatic saves at designer-placed checkpoints (zone transitions, quest
   milestones, boss kills) and at configurable time intervals. Autosave writes to a rotating slot to
   prevent corruption from interrupted writes. For instanced content, checkpoint saves capture
   instance progress (bosses defeated, lockout state) so groups can resume partially-completed
   dungeons and raids after disconnects.
   - **Deps:** F-13.3.1, F-13.1.1 (Game Mode State Machine)
2. **F-13.3.4** — Provides a save slot system with metadata (character name, level, playtime,
   screenshot thumbnail, timestamp, zone name) for UI display. Supports copy, delete, and
   export/import of save files. For MMO, slots map to server-side character records with per-realm
   identity. Slot management operations are transactional to prevent data loss from concurrent
   access or interrupted writes.
   - **Deps:** F-13.3.1

## Cloud and Platform Integration

| ID       | Feature                       |
|----------|-------------------------------|
| F-13.3.5 | Cloud Save with Platform APIs |
| F-13.3.6 | Async Save I/O Pipeline       |

1. **F-13.3.5** — Synchronizes save data with platform cloud storage (Steam Cloud, PlayStation Plus
   Cloud, Xbox Cloud Saves, iCloud, Epic Online Services) using each platform's native API. Handles
   conflict resolution when local and cloud saves diverge (last-write-wins with user prompt for
   manual resolution). Upload and download operations are fully async and never block the game
   thread. For MMO, cloud save coordinates with the authoritative server database to reconcile
   offline progression.
   - **Deps:** F-13.3.4, F-14.5.5 (Platform Cloud Storage)
   - **Platform:** Each platform requires its own SDK integration. Uses Tokio async I/O for file
     transfers.
2. **F-13.3.6** — All save reads and writes flow through an async I/O pipeline that compresses (LZ4
   for speed, Zstd for size on cloud uploads), encrypts (AES-256-GCM to prevent tampering), and
   checksums (CRC-32 for corruption detection) save data. Write operations use atomic rename to
   guarantee that a save file is always in a consistent state even if the process crashes mid-write.
   The pipeline supports priority ordering so autosaves yield to explicit user saves.
   - **Deps:** F-13.3.1
   - **Platform:** Uses Tokio async I/O per project guidelines. No standard library file I/O.

## Advanced Save Features

| ID       | Feature                                             |
|----------|-----------------------------------------------------|
| F-13.3.7 | Procedural Asset Blob Snapshots in Save Archives    |
| F-13.3.8 | Per-Component Schema Versioning with Migrations     |
| F-13.3.9 | Save Context Filtering for Entity Subsets           |
| F-13.3.10 | Typed Save Lifecycle Events                        |

1. **F-13.3.7** — Snapshots procedural asset blobs (terrain modifications, runtime-generated
   textures, voxel edits) alongside entity component state in the save archive. Procedural blobs are
   stored as offset-referenced entries in the archive, loaded on demand when the owning entity is
   deserialized. Blob data is compressed independently for efficient partial loading.
   - **Deps:** F-13.3.1, F-13.3.6 (Async I/O Pipeline)
2. **F-13.3.8** — Tracks schema versions per-component type in the save header rather than a single
   global version. On load, only components whose schema version differs from the current version
   are migrated. Supports advanced migration transforms: split component, merge components, reparent
   entity, create/delete entity, cross-component field move, and data table rekeying. Migrations are
   registered as ordered transform lists per component type.
   - **Deps:** F-13.3.2, F-1.3.3 (Property System)
3. **F-13.3.9** — Filters which entities are serialized based on save context tags (character,
   world, instance, settings). Each entity is tagged with one or more contexts. A save operation
   specifies which contexts to include, enabling separate save files for character data vs. world
   state. Context tags are assigned via the inspector or logic graphs.
   - **Deps:** F-13.3.1, F-1.1.1 (ECS)
4. **F-13.3.10** — Emits typed ECS events for all save lifecycle stages: SaveStarted, SaveComplete,
   SaveFailed, LoadStarted, LoadComplete, LoadFailed, CloudSyncStarted, CloudSyncComplete, and
   CloudConflict. Events carry metadata (slot ID, context, duration). Gameplay systems and UI
   subscribe to these events for loading screens, progress bars, error dialogs, and conflict
   resolution flows.
   - **Deps:** F-13.3.1, F-1.5.1 (Typed Event Channels)
