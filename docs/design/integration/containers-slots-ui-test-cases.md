# Containers/Slots ↔ UI Integration Test Cases

## Upstream Requirements Trace

Each integration requirement below corresponds to an upstream container-system requirement and user
story. Regressions in any test case below also constitute regressions against the listed upstream
IDs.

| IR-ID    | Upstream R-ID | Upstream US-ID |
|----------|---------------|----------------|
| IR-5.9.1 | R-16.2.1      | US-16.2.1      |
| IR-5.9.2 | R-16.2.6      | US-16.2.6      |
| IR-5.9.3 | R-16.2.9      | US-16.2.9      |
| IR-5.9.4 | R-16.2.3      | US-16.2.3      |
| IR-5.9.5 | R-16.2.2      | US-16.2.2      |
| IR-5.9.7 | R-16.2.5      | US-16.2.5      |

## Unit Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-5.9.1.U1 | SmallVec spill threshold | IR-5.9.1 |
| TC-IR-5.9.1.U2 | Binding width/height `u16` bounds | IR-5.9.1 |
| TC-IR-5.9.3.U1 | `UiTransferRequest` quantity 0 = full stack | IR-5.9.3 |
| TC-IR-5.9.3.U2 | `UiTransferRequest` auto-place dest slot | IR-5.9.3 |
| TC-IR-5.9.4.U1 | Split clamp lower bound | IR-5.9.4 |
| TC-IR-5.9.4.U2 | Split clamp upper bound | IR-5.9.4 |
| TC-IR-5.9.7.U1 | `FilterCriteria::None` passes all | IR-5.9.7 |
| TC-IR-5.9.7.U2 | `FilterCriteria::ByTag` predicate | IR-5.9.7 |
| TC-IR-5.9.7.U3 | Sort stable ordering | IR-5.9.7 |

1. **TC-IR-5.9.1.U1** -- Input: 33 slot widgets. Expected: heap spill, no panic.
2. **TC-IR-5.9.1.U2** -- Input: `grid_width = 0xFFFF`. Expected: accepted, no overflow.
3. **TC-IR-5.9.3.U1** -- Input: `quantity = 0`. Expected: full source stack moved.
4. **TC-IR-5.9.3.U2** -- Input: `dest_slot = u16::MAX`. Expected: placed in any free slot.
5. **TC-IR-5.9.4.U1** -- Input: `split qty = 0`. Expected: clamped to 1.
6. **TC-IR-5.9.4.U2** -- Input: `split qty = src.qty + 1`. Expected: clamped to `src.qty`.
7. **TC-IR-5.9.7.U1** -- Input: 20 slots, no filter. Expected: 20 visible.
8. **TC-IR-5.9.7.U2** -- Input: tag `Weapon` filter. Expected: weapons only visible.
9. **TC-IR-5.9.7.U3** -- Input: equal sort keys. Expected: original relative order preserved.

## Integration Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-5.9.1.1 | Grid displays 20 items | IR-5.9.1 |
| TC-IR-5.9.1.2 | Grid updates on item add | IR-5.9.1 |
| TC-IR-5.9.1.3 | Grid updates on item remove | IR-5.9.1 |
| TC-IR-5.9.1.4 | Stack quantity display | IR-5.9.1 |
| TC-IR-5.9.2.1 | Equipment slots show attachments | IR-5.9.2 |
| TC-IR-5.9.2.2 | Equipment slot compatibility hint | IR-5.9.2 |
| TC-IR-5.9.2.3 | Equipment slot shows socket name | IR-5.9.2 |
| TC-IR-5.9.3.1 | Drag item between containers | IR-5.9.3 |
| TC-IR-5.9.3.2 | Drag to full container fails | IR-5.9.3 |
| TC-IR-5.9.3.3 | Drag ghost preview follows cursor | IR-5.9.3 |
| TC-IR-5.9.3.4 | Drop target highlight | IR-5.9.3 |
| TC-IR-5.9.3.5 | One-frame latency bound | IR-5.9.3 |
| TC-IR-5.9.4.1 | Stack split via modifier + drag | IR-5.9.4 |
| TC-IR-5.9.4.2 | Split clamped to stack size | IR-5.9.4 |
| TC-IR-5.9.4.3 | Split minimum is 1 | IR-5.9.4 |
| TC-IR-5.9.5.1 | 2x2 item occupies 4 cells | IR-5.9.5 |
| TC-IR-5.9.5.2 | 2x2 item blocked by occupied cell | IR-5.9.5 |
| TC-IR-5.9.6.1 | Hover shows item tooltip | IR-5.9.6 |
| TC-IR-5.9.6.2 | Tooltip dismissed on mouse exit | IR-5.9.6 |
| TC-IR-5.9.7.1 | Sort by name | IR-5.9.7 |
| TC-IR-5.9.7.2 | Sort by rarity | IR-5.9.7 |
| TC-IR-5.9.7.3 | Filter by type | IR-5.9.7 |
| TC-IR-5.9.7.4 | Filter does not mutate container | IR-5.9.7 |

