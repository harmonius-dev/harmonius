# Plugin System User Stories

## Module Registration

## US-1.6.1 Register Plugins Declaratively With Automatic Ordering

**As an** engine developer (P-26), **I want** a plugin trait that declares contributed
systems, components, resources, and events with automatic ordering from dependencies,
**so that** plugin registration is deterministic regardless of call order.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Plugin trait declares systems, components, resources, events | F-1.6.1 | R-1.6.1 |
| Registration order from declared dependencies, not call order | F-1.6.1 | R-1.6.1 |
| Deterministic initialization guaranteed | F-1.6.1 | R-1.6.1 |

## US-1.6.2 Use Declarative Plugin Registration in the Visual Editor

**As a** designer (P-5), **I want** the visual editor to show which plugins are loaded and
what systems, components, and resources each contributes, **so that** I understand what
functionality is available and where it comes from.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Loaded plugins listed in editor | F-1.6.1 | R-1.6.1 |
| Per-plugin contribution summary (systems, components) | F-1.6.1 | R-1.6.1 |
| Plugin enable/disable visible in editor | F-1.6.1 | R-1.6.1 |

## US-1.6.3 Set Up Projects With Plugin Group Presets

**As a** game developer (P-15), **I want** named plugin groups like `DefaultPlugins` and
`ServerPlugins` that I can customize by disabling individual plugins, **so that** I can set
up client, server, and headless builds with a single registration call.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Named groups register curated plugin sets | F-1.6.2 | R-1.6.2 |
| Individual plugins disableable within groups | F-1.6.2 | R-1.6.2 |
| Different presets for client, server, headless | F-1.6.2 | R-1.6.2 |

## US-1.6.4 Verify Plugin Group Customization Works Correctly

**As an** engine tester (P-27), **I want** to verify that disabling individual plugins within
a group correctly removes their systems and components, **so that** customized plugin sets
produce the expected minimal configuration.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Disabled plugin's systems not registered | F-1.6.2 | R-1.6.2 |
| Disabled plugin's components not available | F-1.6.2 | R-1.6.2 |
| Remaining plugins function correctly | F-1.6.2 | R-1.6.2 |

## Dependency Declaration

## US-1.6.5 Validate Plugin Dependencies at Startup

**As a** game developer (P-15), **I want** plugins to declare dependencies and conflicts
validated at startup, **so that** missing dependencies are reported as actionable errors
before any systems run rather than causing subtle runtime failures.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Dependencies declared per plugin | F-1.6.3 | R-1.6.3 |
| Conflicts declared and checked | F-1.6.3 | R-1.6.3 |
| Missing dependencies reported as errors at startup | F-1.6.3 | R-1.6.3 |

## US-1.6.6 Resolve Plugin Load Order by Topological Sort

**As an** engine developer (P-26), **I want** plugin initialization order resolved by
topological sort with circular dependency detection, **so that** dependencies are always
initialized before dependents and cycles are caught at startup.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Topological sort of dependency graph | F-1.6.4 | R-1.6.4 |
| Circular dependencies detected and reported | F-1.6.4 | R-1.6.4 |
| Late-registering plugins inserted correctly | F-1.6.4 | R-1.6.4 |

## US-1.6.7 Verify Dependency Error Messages Are Actionable

**As a** QA engineer (P-19), **I want** to verify that missing dependency and circular
dependency errors include plugin names and specific missing items, **so that** developers can
quickly resolve configuration issues without debugging.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Missing dependency error names both plugins | F-1.6.3, F-1.6.4 | R-1.6.3, R-1.6.4 |
| Circular dependency error lists the cycle | F-1.6.4 | R-1.6.4 |
| Conflict errors identify conflicting plugins | F-1.6.3 | R-1.6.3 |

## Hot Reload Support

## US-1.6.8 Hot-Reload Gameplay Plugins Without Losing State

**As a** game developer (P-15), **I want** to hot-reload gameplay plugins during development
with ECS world state preserved across reloads, **so that** I can iterate on gameplay logic
without restarting the engine and losing my test scenario.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Plugin unloaded and reloaded at runtime | F-1.6.5 | R-1.6.5 |
| ECS world state preserved across reload | F-1.6.5 | R-1.6.5 |
| Systems torn down and re-registered from new version | F-1.6.5 | R-1.6.5 |
| State migration handles layout changes | F-1.6.5 | R-1.6.5 |

## US-1.6.9 Implement Hot Reload With State Migration

**As an** engine developer (P-26), **I want** to implement plugin hot reload using
dlopen/LoadLibrary with state migration through serialization and reflection, **so that**
component layout changes across reloads are handled gracefully.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| dlopen/dlclose on POSIX, LoadLibrary/FreeLibrary on Windows | F-1.6.5 | R-1.6.5 |
| State migration via serialization and reflection | F-1.6.5 | R-1.6.5 |
| Layout changes detected and migrated automatically | F-1.6.5 | R-1.6.5 |

## US-1.6.10 Verify Hot Reload Preserves Entity State

**As an** engine tester (P-27), **I want** to verify that hot-reloading a gameplay plugin
preserves all entity state including modified components and relationships, **so that**
developers do not lose their test scenarios during iteration.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Entity count unchanged after reload | F-1.6.5 | R-1.6.5 |
| Component values preserved (or migrated) correctly | F-1.6.5 | R-1.6.5 |
| Relationships and hierarchy intact after reload | F-1.6.5 | R-1.6.5 |

## API Stability Contracts

## US-1.6.11 Reject Incompatible Plugin Binaries at Load Time

**As an** engine developer (P-26), **I want** plugins to embed semantic version and ABI hash
metadata checked at load time, **so that** dynamically loaded plugins compiled against
incompatible engine versions are rejected rather than causing memory corruption.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Semantic version embedded in plugin descriptor | F-1.6.6 | R-1.6.6 |
| ABI hash derived from public interface types | F-1.6.6 | R-1.6.6 |
| Incompatible binaries rejected with clear error | F-1.6.6 | R-1.6.6 |

## US-1.6.12 Verify ABI Compatibility Check Rejects Mismatched Plugins

**As an** engine tester (P-27), **I want** to verify that loading a plugin compiled against
a different engine version is rejected with a clear error message, **so that** no
incompatible binary is silently loaded.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Mismatched ABI hash produces rejection | F-1.6.6 | R-1.6.6 |
| Error message includes expected and actual versions | F-1.6.6 | R-1.6.6 |
| Compatible plugins load successfully | F-1.6.6 | R-1.6.6 |

## US-1.6.13 Branch on Optional Plugin Capabilities at Runtime

**As a** game developer (P-15), **I want** plugins to advertise and query optional
capabilities at runtime, **so that** I can branch on feature presence (physics, audio,
networking) without compile-time feature flags or tight coupling.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Capabilities advertised as named feature flags with version | F-1.6.7 | R-1.6.7 |
| Systems query capability presence at runtime | F-1.6.7 | R-1.6.7 |
| Optional integrations decoupled from core code | F-1.6.7 | R-1.6.7 |

## US-1.6.14 Verify Capability Negotiation Returns Correct Results

**As an** engine tester (P-27), **I want** to verify that capability queries return correct
results when plugins are loaded, unloaded, or absent, **so that** systems branch correctly
on capability presence in all configurations.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Capability present when owning plugin loaded | F-1.6.7 | R-1.6.7 |
| Capability absent when owning plugin not loaded | F-1.6.7 | R-1.6.7 |
| Version check rejects incompatible capability versions | F-1.6.7 | R-1.6.7 |
