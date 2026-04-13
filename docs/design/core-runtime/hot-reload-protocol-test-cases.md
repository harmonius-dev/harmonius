# Hot-Reload Protocol Test Cases

Companion test cases for [hot-reload-protocol.md](hot-reload-protocol.md).

## Unit Tests

### TC-1.11.1.1 VersionEpoch Monotonic

| # | Requirement |
|---|-------------|
| 1 | F-1.11.4    |

1. **#1** — `VersionEpoch(5).next()`
   - **Expected:** `VersionEpoch(6)`

### TC-1.11.1.2 ReloadRequest rkyv Round Trip

| # | Requirement |
|---|-------------|
| 1 | F-1.11.6    |

1. **#1** — Construct `ReloadRequest { subject: MaterialGraph, ... }`, serialize, deserialize
   - **Expected:** Byte-identical after round-trip

### TC-1.11.1.3 SymbolManifest Delta Added

| # | Requirement |
|---|-------------|
| 1 | F-1.11.5    |

1. **#1** — Old manifest has 3 symbols, new has 4 (one added)
   - **Expected:** Delta contains exactly one `Added` entry

### TC-1.11.1.4 SymbolManifest Delta Removed

| # | Requirement |
|---|-------------|
| 1 | F-1.11.5    |

1. **#1** — Old manifest has 3 symbols, new has 2 (one removed)
   - **Expected:** Delta contains exactly one `Removed` entry

### TC-1.11.1.5 SymbolManifest Delta Replaced

| # | Requirement |
|---|-------------|
| 1 | F-1.11.5    |

1. **#1** — Same SymbolId, different `address`
   - **Expected:** Delta contains one `Replaced` entry

### TC-1.11.2.1 StateMigrationFn Layout Mismatch

| # | Requirement |
|---|-------------|
| 1 | F-1.11.2    |

1. **#1** — `StateMigrationFn` called with `old.len() != expected_old_layout`
   - **Expected:** Returns `Err(MigrationError::LayoutIncompatible)`

### TC-1.11.2.2 StateMigrationFn Field Rename

| # | Requirement |
|---|-------------|
| 1 | F-1.11.2    |

1. **#1** — Old struct has field `x`, new has field `pos_x`; migration renames
   - **Expected:** New buffer contains remapped value

### TC-1.11.3.1 Compile Failure Rollback

| # | Requirement |
|---|-------------|
| 1 | F-1.11.3    |
| 2 | F-1.11.3    |

1. **#1** — Submit `ReloadRequest`, simulated link failure
   - **Expected:** `ReloadResult::RolledBack`; `current_epoch()` unchanged
2. **#2** — World content after rollback
   - **Expected:** All pre-request state preserved

### TC-1.11.1.6 HotReloadManager Tick Processes Pending

| # | Requirement |
|---|-------------|
| 1 | F-1.11.1    |

1. **#1** — Submit 3 requests, call `tick_frame_boundary()`
   - **Expected:** All 3 requests processed in submission order

### TC-1.11.1.7 HotReloadManager Ignores Duplicate Epoch

| # | Requirement |
|---|-------------|
| 1 | F-1.11.4    |

1. **#1** — Submit two ReloadRequests with same `epoch` and subject
   - **Expected:** Second is `Deferred` or dropped

## Integration Tests

### TC-2.7.2.1 Material Graph Reload Swaps PSO

| # | Requirement |
|---|-------------|
| 1 | F-2.7.2     |

1. **#1** — Material graph compiled to PSO#1; author edits -> PSO#2
   - **Expected:** After swap, all bound entities use PSO#2

### TC-13.4.3.2 Logic Graph Instance Preserves State

| # | Requirement |
|---|-------------|
| 1 | F-13.4.3    |
| 2 | F-13.4.3    |

1. **#1** — LogicGraph v1 has "counter = 5"; edit to add branch; reload
   - **Expected:** Running instances retain counter = 5
2. **#2** — New branch executes on next tick
   - **Expected:** Branch runs; counter still 5

### TC-1.11.1.8 Middleman Dylib Reload End-to-End

| # | Requirement |
|---|-------------|
| 1 | F-1.11.1    |

1. **#1** — Edit component definition, trigger rustc recompile, submit reload
   - **Expected:** Middleman swap completes; new component available

### TC-12.4.1 Texture Asset Hot Reload

| # | Requirement |
|---|-------------|
| 1 | F-12.4      |

1. **#1** — Texture file modified on disk
   - **Expected:** Asset pipeline submits `ReloadRequest::TextureAsset`; GPU upload swaps next frame

### TC-7.2.4.1 Behavior Tree Reload Preserves BB

| # | Requirement |
|---|-------------|
| 1 | F-7.2.4     |

1. **#1** — BT has blackboard with 10 keys; edit tree shape; reload
   - **Expected:** Blackboard preserved; new tree executes using existing keys

## Benchmarks

### TC-1.11.1.9 Middleman Reload Under 50 ms

| # | Requirement |
|---|-------------|
| 1 | R-1.11.1a   |

1. **#1** — Drain + swap + resume without state migration
   - **Expected:** Under 50 ms on reference workstation

### TC-1.11.2.3 Migrate 1000 Instances Under 20 ms

| # | Requirement |
|---|-------------|
| 1 | R-1.11.2a   |

1. **#1** — 1000 logic-graph instances migrated via `StateMigrationFn`
   - **Expected:** Under 20 ms

### TC-1.11.6.1 Manifest Delta 10K Symbols Under 5 ms

| # | Requirement |
|---|-------------|
| 1 | R-1.11.5a   |

1. **#1** — Compute delta between two 10K-symbol manifests (one symbol changed)
   - **Expected:** Under 5 ms
