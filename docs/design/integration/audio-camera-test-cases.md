# Audio ↔ Camera Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.7.1.1 | Listener gets cam pos | Camera at (5,0,0) | Listener pos == (5,0,0) | IR-1.7.1 |
| TC-IR-1.7.1.2 | Listener tracks movement | Camera moves 1m/frame | Listener follows | IR-1.7.1 |
| TC-IR-1.7.1.3 | No active brain | All brains despawned | Last known pos retained | IR-1.7.1 |
| TC-IR-1.7.2.1 | Listener gets cam rot | Identity rotation | Forward == (0,0,-1) | IR-1.7.2 |
| TC-IR-1.7.2.2 | HRTF uses orientation | Source left of cam | Left ear louder | IR-1.7.2 |
| TC-IR-1.7.3.1 | Velocity from delta | Move 10m in 0.016s | Velocity ~625 m/s | IR-1.7.3 |
| TC-IR-1.7.3.2 | Zero vel when static | Camera stationary | Velocity == (0,0,0) | IR-1.7.3 |
| TC-IR-1.7.3.3 | Teleport clamps vel | Jump 1000m in 1 frame | Velocity clamped | IR-1.7.3 |
| TC-IR-1.7.3.4 | Zero dt skips vel | delta_seconds == 0.0 | Velocity == (0,0,0) | IR-1.7.3 |
| TC-IR-1.7.4.1 | Two listeners split | 2 CameraBrains | 2 UpdateListener cmds | IR-1.7.4 |
| TC-IR-1.7.4.2 | Player indices unique | P1 idx=0, P2 idx=1 | Separate mixes | IR-1.7.4 |
| TC-IR-1.7.5.1 | Cutscene listener | Cine camera active | Listener at cine pos | IR-1.7.5 |
| TC-IR-1.7.5.2 | Exit snaps back | Cutscene ends | Listener at GP pos | IR-1.7.5 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-1.7.1.B1 | 4-player listener sync | < 0.01 ms | IR-1.7.1 |
| TC-IR-1.7.3.B1 | Velocity derivation 4 | < 0.005 ms | IR-1.7.3 |
