# Input ↔ Camera Integration Test Cases

## Scope

3D and VR camera input only. **2D / 2.5D is intentionally out of scope** — ortho cameras bypass
`CameraInputAxisController` and are covered by `camera.md` tests.

All tests are CI-runnable under `cargo test`. They use a real `ActionMapping`, a real
`crossbeam_channel::bounded` MPSC, and a fake `HmdTracker` that feeds deterministic VR poses. No
mocking libraries are used. Per-test setup builds an ECS world with `Rng` seed `0x1234`.

## Integration Tests

### Positive Cases

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-4.1.1.1 | Mouse yaw | Axis2D(10,0) | Yaw += 10*sens.x | IR-4.1.1 |
| TC-IR-4.1.1.2 | Mouse pitch | Axis2D(0,5) | Pitch += 5*sens.y | IR-4.1.1 |
| TC-IR-4.1.1.3 | Invert Y | inv, Axis2D(0,5) | Pitch -= 5*sens.y | IR-4.1.1 |
| TC-IR-4.1.2.1 | Stick orbit H | Axis2D(0.8,0) | H angle changes | IR-4.1.2 |
| TC-IR-4.1.2.2 | Stick orbit V | Axis2D(0,0.6) | V angle changes | IR-4.1.2 |
| TC-IR-4.1.3.1 | VR pos override | XrHeadPose(1,2,3) | pos=(1,2,3) | IR-4.1.3 |
| TC-IR-4.1.3.2 | VR rot override | XrHeadPose(quat) | rot=quat | IR-4.1.3 |
| TC-IR-4.1.3.3 | VR track lost | Tracking lost | Last pose held | IR-4.1.3 |
| TC-IR-4.1.4.1 | FreeLook insert | Pressed action | Marker inserted | IR-4.1.4 |
| TC-IR-4.1.4.2 | FreeLook remove | Released action | Marker removed | IR-4.1.4 |
| TC-IR-4.1.5.1 | Aim deflect | Delta + target | Delta bent | IR-4.1.5 |
| TC-IR-4.1.5.2 | Aim passthrough | No targets | Delta unchanged | IR-4.1.5 |
| TC-IR-4.1.6.1 | Dead zone | Axis2D(0.05,0.05) | Output = (0,0) | IR-4.1.6 |
| TC-IR-4.1.6.2 | Curve shape | Axis2D(0.5,0) | Curve applied | IR-4.1.6 |
| TC-IR-4.1.7.1 | Sens scale | sens=2, d=(10,5) | Out=(20,10) | IR-4.1.7 |
| TC-IR-4.1.8.1 | Blend suppress | Blending | CIAC=zero | IR-4.1.8 |
| TC-IR-4.1.8.2 | Blend resume | Blend done | CIAC=normal | IR-4.1.8 |

1. **TC-IR-4.1.1.3** — Configure CIAC with `invert_y = true`, `sensitivity = (1.0, 1.0)`. Feed
   `ActionValue::Axis2D(Vec2::new(0.0, 5.0))`. Expect `PanTilt.pitch` to decrease by
   `5.0 * sensitivity.y` rather than increase.
2. **TC-IR-4.1.3.3** — Write a valid `XrHeadPose`, then simulate tracking loss by withholding
   updates for 10 frames. Expect `CameraOutput` to match the last valid pose exactly; no
   interpolation or prediction.
3. **TC-IR-4.1.4.1 / TC-IR-4.1.4.2** — Verify marker-component insertion and removal on the active
   virtual camera entity. `FreeLookModifier` is a zero-field marker struct.
4. **TC-IR-4.1.8.1** — Start blend, tick several frames; assert `CIAC` writes zero delta to
   `PanTilt` while `BlendSystem.is_blending() == true`.

