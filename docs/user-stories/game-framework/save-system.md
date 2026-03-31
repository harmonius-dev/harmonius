# User Stories — Save System (13.3)

## Serialization

| ID          | Persona              |
|-------------|----------------------|
| US-13.3.1.1 | player (P-23)        |
| US-13.3.1.2 | game developer (P-15)|
| US-13.3.2.1 | game developer (P-15)|
| US-13.3.2.2 | server administrator (P-22)|

1. **US-13.3.1.1** — **As a** player (P-23), **I want** my character data, inventory, quest
   progress, and ability loadout saved into a single file, **so that** I resume exactly where I left
   off.
2. **US-13.3.1.2** — **As a** game developer (P-15), **I want** save serialization to capture ECS
   components via reflection without hand-written serializers, **so that** new types are saved
   automatically.
3. **US-13.3.2.1** — **As a** game developer (P-15), **I want** schema-versioned save files with
   ordered migration steps, **so that** older saves load after patches.
4. **US-13.3.2.2** — **As a** server administrator (P-22), **I want** migrations to run
   automatically on load, **so that** live-service patches do not invalidate player data.

## Save Triggers

| ID          | Persona              |
|-------------|----------------------|
| US-13.3.3.1 | game designer (P-5)  |
| US-13.3.3.2 | player (P-23)        |
| US-13.3.4.1 | player (P-23)        |
| US-13.3.4.2 | player (P-23)        |

1. **US-13.3.3.1** — **As a** game designer (P-5), **I want** automatic saves at designer-placed
   checkpoints and configurable intervals, **so that** progress is preserved without manual saves.
2. **US-13.3.3.2** — **As a** player (P-23), **I want** autosave to use a rotating slot, **so that**
   a crash mid-write does not corrupt my only save.
3. **US-13.3.4.1** — **As a** player (P-23), **I want** save slots with metadata (character name,
   level, playtime, thumbnail), **so that** I choose the right save to load.
4. **US-13.3.4.2** — **As a** player (P-23), **I want** copy, delete, and export/import of save
   files, **so that** I manage my saves.

## Cloud and Platform

| ID          | Persona              |
|-------------|----------------------|
| US-13.3.5.1 | player (P-23)        |
| US-13.3.5.2 | game developer (P-15)|
| US-13.3.6.1 | engine developer (P-26)|

1. **US-13.3.5.1** — **As a** player (P-23), **I want** cloud save sync with conflict resolution
   when local and cloud diverge, **so that** I continue on any device.
2. **US-13.3.5.2** — **As a** game developer (P-15), **I want** cloud save using each platform's
   native API fully async, **so that** sync never blocks the game.
3. **US-13.3.6.1** — **As a** engine developer (P-26), **I want** the save I/O pipeline to compress,
   encrypt, and checksum data with atomic rename, **so that** saves are always consistent.
