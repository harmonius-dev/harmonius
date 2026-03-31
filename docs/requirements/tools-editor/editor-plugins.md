# R-15.20 -- Editor Plugin Architecture Requirements

## Requirements

1. **R-15.20.1** — The engine **SHALL** provide a stable ABI plugin system with capability-based API
   access, supporting custom panels, inspectors, gizmos, and importers.
   - **Rationale:** Stable ABI enables plugins that survive engine updates.
   - **Verification:** Load a plugin compiled against version N in version N+1 and verify it
     functions correctly.

2. **R-15.20.2** — The engine **SHALL** allow plugins to register custom editor widgets for
   component types with automatic undo/redo integration.
   - **Rationale:** Custom editors improve usability of plugin components.
   - **Verification:** Register a custom widget, edit a value, undo, and verify the value reverts.

3. **R-15.20.3** — The engine **SHALL** register built-in subsystem editors through the same plugin
   mechanism as third-party plugins.
   - **Rationale:** A single extension point prevents divergence between internal and external
     editors.
   - **Verification:** Verify that the animation editor loads through the plugin API.

4. **R-15.20.4** — The engine **SHALL** support plugin hot-reload preserving open panels, inspector
   state, and undo history via reflection-based state serialization.
   - **Rationale:** Hot-reload accelerates plugin development.
   - **Verification:** Modify a plugin, trigger reload, and verify panel state and undo history are
     preserved.

5. **R-15.20.5** — The engine **SHALL** resolve plugin dependency graphs from manifest declarations
   using semantic version ranges, detecting cycles and conflicts.
   - **Rationale:** Dependency resolution prevents load-order errors and version conflicts.
   - **Verification:** Declare conflicting version ranges and verify the host reports the conflict.

6. **R-15.20.6** — The engine **SHALL** support plugin publication to and installation from the
   asset marketplace with automatic update notifications.
   - **Rationale:** Marketplace distribution enables the plugin ecosystem.
   - **Verification:** Install a plugin from the marketplace and verify an update notification when
     a new version is available.

7. **R-15.20.7** — The engine **SHALL** support no-code plugin creation using logic graphs and the
   UI editor, packaged as standard editor plugins.
   - **Rationale:** No-code plugin creation empowers designers to build custom tools.
   - **Verification:** Create a no-code plugin and verify it loads and hot-reloads like a native
     plugin.

8. **R-15.20.8** — The engine **SHALL** provide editor-specific logic graph nodes for panels, menus,
   gizmos, and undo commands with editor lifecycle event-driven execution.
   - **Rationale:** Editor nodes are required for no-code plugin behavior authoring.
   - **Verification:** Create a graph responding to selection change and verify it fires when
     selection changes.

9. **R-15.20.9** — The engine **SHALL** support visual UI authoring for plugin panels with property
   bindings and editor theme inheritance.
   - **Rationale:** Consistent UI authoring ensures plugins blend with the built-in editor.
   - **Verification:** Create a plugin panel and verify it inherits the current editor theme.

10. **R-15.20.10** — The engine **SHALL** support hot module reload completing within 2 seconds for
    both no-code and native plugins in debug and release builds.
    - **Rationale:** Fast reload is essential for productive plugin iteration.
    - **Verification:** Modify a plugin and verify reload completes within 2 seconds.
