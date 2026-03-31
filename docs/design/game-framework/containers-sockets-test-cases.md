# Container and Socket Systems -- Test Cases

Companion test cases for [containers-sockets.md](containers-sockets.md).

## Unit Tests -- Containers

### TC-13.9.1.1 List Container Add Item

| # | Requirement |
|---|-------------|
| 1 | R-13.9.1    |

1. **#1** -- Add item entity to List container with 3/24 slots used
   - **Expected:** `used_slots == 4`, item has `ContainerSlot` with `slot_index == 3`

### TC-13.9.1.2 List Container Remove Item

| # | Requirement |
|---|-------------|
| 1 | R-13.9.1    |

1. **#1** -- Remove item from slot 2 in List container with 5 items
   - **Expected:** `used_slots == 4`, `ContainerSlot` removed from item entity

### TC-13.9.2.1 Grid Occupy Region

| # | Requirement |
|---|-------------|
| 1 | R-13.9.2    |

1. **#1** -- Place 2x3 item at (1, 0) in 6x4 grid
   - **Expected:** `is_region_free(1, 0, 2, 3) == false`, 6 bits set in `GridOccupancy`

### TC-13.9.2.2 Grid Vacate Region

| # | Requirement |
|---|-------------|
| 1 | R-13.9.2    |

1. **#1** -- Occupy (0, 0, 2, 2) then vacate same region
   - **Expected:** `is_region_free(0, 0, 2, 2) == true`, 0 bits set

### TC-13.9.2.3 Grid Region Overlap Reject

| # | Requirement |
|---|-------------|
| 1 | R-13.9.2    |

1. **#1** -- Occupy (0, 0, 2, 2), attempt place at (1, 1, 2, 2)
   - **Expected:** `is_region_free(1, 1, 2, 2) == false`, transfer returns `InsufficientCapacity`

### TC-13.9.2.4 Grid Bin Pack Auto Sort

| # | Requirement |
|---|-------------|
| 1 | R-13.9.2    |

1. **#1** -- Fill 6x4 grid with 3 items (2x2, 1x3, 3x1) in scattered positions, then request
   auto-sort
   - **Expected:** Items repacked to top-left without overlap, `GridOccupancy` matches new positions

### TC-13.9.3.1 Stack Split

| # | Requirement |
|---|-------------|
| 1 | R-13.9.3    |

1. **#1** -- Stack of 20 potions, split 8 to empty slot
   - **Expected:** Source `quantity == 12`, new `ContainerSlot` with `quantity == 8`

### TC-13.9.3.2 Stack Merge

| # | Requirement |
|---|-------------|
| 1 | R-13.9.3    |

1. **#1** -- Stack A has 15, stack B has 10, max_stack 30
   - **Expected:** B `quantity == 25`, A entity despawned

### TC-13.9.3.3 Stack Max Enforced

| # | Requirement |
|---|-------------|
| 1 | R-13.9.3    |
| 2 | R-13.9.3    |

1. **#1** -- Stack of 25, max_stack 20
   - **Expected:** Transfer rejects with `StackFull`
2. **#2** -- Merge 15 + 10, max_stack 20
   - **Expected:** Target `quantity == 20`, source `quantity == 5` (partial merge)

### TC-13.9.1.3 Slot Constraint Accept

| # | Requirement |
|---|-------------|
| 1 | R-13.9.1    |

1. **#1** -- Slot accepts tag "weapon", item has tag "weapon"
   - **Expected:** `validate_transfer()` returns `Ok(())`

### TC-13.9.1.4 Slot Constraint Reject

| # | Requirement |
|---|-------------|
| 1 | R-13.9.1    |
| 2 | R-13.9.1    |

1. **#1** -- Slot accepts tag "weapon", item has tag "armor"
   - **Expected:** `Err(ConstraintViolation)`
2. **#2** -- Slot rejects tag "quest_item", item has "quest_item"
   - **Expected:** `Err(ConstraintViolation)`

### TC-13.9.1.5 Unique Constraint

| # | Requirement |
|---|-------------|
| 1 | R-13.9.1    |

1. **#1** -- Slot has `unique = true`, container already holds one item of the same type
   - **Expected:** Transfer returns `ConstraintViolation`

### TC-13.9.1.6 Capacity Slot Count

| # | Requirement |
|---|-------------|
| 1 | R-13.9.1    |

