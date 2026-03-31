# R-1.6 — Plugin System Requirements

## Module Registration

1. **R-1.6.1** — The engine **SHALL** provide a plugin trait declaring contributed systems,
   components, resources, and events, with registration order determined from declared dependencies
   rather than call order.
   - **Rationale:** Automatic ordering prevents subtle initialization bugs from incorrect manual
     registration.
   - **Verification:** Register 3 plugins in reverse dependency order; verify engine initializes
     them in correct order. Verify all declared items present in world after initialization.
2. **R-1.6.2** — The engine **SHALL** support named plugin groups that register a curated set of
   plugins with a single call, with individual plugins disableable before registration.
   - **Rationale:** Groups simplify setup and enable different configurations for client, server,
     headless.
   - **Verification:** Create group of 5 plugins; disable 1; register group; verify 4 active and
     disabled plugin's systems absent.

## Dependency Declaration

1. **R-1.6.3** — The engine **SHALL** require plugins to declare dependencies and conflicts,
   validating the graph at startup and reporting missing dependencies and conflicts as actionable
   errors before any systems run.
   - **Rationale:** Early validation prevents subtle runtime failures from unmet assumptions.
   - **Verification:** Register plugin with missing dependency; verify startup error naming it.
     Register two conflicting plugins; verify conflict error.
2. **R-1.6.4** — The engine **SHALL** resolve plugin initialization order by topological sort,
   detecting circular dependencies at startup. Late-registering plugins **SHALL** trigger
   re-validation. All errors **SHALL** report the full dependency chain, involved plugin names, and
   suggested resolution in a single pass.
   - **Rationale:** Topological sorting guarantees correct initialization; actionable errors help
     resolve issues.
   - **Verification:** Register A->B->C; verify order A, B, C. Create cycle A->B->A; verify cycle
     error with path. Register plugins with 3 simultaneous issues; verify all reported in one pass.

## Hot Reload Support

1. **R-1.6.5** — The engine **SHALL** support unloading and reloading gameplay plugins at runtime,
   preserving ECS world state across reloads and handling layout changes through serialization and
   reflection. Uses dlopen/dlclose on POSIX and LoadLibrary/FreeLibrary on Windows.
   - **Rationale:** Hot reload accelerates iteration by avoiding full restart cycles.
   - **Verification:** Load plugin writing component A; run one frame; modify and hot-reload; verify
     entity state survives. Verify new system behavior active.
2. **R-1.6.6** — Hot reload cycle time (unload, compile, reload, re-register) **SHALL** complete in
   under 2 seconds for up to 100 systems (excluding user compile time). The engine **SHALL**
   preserve 100% of entity and component state. If migration fails for a component, report the type
   and retain the pre-reload value.
   - **Rationale:** Fast cycles and zero data loss are the value proposition of hot reload.
   - **Verification:** Hot-reload 50-system plugin; verify under 2 seconds. Create 10 component
     types; reload; verify all data survives. Introduce layout change failing migration; verify
     error with pre-reload value retained.

## API Stability Contracts

1. **R-1.6.7** — The engine **SHALL** embed semantic version metadata in each plugin descriptor and
   compare ABI hashes at load time for dynamic plugins, rejecting incompatible binaries.
   - **Rationale:** Loading plugins compiled against a different ABI causes memory corruption.
   - **Verification:** Load plugin with matching ABI hash; verify success. Load mismatched; verify
     rejection with version error.
2. **R-1.6.8** — The engine **SHALL** allow plugins to advertise named capabilities with versions at
   registration, and provide a query API for runtime capability checks.
   - **Rationale:** Capability negotiation decouples optional integrations without compile-time
     feature flags.
   - **Verification:** Register plugin with "physics" v1.2. Query "physics"; verify v1.2 returned.
     Query "audio"; verify not-available. Branch on presence; verify correct branch executes.
