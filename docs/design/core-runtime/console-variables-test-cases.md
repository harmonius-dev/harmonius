# Console Variables Test Cases

Companion test cases for [console-variables.md](console-variables.md).

## Unit Tests

### TC-1.16.1.1 ConVar Entry Construction

| # | Requirement |
|---|-------------|
| 1 | F-1.16.1    |
| 2 | F-1.16.2    |

1. **#1** — Build `ConVarEntry` for `r.debug.wireframe` with `Bool(false)` default, `Global` scope
   - **Expected:** Entry fields match inputs; `default == value`
2. **#2** — Name is constructed via `ConVarName::new("r.debug.wireframe")`
   - **Expected:** `stable_hash` is the FNV-1a of the literal

### TC-1.16.4.1 Registry Register Success

| # | Requirement |
|---|-------------|
| 1 | F-1.16.4    |

1. **#1** — Register one entry; call `get("r.debug.wireframe")`
   - **Expected:** `Some(ConVarValue::Bool(false))`

### TC-1.16.4.2 Registry Register Duplicate

| # | Requirement |
|---|-------------|
| 1 | F-1.16.4    |

1. **#1** — Register `r.scale`; call `register` again with the same name
   - **Expected:** `Err(ConVarError::AlreadyRegistered)`

### TC-1.16.4.3 Registry Get Unknown Name

| # | Requirement |
|---|-------------|
| 1 | F-1.16.4    |

1. **#1** — `get(ConVarName::new("unknown"))` on empty registry
   - **Expected:** `None`

### TC-1.16.1.2 Typed Getters

| # | Requirement |
|---|-------------|
| 1 | F-1.16.1    |

1. **#1** — Register `Bool`, `Int`, `Float`, `StaticStr` entries; call the typed getter for each
   - **Expected:** Each returns `Some(expected)`
2. **#2** — Call the wrong typed getter (e.g. `get_int` on a bool)
   - **Expected:** `None`

### TC-1.16.1.3 Set Mutates Current Preserves Default

| # | Requirement |
|---|-------------|
| 1 | F-1.16.1    |

1. **#1** — `set("r.scale", Float(2.0))` on entry with default `Float(1.0)`
   - **Expected:** `get` returns `Float(2.0)`; `entry.default` still `Float(1.0)`

### TC-1.16.1.4 Reset Reverts To Default

| # | Requirement |
|---|-------------|
| 1 | F-1.16.1    |

1. **#1** — Mutate `r.scale` to `Float(2.0)`, call `reset("r.scale")`
   - **Expected:** `get` returns `Float(1.0)`

### TC-1.16.1.5 Set Type Mismatch

| # | Requirement |
|---|-------------|
| 1 | F-1.16.1    |

1. **#1** — `set("r.debug.wireframe", Int(1))` (entry is Bool)
   - **Expected:** `Err(ConVarError::TypeMismatch)`; value unchanged

### TC-1.16.3.1 Read Only Flag Blocks Set

| # | Requirement |
|---|-------------|
| 1 | F-1.16.3    |

1. **#1** — Register an entry with `ConVarFlags::READ_ONLY`, call `set`
   - **Expected:** `Err(ConVarError::ReadOnly)`

### TC-1.16.3.2 Cheat Flag Requires Enable

| # | Requirement |
|---|-------------|
| 1 | F-1.16.3    |
| 2 | F-1.16.2    |

1. **#1** — Register a `Cheat`-scoped entry with `ConVarFlags::CHEAT`, call `set` while session
   cheats are disabled
   - **Expected:** `Err(ConVarError::CheatBlocked)`
2. **#2** — Call `enable_cheats`, retry `set`
   - **Expected:** `Ok(())`

### TC-1.16.8.1 Set Emits ConVarChanged Event

| # | Requirement |
|---|-------------|
| 1 | F-1.16.8    |

1. **#1** — `set("r.scale", Float(2.0))`, call `drain_events`
   - **Expected:** One `ConVarChanged` event with `old_value=Float(1.0)`, `new_value=Float(2.0)`

### TC-1.16.8.2 Reset Emits ConVarChanged Event

| # | Requirement |
|---|-------------|
| 1 | F-1.16.8    |

1. **#1** — Mutate `r.scale` to `Float(2.0)`, call `reset`, then `drain_events`
   - **Expected:** Two events; the second has `old=Float(2.0), new=Float(1.0)`

### TC-1.16.7.1 Serialize Cfg Only Archive Flag

| # | Requirement |
|---|-------------|
| 1 | F-1.16.7    |

1. **#1** — Register one entry with `ARCHIVE` and one without; call `serialize_cfg`
   - **Expected:** Output contains only the `ARCHIVE` entry

### TC-1.16.7.2 Parse Cfg Apply Values

| # | Requirement |
|---|-------------|
| 1 | F-1.16.7    |

