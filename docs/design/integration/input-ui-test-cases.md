# Input ↔ UI Framework Integration Test Cases

All tests in this file are CI-runnable unit or integration tests. Negative test cases are tagged
with a `(neg)` marker in the Test column. Edge cases added per review are TC-IR-4.2.1.4 through
TC-IR-4.2.7.3.

## Integration Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-4.2.1.1 | Pointer hover triggers Enter | IR-4.2.1 |
| TC-IR-4.2.1.2 | Pointer click triggers press | IR-4.2.1 |
| TC-IR-4.2.1.3 | Pointer leave clears hover | IR-4.2.1 |
| TC-IR-4.2.1.4 | Overlapping widgets: topmost z-order wins | IR-4.2.1 |
| TC-IR-4.2.1.5 | Disabled widget ignores target events (neg) | IR-4.2.1 |
| TC-IR-4.2.1.6 | Simultaneous touch + mouse routed independently | IR-4.2.1 |
| TC-IR-4.2.1.7 | Capture phase stops dispatch before target (neg) | IR-4.2.1 |
| TC-IR-4.2.1.8 | Bubble phase reaches root ancestors | IR-4.2.1 |
| TC-IR-4.2.2.1 | Tab advances focus | IR-4.2.2 |
| TC-IR-4.2.2.2 | Shift+Tab reverses focus | IR-4.2.2 |
| TC-IR-4.2.2.3 | Arrow keys navigate directional | IR-4.2.2 |
| TC-IR-4.2.3.1 | Gamepad south confirms | IR-4.2.3 |
| TC-IR-4.2.3.2 | Gamepad east cancels | IR-4.2.3 |
| TC-IR-4.2.3.3 | DPad wrap-around at focus group boundary | IR-4.2.3 |
| TC-IR-4.2.4.1 | Pan gesture scrolls | IR-4.2.4 |
| TC-IR-4.2.4.2 | Swipe triggers drag | IR-4.2.4 |
| TC-IR-4.2.4.3 | Pinch zooms ScrollView content | IR-4.2.4 |
| TC-IR-4.2.4.4 | Pinch Cancelled reverts to Began scale (neg) | IR-4.2.4 |
| TC-IR-4.2.5.1 | UI context blocks gameplay | IR-4.2.5 |
| TC-IR-4.2.5.2 | Pop restores gameplay | IR-4.2.5 |
| TC-IR-4.2.5.3 | Chat window: keyboard consumed, mouse allowed | IR-4.2.5 |
| TC-IR-4.2.6.1 | VR laser hits world-space panel | IR-4.2.6 |
| TC-IR-4.2.6.2 | VR laser misses panels (neg) | IR-4.2.6 |
| TC-IR-4.2.6.3 | VR trigger_pressed fires UiPointerEvent::Down | IR-4.2.6 |
| TC-IR-4.2.7.1 | Key press routes to TextInput | IR-4.2.7 |
| TC-IR-4.2.7.2 | Keys blocked from gameplay (neg) | IR-4.2.7 |
| TC-IR-4.2.7.3 | IME composition commit on focus loss | IR-4.2.7 |
| TC-IR-4.2.7.4 | IME composition cancel on focus loss (neg) | IR-4.2.7 |
| TC-IR-4.2.8.1 | Menu open pushes context | IR-4.2.8 |
| TC-IR-4.2.8.2 | Menu close pops context | IR-4.2.8 |
| TC-IR-4.2.8.3 | ContextStack underflow logs warning (neg) | IR-4.2.8 |

### Test case details

1. **TC-IR-4.2.1.1** -- Input: `MouseState.position` inside button rect; Expected:
   `InteractionState.hovered = true` on target; `UiPointerEvent::Enter` fired.
2. **TC-IR-4.2.1.2** -- Input: `MouseButton::Left` down inside button rect; Expected:
   `InteractionState.pressed = true`; `UiPointerEvent::Down` fired.
3. **TC-IR-4.2.1.3** -- Input: Mouse exits button rect; Expected:
   `InteractionState.hovered = false`; `UiPointerEvent::Leave` fired.
4. **TC-IR-4.2.1.4** -- Input: Two overlapping buttons, click at overlap; Expected: Topmost widget
   (highest `z_order`) receives target event; lower widget receives no target event.
5. **TC-IR-4.2.1.5** -- Input: Click on disabled button; Expected: No target event delivered;
   capture phase still visits ancestors; `InteractionState.pressed` stays false.
6. **TC-IR-4.2.1.6** -- Input: Touch pointer id 1 at (10, 10) and mouse at (50, 50) same frame;
   Expected: Two distinct `UiPointerEvent` streams, each routed to its own hit target.
7. **TC-IR-4.2.1.7** -- Input: Ancestor handler sets `handled = true` during Capture; Expected:
   Target and Bubble phases skipped; only capture ancestors before handler are visited.
8. **TC-IR-4.2.1.8** -- Input: Click leaf widget with three ancestors; Expected: Bubble visits
   parent, grandparent, root in order after target delivery.
