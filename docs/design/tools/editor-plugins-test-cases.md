# Editor Plugin Architecture Test Cases

Companion test cases for [editor-plugins.md](editor-plugins.md).

## Unit Tests

### TC-15.1.8.1 Plugin Discovery Scans Directories

| # | Requirement |
|---|-------------|
| 1 | R-1.6.1     |
| 2 | R-1.6.1     |
| 3 | R-1.6.1     |

1. **#1** — Search path with 3 `.dylib` files
   - **Expected:** `discover()` returns 3 `PluginMetadata` entries
2. **#2** — Empty search path
   - **Expected:** `discover()` returns empty `Vec`
3. **#3** — Path with non-plugin files
   - **Expected:** `discover()` returns only valid plugin entries

### TC-15.1.8.2 Plugin Load Creates Handle

| # | Requirement |
|---|-------------|
| 1 | R-1.6.2     |
| 2 | R-1.6.2     |
| 3 | R-1.6.2     |

1. **#1** — `load(valid_plugin_path, api)`
   - **Expected:** Returns `Ok(PluginHandle)` with `state == Active`
2. **#2** — `load(nonexistent_path, api)`
   - **Expected:** Returns `Err(PluginError::LoadFailed)`
3. **#3** — `load(invalid_dylib, api)`
   - **Expected:** Returns `Err(PluginError::MissingEntryPoint)`

### TC-15.1.8.3 Plugin Unload Removes Registrations

| # | Requirement |
|---|-------------|
| 1 | R-1.6.2     |
| 2 | R-1.6.2     |
| 3 | R-1.6.2     |

1. **#1** — Load plugin that registers 1 panel, `unload()`
   - **Expected:** Panel no longer in `PanelRegistry`
2. **#2** — Load plugin that registers 1 gizmo, `unload()`
   - **Expected:** Gizmo no longer in `GizmoManager`
3. **#3** — Load plugin that registers 1 widget, `unload()`
   - **Expected:** Widget no longer in `EditorWidgetRegistry`

### TC-15.1.8.4 Dependency Resolution Topological Sort

| # | Requirement |
|---|-------------|
| 1 | R-1.6.5     |
| 2 | R-1.6.5     |
| 3 | R-1.6.5     |

1. **#1** — A depends on B, B depends on C
   - **Expected:** Load order: `[C, B, A]`
2. **#2** — A and B independent
   - **Expected:** Load order contains both (either order)
3. **#3** — A depends on B, B depends on A
   - **Expected:** `Err(DependencyError::CircularDependency)`

### TC-15.1.8.5 Dependency Resolution Missing Dependency

| # | Requirement |
|---|-------------|
| 1 | R-1.6.5     |

1. **#1** — A depends on B, B not available
   - **Expected:** `Err(DependencyError::MissingDependency { plugin: "A", missing: "B" })`

### TC-15.1.8.6 Version Compatibility Check

| # | Requirement |
|---|-------------|
| 1 | R-1.6.6     |
| 2 | R-1.6.6     |
| 3 | R-1.6.6     |

1. **#1** — Plugin `min_host_version: 1.0.0`, host `1.2.0`
   - **Expected:** `Ok(())` — compatible
2. **#2** — Plugin `min_host_version: 2.0.0`, host `1.2.0`
   - **Expected:** `Err(DependencyError::HostVersionIncompatible)`
3. **#3** — Plugin depends on B `>=1.0`, B version `0.9.0`
   - **Expected:** `Err(DependencyError::VersionIncompatible)`

### TC-15.1.8.7 Plugin Isolation Catches Panic

| # | Requirement |
|---|-------------|
| 1 | R-1.6.4     |
| 2 | R-1.6.4     |

1. **#1** — Plugin `register_editor()` panics
   - **Expected:** `PluginHandle.state == Faulted(msg)`, editor continues running
2. **#2** — Plugin `on_unload()` panics
   - **Expected:** Unload completes, error logged, editor unaffected

### TC-15.1.8.8 Plugin Allocator Tracking

| # | Requirement |
|---|-------------|
| 1 | R-1.6.4     |
| 2 | R-1.6.4     |
| 3 | R-1.6.4     |

1. **#1** — Plugin allocates 1 MB
   - **Expected:** `allocator.allocated_bytes() >= 1_000_000`
2. **#2** — Plugin unloaded
   - **Expected:** `allocator.allocated_bytes() == 0` after cleanup
3. **#3** — Check peak after load
   - **Expected:** `allocator.peak_bytes() >= allocated_bytes()`

