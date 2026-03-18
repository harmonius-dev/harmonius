# User Stories: Editor Plugin Architecture

## F-15.20.1 Plugin Architecture

| ID           | Persona                    | Features | Requirements |
|--------------|----------------------------|----------|--------------|
| US-15.20.1.1 | developer (P-15)           |          |              |
| US-15.20.1.2 | extension developer (P-25) |          |              |
| US-15.20.1.3 | engine developer (P-26)    |          |              |
| US-15.20.1.4 | developer (P-15)           |          |              |
| US-15.20.1.5 | engine developer (P-26)    |          |              |
| US-15.20.1.6 | tech artist (P-13)         |          |              |
| US-15.20.1.7 | designer (P-5)             |          |              |

1. **US-15.20.1.1** — to place a compiled plugin dynamic library in the project plugin directory and
   have the editor discover and load it automatically
   - **Acceptance:** I can extend the editor without modifying engine source code
2. **US-15.20.1.2** — the plugin API to expose a versioned C ABI boundary through Rust wrapper types
   - **Acceptance:** my plugins remain compatible across minor engine versions without recompilation
3. **US-15.20.1.3** — the plugin host to validate ABI compatibility before loading a plugin
   - **Acceptance:** binary-incompatible plugins are rejected with a clear error instead of causing
     undefined behavior
4. **US-15.20.1.4** — my plugin to declare which editor subsystems it needs access to via a
   capability-based API
   - **Acceptance:** the plugin host grants only the minimum required permissions
5. **US-15.20.1.5** — a crashing plugin to be caught and unloaded without bringing down the editor
   - **Acceptance:** unstable third-party plugins do not compromise editor reliability
6. **US-15.20.1.6** — to create a plugin that adds a custom panel, toolbar button, and context menu
   action
   - **Acceptance:** I can build studio-specific tools accessible from the editor UI
7. **US-15.20.1.7** — clear error messages when a plugin fails to load (wrong version, missing
   dependency, crash)
   - **Acceptance:** I can report the issue to the plugin developer with actionable information

## F-15.20.2 Custom Component Editors

| ID           | Persona                    | Features | Requirements |
|--------------|----------------------------|----------|--------------|
| US-15.20.2.1 | developer (P-15)           |          |              |
| US-15.20.2.2 | designer (P-5)             |          |              |
| US-15.20.2.3 | tech artist (P-13)         |          |              |
| US-15.20.2.4 | engine developer (P-26)    |          |              |
| US-15.20.2.5 | extension developer (P-25) |          |              |

1. **US-15.20.2.1** — to register a custom editor widget for my plugin-defined component type via
   the reflection system
   - **Acceptance:** the inspector renders my custom editor instead of the default property grid
2. **US-15.20.2.2** — to select an entity with a plugin-defined component and see a custom editor
   tailored to that component's data
   - **Acceptance:** I can edit complex plugin data without deciphering raw property values
3. **US-15.20.2.3** — custom component editors to integrate with the undo/redo stack automatically
   - **Acceptance:** edits made through plugin editors are undoable like any other editor operation
4. **US-15.20.2.4** — the inspector to fall back to the default reflection-based property grid when
   no custom editor is registered
   - **Acceptance:** all components remain editable regardless of plugin presence
5. **US-15.20.2.5** — my custom editor to receive read/write access to the component data through
   the reflection API
   - **Acceptance:** I can build rich editing interfaces with validated input

## F-15.20.3 In-Engine Feature Editors

| ID           | Persona                    | Features | Requirements |
|--------------|----------------------------|----------|--------------|
| US-15.20.3.1 | engine developer (P-26)    |          |              |
| US-15.20.3.2 | tech artist (P-13)         |          |              |
| US-15.20.3.3 | developer (P-15)           |          |              |
| US-15.20.3.4 | extension developer (P-25) |          |              |
| US-15.20.3.5 | designer (P-5)             |          |              |

1. **US-15.20.3.1** — engine subsystems (animation, physics, audio, AI, VFX) to register their
   editors through the same plugin API as third-party plugins
   - **Acceptance:** there is a single extension mechanism for all editors
2. **US-15.20.3.2** — in-engine editors to be extensible by plugins
   - **Acceptance:** I can add custom panels or features to the built-in animation or material
     editors
3. **US-15.20.3.3** — built-in and third-party editors to use the same registration, lifecycle, and
   rendering APIs
   - **Acceptance:** learning the plugin API once covers all editor extension scenarios
4. **US-15.20.3.4** — in-engine feature editors to serve as reference implementations of the plugin
   API
   - **Acceptance:** I can study how official editors are built when writing my own
5. **US-15.20.3.5** — all editors to behave consistently regardless of whether they are built-in or
   from a plugin
   - **Acceptance:** I have a uniform experience across the editor

