# ID Conventions Test Cases

Companion test cases for [ids.md](ids.md).

## Unit Tests

### TC-1.10.1.1 StableId Round Trip

| # | Requirement |
|---|-------------|
| 1 | F-1.10.1    |
| 2 | F-1.10.1    |

1. **#1** — `AssetId(0xCAFEBABE)` via `to_bytes()` then `from_bytes()`
   - **Expected:** Equal to original
2. **#2** — `DefinitionId(0xDEADBEEF)` via round-trip
   - **Expected:** Equal to original

### TC-1.10.1.2 StableId Namespace Discrimination

| # | Requirement |
|---|-------------|
| 1 | F-1.10.1    |

1. **#1** — `AssetId::namespace()` and `NetworkEntityId::namespace()`
   - **Expected:** `Asset` and `Network` respectively

### TC-1.10.2.1 Entity Not StableId

| # | Requirement |
|---|-------------|
| 1 | F-1.10.2    |

1. **#1** — `Entity` type does not implement `StableId`
   - **Expected:** Compile error if you try `fn take<T: StableId>(_:T)` with Entity

### TC-1.10.3.1 NetworkEntityId Distinct From Entity

| # | Requirement |
|---|-------------|
| 1 | F-1.10.3    |

1. **#1** — `let e: Entity = NetworkEntityId(5)`
   - **Expected:** Compile error (type mismatch)
2. **#2** — `NetworkEntityId(5) == NetworkEntityId(5)`
   - **Expected:** `true`

### TC-1.10.3.2 NetworkEntityMap Bind Resolve

| # | Requirement |
|---|-------------|
| 1 | F-1.10.3    |
| 2 | F-1.10.3    |

1. **#1** — `bind(NetworkEntityId(42), local_entity)`, `resolve(NetworkEntityId(42))`
   - **Expected:** `Some(local_entity)`
2. **#2** — `reverse(local_entity)`
   - **Expected:** `Some(NetworkEntityId(42))`

### TC-1.10.3.3 NetworkEntityMap Unknown Returns None

| # | Requirement |
|---|-------------|
| 1 | F-1.10.3    |

1. **#1** — `resolve(NetworkEntityId(999))` on empty map
   - **Expected:** `None`

### TC-1.10.4.1 AssetId Content Hash Deterministic

| # | Requirement |
|---|-------------|
| 1 | F-1.10.4    |

1. **#1** — Hash same 1 KiB byte sequence twice
   - **Expected:** Identical `AssetId`

### TC-1.10.4.2 AssetId rkyv Round Trip

| # | Requirement |
|---|-------------|
| 1 | F-1.10.4    |

1. **#1** — Serialize `AssetId(0x1234)` via rkyv, then deserialize
   - **Expected:** Equal to original

### TC-1.10.5.1 DefinitionId Stable Across Content Edit

| # | Requirement |
|---|-------------|
| 1 | F-1.10.5    |

1. **#1** — `DefinitionId` from slug `"quest.intro"` at version 1 and version 2 (content changed)
   - **Expected:** Identical DefinitionId

### TC-1.10.5.2 DefinitionId Changes on Rename

| # | Requirement |
|---|-------------|
| 1 | F-1.10.5    |

1. **#1** — DefinitionId from `"quest.intro"` vs `"quest.prologue"`
   - **Expected:** Different DefinitionIds

### TC-1.10.4.3 SaveSlotId UUID v4

| # | Requirement |
|---|-------------|
| 1 | F-1.10.4    |

1. **#1** — Allocate two `SaveSlotId` via `Uuid::new_v4()`
   - **Expected:** Distinct, 128-bit, version 4 nibble set

## Integration Tests

### TC-1.10.4.4 Save Round Trip Preserves StableIds

| # | Requirement |
|---|-------------|
| 1 | F-1.10.4    |

1. **#1** — Save file contains 1000 `AssetId` and `DefinitionId`, load, compare
   - **Expected:** All IDs byte-identical after round-trip

### TC-1.10.3.4 Network Spawn Populates Map

| # | Requirement |
|---|-------------|
| 1 | F-1.10.3    |

1. **#1** — Replication spawn replicates 100 entities from server
   - **Expected:** Client `NetworkEntityMap` has 100 bindings; each resolves to a valid `Entity`

### TC-1.10.3.5 Network Despawn Removes Mapping

| # | Requirement |
|---|-------------|
| 1 | F-1.10.3    |

1. **#1** — Despawn replicated entity on server
   - **Expected:** Client removes binding; `resolve` returns `None`

## Benchmarks

### TC-1.10.3.6 NetworkEntityMap 10K Lookups Under 1 ms

| # | Requirement |
|---|-------------|
| 1 | R-1.10.3a   |

1. **#1** — `NetworkEntityMap` with 10,000 bindings, run 10,000 `resolve` calls
   - **Expected:** Total time under 1 ms

### TC-1.10.4.5 AssetId Hash 1 MiB Under 5 ms

| # | Requirement |
|---|-------------|
| 1 | R-1.10.4a   |

1. **#1** — xxhash3 over 1 MiB buffer
   - **Expected:** Completes under 5 ms on reference workstation

### TC-1.10.5.3 DefinitionId Hash 10K Slugs Under 2 ms

| # | Requirement |
|---|-------------|
| 1 | R-1.10.5a   |

1. **#1** — Hash 10,000 slug strings
   - **Expected:** Under 2 ms total