### TC-15.1.8.9 Hot-Reload Preserves State

| # | Requirement |
|---|-------------|
| 1 | R-1.6.3     |
| 2 | R-1.6.3     |

1. **#1** — Plugin with state `{counter: 42}`, `hot_reload()`
   - **Expected:** After reload, `counter == 42`
2. **#2** — Plugin with empty state, `hot_reload()`
   - **Expected:** Reload succeeds, plugin active

### TC-15.1.8.10 Hot-Reload Re-Registers UI

| # | Requirement |
|---|-------------|
| 1 | R-1.6.3     |
| 2 | R-1.6.3     |

1. **#1** — Plugin registers panel, `hot_reload()`
   - **Expected:** Panel still in `PanelRegistry` after reload
2. **#2** — Plugin registers widget, `hot_reload()`
   - **Expected:** Widget still in `EditorWidgetRegistry`

### TC-15.1.8.11 Hot-Reload Deserialize Failure

| # | Requirement |
|---|-------------|
| 1 | R-1.6.3     |

1. **#1** — Plugin state format changed, `hot_reload()`
   - **Expected:** `Err(PluginError::DeserializeFailed)`, old version stays loaded

### TC-15.1.8.12 Widget Registry Register and Lookup

| # | Requirement |
|---|-------------|
| 1 | R-1.6.7     |
| 2 | R-1.6.7     |
| 3 | R-1.6.7     |

1. **#1** — `register(TypeA, factory_a)`, `lookup(TypeA)`
   - **Expected:** Returns `Some(&factory_a)`
2. **#2** — `lookup(TypeB)` without registration
   - **Expected:** Returns `None`
3. **#3** — `register(TypeA, factory_a)`, `unregister(TypeA)`, `lookup(TypeA)`
   - **Expected:** Returns `None`

### TC-15.1.8.13 Widget Registry Override

| # | Requirement |
|---|-------------|
| 1 | R-1.6.7     |

1. **#1** — `register(TypeA, factory_1)`, `register(TypeA, factory_2)`
   - **Expected:** `lookup(TypeA)` returns `factory_2`

### TC-15.1.8.14 Editor Plugin API Register Panel

| # | Requirement |
|---|-------------|
| 1 | R-15.1.8    |
| 2 | R-15.1.8    |

1. **#1** — `api.register_panel(descriptor)`
   - **Expected:** `PanelRegistry` contains the panel
2. **#2** — `api.unregister_panel(id)`
   - **Expected:** Panel removed from `PanelRegistry`

### TC-15.1.8.15 Editor Plugin API Register Gizmo

| # | Requirement |
|---|-------------|
| 1 | R-15.1.8    |

1. **#1** — `api.register_gizmo(descriptor)`
   - **Expected:** Gizmo appears in `GizmoManager` custom list

### TC-15.1.8.16 Editor Plugin API Register Hotkey

| # | Requirement |
|---|-------------|
| 1 | R-15.1.8    |

1. **#1** — `api.register_hotkey(Ctrl+T, "my_action")`
   - **Expected:** `HotKeyMap.lookup(Ctrl+T) == "my_action"`

### TC-15.1.8.17 Plugin Get By Name

| # | Requirement |
|---|-------------|
| 1 | R-1.6.1     |
| 2 | R-1.6.1     |

1. **#1** — Load plugin "foo", `get_by_name("foo")`
   - **Expected:** Returns `Some(handle)` with `name == "foo"`
2. **#2** — `get_by_name("nonexistent")`
   - **Expected:** Returns `None`

### TC-15.1.8.18 Plugin Metadata Extraction

| # | Requirement |
|---|-------------|
| 1 | R-1.6.6     |

1. **#1** — `harmonius_plugin_metadata(ptr)`
   - **Expected:** Returns metadata with correct name, version, dependencies

### TC-15.1.8.19 C ABI Entry Point Validation

| # | Requirement |
|---|-------------|
| 1 | R-1.6.6     |
| 2 | R-1.6.6     |

1. **#1** — dylib with `harmonius_plugin_create` symbol
   - **Expected:** `load()` succeeds
2. **#2** — dylib without `harmonius_plugin_create`
   - **Expected:** `Err(PluginError::MissingEntryPoint)`

### TC-15.1.8.20 Plugin State Serialization Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-1.6.3     |

1. **#1** — `serialize_state()` then `deserialize_state(bytes)`
   - **Expected:** Plugin state identical before and after

