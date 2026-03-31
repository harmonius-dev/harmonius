# 13.9 — Inventory System

## Storage Model

| ID       | Feature                        |
|----------|--------------------------------|
| F-13.9.1 | ECS-Based Inventory Containers |
| F-13.9.2 | Grid-Based Inventory Layout    |
| F-13.9.3 | Item Stacking and Splitting    |

1. **F-13.9.1** — Each inventory is an ECS entity with an `Inventory` component defining slot count,
   weight capacity, and layout mode (list, grid with item width/height). Individual item stacks are
   child entities with `ItemStack` (item ID, quantity), `InventorySlot` (position/coordinates), and
   optional per-instance components (`Durability`, `Enchantment`, `CustomProperties`). All inventory
   operations are ECS commands — spawning, moving, and removing item entities.
   - **Deps:** F-1.1.1 (ECS), F-1.1.16 (Hierarchy), F-13.7.2 (Data Tables)
2. **F-13.9.2** — Items occupy rectangular cells (width x height) in a 2D grid. Placement validation
   ensures no overlap. Auto-sort rearranges items to minimize wasted space using a bin-packing
   heuristic. Grid dimensions and cell size are configurable per container. Supports Tetris-style
   inventory (Diablo, Escape from Tarkov) and fixed-slot inventory (WoW bags).
   - **Deps:** F-13.9.1
3. **F-13.9.3** — Items with matching IDs and compatible properties stack up to a per-item-type
   maximum. Split operations divide a stack into two item entities. Auto-stack consolidates partial
   stacks when items enter a container. Stack size limits are defined in the item data table
   (F-13.7.2). Stacking rules can be overridden per container (e.g., trade window forces single
   stacks).
   - **Deps:** F-13.9.1, F-13.7.5 (Row Inheritance)

## Item Instances

| ID       | Feature                             |
|----------|-------------------------------------|
| F-13.9.4 | Per-Instance Item Properties        |
| F-13.9.5 | Item Socket and Augmentation System |

1. **F-13.9.4** — Individual item instances carry mutable properties beyond the base definition:
   durability, enchantments, gem sockets (filled/empty), custom name, bind status (unbound,
   bound-on-pickup, bound-on-equip), and arbitrary key-value pairs. Properties are ECS components on
   the item entity, enabling systems to query items by property (e.g., "all items with durability <
   10%").
   - **Deps:** F-13.9.1, F-13.7.5 (Row Inheritance)
2. **F-13.9.5** — Item entities can have `SocketSlot` child entities, each accepting another item
   entity as an insert (gems, runes, enchantments). Socket type filtering uses the item type
   hierarchy (F-13.7.5) — a "Rune" socket accepts any item descending from the "Rune" prototype.
   Inserting a socket item merges compatible stat modifiers onto the parent item. Removal may
   destroy the insert or require a special item/currency.
   - **Deps:** F-13.9.4, F-13.7.5 (Row Inheritance), F-13.7.9 (Stats)

## Operations

| ID       | Feature                          |
|----------|----------------------------------|
| F-13.9.6 | Inventory Transfer and Drag-Drop |
| F-13.9.7 | Loot Distribution                |
| F-13.9.8 | Merchant and Trading             |

1. **F-13.9.6** — Move items between inventories (player bag → bank, player → trade window, loot
   container → player) as ECS entity reparenting operations. Transfers validate capacity, weight,
   grid fit, and item restrictions on the target container before committing. Drag-and-drop state is
   tracked via a transient `DragPayload` ECS resource. All transfers are server-authoritative with
   client prediction for responsiveness.
   - **Deps:** F-13.9.1, F-8.3.1 (RPCs)
2. **F-13.9.7** — Distribute items from a loot source (defeated enemy, chest, quest reward) to one
   or more players. Supports distribution modes: free-for-all, round-robin, need/greed voting,
   master looter, and personal loot (per-player instanced drops). Loot tables (F-13.7.8) generate
   the drop list; distribution rules determine allocation. Unclaimed items expire after a
   configurable timeout.
   - **Deps:** F-13.9.1, F-13.7.8 (Loot Tables)
3. **F-13.9.8** — Buy/sell items between player inventories and NPC merchant inventories. Prices are
   driven by currency definitions (F-13.7.6) with optional modifiers (reputation discounts,
   supply/demand). Player-to-player trading uses a two-inventory trade window with mutual
   confirmation. Auction house integration lists items with buyout prices and timed bidding.
   - **Deps:** F-13.9.1, F-13.7.6 (Currencies)

## Equipment Integration

| ID       | Feature                |
|----------|------------------------|
| F-13.9.9 | Equipment Slot Binding |

1. **F-13.9.9** — Named equipment slots (head, chest, legs, feet, hands, weapon main/off, ring,
   necklace, trinket) are ECS components on the character entity. Equipping moves an item entity
   from inventory to the equipment slot and triggers stat modifier application, visual attachment
   (F-13.8.10), and animation state changes. Slot compatibility uses item type hierarchy filtering.
   Unequipping returns the item to inventory.
   - **Deps:** F-13.9.1, F-13.8.10 (Sockets), F-13.7.9 (Stats)

## Persistence

| ID        | Feature                                 |
|-----------|-----------------------------------------|
| F-13.9.10 | Inventory Serialization and Persistence |

1. **F-13.9.10** — Serialize complete inventory state — container layout, all item entities with
   their per-instance properties, socket contents, and equipment bindings — to the save system
   (F-13.3.1) and network replication (F-8.2.1). Schema-versioned for live-service migrations
   (adding new item properties without wiping inventories). Server-authoritative storage prevents
   duplication exploits.
   - **Deps:** F-13.9.1, F-1.4.1 (Serialization), F-13.3.1 (Save System)
