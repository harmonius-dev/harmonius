# AI Behavior ↔ Animation Integration Test Cases

Companion test cases for [ai-animation.md](ai-animation.md).

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-US-9.4.10.I1 | AI + anim shared budget for 500 agents | 500 concurrent AI+anim entities | Combined evaluation under 2 ms/frame; shared FrameBudget records both costs | US-9.4.10 |
| TC-IR-1.1.1.1 | BT leaf writes speed | BT MoveTo at 3.0 | ParameterMap.speed == 3.0 | IR-1.1.1 |
| TC-IR-1.1.1.2 | GOAP action writes trigger | Attack action | "attack" in triggers | IR-1.1.1 |
| TC-IR-1.1.2.1 | AI reads anim remaining | Anim 80% done | state_remaining < 0.2 | IR-1.1.2 |
| TC-IR-1.1.2.2 | AI gates on transition | SM transitioning | is_transitioning == true | IR-1.1.2 |
| TC-IR-1.1.3.1 | Combat enter plays montage | AI -> Combat | ActiveMontage inserted | IR-1.1.3 |
| TC-IR-1.1.3.2 | Montage removed on finish | Montage ends | ActiveMontage removed | IR-1.1.3 |
| TC-IR-1.1.4.1 | Nav speed drives blend | Speed 2.5, dir 45 | Blend space walk/run | IR-1.1.4 |
| TC-IR-1.1.4.2 | Zero speed yields idle | Speed 0.0 | Idle state selected | IR-1.1.4 |
| TC-IR-1.1.5.1 | 500 agents under budget | 500 AI+anim entities | Total < 2 ms | IR-1.1.5 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-1.1.5.B1 | 500 AI+anim eval | < 2 ms | IR-1.1.5 |
| TC-IR-1.1.1.B1 | 1000 param writes | < 0.1 ms | IR-1.1.1 |
| TC-IR-1.1.2.B1 | 1000 anim queries | < 0.05 ms | IR-1.1.2 |
