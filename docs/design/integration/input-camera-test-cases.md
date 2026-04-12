# Input ↔ Camera Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-4.1.1.1 | Mouse delta drives PanTilt yaw | Axis2D(10, 0) | Yaw += 10 * sensitivity.x | IR-4.1.1 |
| TC-IR-4.1.1.2 | Mouse delta drives PanTilt pitch | Axis2D(0, 5) | Pitch += 5 * sensitivity.y | IR-4.1.1 |
| TC-IR-4.1.1.3 | Invert Y negates pitch delta | invert_y=true, Axis2D(0,5) | Pitch -= 5 * sensitivity.y | IR-4.1.1 |
| TC-IR-4.1.2.1 | Gamepad stick drives orbit horizontal | Axis2D(0.8, 0) | Horizontal angle changes | IR-4.1.2 |
| TC-IR-4.1.2.2 | Gamepad stick drives orbit vertical | Axis2D(0, 0.6) | Vertical angle changes | IR-4.1.2 |
| TC-IR-4.1.3.1 | VR HMD pose overrides position | XrHeadPose(1,2,3) | CameraOutput.position=(1,2,3) | IR-4.1.3 |
| TC-IR-4.1.3.2 | VR HMD pose overrides rotation | XrHeadPose(quat) | CameraOutput.rotation=quat | IR-4.1.3 |
| TC-IR-4.1.3.3 | VR tracking lost holds last pose | Tracking lost | Output = last valid pose | IR-4.1.3 |
| TC-IR-4.1.4.1 | Bool action enables FreeLook | Pressed("FreeLook") | FreeLookModifier enabled | IR-4.1.4 |
| TC-IR-4.1.4.2 | Bool action disables FreeLook | Released("FreeLook") | FreeLookModifier disabled | IR-4.1.4 |
| TC-IR-4.1.5.1 | Aim assist deflects toward target | Look delta + target at 5 deg | Delta bent toward target | IR-4.1.5 |
| TC-IR-4.1.5.2 | Aim assist no targets passthrough | Look delta, no targets | Delta unchanged | IR-4.1.5 |
| TC-IR-4.1.6.1 | Dead zone filters small stick input | Axis2D(0.05, 0.05) | Output = (0, 0) | IR-4.1.6 |
| TC-IR-4.1.6.2 | Response curve shapes stick output | Axis2D(0.5, 0) | Output follows curve | IR-4.1.6 |
| TC-IR-4.1.7.1 | Sensitivity scales mouse delta | sensitivity=(2,2), delta=(10,5) | Effective = (20,10) | IR-4.1.7 |
| TC-IR-4.1.8.1 | Input suppressed during blend | BlendSystem active | CIAC outputs zero | IR-4.1.8 |
| TC-IR-4.1.8.2 | Input resumes after blend complete | Blend finishes | CIAC outputs normal delta | IR-4.1.8 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-4.1.1.B1 | CIAC eval for 4 split-screen cameras | < 0.05 ms | IR-4.1.1 |
| TC-IR-4.1.3.B1 | VR head tracking to camera output | < 1 ms | IR-4.1.3 |
| TC-IR-4.1.5.B1 | Aim assist with 50 potential targets | < 0.1 ms | IR-4.1.5 |
