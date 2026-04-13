# Scripting ↔ UI Integration Test Cases

All tests are CI-runnable in a headless harness. Logic graphs used here are constructed
programmatically (same shape that the graph editor would codegen).

## Integration Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-4.5.1.1 | Script sets widget text | IR-4.5.1 |
| TC-IR-4.5.1.2 | Script sets widget visibility | IR-4.5.1 |
| TC-IR-4.5.1.3 | Script sets progress fraction | IR-4.5.1 |
| TC-IR-4.5.1.4 | Script sets image asset | IR-4.5.1 |
| TC-IR-4.5.2.1 | Click fires `WidgetEvent::Clicked` | IR-4.5.2 |
| TC-IR-4.5.2.2 | Script handler runs on click event | IR-4.5.2 |
| TC-IR-4.5.2.3 | ValueChanged carries new value | IR-4.5.2 |
| TC-IR-4.5.3.1 | Component binding resolves at layout | IR-4.5.3 |
| TC-IR-4.5.3.2 | Resource binding resolves at layout | IR-4.5.3 |
| TC-IR-4.5.3.3 | Data table binding resolves at layout | IR-4.5.3 |
| TC-IR-4.5.3.4 | `OnChange` mode skips unchanged | IR-4.5.3 |
| TC-IR-4.5.4.1 | Dialogue choice flows to quest | IR-4.5.4 |
| TC-IR-4.5.4.2 | Choice index encoded correctly | IR-4.5.4 |
| TC-IR-4.5.5.1 | Formatted string args pass through | IR-4.5.5 |
| TC-IR-4.5.6.1 | `visible_expr=false` hides widget | IR-4.5.6 |
| TC-IR-4.5.6.2 | `enabled_expr=false` disables widget | IR-4.5.6 |
| TC-IR-4.5.6.3 | Hidden widget skipped in layout | IR-4.5.6 |

## Negative Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-4.5.2.N1 | Event to despawned widget (FM-1) | IR-4.5.2 |
| TC-IR-4.5.3.N1 | Binding type mismatch (FM-2) | IR-4.5.3 |
| TC-IR-4.5.4.N1 | `CH-9` full backpressure (FM-3) | IR-4.5.4 |
| TC-IR-4.5.1.N1 | Script panic caught (FM-4) | IR-4.5.1 |
| TC-IR-4.5.6.N1 | Missing expr dep default visible (FM-5) | IR-4.5.6 |
| TC-IR-4.5.3.N2 | Binding source not found (FM-6) | IR-4.5.3 |
| TC-IR-4.5.4.N2 | Orphan dialogue choice (FM-7) | IR-4.5.4 |

### Test case details

1. **TC-IR-4.5.1.1** -- Input: `SetWidgetState { kind: Text, value: literal("Hi") }`. Expected:
   target widget's text component becomes "Hi" after UI layout.
2. **TC-IR-4.5.1.2** -- Input: `SetWidgetState { kind: Visible, value: literal(false) }`. Expected:
   widget `Visible` component = false after apply.
3. **TC-IR-4.5.1.3** -- Input: `SetWidgetState { kind: ProgressFraction, value: literal(0.42) }`.
   Expected: progress bar fraction = 0.42.
4. **TC-IR-4.5.1.4** -- Input:
   `SetWidgetState { kind: ImageAsset, value: literal(AssetId::from(7)) }`. Expected: image
   component = asset 7.
5. **TC-IR-4.5.2.1** -- Input: simulate click on button. Expected:
   `WidgetEvent { target, kind: Clicked }` emitted into ECS event bus.
6. **TC-IR-4.5.2.2** -- Input: script handler registered on button click. Expected: handler runs in
   Phase 3 Simulation next frame; handler body observes the event.
7. **TC-IR-4.5.2.3** -- Input: slider value dragged to 0.75. Expected:
   `WidgetEvent { kind: ValueChanged { new: 0.75 } }`.
8. **TC-IR-4.5.3.1** -- Input: binding path `player.hp` reading component `Health::current`.
   Expected: widget text shows `Health::current` value; matches ECS value after tick.
9. **TC-IR-4.5.3.2** -- Input: binding source `Resource`, path `game_time.tick`. Expected: text
   matches tick count.
10. **TC-IR-4.5.3.3** -- Input: binding source `DataTableRow`, row id `weapon.sword`, field
    `damage`. Expected: text matches table value.
11. **TC-IR-4.5.3.4** -- Input: `OnChange` mode; binding value unchanged for 60 frames. Expected:
    binding resolved once; subsequent frames skip the eval.
12. **TC-IR-4.5.4.1** -- Input: player clicks choice option 2. Expected:
    `DialogueChoice { choice_index: 2 }` enqueued on `CH-9`; quest drains.
13. **TC-IR-4.5.4.2** -- Input: 4 choice buttons, click third. Expected: `choice_index == 2`
    (0-based).
14. **TC-IR-4.5.5.1** -- Input: script builds `FormattedString` with arg `n=3`; widget uses plural
    `"{n, plural, one{# item} other{# items}}"`. Expected: widget text "3 items".
15. **TC-IR-4.5.6.1** -- Input: `visible_expr` evaluates false. Expected: widget absent from layout
    output.
16. **TC-IR-4.5.6.2** -- Input: `enabled_expr` evaluates false. Expected: widget present but
    `InteractionState.disabled = true`; no `Clicked` events reach handler.
17. **TC-IR-4.5.6.3** -- Input: 20 widgets, 10 with `visible=false`. Expected: layout walks 10
    visible widgets only.
18. **TC-IR-4.5.2.N1** -- Input: event for entity despawned same frame. Expected: event dropped;
    `FM-1` counter increments.
19. **TC-IR-4.5.3.N1** -- Input: binding to a `u32` field but widget expects `Text`. Expected:
    widget renders `<err>`; `FM-2` counter increments.
20. **TC-IR-4.5.4.N1** -- Input: 32 dialogue choices against `CH-9` cap=16. Expected: UI worker
    parks briefly; eventually delivered; `FM-3` counter increments.
21. **TC-IR-4.5.1.N1** -- Input: script handler panics. Expected: caught; error banner raised;
    handler marked disabled; `FM-4` counter increments.
22. **TC-IR-4.5.6.N1** -- Input: `visible_expr` references a missing dep. Expected: widget defaults
    to `visible=true`; `FM-5` counter increments.
23. **TC-IR-4.5.3.N2** -- Input: binding to non-existent resource. Expected: widget renders
    `<missing>`; `FM-6` counter increments.
24. **TC-IR-4.5.4.N2** -- Input: dialogue choice for dialogue already finished. Expected: dropped;
    logged; `FM-7` counter increments.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-4.5.1.B1 | Apply 50 SetWidgetState | < 0.02 ms | IR-4.5.1 |
| TC-IR-4.5.3.B1 | Resolve 500 bindings | < 0.1 ms | IR-4.5.3 |
| TC-IR-4.5.6.B1 | Visibility eval 500 widgets | < 0.05 ms | IR-4.5.6 |
| TC-IR-4.5.2.B1 | Dispatch 100 WidgetEvents to handlers | < 0.05 ms | IR-4.5.2 |

All benchmarks run under `cargo bench` in CI; thresholds enforced via the benchmark harness.
