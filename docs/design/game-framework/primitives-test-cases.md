# Gameplay Primitives Test Cases

Companion test cases for [primitives.md](primitives.md).

## Unit Tests

### TC-13.1.2.1 Valid Game State Transition

| # | Requirement |
|---|-------------|
| 1 | R-13.1.2    |
| 2 | R-13.1.2    |
| 3 | R-13.1.2    |

1. **#1** — `request_transition(MainMenu, Loading)`
   - **Expected:** `Ok(Loading)`
2. **#2** — `request_transition(Loading, InGame)`
   - **Expected:** `Ok(InGame)`
3. **#3** — `request_transition(InGame, Paused)`
   - **Expected:** `Ok(Paused)`

### TC-13.1.2.2 Invalid Game State Transition

| # | Requirement |
|---|-------------|
| 1 | R-13.1.2    |
| 2 | R-13.1.2    |

1. **#1** — `request_transition(Loading, Paused)`
   - **Expected:** `Err(InvalidTransition)`
2. **#2** — `request_transition(MainMenu, InGame)`
   - **Expected:** `Err(InvalidTransition)`

### TC-13.1.2.3 Game State No Resource Leak

| # | Requirement |
|---|-------------|
| 1 | US-13.1.1.7 |

1. **#1** — Cycle all valid transitions 100x
   - **Expected:** Zero resource growth measured by entity count delta = 0

### TC-13.1.1.1 Mode Graph Validate Reachable

| # | Requirement |
|---|-------------|
| 1 | US-13.1.1.8 |

1. **#1** — Graph with orphan mode node
   - **Expected:** `Err(UnreachableMode { mode })`

### TC-13.1.1.2 Mode Graph Cycle Detection

| # | Requirement |
|---|-------------|
| 1 | R-13.1.1    |

1. **#1** — Graph with A->B->C->A cycle
   - **Expected:** `Err(CycleDetected { path: [A,B,C] })`

### TC-13.1.1.3 Mode Sub-Mode Enter Exit

| # | Requirement |
|---|-------------|
| 1 | R-13.1.1    |
| 2 | R-13.1.1    |

1. **#1** — Enter sub-mode `Combat` under `Exploration`
   - **Expected:** `active_mode() == Combat`, `parent_mode() == Exploration`
2. **#2** — Exit sub-mode `Combat`
   - **Expected:** `active_mode() == Exploration`

### TC-13.1.7.1 Health Apply Damage

| # | Requirement |
|---|-------------|
| 1 | R-13.1.7    |

1. **#1** — Health 100.0, damage 30.0
   - **Expected:** `current == 70.0`, `overkill == 0.0`

### TC-13.1.7.2 Health Clamp Zero

| # | Requirement |
|---|-------------|
| 1 | R-13.1.7    |

1. **#1** — Health 10.0, damage 50.0
   - **Expected:** `current == 0.0`, `overkill == 40.0`

### TC-13.1.7.3 Health Invulnerable

| # | Requirement |
|---|-------------|
| 1 | R-13.1.7    |

1. **#1** — Health 100.0, `invulnerable = true`, damage 50.0
   - **Expected:** `current == 100.0`, `final_amount == 0.0`

### TC-13.1.7.4 Damage Armor Mitigation

| # | Requirement |
|---|-------------|
| 1 | R-13.1.7    |

1. **#1** — Raw 100.0, armor 50% physical
   - **Expected:** `final_amount == 50.0`

### TC-13.1.7.5 Damage Resistance Per School

| # | Requirement |
|---|-------------|
| 1 | R-13.1.7    |

1. **#1** — Fire damage 100.0, fire resist 0.3, physical resist 0.5
   - **Expected:** Fire `final == 70.0`, physical resist not applied

### TC-13.1.7.6 Damage Absorb Shield

| # | Requirement |
|---|-------------|
| 1 | R-13.1.7    |

1. **#1** — Shield 20.0, damage 50.0
   - **Expected:** Shield 0.0, health reduced by 30.0

### TC-13.1.7.7 Damage Determinism

| # | Requirement |
|---|-------------|
| 1 | US-13.1.7.5 |

