# User Stories — 13.9 Inventory System

## F-13.9.1 ECS-Based Inventory Containers

## US-13.9.1.1 Configure Inventory Containers in the Editor

**As a** designer (P-5), **I want to** create inventory entities with configurable slot count,
weight capacity, and layout mode (list or grid) in the visual editor, **so that** different
container types are authored as data.

## US-13.9.1.2 Open and Browse Inventory Contents

**As a** player (P-23), **I want to** open my inventory and see all items organized by slot position
with quantity, icon, and name, **so that** I can find and manage my items easily.

## US-13.9.1.3 Verify Inventory Operations Are ECS Commands

**As an** engine tester (P-27), **I want to** perform all inventory operations (add, move, remove
items) and verify they execute as ECS entity commands, **so that** inventory state is consistent
with the ECS world.

## US-13.9.1.4 Verify Weight Capacity Enforcement

**As an** engine tester (P-27), **I want to** add items exceeding the container's weight capacity
and verify the operation is rejected, **so that** weight limits are enforced.

## F-13.9.2 Grid-Based Inventory Layout

## US-13.9.2.1 Configure Grid Dimensions Per Container

**As a** designer (P-5), **I want to** set grid dimensions and cell size per container (Tetris-
style or fixed-slot), **so that** different inventory layouts match different game designs.

## US-13.9.2.2 Place Items in a 2D Grid Inventory

**As a** player (P-23), **I want to** place items in a 2D grid where they occupy width x height
cells, **so that** inventory management is a spatial puzzle.

## US-13.9.2.3 Auto-Sort Items to Minimize Wasted Space

**As a** player (P-23), **I want to** auto-sort my inventory to minimize wasted space using a
packing heuristic, **so that** I can fit more items without manual rearrangement.

## US-13.9.2.4 Verify Grid Placement Prevents Overlap

**As an** engine tester (P-27), **I want to** attempt to place two items in overlapping cells and
verify the placement is rejected, **so that** grid integrity is maintained.

## F-13.9.3 Item Stacking and Splitting

## US-13.9.3.1 Configure Stack Size Limits Per Item Type

**As a** designer (P-5), **I want to** define per-item-type stack size limits in the item data
table, **so that** stackable items consolidate within defined bounds.

## US-13.9.3.2 Stack and Split Items

**As a** player (P-23), **I want to** items with matching IDs to stack automatically and split
stacks manually, **so that** I can manage quantities flexibly.

## US-13.9.3.3 Verify Auto-Stack Consolidation on Item Entry

**As an** engine tester (P-27), **I want to** add items to a container with existing partial stacks
and verify auto-stacking fills existing stacks before creating new ones, **so that** consolidation
works correctly.

## US-13.9.3.4 Verify Stack Size Limits Are Enforced

**As an** engine tester (P-27), **I want to** attempt to exceed a stack size limit and verify the
excess creates a new stack, **so that** limits are respected.

## F-13.9.4 Per-Instance Item Properties

## US-13.9.4.1 Define Per-Instance Properties as Components

**As a** designer (P-5), **I want to** item instances to carry mutable properties (durability,
enchantments, sockets, bind status) as ECS components, **so that** individual items are uniquely
identifiable and queryable.

## US-13.9.4.2 Inspect Item Properties in Tooltips

**As a** player (P-23), **I want to** see per-instance properties (durability, enchantments, gem
sockets, custom name) in item tooltips, **so that** I can evaluate each item's condition.

## US-13.9.4.3 Verify Property-Based Item Queries

**As an** engine tester (P-27), **I want to** query all items with durability below 10% and verify
correct results, **so that** ECS component queries on item properties work correctly.

## F-13.9.5 Item Socket and Augmentation System

## US-13.9.5.1 Configure Socket Types and Filtering Rules

**As a** designer (P-5), **I want to** define socket slot types (gem, rune, enchantment) with type
filtering based on item prototype hierarchy, **so that** only compatible inserts fit each socket.

## US-13.9.5.2 Insert Gems and Enchantments Into Sockets

**As a** player (P-23), **I want to** insert gems or runes into item sockets to gain stat bonuses,
**so that** I can augment my equipment.

## US-13.9.5.3 Verify Socket Insert Merges Stat Modifiers

**As an** engine tester (P-27), **I want to** insert a socket item and verify its stat modifiers
merge onto the parent item correctly, **so that** augmentation affects character stats.

## US-13.9.5.4 Verify Socket Type Filtering Rejects Incompatible Inserts

**As an** engine tester (P-27), **I want to** attempt to insert an incompatible item type into a
socket and verify it is rejected, **so that** socket type filtering works correctly.

## F-13.9.6 Inventory Transfer and Drag-Drop

