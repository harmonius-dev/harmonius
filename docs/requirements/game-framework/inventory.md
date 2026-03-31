# R-13.9 — Inventory System Requirements

## Storage Model

1. **R-13.9.1** — The engine **SHALL** represent inventories as ECS entities with configurable slot
   count, weight capacity, and layout mode (list, grid), with item stacks as child entities carrying
   quantity, position, and optional per-instance components.
   - **Rationale:** ECS-based inventory integrates with queries and the entity hierarchy.
   - **Verification:** Create an inventory with 20 slots. Add items and verify child entities are
     created. Exceed capacity and verify rejection.

2. **R-13.9.2** — The engine **SHALL** support grid-based inventory with rectangular item cells,
   overlap prevention, and auto-sort using a bin-packing heuristic.
   - **Rationale:** Grid layout enables spatial inventory puzzles across genres.
   - **Verification:** Place items of varying sizes in a grid. Verify no overlap. Trigger auto-sort
     and verify reduced wasted space.

3. **R-13.9.3** — The engine **SHALL** support item stacking up to a per-type maximum with split
   operations and auto-consolidation of partial stacks on container entry.
   - **Rationale:** Stacking reduces slot usage for common items.
   - **Verification:** Add 10 items with max stack 5 and verify two stacks of 5. Split a stack and
     verify two item entities. Add a partial stack and verify auto-consolidation.

## Item Instances

4. **R-13.9.4** — The engine **SHALL** support per-instance item properties (durability,
   enchantments, sockets, bind status, key-value pairs) as ECS components on item entities.
   - **Rationale:** ECS components enable systems to query items by property efficiently.
   - **Verification:** Create an item with durability and enchantment components. Query all items
     with durability under 10% and verify correct results.

5. **R-13.9.5** — The engine **SHALL** support item socket slots as child entities accepting insert
   items filtered by type hierarchy, with stat modifier merging on insertion.
   - **Rationale:** Socket systems enable item augmentation using the same type hierarchy as the
     data tables.
   - **Verification:** Create an item with a "Rune" socket. Insert a rune and verify stat merge.
     Attempt inserting a non-rune and verify rejection.

## Operations

6. **R-13.9.6** — The engine **SHALL** validate item transfers between containers for capacity,
   weight, grid fit, and restrictions before committing, with server-authoritative validation and
   client prediction.
   - **Rationale:** Server authority prevents duplication; client prediction keeps transfers
     responsive.
   - **Verification:** Transfer an item to a full container and verify rejection. Transfer to a
     valid container and verify the item moves. Simulate latency and verify client prediction shows
     immediate movement.

7. **R-13.9.7** — The engine **SHALL** support loot distribution modes (free-for-all, round-robin,
   personal loot) from loot table generation to player allocation with configurable expiration.
   - **Rationale:** Multiple distribution modes support different group dynamics.
   - **Verification:** Generate loot from a table with 5 entries. Distribute with round-robin and
     verify even allocation. Verify unclaimed items expire after timeout.

8. **R-13.9.8** — The engine **SHALL** support buy/sell transactions between player and merchant
   inventories driven by currency definitions, with player-to-player trade windows requiring mutual
   confirmation.
   - **Rationale:** Currency-driven transactions enable data-driven merchant economies.
   - **Verification:** Buy an item and verify currency deduction. Sell an item and verify currency
     credit. Open a trade window, add items, and verify mutual confirmation is required.

## Equipment and Persistence

9. **R-13.9.9** — The engine **SHALL** provide named equipment slots with slot compatibility using
   item type hierarchy filtering, triggering stat modifier application and visual attachment on
   equip.
   - **Rationale:** Type-filtered slots prevent equipping incompatible items without hardcoded
     rules.
   - **Verification:** Equip a sword in the weapon slot and verify stat application. Attempt
     equipping armor in the weapon slot and verify rejection.

10. **R-13.9.10** — The engine **SHALL** serialize complete inventory state (layout, items,
    per-instance properties, sockets, equipment bindings) with schema versioning for live-service
    migrations, using server-authoritative storage.
    - **Rationale:** Schema versioning enables patching without data loss; server authority prevents
      duplication.
    - **Verification:** Save inventory state, apply a schema migration adding a new property, load,
      and verify existing items are intact with the new property defaulted.
