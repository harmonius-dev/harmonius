# 1.6 — Plugin System

## Module Registration

| ID      | Feature                        |
|---------|---------------------------------|
| F-1.6.1 | Declarative Plugin Registration|
| F-1.6.2 | Plugin Groups and Presets      |

1. **F-1.6.1** — Provide a plugin trait that declares the systems, components, resources, and events
   a module contributes to the engine. Plugins are registered at application startup through a
   builder API. Registration order is determined automatically from declared dependencies, not from
   call order, ensuring deterministic initialization regardless of how plugins are composed.
   - **Deps:** F-1.1.4 (Static and Dynamic Component Registration), F-1.3.1 (Type Registry)
2. **F-1.6.2** — Allow plugins to be bundled into named groups (e.g., `DefaultPlugins`,
   `ServerPlugins`, `MinimalPlugins`) that register a curated set of functionality with a single
   call. Groups can be customized by disabling individual plugins before registration. This
   simplifies application setup and enables shipping different plugin sets for client, server, and
   headless simulation.
   - **Deps:** F-1.6.1

## Dependency Declaration

| ID      | Feature                               |
|---------|----------------------------------------|
| F-1.6.3 | Explicit Plugin Dependency Declaration|
| F-1.6.4 | Plugin Load Order Resolution          |

1. **F-1.6.3** — Each plugin declares which other plugins it requires and which it conflicts with.
   The plugin loader validates the dependency graph at startup, reporting missing dependencies and
   conflicts as actionable errors before any systems run. This prevents subtle runtime failures
   caused by unmet assumptions about available components or resources.
   - **Deps:** F-1.6.1
2. **F-1.6.4** — Resolve plugin initialization order by topologically sorting the dependency graph.
   Circular dependencies are detected and reported at startup. Late-registering plugins (those added
   after initial setup, e.g., from a mod loader) are inserted into the correct position and trigger
   re-validation of the full dependency graph.
   - **Deps:** F-1.6.3

## Hot Reload Support

| ID      | Feature                       |
|---------|--------------------------------|
| F-1.6.5 | Hot Reload of Gameplay Plugins|

1. **F-1.6.5** — Support unloading and reloading gameplay plugins at runtime during development. On
   reload, the engine preserves ECS world state, tears down systems from the old plugin, loads the
   updated shared library, and re-registers systems from the new version. State migration is handled
   through the serialization and reflection systems to accommodate layout changes.
   - **Deps:** F-1.6.1, F-1.4.5 (Data Migration), F-1.3.5 (Dynamic Values)
   - **Platform:** Uses `dlopen`/`dlclose` on POSIX and `LoadLibrary`/`FreeLibrary` on Windows.

## API Stability Contracts

| ID      | Feature                                       |
|---------|------------------------------------------------|
| F-1.6.6 | Semantic Versioning and ABI Stability Metadata|
| F-1.6.7 | Capability Negotiation for Optional Features  |

1. **F-1.6.6** — Embed semantic version metadata in each plugin's registration descriptor. The
   engine checks plugin versions against declared compatibility ranges at load time. For dynamically
   loaded plugins, an ABI hash derived from the plugin's public interface types is compared to
   prevent loading binaries compiled against incompatible engine versions.
   - **Deps:** F-1.6.1, F-1.3.2 (Structured Type Descriptors)
2. **F-1.6.7** — Allow plugins to advertise and query optional capabilities at registration time. A
   capability is a named feature flag with an associated version. Systems can branch on capability
   presence to enable enhanced functionality when a supporting plugin is loaded. This decouples
   optional integrations (e.g., physics, audio, networking) from core engine code without
   compile-time feature flags.
   - **Deps:** F-1.6.1, F-1.6.3

## Middleman .dylib Architecture

| ID      | Feature                              |
|---------|--------------------------------------|
| F-1.6.8 | Middleman .dylib Codegen Architecture|
| F-1.6.9 | Static LTO Linking for Shipping Builds|

1. **F-1.6.8** — Organize all codegen'd types (components, events, systems, serialization code) into
   a single middleman .dylib. The engine binary loads the middleman at startup. Plugin .dylibs link
   against the middleman for shared type layouts. During hot reload, the middleman is recompiled and
   swapped, ensuring all plugins see consistent type layouts. This three-layer architecture (engine
   binary, middleman .dylib, plugin .dylibs) is the foundation for safe hot reload.
   - **Deps:** F-1.6.5 (Hot Reload), F-1.6.6 (ABI Stability)
   - **Platform:** Uses dlopen/dlclose on POSIX and LoadLibrary/FreeLibrary on Windows. The
     middleman .dylib path is configurable.
2. **F-1.6.9** — In shipping builds, all plugin code is statically linked into the engine binary
   with LTO (Link-Time Optimization) enabled. No .dylib files are distributed with released games.
   Static linking eliminates dynamic dispatch overhead, enables cross-module inlining, and reduces
   the attack surface by removing loadable code paths.
   - **Deps:** F-1.6.8
   - **Platform:** LTO mode: thin LTO for faster builds during development, fat LTO for shipping.