## US-13.9.6.1 Configure Transfer Validation Rules

**As a** designer (P-5), **I want to** define capacity, weight, grid fit, and item restriction
validation rules per container type, **so that** transfers enforce container-specific constraints.

## US-13.9.6.2 Move Items Between Inventories via Drag-Drop

**As a** player (P-23), **I want to** drag items between inventories (bag to bank, loot to bag, bag
to trade window) with visual feedback, **so that** inventory management is intuitive.

## US-13.9.6.3 Verify Transfer Validation Rejects Invalid Moves

**As an** engine tester (P-27), **I want to** attempt to transfer an item to a full container and
verify the operation is rejected, **so that** transfer validation prevents invalid state.

## US-13.9.6.4 Verify Transfers Are Server-Authoritative With Client Prediction

**As an** engine tester (P-27), **I want to** perform a transfer under 200ms latency and verify the
client predicts the result while the server validates and confirms, **so that** transfers feel
responsive yet secure.

## F-13.9.7 Loot Distribution

## US-13.9.7.1 Configure Loot Distribution Modes

**As a** designer (P-5), **I want to** configure loot distribution modes (free-for-all, round-
robin, need/greed, master looter, personal loot) per encounter, **so that** loot rules match the
game's social contract.

## US-13.9.7.2 Receive Loot From Defeated Enemies

**As a** player (P-23), **I want to** receive loot drops from defeated enemies according to the
active distribution mode, **so that** rewards are allocated fairly.

## US-13.9.7.3 Vote Need/Greed on Group Loot Drops

**As a** player (P-23), **I want to** vote need or greed on group loot drops, **so that** items go
to players who need them most.

## US-13.9.7.4 Verify Unclaimed Loot Expires After Timeout

**As an** engine tester (P-27), **I want to** leave loot unclaimed and verify it expires after the
configured timeout, **so that** abandoned loot does not persist indefinitely.

## F-13.9.8 Merchant and Trading

## US-13.9.8.1 Configure Merchant Inventories and Pricing

**As a** designer (P-5), **I want to** configure NPC merchant inventories with prices driven by
currency definitions and optional reputation discounts, **so that** shop economies are data- driven.

## US-13.9.8.2 Buy and Sell Items at Merchants

**As a** player (P-23), **I want to** buy and sell items at NPC merchants with clear price display,
**so that** I can acquire needed items and sell unwanted ones.

## US-13.9.8.3 Trade Items With Other Players

**As a** player (P-23), **I want to** trade items and currency with other players through a
two-inventory trade window with mutual confirmation, **so that** peer-to-peer commerce is safe.

## US-13.9.8.4 Verify Trade Requires Mutual Confirmation

**As an** engine tester (P-27), **I want to** initiate a trade and verify both parties must confirm
before items transfer, **so that** trades cannot be forced or tricked.

## F-13.9.9 Equipment Slot Binding

## US-13.9.9.1 Define Equipment Slot Compatibility Rules

**As a** designer (P-5), **I want to** define named equipment slots (head, chest, weapon, ring) with
item type hierarchy filtering, **so that** only compatible items can be equipped in each slot.

## US-13.9.9.2 Equip and Unequip Items to Character Slots

**As a** player (P-23), **I want to** equip items to character slots and see stat modifiers applied,
visual attachments updated, and animation states changed, **so that** equipping gear has immediate
gameplay and visual effects.

## US-13.9.9.3 Verify Equip Triggers Stat Application and Visual Attachment

**As an** engine tester (P-27), **I want to** equip an item and verify stat modifiers are applied,
the visual mesh attaches to the correct socket, and unequipping reverses all changes, **so that**
the equip/unequip cycle is complete.

## F-13.9.10 Inventory Serialization and Persistence

## US-13.9.10.1 Configure Schema-Versioned Inventory Serialization

**As a** designer (P-5), **I want to** inventory serialization to be schema-versioned so new item
properties can be added without wiping existing inventories, **so that** live-service updates
preserve player items.

## US-13.9.10.2 Retain Full Inventory Across Save/Load

**As a** player (P-23), **I want to** my complete inventory state (layout, items, sockets,
equipment) to persist across save/load and server restarts, **so that** I never lose items.

## US-13.9.10.3 Verify Server-Authoritative Storage Prevents Duplication

**As an** engine tester (P-27), **I want to** attempt item duplication via save/load and network
manipulation and verify all attempts are prevented, **so that** inventory integrity is maintained.

## US-13.9.10.4 Verify Inventory Migration Preserves Items

**As an** engine tester (P-27), **I want to** load an inventory from a previous schema version and
verify all items are preserved with new properties set to defaults, **so that** schema migrations do
not lose data.
