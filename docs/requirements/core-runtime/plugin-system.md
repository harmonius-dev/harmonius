# R-1.6 — Plugin System Requirements

## Module Registration

| ID      | Derived From                                            |
|---------|---------------------------------------------------------|
| R-1.6.1 | [F-1.6.1](../../features/core-runtime/plugin-system.md) |
| R-1.6.2 | [F-1.6.2](../../features/core-runtime/plugin-system.md) |

1. **R-1.6.1** — The engine **SHALL** provide a plugin trait that declares the systems, components,
   resources, and events a module contributes, with registration order determined automatically from
   declared dependencies rather than call order.
   - **Rationale:** Automatic ordering from dependencies prevents subtle initialization bugs caused
     by incorrect manual registration sequences.
   - **Verification:** Unit test: register 3 plugins in reverse dependency order. Verify the engine
     initializes them in correct dependency order. Verify all declared systems, components, and
     resources are present in the world after initialization.
2. **R-1.6.2** — The engine **SHALL** support named plugin groups (e.g., `DefaultPlugins`,
   `ServerPlugins`) that register a curated set of plugins with a single call, with the ability to
   disable individual plugins within a group before registration.
   - **Rationale:** Groups simplify application setup and enable shipping different plugin
     configurations for client, server, and headless simulation.
   - **Verification:** Unit test: create a group of 5 plugins, disable 1, register the group. Verify
     4 plugins are active and the disabled plugin's systems are absent. Verify `ServerPlugins`
     excludes rendering plugins while `DefaultPlugins` includes them.

## Dependency Declaration

| ID       | Derived From                                            |
|----------|---------------------------------------------------------|
| R-1.6.3  | [F-1.6.3](../../features/core-runtime/plugin-system.md) |
| R-1.6.4  | [F-1.6.4](../../features/core-runtime/plugin-system.md) |
| R-1.6.4a | [F-1.6.4](../../features/core-runtime/plugin-system.md) |

1. **R-1.6.3** — The engine **SHALL** require plugins to declare their dependencies and conflicts,
   validating the dependency graph at startup and reporting missing dependencies and conflicts as
   actionable errors before any systems run.
   - **Rationale:** Early validation prevents subtle runtime failures from unmet assumptions about
     available components or resources.
   - **Verification:** Unit test: register a plugin that declares a dependency on an unregistered
     plugin. Verify startup fails with an error message naming the missing dependency. Register two
     plugins that declare mutual conflict and verify startup fails with a conflict error.
2. **R-1.6.4** — The engine **SHALL** resolve plugin initialization order by topologically sorting
   the dependency graph, detecting circular dependencies at startup and supporting late-registering
   plugins with re-validation.
   - **Rationale:** Topological sorting guarantees dependencies are initialized before dependents,
     preventing use-before-init errors.
   - **Verification:** Unit test: register plugins A->B->C (C depends on B, B depends on A). Verify
     initialization order is A, B, C. Create a circular dependency A->B->A and verify a cycle error
     is reported at startup with the cycle path.
3. **R-1.6.4a** — All plugin graph validation errors (missing dependencies, conflicts, cycles)
   **SHALL** include the full dependency chain leading to the error, the names of all involved
   plugins, and a suggested resolution. Validation **SHALL** report all errors in a single pass
   rather than stopping at the first error.
   - **Rationale:** Developers configuring plugin sets need actionable diagnostics to resolve issues
     quickly, especially when multiple errors exist simultaneously.
   - **Verification:** Register plugins with 3 simultaneous issues (missing dep, conflict, cycle).
     Verify all 3 errors are reported in a single validation pass with full chain context.

## Hot Reload Support

| ID       | Derived From                                            |
|----------|---------------------------------------------------------|
| R-1.6.5  | [F-1.6.5](../../features/core-runtime/plugin-system.md) |
| R-1.6.5a | [F-1.6.5](../../features/core-runtime/plugin-system.md) |

1. **R-1.6.5** — The engine **SHALL** support unloading and reloading gameplay plugins at runtime
   during development, preserving ECS world state across reloads and handling layout changes through
   the serialization and reflection systems.
   - **Rationale:** Hot reload accelerates iteration by avoiding full restart cycles when modifying
     gameplay logic.
   - **Verification:** Integration test: load a plugin that registers a system writing component A.
     Run one frame. Modify the plugin to change the system behavior. Hot-reload the plugin. Verify
     ECS world state (entities, component values) survives the reload. Verify the new system
     behavior is active after reload.
2. **R-1.6.5a** — Hot reload cycle time (unload, compile, reload, re-register) **SHALL** complete in
   under 2 seconds for a plugin with up to 100 systems, excluding user compile time. The engine
   **SHALL** preserve 100% of ECS entity and component state across a reload. If state migration
   fails for any component, the engine **SHALL** report the failing type and retain the pre-reload
   value rather than losing data.
   - **Rationale:** Fast reload cycles are the primary value proposition; data loss during reload
     undermines developer trust in the workflow.
   - **Verification:** Benchmark: hot-reload a 50-system plugin and verify total cycle time. Create
     entities with 10 component types, reload, and verify all entity data survives. Introduce a
     layout change that fails migration for one type and verify the error is reported with the
     pre-reload value retained.

## API Stability Contracts

| ID      | Derived From                                            |
|---------|---------------------------------------------------------|
| R-1.6.6 | [F-1.6.6](../../features/core-runtime/plugin-system.md) |
| R-1.6.7 | [F-1.6.7](../../features/core-runtime/plugin-system.md) |

1. **R-1.6.6** — The engine **SHALL** embed semantic version metadata in each plugin's registration
   descriptor and compare ABI hashes at load time for dynamically loaded plugins, rejecting binaries
   compiled against incompatible engine versions.
   - **Rationale:** Loading plugins compiled against a different ABI causes memory corruption and
     undefined behavior that must be prevented at load time.
   - **Verification:** Unit test: load a plugin with a matching ABI hash and verify success. Load a
     plugin with a mismatched ABI hash and verify it is rejected with an error message specifying
     the version mismatch.
2. **R-1.6.7** — The engine **SHALL** allow plugins to advertise named capabilities with associated
   versions at registration time, and provide a query API for systems to check capability presence
   and version at runtime.
   - **Rationale:** Capability negotiation decouples optional integrations (physics, audio,
     networking) from core engine code without compile-time feature flags.
   - **Verification:** Unit test: register a plugin advertising capability `"physics"` at version
     `1.2`. Query for `"physics"` and verify it returns version `1.2`. Query for `"audio"` and
     verify it returns not-available. Register a second plugin that branches on `"physics"` presence
     and verify the correct branch executes.
