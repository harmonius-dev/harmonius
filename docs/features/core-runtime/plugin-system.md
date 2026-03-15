# 1.6 — Plugin System

## Module Registration

### F-1.6.1 Declarative Plugin Registration

Provide a plugin trait that declares the systems, components, resources, and events a module
contributes to the engine. Plugins are registered at application startup through a builder API.
Registration order is determined automatically from declared dependencies, not from call order,
ensuring deterministic initialization regardless of how plugins are composed.

- **Requirements:** R-1.6.1
- **Dependencies:** F-1.1.4 (Static and Dynamic Component Registration), F-1.3.1 (Type Registry)
- **Platform notes:** None

### F-1.6.2 Plugin Groups and Presets

Allow plugins to be bundled into named groups (e.g., `DefaultPlugins`, `ServerPlugins`,
`MinimalPlugins`) that register a curated set of functionality with a single call. Groups can be
customized by disabling individual plugins before registration. This simplifies application setup
and enables shipping different plugin sets for client, server, and headless simulation.

- **Requirements:** R-1.6.2
- **Dependencies:** F-1.6.1
- **Platform notes:** None

## Dependency Declaration

### F-1.6.3 Explicit Plugin Dependency Declaration

Each plugin declares which other plugins it requires and which it conflicts with. The plugin loader
validates the dependency graph at startup, reporting missing dependencies and conflicts as
actionable errors before any systems run. This prevents subtle runtime failures caused by unmet
assumptions about available components or resources.

- **Requirements:** R-1.6.3
- **Dependencies:** F-1.6.1
- **Platform notes:** None

### F-1.6.4 Plugin Load Order Resolution

Resolve plugin initialization order by topologically sorting the dependency graph. Circular
dependencies are detected and reported at startup. Late-registering plugins (those added after
initial setup, e.g., from a mod loader) are inserted into the correct position and trigger
re-validation of the full dependency graph.

- **Requirements:** R-1.6.4
- **Dependencies:** F-1.6.3
- **Platform notes:** None

## Hot Reload Support

### F-1.6.5 Hot Reload of Gameplay Plugins

Support unloading and reloading gameplay plugins at runtime during development. On reload, the
engine preserves ECS world state, tears down systems from the old plugin, loads the updated shared
library, and re-registers systems from the new version. State migration is handled through the
serialization and reflection systems to accommodate layout changes.

- **Requirements:** R-1.6.5
- **Dependencies:** F-1.6.1, F-1.4.5 (Data Migration), F-1.3.5 (Dynamic Values)
- **Platform notes:** Uses `dlopen`/`dlclose` on POSIX and `LoadLibrary`/`FreeLibrary` on Windows.

## API Stability Contracts

### F-1.6.6 Semantic Versioning and ABI Stability Metadata

Embed semantic version metadata in each plugin's registration descriptor. The engine checks plugin
versions against declared compatibility ranges at load time. For dynamically loaded plugins, an ABI
hash derived from the plugin's public interface types is compared to prevent loading binaries
compiled against incompatible engine versions.

- **Requirements:** R-1.6.6
- **Dependencies:** F-1.6.1, F-1.3.2 (Structured Type Descriptors)
- **Platform notes:** None

### F-1.6.7 Capability Negotiation for Optional Features

Allow plugins to advertise and query optional capabilities at registration time. A capability is a
named feature flag with an associated version. Systems can branch on capability presence to enable
enhanced functionality when a supporting plugin is loaded. This decouples optional integrations
(e.g., physics, audio, networking) from core engine code without compile-time feature flags.

- **Requirements:** R-1.6.7
- **Dependencies:** F-1.6.1, F-1.6.3
- **Platform notes:** None
