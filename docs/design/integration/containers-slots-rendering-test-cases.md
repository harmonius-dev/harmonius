# Containers/Slots ↔ Rendering Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-5.8.1.1 | Sword renders at hand socket | Attach sword to right_hand socket | Sword mesh at hand world transform + offset | IR-5.8.1 |
| TC-IR-5.8.1.2 | Helmet renders at head socket | Attach helmet to head socket | Helmet mesh at head bone transform | IR-5.8.1 |
| TC-IR-5.8.1.3 | Detach removes rendered item | Detach sword from socket | Sword mesh no longer in draw list | IR-5.8.1 |
| TC-IR-5.8.2.1 | Mesh override on attach | VisualOverride with mesh_override set | Socket entity renders override mesh | IR-5.8.2 |
| TC-IR-5.8.2.2 | Material override on attach | VisualOverride with material_override | Socket entity uses override material | IR-5.8.2 |
| TC-IR-5.8.2.3 | Override reverts on detach | Detach item with VisualOverride | Original mesh/material restored | IR-5.8.2 |
| TC-IR-5.8.3.1 | Socket debug spheres in editor | Enable socket debug mode | Sphere gizmos at each socket position | IR-5.8.3 |
| TC-IR-5.8.3.2 | Socket names in debug overlay | Enable socket debug mode | Text labels at each socket position | IR-5.8.3 |
| TC-IR-5.8.4.1 | Hide socket visual on attach | hide_socket_visual = true | Socket base mesh hidden | IR-5.8.4 |
| TC-IR-5.8.4.2 | Show socket visual on detach | Detach item, hide was true | Socket base mesh visible again | IR-5.8.4 |
| TC-IR-5.8.5.1 | Snap preview ghost mesh | Drag item near compatible socket | Translucent ghost at snap position | IR-5.8.5 |
| TC-IR-5.8.5.2 | Snap preview disappears | Drag item away from socket | Ghost mesh removed | IR-5.8.5 |
| TC-IR-5.8.6.1 | Equipment change updates proxy | Swap weapon in equipment slot | Render proxy updated same frame | IR-5.8.6 |
| TC-IR-5.8.6.2 | Batch equipment swap | Swap 8 equipment slots at once | All 8 proxies updated, no flicker | IR-5.8.6 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-5.8.1.B1 | VisualBinding 64 sockets | < 0.5 ms | IR-5.8.1 |
| TC-IR-5.8.5.B1 | Snap query 500 sockets | < 2 ms | IR-5.8.5 |
| TC-IR-5.8.6.B1 | Proxy update 100 equipment changes | < 0.2 ms | IR-5.8.6 |
