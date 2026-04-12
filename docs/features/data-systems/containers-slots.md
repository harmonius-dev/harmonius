# 16.2 — Containers and Slots

## Containers

| ID       | Feature                     |
|----------|-----------------------------|
| F-16.2.1 | Bounded Container Component|
| F-16.2.2 | Grid Bin-Packing           |
| F-16.2.3 | Item Stacking              |
| F-16.2.4 | Container Nesting Validation|
| F-16.2.5 | Configurable Sorting       |

1. **F-16.2.1** — Containers are ECS components holding a bounded collection of child item entities
   with configurable capacity, weight limit, and slot count. Items are referenced by generational
   entity IDs; stored metadata comes from gameplay database rows via `RowRef`.
   - **Deps:** F-1.1.1 (ECS Storage), F-16.3.3 (Data Table Row Refs)
2. **F-16.2.2** — Grid-layout containers place rectangular items into a 2D occupancy grid. The
   bin-packer finds the first available region for insertion in row-major order. Used for
   Tetris-style and grid-slot inventories (Diablo, Tarkov, Resident Evil).
   - **Deps:** F-16.2.1
3. **F-16.2.3** — Stackable items share a single container slot when compatible. On insertion, the
   engine merges into existing stacks up to the per-item-type stack maximum before consuming empty
   slots.
   - **Deps:** F-16.2.1, F-16.3.3
4. **F-16.2.4** — Transfers validate container nesting depth against a configurable maximum,
   rejecting bag-in-bag chains that exceed the configured limit. Prevents serialization blowup and
   circular nesting.
   - **Deps:** F-16.2.1
5. **F-16.2.5** — Containers sort contents by configurable criteria (name, weight, rarity, type,
   custom) through a `SortRequest`. Sorting reorders slots in place without modifying the underlying
   item entities.
   - **Deps:** F-16.2.1

## Sockets

| ID       | Feature                       |
|----------|-------------------------------|
| F-16.2.6 | Typed Socket Attachment Points|
| F-16.2.7 | Socket Stat Propagation      |
| F-16.2.8 | Socket Visual Binding        |

1. **F-16.2.6** — Sockets are typed attachment points on an entity with a compatibility tag set,
   transform offset, and optional occupant reference. Items attach only when their tags match the
   socket's required tags. Used for equipment slots, weapon attachments, and gem sockets.
   - **Deps:** F-1.1.1, F-13.1.2 (Gameplay Tags)
2. **F-16.2.7** — Attached items propagate their stat modifiers to the parent entity via the shared
   modifier stack when `propagate_stats` is set. Removing the item removes its modifiers atomically.
   - **Deps:** F-16.2.6, F-16.1.6
3. **F-16.2.8** — Sockets bind visual overrides (mesh, material, color tint) from attached items to
   the socket transform offset on the parent. Weapons, helmets, and shoulder pads render at the
   correct bone-relative position.
   - **Deps:** F-16.2.6, F-2.4 (Rendering Transform)

## Transfers

| ID        | Feature                   |
|-----------|---------------------------|
| F-16.2.9  | Fast Transfer Validation |
| F-16.2.10 | Crafting Recipe Integration|

1. **F-16.2.9** — Transfer validation runs capacity, weight, tag, and nesting checks within 0.1 ms
   per transfer. Drag-and-drop, batched pickups, and recipe consumption all issue through the same
   validator for consistency.
   - **Deps:** F-16.2.1, F-16.2.6
2. **F-16.2.10** — Containers integrate with the crafting system to validate recipe ingredient
   availability and atomically consume ingredients on craft completion. Failed recipes leave the
   container unchanged.
   - **Deps:** F-16.2.1, F-16.2.9, F-13.10 (Crafting)
