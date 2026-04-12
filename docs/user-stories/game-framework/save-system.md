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

## Save Context Filtering

| ID          | Persona              |
|-------------|----------------------|
| US-13.3.7.1 | game developer (P-15)|

1. **US-13.3.7.1** — **As a** game developer (P-15), **I want** to tag entities with save contexts
   (character, world, instance, settings) so that I can selectively serialize entity subsets,
   **so that** character data saves separately from world state and can be carried across maps
   independently.

## Rich Save Slot Metadata

| ID          | Persona       |
|-------------|---------------|
| US-13.3.8.1 | player (P-23) |

1. **US-13.3.8.1** — **As a** player (P-23), **I want** save slots to display rich metadata
   including character class, party composition, quest progress summary, death count, currency,
   completion percentage, and difficulty, **so that** I can choose the right save to load without
   opening each one.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-13.3.1 | player (P-23) |
| US-13.3.2 | game developer (P-15) |
| US-13.3.3 | game designer (P-5) |
| US-13.3.4 | player (P-23) |
| US-13.3.5 | player (P-23) |
| US-13.3.6 | engine developer (P-26) |
| US-13.3.7 | game developer (P-15) |
| US-13.3.8 | player (P-23) |

1. **US-13.3.1** -- **As a** player (P-23), **I want** the capabilities defined in sub-stories
   US-13.3.1.1 through US-13.3.1.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

2. **US-13.3.2** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-13.3.2.1 through US-13.3.2.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

3. **US-13.3.3** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-13.3.3.1 through US-13.3.3.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

4. **US-13.3.4** -- **As a** player (P-23), **I want** the capabilities defined in sub-stories
   US-13.3.4.1 through US-13.3.4.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

5. **US-13.3.5** -- **As a** player (P-23), **I want** the capabilities defined in sub-stories
   US-13.3.5.1 through US-13.3.5.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

6. **US-13.3.6** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-13.3.6.1 through US-13.3.6.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

7. **US-13.3.7** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-13.3.7.1 through US-13.3.7.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

8. **US-13.3.8** -- **As a** player (P-23), **I want** the capabilities defined in sub-stories
   US-13.3.8.1 through US-13.3.8.1 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.