1. **#1** -- Container with `CapacityMode::SlotCount`, `slot_count = 24`, `used_slots = 24`
   - **Expected:** `Err(InsufficientCapacity)`

### TC-13.9.1.7 Capacity Weight

| # | Requirement |
|---|-------------|
| 1 | R-13.9.1    |

1. **#1** -- Container with `CapacityMode::Weight`, `weight_capacity = 50.0`,
   `current_weight = 48.0`, item weight 5.0
   - **Expected:** `Err(WeightExceeded)`

### TC-13.9.1.8 Capacity Both

| # | Requirement |
|---|-------------|
| 1 | R-13.9.1    |
| 2 | R-13.9.1    |

1. **#1** -- `CapacityMode::Both`, slots available but weight exceeded
   - **Expected:** `Err(WeightExceeded)`
2. **#2** -- `CapacityMode::Both`, weight available but slots full
   - **Expected:** `Err(InsufficientCapacity)`

### TC-13.9.1.9 Transfer Success

| # | Requirement |
|---|-------------|
| 1 | R-13.9.1    |

1. **#1** -- Move item from container A slot 0 to container B slot 3, both have capacity
   - **Expected:** `TransferEvent` with `result == Success`, item reparented to B,
     `ContainerSlot.slot_index == 3`

### TC-13.9.1.10 Transfer Insufficient Capacity

| # | Requirement |
|---|-------------|
| 1 | R-13.9.1    |

1. **#1** -- Destination container at full slot capacity
   - **Expected:** `TransferEvent` with `result == InsufficientCapacity`

### TC-13.9.1.11 Transfer Constraint Fail

| # | Requirement |
|---|-------------|
| 1 | R-13.9.1    |

1. **#1** -- Item tags do not match destination container `accepted_tags`
   - **Expected:** `TransferEvent` with `result == ConstraintViolation`

### TC-13.9.1.12 Transfer Invalid Slot

| # | Requirement |
|---|-------------|
| 1 | R-13.9.1    |
| 2 | R-13.9.1    |

1. **#1** -- Source slot index exceeds `slot_count`
   - **Expected:** `Err(InvalidSlot)`
2. **#2** -- Destination slot index exceeds `slot_count`
   - **Expected:** `Err(InvalidSlot)`

### TC-13.9.2.5 Transfer Auto Place

| # | Requirement |
|---|-------------|
| 1 | R-13.9.2    |

1. **#1** -- `dest_slot = u16::MAX` (auto-place) on grid container with scattered items
   - **Expected:** Item placed in first valid free region, `GridOccupancy` updated

### TC-13.9.1.13 Nesting Allowed

| # | Requirement |
|---|-------------|
| 1 | R-13.9.1    |

1. **#1** -- Container with `nestable = true`, `max_nesting_depth = 2`, transfer bag at depth 0
   - **Expected:** `Ok(())`, bag placed inside container

### TC-13.9.1.14 Nesting Depth Exceeded

| # | Requirement |
|---|-------------|
| 1 | R-13.9.1    |

1. **#1** -- Container with `max_nesting_depth = 1`, transfer bag already at depth 1
   - **Expected:** `Err(NestingDepthExceeded)`

### TC-13.9.2.6 Sort By Name

| # | Requirement |
|---|-------------|
| 1 | R-13.9.2    |

1. **#1** -- Container with items ["Zephyr", "Apple", "Mace"], `SortCriteria::Name`
   - **Expected:** Slot order: Apple, Mace, Zephyr

### TC-13.9.2.7 Sort By Weight

| # | Requirement |
|---|-------------|
| 1 | R-13.9.2    |

1. **#1** -- Items with weights [5.0, 1.0, 3.0], `SortCriteria::Weight`
   - **Expected:** Slot order by ascending weight: 1.0, 3.0, 5.0

### TC-13.9.2.8 Sort By Rarity

| # | Requirement |
|---|-------------|
| 1 | R-13.9.2    |

1. **#1** -- Items with rarities [Common, Legendary, Rare], `SortCriteria::Rarity`
   - **Expected:** Slot order: Common, Rare, Legendary

### TC-13.9.1.15 Weight Recalc After Transfer

| # | Requirement |
|---|-------------|
| 1 | R-13.9.1    |

1. **#1** -- Move item (weight 5.0) from container A (`current_weight = 30.0`) to B
   (`current_weight = 10.0`)
   - **Expected:** A `current_weight == 25.0`, B `current_weight == 15.0`

