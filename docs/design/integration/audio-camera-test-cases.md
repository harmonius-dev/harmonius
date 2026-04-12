# Audio ↔ Camera Integration Test Cases

All test cases are CI-runnable under `cargo test` with no audio device, GPU, or external services
required. The bridge system is a pure ECS system exercised with real (non-mock)
`ListenerPrevPositions`, `GameTime`, and `CommandSender` fakes that implement the same MPSC trait as
production.

## Unit Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.7.1.U1 | Prev pos slot roundtrip | `set(1, p)` then `get(1)` | `Some(p)` | IR-1.7.1 |
| TC-IR-1.7.1.U2 | Prev pos empty slot | Fresh `new()`, `get(2)` | `None` | IR-1.7.1 |
| TC-IR-1.7.1.U3 | Prev pos clear | `set`, `clear`, `get` | `None` | IR-1.7.1 |
| TC-IR-1.7.2.U1 | Forward axis const | `LISTENER_FORWARD` | `Vec3::NEG_Z` | IR-1.7.2 |
| TC-IR-1.7.3.U1 | Velocity clamp cap | Raw 500 m/s vector | Length == 100.0 | IR-1.7.3 |
| TC-IR-1.7.3.U2 | Velocity below cap | Raw 50 m/s vector | Length == 50.0 | IR-1.7.3 |
| TC-IR-1.7.3.U3 | Backward difference | p0=0, p1=1, dt=0.5 | Vel == (2,0,0) | IR-1.7.3 |
| TC-IR-1.7.4.U1 | Slot idx out of range | `set(10, p)` | No panic, no write | IR-1.7.4 |

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.7.1.1 | Listener gets cam pos | Camera at (5,0,0) | Cmd pos == (5,0,0) | IR-1.7.1 |
| TC-IR-1.7.1.2 | Listener tracks movement | Camera 1m/frame | Cmd pos follows frames | IR-1.7.1 |
| TC-IR-1.7.1.3 | No active brain | All brains despawned | Last known pos retained | IR-1.7.1 |
| TC-IR-1.7.2.1 | Identity rot forward | Identity rotation | `rot * NEG_Z == (0,0,-1)` | IR-1.7.2 |
| TC-IR-1.7.2.2 | HRTF uses orientation | Source left of cam | Left ear louder | IR-1.7.2 |
| TC-IR-1.7.3.1 | Velocity from delta | 1m in 0.016s | Vel length ~62.5 m/s | IR-1.7.3 |
| TC-IR-1.7.3.2 | Zero vel when static | Camera stationary | Vel == (0,0,0) | IR-1.7.3 |
| TC-IR-1.7.3.3 | Teleport clamps vel | Jump 1000m in 1 frame | Vel length == 100.0 | IR-1.7.3 |
| TC-IR-1.7.3.4 | Zero dt skips vel | delta_seconds == 0.0 | Vel == (0,0,0) | IR-1.7.3 |
| TC-IR-1.7.4.1 | Two listeners split | 2 CameraBrains P0, P1 | 2 UpdateListener cmds | IR-1.7.4 |
| TC-IR-1.7.4.2 | Player indices unique | P1 idx=0, P2 idx=1 | Separate ListenerIds | IR-1.7.4 |
| TC-IR-1.7.5.1 | Cutscene listener | Cine camera active | Listener at cine pos | IR-1.7.5 |
| TC-IR-1.7.5.2 | Exit snaps back | Cutscene ends | Listener at GP pos | IR-1.7.5 |

## Negative Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.7.1.N1 | No CameraBrain entity | Empty query | No cmd, no panic | IR-1.7.1 |
| TC-IR-1.7.1.N2 | Despawn mid-frame | Brain removed between frames | Slot cleared | IR-1.7.1 |
| TC-IR-1.7.3.N1 | Zero delta panic guard | dt == 0.0 | Vec3::ZERO, no NaN | IR-1.7.3 |
| TC-IR-1.7.3.N2 | Negative dt | dt == -0.001 | Vec3::ZERO (guard) | IR-1.7.3 |
| TC-IR-1.7.3.N3 | NaN position input | Position == NaN | Cmd dropped, warn log | IR-1.7.3 |
| TC-IR-1.7.4.N1 | Idx > MAX_LOCAL_PLAYERS | listener.idx = 9 | `set` no-op, no crash | IR-1.7.4 |
| TC-IR-1.7.4.N2 | MPSC queue full | Send until Err | Warn log, update dropped | IR-1.7.4 |
| TC-IR-1.7.5.N1 | Cine + gameplay both | Two active brains | First in query wins | IR-1.7.5 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-1.7.1.B1 | 4-player listener sync | < 0.01 ms | IR-1.7.1 |
| TC-IR-1.7.1.B2 | MPSC send (4 cmds) | < 0.002 ms | IR-1.7.1 |
| TC-IR-1.7.3.B1 | Velocity derivation x4 | < 0.005 ms | IR-1.7.3 |
