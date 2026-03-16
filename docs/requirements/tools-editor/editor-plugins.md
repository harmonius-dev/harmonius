# R-15.20 -- Editor Plugin Architecture Requirements

## Plugin System

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.20.1 | The editor **SHALL** provide a stable ABI plugin system using dynamic libraries with a versioned C ABI boundary exposed through Rust wrapper types. The plugin host **SHALL** discover plugins in designated directories, validate ABI compatibility, and load them into the editor process. Plugins **SHALL** access editor functionality through a capability-based API that grants access only to requested subsystems. | [F-15.20.1](../../features/tools-editor/editor-plugins.md) | Studios require project-specific editor extensions without forking the editor codebase. | Integration test: load a plugin, verify its panel appears. Unload the plugin and verify its UI elements are removed cleanly. |
| R-15.20.1a | The plugin ABI **SHALL** remain backward-compatible across minor engine versions. Plugins compiled against engine version N.M **SHALL** load successfully on engine versions N.M through N.(M+K) where K covers at least 3 minor releases. | [F-15.20.1](../../features/tools-editor/editor-plugins.md) | ABI stability reduces the maintenance burden on plugin developers and marketplace publishers. | Integration test: compile a plugin against engine version N.M and verify it loads on N.(M+3). |
| R-15.20.1b | Plugins **SHALL** run in isolation from the editor core such that a crashing plugin does not crash the editor. The plugin host **SHALL** catch plugin panics and unload the offending plugin with an error message. | [F-15.20.1](../../features/tools-editor/editor-plugins.md) | Unstable third-party plugins must not compromise editor reliability. | Unit test: trigger a panic in a test plugin and verify the editor continues running with the plugin unloaded. |

## Custom Component Editors

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.20.2 | Plugins **SHALL** be able to register custom editor widgets for their own component types via the reflection system (F-7.6.1). Custom editors **SHALL** integrate with the undo/redo stack (F-15.1.3) automatically. | [F-15.20.2](../../features/tools-editor/editor-plugins.md) | Default property grids are insufficient for complex component types; custom editors improve usability. | Integration test: register a custom editor for a test component, select an entity with that component, and verify the custom editor renders in the inspector. |
| R-15.20.2a | When no custom editor is registered for a component type, the inspector **SHALL** fall back to the default reflection-based property grid. | [F-15.20.2](../../features/tools-editor/editor-plugins.md) | All components must remain editable even without a custom editor. | Unit test: select an entity with a component that has no registered custom editor and verify the default property grid displays. |

## In-Engine Feature Editors

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.20.3 | Engine subsystems **SHALL** register their specialized editors through the same plugin API as third-party plugins, preventing divergence between built-in and external editor extensions. | [F-15.20.3](../../features/tools-editor/editor-plugins.md) | A single extension point ensures consistent behavior and avoids parallel code paths. | Integration test: verify that animation, physics, and VFX subsystem editors load via the plugin API. |

## Plugin Hot-Reload

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.20.4 | The editor **SHALL** support hot-reloading plugins by serializing plugin state, unloading the old library, loading the new library, and deserializing state back. Hot-reload **SHALL** preserve open panels, inspector state, and undo history. A file watcher **SHALL** monitor plugin build output directories and trigger reload automatically. | [F-15.20.4](../../features/tools-editor/editor-plugins.md) | Rapid iteration on editor tools requires reloading without restart. | Integration test: modify a plugin, trigger hot-reload, and verify updated behavior without restart and with undo history intact. |
| R-15.20.4a | Plugin hot-reload **SHOULD** be available only in debug builds. Release builds **SHALL** require a full editor restart to load new plugin versions. | [F-15.20.4](../../features/tools-editor/editor-plugins.md) | Hot-reload in release builds risks state corruption from optimized code paths. | Unit test: attempt hot-reload in a release build and verify it is rejected with an error message. |

## Plugin Dependency Management

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.20.5 | Plugins **SHALL** declare dependencies on other plugins, engine subsystems, and minimum engine versions in a JSON manifest file. The plugin host **SHALL** resolve the dependency graph, detect cycles and version conflicts, load plugins in topological order, and produce clear error messages for missing dependencies with installation guidance. | [F-15.20.5](../../features/tools-editor/editor-plugins.md) | Dependency resolution prevents load-order bugs and missing-dependency crashes. | Unit test: declare a circular dependency and verify the host reports a cycle error. Declare a missing dependency and verify the error includes installation guidance. |
| R-15.20.5a | Plugin version constraints **SHALL** use semantic versioning ranges (e.g., ">=1.2.0, <2.0.0"). The dependency resolver **SHALL** select the highest compatible version when multiple versions are available. | [F-15.20.5](../../features/tools-editor/editor-plugins.md) | Semantic versioning enables predictable compatibility across plugin versions. | Unit test: provide two plugin versions and verify the resolver selects the highest compatible version. |

