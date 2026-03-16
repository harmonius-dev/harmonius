# User Stories — 13.9 Inventory System

## F-13.9.1 ECS-Based Inventory Containers

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.9.1.1 | designer (P-5) | I want to create inventory entities with configurable slot count, weight capacity, and layout mode (list or grid) in the visual editor so that different container types are authored as data |  | F-13.9.1 | R-13.9.1 |
| US-13.9.1.2 | player (P-23) | I want to open my inventory and see all items organized by slot position with quantity, icon, and name so that I can find and manage my items easily |  | F-13.9.1 | R-13.9.1 |
| US-13.9.1.3 | engine tester (P-27) | I want to perform all inventory operations (add, move, remove items) and verify they execute as ECS entity commands so that inventory state is consistent with the ECS world |  | F-13.9.1 | R-13.9.1 |
| US-13.9.1.4 | engine tester (P-27) | I want to add items exceeding the container's weight capacity and verify the operation is rejected so that weight limits are enforced |  | F-13.9.1 | R-13.9.1 |

## F-13.9.2 Grid-Based Inventory Layout

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.9.2.1 | designer (P-5) | I want to set grid dimensions and cell size per container (Tetris- style or fixed-slot) so that different inventory layouts match different game designs |  | F-13.9.2 | R-13.9.2 |
| US-13.9.2.2 | player (P-23) | I want to place items in a 2D grid where they occupy width x height cells so that inventory management is a spatial puzzle |  | F-13.9.2 | R-13.9.2 |
| US-13.9.2.3 | player (P-23) | I want to auto-sort my inventory to minimize wasted space using a packing heuristic so that I can fit more items without manual rearrangement |  | F-13.9.2 | R-13.9.2 |
| US-13.9.2.4 | engine tester (P-27) | I want to attempt to place two items in overlapping cells and verify the placement is rejected so that grid integrity is maintained |  | F-13.9.2 | R-13.9.2 |

## F-13.9.3 Item Stacking and Splitting

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.9.3.1 | designer (P-5) | I want to define per-item-type stack size limits in the item data table so that stackable items consolidate within defined bounds |  | F-13.9.3 | R-13.9.3 |
| US-13.9.3.2 | player (P-23) | I want to items with matching IDs to stack automatically and split stacks manually so that I can manage quantities flexibly |  | F-13.9.3 | R-13.9.3 |
| US-13.9.3.3 | engine tester (P-27) | I want to add items to a container with existing partial stacks and verify auto-stacking fills existing stacks before creating new ones so that consolidation works correctly |  | F-13.9.3 | R-13.9.3 |
| US-13.9.3.4 | engine tester (P-27) | I want to attempt to exceed a stack size limit and verify the excess creates a new stack so that limits are respected |  | F-13.9.3 | R-13.9.3 |

## F-13.9.4 Per-Instance Item Properties

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.9.4.1 | designer (P-5) | I want to item instances to carry mutable properties (durability, enchantments, sockets, bind status) as ECS components so that individual items are uniquely identifiable and queryable |  | F-13.9.4 | R-13.9.4 |
| US-13.9.4.2 | player (P-23) | I want to see per-instance properties (durability, enchantments, gem sockets, custom name) in item tooltips so that I can evaluate each item's condition |  | F-13.9.4 | R-13.9.4 |
| US-13.9.4.3 | engine tester (P-27) | I want to query all items with durability below 10% and verify correct results so that ECS component queries on item properties work correctly |  | F-13.9.4 | R-13.9.4 |

## F-13.9.5 Item Socket and Augmentation System

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.9.5.1 | designer (P-5) | I want to define socket slot types (gem, rune, enchantment) with type filtering based on item prototype hierarchy so that only compatible inserts fit each socket |  | F-13.9.5 | R-13.9.5 |
| US-13.9.5.2 | player (P-23) | I want to insert gems or runes into item sockets to gain stat bonuses so that I can augment my equipment |  | F-13.9.5 | R-13.9.5 |
| US-13.9.5.3 | engine tester (P-27) | I want to insert a socket item and verify its stat modifiers merge onto the parent item correctly so that augmentation affects character stats |  | F-13.9.5 | R-13.9.5 |
| US-13.9.5.4 | engine tester (P-27) | I want to attempt to insert an incompatible item type into a socket and verify it is rejected so that socket type filtering works correctly |  | F-13.9.5 | R-13.9.5 |

