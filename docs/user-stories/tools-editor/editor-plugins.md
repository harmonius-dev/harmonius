# User Stories: Editor Plugin Architecture

## F-15.20.1 Plugin Architecture

## US-15.20.1.1 Developer Loads Custom Plugin

**As a** developer (P-15), **I want** to place a compiled plugin dynamic library in the project
plugin directory and have the editor discover and load it automatically, **so that** I can extend
the editor without modifying engine source code.

## US-15.20.1.2 Extension Dev Targets Stable ABI

**As an** extension developer (P-25), **I want** the plugin API to expose a versioned C ABI boundary
through Rust wrapper types, **so that** my plugins remain compatible across minor engine versions
without recompilation.

## US-15.20.1.3 Engine Dev Validates ABI Compatibility

**As an** engine developer (P-26), **I want** the plugin host to validate ABI compatibility before
loading a plugin, **so that** binary-incompatible plugins are rejected with a clear error instead of
causing undefined behavior.

## US-15.20.1.4 Developer Requests Capabilities

**As a** developer (P-15), **I want** my plugin to declare which editor subsystems it needs access
to via a capability-based API, **so that** the plugin host grants only the minimum required
permissions.

## US-15.20.1.5 Engine Dev Isolates Plugin Crashes

**As an** engine developer (P-26), **I want** a crashing plugin to be caught and unloaded without
bringing down the editor, **so that** unstable third-party plugins do not compromise editor
reliability.

## US-15.20.1.6 Tech Artist Adds Custom Panel via Plugin

**As a** tech artist (P-13), **I want** to create a plugin that adds a custom panel, toolbar button,
and context menu action, **so that** I can build studio-specific tools accessible from the editor
UI.

## US-15.20.1.7 Designer Sees Plugin Error Messages

**As a** designer (P-5), **I want** clear error messages when a plugin fails to load (wrong version,
missing dependency, crash), **so that** I can report the issue to the plugin developer with
actionable information.

## F-15.20.2 Custom Component Editors

## US-15.20.2.1 Developer Registers Custom Widget

**As a** developer (P-15), **I want** to register a custom editor widget for my plugin-defined
component type via the reflection system, **so that** the inspector renders my custom editor instead
of the default property grid.

## US-15.20.2.2 Designer Edits Plugin Components

**As a** designer (P-5), **I want** to select an entity with a plugin-defined component and see a
custom editor tailored to that component's data, **so that** I can edit complex plugin data without
deciphering raw property values.

## US-15.20.2.3 Tech Artist Gets Undo Integration

**As a** tech artist (P-13), **I want** custom component editors to integrate with the undo/redo
stack automatically, **so that** edits made through plugin editors are undoable like any other
editor operation.

## US-15.20.2.4 Engine Dev Ensures Fallback Grid

**As an** engine developer (P-26), **I want** the inspector to fall back to the default
reflection-based property grid when no custom editor is registered, **so that** all components
remain editable regardless of plugin presence.

## US-15.20.2.5 Extension Dev Previews Component

**As an** extension developer (P-25), **I want** my custom editor to receive read/write access to
the component data through the reflection API, **so that** I can build rich editing interfaces with
validated input.

## F-15.20.3 In-Engine Feature Editors

## US-15.20.3.1 Engine Dev Registers Internal Editor

**As an** engine developer (P-26), **I want** engine subsystems (animation, physics, audio, AI, VFX)
to register their editors through the same plugin API as third-party plugins, **so that** there is a
single extension mechanism for all editors.

## US-15.20.3.2 Tech Artist Extends Internal Editor

**As a** tech artist (P-13), **I want** in-engine editors to be extensible by plugins, **so that** I
can add custom panels or features to the built-in animation or material editors.

## US-15.20.3.3 Developer Sees Consistent API

**As a** developer (P-15), **I want** built-in and third-party editors to use the same registration,
lifecycle, and rendering APIs, **so that** learning the plugin API once covers all editor extension
scenarios.

## US-15.20.3.4 Extension Dev Reads Internal Source

**As an** extension developer (P-25), **I want** in-engine feature editors to serve as reference
implementations of the plugin API, **so that** I can study how official editors are built when
writing my own.

## US-15.20.3.5 Designer Benefits from Unified Editors

**As a** designer (P-5), **I want** all editors to behave consistently regardless of whether they
are built-in or from a plugin, **so that** I have a uniform experience across the editor.

