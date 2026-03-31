# R-12.7 Content Plugin System

## Content Plugin System

1. **R-12.7.1** — The engine **SHALL** support content plugins as self-contained packages of assets,
   logic graphs, components, and data that extend game functionality without native Rust code. The
   plugin host **SHALL** validate manifests, resolve dependencies, and activate plugins in
   dependency order.
   - **Rationale:** Content plugins enable designers, modders, and asset store developers to add
     gameplay content without requiring Rust compilation or engine source access.
   - **Verification:** Load a content plugin containing a custom component and logic graph; spawn an
     entity with the plugin's component and verify the graph executes correctly; unload the plugin
     and verify cleanup.
2. **R-12.7.1a** — Content plugins **SHALL** participate in all standard engine systems (rendering,
   physics, audio, UI) through the ECS world. Plugin-defined components **SHALL** be
   indistinguishable from native components at runtime.
   - **Rationale:** Full engine integration ensures content plugins are not second-class citizens
     with limited capabilities.
   - **Verification:** Create a content plugin with a renderable mesh, a physics collider, and an
     audio source; verify all three systems process the plugin's components correctly.

## Content Plugin Manifest

3. **R-12.7.2** — Content plugins **SHALL** declare a JSON manifest specifying: plugin ID, version,
   display name, author, description, dependencies, exported components, exported logic graph node
   types, exported assets, and entry-point logic graph. The engine **SHALL** validate the manifest
   schema on load and reject plugins with malformed or incomplete manifests.
   - **Rationale:** A declarative manifest enables automated validation, dependency resolution, and
     marketplace integration without executing plugin code.
   - **Verification:** Load a valid manifest and verify all fields parse correctly; load manifests
     with missing required fields, invalid versions, and malformed JSON and verify each is rejected
     with a specific error message.
4. **R-12.7.2a** — Manifest JSON **SHALL** have keys sorted in lexicographical order and conform to
   the engine's JSON configuration conventions (F-15.1.7).
   - **Rationale:** Consistent JSON formatting enables reliable diffing and version control of
     manifest files.
   - **Verification:** Generate a manifest and verify key ordering is lexicographical.

## Content Plugin Hot Reload

5. **R-12.7.3** — The engine **SHALL** hot-reload content plugins at runtime when source assets or
   logic graphs change, re-importing affected assets, recompiling logic graphs, and patching running
   instances without restarting. Component data **SHALL** be preserved via reflection serialization
   when the schema is compatible.
   - **Rationale:** Hot reload enables iterative content authoring with immediate feedback,
     eliminating restart latency during plugin development.
   - **Verification:** Load a content plugin, modify a logic graph, trigger hot reload, and verify
     updated behavior within 2 seconds; verify component data is preserved; modify the schema and
     verify a clean reload occurs.
6. **R-12.7.3a** — Content plugin hot reload **SHALL** be available only on desktop platforms during
   development. Shipping builds on console and mobile **SHALL NOT** support content plugin hot
   reload.
   - **Rationale:** Console and mobile shipping builds require signed, immutable content for
     certification compliance.
   - **Verification:** Attempt hot reload on a simulated shipping build and verify it is rejected.

## Content Plugin Sandboxing

7. **R-12.7.4** — Content plugins **SHALL** run in a restricted execution environment with no direct
   filesystem access, no network access, and access limited to the component types declared in their
   manifest. The sandbox **SHALL** enforce memory limits and per-frame execution time limits.
   Sandbox violations **SHALL** be logged and the offending plugin deactivated.
   - **Rationale:** Sandboxing protects players from malicious or buggy content plugins and is
     required for console certification.
   - **Verification:** Create a plugin that attempts filesystem access, network access, and
     undeclared component access; verify each is blocked, logged, and the plugin deactivated after
     repeated violations.
8. **R-12.7.4a** — Sandbox enforcement **SHALL** be consistent across all platforms. Console
   certification requirements for content isolation **SHALL** be met by the sandbox.
   - **Rationale:** Cross-platform consistency prevents security gaps on any target platform.
   - **Verification:** Run the same sandbox violation test suite on each platform and verify
     identical enforcement.

## Content Plugin Packaging

9. **R-12.7.5** — Content plugins **SHALL** be exportable as distributable packages for the asset
   marketplace (F-15.17.1) including manifest, all referenced assets, compiled logic graphs,
   metadata, thumbnails, and documentation. The packaging tool **SHALL** validate completeness and
   generate a content hash for integrity verification.
   - **Rationale:** Validated, integrity-checked packages prevent incomplete or tampered plugins
     from reaching users.
   - **Verification:** Package a content plugin; verify all referenced assets are included; verify
     the content hash matches after transfer; intentionally omit an asset and verify the packaging
     tool rejects the build.
10. **R-12.7.5a** — Content plugin packages **SHALL** be platform-independent. Runtime asset cooking
    for each target platform **SHALL** occur during installation, not packaging.
    - **Rationale:** Platform-independent packages reduce marketplace storage and enable a single
      upload to serve all platforms.
    - **Verification:** Package on one platform, install on a different platform, and verify asset
      cooking produces correct platform-specific assets.

## Content Plugin Dependencies

11. **R-12.7.6** — Content plugins **SHALL** be able to depend on other content plugins. The engine
    **SHALL** resolve the dependency graph at load time, detect cycles and version conflicts, load
    plugins in topological order, and produce error messages with installation guidance for missing
    dependencies. Version constraints **SHALL** use semantic versioning ranges.
    - **Rationale:** Dependency resolution prevents load-order bugs and enables modular content
      plugin ecosystems.
    - **Verification:** Declare a circular dependency and verify the engine reports a cycle error;
      declare a missing dependency and verify the error includes installation guidance; load a valid
      dependency chain and verify topological order.
12. **R-12.7.6a** — The dependency resolver **SHALL** use the same algorithm as editor plugin
    dependencies (F-15.20.5) and select the highest compatible version when multiple versions are
    available.
    - **Rationale:** A shared resolver algorithm reduces implementation cost and ensures consistent
      behavior.
    - **Verification:** Provide two content plugin versions and verify the resolver selects the
      highest compatible version.