1. **TC-IR-5.9.1.1** -- Input: container with 20 filled slots. Expected: 20 icons in grid cells.
2. **TC-IR-5.9.1.2** -- Input: add item to container. Expected: new icon in next free cell.
3. **TC-IR-5.9.1.3** -- Input: remove item from container. Expected: cell becomes empty.
4. **TC-IR-5.9.1.4** -- Input: slot with `quantity = 5`. Expected: "5" label on item icon.
5. **TC-IR-5.9.2.1** -- Input: 6 sockets, 4 with items. Expected: 4 show icons, 2 empty.
6. **TC-IR-5.9.2.2** -- Input: drag incompatible item over slot. Expected: slot border turns red.
7. **TC-IR-5.9.2.3** -- Input: head socket. Expected: "Head" label on slot widget.
8. **TC-IR-5.9.3.1** -- Input: drag from bag to chest. Expected: item in chest, removed from bag.
9. **TC-IR-5.9.3.2** -- Input: drag to full container. Expected: error feedback, stays in source.
10. **TC-IR-5.9.3.3** -- Input: begin drag. Expected: translucent item icon at cursor.
11. **TC-IR-5.9.3.4** -- Input: drag over valid slot. Expected: slot border highlights green.
12. **TC-IR-5.9.3.5** -- Input: transfer requested in frame N. Expected: visible in UI by N+1.
13. **TC-IR-5.9.4.1** -- Input: shift+drag stack of 10. Expected: split dialog, enter 5.
14. **TC-IR-5.9.4.2** -- Input: try split 15 from stack of 10. Expected: clamped to 10.
15. **TC-IR-5.9.4.3** -- Input: try split 0. Expected: clamped to 1.
16. **TC-IR-5.9.5.1** -- Input: place 2x2 item in grid. Expected: `GridOccupancy` marks 4 cells.
17. **TC-IR-5.9.5.2** -- Input: place 2x2 with 1 cell taken. Expected: transfer denied.
18. **TC-IR-5.9.6.1** -- Input: hover over sword icon. Expected: tooltip with name, stats, weight.
19. **TC-IR-5.9.6.2** -- Input: move mouse away. Expected: tooltip disappears.
20. **TC-IR-5.9.7.1** -- Input: click "Sort by Name". Expected: items reordered alphabetically.
21. **TC-IR-5.9.7.2** -- Input: click "Sort by Rarity". Expected: items reordered by rarity tier.
22. **TC-IR-5.9.7.3** -- Input: select "Weapons" filter. Expected: only weapon items visible.
23. **TC-IR-5.9.7.4** -- Input: apply filter. Expected: underlying `Container` unchanged.

## Failure-Mode Tests (Negative)

| ID | Failure Mode | Req |
|----|--------------|-----|
| TC-IR-5.9.3.F1 | Capacity denial | IR-5.9.3 |
| TC-IR-5.9.3.F2 | Constraint denial | IR-5.9.3 |
| TC-IR-5.9.4.F1 | Split to zero | IR-5.9.4 |
| TC-IR-5.9.1.F1 | Container despawned mid-frame | IR-5.9.1 |
| TC-IR-5.9.5.F1 | Grid occupancy desync | IR-5.9.5 |
| TC-IR-5.9.3.F3 | Network rollback after transfer | IR-5.9.3 |

1. **TC-IR-5.9.3.F1** -- Input: drag to full container. Expected: `InsufficientCapacity` feedback,
   red shake animation.
2. **TC-IR-5.9.3.F2** -- Input: drag tagged item to rejecting slot. Expected: `ConstraintViolation`
   feedback, "Incompatible" tooltip.
3. **TC-IR-5.9.4.F1** -- Input: `split qty = 0`. Expected: clamped to 1, treated as a full-move, no
   panic.
4. **TC-IR-5.9.1.F1** -- Input: despawn bound `Container` entity. Expected: panel closes gracefully,
   no panic, no stale queries.
5. **TC-IR-5.9.5.F1** -- Input: corrupt `GridOccupancy` map. Expected: rebuilt from `ContainerSlot`
   list, warning logged once per container.
6. **TC-IR-5.9.3.F3** -- Input: server rejects previously-accepted transfer. Expected: UI animates
   smooth rollback, no flicker.

## Fallback-Path Tests (Negative)

