# User Stories — 13.3 Save System

## F-13.3.1 Save Game Serialization

## US-13.3.1.1 Save All Character State to a Versioned Binary File

**As a** player (P-23), **I want to** save my character data, inventory, quest progress, ability
loadout, and achievement progress into a single save file, **so that** I can resume exactly where I
left off.

## US-13.3.1.2 Serialize Dirty Fields Only for Partial Saves

**As a** developer (P-15), **I want to** the save system to serialize only modified fields using
reflection, **so that** save size and I/O cost are minimized for characters with extensive data.

## US-13.3.1.3 Use Reflection to Avoid Hand-Written Serializers

**As a** developer (P-15), **I want to** the serialization system to capture component data
automatically via the type registry, **so that** new component types are saved without writing
custom serialization code.

## US-13.3.1.4 Verify Save File Contains All Required State

**As an** engine tester (P-27), **I want to** save a character with full inventory, quest progress,
and ability loadout, then load the save and verify every field matches, **so that** no data is lost
during serialization.

## F-13.3.2 Save Data Migration and Versioning

## US-13.3.2.1 Load Saves From Previous Game Versions

**As a** player (P-23), **I want to** load save files created in earlier game versions without data
loss, **so that** patches and expansions do not invalidate my progress.

## US-13.3.2.2 Register Migration Steps for Schema Changes

**As a** developer (P-15), **I want to** register ordered migration steps (version N to N+1) that
add, remove, rename, or reshape fields, **so that** save schema evolves safely across updates.

## US-13.3.2.3 Verify Migrations Apply in Sequence

**As an** engine tester (P-27), **I want to** load a save file three major versions old and verify
all migration steps apply in order producing a valid current-version save, **so that** multi-step
migrations are reliable.

## US-13.3.2.4 Verify Migration Failure Preserves Original Save

**As an** engine tester (P-27), **I want to** introduce a failing migration step and verify the
original save file is not modified, **so that** migration errors never corrupt existing data.

## F-13.3.3 Checkpoint and Autosave

## US-13.3.3.1 Trigger Saves at Designer-Placed Checkpoints

**As a** player (P-23), **I want to** the game to save automatically at zone transitions, quest
milestones, and boss kills, **so that** I do not lose progress when unexpected events occur.

## US-13.3.3.2 Configure Autosave Intervals and Rotation

**As a** developer (P-15), **I want to** configure autosave time intervals and rotating slot count,
**so that** periodic saves protect against data loss without overwriting recent saves.

## US-13.3.3.3 Capture Instance Progress for Group Resume

**As a** developer (P-15), **I want to** checkpoint saves in instanced content to capture bosses
defeated and lockout state, **so that** groups can resume partially-completed dungeons after
disconnects.

## US-13.3.3.4 Verify Autosave Rotation Prevents Corruption

**As an** engine tester (P-27), **I want to** interrupt an autosave mid-write and verify the
previous rotating slot remains intact, **so that** interrupted writes never corrupt all save slots.

## F-13.3.4 Save Slots and Management

## US-13.3.4.1 Browse Save Slots With Metadata

**As a** player (P-23), **I want to** browse my save slots seeing character name, level, playtime,
screenshot thumbnail, timestamp, and zone name, **so that** I can identify and select the correct
save.

## US-13.3.4.2 Copy, Delete, and Export Save Files

**As a** player (P-23), **I want to** copy, delete, and export/import save files, **so that** I can
manage my saves and share them across devices.

## US-13.3.4.3 Map Save Slots to Server-Side Character Records

**As a** developer (P-15), **I want to** save slots for online games to map to server-side character
records with per-realm identity, **so that** the save system works for both offline and online play.

## US-13.3.4.4 Verify Slot Operations Are Transactional

**As an** engine tester (P-27), **I want to** interrupt a save slot copy mid-operation and verify no
partial copy exists, **so that** slot management operations are atomic and never produce corrupt
files.

## F-13.3.5 Cloud Save with Platform APIs

## US-13.3.5.1 Sync Saves to Platform Cloud Storage

**As a** player (P-23), **I want to** my save data to sync automatically with my platform's cloud
storage (Steam, PlayStation, Xbox, iCloud), **so that** I can continue playing on a different
device.

## US-13.3.5.2 Resolve Cloud Save Conflicts With User Prompt

**As a** player (P-23), **I want to** be prompted to choose between local and cloud saves when they
diverge, **so that** I never lose progress due to silent conflict resolution.

## US-13.3.5.3 Implement Platform-Specific Cloud Storage Integration

**As a** developer (P-15), **I want to** integrate with each platform's native cloud save API using
fully async operations, **so that** cloud sync never blocks the game thread.

## US-13.3.5.4 Verify Cloud Sync Completes Without Blocking

**As an** engine tester (P-27), **I want to** trigger a cloud upload during gameplay and verify the
game thread is never blocked, **so that** cloud operations are truly asynchronous.

## F-13.3.6 Async Save I/O Pipeline

## US-13.3.6.1 Save and Load Without Frame Drops

**As a** player (P-23), **I want to** save and load operations to complete without visible frame
drops, **so that** saving never interrupts gameplay.

## US-13.3.6.2 Compress, Encrypt, and Checksum Save Data

**As a** developer (P-15), **I want to** the save pipeline to compress (LZ4/Zstd), encrypt
(AES-256-GCM), and checksum (CRC-32) all save data automatically, **so that** saves are compact,
tamper-resistant, and corruption-detectable.

## US-13.3.6.3 Use Atomic Rename for Crash-Safe Writes

**As a** developer (P-15), **I want to** save writes to use atomic rename so files are always
consistent even if the process crashes mid-write, **so that** save corruption is impossible under
normal failure scenarios.

## US-13.3.6.4 Verify Save Integrity After Simulated Crash

**As an** engine tester (P-27), **I want to** kill the process during a save write and verify the
previous save file is still intact, **so that** atomic rename provides crash safety.
