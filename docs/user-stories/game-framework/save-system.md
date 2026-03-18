# User Stories — 13.3 Save System

## F-13.3.1 Save Game Serialization

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-13.3.1.1 | player (P-23)        | F-13.3.1 | R-13.3.1     |
| US-13.3.1.2 | developer (P-15)     | F-13.3.1 | R-13.3.1     |
| US-13.3.1.3 | developer (P-15)     | F-13.3.1 | R-13.3.1     |
| US-13.3.1.4 | engine tester (P-27) | F-13.3.1 | R-13.3.1     |

1. **US-13.3.1.1** — I want to save my character data, inventory, quest progress, ability loadout,
   and achievement progress into a single save file so that I can resume exactly where I left off
2. **US-13.3.1.2** — I want to the save system to serialize only modified fields using reflection so
   that save size and I/O cost are minimized for characters with extensive data
3. **US-13.3.1.3** — I want to the serialization system to capture component data automatically via
   the type registry so that new component types are saved without writing custom serialization code
4. **US-13.3.1.4** — I want to save a character with full inventory, quest progress, and ability
   loadout, then load the save and verify every field matches so that no data is lost during
   serialization

## F-13.3.2 Save Data Migration and Versioning

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-13.3.2.1 | player (P-23)        | F-13.3.2 | R-13.3.2     |
| US-13.3.2.2 | developer (P-15)     | F-13.3.2 | R-13.3.2     |
| US-13.3.2.3 | engine tester (P-27) | F-13.3.2 | R-13.3.2     |
| US-13.3.2.4 | engine tester (P-27) | F-13.3.2 | R-13.3.2     |

1. **US-13.3.2.1** — I want to load save files created in earlier game versions without data loss so
   that patches and expansions do not invalidate my progress
2. **US-13.3.2.2** — I want to register ordered migration steps (version N to N+1) that add, remove,
   rename, or reshape fields so that save schema evolves safely across updates
3. **US-13.3.2.3** — I want to load a save file three major versions old and verify all migration
   steps apply in order producing a valid current-version save so that multi-step migrations are
   reliable
4. **US-13.3.2.4** — I want to introduce a failing migration step and verify the original save file
   is not modified so that migration errors never corrupt existing data

## F-13.3.3 Checkpoint and Autosave

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-13.3.3.1 | player (P-23)        | F-13.3.3 | R-13.3.3     |
| US-13.3.3.2 | developer (P-15)     | F-13.3.3 | R-13.3.3     |
| US-13.3.3.3 | developer (P-15)     | F-13.3.3 | R-13.3.3     |
| US-13.3.3.4 | engine tester (P-27) | F-13.3.3 | R-13.3.3     |

1. **US-13.3.3.1** — I want to the game to save automatically at zone transitions, quest milestones,
   and boss kills so that I do not lose progress when unexpected events occur
2. **US-13.3.3.2** — I want to configure autosave time intervals and rotating slot count so that
   periodic saves protect against data loss without overwriting recent saves
3. **US-13.3.3.3** — I want to checkpoint saves in instanced content to capture bosses defeated and
   lockout state so that groups can resume partially-completed dungeons after disconnects
4. **US-13.3.3.4** — I want to interrupt an autosave mid-write and verify the previous rotating slot
   remains intact so that interrupted writes never corrupt all save slots

## F-13.3.4 Save Slots and Management

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-13.3.4.1 | player (P-23)        | F-13.3.4 | R-13.3.4     |
| US-13.3.4.2 | player (P-23)        | F-13.3.4 | R-13.3.4     |
| US-13.3.4.3 | developer (P-15)     | F-13.3.4 | R-13.3.4     |
| US-13.3.4.4 | engine tester (P-27) | F-13.3.4 | R-13.3.4     |

1. **US-13.3.4.1** — I want to browse my save slots seeing character name, level, playtime,
   screenshot thumbnail, timestamp, and zone name so that I can identify and select the correct save
2. **US-13.3.4.2** — I want to copy, delete, and export/import save files so that I can manage my
   saves and share them across devices
3. **US-13.3.4.3** — I want to save slots for online games to map to server-side character records
   with per-realm identity so that the save system works for both offline and online play
4. **US-13.3.4.4** — I want to interrupt a save slot copy mid-operation and verify no partial copy
   exists so that slot management operations are atomic and never produce corrupt files

## F-13.3.5 Cloud Save with Platform APIs

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-13.3.5.1 | player (P-23)        | F-13.3.5 | R-13.3.5     |
| US-13.3.5.2 | player (P-23)        | F-13.3.5 | R-13.3.5     |
| US-13.3.5.3 | developer (P-15)     | F-13.3.5 | R-13.3.5     |
| US-13.3.5.4 | engine tester (P-27) | F-13.3.5 | R-13.3.5     |

1. **US-13.3.5.1** — I want to my save data to sync automatically with my platform's cloud storage
   (Steam, PlayStation, Xbox, iCloud) so that I can continue playing on a different device
2. **US-13.3.5.2** — I want to be prompted to choose between local and cloud saves when they diverge
   so that I never lose progress due to silent conflict resolution
3. **US-13.3.5.3** — I want to integrate with each platform's native cloud save API using fully
   async operations so that cloud sync never blocks the game thread
4. **US-13.3.5.4** — I want to trigger a cloud upload during gameplay and verify the game thread is
   never blocked so that cloud operations are truly asynchronous

## F-13.3.6 Async Save I/O Pipeline

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-13.3.6.1 | player (P-23)        | F-13.3.6 | R-13.3.6     |
| US-13.3.6.2 | developer (P-15)     | F-13.3.6 | R-13.3.6     |
| US-13.3.6.3 | developer (P-15)     | F-13.3.6 | R-13.3.6     |
| US-13.3.6.4 | engine tester (P-27) | F-13.3.6 | R-13.3.6     |

1. **US-13.3.6.1** — I want to save and load operations to complete without visible frame drops so
   that saving never interrupts gameplay
2. **US-13.3.6.2** — I want to the save pipeline to compress (LZ4/Zstd), encrypt (AES-256-GCM), and
   checksum (CRC-32) all save data automatically so that saves are compact, tamper-resistant, and
   corruption-detectable
3. **US-13.3.6.3** — I want to save writes to use atomic rename so files are always consistent even
   if the process crashes mid-write so that save corruption is impossible under normal failure
   scenarios
4. **US-13.3.6.4** — I want to kill the process during a save write and verify the previous save
   file is still intact so that atomic rename provides crash safety
