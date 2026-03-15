# R-13.9 — Inventory System Requirements

## Storage Model

### R-13.9.1 ECS-Based Inventory Containers

The engine **SHALL** represent inventories as ECS entities with `Inventory` components (slot count,
weight capacity, layout mode) and individual item stacks as child entities with `ItemStack`,
`InventorySlot`, and optional per-instance components, with all inventory operations executed as ECS
commands.

- **Derived from:** [F-13.9.1](../../features/game-framework/inventory.md)
- **Rationale:** ECS-based inventories enable standard ECS queries over items (e.g., "all items with
  durability below 10%") and ensure inventory operations participate in the command buffer
  scheduling model.
- **Verification:** Unit test: create an inventory entity, spawn item stack child entities, and
  verify parent-child hierarchy via ECS queries. Verify `ItemStack` and `InventorySlot` components
  are queryable and return correct values.

### R-13.9.2 Grid-Based Inventory Layout

The engine **SHALL** support 2D grid inventory layouts where items occupy rectangular cells (width x
height) with overlap validation, configurable grid dimensions, and an auto-sort bin-packing
heuristic to minimize wasted space.

- **Derived from:** [F-13.9.2](../../features/game-framework/inventory.md)
- **Rationale:** Grid-based layouts support Tetris-style and bag-based inventory systems used across
  RPG and survival genres.
- **Verification:** Unit test: place a 2x3 item in a 10x10 grid and verify occupied cells are
  marked. Attempt to place an overlapping item and verify rejection. Run auto-sort on a partially
  filled grid and verify item count is preserved and no overlaps exist.

### R-13.9.3 Item Stacking and Splitting

The engine **SHALL** stack items with matching IDs and compatible properties up to a per-type
maximum, split stacks into two item entities, and auto-consolidate partial stacks on container
entry, with per-container stacking rule overrides.

- **Derived from:** [F-13.9.3](../../features/game-framework/inventory.md)
- **Rationale:** Stacking reduces inventory clutter for consumables and materials, while splitting
  enables partial trades and precise quantity control.
- **Verification:** Unit test: add 150 units of an item with max stack size 100 and verify two
  stacks (100 + 50) are created. Split the 100-stack at 60 and verify two stacks (60 + 40). Verify
  auto-consolidation merges two partial stacks when a new item enters the container.

## Item Instances

### R-13.9.4 Per-Instance Item Properties

The engine **SHALL** support mutable per-instance item properties (durability, enchantments, gem
sockets, custom name, bind status, arbitrary key-value pairs) as ECS components on item entities,
enabling system-level queries by property.

- **Derived from:** [F-13.9.4](../../features/game-framework/inventory.md)
- **Rationale:** Per-instance properties as ECS components enable efficient system queries (e.g.,
  durability warnings) without scanning all items linearly.
- **Verification:** Unit test: create two instances of the same item with different durability
  values and verify an ECS query filtering by durability returns only the expected instance. Verify
  bind-on-pickup status is applied when the item enters a player inventory.

### R-13.9.5 Item Socket and Augmentation System

The engine **SHALL** represent item sockets as child entities accepting insert item entities
filtered by the item type hierarchy, merging compatible stat modifiers onto the parent item on
insertion, with configurable removal rules (destroy insert or require currency).

- **Derived from:** [F-13.9.5](../../features/game-framework/inventory.md)
- **Rationale:** Socket systems add item customization depth; type hierarchy filtering prevents
  invalid combinations and stat merging ensures tooltip accuracy.
- **Verification:** Unit test: insert a "Rune" item into a "Rune" socket and verify stat modifiers
  are merged onto the parent. Attempt to insert a "Gem" into a "Rune" socket and verify rejection.
  Remove an insert with the destroy rule and verify the insert entity is despawned.

## Operations

### R-13.9.6 Inventory Transfer and Drag-Drop

The engine **SHALL** transfer items between inventories as ECS entity reparenting operations,
validating capacity, weight, grid fit, and item restrictions on the target container before
committing, with server-authoritative validation and client-side prediction.

- **Derived from:** [F-13.9.6](../../features/game-framework/inventory.md)
- **Rationale:** Server-authoritative transfers prevent item duplication and injection exploits,
  while client prediction ensures responsive drag-and-drop UX.
- **Verification:** Unit test: transfer an item between two inventories and verify parent entity
  changes and slot assignment updates. Attempt a transfer exceeding target capacity and verify
  rejection. Simulate client prediction followed by server rejection and verify rollback.

### R-13.9.7 Loot Distribution

The engine **SHALL** distribute loot from a source to one or more players using configurable
distribution modes (free-for-all, round-robin, need/greed voting, master looter, personal loot) with
loot table generation and configurable expiration timeout for unclaimed items.

- **Derived from:** [F-13.9.7](../../features/game-framework/inventory.md)
- **Rationale:** Multiple distribution modes support different group dynamics and game genres, while
  expiration prevents permanent loot lock on the ground.
- **Verification:** Unit test: distribute loot to a 4-player group in round-robin mode and verify
  each player receives items in rotation. Test need/greed voting with tied rolls and verify
  tiebreaker resolution. Verify unclaimed items despawn after the configured timeout.

### R-13.9.8 Merchant and Trading

The engine **SHALL** support buy/sell transactions between player and NPC merchant inventories using
currency definitions with optional price modifiers (reputation, supply/demand), player-to-player
trading via a mutual-confirmation trade window, and auction house listing with buyout and timed
bidding.

- **Derived from:** [F-13.9.8](../../features/game-framework/inventory.md)
- **Rationale:** Merchant and player trading are core economy systems that must integrate with the
  currency and inventory models for transactional consistency.
- **Verification:** Unit test: buy an item from a merchant and verify currency is deducted and the
  item is added to the player inventory. Apply a reputation discount and verify the reduced price.
  Open a trade window, add items from both sides, confirm, and verify items swap correctly.

## Equipment Integration

### R-13.9.9 Equipment Slot Binding

The engine **SHALL** represent named equipment slots as ECS components on character entities, moving
item entities from inventory to equipment slots on equip (triggering stat modifier application,
visual attachment, and animation state changes) and returning items to inventory on unequip, with
slot compatibility filtered by item type hierarchy.

- **Derived from:** [F-13.9.9](../../features/game-framework/inventory.md)
- **Rationale:** ECS-based equipment slots integrate directly with stat, visual, and animation
  systems, enabling reactive updates when gear changes.
- **Verification:** Unit test: equip an item to the "weapon main" slot and verify stat modifiers are
  applied to the character and the item entity is reparented. Unequip and verify stats revert and
  the item returns to inventory. Attempt to equip a "head" item in the "weapon" slot and verify
  rejection via type hierarchy filtering.

## Persistence

### R-13.9.10 Inventory Serialization and Persistence

The engine **SHALL** serialize complete inventory state (container layout, all item entities with
per-instance properties, socket contents, equipment bindings) to the save system and network
replication, using schema versioning for live-service migrations and server-authoritative storage to
prevent duplication exploits.

- **Derived from:** [F-13.9.10](../../features/game-framework/inventory.md)
- **Rationale:** Schema-versioned serialization ensures inventory data survives game updates without
  data loss, while server authority prevents item duplication.
- **Verification:** Unit test: serialize an inventory with socketed and enchanted items, deserialize
  it, and verify all per-instance properties and socket contents are restored. Serialize with schema
  version N, add a new property in version N+1, and verify migration preserves existing data and
  applies defaults for the new property.

## Non-Functional Requirements

### R-13.9.NF1 Maximum Items Per Container

The engine **SHALL** support at least 500 item stacks per inventory container without degrading
query or transfer performance below the latency thresholds defined in R-13.9.NF3.

- **Derived from:** F-13.9.1, F-13.9.2
- **Rationale:** Large inventories (bank tabs, guild banks, vendor catalogs) commonly hold hundreds
  of item stacks; the system must scale without per-operation slowdowns.
- **Verification:** Benchmark: fill a container with 500 item stacks. Perform 1,000 random lookups,
  transfers, and stack splits. Verify all operations complete within the per-operation latency
  budget.

### R-13.9.NF2 Maximum Containers Per Player

The engine **SHALL** support at least 20 concurrent inventory containers per player entity (bags,
bank tabs, mail attachments, trade window, equipment) without exceeding 2 MB of memory per player's
total inventory state.

- **Derived from:** F-13.9.1, F-13.9.6
- **Rationale:** MMO characters commonly manage multiple bags, bank tabs, and temporary containers
  simultaneously; per-player memory must remain bounded for server scalability.
- **Verification:** Create a player with 20 containers, each holding 100 item stacks with
  per-instance properties. Measure total memory footprint and verify it stays under 2 MB.

### R-13.9.NF3 Inventory Operation Latency

The engine **SHALL** complete all inventory operations (move, stack, split, equip, transfer) within
1 ms on the server, excluding network round-trip time, to ensure responsive UI feedback with client
prediction.

- **Derived from:** F-13.9.6, F-13.9.3
- **Rationale:** Inventory interactions are high-frequency player actions; perceptible server-side
  lag breaks the drag-and-drop interaction model.
- **Verification:** Benchmark: execute 10,000 random inventory operations (moves, splits, equips) on
  a container with 200 items. Verify the 99th percentile per-operation latency is under 1 ms.
