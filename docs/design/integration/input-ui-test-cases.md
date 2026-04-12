# Input ↔ UI Framework Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-4.2.1.1 | Pointer hover triggers Enter | Mouse over button | InteractionState.hovered=true | IR-4.2.1 |
| TC-IR-4.2.1.2 | Pointer click triggers press | MouseButton::Left down | InteractionState.pressed=true | IR-4.2.1 |
| TC-IR-4.2.1.3 | Pointer leave clears hover | Mouse exits button rect | InteractionState.hovered=false | IR-4.2.1 |
| TC-IR-4.2.2.1 | Tab advances focus | Tab key action | Focus moves to next widget | IR-4.2.2 |
| TC-IR-4.2.2.2 | Shift+Tab reverses focus | Shift+Tab action | Focus moves to prev widget | IR-4.2.2 |
| TC-IR-4.2.2.3 | Arrow keys navigate directional | DPad right action | Focus moves right | IR-4.2.2 |
| TC-IR-4.2.3.1 | Gamepad south confirms | South button pressed | Focused button activated | IR-4.2.3 |
| TC-IR-4.2.3.2 | Gamepad east cancels | East button pressed | Menu/dialog closes | IR-4.2.3 |
| TC-IR-4.2.4.1 | Pan gesture scrolls | GestureEvent::Pan(dy=50) | ScrollView offset += 50 | IR-4.2.4 |
| TC-IR-4.2.4.2 | Swipe triggers drag | GestureEvent::Swipe | DragDropManager initiates | IR-4.2.4 |
| TC-IR-4.2.5.1 | UI context blocks gameplay | UI MappingContext pushed | Gameplay action not fired | IR-4.2.5 |
| TC-IR-4.2.5.2 | Pop restores gameplay | UI MappingContext popped | Gameplay action fires again | IR-4.2.5 |
| TC-IR-4.2.6.1 | VR laser hits world-space panel | Ray intersects panel | UiPointerEvent dispatched | IR-4.2.6 |
| TC-IR-4.2.6.2 | VR laser misses panels | Ray misses all panels | No UiPointerEvent | IR-4.2.6 |
| TC-IR-4.2.7.1 | Key press routes to TextInput | 'A' key, TextInput focused | Character 'a' inserted | IR-4.2.7 |
| TC-IR-4.2.7.2 | Keys blocked from gameplay | TextInput focused, 'W' key | No movement action fired | IR-4.2.7 |
| TC-IR-4.2.8.1 | Menu open pushes context | Open inventory | UI context on stack top | IR-4.2.8 |
| TC-IR-4.2.8.2 | Menu close pops context | Close inventory | Gameplay context restored | IR-4.2.8 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-4.2.1.B1 | Hit test 500 widgets | < 0.2 ms | IR-4.2.1 |
| TC-IR-4.2.2.B1 | Focus traversal 100 widgets | < 0.05 ms | IR-4.2.2 |
| TC-IR-4.2.6.B1 | VR ray vs 10 world panels | < 0.1 ms | IR-4.2.6 |
