# Editor Plugin Architecture Test Cases

Companion test cases for [editor-plugins.md](editor-plugins.md).

## Unit Tests

### TC-15.1.8.1 Plugin Discovery Scans Directories

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Search path with 3 `.dylib` files | `discover()` returns 3 `PluginMetadata` entries | R-1.6.1 |
| 2 | Empty search path | `discover()` returns empty `Vec` | R-1.6.1 |
| 3 | Path with non-plugin files | `discover()` returns only valid plugin entries | R-1.6.1 |

### TC-15.1.8.2 Plugin Load Creates Handle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `load(valid_plugin_path, api)` | Returns `Ok(PluginHandle)` with `state == Active` | R-1.6.2 |
| 2 | `load(nonexistent_path, api)` | Returns `Err(PluginError::LoadFailed)` | R-1.6.2 |
| 3 | `load(invalid_dylib, api)` | Returns `Err(PluginError::MissingEntryPoint)` | R-1.6.2 |

### TC-15.1.8.3 Plugin Unload Removes Registrations

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load plugin that registers 1 panel, `unload()` | Panel no longer in `PanelRegistry` | R-1.6.2 |
| 2 | Load plugin that registers 1 gizmo, `unload()` | Gizmo no longer in `GizmoManager` | R-1.6.2 |
| 3 | Load plugin that registers 1 widget, `unload()` | Widget no longer in `EditorWidgetRegistry` | R-1.6.2 |

### TC-15.1.8.4 Dependency Resolution Topological Sort

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | A depends on B, B depends on C | Load order: `[C, B, A]` | R-1.6.5 |
| 2 | A and B independent | Load order contains both (either order) | R-1.6.5 |
| 3 | A depends on B, B depends on A | `Err(DependencyError::CircularDependency)` | R-1.6.5 |

### TC-15.1.8.5 Dependency Resolution Missing Dependency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | A depends on B, B not available | `Err(DependencyError::MissingDependency { plugin: "A", missing: "B" })` | R-1.6.5 |

### TC-15.1.8.6 Version Compatibility Check

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Plugin `min_host_version: 1.0.0`, host `1.2.0` | `Ok(())` — compatible | R-1.6.6 |
| 2 | Plugin `min_host_version: 2.0.0`, host `1.2.0` | `Err(DependencyError::HostVersionIncompatible)` | R-1.6.6 |
| 3 | Plugin depends on B `>=1.0`, B version `0.9.0` | `Err(DependencyError::VersionIncompatible)` | R-1.6.6 |

### TC-15.1.8.7 Plugin Isolation Catches Panic

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Plugin `register_editor()` panics | `PluginHandle.state == Faulted(msg)`, editor continues running | R-1.6.4 |
| 2 | Plugin `on_unload()` panics | Unload completes, error logged, editor unaffected | R-1.6.4 |

### TC-15.1.8.8 Plugin Allocator Tracking

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Plugin allocates 1 MB | `allocator.allocated_bytes() >= 1_000_000` | R-1.6.4 |
| 2 | Plugin unloaded | `allocator.allocated_bytes() == 0` after cleanup | R-1.6.4 |
| 3 | Check peak after load | `allocator.peak_bytes() >= allocated_bytes()` | R-1.6.4 |

### TC-15.1.8.9 Hot-Reload Preserves State

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Plugin with state `{counter: 42}`, `hot_reload()` | After reload, `counter == 42` | R-1.6.3 |
| 2 | Plugin with empty state, `hot_reload()` | Reload succeeds, plugin active | R-1.6.3 |

### TC-15.1.8.10 Hot-Reload Re-Registers UI

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Plugin registers panel, `hot_reload()` | Panel still in `PanelRegistry` after reload | R-1.6.3 |
| 2 | Plugin registers widget, `hot_reload()` | Widget still in `EditorWidgetRegistry` | R-1.6.3 |

### TC-15.1.8.11 Hot-Reload Deserialize Failure

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Plugin state format changed, `hot_reload()` | `Err(PluginError::DeserializeFailed)`, old version stays loaded | R-1.6.3 |

### TC-15.1.8.12 Widget Registry Register and Lookup

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `register(TypeA, factory_a)`, `lookup(TypeA)` | Returns `Some(&factory_a)` | R-1.6.7 |
| 2 | `lookup(TypeB)` without registration | Returns `None` | R-1.6.7 |
| 3 | `register(TypeA, factory_a)`, `unregister(TypeA)`, `lookup(TypeA)` | Returns `None` | R-1.6.7 |

