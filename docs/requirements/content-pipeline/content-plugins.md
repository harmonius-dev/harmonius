# R-12.7 -- Content Plugin System Requirements

## Content Plugin System

| ID        | Derived From                                                   |
|-----------|----------------------------------------------------------------|
| R-12.7.1  | [F-12.7.1](../../features/content-pipeline/content-plugins.md) |
| R-12.7.1a | [F-12.7.1](../../features/content-pipeline/content-plugins.md) |

1. **R-12.7.1** — The engine **SHALL** support content plugins — self-contained packages of assets,
   logic graphs, components, and data that extend game functionality without native Rust code. The
   plugin host **SHALL** validate manifests, resolve dependencies, and activate plugins in
   dependency order.
   - **Rationale:** Content plugins enable designers, modders, and asset store developers to add
     gameplay content without requiring Rust compilation or engine source access.
   - **Verification:** Integration test: load a content plugin containing a custom component and
     logic graph. Spawn an entity with the plugin's component and verify the logic graph executes
     correctly. Unload the plugin and verify its components are cleaned up.
2. **R-12.7.1a** — Content plugins **SHALL** participate in all standard engine systems (rendering,
   physics, audio, UI) through the ECS world. Plugin-defined components **SHALL** be
   indistinguishable from native components at runtime.
   - **Rationale:** Full engine integration ensures content plugins are not second-class citizens
     with limited capabilities.
   - **Verification:** Integration test: create a content plugin with a renderable mesh component, a
     physics collider, and an audio source. Verify all three systems process the plugin's components
     correctly.

## Content Plugin Manifest

| ID        | Derived From                                                   |
|-----------|----------------------------------------------------------------|
| R-12.7.2  | [F-12.7.2](../../features/content-pipeline/content-plugins.md) |
| R-12.7.2a | [F-12.7.2](../../features/content-pipeline/content-plugins.md) |

1. **R-12.7.2** — Content plugins **SHALL** declare a JSON manifest specifying: plugin ID, version,
   display name, author, description, dependencies (other content plugins and minimum engine
   version), exported components, exported logic graph node types, exported assets, and entry-point
   logic graph. The engine **SHALL** validate the manifest schema on load and reject plugins with
   malformed or incomplete manifests.
   - **Rationale:** A declarative manifest enables automated validation, dependency resolution, and
     marketplace integration without executing plugin code.
   - **Verification:** Unit test: load a valid manifest and verify all fields parse correctly. Load
     manifests with missing required fields, invalid version strings, and malformed JSON, and verify
     each is rejected with a specific error message.
2. **R-12.7.2a** — Manifest JSON **SHALL** have keys sorted in lexicographical order and conform to
   the engine's JSON configuration conventions (F-15.1.7).
   - **Rationale:** Consistent JSON formatting enables reliable diffing and version control of
     manifest files.
   - **Verification:** Unit test: generate a manifest and verify key ordering is lexicographical.

## Content Plugin Hot Reload

| ID        | Derived From                                                   |
|-----------|----------------------------------------------------------------|
| R-12.7.3  | [F-12.7.3](../../features/content-pipeline/content-plugins.md) |
| R-12.7.3a | [F-12.7.3](../../features/content-pipeline/content-plugins.md) |

1. **R-12.7.3** — The engine **SHALL** hot-reload content plugins at runtime when source assets or
   logic graphs change, re-importing affected assets, recompiling logic graphs, and patching running
   instances without restarting the game or editor. Component data **SHALL** be preserved via
   reflection serialization when the schema is compatible.
   - **Rationale:** Hot reload enables iterative content authoring with immediate feedback,
     eliminating restart latency during plugin development.
   - **Verification:** Integration test: load a content plugin, modify one of its logic graphs,
     trigger hot reload, and verify the updated behavior takes effect within 2 seconds. Verify
     component data on existing entities is preserved. Modify the component schema and verify a
     clean reload occurs.
2. **R-12.7.3a** — Content plugin hot reload **SHALL** be available only on desktop platforms during
   development. Shipping builds on console and mobile **SHALL NOT** support content plugin hot
   reload.
   - **Rationale:** Console and mobile shipping builds require signed, immutable content for
     certification compliance.
   - **Verification:** Unit test: attempt content plugin hot reload on a simulated shipping build
     configuration and verify it is rejected.