1. **#1** — 1000 identical `DamageRequest` events
   - **Expected:** 1000 identical `DamageEvent` outputs

### TC-13.1.0.1 Tag Set Insert Remove

| # | Requirement |
|---|-------------|
| 1 | R-13.1.1    |

1. **#1** — Insert tags [A, B, C], remove B
   - **Expected:** `contains(A) == true`, `contains(B) == false`, `contains(C) == true`

### TC-13.1.0.2 Tag Set Contains All

| # | Requirement |
|---|-------------|
| 1 | R-13.1.1    |
| 2 | R-13.1.1    |

1. **#1** — Set {A, B, C}, query {A, B}
   - **Expected:** `contains_all == true`
2. **#2** — Set {A, C}, query {A, B}
   - **Expected:** `contains_all == false`

### TC-13.1.0.3 Tag Set Contains Any

| # | Requirement |
|---|-------------|
| 1 | R-13.1.1    |
| 2 | R-13.1.1    |

1. **#1** — Set {A, C}, query {B, C}
   - **Expected:** `contains_any == true`
2. **#2** — Set {A}, query {B, C}
   - **Expected:** `contains_any == false`

### TC-13.1.NF2.1 Timer Tick Expiry

| # | Requirement |
|---|-------------|
| 1 | R-13.1.NF2  |

1. **#1** — Timer 1.5s at 60 fps, tick until expired
   - **Expected:** Expires within 1 ms of 1.5s target

### TC-13.1.NF2.2 Timer Repeating

| # | Requirement |
|---|-------------|
| 1 | R-13.1.NF2  |

1. **#1** — Repeating timer 1.0s, tick past 1.0s
   - **Expected:** `is_expired() == true`, then auto-resets `remaining ~= 1.0`

### TC-13.1.5.1 Cooldown Not Ready

| # | Requirement |
|---|-------------|
| 1 | US-13.1.5.6 |

1. **#1** — Trigger 5.0s cooldown, immediately query
   - **Expected:** `is_ready() == false`

### TC-13.1.5.2 Cooldown Expired Event

| # | Requirement |
|---|-------------|
| 1 | R-13.1.5    |

1. **#1** — Trigger 1.0s cooldown, tick 1.0s
   - **Expected:** Exactly 1 `CooldownExpiredEvent` emitted

### TC-13.1.4.1 Allegiance Symmetric

| # | Requirement |
|---|-------------|
| 1 | R-13.1.4    |

1. **#1** — `set_disposition(A, B, Hostile)`
   - **Expected:** `disposition(B, A) == Hostile`

### TC-13.1.4.2 Allegiance Default Neutral

| # | Requirement |
|---|-------------|
| 1 | R-13.1.4    |

1. **#1** — Query unset faction pair (C, D)
   - **Expected:** `disposition(C, D) == Neutral`

### TC-13.1.4.3 Pawn Possession Transfer

| # | Requirement |
|---|-------------|
| 1 | US-13.1.4.4 |

1. **#1** — Possess pawn A, then possess pawn B
   - **Expected:** A released with state intact, B possessed

### TC-13.1.3.1 Controller Context Clears Queue

| # | Requirement |
|---|-------------|
| 1 | US-13.1.3.6 |

1. **#1** — Queue 2 inputs, switch context
   - **Expected:** `input_queue.len() == 0`

### TC-13.1.4.4 Spawn From Template

| # | Requirement |
|---|-------------|
| 1 | R-13.1.4    |

1. **#1** — `SpawnRequest` with 5-component template
   - **Expected:** Entity spawned with all 5 components present

### TC-13.1.4.5 Spawn With Children

| # | Requirement |
|---|-------------|
| 1 | R-13.1.4    |

1. **#1** — Template with 2 child templates
   - **Expected:** Parent + 2 children with `ChildOf` relationships

### TC-13.1.9.1 Module Enable Transitive

| # | Requirement |
|---|-------------|
| 1 | US-13.1.9.5 |

1. **#1** — Enable Combat (depends on Physics, Animation)
   - **Expected:** Physics and Animation auto-enabled, returns `[Physics, Animation]`

