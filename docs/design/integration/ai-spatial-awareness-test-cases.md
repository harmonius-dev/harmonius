# AI ↔ Spatial Awareness Integration Test Cases

All tests are CI-runnable via `cargo test -p harmonius-integration-ai-sa`. Benchmarks run via
`cargo bench -p harmonius-integration-ai-sa` and are gated on the perf CI job.

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
| TC-IR-1.10.5.3 | Budget split | Perc + Dec schedule | No Res conflict | IR-1.10.5 |

## Negative Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.10.N.1 | Target despawn | Despawn perceived | See detail 1 | IR-1.10.1 |
| TC-IR-1.10.N.2 | Faction missing | Target no FactionId | See detail 2 | IR-1.10.4 |
| TC-IR-1.10.N.3 | Propagation missing | No PropagationResult | See detail 3 | IR-1.10.2 |
| TC-IR-1.10.N.4 | Sense def missing | SenseDefinition unloaded | See detail 4 | IR-1.10.1 |
| TC-IR-1.10.N.5 | Empty BVH | Zero candidates | See detail 5 | IR-1.10.1 |
| TC-IR-1.10.N.6 | Awareness unchanged | No transition | See detail 6 | IR-1.10.3 |

1. **TC-IR-1.10.N.1** -- Perceived target despawns mid-frame. Memory decay removes the stale
   `PerceivedEntity` once `last_seen_time` exceeds `memory_duration`. The next
   `awareness_blackboard_sync` observes `highest_scored_target() == None` and clears
   `THREAT_TARGET_KEY` and `THREAT_POSITION_KEY`.
2. **TC-IR-1.10.N.2** -- Target has no `FactionId` component. Perception treats it as neutral (no
   friend/foe scoring bonus). Target is still perceived and ranked by distance/angle/occlusion.
3. **TC-IR-1.10.N.3** -- `PropagationResultStore` has no entry for a live sound source. Hearing
   check is skipped for that source this frame. No `PerceivedEntity` with `sense: Hearing` is added
   until the next frame when propagation data arrives.
4. **TC-IR-1.10.N.4** -- `SenseDefinition` asset handle fails to resolve. A warning logs once per
   missing handle. The affected sense is skipped; the agent perceives via remaining senses only.
5. **TC-IR-1.10.N.5** -- BVH query returns zero candidates. `AiPerception.known_entities` is left
   intact (subject to memory decay). No new `SenseResult` entries are produced.
6. **TC-IR-1.10.N.6** -- `AwarenessState` unchanged across frames. `Changed<AwarenessState>` filter
   excludes the agent. No blackboard write occurs. BT/GOAP read prior values unchanged.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-1.10.1.B1 | 100 sight queries x 1000 | < 1 ms | IR-1.10.1 |
| TC-IR-1.10.2.B1 | 100 hearing queries | < 0.3 ms | IR-1.10.2 |
| TC-IR-1.10.5.B1 | 500 agents full perception | < 2 ms | IR-1.10.5 |
| TC-IR-1.10.5.B2 | 500 agents decision step | < 0.8 ms | IR-1.10.5 |