## F-15.20.4 Plugin Hot-Reload

## US-15.20.4.1 Tech Artist Iterates Without Restart

**As a** tech artist (P-13), **I want** to recompile my plugin and have the editor hot-reload it
automatically without restarting, **so that** I can iterate on custom tools in seconds.

## US-15.20.4.2 Developer Preserves Panel State

**As a** developer (P-15), **I want** hot-reload to preserve open panels, inspector state, and undo
history, **so that** I do not lose my editing context when reloading a plugin.

## US-15.20.4.3 Engine Dev Watches Build Output

**As an** engine developer (P-26), **I want** a file watcher monitoring plugin build output
directories that triggers reload automatically, **so that** reloading does not require manual
commands.

## US-15.20.4.4 Extension Dev Tests Release Restriction

**As an** extension developer (P-25), **I want** hot-reload to be restricted to debug builds,
**so that** release builds do not risk state corruption from library swapping.

## US-15.20.4.5 Designer Sees Reload Notification

**As a** designer (P-5), **I want** a notification when a plugin is hot-reloaded showing what
changed, **so that** I know my tools have been updated and can verify the new behavior.

## F-15.20.5 Plugin Dependency Management

## US-15.20.5.1 Extension Dev Declares Dependencies

**As an** extension developer (P-25), **I want** to declare dependencies on other plugins, engine
subsystems, and minimum engine version in a JSON manifest, **so that** the plugin host resolves and
loads dependencies in the correct order.

## US-15.20.5.2 Developer Gets Clear Dependency Errors

**As a** developer (P-15), **I want** clear error messages for missing dependencies including
installation guidance, **so that** I can resolve dependency issues without guessing what is needed.

## US-15.20.5.3 Engine Dev Detects Cycles

**As an** engine developer (P-26), **I want** the plugin host to detect circular dependencies and
report them as errors, **so that** plugins with impossible dependency graphs are rejected before
loading.

## US-15.20.5.4 Extension Dev Uses Semver Ranges

**As an** extension developer (P-25), **I want** to specify semantic versioning ranges for
dependencies (e.g., ">=1.2.0, <2.0.0"), **so that** my plugin works with compatible versions without
pinning to exact releases.

## US-15.20.5.5 Tech Artist Installs Dependencies

**As a** tech artist (P-13), **I want** the editor to automatically resolve and load plugin
dependencies in topological order, **so that** I do not need to manually manage plugin load order.

## F-15.20.6 Plugin Marketplace Integration

## US-15.20.6.1 Designer Installs Plugin from Store

**As a** designer (P-5), **I want** to browse plugins in the asset marketplace and install them with
one click, **so that** I can extend the editor without manual file management.

## US-15.20.6.2 Extension Dev Publishes to Marketplace

**As an** extension developer (P-25), **I want** to publish my plugin to the asset marketplace with
metadata (supported engine versions, dependencies, screenshots), **so that** users can discover and
install my plugin.

## US-15.20.6.3 Developer Gets Update Notifications

**As a** developer (P-15), **I want** automatic notifications when a newer compatible version of an
installed plugin is available, **so that** I stay current with bug fixes and new features.

## US-15.20.6.4 Tech Artist Resolves Marketplace Deps

**As a** tech artist (P-13), **I want** marketplace plugin install to resolve and download
transitive dependencies automatically, **so that** I do not need to hunt down required sub-plugins
manually.

## US-15.20.6.5 Engine Dev Validates Marketplace Compat

**As an** engine developer (P-26), **I want** marketplace listings to display compatibility badges
per engine version, **so that** users only install plugins verified to work with their engine
version.

## F-15.20.7 No-Code Editor Plugin Creation

## US-15.20.7.1 Tech Artist Creates Plugin Without Code

**As a** technical artist (P-13), **I want** to create a custom material preview panel as an editor
plugin using only logic graphs and the visual UI editor, **so that** I can build studio-specific
tools without writing Rust code.

## US-15.20.7.2 Extension Dev Scaffolds No-Code Plugin

**As an** extension developer (P-25), **I want** a plugin project template that scaffolds a
manifest, entry-point logic graph, and UI layout, **so that** I can start building a no-code plugin
without manual boilerplate.

## US-15.20.7.3 Designer Creates Plugin From Template

