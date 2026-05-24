# World Management

Level loading, streaming, entity lifecycle, and environmental state.

## What it covers

- Level loading: bringing worlds into memory.
- Streaming: loading and unloading chunks based on player position.
- Zones and regions: organizing world into manageable pieces.
- Entity spawn and despawn: creating and destroying game objects.
- Respawning: replicating players and enemies after death.
- Environmental state: weather, lighting, time of day.
- World events: global triggers affecting all players.
- Dynamic loading: loading assets on-demand during gameplay.
- Memory management: balancing loaded content vs available memory.
- Procedural generation integration: generating levels or content.

## Concepts

### Level and Streaming

Levels load entirely (small levels) or stream in chunks (large worlds). Streaming loads chunks near
the player and unloads distant chunks. Each chunk is independently loadable: textures, meshes, NPCs,
spawn points. Streaming is asynchronous: load next chunk while playing current chunk. Transitions
between chunks are seamless (no loading screens).

### Zones and Regions

Worlds organize into regions: forest, cave, town. Each region has unique mood and rules. Regions can
stream independently: fast travel transitions between regions (may show loading screen). Dungeons are
self-contained regions: entering loads interior, leaving unloads.

### Entity Lifecycle

Entities spawn at specified positions; despawn when destroyed or out-of-range. Respawning revives
players at checkpoints or spawn points; enemies respawn in waves or continuously. Entity lifecycle
syncs across network: server tells clients when entities spawn/despawn.

### Environmental State

Environmental state includes weather (rain, snow), lighting (day/night cycle, weather modulation),
and atmospheric effects (fog, haze). These parameters drive visual presentation and gameplay (poor
visibility in fog). World events affect all players: meteor strike, eclipse, earthquake. World
events can lock progression (bridge collapses) or create opportunities.

### Dynamic Loading

Assets (meshes, textures, audio) load as needed. Streaming prioritizes nearby assets. Unused assets
unload to free memory. References from active entities prevent unloading (entity can't render if
mesh unloads). This balances visual fidelity vs memory footprint.

## How it fits

- See [save-and-persistence.md](./save-and-persistence.md) for saving world state.
- See [authoring-and-scripting.md](./authoring-and-scripting.md) for world event scripting.
- See [../content-pipeline/streaming-and-hot-reload.md](../content-pipeline/streaming-and-hot-reload.md)
  for asset streaming.