## F-13.9.6 Inventory Transfer and Drag-Drop

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.9.6.1 | designer (P-5) | I want to define capacity, weight, grid fit, and item restriction validation rules per container type so that transfers enforce container-specific constraints |  | F-13.9.6 | R-13.9.6 |
| US-13.9.6.2 | player (P-23) | I want to drag items between inventories (bag to bank, loot to bag, bag to trade window) with visual feedback so that inventory management is intuitive |  | F-13.9.6 | R-13.9.6 |
| US-13.9.6.3 | engine tester (P-27) | I want to attempt to transfer an item to a full container and verify the operation is rejected so that transfer validation prevents invalid state |  | F-13.9.6 | R-13.9.6 |
| US-13.9.6.4 | engine tester (P-27) | I want to perform a transfer under 200ms latency and verify the client predicts the result while the server validates and confirms so that transfers feel responsive yet secure |  | F-13.9.6 | R-13.9.6 |

## F-13.9.7 Loot Distribution

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.9.7.1 | designer (P-5) | I want to configure loot distribution modes (free-for-all, round- robin, need/greed, master looter, personal loot) per encounter so that loot rules match the game's social contract |  | F-13.9.7 | R-13.9.7 |
| US-13.9.7.2 | player (P-23) | I want to receive loot drops from defeated enemies according to the active distribution mode so that rewards are allocated fairly |  | F-13.9.7 | R-13.9.7 |
| US-13.9.7.3 | player (P-23) | I want to vote need or greed on group loot drops so that items go to players who need them most |  | F-13.9.7 | R-13.9.7 |
| US-13.9.7.4 | engine tester (P-27) | I want to leave loot unclaimed and verify it expires after the configured timeout so that abandoned loot does not persist indefinitely |  | F-13.9.7 | R-13.9.7 |

## F-13.9.8 Merchant and Trading

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.9.8.1 | designer (P-5) | I want to configure NPC merchant inventories with prices driven by currency definitions and optional reputation discounts so that shop economies are data- driven |  | F-13.9.8 | R-13.9.8 |
| US-13.9.8.2 | player (P-23) | I want to buy and sell items at NPC merchants with clear price display so that I can acquire needed items and sell unwanted ones |  | F-13.9.8 | R-13.9.8 |
| US-13.9.8.3 | player (P-23) | I want to trade items and currency with other players through a two-inventory trade window with mutual confirmation so that peer-to-peer commerce is safe |  | F-13.9.8 | R-13.9.8 |
| US-13.9.8.4 | engine tester (P-27) | I want to initiate a trade and verify both parties must confirm before items transfer so that trades cannot be forced or tricked |  | F-13.9.8 | R-13.9.8 |

## F-13.9.9 Equipment Slot Binding

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.9.9.1 | designer (P-5) | I want to define named equipment slots (head, chest, weapon, ring) with item type hierarchy filtering so that only compatible items can be equipped in each slot |  | F-13.9.9 | R-13.9.9 |
| US-13.9.9.2 | player (P-23) | I want to equip items to character slots and see stat modifiers applied, visual attachments updated, and animation states changed so that equipping gear has immediate gameplay and visual effects |  | F-13.9.9 | R-13.9.9 |
| US-13.9.9.3 | engine tester (P-27) | I want to equip an item and verify stat modifiers are applied, the visual mesh attaches to the correct socket, and unequipping reverses all changes so that the equip/unequip cycle is complete |  | F-13.9.9 | R-13.9.9 |

## F-13.9.10 Inventory Serialization and Persistence

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.9.10.1 | designer (P-5) | I want to inventory serialization to be schema-versioned so new item properties can be added without wiping existing inventories so that live-service updates preserve player items |  | F-13.9.10 | R-13.9.10 |
| US-13.9.10.2 | player (P-23) | I want to my complete inventory state (layout, items, sockets, equipment) to persist across save/load and server restarts so that I never lose items |  | F-13.9.10 | R-13.9.10 |
| US-13.9.10.3 | engine tester (P-27) | I want to attempt item duplication via save/load and network manipulation and verify all attempts are prevented so that inventory integrity is maintained |  | F-13.9.10 | R-13.9.10 |
| US-13.9.10.4 | engine tester (P-27) | I want to load an inventory from a previous schema version and verify all items are preserved with new properties set to defaults so that schema migrations do not lose data |  | F-13.9.10 | R-13.9.10 |
