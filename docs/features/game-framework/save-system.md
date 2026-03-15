# 13.3 — Save System

## Serialization

### F-13.3.1 Save Game Serialization

Serializes the player's game state (character data, inventory, quest progress, ability loadout, map
exploration, achievement progress) into a versioned binary format. Uses the engine's reflection and
serialization systems to capture component data without hand-written serializers for each type.
Supports partial serialization where only dirty fields are written, reducing save size and I/O cost
for MMO characters with extensive inventories and progression data.

- **Requirements:** R-13.3.1
- **Dependencies:** F-1.4.1 (Binary Serialization), F-1.3.1 (Type Registry)
- **Platform notes:** None

### F-13.3.2 Save Data Migration and Versioning

Stamps each save file with a schema version and applies migration transforms when loading older
saves. Migrations are registered as ordered steps (version N to N+1) that add, remove, rename, or
reshape fields. The migration pipeline runs automatically on load, allowing live-service MMO patches
to evolve save schemas without invalidating existing player data across expansions and content
updates.

- **Requirements:** R-13.3.2
- **Dependencies:** F-13.3.1, F-1.3.3 (Property System)
- **Platform notes:** None

## Save Triggers

### F-13.3.3 Checkpoint and Autosave

Triggers automatic saves at designer-placed checkpoints (zone transitions, quest milestones, boss
kills) and at configurable time intervals. Autosave writes to a rotating slot to prevent corruption
from interrupted writes. For instanced content, checkpoint saves capture instance progress (bosses
defeated, lockout state) so groups can resume partially-completed dungeons and raids after
disconnects.

- **Requirements:** R-13.3.3
- **Dependencies:** F-13.3.1, F-13.1.1 (Game Mode State Machine)
- **Platform notes:** None

### F-13.3.4 Save Slots and Management

Provides a save slot system with metadata (character name, level, playtime, screenshot thumbnail,
timestamp, zone name) for UI display. Supports copy, delete, and export/import of save files. For
MMO, slots map to server-side character records with per-realm identity. Slot management operations
are transactional to prevent data loss from concurrent access or interrupted writes.

- **Requirements:** R-13.3.4
- **Dependencies:** F-13.3.1
- **Platform notes:** None

## Cloud and Platform Integration

### F-13.3.5 Cloud Save with Platform APIs

Synchronizes save data with platform cloud storage (Steam Cloud, PlayStation Plus Cloud, Xbox Cloud
Saves, iCloud, Epic Online Services) using each platform's native API. Handles conflict resolution
when local and cloud saves diverge (last-write-wins with user prompt for manual resolution). Upload
and download operations are fully async and never block the game thread. For MMO, cloud save
coordinates with the authoritative server database to reconcile offline progression.

- **Requirements:** R-13.3.5
- **Dependencies:** F-13.3.4, F-14.5.5 (Platform Cloud Storage)
- **Platform notes:** Each platform requires its own SDK integration. Uses platform-native async I/O
  (IOCP on Windows, GCD on macOS, io_uring on Linux) for file transfers.

### F-13.3.6 Async Save I/O Pipeline

All save reads and writes flow through an async I/O pipeline that compresses (LZ4 for speed, Zstd
for size on cloud uploads), encrypts (AES-256-GCM to prevent tampering), and checksums (CRC-32 for
corruption detection) save data. Write operations use atomic rename to guarantee that a save file is
always in a consistent state even if the process crashes mid-write. The pipeline supports priority
ordering so autosaves yield to explicit user saves.

- **Requirements:** R-13.3.6
- **Dependencies:** F-13.3.1
- **Platform notes:** Uses platform-native async I/O (IOCP on Windows, GCD on macOS, io_uring on
  Linux) per project guidelines. No standard library file I/O.