9. **TC-IR-4.2.2.1** -- Input: Tab action; Expected: `FocusManager` advances to next focusable
   entity in tab order.
10. **TC-IR-4.2.2.2** -- Input: Shift+Tab action; Expected: Focus moves to previous focusable.
11. **TC-IR-4.2.2.3** -- Input: DPad right action; Expected: Focus moves to widget with closest
    center along +X within focus group.
12. **TC-IR-4.2.3.1** -- Input: Gamepad South button while button focused; Expected: Widget
    activation event fired.
13. **TC-IR-4.2.3.2** -- Input: Gamepad East button while dialog focused; Expected: Dialog `Close`
    event fired.
14. **TC-IR-4.2.3.3** -- Input: DPad right at rightmost widget in group with `wrap = true`;
    Expected: Focus wraps to leftmost widget in same group.
15. **TC-IR-4.2.4.1** -- Input: `GestureEvent { gesture_type: Pan, delta: Vec2::new(0, 50), .. }`;
    Expected: `ScrollView.offset.y += 50`.
16. **TC-IR-4.2.4.2** -- Input: `GestureEvent { gesture_type: Swipe { direction: Up }, .. }`;
    Expected: `DragDropManager` initiates drag with upward swipe.
17. **TC-IR-4.2.4.3** -- Input:
    `GestureEvent { gesture_type: Pinch, scale: 1.5, phase: Changed, .. }` on `ScrollView`;
    Expected: `ScrollView.content_scale` becomes `clamp(prev * 1.5, min_zoom, max_zoom)`.
18. **TC-IR-4.2.4.4** -- Input: Pinch `Began` then `Cancelled`; Expected: `content_scale` reverts to
    the value captured at `Began`.
19. **TC-IR-4.2.5.1** -- Input: Push UI `MappingContext` with
    `InputConsumption { pointer: true, keyboard: true, gamepad: true }`, then raw click; Expected:
    Gameplay action mapping does not observe the click.
20. **TC-IR-4.2.5.2** -- Input: Pop UI `MappingContext`; Expected: Gameplay action mapping observes
    subsequent clicks again.
21. **TC-IR-4.2.5.3** -- Input:
    `InputConsumption { pointer: false, keyboard: true, gamepad: false }` active, then mouse-move +
    key press; Expected: Gameplay sees mouse-move but not key press.
22. **TC-IR-4.2.6.1** -- Input: `ControllerPose` ray intersects world-space panel at UV (0.5, 0.5);
    Expected: `UiPointerEvent::Move` fired with panel-local pixel coords.
23. **TC-IR-4.2.6.2** -- Input: Ray misses all panels; Expected: No `UiPointerEvent` emitted;
    `InteractionState.hovered` remains false.
24. **TC-IR-4.2.6.3** -- Input: `VrControllerState.trigger_pressed` transitions false -> true while
    ray hits button; Expected: `UiPointerEvent::Down` with `button = MouseButton::Primary`.
25. **TC-IR-4.2.7.1** -- Input: 'A' key press while `TextInput` focused; Expected: Character 'a'
    inserted in `TextInput.text`.
26. **TC-IR-4.2.7.2** -- Input: 'W' key press while `TextInput` focused; Expected: No `MoveForward`
    gameplay action fired.
27. **TC-IR-4.2.7.3** -- Input: IME composition active, focus moves to another widget, commit policy
    `CommitOnBlur`; Expected: Composition text committed to `TextInput.text`.
28. **TC-IR-4.2.7.4** -- Input: IME composition active, focus moves away, commit policy
    `CancelOnBlur`; Expected: Composition discarded; `TextInput.text` unchanged.
29. **TC-IR-4.2.8.1** -- Input: Open inventory menu; Expected: UI `MappingContext` on top of
    `ContextStack`.
30. **TC-IR-4.2.8.2** -- Input: Close inventory; Expected: Top of stack is the pre-menu gameplay
    context.
31. **TC-IR-4.2.8.3** -- Input: Pop on empty `ContextStack`; Expected: No-op; warning logged through
    `InputDebugFlags.log_consumption`.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-4.2.1.B1 | Hit test 500 widgets | < 0.2 ms | IR-4.2.1 |
| TC-IR-4.2.1.B2 | Capture/bubble dispatch depth 20 | < 0.05 ms | IR-4.2.1 |
| TC-IR-4.2.2.B1 | Focus traversal 100 widgets | < 0.05 ms | IR-4.2.2 |
| TC-IR-4.2.4.B1 | Pinch gesture apply to ScrollView | < 0.02 ms | IR-4.2.4 |
| TC-IR-4.2.5.B1 | Consumption check across 16 contexts | < 0.01 ms | IR-4.2.5 |
| TC-IR-4.2.6.B1 | VR ray vs 10 world panels | < 0.1 ms | IR-4.2.6 |

All benchmarks run under `cargo bench` in CI; thresholds enforced via the benchmark harness.
