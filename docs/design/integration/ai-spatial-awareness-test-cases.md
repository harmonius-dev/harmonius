# AI ↔ Spatial Awareness Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.10.1.1 | Sight in cone | Target in cone+LOS | Perceived added | IR-1.10.1 |
| TC-IR-1.10.1.2 | Sight behind | Target behind agent | Not perceived | IR-1.10.1 |
| TC-IR-1.10.1.3 | Sight wall block | Wall occludes LOS | Not perceived | IR-1.10.1 |
| TC-IR-1.10.1.4 | Sight max range | Target at limit | Low score perceived | IR-1.10.1 |
| TC-IR-1.10.2.1 | Hearing detects | Loud sound in range | Perceived (Hearing) | IR-1.10.2 |
| TC-IR-1.10.2.2 | Hearing blocked | Wall attenuates | Below threshold | IR-1.10.2 |
| TC-IR-1.10.2.3 | Quiet ignored | Sound below thresh | Not perceived | IR-1.10.2 |
| TC-IR-1.10.3.1 | Alert to BB | Awareness -> Alert | BB level == 2 | IR-1.10.3 |
| TC-IR-1.10.3.2 | Lost clears | Awareness -> Lost | BB target cleared | IR-1.10.3 |
| TC-IR-1.10.3.3 | Transition evt | Suspicious->Alert | AwarenessTransitionEvent | IR-1.10.3 |
| TC-IR-1.10.4.1 | Top score threat | 3 varied targets | Top score in BB | IR-1.10.4 |
| TC-IR-1.10.4.2 | Closer scores more | Same angle, closer | Higher score | IR-1.10.4 |
| TC-IR-1.10.4.3 | Occluded penalized | Partially occluded | Score reduced | IR-1.10.4 |
| TC-IR-1.10.5.1 | Budget limits | 500 agents, 2ms | Some deferred | IR-1.10.5 |
| TC-IR-1.10.5.2 | Deferred stale | Deferred 1 frame | Old data valid | IR-1.10.5 |
| TC-IR-1.10.6.1 | Target despawn | Despawn perceived | Memory decay removes | IR-1.10.1 |
| TC-IR-1.10.6.2 | Faction missing | Target no FactionId | Treated as neutral | IR-1.10.4 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-1.10.1.B1 | 100 sight queries x 1000 | < 1 ms | IR-1.10.1 |
| TC-IR-1.10.2.B1 | 100 hearing queries | < 0.3 ms | IR-1.10.2 |
| TC-IR-1.10.5.B1 | 500 agents full perception | < 2 ms | IR-1.10.5 |
