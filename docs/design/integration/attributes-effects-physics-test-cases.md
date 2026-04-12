# Attributes/Effects ↔ Physics Integration Test Cases

All test cases are CI-runnable under `cargo test` with no external services or GPU required.

## Unit Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.6.1.U1 | GravityOverride write/clear | 0.1 then 1.0 | Insert then remove | IR-2.6.1 |
| TC-IR-2.6.2.U1 | Mass inverse formula | base 0.5, scale 2.0 | inverse = 0.25 | IR-2.6.2 |
| TC-IR-2.6.2.U2 | Featherfall mass | base 0.5, scale 0.5 | inverse = 1.0 | IR-2.6.2 |
| TC-IR-2.6.3.U1 | FrictionOverride pair | sf 0.8 df 0.6 x 0.5 | 0.4 / 0.3 | IR-2.6.3 |
| TC-IR-2.6.3.U2 | Friction clamp upper | scale 10.0 | Both clamped 1.0 | IR-2.6.3 |
| TC-IR-2.6.5.U1 | SurfaceEffectMap lookup | SurfaceType::Ice | ice_walk RowRef | IR-2.6.5 |

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.6.1.1 | Levitate reduces gravity | 0.1x gravity_scale | Force = g * 0.1 | IR-2.6.1 |
| TC-IR-2.6.1.2 | Heavy curse doubles gravity | 2.0x gravity_scale | Force = g * 2.0 | IR-2.6.1 |
| TC-IR-2.6.1.3 | Stacked gravity effects | 0.5x + 0.5x mult | Force = g * 0.25 | IR-2.6.1 |
| TC-IR-2.6.2.1 | Featherfall reduces mass | 0.5x mass_scale | Mass halved | IR-2.6.2 |
| TC-IR-2.6.2.2 | Petrify increases mass | 5.0x mass_scale | Mass quintupled | IR-2.6.2 |
| TC-IR-2.6.3.1 | Ice walk reduces friction | 0.1x friction_scale | sf+df near 0 | IR-2.6.3 |
| TC-IR-2.6.3.2 | Root maximizes friction | 10.0x friction_scale | sf+df clamp 1.0 | IR-2.6.3 |
| TC-IR-2.6.3.3 | Static friction written | scale 0.5 | sf halved | IR-2.6.3 |
| TC-IR-2.6.4.1 | Strength scales knockback | Strength = 50 | Force = base * 1.5 | IR-2.6.4 |
| TC-IR-2.6.4.2 | Zero strength no force | Strength = 0 | Force = 0 | IR-2.6.4 |
| TC-IR-2.6.5.1 | Ice collision slips | Step on SurfaceType::Ice | ice_walk applied | IR-2.6.5 |
| TC-IR-2.6.5.2 | Water collision | Step on SurfaceType::Water | water fx applied | IR-2.6.5 |
| TC-IR-2.6.6.1 | Levitate expires restore | Duration ends | Gravity = normal | IR-2.6.6 |
| TC-IR-2.6.6.2 | Mass effect expires | Petrify wears off | Mass = original | IR-2.6.6 |
| TC-IR-2.6.6.3 | Friction expiry restore | ice_walk expires | FrictionOverride gone | IR-2.6.6 |
| TC-IR-2.6.6.4 | Gravity expiry restore | levitate expires | GravityOverride gone | IR-2.6.6 |

## Negative Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.6.1.N1 | Zero gravity_scale | scale = 0.0 | Clamp to 0.01 | IR-2.6.1 |
| TC-IR-2.6.2.N1 | Zero mass_scale | scale = 0.0 | Clamp to 0.001 | IR-2.6.2 |
| TC-IR-2.6.3.N1 | Negative friction_scale | scale = -1.0 | Clamp to 0.0 | IR-2.6.3 |
| TC-IR-2.6.3.N2 | Missing handle | No PhysicsMaterialHandle | Skip, warn once | IR-2.6.3 |
| TC-IR-2.6.3.N3 | Unloaded material asset | Handle, no asset | Skip, warn once | IR-2.6.3 |
| TC-IR-2.6.5.N1 | Unmapped surface | SurfaceType::Default | Skip silently | IR-2.6.5 |
| TC-IR-2.6.5.N2 | Target has no effects | Env object collision | Skip, warn once | IR-2.6.5 |
| TC-IR-2.6.5.N3 | Missing handle on hit | No handle on collider | Skip silently | IR-2.6.5 |
| TC-IR-2.6.6.N1 | Effect stack overflow | 17 effects applied | Evict lowest prio | IR-2.6.6 |
| TC-IR-2.6.6.N2 | Expiry with no override | Clean state expiry | No-op, no panic | IR-2.6.6 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-2.6.1.B1 | 1000 gravity syncs | < 0.1 ms | IR-2.6.1 |
| TC-IR-2.6.2.B1 | 1000 mass syncs | < 0.1 ms | IR-2.6.2 |
| TC-IR-2.6.3.B1 | 1000 friction syncs | < 0.15 ms | IR-2.6.3 |
| TC-IR-2.6.4.B1 | 1000 force scalings | < 0.1 ms | IR-2.6.4 |
| TC-IR-2.6.5.B1 | 500 collision effects | < 0.3 ms | IR-2.6.5 |
| TC-IR-2.6.6.B1 | 200 effect expiry restores | < 0.1 ms | IR-2.6.6 |
| TC-IR-2.6.0.B1 | Idle (no changes) 1000 ent | 0 us iter | IR-2.6.* |