| ID | Fallback | Req |
|----|----------|-----|
| TC-IR-5.9.1.FB1 | Bound container despawned | IR-5.9.1 |
| TC-IR-5.9.3.FB1 | Unknown slot index | IR-5.9.3 |
| TC-IR-5.9.3.FB2 | EventBridge channel full | IR-5.9.3 |
| TC-IR-5.9.7.FB1 | Sort comparator panics | IR-5.9.7 |
| TC-IR-5.9.1.FB2 | Asset icon missing | IR-5.9.1 |
| TC-IR-5.9.6.FB1 | Tooltip row missing | IR-5.9.6 |
| TC-IR-5.9.7.FB2 | Filter substring invalid UTF-8 | IR-5.9.7 |

1. **TC-IR-5.9.1.FB1** -- Input: despawn bound `Container`. Expected: bindings dropped, panel
   closes, info log emitted.
2. **TC-IR-5.9.3.FB1** -- Input: `source_slot = u16::MAX - 1` with slot count 32. Expected:
   `TransferResult::InvalidSlot`, red shake feedback.
3. **TC-IR-5.9.3.FB2** -- Input: flood 2000 `UiTransferRequest` events in one frame. Expected:
   oldest dropped past buffer length 1024, warning logged.
4. **TC-IR-5.9.7.FB1** -- Input: `SortCriteria::Custom(_)` comparator panics. Expected: sort aborts,
   prior slot order preserved, error logged.
5. **TC-IR-5.9.1.FB2** -- Input: unresolved `AssetHandle<UiIcon>`. Expected: placeholder icon drawn,
   warning logged once per asset.
6. **TC-IR-5.9.6.FB1** -- Input: item definition row deleted. Expected: tooltip displays
   `"<missing>"`, warning logged once per row.
7. **TC-IR-5.9.7.FB2** -- Input: `FilterCriteria::ByName(StringId)` with invalid UTF-8. Expected:
   treated as empty string, warning logged once.

## Timing Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-5.9.3.T1 | Fixed-to-variable handoff | IR-5.9.3 |
| TC-IR-5.9.3.T2 | No variable tick between fixed | IR-5.9.3 |
| TC-IR-5.9.3.T3 | Render handoff ring slot | IR-5.9.3 |

1. **TC-IR-5.9.3.T1** -- Input: `TransferSystem` runs at fixed 60 Hz, UI at variable 240 Hz.
   Expected: `InventoryBindingSystem` sees the container change on the next variable tick.
2. **TC-IR-5.9.3.T2** -- Input: pause variable UI loop for one frame. Expected: previous binding
   state persists unchanged, no stale data.
3. **TC-IR-5.9.3.T3** -- Input: worker paints frame N. Expected: render thread reads exact ring slot
   index via the MPSC channel, GPU submission matches.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-5.9.1.B1 | Inventory grid 500 items render | < 2 ms | IR-5.9.1 |
| TC-IR-5.9.1.B2 | Binding update 100 containers | < 0.5 ms | IR-5.9.1 |
| TC-IR-5.9.3.B1 | Drag-drop transfer validation | < 0.1 ms | IR-5.9.3 |
| TC-IR-5.9.3.B2 | EventBridge route 1024 events | < 0.2 ms | IR-5.9.3 |
| TC-IR-5.9.5.B1 | Grid bin-pack 500 items | < 1 ms | IR-5.9.5 |
| TC-IR-5.9.7.B1 | Sort 500 items (pdqsort) | < 1 ms | IR-5.9.7 |
| TC-IR-5.9.7.B2 | Filter 500 items linear scan | < 0.25 ms | IR-5.9.7 |

1. **TC-IR-5.9.1.B1** -- 500-item inventory grid layout + paint on worker thread.
2. **TC-IR-5.9.1.B2** -- Binding update for 100 changed containers in one variable tick.
3. **TC-IR-5.9.3.B1** -- `TransferSystem::validate_transfer` on a representative 32-slot container.
4. **TC-IR-5.9.3.B2** -- Full buffer of 1024 events routed through `EventBridge<T>`.
5. **TC-IR-5.9.5.B1** -- Shelf-best-fit bin pack for 500 items of mixed sizes.
6. **TC-IR-5.9.7.B1** -- `pdqsort` over 500 `ContainerSlot` entries by `SortCriteria::Name`.
7. **TC-IR-5.9.7.B2** -- Linear filter scan over 500 `ContainerSlot` entries.

## CI Execution

All tests above are CI-runnable with no GPU or display dependency. Paint and submit stages use a
headless quad batcher; render-thread handoff tests verify only the channel protocol, not GPU
submission. Benchmarks execute under `cargo bench` on a fixed-hardware CI runner.
