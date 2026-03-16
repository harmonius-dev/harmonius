# User Stories: Editor Plugin Architecture

## F-15.20.1 Plugin Architecture

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.20.1.1 | developer (P-15) | to place a compiled plugin dynamic library in the project plugin directory and have the editor discover and load it automatically | I can extend the editor without modifying engine source code |  |  |
| US-15.20.1.2 | extension developer (P-25) | the plugin API to expose a versioned C ABI boundary through Rust wrapper types | my plugins remain compatible across minor engine versions without recompilation |  |  |
| US-15.20.1.3 | engine developer (P-26) | the plugin host to validate ABI compatibility before loading a plugin | binary-incompatible plugins are rejected with a clear error instead of causing undefined behavior |  |  |
| US-15.20.1.4 | developer (P-15) | my plugin to declare which editor subsystems it needs access to via a capability-based API | the plugin host grants only the minimum required permissions |  |  |
| US-15.20.1.5 | engine developer (P-26) | a crashing plugin to be caught and unloaded without bringing down the editor | unstable third-party plugins do not compromise editor reliability |  |  |
| US-15.20.1.6 | tech artist (P-13) | to create a plugin that adds a custom panel, toolbar button, and context menu action | I can build studio-specific tools accessible from the editor UI |  |  |
| US-15.20.1.7 | designer (P-5) | clear error messages when a plugin fails to load (wrong version, missing dependency, crash) | I can report the issue to the plugin developer with actionable information |  |  |

## F-15.20.2 Custom Component Editors

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.20.2.1 | developer (P-15) | to register a custom editor widget for my plugin-defined component type via the reflection system | the inspector renders my custom editor instead of the default property grid |  |  |
| US-15.20.2.2 | designer (P-5) | to select an entity with a plugin-defined component and see a custom editor tailored to that component's data | I can edit complex plugin data without deciphering raw property values |  |  |
| US-15.20.2.3 | tech artist (P-13) | custom component editors to integrate with the undo/redo stack automatically | edits made through plugin editors are undoable like any other editor operation |  |  |
| US-15.20.2.4 | engine developer (P-26) | the inspector to fall back to the default reflection-based property grid when no custom editor is registered | all components remain editable regardless of plugin presence |  |  |
| US-15.20.2.5 | extension developer (P-25) | my custom editor to receive read/write access to the component data through the reflection API | I can build rich editing interfaces with validated input |  |  |

## F-15.20.3 In-Engine Feature Editors

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.20.3.1 | engine developer (P-26) | engine subsystems (animation, physics, audio, AI, VFX) to register their editors through the same plugin API as third-party plugins | there is a single extension mechanism for all editors |  |  |
| US-15.20.3.2 | tech artist (P-13) | in-engine editors to be extensible by plugins | I can add custom panels or features to the built-in animation or material editors |  |  |
| US-15.20.3.3 | developer (P-15) | built-in and third-party editors to use the same registration, lifecycle, and rendering APIs | learning the plugin API once covers all editor extension scenarios |  |  |
| US-15.20.3.4 | extension developer (P-25) | in-engine feature editors to serve as reference implementations of the plugin API | I can study how official editors are built when writing my own |  |  |
| US-15.20.3.5 | designer (P-5) | all editors to behave consistently regardless of whether they are built-in or from a plugin | I have a uniform experience across the editor |  |  |

## F-15.20.4 Plugin Hot-Reload

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.20.4.1 | tech artist (P-13) | to recompile my plugin and have the editor hot-reload it automatically without restarting | I can iterate on custom tools in seconds |  |  |
| US-15.20.4.2 | developer (P-15) | hot-reload to preserve open panels, inspector state, and undo history | I do not lose my editing context when reloading a plugin |  |  |
| US-15.20.4.3 | engine developer (P-26) | a file watcher monitoring plugin build output directories that triggers reload automatically | reloading does not require manual commands |  |  |
| US-15.20.4.4 | extension developer (P-25) | hot-reload to be restricted to debug builds | release builds do not risk state corruption from library swapping |  |  |
| US-15.20.4.5 | designer (P-5) | a notification when a plugin is hot-reloaded showing what changed | I know my tools have been updated and can verify the new behavior |  |  |

## F-15.20.5 Plugin Dependency Management

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.20.5.1 | extension developer (P-25) | to declare dependencies on other plugins, engine subsystems, and minimum engine version in a JSON manifest | the plugin host resolves and loads dependencies in the correct order |  |  |
| US-15.20.5.2 | developer (P-15) | clear error messages for missing dependencies including installation guidance | I can resolve dependency issues without guessing what is needed |  |  |
| US-15.20.5.3 | engine developer (P-26) | the plugin host to detect circular dependencies and report them as errors | plugins with impossible dependency graphs are rejected before loading |  |  |
| US-15.20.5.4 | extension developer (P-25) | to specify semantic versioning ranges for dependencies (e.g., ">=1.2.0, <2.0.0") | my plugin works with compatible versions without pinning to exact releases |  |  |
| US-15.20.5.5 | tech artist (P-13) | the editor to automatically resolve and load plugin dependencies in topological order | I do not need to manually manage plugin load order |  |  |

