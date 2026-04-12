# Containers and Slots — Test Cases

Companion to [containers-slots.md](containers-slots.md).

Test case IDs use `TC-16.2.Z.N` format. Every row links to a specific R-X.Y.Z or F-X.Y.Z.

## Unit Tests

| ID            | Name                                | Req       |
|---------------|-------------------------------------|-----------|
| TC-16.2.1.1   | `test_container_construct_capacity` | R-16.2.1  |
| TC-16.2.1.2   | `test_container_insert_until_full`  | R-16.2.1  |
| TC-16.2.1.3   | `test_container_remove_frees_slot`  | R-16.2.1  |
| TC-16.2.1.4   | `test_container_weight_limit`       | R-16.2.1  |
| TC-16.2.2.1   | `test_grid_insert_3x2`              | R-16.2.2  |
| TC-16.2.2.2   | `test_grid_bin_pack_first_fit`      | R-16.2.2  |
| TC-16.2.2.3   | `test_grid_overlap_rejected`        | R-16.2.2  |
| TC-16.2.2.4   | `test_grid_remove_clears_cells`     | R-16.2.2  |
| TC-16.2.3.1   | `test_stack_merge_partial`          | R-16.2.3  |
| TC-16.2.3.2   | `test_stack_overflow_new_slot`      | R-16.2.3  |
| TC-16.2.3.3   | `test_stack_incompatible_no_merge`  | R-16.2.3  |
| TC-16.2.4.1   | `test_nesting_depth_within_limit`   | R-16.2.4  |
| TC-16.2.4.2   | `test_nesting_depth_exceeds`        | R-16.2.4  |
| TC-16.2.4.3   | `test_nesting_circular_rejected`    | R-16.2.4  |
| TC-16.2.5.1   | `test_sort_by_weight_ascending`     | R-16.2.5  |
| TC-16.2.5.2   | `test_sort_by_name_lexicographic`   | R-16.2.5  |
| TC-16.2.5.3   | `test_sort_by_rarity_desc`          | R-16.2.5  |
| TC-16.2.5.4   | `test_sort_preserves_entities`      | R-16.2.5  |
| TC-16.2.6.1   | `test_socket_tag_match_attach`      | R-16.2.6  |
| TC-16.2.6.2   | `test_socket_tag_mismatch_reject`   | R-16.2.6  |
| TC-16.2.6.3   | `test_socket_offset_stored`         | R-16.2.6  |
| TC-16.2.9.1   | `test_transfer_capacity_check`      | R-16.2.9  |
| TC-16.2.9.2   | `test_transfer_weight_check`        | R-16.2.9  |
| TC-16.2.9.3   | `test_transfer_tag_check`           | R-16.2.9  |

1. **TC-16.2.1.1** `test_container_construct_capacity` — Construct a container with capacity 8;
   assert empty state and reported capacity.
   - Input: `Container::new(ContainerDef { capacity: 8, weight_limit: 100.0, slots: 8 })`
   - Expected: `container.len() == 0`, `container.capacity() == 8`, `container.is_empty()`

2. **TC-16.2.1.2** `test_container_insert_until_full` — Insert 8 items into a capacity-8 container,
   then attempt a 9th. Assert 9th rejected with `ContainerFull`.
   - Input: capacity-8 container; 9 distinct item entities
   - Expected: first 8 inserts return `Ok`; 9th returns `Err(TransferError::ContainerFull)`;
     `container.len() == 8`

3. **TC-16.2.1.3** `test_container_remove_frees_slot` — Insert 8 items into a capacity-8 container,
   remove one, insert another. Assert second insert succeeds.
   - Input: full container, `remove(item_3)`, `insert(item_9)`
   - Expected: removal returns `Ok(item_3)`; subsequent insert returns `Ok`; `container.len() == 8`

4. **TC-16.2.1.4** `test_container_weight_limit` — Container weight limit 50 kg; insert items of
   weights `[10, 20, 25]`. Assert third insert is rejected.
   - Input: items with weights 10.0, 20.0, 25.0; container weight_limit 50.0
   - Expected: first two insert; third returns `Err(TransferError::OverWeight)`;
     `container.total_weight() == 30.0`

