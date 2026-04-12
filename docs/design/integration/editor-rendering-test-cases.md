# Editor ↔ Rendering Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-5.5.1.1 | Viewport renders scene | Scene with 100 entities | All meshes visible in viewport | IR-5.5.1 |
| TC-IR-5.5.1.2 | Viewport camera orbit | Drag middle mouse button | Camera orbits around pivot point | IR-5.5.1 |
| TC-IR-5.5.2.1 | Translate gizmo visible | Select entity, translate mode | RGB arrows at entity origin | IR-5.5.2 |
| TC-IR-5.5.2.2 | Rotate gizmo visible | Select entity, rotate mode | RGB circles at entity origin | IR-5.5.2 |
| TC-IR-5.5.2.3 | Scale gizmo visible | Select entity, scale mode | RGB cubes at entity origin | IR-5.5.2 |
| TC-IR-5.5.2.4 | Gizmo transform with snap | Drag X axis, snap=1.0 | Entity moves in 1.0 increments on X | IR-5.5.2 |
| TC-IR-5.5.2.5 | Gizmo undo/redo | Move entity via gizmo, undo | Entity returns to original position | IR-5.5.2 |
| TC-IR-5.5.3.1 | Selection outline single | Click entity in viewport | Orange outline around entity mesh | IR-5.5.3 |
| TC-IR-5.5.3.2 | Selection outline multiple | Marquee select 5 entities | All 5 have outlines | IR-5.5.3 |
| TC-IR-5.5.3.3 | Selection raycast accuracy | Click on overlapping meshes | Nearest mesh selected | IR-5.5.3 |
| TC-IR-5.5.4.1 | Wireframe overlay toggle | Enable wireframe mode | All meshes show wireframe | IR-5.5.4 |
| TC-IR-5.5.4.2 | Normal visualization | Enable world normals mode | Normals rendered as RGB colors | IR-5.5.4 |
| TC-IR-5.5.5.1 | Dual viewport layout | Split viewport into two | Two independent camera views | IR-5.5.5 |
| TC-IR-5.5.5.2 | Viewport camera independence | Orbit one viewport | Other viewport unchanged | IR-5.5.5 |
| TC-IR-5.5.6.1 | Buffer vis albedo | Select albedo buffer mode | Only albedo channel displayed | IR-5.5.6 |
| TC-IR-5.5.6.2 | Buffer vis overdraw | Select overdraw mode | Overdraw heatmap displayed | IR-5.5.6 |
| TC-IR-5.5.7.1 | Editor grid renders | Open viewport | Infinite grid on XZ plane | IR-5.5.7 |
| TC-IR-5.5.7.2 | Measurement gizmo | Activate measurement tool | Distance label between two points | IR-5.5.7 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-5.5.1.B1 | Viewport render 10K entities | < 16 ms total frame | IR-5.5.1 |
| TC-IR-5.5.2.B1 | Gizmo overlay 100 selected | < 0.5 ms GPU | IR-5.5.2 |
| TC-IR-5.5.3.B1 | Selection raycast 50K meshlets | < 2 ms CPU | IR-5.5.3 |
| TC-IR-5.5.3.B2 | Outline pass 256 entities | < 0.5 ms GPU | IR-5.5.3 |
| TC-IR-5.5.4.B1 | Debug overlay zero shipping cost | 0 ms in shipping build | IR-5.5.4 |
