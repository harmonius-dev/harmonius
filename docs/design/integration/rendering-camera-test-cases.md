# Rendering ↔ Camera Integration Test Cases

Companion test cases for [rendering-camera.md](rendering-camera.md).

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
| TC-13.25.1.I1 | Virtual camera priority selects active | Two virtual cameras with priorities 5 and 10 | Brain selects priority-10 camera for RenderView | R-13.25.1 |
| TC-13.25.2.I1 | Camera brain produces output per frame | Enabled brain on one active virtual camera | `CameraOutput` component updated every frame | R-13.25.2 |
| TC-13.25.36.I1 | Modifier stack applies in order | Two modifiers `[shake, noise]` on brain | Final output = `noise(shake(base))` | R-13.25.36 |
| TC-2.3.5.I1 | Orthographic projection reaches render | Camera with ortho size=10 | Render uses orthographic projection matrix through full pipeline | R-2.3.5 |
| TC-2.3.6.I1 | Perspective reverse-Z reaches render | Camera with 60 deg FOV, near 0.1, far 1000 | Reverse-Z projection matrix installed in ViewUniform | R-2.3.6 |
| TC-2.3.11.I1 | Dynamic resolution scaling feedback loop | GPU over-budget signal for 2 frames | DRS scale decreases, viewport resized next frame | R-2.3.11 |
| TC-2.10.4.I1 | View registered with projection | New camera registered to render world | View entry with projection matrix present in `RenderView` list | R-2.10.4 |
| TC-2.10.5.I1 | Multi-view renders from one snapshot | Four cameras enabled simultaneously | Single snapshot produces four `RenderView` entries in one extract phase | R-2.10.5 |
| TC-13.25.1.I2 | Game dev places virtual camera with priority | Designer adds high-priority camera during cutscene | Brain switches to cutscene camera; previous camera retained at lower priority | US-13.25.1 |
| TC-13.25.2.I2 | Game dev reads camera brain output | Designer queries brain output for HUD projection | Output matches current active virtual camera transform and FOV | US-13.25.2 |
| TC-13.25.36.I2 | Game dev stacks camera modifiers | Designer adds shake and zoom modifiers | Stack processes in declaration order, each modifier contributes to final output | US-13.25.36 |
| TC-2.3.5.I2 | Game dev enables orthographic mode | Designer toggles ortho on 2D game camera | Render pipeline uses orthographic projection, reverse-Z still correct | US-2.3.5 |
| TC-2.3.6.I2 | Game dev configures perspective camera | Designer sets FOV 75 deg and near/far 0.1/500 | Render pipeline uses reverse-Z perspective matrix | US-2.3.6 |
| TC-2.3.11.I2 | Engine dev observes DRS maintaining framerate | Developer monitors frame time over 60 frames with GPU stress | DRS adjusts scale to maintain 60 FPS within tolerance | US-2.3.11 |
| TC-2.10.4.I2 | Engine dev registers view with custom projection | Developer adds new view with cinematic projection | View registered cleanly, projection available to shaders | US-2.10.4 |
| TC-2.10.5.I2 | Engine dev uses multi-view for split screen | Developer enables 4-player split screen | All 4 views render from single extract phase, each with correct transform | US-2.10.5 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-3.1.1.B1 | RenderView build from CameraOutput | < 0.01 ms | IR-3.1.1 |
| TC-IR-3.1.4.B1 | 4-camera split-screen extraction | < 0.1 ms total | IR-3.1.4 |
| TC-IR-3.1.5.B1 | ViewUniform GPU upload (4 views) | < 0.05 ms | IR-3.1.5 |