## Unit Tests -- Sockets

### TC-13.8.7.1 Attach Compatible

| # | Requirement |
|---|-------------|
| 1 | R-13.8.7    |

1. **#1** -- Socket with `compatible_tags = {"helmet"}`, attachment with tag "helmet", 0 current
   attachments
   - **Expected:** `check_compatibility()` returns `Ok(())`

### TC-13.8.7.2 Attach Incompatible Tags

| # | Requirement |
|---|-------------|
| 1 | R-13.8.7    |

1. **#1** -- Socket with `compatible_tags = {"helmet"}`, attachment with tag "weapon"
   - **Expected:** `Err(Incompatible)`

### TC-13.8.7.3 Attach Socket Full

| # | Requirement |
|---|-------------|
| 1 | R-13.8.7    |

1. **#1** -- Socket with `max_attachments = 1`, `current_attachment_count = 1`
   - **Expected:** `Err(Full)`

### TC-13.8.7.4 Attach Already Attached

| # | Requirement |
|---|-------------|
| 1 | R-13.8.7    |

1. **#1** -- Entity already has `Attachment` component referencing the same socket
   - **Expected:** `Err(AlreadyAttached)`

### TC-13.8.7.5 Detach Removes Component

| # | Requirement |
|---|-------------|
| 1 | R-13.8.7    |

1. **#1** -- Detach entity from socket
   - **Expected:** `Attachment` component removed, entity unparented from socket, `DetachEvent`
     emitted

### TC-13.8.7.6 Detach Reverses Stats

| # | Requirement |
|---|-------------|
| 1 | R-13.8.7    |

1. **#1** -- Attach item with `Add(Strength, 10.0)`, then detach
   - **Expected:** Owner stat Strength reduced by 10.0 after detach; stat matches pre-attach value

### TC-13.10.3.1 Stat Propagation Add

| # | Requirement |
|---|-------------|
| 1 | R-13.10.3   |

1. **#1** -- Attach entity with `StatModifier(Strength, Add, 5.0)` to socket with
   `propagate_stats = true`
   - **Expected:** Owner receives `ApplyEffectRequest` with `Add(Strength, 5.0)`

### TC-13.10.3.2 Stat Propagation Multiply

| # | Requirement |
|---|-------------|
| 1 | R-13.10.3   |

1. **#1** -- Attach entity with `StatModifier(Speed, Multiply, 0.2)` to socket
   - **Expected:** Owner receives `Multiply(Speed, 0.2)`, effective speed = base * 1.2

### TC-13.10.3.3 Stat Propagation Override

| # | Requirement |
|---|-------------|
| 1 | R-13.10.3   |

1. **#1** -- Attach entity with `StatModifier(FireRate, Override, 10.0)`
   - **Expected:** Owner FireRate set to 10.0, ignoring other modifiers

### TC-13.10.3.4 Stat Propagation Multi Attachment

| # | Requirement |
|---|-------------|
| 1 | R-13.10.3   |

1. **#1** -- Socket with `max_attachments = 3`, attach 3 entities with `Add(Armor, 5.0)` each
   - **Expected:** Owner receives combined `Add(Armor, 15.0)` across all modifiers

### TC-13.8.7.7 Visual Binding Transform

| # | Requirement |
|---|-------------|
| 1 | R-13.8.7    |

1. **#1** -- Attach entity to socket with `transform_offset = (0.0, 1.5, 0.0)` and
   `rotation_offset = Quat::from_rotation_y(PI/2)`
   - **Expected:** Attachment `Transform.translation == (0.0, 1.5, 0.0)`, `rotation` matches offset

### TC-13.8.7.8 Visual Binding Mesh Override

| # | Requirement |
|---|-------------|
| 1 | R-13.8.7    |

1. **#1** -- Attach entity with `VisualOverride { mesh_override: Some(handle_a), .. }`
   - **Expected:** Entity `MeshHandle` updated to `handle_a`

### TC-13.8.7.9 Visual Hide Socket

| # | Requirement |
|---|-------------|
| 1 | R-13.8.7    |

1. **#1** -- Attach with `VisualOverride { hide_socket_visual: true, .. }`
   - **Expected:** Socket default visual hidden (render flag disabled)

### TC-13.14.1.1 Snap Query Finds Nearby

