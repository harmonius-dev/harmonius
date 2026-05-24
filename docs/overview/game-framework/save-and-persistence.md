# Save and Persistence

Player progression, world state serialization, and checkpoint systems.

## What it covers

- Save files: storing player progress and world state.
- Autosave: periodic or event-triggered automatic saves.
- Checkpoints: mid-mission save points.
- Player inventory: items carried, equipment, currency.
- Quest state: progress toward quest objectives.
- World state: changed environment, defeated enemies, unlocked doors.
- Cosmetics: character skins, weapon variants, emotes.
- Settings and preferences: control bindings, graphics options.
- Cloud save: synchronizing saves across devices.
- Corrupt save recovery: fallback to last-known-good save.

## Concepts

### Save File Structure

Save files store snapshots of game state: player position, health, inventory, quest progress, world
state. Saves serialize to binary (efficiency) or JSON (debuggability). Version numbers enable
loading old saves in new game versions (backward compatibility). Compression reduces file size.
Encryption protects against tampering on multiplayer games.

### Autosave and Checkpoints

Autosaves trigger on intervals (every 5 minutes) or events (level complete, entering new area).
Autosaves are ephemeral: limited number kept (last 5 autosaves), older ones overwrite. Checkpoints
save mid-mission: reaching a checkpoint loads if mission restarted. Checkpoints are finer-grained
than full saves.

### Player Progression

Progress tracks include experience, level, skills unlocked, cosmetics obtained. Save files store
progression; it persists across sessions. Progression gates content: level 10 enemies unlock at
level 10; legendary skins available after 100 wins. Progression systems motivate continued play.

### Cloud Saves and Sync

Cloud saves synchronize across devices: save on PC, load on mobile. Conflict resolution (last-write-wins)
handles simultaneous saves. Cloud saves enable seamless session continuity: quit on PC, resume on
mobile. Offline saves work locally, sync when reconnected.

### Recovery and Validation

Save validation detects corruption (incomplete write, file truncation). If save is corrupt, rollback
to previous save. Impossible state detection: player position outside world, invalid item ID. On
detection, error log contains details; player prompted to restore last valid save or start fresh.

## How it fits

- See [authoring-and-scripting.md](./authoring-and-scripting.md) for script-driven save events.
- See [world-management.md](./world-management.md) for streaming and world state.
- See [../core-runtime/data-and-io.md](../core-runtime/data-and-io.md) for serialization.
