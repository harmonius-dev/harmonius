# Simulation

Grids, spatial awareness, timelines, and event recording.

## Topics

- [grids-and-volumes](./grids-and-volumes.md) — voxel grids, volume queries, and spatial
  partitioning.
- [spatial-awareness](./spatial-awareness.md) — what agents can see, remember, and share.
- [timelines](./timelines.md) — timed sequences, cues, and cinematic events.
- [event-logs](./event-logs.md) — recording what happened for queries and replays.

## Key takeaways

- Voxel grids represent 3D occupancy; sparse voxel octrees reduce memory for large worlds; dense
  grids support fast local queries (what's at this position).
- Spatial partitioning (BVH, octree, grid) accelerates range queries (what's near this point) from
  O(N) to O(log N).
- Visibility grids and memory decay enable agents to remember past events while forgetting if not
  reinforced.
- Timelines schedule events at specific frame times enabling cinematics and synchronized events
  without update logic.
- Event logs record all gameplay events (damage dealt, quest complete, NPC died) enabling replays,
  analytics, and undo/redo.

## Integration risks

- Voxel grid resolution (cell size) directly impacts memory and query accuracy; too coarse loses
  detail, too fine uses excessive memory. See [grids-and-volumes.md](./grids-and-volumes.md)
  for resolution tuning.
- Spatial query radius misconfiguration (AI perception range too small or too large) affects
  behavior believability; tuning necessary per game. See [spatial-awareness.md](./spatial-awareness.md)
  for calibration guidance.
- Event log growth (unbounded recording) consumes memory; archival and garbage collection necessary.
  See [event-logs.md](./event-logs.md) for retention policies.
- Timeline event scheduling with frame rate variations causes timing drift; timestamp-based instead
  of frame-count scheduling necessary. See [timelines.md](./timelines.md) for sync strategy.
- Visibility memory decay rates must match narrative pacing; decay too fast causes AI to forget too
  quickly, decay too slow causes unrealistic memory. See [spatial-awareness.md](./spatial-awareness.md)
  for decay tuning.