| # | Requirement |
|---|-------------|
| 1 | R-13.14.1   |

1. **#1** -- Source socket at (0, 0, 0) with `snap_radius = 1.0`, target socket at (0.5, 0, 0) in
   spatial index
   - **Expected:** `query_snap_candidates()` returns 1 candidate with `distance ~= 0.5`

### TC-13.14.1.2 Snap Query Respects Radius

| # | Requirement |
|---|-------------|
| 1 | R-13.14.1   |
| 2 | R-13.14.1   |

1. **#1** -- Source `snap_radius = 0.5`, target at distance 0.6
   - **Expected:** No candidates returned
2. **#2** -- Source `snap_radius = 0.0` (snap disabled)
   - **Expected:** Empty result regardless of nearby sockets

### TC-13.14.1.3 Snap Query Sorted By Distance

| # | Requirement |
|---|-------------|
| 1 | R-13.14.1   |

1. **#1** -- 3 target sockets at distances 0.8, 0.2, 0.5
   - **Expected:** Candidates returned in order: 0.2, 0.5, 0.8

### TC-13.14.1.4 Snap Tag Compatibility

| # | Requirement |
|---|-------------|
| 1 | R-13.14.1   |

1. **#1** -- Source socket tags = {"stone_edge"}, targets have tags {"stone_edge"} and {"wood_edge"}
   - **Expected:** Only the "stone_edge" target returned as candidate

### TC-13.14.1.5 Building Edge Snap

| # | Requirement |
|---|-------------|
| 1 | R-13.14.1   |

1. **#1** -- Wall piece with side edge socket (`snap_radius = 0.5`), adjacent wall within range
   - **Expected:** Snap candidate returned; attach produces wall-to-wall connection with aligned
     transforms

### TC-13.14.1.6 Building Corner Multi Attach

| # | Requirement |
|---|-------------|
| 1 | R-13.14.1   |

1. **#1** -- Corner socket with `max_attachments = 3`, attach 3 wall pieces
   - **Expected:** All 3 attached, `AttachEvent(Success)` for each; 4th attach returns `Full`

### TC-13.16.6.1 Weapon Scope Attach

| # | Requirement |
|---|-------------|
| 1 | R-13.16.6   |

1. **#1** -- Scope socket on rifle with `compatible_tags = {"scope"}`, attach 4x scope with tag
   "scope"
   - **Expected:** `AttachEvent(Success)`, scope parented to socket, transform matches socket offset

### TC-13.16.6.2 Weapon Stat Mod Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-13.16.6   |

1. **#1** -- Attach scope with `StatModifier(Accuracy, Add, 15.0)` to rifle
   - **Expected:** Rifle Accuracy stat increased by 15.0 via `ApplyEffectRequest`

### TC-13.15.3a.1 Mount Rider Attach

| # | Requirement  |
|---|--------------|
| 1 | R-13.15.3a   |

1. **#1** -- Saddle socket with `compatible_tags = {"rider"}`, `max_attachments = 1`, attach rider
   entity with tag "rider"
   - **Expected:** `AttachEvent(Success)`, rider parented to saddle socket

### TC-13.15.3a.2 Mount Saddlebag Container

| # | Requirement  |
|---|--------------|
| 1 | R-13.15.3a   |

1. **#1** -- Saddlebag socket with `max_attachments = 2`, attach container entity
   - **Expected:** Container entity attached to saddlebag socket; container remains functional for
     transfers

## Integration Tests

### TC-13.9.1.I1 Inventory Equip Flow

| # | Requirement          |
|---|----------------------|
| 1 | F-13.9.1, F-13.8.7  |

1. **#1** -- Move sword from backpack slot to equipment MainHand socket via transfer then attach
   - **Expected:** Sword removed from backpack container, attached to MainHand socket, stat
     modifiers applied to character

### TC-13.13.2.I1 Guild Bank Permissioned

| # | Requirement |
|---|-------------|
| 1 | F-13.13.2   |

1. **#1** -- Guild member with rank "Officer" transfers item to guild bank tab restricted to
   "Officer+" rank
   - **Expected:** Transfer succeeds; member with rank "Recruit" attempts same transfer, gets
     `PermissionDenied`

### TC-13.13.1a.I1 Guild Roster Membership

| # | Requirement |
|---|-------------|
| 1 | F-13.13.1a  |

