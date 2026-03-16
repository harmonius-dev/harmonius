# Gameplay Primitives Test Cases

Companion test cases for [primitives.md](primitives.md).

## Unit Tests

### TC-13.1.2.1 Valid Game State Transition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `request_transition(MainMenu, Loading)` | `Ok(Loading)` | R-13.1.2 |
| 2 | `request_transition(Loading, InGame)` | `Ok(InGame)` | R-13.1.2 |
| 3 | `request_transition(InGame, Paused)` | `Ok(Paused)` | R-13.1.2 |

### TC-13.1.2.2 Invalid Game State Transition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `request_transition(Loading, Paused)` | `Err(InvalidTransition)` | R-13.1.2 |
| 2 | `request_transition(MainMenu, InGame)` | `Err(InvalidTransition)` | R-13.1.2 |

### TC-13.1.2.3 Game State No Resource Leak

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cycle all valid transitions 100x | Zero resource growth measured by entity count delta = 0 | US-13.1.1.7 |

### TC-13.1.1.1 Mode Graph Validate Reachable

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph with orphan mode node | `Err(UnreachableMode { mode })` | US-13.1.1.8 |

### TC-13.1.1.2 Mode Graph Cycle Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph with A->B->C->A cycle | `Err(CycleDetected { path: [A,B,C] })` | R-13.1.1 |

### TC-13.1.1.3 Mode Sub-Mode Enter Exit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enter sub-mode `Combat` under `Exploration` | `active_mode() == Combat`, `parent_mode() == Exploration` | R-13.1.1 |
| 2 | Exit sub-mode `Combat` | `active_mode() == Exploration` | R-13.1.1 |

### TC-13.1.7.1 Health Apply Damage

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Health 100.0, damage 30.0 | `current == 70.0`, `overkill == 0.0` | R-13.1.7 |

### TC-13.1.7.2 Health Clamp Zero

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Health 10.0, damage 50.0 | `current == 0.0`, `overkill == 40.0` | R-13.1.7 |

### TC-13.1.7.3 Health Invulnerable

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Health 100.0, `invulnerable = true`, damage 50.0 | `current == 100.0`, `final_amount == 0.0` | R-13.1.7 |

### TC-13.1.7.4 Damage Armor Mitigation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Raw 100.0, armor 50% physical | `final_amount == 50.0` | R-13.1.7 |

### TC-13.1.7.5 Damage Resistance Per School

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fire damage 100.0, fire resist 0.3, physical resist 0.5 | Fire `final == 70.0`, physical resist not applied | R-13.1.7 |

### TC-13.1.7.6 Damage Absorb Shield

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Shield 20.0, damage 50.0 | Shield 0.0, health reduced by 30.0 | R-13.1.7 |

### TC-13.1.7.7 Damage Determinism

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000 identical `DamageRequest` events | 1000 identical `DamageEvent` outputs | US-13.1.7.5 |

### TC-13.1.0.1 Tag Set Insert Remove

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert tags [A, B, C], remove B | `contains(A) == true`, `contains(B) == false`, `contains(C) == true` | R-13.1.1 |

### TC-13.1.0.2 Tag Set Contains All

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set {A, B, C}, query {A, B} | `contains_all == true` | R-13.1.1 |
| 2 | Set {A, C}, query {A, B} | `contains_all == false` | R-13.1.1 |

### TC-13.1.0.3 Tag Set Contains Any

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set {A, C}, query {B, C} | `contains_any == true` | R-13.1.1 |
| 2 | Set {A}, query {B, C} | `contains_any == false` | R-13.1.1 |

### TC-13.1.NF2.1 Timer Tick Expiry

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Timer 1.5s at 60 fps, tick until expired | Expires within 1 ms of 1.5s target | R-13.1.NF2 |

### TC-13.1.NF2.2 Timer Repeating

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Repeating timer 1.0s, tick past 1.0s | `is_expired() == true`, then auto-resets `remaining ~= 1.0` | R-13.1.NF2 |

### TC-13.1.5.1 Cooldown Not Ready

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger 5.0s cooldown, immediately query | `is_ready() == false` | US-13.1.5.6 |

### TC-13.1.5.2 Cooldown Expired Event

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger 1.0s cooldown, tick 1.0s | Exactly 1 `CooldownExpiredEvent` emitted | R-13.1.5 |

### TC-13.1.4.1 Allegiance Symmetric

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `set_disposition(A, B, Hostile)` | `disposition(B, A) == Hostile` | R-13.1.4 |

### TC-13.1.4.2 Allegiance Default Neutral

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Query unset faction pair (C, D) | `disposition(C, D) == Neutral` | R-13.1.4 |

