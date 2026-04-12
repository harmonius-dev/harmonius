# Animation ↔ VFX Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.6.1.1 | VfxSpawn fires | VfxSpawn at t=1 | Emitter spawned | IR-1.6.1 |
| TC-IR-1.6.1.2 | Spawn at bone pos | Event on hand bone | Emitter at hand pos | IR-1.6.1 |
| TC-IR-1.6.1.3 | No dup same frame | 1 event, 1 frame | Exactly 1 emitter | IR-1.6.1 |
| TC-IR-1.6.2.1 | Trail follows bone | Swing arm anim | Ribbon on arc | IR-1.6.2 |
| TC-IR-1.6.2.2 | Trail stops idle | Arm stops moving | No new ribbon pts | IR-1.6.2 |
| TC-IR-1.6.3.1 | Weapon trail on | active=true | Spawn rate > 0 | IR-1.6.3 |
| TC-IR-1.6.3.2 | Weapon trail off | active=false | Spawn rate == 0 | IR-1.6.3 |
| TC-IR-1.6.3.3 | Trail matches hit | HitWindow timing | Active in window only | IR-1.6.3 |
| TC-IR-1.6.4.1 | LOD Full keeps VFX | Distance < 10 m | Full emitter eval | IR-1.6.4 |
| TC-IR-1.6.4.2 | LOD VAT culls VFX | Distance > 100 m | Emitter culled | IR-1.6.4 |
| TC-IR-1.6.5.1 | Bone attach syncs | Parent anim plays | Child follows bone | IR-1.6.5 |
| TC-IR-1.6.5.2 | Attach on despawn | Parent removed | Child cleaned up | IR-1.6.5 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-1.6.1.B1 | 100 VFX spawns/frame | < 0.3 ms | IR-1.6.1 |
| TC-IR-1.6.2.B1 | 200 trail emitters sync | < 0.2 ms | IR-1.6.2 |
| TC-IR-1.6.5.B1 | 500 bone attachments | < 0.3 ms | IR-1.6.5 |