## Content Plugin Sandboxing

| ID        | Derived From                                                   |
|-----------|----------------------------------------------------------------|
| R-12.7.4  | [F-12.7.4](../../features/content-pipeline/content-plugins.md) |
| R-12.7.4a | [F-12.7.4](../../features/content-pipeline/content-plugins.md) |

1. **R-12.7.4** — Content plugins **SHALL** run in a restricted execution environment with no direct
   filesystem access, no network access, and access limited to the component types declared in their
   manifest. The sandbox **SHALL** enforce memory limits and per-frame execution time limits.
   Sandbox violations **SHALL** be logged and the offending plugin **SHALL** be deactivated.
   - **Rationale:** Sandboxing protects players from malicious or buggy content plugins and is
     required for console certification.
   - **Verification:** Integration test: create a content plugin that attempts filesystem access,
     network access, and access to an undeclared component type. Verify each attempt is blocked,
     logged, and the plugin is deactivated after repeated violations.
2. **R-12.7.4a** — Sandbox enforcement **SHALL** be consistent across all platforms. Console
   certification requirements for content isolation **SHALL** be met by the sandbox implementation.
   - **Rationale:** Cross-platform consistency prevents security gaps on any target platform.
   - **Verification:** Integration test: run the same sandbox violation test suite on each supported
     platform and verify identical enforcement behavior.

## Content Plugin Packaging

| ID        | Derived From                                                   |
|-----------|----------------------------------------------------------------|
| R-12.7.5  | [F-12.7.5](../../features/content-pipeline/content-plugins.md) |
| R-12.7.5a | [F-12.7.5](../../features/content-pipeline/content-plugins.md) |

1. **R-12.7.5** — Content plugins **SHALL** be exportable as distributable packages for the asset
   marketplace (F-15.17.1) including the manifest, all referenced assets, compiled logic graphs,
   metadata, thumbnail images, and documentation. The packaging tool **SHALL** validate completeness
   and generate a content hash for integrity verification.
   - **Rationale:** Validated, integrity-checked packages prevent incomplete or tampered plugins
     from reaching users.
   - **Verification:** Integration test: package a content plugin, verify all referenced assets are
     included, verify the content hash matches after transfer, and verify installation from the
     package succeeds. Intentionally omit an asset reference and verify the packaging tool rejects
     the build.
2. **R-12.7.5a** — Content plugin packages **SHALL** be platform-independent. Runtime asset cooking
   for each target platform **SHALL** occur during installation, not during packaging.
   - **Rationale:** Platform-independent packages reduce marketplace storage and enable a single
     upload to serve all platforms.
   - **Verification:** Integration test: package a content plugin on one platform, install it on a
     different platform, and verify asset cooking produces correct platform-specific assets.

## Content Plugin Dependencies

| ID        | Derived From                                                   |
|-----------|----------------------------------------------------------------|
| R-12.7.6  | [F-12.7.6](../../features/content-pipeline/content-plugins.md) |
| R-12.7.6a | [F-12.7.6](../../features/content-pipeline/content-plugins.md) |

1. **R-12.7.6** — Content plugins **SHALL** be able to depend on other content plugins. The engine
   **SHALL** resolve the dependency graph at load time, detect cycles and version conflicts, load
   plugins in topological order, and produce error messages with installation guidance for missing
   dependencies. Version constraints **SHALL** use semantic versioning ranges.
   - **Rationale:** Dependency resolution prevents load-order bugs and enables modular content
     plugin ecosystems.
   - **Verification:** Unit test: declare a circular dependency and verify the engine reports a
     cycle error. Declare a missing dependency and verify the error includes installation guidance.
     Load plugins with a valid dependency chain and verify topological load order.
2. **R-12.7.6a** — The dependency resolver **SHALL** use the same algorithm as editor plugin
   dependencies (F-15.20.5) and select the highest compatible version when multiple versions are
   available.
   - **Rationale:** A shared resolver algorithm reduces implementation and maintenance cost and
     ensures consistent behavior.
   - **Verification:** Unit test: provide two content plugin versions and verify the resolver
     selects the highest compatible version.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/content-pipeline/content-plugins.md](../../user-stories/content-pipeline/content-plugins.md).
Requirements in this document are derived from those user stories.
