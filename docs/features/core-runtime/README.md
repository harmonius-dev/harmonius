# Core Runtime Features

Entity component system, scene management, reflection, serialization, events, plugins, memory
management, and async I/O foundations for the engine.

## Files

| File | Scope |
|------|-------|
| [entity-component-system.md](entity-component-system.md) | Dense/sparse storage, archetype graph, components (tags, shared, buffers, enableable, hooks, bundles), generational entities, cleanup components, entity names, relationships/pairs, hierarchies, queries (sorting, grouping, variables, parallel), aspects, change detection, resources, system groups/phases/run criteria/ambiguity detection, observers, command buffers (parallel), multiple worlds, entity migration, prefabs/prototypes, state machines |
| [scene-and-transforms.md](scene-and-transforms.md) | Scene hierarchy, parent-child, transform propagation, dirty tracking, spatial partitioning |
| [reflection-and-type-system.md](reflection-and-type-system.md) | Type registry, type info, property system, dynamic access, attributes/metadata |
| [serialization.md](serialization.md) | Binary/text serialization, schema versioning, migration, asset/scene serialization |
| [events-and-messaging.md](events-and-messaging.md) | Typed event channels, observers, reactive queries, deferred commands, inter-system messaging |
| [plugin-system.md](plugin-system.md) | Module registration, dependency declaration, hot reload, API stability contracts |
| [memory-management.md](memory-management.md) | Arena/pool allocators, resource handles, generational indices, memory budgets, profiling hooks |
| [async-io.md](async-io.md) | Platform I/O backends (IOCP/GCD/io_uring), completion model, file/network/audio I/O, vectored I/O, priority, cancellation, buffer pools |
| [spatial-indexing.md](spatial-indexing.md) | Shared BVH/octree, incremental updates, unified query API, batch queries, physics/rendering/network/AI integration |
