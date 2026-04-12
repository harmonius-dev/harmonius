# Scripting ↔ ECS Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.8.1.1 | Graph reads own component | Entity with Health(100) | ctx.read::<Health>() = 100 | IR-2.8.1 |
| TC-IR-2.8.1.2 | Graph reads other entity | Target with Armor(50) | read_entity = 50 | IR-2.8.1 |
| TC-IR-2.8.1.3 | Graph reads missing comp | Entity without Armor | Returns None | IR-2.8.1 |
| TC-IR-2.8.2.1 | Graph writes component | write(e, Damage(10)) | Damage applied after flush | IR-2.8.2 |
| TC-IR-2.8.2.2 | Multiple writes same comp | Two writes to Health | Last write wins | IR-2.8.2 |
| TC-IR-2.8.3.1 | Graph spawns entity | ctx.spawn().insert(T) | New entity after flush | IR-2.8.3 |
| TC-IR-2.8.3.2 | Graph despawns entity | commands.despawn(e) | Entity removed after flush | IR-2.8.3 |
| TC-IR-2.8.4.1 | Parallel graph execution | 100 disjoint graphs | All execute in parallel | IR-2.8.4 |
| TC-IR-2.8.4.2 | Conflicting graphs serialize | 2 graphs write same T | Run sequentially | IR-2.8.4 |
| TC-IR-2.8.5.1 | Access sets computed | Graph reads A, writes B | reads={A}, writes={B} | IR-2.8.5 |
| TC-IR-2.8.5.2 | Access sets on reload | New graph version | Access sets updated | IR-2.8.5 |
| TC-IR-2.8.6.1 | Entity var reads component | Entity var "health" | Reads Health component | IR-2.8.6 |
| TC-IR-2.8.6.2 | Entity var writes component | Set entity var "speed" | Speed component updated | IR-2.8.6 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-2.8.1.B1 | 10000 component reads | < 0.1 ms | IR-2.8.1 |
| TC-IR-2.8.2.B1 | 5000 deferred writes | < 0.5 ms | IR-2.8.2 |
| TC-IR-2.8.3.B1 | 1000 entity spawns | < 1 ms | IR-2.8.3 |
| TC-IR-2.8.4.B1 | 1000 active graphs | < 4 ms | IR-2.8.4 |