## F-15.20.4 Plugin Hot-Reload

| ID           | Persona                    | Features | Requirements |
|--------------|----------------------------|----------|--------------|
| US-15.20.4.1 | tech artist (P-13)         |          |              |
| US-15.20.4.2 | developer (P-15)           |          |              |
| US-15.20.4.3 | engine developer (P-26)    |          |              |
| US-15.20.4.4 | extension developer (P-25) |          |              |
| US-15.20.4.5 | designer (P-5)             |          |              |

1. **US-15.20.4.1** — to recompile my plugin and have the editor hot-reload it automatically without
   restarting
   - **Acceptance:** I can iterate on custom tools in seconds
2. **US-15.20.4.2** — hot-reload to preserve open panels, inspector state, and undo history
   - **Acceptance:** I do not lose my editing context when reloading a plugin
3. **US-15.20.4.3** — a file watcher monitoring plugin build output directories that triggers reload
   automatically
   - **Acceptance:** reloading does not require manual commands
4. **US-15.20.4.4** — hot-reload to be restricted to debug builds
   - **Acceptance:** release builds do not risk state corruption from library swapping
5. **US-15.20.4.5** — a notification when a plugin is hot-reloaded showing what changed
   - **Acceptance:** I know my tools have been updated and can verify the new behavior

## F-15.20.5 Plugin Dependency Management

| ID           | Persona                    | Features | Requirements |
|--------------|----------------------------|----------|--------------|
| US-15.20.5.1 | extension developer (P-25) |          |              |
| US-15.20.5.2 | developer (P-15)           |          |              |
| US-15.20.5.3 | engine developer (P-26)    |          |              |
| US-15.20.5.4 | extension developer (P-25) |          |              |
| US-15.20.5.5 | tech artist (P-13)         |          |              |

1. **US-15.20.5.1** — to declare dependencies on other plugins, engine subsystems, and minimum
   engine version in a JSON manifest
   - **Acceptance:** the plugin host resolves and loads dependencies in the correct order
2. **US-15.20.5.2** — clear error messages for missing dependencies including installation guidance
   - **Acceptance:** I can resolve dependency issues without guessing what is needed
3. **US-15.20.5.3** — the plugin host to detect circular dependencies and report them as errors
   - **Acceptance:** plugins with impossible dependency graphs are rejected before loading
