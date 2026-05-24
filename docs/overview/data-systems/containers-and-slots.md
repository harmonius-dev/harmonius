# Containers and Slots

Inventories, equipment, and bounded collections.

## What it covers

- Containers: bounded collections with capacity, weight limits, and slot counts.
- Grid-based packing: optional 2D occupancy grid for Tetris-style inventory layouts.
- Stacking: compatible items merge into shared slots up to per-type limits.
- Nesting: configurable max depth prevents pathological bag-in-bag chains.
- Sorting: reorder slots in-place by name, weight, rarity, type, or custom criteria.
- Sockets: typed attachment points (equipment slots, weapon attachments, gem sockets) with
  transform offsets and compatibility tags.
- Stat propagation: socketed items push modifiers to the parent entity.
- Visual binding: sockets override mesh, material, and color at socket-relative positions.
- Transfer validation: unified checks for capacity, weight, tags, and nesting before moves.
- Crafting: atomic ingredient consumption with rollback on failure.

## Concepts

### Container Model

A container is an ECS component with capacity and weight limits. Items are stored as entity IDs with
metadata (count, condition, binding state). Insertion checks capacity, weight, and tags. Removal is
atomic. Optional grid-based packing arranges items in 2D; items have footprints and are placed with
first-fit row-major bin-packing.

### Sockets and Stat Binding

A socket is a typed attachment point: equipment slot, gem socket, or weapon attachment. Sockets have
compatibility tags controlling what items fit. When an item occupies a socket, its stat modifiers
propagate to the parent entity. Visual overrides (mesh, material, color) are applied at
socket-relative transforms.

### Crafting Integration

Recipe validation and ingredient consumption are atomic. If validation fails, the container is
unchanged. Failed recipes do not consume ingredients or produce output.

## How it fits

- See [attributes-and-effects](./attributes-and-effects.md) for stat modifiers applied by
  socketed items.
- See [data-tables](./data-tables.md) for item type definitions and recipe definitions.
- See [composition](./composition.md) for how containers compose into inventory systems.
- Integrates with [game-framework](../game-framework/gameplay-features.md) for inventory UIs
  and crafting systems.