### TC-15.1.8.21 File Watch Detects Change

| # | Requirement |
|---|-------------|
| 1 | R-1.6.3     |
| 2 | R-1.6.3     |

1. **#1** — Modify plugin `.dylib` on disk, `poll_changes()`
   - **Expected:** Returns handle of changed plugin
2. **#2** — No file changes, `poll_changes()`
   - **Expected:** Returns empty `Vec`

### TC-15.1.8.22 Multiple Plugins Coexist

| # | Requirement |
|---|-------------|
| 1 | R-1.6.2     |
| 2 | R-1.6.2     |

1. **#1** — Load plugin A and B, both register panels
   - **Expected:** Both panels in `PanelRegistry`
2. **#2** — Unload plugin A
   - **Expected:** Plugin A's panel removed, B's panel remains

## Integration Tests

### TC-15.1.8.30 End-to-End Plugin Lifecycle

| # | Requirement      |
|---|------------------|
| 1 | R-1.6.2, R-1.6.3 |

1. **#1** — `discover()`, `load()`, verify active, `hot_reload()`, verify active, `unload()`
   - **Expected:** Plugin transitions through all states correctly

### TC-15.1.8.31 Component Editor Dispatch via Plugin

| # | Requirement |
|---|-------------|
| 1 | R-1.6.7     |
| 2 | R-1.6.7     |

1. **#1** — Plugin registers widget for `HealthComponent`, select entity with `HealthComponent`
   - **Expected:** Inspector uses custom widget, not auto-generated
2. **#2** — Unload plugin, reselect entity
   - **Expected:** Inspector falls back to auto-generated widget

### TC-15.1.8.32 Plugin Panel Opens and Functions

| # | Requirement |
|---|-------------|
| 1 | R-15.1.8    |

1. **#1** — Load plugin, open its panel, interact with widget
   - **Expected:** Panel renders, interactions produce `EditorCommand` on undo stack

### TC-15.1.8.33 Plugin Crash Does Not Kill Editor

| # | Requirement |
|---|-------------|
| 1 | R-1.6.4     |

1. **#1** — Load plugin that crashes on `update()`, trigger update
   - **Expected:** Plugin transitions to `Faulted`, editor continues, error logged

### TC-15.1.8.34 Dependency Chain Loading

| # | Requirement |
|---|-------------|
| 1 | R-1.6.5     |

1. **#1** — Load plugin A (depends on B and C), B depends on C
   - **Expected:** C loaded first, then B, then A

### TC-15.1.8.35 Hot-Reload Under Collaboration

| # | Requirement        |
|---|--------------------|
| 1 | R-1.6.3, R-15.12.3 |

1. **#1** — Two editors connected, hot-reload plugin on editor A
   - **Expected:** Editor A's plugin reloaded, editor B unaffected, CRDT sync continues

### TC-15.1.8.36 Engine Built-In Uses Same API

| # | Requirement |
|---|-------------|
| 1 | R-15.1.8    |

1. **#1** — Animation subsystem registers widget via `EditorPluginApi`
   - **Expected:** Inspector dispatches to animation editor for `AnimStateMachine` component

### TC-15.1.8.37 Plugin Menu and Toolbar Integration

| # | Requirement |
|---|-------------|
| 1 | R-15.1.8    |
| 2 | R-15.1.8    |

1. **#1** — Plugin adds menu action "Tools/MyPlugin/Action"
   - **Expected:** Action appears in menu, clickable, triggers callback
2. **#2** — Plugin adds toolbar button, click it
   - **Expected:** Button visible, click triggers callback

## Benchmarks

### TC-15.1.8.50 Plugin Load Time

| Benchmark | Target | Requirement |
|-----------|--------|-------------|
| Load a single plugin (dylib + register) | < 100 ms | US-15.1.8.2 |

### TC-15.1.8.51 Hot-Reload Time

| Benchmark | Target | Requirement |
|-----------|--------|-------------|
| Hot-reload cycle (serialize + unload + load + deserialize) | < 500 ms | US-15.1.8.4 |

### TC-15.1.8.52 Widget Dispatch Latency

| Benchmark | Target | Requirement |
|-----------|--------|-------------|
| `EditorWidgetRegistry::lookup()` per component | < 1 us | US-15.1.8.7 |

### TC-15.1.8.53 Plugin Memory Overhead

| Benchmark | Target | Requirement |
|-----------|--------|-------------|
| Per-plugin isolation scope overhead | < 64 KB base | US-15.1.8.8 |
