# Rendering ↔ Camera Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-3.1.1.1 | Perspective camera produces valid RenderView | CameraOutput with 60 deg FOV, pos (0,5,-10) | RenderView with correct view and reverse-Z projection matrices | IR-3.1.1 |
| TC-IR-3.1.1.2 | Orthographic camera produces valid RenderView | CameraOutput with ortho size=10 | RenderView with orthographic projection matrix | IR-3.1.1 |
| TC-IR-3.1.2.1 | Render layers filter objects | Camera layers=0x01, entity A layers=0x01, entity B layers=0x02 | Only entity A in draw list | IR-3.1.2 |
| TC-IR-3.1.2.2 | Render layers 0xFF sees all | Camera layers=0xFF, entities with layers 0x01..0x04 | All entities in draw list | IR-3.1.2 |
| TC-IR-3.1.3.1 | PP volume blends per camera | Two cameras, one inside PP volume, one outside | First camera has PP params, second uses defaults | IR-3.1.3 |
| TC-IR-3.1.4.1 | Multi-camera single snapshot | Two CameraBrains active | Two RenderView entries in RenderWorld | IR-3.1.4 |
| TC-IR-3.1.4.2 | Render order respected | Camera A order=0, Camera B order=1 | Camera A renders first | IR-3.1.4 |
| TC-IR-3.1.5.1 | ViewUniform matches CameraOutput | CameraOutput at pos (1,2,3), rot 45 deg Y | ViewUniform.view matches inverse transform | IR-3.1.5 |
| TC-IR-3.1.5.2 | Reverse-Z near/far correct | Near=0.1, far=1000 | Projection[2][2] near 0.0, [2][3] near 0.1 | IR-3.1.5 |
| TC-IR-3.1.6.1 | DRS feeds back to viewport | DRS scale=0.75, base 1920x1080 | Viewport 1440x810 | IR-3.1.6 |
| TC-IR-3.1.6.2 | DRS clamp prevents zero | DRS scale=0.0 | Clamped to min_scale, viewport > 0 | IR-3.1.6 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-3.1.1.B1 | RenderView build from CameraOutput | < 0.01 ms | IR-3.1.1 |
| TC-IR-3.1.4.B1 | 4-camera split-screen extraction | < 0.1 ms total | IR-3.1.4 |
| TC-IR-3.1.5.B1 | ViewUniform GPU upload (4 views) | < 0.05 ms | IR-3.1.5 |