## Plugin Marketplace Integration

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.20.6 | Plugins **SHALL** be publishable to and installable from the asset marketplace (F-15.17.1). One-click install **SHALL** download the plugin, resolve dependencies via F-15.20.5, and place files in the project plugin directory. Automatic update notifications **SHALL** alert users when newer compatible versions are available. | [F-15.20.6](../../features/tools-editor/editor-plugins.md) | Marketplace distribution enables a plugin ecosystem with minimal friction. | Integration test: install a plugin from the marketplace, verify it loads, and verify an update notification appears when a newer version is published. |

## No-Code Editor Plugin Creation

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.20.7 | Editor plugins **SHALL** be creatable entirely using the engine's visual authoring tools (logic graphs, UI editor, inspector graphs) with no Rust code required. A plugin project template **SHALL** scaffold the required manifest, entry-point graph, and UI layout. | [F-15.20.7](../../features/tools-editor/editor-plugins.md) | No-code plugin creation enables designers and technical artists to extend the editor without engineering support. | Integration test: create a no-code plugin from the template, add a panel via the UI editor, wire behavior via a logic graph, and verify the plugin loads and displays the panel in the editor. |
| R-15.20.7a | No-code plugins **SHALL** be packaged as standard editor plugins (F-15.20.1) and participate in dependency management (F-15.20.5), marketplace distribution (F-15.20.6), and hot module reload (F-15.20.10). | [F-15.20.7](../../features/tools-editor/editor-plugins.md) | A unified plugin format avoids separate distribution and loading paths for no-code vs. native plugins. | Integration test: publish a no-code plugin to the marketplace, install it in a fresh project, and verify it loads identically to a native plugin. |

## Plugin Logic Graphs

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.20.8 | Plugin logic graphs **SHALL** have access to editor-specific node types for panel creation, menu item registration, gizmo registration, selection change response, asset database queries, component read/write, and undo/redo invocation. | [F-15.20.8](../../features/tools-editor/editor-plugins.md) | Editor-specific nodes give no-code plugins the same extension capabilities as native plugins. | Integration test: build a logic graph using each editor-specific node type and verify each node executes its editor action correctly. |
| R-15.20.8a | Plugin logic graph execution **SHALL** be driven by editor lifecycle events (plugin load, panel open, selection change, tick) and compiled through the shared build cache (F-15.11.3). | [F-15.20.8](../../features/tools-editor/editor-plugins.md) | Event-driven execution ensures plugin logic responds to editor state changes without polling. | Integration test: register a selection-change handler in a plugin logic graph, change the selection, and verify the handler fires within one editor tick. |

## Plugin UI Authoring

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.20.9 | Plugin UI **SHALL** be authorable using the visual UI editor with drag-and-drop panel layout, property bindings to logic graph variables, custom inspector templates, toolbar buttons, context menu entries, and floating tool windows. | [F-15.20.9](../../features/tools-editor/editor-plugins.md) | Visual UI authoring enables non-programmers to build rich editor extensions with interactive controls. | Integration test: create a plugin panel with bound properties, a toolbar button, and a context menu entry via the visual UI editor. Verify all elements render and respond to interaction in the running editor. |
| R-15.20.9a | Plugin UI **SHALL** provide live preview within the editor during authoring and **SHALL** inherit styles from the editor theme for visual consistency. | [F-15.20.9](../../features/tools-editor/editor-plugins.md) | Live preview and theme inheritance prevent visual mismatches between plugin UI and the rest of the editor. | Integration test: open the UI editor for a plugin, modify a widget, and verify the preview updates in real time. Switch editor themes and verify the plugin UI adapts. |

## Plugin Hot Module Reload

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.20.10 | Plugin hot module reload **SHALL** preserve plugin state (via reflection serialization), UI layout, open panels, undo history, and inspector state across reload and **SHALL** complete within 2 seconds for typical plugins. | [F-15.20.10](../../features/tools-editor/editor-plugins.md) | Fast, state-preserving reload enables rapid iteration without losing editing context. | Integration test: open a plugin panel with modified state, trigger hot module reload, and verify the panel reappears with identical state within 2 seconds. Measure reload latency and verify it is under 2 seconds. |
| R-15.20.10a | Plugin hot module reload **SHALL** work in both debug and release builds for both no-code and native plugins. A file watcher **SHALL** monitor plugin source directories and trigger reload automatically. | [F-15.20.10](../../features/tools-editor/editor-plugins.md) | Supporting reload in release builds eliminates the restriction that limited the earlier hot-reload feature (F-15.20.4) to debug only. | Integration test: build a plugin in release mode, modify it, and verify the file watcher triggers automatic reload. Repeat for a no-code plugin. |

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/editor-plugins.md](../../user-stories/tools-editor/editor-plugins.md).
Requirements in this document are derived from those user stories.