### TC-13.1.4.3 Pawn Possession Transfer

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Possess pawn A, then possess pawn B | A released with state intact, B possessed | US-13.1.4.4 |

### TC-13.1.3.1 Controller Context Clears Queue

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Queue 2 inputs, switch context | `input_queue.len() == 0` | US-13.1.3.6 |

### TC-13.1.4.4 Spawn From Template

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `SpawnRequest` with 5-component template | Entity spawned with all 5 components present | R-13.1.4 |

### TC-13.1.4.5 Spawn With Children

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Template with 2 child templates | Parent + 2 children with `ChildOf` relationships | R-13.1.4 |

### TC-13.1.9.1 Module Enable Transitive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enable Combat (depends on Physics, Animation) | Physics and Animation auto-enabled, returns `[Physics, Animation]` | US-13.1.9.5 |

### TC-13.1.9.2 Module Disable With Dependents

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Disable Physics while Combat enabled | `Err(DependentModulesActive { dependents: [Combat] })` | R-13.1.9 |

### TC-13.1.9.3 Module Cycle Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register A->B->C->A dependency cycle | `Err(CycleDetected { path: [A, B, C] })` | R-13.1.9 |

## Integration Tests

### TC-13.1.2.I1 Full State Lifecycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Transition through MainMenu->Loading->InGame->Paused->InGame->Loading->MainMenu | Resources load/unload at each step; no leaks | R-13.1.2 |

### TC-13.1.2.I2 Rapid Pause Unpause

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Toggle Paused/InGame at 60 Hz for 10 seconds | No state corruption; entity count stable | US-13.1.2.5 |

### TC-13.1.NF1.I1 Mode Transition Under One Frame

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger mode transition, measure latency | Latency < 16.67 ms | R-13.1.NF1 |

### TC-13.1.7.I1 Damage 1K Events Per Frame

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000 `DamageRequest` events in one frame | All health values correct; total < 0.5 ms | R-13.1.7 |

### TC-13.1.8.I1 Death Respawn Full Cycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Kill entity, wait for respawn timer | Entity respawns at nearest valid point with full health | R-13.1.8 |

### TC-13.1.8.I2 Encounter Reset

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger wipe event on encounter | Boss HP restored, adds despawned, mode reverts | US-13.1.8.4 |

### TC-13.1.8.I3 Death Debuff Stacking

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Die 3 times within reset window | `DeathCounter.count == 3`; debuff stacks applied | US-13.1.8.6 |

### TC-13.1.4.I1 Spawn 100 Entities

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100 `SpawnRequest` events in one frame | 100 entities created; total time < 1 ms | R-13.1.4 |

### TC-13.1.0.I1 Tag Query 10K Entities

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10,000 entities with 8 tags, filter by 1 tag | Correct subset returned; query time < 0.1 ms | R-13.1.1 |

### TC-13.1.NF2.I1 Cooldown Precision 144fps

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1.5s cooldown at 144 fps, 1000 trials | Max drift <= 1 ms across all trials | R-13.1.NF2 |

### TC-13.1.2.I3 Server Authoritative State

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Client-server transition sequence | Both agree on state after all transitions | US-13.1.2.2 |

### TC-13.1.3.I1 Controller All Contexts

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Exercise all `InputContext` transitions | No input leaks between contexts | US-13.1.3.6 |

### TC-13.1.6.I1 Effect Cleanup On Death

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Kill entity with 5 active effects | All effect references cleaned up; no dangling entities | US-13.1.6.7 |

## Benchmarks

### TC-13.1.NF1.B1 Game State Transition Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Mode transition with asset load | Wall-clock latency | < 16.67 ms | R-13.1.NF1 |

### TC-13.1.NF2.B1 Cooldown Precision

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1.5s cooldown, 1000 trials at 144 fps | Maximum drift from target | <= 1 ms | R-13.1.NF2 |

### TC-13.1.7.B1 Damage Pipeline Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1000 damage events per frame | Total processing time | < 0.5 ms | R-13.1.7 |

### TC-13.1.0.B1 Tag Query Performance

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10,000 entities, 8 tags, filter by 1 | Query time | < 0.1 ms | R-13.1.1 |

### TC-13.1.4.B1 Spawn Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Spawn 100 entities from template | Total spawn time | < 1 ms | R-13.1.4 |

### TC-13.1.NF2.B2 Timer Tick Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100,000 active `GameTimer` components | Total tick time | < 0.5 ms | R-13.1.NF2 |

### TC-13.1.4.B2 Allegiance Lookup

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10,000 faction pair lookups | Total lookup time | < 0.05 ms | R-13.1.4 |
