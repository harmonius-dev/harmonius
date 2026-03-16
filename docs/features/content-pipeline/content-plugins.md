# 12.7 -- Content Plugin System

## Content Plugins

### F-12.7.1 Content Plugin System

A content plugin is a self-contained package of assets, logic graphs, components, and data that
extends game functionality without native Rust code. Content plugins are loaded by the engine at
runtime, registered in the ECS world, and participate in all standard engine systems (rendering,
physics, audio, UI). The plugin host validates manifests, resolves dependencies, and activates
plugins in dependency order.

- **Requirements:** R-12.7.1
- **Dependencies:** F-1.1.1, F-15.8.1, F-7.6.1, F-12.2.8
- **Platform notes:** Supported on all platforms (desktop, mobile, console). Console platforms
  require content signing.

### F-12.7.2 Content Plugin Manifest

A declarative JSON manifest defining: plugin ID, version, display name, author, description,
dependencies (other content plugins and minimum engine version), exported components, exported logic
graph node types, exported assets (with relative paths), and entry-point logic graph. The engine
validates the manifest schema on load and rejects plugins with malformed or incomplete manifests.

- **Requirements:** R-12.7.2
- **Dependencies:** F-12.7.1
- **Platform notes:** JSON format consistent with engine configuration conventions (F-15.1.7). Keys
  sorted lexicographically.

### F-12.7.3 Content Plugin Hot Reload

Hot-reload content plugins at runtime. When a content plugin's source assets or logic graphs change,
the engine re-imports affected assets, recompiles logic graphs, and patches running plugin instances
without restarting the game or editor. Component data is preserved via reflection serialization when
the schema is compatible. Incompatible schema changes trigger a clean reload of the affected plugin.

- **Requirements:** R-12.7.3
- **Dependencies:** F-12.7.1, F-12.4.1, F-12.4.2, F-12.4.4, F-7.6.1
- **Platform notes:** Desktop only during development. Not available on console or mobile shipping
  builds.

### F-12.7.4 Content Plugin Sandboxing

Content plugins run in a restricted execution environment. Plugins have no direct filesystem access,
no network access, and are limited to the component types declared in their manifest. The sandbox
enforces memory limits, execution time limits per frame, and prevents access to engine internals
outside the declared capability set. Sandbox violations are logged and the offending plugin is
deactivated.

- **Requirements:** R-12.7.4
- **Dependencies:** F-12.7.1, F-12.7.2
- **Platform notes:** Sandbox enforcement is consistent across all platforms. Console certification
  requires sandbox isolation.

### F-12.7.5 Content Plugin Packaging

Export content plugins as distributable packages for the asset marketplace (F-15.17.1). Packages
include the manifest, all referenced assets, compiled logic graphs, metadata (author, license,
tags), thumbnail images, and documentation. The packaging tool validates completeness (no missing
asset references), generates a content hash for integrity verification, and compresses the package.

- **Requirements:** R-12.7.5
- **Dependencies:** F-12.7.1, F-12.7.2, F-15.17.1
- **Platform notes:** Package format is platform-independent. Runtime asset cooking occurs on
  install for each target platform.

### F-12.7.6 Content Plugin Dependencies

Content plugins can depend on other content plugins. The engine resolves the dependency graph at
load time, detects cycles and version conflicts, and loads plugins in topological order. Missing
dependencies produce error messages with installation guidance. Version constraints use semantic
versioning ranges consistent with editor plugin dependencies (F-15.20.5).

- **Requirements:** R-12.7.6
- **Dependencies:** F-12.7.1, F-12.7.2, F-15.20.5
- **Platform notes:** Dependency resolution uses the same algorithm as editor plugin dependencies
  (F-15.20.5).