### TC-15.1.8.13 Widget Registry Override

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `register(TypeA, factory_1)`, `register(TypeA, factory_2)` | `lookup(TypeA)` returns `factory_2` | R-1.6.7 |

### TC-15.1.8.14 Editor Plugin API Register Panel

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `api.register_panel(descriptor)` | `PanelRegistry` contains the panel | R-15.1.8 |
| 2 | `api.unregister_panel(id)` | Panel removed from `PanelRegistry` | R-15.1.8 |

### TC-15.1.8.15 Editor Plugin API Register Gizmo

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `api.register_gizmo(descriptor)` | Gizmo appears in `GizmoManager` custom list | R-15.1.8 |

### TC-15.1.8.16 Editor Plugin API Register Hotkey

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `api.register_hotkey(Ctrl+T, "my_action")` | `HotKeyMap.lookup(Ctrl+T) == "my_action"` | R-15.1.8 |

### TC-15.1.8.17 Plugin Get By Name

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load plugin "foo", `get_by_name("foo")` | Returns `Some(handle)` with `name == "foo"` | R-1.6.1 |
| 2 | `get_by_name("nonexistent")` | Returns `None` | R-1.6.1 |

### TC-15.1.8.18 Plugin Metadata Extraction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `harmonius_plugin_metadata(ptr)` | Returns metadata with correct name, version, dependencies | R-1.6.6 |

### TC-15.1.8.19 C ABI Entry Point Validation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | dylib with `harmonius_plugin_create` symbol | `load()` succeeds | R-1.6.6 |
| 2 | dylib without `harmonius_plugin_create` | `Err(PluginError::MissingEntryPoint)` | R-1.6.6 |

### TC-15.1.8.20 Plugin State Serialization Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `serialize_state()` then `deserialize_state(bytes)` | Plugin state identical before and after | R-1.6.3 |

### TC-15.1.8.21 File Watch Detects Change

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Modify plugin `.dylib` on disk, `poll_changes()` | Returns handle of changed plugin | R-1.6.3 |
| 2 | No file changes, `poll_changes()` | Returns empty `Vec` | R-1.6.3 |

### TC-15.1.8.22 Multiple Plugins Coexist

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load plugin A and B, both register panels | Both panels in `PanelRegistry` | R-1.6.2 |
| 2 | Unload plugin A | Plugin A's panel removed, B's panel remains | R-1.6.2 |

## Integration Tests

### TC-15.1.8.30 End-to-End Plugin Lifecycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `discover()`, `load()`, verify active, `hot_reload()`, verify active, `unload()` | Plugin transitions through all states correctly | R-1.6.2, R-1.6.3 |

### TC-15.1.8.31 Component Editor Dispatch via Plugin

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Plugin registers widget for `HealthComponent`, select entity with `HealthComponent` | Inspector uses custom widget, not auto-generated | R-1.6.7 |
| 2 | Unload plugin, reselect entity | Inspector falls back to auto-generated widget | R-1.6.7 |

### TC-15.1.8.32 Plugin Panel Opens and Functions

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load plugin, open its panel, interact with widget | Panel renders, interactions produce `EditorCommand` on undo stack | R-15.1.8 |

### TC-15.1.8.33 Plugin Crash Does Not Kill Editor

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load plugin that crashes on `update()`, trigger update | Plugin transitions to `Faulted`, editor continues, error logged | R-1.6.4 |

### TC-15.1.8.34 Dependency Chain Loading

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load plugin A (depends on B and C), B depends on C | C loaded first, then B, then A | R-1.6.5 |

### TC-15.1.8.35 Hot-Reload Under Collaboration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two editors connected, hot-reload plugin on editor A | Editor A's plugin reloaded, editor B unaffected, CRDT sync continues | R-1.6.3, R-15.12.3 |

### TC-15.1.8.36 Engine Built-In Uses Same API

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Animation subsystem registers widget via `EditorPluginApi` | Inspector dispatches to animation editor for `AnimStateMachine` component | R-15.1.8 |

### TC-15.1.8.37 Plugin Menu and Toolbar Integration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Plugin adds menu action "Tools/MyPlugin/Action" | Action appears in menu, clickable, triggers callback | R-15.1.8 |
| 2 | Plugin adds toolbar button, click it | Button visible, click triggers callback | R-15.1.8 |

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
