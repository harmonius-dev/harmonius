# Attributes/Effects ↔ Physics Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.6.1.1 | Levitate reduces gravity | 0.1x gravity_scale | Force = g * 0.1 | IR-2.6.1 |
| TC-IR-2.6.1.2 | Heavy curse doubles gravity | 2.0x gravity_scale | Force = g * 2.0 | IR-2.6.1 |
| TC-IR-2.6.1.3 | Stacked gravity effects | 0.5x + 0.5x mult | Force = g * 0.25 | IR-2.6.1 |
| TC-IR-2.6.2.1 | Featherfall reduces mass | 0.5x mass_scale | Mass halved | IR-2.6.2 |
| TC-IR-2.6.2.2 | Petrify increases mass | 5.0x mass_scale | Mass quintupled | IR-2.6.2 |
| TC-IR-2.6.3.1 | Ice walk reduces friction | 0.1x friction_scale | Near-zero friction | IR-2.6.3 |
| TC-IR-2.6.3.2 | Root maximizes friction | 10.0x friction_scale | Friction clamped 1.0 | IR-2.6.3 |
| TC-IR-2.6.4.1 | Strength scales knockback | Strength = 50 | Force = base * 1.5 | IR-2.6.4 |
| TC-IR-2.6.4.2 | Zero strength no force | Strength = 0 | Force = 0 | IR-2.6.4 |
| TC-IR-2.6.5.1 | Lava collision burns | Step on SurfaceType::Lava | Burn effect applied | IR-2.6.5 |
| TC-IR-2.6.5.2 | Ice collision slips | Step on SurfaceType::Ice | Ice-walk effect applied | IR-2.6.5 |
| TC-IR-2.6.6.1 | Levitate expires restore | Effect duration ends | Gravity = normal | IR-2.6.6 |
| TC-IR-2.6.6.2 | Mass effect expires | Petrify wears off | Mass = original | IR-2.6.6 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-2.6.1.B1 | 1000 gravity syncs | < 0.1 ms | IR-2.6.1 |
| TC-IR-2.6.2.B1 | 1000 mass syncs | < 0.1 ms | IR-2.6.2 |
| TC-IR-2.6.5.B1 | 500 collision effects | < 0.3 ms | IR-2.6.5 |
| TC-IR-2.6.6.B1 | 200 effect expiry restores | < 0.1 ms | IR-2.6.6 |