4. **US-15.20.5.4** — to specify semantic versioning ranges for dependencies (e.g., ">=1.2.0,
   <2.0.0")
   - **Acceptance:** my plugin works with compatible versions without pinning to exact releases
5. **US-15.20.5.5** — the editor to automatically resolve and load plugin dependencies in
   topological order
   - **Acceptance:** I do not need to manually manage plugin load order

## F-15.20.6 Plugin Marketplace Integration

| ID           | Persona                    | Features | Requirements |
|--------------|----------------------------|----------|--------------|
| US-15.20.6.1 | designer (P-5)             |          |              |
| US-15.20.6.2 | extension developer (P-25) |          |              |
| US-15.20.6.3 | developer (P-15)           |          |              |
| US-15.20.6.4 | tech artist (P-13)         |          |              |
| US-15.20.6.5 | engine developer (P-26)    |          |              |

1. **US-15.20.6.1** — to browse plugins in the asset marketplace and install them with one click
   - **Acceptance:** I can extend the editor without manual file management
2. **US-15.20.6.2** — to publish my plugin to the asset marketplace with metadata (supported engine
   versions, dependencies, screenshots)
   - **Acceptance:** users can discover and install my plugin
3. **US-15.20.6.3** — automatic notifications when a newer compatible version of an installed plugin
   is available
   - **Acceptance:** I stay current with bug fixes and new features
4. **US-15.20.6.4** — marketplace plugin install to resolve and download transitive dependencies
   automatically
   - **Acceptance:** I do not need to hunt down required sub-plugins manually
5. **US-15.20.6.5** — marketplace listings to display compatibility badges per engine version
   - **Acceptance:** users only install plugins verified to work with their engine version

## F-15.20.7 No-Code Editor Plugin Creation

| ID           | Persona                    | Features | Requirements |
|--------------|----------------------------|----------|--------------|
| US-15.20.7.1 | technical artist (P-13)    |          |              |
| US-15.20.7.2 | extension developer (P-25) |          |              |
| US-15.20.7.3 | designer (P-5)             |          |              |
| US-15.20.7.4 | modder (P-24)              |          |              |
| US-15.20.7.5 | technical artist (P-13)    |          |              |

1. **US-15.20.7.1** — to create a custom material preview panel as an editor plugin using only logic
   graphs and the visual UI editor
   - **Acceptance:** I can build studio-specific tools without writing Rust code
2. **US-15.20.7.2** — a plugin project template that scaffolds a manifest, entry-point logic graph,
   and UI layout
   - **Acceptance:** I can start building a no-code plugin without manual boilerplate
3. **US-15.20.7.3** — to create a new editor plugin project from a template in the project creation
   wizard
   - **Acceptance:** I can start extending the editor without needing an engineer to set up the
     project structure
4. **US-15.20.7.4** — my no-code editor plugin to be packaged as a standard plugin distributable on
   the marketplace
   - **Acceptance:** other users can install and use my custom editor tools
5. **US-15.20.7.5** — to publish my no-code plugin to the asset marketplace alongside native plugins
   - **Acceptance:** users cannot distinguish between no-code and native plugins from the consumer
     side

## F-15.20.8 Plugin Logic Graphs

| ID           | Persona                    | Features | Requirements |
|--------------|----------------------------|----------|--------------|
| US-15.20.8.1 | extension developer (P-25) |          |              |
| US-15.20.8.2 | technical artist (P-13)    |          |              |
| US-15.20.8.3 | designer (P-5)             |          |              |
| US-15.20.8.4 | modder (P-24)              |          |              |
| US-15.20.8.5 | extension developer (P-25) |          |              |

1. **US-15.20.8.1** — to build a terrain brush plugin using editor-specific logic graph nodes
   (register gizmo, respond to selection change, read/write component data)
   - **Acceptance:** I can author interactive editor tools visually
2. **US-15.20.8.2** — to use an "add menu item" node in my plugin logic graph to register a custom
   menu action
   - **Acceptance:** users can invoke my tool from the editor menu bar
3. **US-15.20.8.3** — my plugin logic graph to respond to entity selection change events
   - **Acceptance:** my custom panel updates its contents when I select a different entity in the
     viewport
4. **US-15.20.8.4** — to use a "register gizmo" node in a logic graph to add a custom 3D
   manipulation gizmo to the viewport
   - **Acceptance:** I can create specialized editing handles for my mod's content
5. **US-15.20.8.5** — to use an "asset database query" node in my plugin logic graph to search and
   filter project assets
   - **Acceptance:** my plugin can present asset lists and perform batch operations

## F-15.20.9 Plugin UI Authoring

| ID           | Persona                    | Features | Requirements |
|--------------|----------------------------|----------|--------------|
| US-15.20.9.1 | designer (P-5)             |          |              |
| US-15.20.9.2 | technical artist (P-13)    |          |              |
| US-15.20.9.3 | extension developer (P-25) |          |              |
| US-15.20.9.4 | modder (P-24)              |          |              |
| US-15.20.9.5 | designer (P-5)             |          |              |

1. **US-15.20.9.1** — to create a custom inspector for a gameplay component using drag-and-drop in
   the visual UI editor
   - **Acceptance:** I can tailor the inspector to show only the fields I need with appropriate
     widgets
2. **US-15.20.9.2** — to define a floating tool window in the visual UI editor with property
   bindings to my plugin's logic graph
   - **Acceptance:** I can build interactive tool panels with live data
3. **US-15.20.9.3** — live preview of my plugin UI inside the editor while I author it
   - **Acceptance:** I can see layout and styling changes immediately without reloading
4. **US-15.20.9.4** — to add a toolbar button to the editor using the visual UI editor
   - **Acceptance:** users can launch my mod's editor panel with a single click
5. **US-15.20.9.5** — my plugin UI to automatically inherit the current editor theme
   - **Acceptance:** my custom panels match the look and feel of the rest of the editor

## F-15.20.10 Plugin Hot Module Reload

| ID            | Persona                    | Features | Requirements |
|---------------|----------------------------|----------|--------------|
| US-15.20.10.1 | technical artist (P-13)    |          |              |
| US-15.20.10.2 | extension developer (P-25) |          |              |
| US-15.20.10.3 | designer (P-5)             |          |              |
| US-15.20.10.4 | modder (P-24)              |          |              |
| US-15.20.10.5 | extension developer (P-25) |          |              |

1. **US-15.20.10.1** — to modify my no-code plugin's logic graph and see the updated behavior in the
   editor within 2 seconds without restarting
   - **Acceptance:** I can iterate on custom tools rapidly
2. **US-15.20.10.2** — hot module reload to preserve my plugin's open panels, undo history, and
   inspector state
   - **Acceptance:** I do not lose my editing context when reloading
3. **US-15.20.10.3** — a notification when a plugin is hot-reloaded showing what changed
   - **Acceptance:** I know my tools have been updated and can verify the new behavior
4. **US-15.20.10.4** — plugin hot module reload to work in release builds
   - **Acceptance:** I can iterate on my mod's editor tools without switching to a debug build
5. **US-15.20.10.5** — a file watcher to detect changes to my plugin source files and trigger reload
   automatically
   - **Acceptance:** I do not need to manually invoke reload after every edit