1. **#1** — Input `r.debug.wireframe = true\nr.scale = 1.5\n`; parse
   - **Expected:** `applied == 2`; `get_bool("r.debug.wireframe") == Some(true)`;
     `get_float("r.scale") == Some(1.5)`

### TC-1.16.7.3 Parse Cfg Unknown Key

| # | Requirement |
|---|-------------|
| 1 | F-1.16.7    |

1. **#1** — Input `unknown.key = 1\nr.scale = 1.5\n`; parse
   - **Expected:** `applied == 1`; `unknown.len() == 1`; no error thrown

### TC-1.16.7.4 Parse Cfg Type Mismatch

| # | Requirement |
|---|-------------|
| 1 | F-1.16.7    |

1. **#1** — Input `r.debug.wireframe = 5` (expected Bool)
   - **Expected:** `type_mismatch.len() == 1`; entry unchanged

### TC-1.16.7.5 Parse Cfg Comments Skipped

| # | Requirement |
|---|-------------|
| 1 | F-1.16.7    |

1. **#1** — Input `# comment\nr.scale = 2.0\n`; parse
   - **Expected:** `applied == 1`; no parse errors

### TC-1.16.5.1 Survive Reload Preserves Hot Reloadable

| # | Requirement |
|---|-------------|
| 1 | F-1.16.5    |

1. **#1** — Old registry has `r.scale = 2.0` with `HOT_RELOADABLE` set; new registry default is
   `1.0`; call `survive_reload(old)`
   - **Expected:** `get_float("r.scale") == Some(2.0)`

### TC-1.16.5.2 Survive Reload Skips Non Hot Reloadable

| # | Requirement |
|---|-------------|
| 1 | F-1.16.5    |

1. **#1** — Old registry has `phys.gravity.y = -20.0` without `HOT_RELOADABLE`; call
   `survive_reload`
   - **Expected:** New value is the default `-9.81`

### TC-1.16.5.3 Survive Reload Skips Type Mismatch

| # | Requirement |
|---|-------------|
| 1 | F-1.16.5    |

1. **#1** — Old entry type `Int`, new entry type `Float`, both `HOT_RELOADABLE`
   - **Expected:** New value is the new default; change logged in diagnostics

## Integration Tests

### TC-1.16.6.1 ECS Dispatch System Forwards Events

| # | Requirement |
|---|-------------|
| 1 | F-1.16.6    |
| 2 | F-1.16.8    |

1. **#1** — Register `convar_dispatch_system`; set a ConVar; run one frame
   - **Expected:** Subscribed `EventReader<ConVarChanged>` receives one event

### TC-1.16.6.2 Res ConVarRegistry Access

| # | Requirement |
|---|-------------|
| 1 | F-1.16.6    |

1. **#1** — Read a ConVar from one system via `Res<ConVarRegistry>`, write via
   `ResMut<ConVarRegistry>` from another system in the same frame
   - **Expected:** Read observes old value; write queues change; event fires next frame

### TC-1.16.7.6 Cfg File Round Trip Via IoDispatcher

| # | Requirement |
|---|-------------|
| 1 | F-1.16.7    |
| 2 | F-1.8.1     |

1. **#1** — Mutate 3 ARCHIVE entries, serialize via `serialize_cfg`, submit `IoRequest::WriteFile`
   to `save://user.cfg`, then read back via `IoRequest::ReadFile`
   - **Expected:** Parsed entries equal mutated values byte-for-byte
2. **#2** — Drop current registry, register from scratch, apply parsed cfg
   - **Expected:** New registry contains the same 3 mutations

### TC-1.16.5.4 Survive Reload Round Trip

| # | Requirement |
|---|-------------|
| 1 | F-1.16.5    |
| 2 | F-1.11.1    |

1. **#1** — Simulate hot-reload via `HotReloadManager`: take a snapshot of the registry, produce a
   new one with fresh defaults, call `survive_reload`, emit `ConVarChanged` events
   - **Expected:** All `HOT_RELOADABLE` entries retain their mutated values; all others reset

### TC-1.16.2.1 Per World Scope Isolated

| # | Requirement |
|---|-------------|
| 1 | F-1.16.2    |

1. **#1** — Two `World` instances each holding a `ConVarRegistry` resource, mutate a `PerWorld`
   entry in one world only
   - **Expected:** Other world's value is unchanged

## Benchmarks

### TC-1.16.4.4 Get Cached Lookup Under 100 ns

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Registry with 200 entries, `get("r.scale")` | Latency | < 100 ns | R-1.16.4a |

### TC-1.16.4.5 Set Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Registry with 200 entries, `set("r.scale", Float(x))` | Latency | < 300 ns | R-1.16.4a |

### TC-1.16.7.7 Parse Cfg 200 Entries

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 200-line cfg file, full parse | Time | < 1 ms | R-1.16.7a |

### TC-1.16.7.8 Serialize Cfg 200 Entries

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Registry with 200 ARCHIVE entries | Time | < 1 ms | R-1.16.7a |
