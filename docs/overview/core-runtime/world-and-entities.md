# World and Entities

The entity-component model: how things are stored, identified, and accessed.

## What it covers

- Entities as unique identifiers with hierarchical names and relationships.
- Components as typed data attached to entities.
- Archetypes: efficient dense storage grouping entities with the same component set.
- Sparse-set storage for components that churn frequently.
- Generational handles enabling safe detection of stale references.
- Parent/child hierarchies with cascading lifecycle propagation.
- Dynamic component registration and toggleable (enableable) components.

## Concepts

### Entities and Components

An entity is a unique ID. A component is a piece of typed data attached to an entity. The engine
stores entities with identical component sets together in archetype tables for cache-friendly
iteration. Entities belong to a single world, and worlds may exchange entities via migration.

### Bundles and Relationships

Bundles are named collections of components that spawn together; the engine auto-inserts required
companion components. Relationships form pair data: (relationship type, target entity) with rich
properties such as exclusive, symmetric, transitive, or acyclic semantics. Parent/child is the
built-in deep hierarchy; despawn can cascade or leave children orphaned.

### Naming and Paths

Entities may be given hierarchical names. Scenes map entity paths to entities with O(log n) lookup.
Prefabs (templates) enable hierarchy composition with inheritance via "IsA" copy-on-write and nested
slot references.

## How it fits

- See [simulation-loop](./simulation-loop.md) for when structural changes (add/remove component)
  occur and how they are ordered relative to game logic.
- See [data-and-io](./data-and-io.md) for how entities serialize and deserialize.
- Integrates with [spatial-indexing](./spatial-indexing.md) for location-based lookups.