5. **TC-16.2.2.1** `test_grid_insert_3x2` — 10x4 grid container; insert a 3x2 item at first
   position. Assert occupancy mask covers cells (0..3, 0..2).
   - Input: `GridContainer::new(10, 4)`; item with footprint `(3, 2)`
   - Expected: occupancy true for `(x in 0..3, y in 0..2)`, false elsewhere; item placed at origin
     `(0, 0)`

6. **TC-16.2.2.2** `test_grid_bin_pack_first_fit` — 10x4 grid; insert 3x2 then 2x4. Assert second
   item placed at `(3, 0)` (first available column to the right).
   - Input: grid 10x4; first item 3x2, second item 2x4
   - Expected: first at `(0, 0)`; second at `(3, 0)`; both within bounds; no overlap

7. **TC-16.2.2.3** `test_grid_overlap_rejected` — 10x4 grid with a 5x4 item placed; attempt to
   insert another 8x1 item. Assert rejection (would overlap).
   - Input: grid 10x4 with `(5, 4)` item at origin; new item `(8, 1)`
   - Expected: only fits if width ≤ 5 from x=5; 8 > 5, so `Err(TransferError::NoFreeRegion)`

8. **TC-16.2.2.4** `test_grid_remove_clears_cells` — Insert a 3x2 item then remove it. Assert
   occupancy mask is fully cleared.
   - Input: grid 10x4; insert then remove `(3, 2)` item
   - Expected: occupancy mask all false; subsequent insert at `(0, 0)` succeeds

9. **TC-16.2.3.1** `test_stack_merge_partial` — Item type with max stack 16; existing stack of 10.
   Insert another 5. Assert merged into one stack of 15.
   - Input: container with one slot containing `Stack { item: id, count: 10 }`; insert `count: 5`
   - Expected: container has one slot with `Stack { item: id, count: 15 }`

10. **TC-16.2.3.2** `test_stack_overflow_new_slot` — Item with max stack 16; existing stack of 10.
    Insert 10 more. Assert one stack of 16 and one stack of 4.
    - Input: stack of 10, insert 10
    - Expected: container has two slots: `Stack { count: 16 }` and `Stack { count: 4 }`

11. **TC-16.2.3.3** `test_stack_incompatible_no_merge` — Two distinct item types in adjacent slots;
    insert more of the first type. Assert merges only with the matching type.
    - Input: slot 0 `(item_A, 5)`, slot 1 `(item_B, 5)`; insert `(item_A, 5)`
    - Expected: slot 0 becomes `(item_A, 10)`; slot 1 unchanged

12. **TC-16.2.4.1** `test_nesting_depth_within_limit` — Max depth 3; nest containers A → B → C.
    Assert insert succeeds.
    - Input: `max_depth = 3`; transfers `B into A`, `C into B`
    - Expected: both transfers return `Ok`; `compute_depth(C) == 3`

13. **TC-16.2.4.2** `test_nesting_depth_exceeds` — Max depth 3; attempt to nest a 4th container.
    Assert `NestingTooDeep` rejection.
    - Input: existing nest A → B → C; insert D into C
    - Expected: `Err(TransferError::NestingTooDeep { max: 3, attempted: 4 })`; D not transferred

14. **TC-16.2.4.3** `test_nesting_circular_rejected` — Attempt to insert container A into one of its
    descendants B. Assert `CircularNesting` rejection.
    - Input: A contains B; transfer A into B
    - Expected: `Err(TransferError::CircularNesting)`; A unchanged, B unchanged

15. **TC-16.2.5.1** `test_sort_by_weight_ascending` — Container with items of weights `[5, 1, 3]`;
    sort by weight ascending. Assert slot order matches `[1, 3, 5]`.
    - Input: items with weights 5.0, 1.0, 3.0; `SortRequest { criterion: Weight, order: Asc }`
    - Expected: `slot[0].weight == 1.0`, `slot[1].weight == 3.0`, `slot[2].weight == 5.0`