1. **#1** -- Add member entity to guild roster container, then remove
   - **Expected:** Add: `used_slots` increments, member parented to roster. Remove: `used_slots`
     decrements, member unparented.

### TC-13.14.1.I1 Building Snap Chain

| # | Requirement |
|---|-------------|
| 1 | F-13.14.1   |

1. **#1** -- Place 5 wall pieces in sequence, each snapping to the previous via edge sockets
   - **Expected:** All 5 connected via socket attachments; transforms form a continuous wall line

### TC-13.14.5b.I1 Furniture Placement

| # | Requirement |
|---|-------------|
| 1 | F-13.14.5b  |

1. **#1** -- Housing plot container with FixedSlot layout, place bed entity in furniture slot with
   tag "furniture"
   - **Expected:** Bed placed in slot, positioned at slot coordinates within the housing instance

### TC-13.16.6.I1 Weapon Full Customization

| # | Requirement |
|---|-------------|
| 1 | F-13.16.6   |

1. **#1** -- Attach scope, suppressor, and grip to rifle's 3 sockets in sequence
   - **Expected:** All 3 `AttachEvent(Success)`, cumulative stat modifiers applied to rifle

### TC-13.15.3a.I1 Mount Ride Cycle

| # | Requirement  |
|---|--------------|
| 1 | F-13.15.3a   |

1. **#1** -- Attach rider to saddle socket, ride for 100 frames, detach rider
   - **Expected:** Rider follows mount transform while attached; stats restored after detach

### TC-13.9.1.I2 Container Nesting Inventory

| # | Requirement |
|---|-------------|
| 1 | F-13.9.1    |

1. **#1** -- Place bag container inside backpack container (depth 1), add items to nested bag
   - **Expected:** Items accessible in nested bag; further nesting at depth 2 returns
     `NestingDepthExceeded` when `max_nesting_depth = 1`

### TC-13.9.1.I3 Transfer Rollback On Reject

| # | Requirement          |
|---|----------------------|
| 1 | F-13.9.1, F-8.3.1   |

1. **#1** -- Client predicts transfer, server rejects with `ConstraintViolation`
   - **Expected:** Client receives `TransferEvent` with failure reason; prediction rolled back;
     container state matches pre-transfer state

### TC-13.8.7.I1 Attach Detach Cycle Stats

| # | Requirement           |
|---|-----------------------|
| 1 | F-13.8.7, F-13.10.3  |

1. **#1** -- Attach item with `Add(Strength, 10.0)`, verify stat, detach, verify stat, repeat 50
   times
   - **Expected:** Strength increases by 10 on each attach, returns to baseline on each detach; no
     drift

### TC-1.4.1.I1 Serialization Roundtrip

| # | Requirement |
|---|-------------|
| 1 | F-1.4.1     |

1. **#1** -- Serialize container with 5 items and socket with 2 attachments to binary, then
   deserialize
   - **Expected:** All component data matches original; entity references remapped correctly

### TC-1.4.4.I1 Schema Migration

| # | Requirement |
|---|-------------|
| 1 | F-1.4.4     |

1. **#1** -- Deserialize `ContainerDefinition` saved with schema v1 (missing `max_nesting_depth`
   field) using v2
   - **Expected:** Migration fills default value; deserialized definition is fully functional

## Benchmarks

### TC-13.9.NF3.B1 Transfer 500 Items

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 sequential transfers between 2 containers | Wall-clock time | < 1 ms | R-13.9.NF3 |

### TC-13.9.NF3.B2 Grid Bin Pack 500

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Auto-sort 500 variable-size items in 50x50 grid | Wall-clock time | < 1 ms | R-13.9.NF3 |

### TC-13.9.NF3.B3 Sort 500 Items

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Sort 500 items by name in List container | Wall-clock time | < 1 ms | R-13.9.NF3 |

### TC-NFR-13.14.1.B1 Snap Query 500 Pieces

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Snap query with 500 building pieces in index | Wall-clock time | < 2 ms | NFR-13.14.1 |

### TC-13.10.NF1.B1 Stat Propagation 64 Sockets

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Propagate stats across 64 sockets, 2 each | Wall-clock time | < 0.5 ms | R-13.10.NF1 |

### TC-13.9.NF3.B4 Attach Detach Cycle 100

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100 attach + detach cycles on single socket | Wall-clock time | < 0.5 ms | R-13.9.NF3 |