## F-15.20.6 Plugin Marketplace Integration

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.20.6.1 | designer (P-5) | to browse plugins in the asset marketplace and install them with one click | I can extend the editor without manual file management |  |  |
| US-15.20.6.2 | extension developer (P-25) | to publish my plugin to the asset marketplace with metadata (supported engine versions, dependencies, screenshots) | users can discover and install my plugin |  |  |
| US-15.20.6.3 | developer (P-15) | automatic notifications when a newer compatible version of an installed plugin is available | I stay current with bug fixes and new features |  |  |
| US-15.20.6.4 | tech artist (P-13) | marketplace plugin install to resolve and download transitive dependencies automatically | I do not need to hunt down required sub-plugins manually |  |  |
| US-15.20.6.5 | engine developer (P-26) | marketplace listings to display compatibility badges per engine version | users only install plugins verified to work with their engine version |  |  |

## F-15.20.7 No-Code Editor Plugin Creation

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.20.7.1 | technical artist (P-13) | to create a custom material preview panel as an editor plugin using only logic graphs and the visual UI editor | I can build studio-specific tools without writing Rust code |  |  |
| US-15.20.7.2 | extension developer (P-25) | a plugin project template that scaffolds a manifest, entry-point logic graph, and UI layout | I can start building a no-code plugin without manual boilerplate |  |  |
| US-15.20.7.3 | designer (P-5) | to create a new editor plugin project from a template in the project creation wizard | I can start extending the editor without needing an engineer to set up the project structure |  |  |
| US-15.20.7.4 | modder (P-24) | my no-code editor plugin to be packaged as a standard plugin distributable on the marketplace | other users can install and use my custom editor tools |  |  |
| US-15.20.7.5 | technical artist (P-13) | to publish my no-code plugin to the asset marketplace alongside native plugins | users cannot distinguish between no-code and native plugins from the consumer side |  |  |

## F-15.20.8 Plugin Logic Graphs

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.20.8.1 | extension developer (P-25) | to build a terrain brush plugin using editor-specific logic graph nodes (register gizmo, respond to selection change, read/write component data) | I can author interactive editor tools visually |  |  |
| US-15.20.8.2 | technical artist (P-13) | to use an "add menu item" node in my plugin logic graph to register a custom menu action | users can invoke my tool from the editor menu bar |  |  |
| US-15.20.8.3 | designer (P-5) | my plugin logic graph to respond to entity selection change events | my custom panel updates its contents when I select a different entity in the viewport |  |  |
| US-15.20.8.4 | modder (P-24) | to use a "register gizmo" node in a logic graph to add a custom 3D manipulation gizmo to the viewport | I can create specialized editing handles for my mod's content |  |  |
| US-15.20.8.5 | extension developer (P-25) | to use an "asset database query" node in my plugin logic graph to search and filter project assets | my plugin can present asset lists and perform batch operations |  |  |

## F-15.20.9 Plugin UI Authoring

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.20.9.1 | designer (P-5) | to create a custom inspector for a gameplay component using drag-and-drop in the visual UI editor | I can tailor the inspector to show only the fields I need with appropriate widgets |  |  |
| US-15.20.9.2 | technical artist (P-13) | to define a floating tool window in the visual UI editor with property bindings to my plugin's logic graph | I can build interactive tool panels with live data |  |  |
| US-15.20.9.3 | extension developer (P-25) | live preview of my plugin UI inside the editor while I author it | I can see layout and styling changes immediately without reloading |  |  |
| US-15.20.9.4 | modder (P-24) | to add a toolbar button to the editor using the visual UI editor | users can launch my mod's editor panel with a single click |  |  |
| US-15.20.9.5 | designer (P-5) | my plugin UI to automatically inherit the current editor theme | my custom panels match the look and feel of the rest of the editor |  |  |

## F-15.20.10 Plugin Hot Module Reload

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.20.10.1 | technical artist (P-13) | to modify my no-code plugin's logic graph and see the updated behavior in the editor within 2 seconds without restarting | I can iterate on custom tools rapidly |  |  |
| US-15.20.10.2 | extension developer (P-25) | hot module reload to preserve my plugin's open panels, undo history, and inspector state | I do not lose my editing context when reloading |  |  |
| US-15.20.10.3 | designer (P-5) | a notification when a plugin is hot-reloaded showing what changed | I know my tools have been updated and can verify the new behavior |  |  |
| US-15.20.10.4 | modder (P-24) | plugin hot module reload to work in release builds | I can iterate on my mod's editor tools without switching to a debug build |  |  |
| US-15.20.10.5 | extension developer (P-25) | a file watcher to detect changes to my plugin source files and trigger reload automatically | I do not need to manually invoke reload after every edit |  |  |