16. **TC-16.2.5.2** `test_sort_by_name_lexicographic` — Items named `["sword", "apple", "potion"]`;
    sort by name. Assert order `["apple", "potion", "sword"]`.
    - Input: three items; `SortRequest { criterion: Name, order: Asc }`
    - Expected: slot names match `["apple", "potion", "sword"]` in order

17. **TC-16.2.5.3** `test_sort_by_rarity_desc` — Items with rarities `[Common, Legendary, Rare]`;
    sort by rarity descending. Assert order `[Legendary, Rare, Common]`.
    - Input: three items; `SortRequest { criterion: Rarity, order: Desc }`
    - Expected: slot rarities match `[Legendary, Rare, Common]` in order

18. **TC-16.2.5.4** `test_sort_preserves_entities` — Sort a container; assert no item entity is
    despawned, created, or component-modified — only slot order changes.
    - Input: container with 5 items; capture entity ids; sort
    - Expected: same set of entity ids after sort; no `EntitySpawned`/`EntityDespawned` events

19. **TC-16.2.6.1** `test_socket_tag_match_attach` — Socket requires tag "gem"; attach an item
    tagged "gem". Assert success and occupant set.
    - Input: `Socket { required_tags: ["gem"], occupant: None, .. }`, item with tag set `{"gem"}`
    - Expected: `attach()` returns `Ok`; `socket.occupant == Some(item_entity)`

20. **TC-16.2.6.2** `test_socket_tag_mismatch_reject` — Socket requires tag "gem"; attempt to attach
    an item tagged "weapon". Assert rejection with `TagMismatch`.
    - Input: `Socket { required_tags: ["gem"], .. }`, item with tag set `{"weapon"}`
    - Expected: `Err(TransferError::TagMismatch { required: ["gem"], provided: ["weapon"] })`;
      occupant remains `None`

21. **TC-16.2.6.3** `test_socket_offset_stored` — Construct a socket with offset `(0, 1, 0)`; assert
    offset is preserved through attach/detach cycles.
    - Input: `Socket { transform_offset: Vec3::new(0.0, 1.0, 0.0), .. }`; attach then detach
    - Expected: offset remains `Vec3::new(0.0, 1.0, 0.0)` after both operations

22. **TC-16.2.9.1** `test_transfer_capacity_check` — Validate transfer into a full container. Assert
    validator returns `ContainerFull` error before any mutation occurs.
    - Input: full capacity-4 container; `validate_transfer(item, target)`
    - Expected: `Err(TransferError::ContainerFull)`; container state unchanged

23. **TC-16.2.9.2** `test_transfer_weight_check` — Validate transfer that would exceed weight limit.
    Assert validator returns `OverWeight` error.
    - Input: container weight 45.0/50.0; transferring item of weight 10.0
    - Expected: `Err(TransferError::OverWeight { limit: 50.0, attempted: 55.0 })`

24. **TC-16.2.9.3** `test_transfer_tag_check` — Validate transfer of an item without required tags
    into a tag-gated container. Assert `TagMismatch` error.
    - Input: container `required_tags = ["consumable"]`; item tags `["weapon"]`
    - Expected: `Err(TransferError::TagMismatch { .. })`

## Integration Tests

| ID            | Name                              | Req       |
|---------------|-----------------------------------|-----------|
| TC-16.2.I.1   | `test_socket_propagates_stats`    | R-16.2.7  |
| TC-16.2.I.2   | `test_socket_visual_binding`      | R-16.2.8  |
| TC-16.2.I.3   | `test_crafting_consume_atomic`    | R-16.2.10 |
| TC-16.2.I.4   | `test_crafting_insufficient_fail` | R-16.2.10 |
| TC-16.2.I.5   | `test_drag_drop_full_pipeline`    | R-16.2.9  |
| TC-16.2.I.6   | `test_grid_pickup_loot_burst`     | R-16.2.2  |
| TC-16.2.I.7   | `test_nested_bag_serialization`   | R-16.2.4  |

