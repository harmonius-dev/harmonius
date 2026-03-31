# 12.7 -- Content Plugin System

## Content Plugins

| ID       | Feature                     |
|----------|-----------------------------|
| F-12.7.1 | Content Plugin System       |
| F-12.7.2 | Content Plugin Manifest     |
| F-12.7.3 | Content Plugin Hot Reload   |
| F-12.7.4 | Content Plugin Sandboxing   |
| F-12.7.5 | Content Plugin Packaging    |
| F-12.7.6 | Content Plugin Dependencies |

1. **F-12.7.1** — A content plugin is a self-contained package of assets, logic graphs, components,
   and data that extends game functionality without native Rust code. Content plugins are loaded by
   the engine at runtime, registered in the ECS world, and participate in all standard engine
   systems (rendering, physics, audio, UI). The plugin host validates manifests, resolves
   dependencies, and activates plugins in dependency order. require content signing.
   - **Deps:** F-1.1.1, F-15.8.1, F-7.6.1, F-12.2.8
   - **Platform:** Supported on all platforms (desktop, mobile, console). Console platforms
2. **F-12.7.2** — A declarative JSON manifest defining: plugin ID, version, display name, author,
   description, dependencies (other content plugins and minimum engine version), exported
   components, exported logic graph node types, exported assets (with relative paths), and
   entry-point logic graph. The engine validates the manifest schema on load and rejects plugins
   with malformed or incomplete manifests. sorted lexicographically.
   - **Deps:** F-12.7.1
   - **Platform:** JSON format consistent with engine configuration conventions (F-15.1.7). Keys
3. **F-12.7.3** — Hot-reload content plugins at runtime. When a content plugin's source assets or
   logic graphs change, the engine re-imports affected assets, recompiles logic graphs, and patches
   running plugin instances without restarting the game or editor. Component data is preserved via
   reflection serialization when the schema is compatible. Incompatible schema changes trigger a
   clean reload of the affected plugin. builds.
   - **Deps:** F-12.7.1, F-12.4.1, F-12.4.2, F-12.4.4, F-7.6.1
   - **Platform:** Desktop only during development. Not available on console or mobile shipping
4. **F-12.7.4** — Content plugins run in a restricted execution environment. Plugins have no direct
   filesystem access, no network access, and are limited to the component types declared in their
   manifest. The sandbox enforces memory limits, execution time limits per frame, and prevents
   access to engine internals outside the declared capability set. Sandbox violations are logged and
   the offending plugin is deactivated. requires sandbox isolation.
   - **Deps:** F-12.7.1, F-12.7.2
   - **Platform:** Sandbox enforcement is consistent across all platforms. Console certification
5. **F-12.7.5** — Export content plugins as distributable packages for the asset marketplace
   (F-15.17.1). Packages include the manifest, all referenced assets, compiled logic graphs,
   metadata (author, license, tags), thumbnail images, and documentation. The packaging tool
   validates completeness (no missing asset references), generates a content hash for integrity
   verification, and compresses the package. install for each target platform.
   - **Deps:** F-12.7.1, F-12.7.2, F-15.17.1
   - **Platform:** Package format is platform-independent. Runtime asset cooking occurs on
6. **F-12.7.6** — Content plugins can depend on other content plugins. The engine resolves the
   dependency graph at load time, detects cycles and version conflicts, and loads plugins in
   topological order. Missing dependencies produce error messages with installation guidance.
   Version constraints use semantic versioning ranges consistent with editor plugin dependencies
   (F-15.20.5). (F-15.20.5).
   - **Deps:** F-12.7.1, F-12.7.2, F-15.20.5
   - **Platform:** Dependency resolution uses the same algorithm as editor plugin dependencies
