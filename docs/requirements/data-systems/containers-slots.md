# R-16.2 — Containers and Slots Requirements

## Containers

1. **R-16.2.1** — The engine **SHALL** provide a `Container` primitive as an ECS component holding a
   bounded collection of child item entities with configurable capacity, weight limit, and slot
   count.
   - **Rationale:** Containers are the universal primitive for inventories, chests, loot bags, and
     storage across all genres.
   - **Verification:** Unit test: create a container with capacity 8; insert items; assert at-limit
     inserts are rejected; remove items; assert capacity frees.
2. **R-16.2.2** — The engine **SHALL** support optional grid-layout containers with two-dimensional
   cell occupancy where items have rectangular footprints, and SHALL perform automatic bin-packing
   to find the first available region for insertion.
   - **Rationale:** Tetris-style inventories (Diablo, Escape From Tarkov, Resident Evil) require
     grid-based placement that cannot be expressed by flat slot counts.
   - **Verification:** Unit test: author a 10x4 grid container; insert 3x2 and 2x4 items; assert
     occupancy mask matches expected; attempt overlapping insert; assert rejection.
3. **R-16.2.3** — The engine **SHALL** support item stacking within containers with per-item-type
   maximum stack size and automatic merge of compatible stacks on insertion.
   - **Rationale:** Stackable resources (potions, ammunition, crafting materials) must combine
     rather than occupy separate slots to avoid inventory bloat.
   - **Verification:** Unit test: insert 10 of an item with max stack 16; insert 10 more; assert one
     stack of 16 and one stack of 4 in the container.
4. **R-16.2.4** — The engine **SHALL** validate container nesting depth against a configurable
   maximum and reject transfers that would exceed the limit.
   - **Rationale:** Containers inside containers (bag-in-bag) must be bounded to prevent infinite
     nesting loops and excessive serialization cost.
   - **Verification:** Unit test: configure max depth 3; nest 3 bags; attempt to nest a fourth;
     assert transfer validation fails.
5. **R-16.2.5** — The engine **SHALL** sort container contents by configurable criteria (name,
   weight, rarity, type, custom) through a `SortRequest` that reorders slots in place without
   modifying child entities.
   - **Rationale:** Players expect inventory sort buttons; sorting must be a first-class
     engine-level operation, not per-game script.
   - **Verification:** Unit test: populate a container with mixed items; sort by weight ascending;
     assert slot order matches expected permutation.

## Sockets

1. **R-16.2.6** — The engine **SHALL** provide a `Socket` primitive as a typed attachment point on
   an entity with a compatibility tag set, transform offset, and optional occupant entity reference.
   - **Rationale:** Sockets are the universal primitive for equipment slots, weapon attachments, gem
     sockets, and any parent-child attachment that is not a free-form child entity.
   - **Verification:** Unit test: define a socket with tag "gem"; attempt to attach an item tagged
     "weapon"; assert rejection. Attach an item tagged "gem"; assert success.
2. **R-16.2.7** — The engine **SHALL** propagate stat modifiers from socketed items to the parent
   entity via the modifier stack when propagate_stats is enabled on the socket definition.
   - **Rationale:** Gems, runes, and enchants must modify parent entity stats when socketed and
     remove their modifiers when unsocketed — without per-game custom code.
   - **Verification:** Integration test: socket an item granting +10 to an attribute; assert parent
     attribute shows +10 modifier; unsocket; assert modifier removed.
3. **R-16.2.8** — The engine **SHALL** bind visual overrides (mesh, material, color tint) from
   attached items to socket transform offsets on the parent entity so that socketed items render at
   the correct position and orientation.
   - **Rationale:** Weapon in hand, helmet on head, shoulder pauldron — all require a parent-bone or
     parent-socket spatial binding with visual override.
   - **Verification:** Integration test: attach a mesh item to a socket with offset (0,1,0); assert
     rendered mesh transform equals parent transform times socket offset.

## Transfers

1. **R-16.2.9** — The engine **SHALL** validate item transfers between containers and sockets within
   0.1 ms per transfer on desktop hardware, including capacity, weight, tag, and nesting checks.
   - **Rationale:** Drag-and-drop operations must feel instant; batched crafting and loot pickup can
     issue hundreds of transfers per frame.
   - **Verification:** Benchmark: issue 1,000 valid transfers across 100 containers; assert mean
     transfer time under 0.1 ms.
2. **R-16.2.10** — The engine **SHALL** integrate containers with the crafting system to validate
   recipe ingredient availability and atomically consume ingredients on craft completion.
   - **Rationale:** Crafting is a core cross-genre mechanic; atomic ingredient consumption prevents
     race conditions between multiple concurrent craft attempts.
   - **Verification:** Integration test: define a recipe needing 3 iron; craft with 3 iron
     available; assert exactly 3 consumed and output item produced. Attempt with 2; assert rejection
     and no items consumed.
