# 15.20 -- Editor Plugin Architecture

## Plugin System

### F-15.20.1 Plugin Architecture

A stable ABI plugin system for extending the editor with custom panels, inspectors, gizmos,
importers, and toolbar actions. Plugins are compiled as dynamic libraries with a versioned C ABI
boundary exposed through Rust wrapper types. The plugin host discovers plugins in designated
directories, validates ABI compatibility, and loads them into the editor process. Plugins access
editor functionality through a capability-based API that grants access only to requested subsystems.

- **Requirements:** R-15.20.1
- **Dependencies:** F-15.1.8, F-1.6.1
- **Platform notes:** On macOS, plugins are `.dylib` bundles loaded via `dlopen`. On Windows, `.dll`
  via `LoadLibrary`. On Linux, `.so` via `dlopen`.

### F-15.20.2 Custom Component Editors

Plugins can register custom editor widgets for their own component types. When an entity containing
a plugin-defined component is selected, the inspector panel renders the plugin-provided editor
instead of the default property grid. Custom editors receive read/write access to the component data
through the reflection system (F-7.6.1) and integrate with the undo/redo stack (F-15.1.3)
automatically.

- **Requirements:** R-15.20.2
- **Dependencies:** F-15.20.1, F-7.6.1, F-15.1.3
- **Platform notes:** Desktop only. Not available on mobile or console runtime.

### F-15.20.3 In-Engine Feature Editors

Engine subsystems provide their own specialized editors using the same plugin mechanism as
third-party plugins. Animation, physics, audio, AI, and VFX subsystems each register editors for
their data types through the plugin API. This ensures a single extension point for both internal and
external editors, preventing divergence between built-in and plugin-provided editors.

- **Requirements:** R-15.20.3
- **Dependencies:** F-15.20.1, F-15.20.2
- **Platform notes:** Desktop only. Not available on mobile or console runtime.

### F-15.20.4 Plugin Hot-Reload

Plugins can be recompiled and reloaded without restarting the editor. The plugin host serializes
plugin state, unloads the old library, loads the new library, and deserializes the state back into
the new plugin instance. Hot-reload preserves open panels, inspector state, and undo history. A file
watcher monitors plugin build output directories and triggers reload automatically when a new build
is detected.

- **Requirements:** R-15.20.4
- **Dependencies:** F-15.20.1, F-7.6.1
- **Platform notes:** Desktop only. Debug builds only for safety. Release builds require a full
  editor restart.

### F-15.20.5 Plugin Dependency Management

Plugins declare dependencies on other plugins, engine subsystems, and minimum engine versions in a
manifest file. The plugin host resolves the dependency graph, detects cycles and version conflicts,
and loads plugins in topological order. Missing dependencies produce clear error messages with
installation guidance. Version constraints use semantic versioning ranges.

- **Requirements:** R-15.20.5
- **Dependencies:** F-15.20.1
- **Platform notes:** The manifest file uses JSON format, consistent with F-15.1.7 preferences.

### F-15.20.6 Plugin Marketplace Integration

Plugins can be published to and installed from the asset marketplace (F-15.17.1). The marketplace
displays plugin metadata including supported engine versions, dependency list, screenshots, and
compatibility badges. One-click install downloads the plugin, resolves dependencies, and places
files in the project plugin directory. Automatic update notifications alert users when newer
compatible versions are available.

- **Requirements:** R-15.20.6
- **Dependencies:** F-15.20.1, F-15.20.5, F-15.17.1, F-15.17.2
- **Platform notes:** Marketplace communication uses the same REST API as asset store (F-15.17.1).

## No-Code Plugin Authoring

### F-15.20.7 No-Code Editor Plugin Creation

Create editor plugins entirely using the engine's visual authoring tools — logic graphs (F-15.8.1),
the UI editor (F-15.9.1), and custom inspector graphs — without writing any Rust code. A plugin
project template scaffolds the required manifest, logic graph entry point, and UI layout. The
resulting plugin is packaged as a standard editor plugin (F-15.20.1) and participates in hot-reload
(F-15.20.10), dependency management (F-15.20.5), and marketplace distribution (F-15.20.6).

- **Requirements:** R-15.20.7
- **Dependencies:** F-15.20.1, F-15.8.1, F-15.9.1, F-15.20.5
- **Platform notes:** Desktop only. Not available on mobile or console runtime.

### F-15.20.8 Plugin Logic Graphs

Editor plugin behavior authored as logic graphs. The node library includes editor-specific node
types: create panel, add menu item, register gizmo, respond to selection change, query asset
database, read/write component data, and invoke undo/redo commands. Graph execution is driven by
editor lifecycle events (plugin load, panel open, selection change, tick). Compiled through the
shared build cache (F-15.11.3) using the same pipeline as gameplay logic graphs.

- **Requirements:** R-15.20.8
- **Dependencies:** F-15.20.7, F-15.8.1, F-15.8.12, F-15.1.3
- **Platform notes:** Desktop only. Not available on mobile or console runtime.

### F-15.20.9 Plugin UI Authoring

Editor plugin UI defined using the visual UI editor. Drag-and-drop panel layout with property
bindings to logic graph variables. Supports custom inspector templates, toolbar buttons, context
menu entries, and floating tool windows. Live preview renders the plugin UI inside the editor while
authoring. Style inheritance from the editor theme ensures visual consistency.

- **Requirements:** R-15.20.9
- **Dependencies:** F-15.20.7, F-15.9.1, F-15.20.8
- **Platform notes:** Desktop only. Not available on mobile or console runtime.

### F-15.20.10 Plugin Hot Module Reload

Hot-reload editor plugins (both no-code and native) while the editor is running. The reload pipeline
serializes plugin state via the reflection system (F-7.6.1), unloads the old module, loads the
updated module, and deserializes state back. UI layout, open panels, undo history, and inspector
state are preserved across reload. Works in both debug and release builds. A file watcher monitors
plugin source directories and triggers reload automatically when changes are detected. Reload
completes within 2 seconds for typical plugins.

- **Requirements:** R-15.20.10
- **Dependencies:** F-15.20.1, F-15.20.7, F-7.6.1, F-12.4.1
- **Platform notes:** Desktop only. Not available on mobile or console runtime.