**As a** designer (P-5), **I want** to create a new editor plugin project from a template in the
project creation wizard, **so that** I can start extending the editor without needing an engineer to
set up the project structure.

## US-15.20.7.4 Modder Packages No-Code Plugin

**As a** modder (P-24), **I want** my no-code editor plugin to be packaged as a standard plugin
distributable on the marketplace, **so that** other users can install and use my custom editor
tools.

## US-15.20.7.5 Tech Artist Publishes No-Code Plugin

**As a** technical artist (P-13), **I want** to publish my no-code plugin to the asset marketplace
alongside native plugins, **so that** users cannot distinguish between no-code and native plugins
from the consumer side.

## F-15.20.8 Plugin Logic Graphs

## US-15.20.8.1 Extension Dev Builds Terrain Brush

**As an** extension developer (P-25), **I want** to build a terrain brush plugin using
editor-specific logic graph nodes (register gizmo, respond to selection change, read/write component
data), **so that** I can author interactive editor tools visually.

## US-15.20.8.2 Tech Artist Adds Menu Item via Graph

**As a** technical artist (P-13), **I want** to use an "add menu item" node in my plugin logic graph
to register a custom menu action, **so that** users can invoke my tool from the editor menu bar.

## US-15.20.8.3 Designer Responds to Selection Change

**As a** designer (P-5), **I want** my plugin logic graph to respond to entity selection change
events, **so that** my custom panel updates its contents when I select a different entity in the
viewport.

## US-15.20.8.4 Modder Creates Custom Gizmo

**As a** modder (P-24), **I want** to use a "register gizmo" node in a logic graph to add a custom
3D manipulation gizmo to the viewport, **so that** I can create specialized editing handles for my
mod's content.

## US-15.20.8.5 Extension Dev Queries Asset Database

**As an** extension developer (P-25), **I want** to use an "asset database query" node in my plugin
logic graph to search and filter project assets, **so that** my plugin can present asset lists and
perform batch operations.

## F-15.20.9 Plugin UI Authoring

## US-15.20.9.1 Designer Creates Custom Inspector

**As a** designer (P-5), **I want** to create a custom inspector for a gameplay component using
drag-and-drop in the visual UI editor, **so that** I can tailor the inspector to show only the
fields I need with appropriate widgets.

## US-15.20.9.2 Tech Artist Builds Floating Tool Window

**As a** technical artist (P-13), **I want** to define a floating tool window in the visual UI
editor with property bindings to my plugin's logic graph, **so that** I can build interactive tool
panels with live data.

## US-15.20.9.3 Extension Dev Previews Plugin UI Live

**As an** extension developer (P-25), **I want** live preview of my plugin UI inside the editor
while I author it, **so that** I can see layout and styling changes immediately without reloading.

## US-15.20.9.4 Modder Adds Toolbar Button

**As a** modder (P-24), **I want** to add a toolbar button to the editor using the visual UI editor,
**so that** users can launch my mod's editor panel with a single click.

## US-15.20.9.5 Designer Inherits Editor Theme

**As a** designer (P-5), **I want** my plugin UI to automatically inherit the current editor theme,
**so that** my custom panels match the look and feel of the rest of the editor.

## F-15.20.10 Plugin Hot Module Reload

## US-15.20.10.1 Tech Artist Reloads Plugin While Editing

**As a** technical artist (P-13), **I want** to modify my no-code plugin's logic graph and see the
updated behavior in the editor within 2 seconds without restarting, **so that** I can iterate on
custom tools rapidly.

## US-15.20.10.2 Extension Dev Preserves State on Reload

**As an** extension developer (P-25), **I want** hot module reload to preserve my plugin's open
panels, undo history, and inspector state, **so that** I do not lose my editing context when
reloading.

## US-15.20.10.3 Designer Gets Reload Notification

**As a** designer (P-5), **I want** a notification when a plugin is hot-reloaded showing what
changed, **so that** I know my tools have been updated and can verify the new behavior.

## US-15.20.10.4 Modder Reloads in Release Build

**As a** modder (P-24), **I want** plugin hot module reload to work in release builds, **so that** I
can iterate on my mod's editor tools without switching to a debug build.

## US-15.20.10.5 Extension Dev Auto-Reload on Save

**As an** extension developer (P-25), **I want** a file watcher to detect changes to my plugin
source files and trigger reload automatically, **so that** I do not need to manually invoke reload
after every edit.