### Negative / Error-Path Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-4.1.E1 | No mapping | Missing ActionState | Zero delta, warn | IR-4.1.1 |
| TC-IR-4.1.E2 | Type mismatch | Bool bound to CIAC | Zero delta, warn | IR-4.1.1 |
| TC-IR-4.1.E3 | Channel full | 512 msgs > cap 256 | Drop counter ++ | IR-4.1.1 |
| TC-IR-4.1.E4 | Suppress stuck | Blend > 2.0 s | Auto-clears | IR-4.1.8 |
| TC-IR-4.1.E5 | VR pose missing | No XrHeadPose | VrBrain skipped | IR-4.1.3 |
| TC-IR-4.1.E6 | Aim disabled | enabled=false | Raw delta through | IR-4.1.5 |

1. **TC-IR-4.1.E1** — CIAC references an `ActionId` that has no `ActionState` row. Expect a zero
   delta written to `PanTilt` and a single `tracing::warn!` containing the `ActionId`. Subsequent
   frames must not re-log.
2. **TC-IR-4.1.E2** — Bind a `Bool` action to `CameraInputAxisController.look_action`. CIAC's
   `ActionValueType::Axis2D` filter rejects the value; expect zero delta and a single warn. No
   panic.
3. **TC-IR-4.1.E3** — Push `512` `raw_camera_input` messages against a `256`-capacity
   `crossbeam_channel::bounded`. Expect `256` drops reflected in `InputCameraDebug.dropped`.
4. **TC-IR-4.1.E4** — Force `suppress_during_blend = true` and hold for `2.1` simulated seconds.
   Expect `suppression_elapsed` to exceed `suppression_timeout`, CIAC to clear the flag, and a warn
   log to fire.
5. **TC-IR-4.1.E5** — VR brain evaluation runs without an `XrHeadPose` component. Expect
   `VrCameraBrain` to be skipped and the standard `CameraBrain` to take over.
6. **TC-IR-4.1.E6** — Populate `AimAssistConfig` with `enabled = false` and a valid target in range.
   Expect the raw look delta to reach `PanTilt` unmodified.

### Combined Scenario Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-4.1.C1 | Sens+inv+dead zone | sens2,inv,(0.03,0.5) | Yaw=0, pitch neg | IR-4.1.1 |
| TC-IR-4.1.C2 | Sens+inv+curve | sens1.5,inv,(0.5,0.5) | Curve applied | IR-4.1.6 |
| TC-IR-4.1.C3 | Aim + blend | Target + blending | Zero (suppress) | IR-4.1.8 |
| TC-IR-4.1.C4 | Sens + aim | sens2, target 5 deg | Bent + scaled | IR-4.1.5 |
| TC-IR-4.1.C5 | VR + FreeLook | VR + marker | VR wins, noop | IR-4.1.3 |

1. **TC-IR-4.1.C1** — Assert pipeline order: dead-zone filter runs first (yaw raw `0.03` below
   threshold becomes `0.0`), invert_y negates pitch, then sensitivity scales the result. Final yaw =
   `0`, final pitch = `-0.5 * 2.0 = -1.0`.
2. **TC-IR-4.1.C2** — Confirm the response curve shapes the input before sensitivity is applied; the
   end-to-end transform must be deterministic and reproducible.
3. **TC-IR-4.1.C3** — Aim assist must not bypass blend suppression. Expect zero delta even with a
   valid target in range.
4. **TC-IR-4.1.C4** — Deflection vector composes with the sensitivity multiplier. Assert both
   magnitude (scaled) and direction (bent toward target).
5. **TC-IR-4.1.C5** — VR head tracking always wins over `FreeLookModifier`. The marker is a no-op
   while `VrCameraBrain` is active.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-4.1.1.B1 | CIAC eval, 4 split-screen cams | < 0.05 ms | IR-4.1.1 |
| TC-IR-4.1.3.B1 | VR head tracking to output | < 1.00 ms | IR-4.1.3 |
| TC-IR-4.1.5.B1 | Aim assist, 50 targets | < 0.10 ms | IR-4.1.5 |
| TC-IR-4.1.E3.B1 | `raw_camera_input` p99 send | < 5 us | IR-4.1.1 |
| TC-IR-4.1.C4.B1 | Combined pipeline per cam | < 0.05 ms | IR-4.1.5 |

All unit, integration, and negative test cases above are runnable under `cargo test` in CI and must
pass on every push. Benchmarks run under `cargo bench` on the nightly performance job.