### TC-13.1.9.2 Module Disable With Dependents

| # | Requirement |
|---|-------------|
| 1 | R-13.1.9    |

1. **#1** — Disable Physics while Combat enabled
   - **Expected:** `Err(DependentModulesActive { dependents: [Combat] })`

### TC-13.1.9.3 Module Cycle Detection

| # | Requirement |
|---|-------------|
| 1 | R-13.1.9    |

1. **#1** — Register A->B->C->A dependency cycle
   - **Expected:** `Err(CycleDetected { path: [A, B, C] })`

## Integration Tests

### TC-13.1.2.I1 Full State Lifecycle

| # | Requirement |
|---|-------------|
| 1 | R-13.1.2    |

1. **#1** — Transition through MainMenu->Loading->InGame->Paused->InGame->Loading->MainMenu
   - **Expected:** Resources load/unload at each step; no leaks

### TC-13.1.2.I2 Rapid Pause Unpause

| # | Requirement |
|---|-------------|
| 1 | US-13.1.2.5 |

1. **#1** — Toggle Paused/InGame at 60 Hz for 10 seconds
   - **Expected:** No state corruption; entity count stable

### TC-13.1.NF1.I1 Mode Transition Under One Frame

| # | Requirement |
|---|-------------|
| 1 | R-13.1.NF1  |

1. **#1** — Trigger mode transition, measure latency
   - **Expected:** Latency < 16.67 ms

### TC-13.1.7.I1 Damage 1K Events Per Frame

| # | Requirement |
|---|-------------|
| 1 | R-13.1.7    |

1. **#1** — 1000 `DamageRequest` events in one frame
   - **Expected:** All health values correct; total < 0.5 ms

### TC-13.1.8.I1 Death Respawn Full Cycle

| # | Requirement |
|---|-------------|
| 1 | R-13.1.8    |

1. **#1** — Kill entity, wait for respawn timer
   - **Expected:** Entity respawns at nearest valid point with full health

### TC-13.1.8.I2 Encounter Reset

| # | Requirement |
|---|-------------|
| 1 | US-13.1.8.4 |

1. **#1** — Trigger wipe event on encounter
   - **Expected:** Boss HP restored, adds despawned, mode reverts

### TC-13.1.8.I3 Death Debuff Stacking

| # | Requirement |
|---|-------------|
| 1 | US-13.1.8.6 |

1. **#1** — Die 3 times within reset window
   - **Expected:** `DeathCounter.count == 3`; debuff stacks applied

### TC-13.1.4.I1 Spawn 100 Entities

| # | Requirement |
|---|-------------|
| 1 | R-13.1.4    |

1. **#1** — 100 `SpawnRequest` events in one frame
   - **Expected:** 100 entities created; total time < 1 ms

### TC-13.1.0.I1 Tag Query 10K Entities

| # | Requirement |
|---|-------------|
| 1 | R-13.1.1    |

1. **#1** — 10,000 entities with 8 tags, filter by 1 tag
   - **Expected:** Correct subset returned; query time < 0.1 ms

### TC-13.1.NF2.I1 Cooldown Precision 144fps

| # | Requirement |
|---|-------------|
| 1 | R-13.1.NF2  |

1. **#1** — 1.5s cooldown at 144 fps, 1000 trials
   - **Expected:** Max drift <= 1 ms across all trials

### TC-13.1.2.I3 Server Authoritative State

| # | Requirement |
|---|-------------|
| 1 | US-13.1.2.2 |

1. **#1** — Client-server transition sequence
   - **Expected:** Both agree on state after all transitions

### TC-13.1.3.I1 Controller All Contexts

| # | Requirement |
|---|-------------|
| 1 | US-13.1.3.6 |

1. **#1** — Exercise all `InputContext` transitions
   - **Expected:** No input leaks between contexts

### TC-13.1.6.I1 Effect Cleanup On Death

| # | Requirement |
|---|-------------|
| 1 | US-13.1.6.7 |

1. **#1** — Kill entity with 5 active effects
   - **Expected:** All effect references cleaned up; no dangling entities

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