1. **TC-16.2.I.1** `test_socket_propagates_stats` — Socket an item granting `+10` to the parent
   "strength" attribute. Assert parent shows the modifier; unsocket; assert removed.
   - Input: parent entity with `AttributeSet { strength: 50 }`; gem with `propagate_stats: true` and
     `[FlatAdd 10 -> strength]`
   - Expected: while socketed `strength.read() == 60`; after unsocket `strength.read() == 50`; no
     stale modifiers in parent's stack

2. **TC-16.2.I.2** `test_socket_visual_binding` — Attach a mesh item to a socket with offset
   `(0, 1, 0)`. Assert rendered mesh world transform equals parent transform composed with offset.
   - Input: parent `Transform { translation: (5, 0, 0) }`; socket offset `(0, 1, 0)`; attached mesh
   - Expected: mesh world transform translation == `(5, 1, 0)`; rotation/scale follow parent

3. **TC-16.2.I.3** `test_crafting_consume_atomic` — Recipe needs 3 iron; container has 5 iron; craft
   completes. Assert exactly 3 consumed and output item produced.
   - Input: container with one stack `(iron, 5)`; `Recipe { inputs: [(iron, 3)], output: ingot }`
   - Expected: container has `(iron, 2)` and one `ingot`; one `CraftCompleted` event

4. **TC-16.2.I.4** `test_crafting_insufficient_fail` — Recipe needs 3 iron; container has 2 iron.
   Attempt craft. Assert rejection and zero consumption.
   - Input: container with one stack `(iron, 2)`; recipe needs 3
   - Expected: craft returns `Err(CraftError::InsufficientIngredients)`; container still has
     `(iron, 2)`; no output produced

5. **TC-16.2.I.5** `test_drag_drop_full_pipeline` — Issue a drag-drop transfer through the validator
   → mutation → ECS event channel pipeline. Assert source and target containers update atomically.
   - Input: source container `[A, B, C]`; target container empty; transfer B
   - Expected: source has `[A, C]`; target has `[B]`; `ItemTransferred { from, to, item: B }` event
     emitted exactly once

6. **TC-16.2.I.6** `test_grid_pickup_loot_burst` — Drop 50 mixed-footprint loot items into a
   100-cell grid container in a single frame. Assert all fit and no orphan entities exist.
   - Input: 50 items with footprints in `[1x1, 2x1, 1x2, 2x2]`; empty 10x10 grid
   - Expected: all 50 placed without overlap; total occupied cells equals sum of item footprints

7. **TC-16.2.I.7** `test_nested_bag_serialization` — Save and restore a 3-deep nested container
   chain via the rkyv save format. Assert hierarchy and item ids round-trip.
   - Input: bag A contains bag B contains bag C contains 3 items
   - Expected: after save+load, parent-child links match original; item ids preserved; depth count
     == 3

## Benchmarks

| ID            | Benchmark                          | Target    | Req       |
|---------------|------------------------------------|-----------|-----------|
| TC-16.2.B.1   | Transfer validate (100 containers) | < 0.1 ms  | R-16.2.9  |
| TC-16.2.B.2   | 1,000 transfers batch              | < 100 ms  | R-16.2.9  |
| TC-16.2.B.3   | Grid bin-pack (1,000 items)        | < 5 ms    | R-16.2.2  |
| TC-16.2.B.4   | Sort 256-slot container            | < 50 µs   | R-16.2.5  |
| TC-16.2.B.5   | Stack merge scan (256 slots)       | < 10 µs   | R-16.2.3  |

1. **TC-16.2.B.1** — Single transfer validation across 100 candidate containers (capacity, weight,
   tag, nesting). Mean wall time per validation. Measured with `criterion`.

2. **TC-16.2.B.2** — Batch of 1,000 valid transfers across 100 containers. Total wall time end-
   to-end including validator and mutation phases.

3. **TC-16.2.B.3** — Bin-pack 1,000 items with random footprints into a 50x50 grid container. Wall
   time for full insertion sequence; first-fit row-major scanner.

4. **TC-16.2.B.4** — Sort a 256-slot container by weight ascending. Wall time for one
   `SortRequest::apply` call; must not despawn or modify item entities.

5. **TC-16.2.B.5** — Stack merge scan over a 256-slot container searching for a compatible stack to
   merge into. Wall time for one `find_mergeable_slot` call.
