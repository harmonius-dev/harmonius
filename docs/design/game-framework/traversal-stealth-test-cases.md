# Traversal and Stealth System Test Cases

Companion test cases for [traversal-stealth.md](traversal-stealth.md).

## Unit Tests

### TC-13.17.4a.1 Vault Height Classification

| # | Requirement |
|---|-------------|
| 1 | R-13.17.4a  |

1. **#1** — Obstacle height = 0.5 m
   - **Expected:** `TraversalType::Vault`

### TC-13.17.4a.2 Mantle Height Classification

| # | Requirement |
|---|-------------|
| 1 | R-13.17.4a  |

1. **#1** — Obstacle height = 1.5 m
   - **Expected:** `TraversalType::Mantle`

### TC-13.17.4a.3 Auto Detection No Tags

| # | Requirement |
|---|-------------|
| 1 | R-13.17.4a  |

1. **#1** — Surface without editor tags, height = 0.6 m
   - **Expected:** `TraversalOpportunity` generated as Vault

### TC-13.17.4b.1 Vault Stamina Deduction

| # | Requirement |
|---|-------------|
| 1 | R-13.17.4b  |

1. **#1** — Vault with `vault_stamina_cost = 10.0`, stamina = 50.0
   - **Expected:** Stamina = 40.0 after vault

### TC-13.17.4b.2 Vault Fails Low Stamina

| # | Requirement |
|---|-------------|
| 1 | R-13.17.4b  |

1. **#1** — `vault_stamina_cost = 10.0`, stamina = 5.0
   - **Expected:** Vault blocked; stamina unchanged

### TC-13.17.4b.3 Vault Min Approach Speed

| # | Requirement |
|---|-------------|
| 1 | R-13.17.4b  |

1. **#1** — `min_approach_speed = 3.0`, current speed = 2.0
   - **Expected:** Vault not triggered

### TC-13.17.4c.1 Wall Run Gravity Timer

| # | Requirement |
|---|-------------|
| 1 | R-13.17.4c  |

1. **#1** — `max_duration = 1.5`, run for 1.6 seconds
   - **Expected:** Wall run terminates at 1.5s

### TC-13.17.4c.2 Wall Run Jump Off Angle

| # | Requirement |
|---|-------------|
| 1 | R-13.17.4c  |

1. **#1** — `jump_off_angle = 45.0` degrees, jump off wall
   - **Expected:** Launch direction 45 degrees from wall normal

### TC-13.17.4d.1 Slide Distance Scales

| # | Requirement |
|---|-------------|
| 1 | R-13.17.4d  |

1. **#1** — Entry speed 5 m/s vs 10 m/s
   - **Expected:** 10 m/s slide covers greater distance

### TC-13.17.4d.2 Slide Downhill Extends

| # | Requirement |
|---|-------------|
| 1 | R-13.17.4d  |

1. **#1** — Flat slide vs 30-degree downhill, same entry speed
   - **Expected:** Downhill slide covers greater distance

### TC-13.17.4e.1 Balance Fall On Speed

| # | Requirement |
|---|-------------|
| 1 | R-13.17.4e  |

1. **#1** — Narrow surface, movement speed above threshold
   - **Expected:** Character falls off

### TC-13.17.5a.1 Climb Stamina Drain

| # | Requirement |
|---|-------------|
| 1 | R-13.17.5a  |

1. **#1** — `stamina_drain = 5.0/s`, climb for 10s
   - **Expected:** Stamina depleted; character falls at 0

### TC-13.17.5a.2 Climb Rest Point

| # | Requirement |
|---|-------------|
| 1 | R-13.17.5a  |

1. **#1** — Reach `ClimbRestPoint` entity
   - **Expected:** Stamina drain paused

### TC-13.17.5b.1 Ladder No Stamina

| # | Requirement |
|---|-------------|
| 1 | R-13.17.5b  |

1. **#1** — Climb ladder for 30s
   - **Expected:** Stamina unchanged

### TC-13.17.5c.1 Ledge Grab Airborne

| # | Requirement |
|---|-------------|
| 1 | R-13.17.5c  |
| 2 | R-13.17.5c  |

1. **#1** — Airborne near ledge edge
   - **Expected:** `LedgeGrab` triggers
2. **#2** — Grounded near ledge edge
   - **Expected:** `LedgeGrab` does not trigger

### TC-13.17.6.1 Swim Oxygen Drain

| # | Requirement |
|---|-------------|
| 1 | R-13.17.6   |

1. **#1** — `oxygen_drain = 10.0/s`, max_oxygen = 100, submerge 5s
   - **Expected:** `SwimState.oxygen == 50.0`

### TC-13.17.6.2 Drowning Damage

| # | Requirement |
|---|-------------|
| 1 | R-13.17.6   |

1. **#1** — Oxygen = 0, `drowning_damage = 5.0/s`, 2 seconds
   - **Expected:** 10.0 damage applied to Health

### TC-13.17.7.1 Grapple Range Limit

| # | Requirement |
|---|-------------|
| 1 | R-13.17.7   |
| 2 | R-13.17.7   |

1. **#1** — Target at 50 m, `range = 30 m`
   - **Expected:** Hook fails to attach
2. **#2** — Target at 25 m, `range = 30 m`
   - **Expected:** Hook attaches

### TC-13.17.1.1 Interaction Instant

| # | Requirement |
|---|-------------|
| 1 | R-13.17.1   |

1. **#1** — Press interact on `InteractionType::Instant` entity
   - **Expected:** Logic graph executes immediately

### TC-13.17.1.2 Interaction Channeled

| # | Requirement |
|---|-------------|
| 1 | R-13.17.1   |

1. **#1** — Hold interact for `channel_duration = 3.0s`
   - **Expected:** Completes at 3.0s

### TC-13.17.1.3 Interaction Cancel

| # | Requirement |
|---|-------------|
| 1 | R-13.17.1   |

1. **#1** — Start channel, move during channel
   - **Expected:** Channel cancelled, interaction not completed

### TC-13.17.2.1 Door Key Required

| # | Requirement |
|---|-------------|
| 1 | R-13.17.2   |

1. **#1** — Locked door, player has no key
   - **Expected:** Door remains locked

### TC-13.17.2.2 Door NPC Key Usage

| # | Requirement |
|---|-------------|
| 1 | R-13.17.2   |

1. **#1** — NPC with matching key approaches locked door
   - **Expected:** Door opens

### TC-13.17.3.1 Pickup Hold Point

| # | Requirement |
|---|-------------|
| 1 | R-13.17.3   |

1. **#1** — Pick up `Grabbable` object
   - **Expected:** Object at `hold_distance` from character

### TC-13.17.3.2 Throw Damage

| # | Requirement |
|---|-------------|
| 1 | R-13.17.3   |

1. **#1** — Throw held object at target
   - **Expected:** Damage applied based on `throw_strength` and mass

### TC-13.18.1.1 Visibility Darkness

| # | Requirement |
|---|-------------|
| 1 | R-13.18.1   |

1. **#1** — `light_level = 0.1`, standing, no gear
   - **Expected:** `score < 0.15`

### TC-13.18.1.2 Visibility Crouch

| # | Requirement |
|---|-------------|
| 1 | R-13.18.1   |

1. **#1** — Same conditions, standing vs crouching
   - **Expected:** Crouching score < standing score

### TC-13.18.1.3 Visibility Override

| # | Requirement |
|---|-------------|
| 1 | R-13.18.1   |

1. **#1** — `ability_override = Some(0.0)` (invisibility)
   - **Expected:** `score == 0.0` regardless of other factors

### TC-13.18.2.1 Alert Hysteresis

| # | Requirement |
|---|-------------|
| 1 | R-13.18.2   |

1. **#1** — Brief 0.5s glimpse, `alert_threshold = 2.0`
   - **Expected:** `AlertLevel::Suspicious`, not `Alerted`

### TC-13.18.2.2 Alert Sustained Detection

| # | Requirement |
|---|-------------|
| 1 | R-13.18.2   |

1. **#1** — Sustained detection for 3.0s, `alert_threshold = 2.0`
   - **Expected:** Transitions to `AlertLevel::Alerted`

### TC-13.18.2.3 Alert Search Timeout

| # | Requirement |
|---|-------------|
| 1 | R-13.18.2   |

1. **#1** — Searching for `search_timeout` seconds, no stimulus
   - **Expected:** Transitions to `LostTarget` then `Unaware`

### TC-13.18.3.1 Noise Distance Attenuation

| # | Requirement |
|---|-------------|
| 1 | R-13.18.3   |

1. **#1** — Noise intensity 1.0 at 10 m, AI at 20 m, range 30 m
   - **Expected:** Effective intensity = 1.0 * (1 - 20/30) = 0.33

### TC-13.18.3.2 Noise Wall Occlusion

| # | Requirement |
|---|-------------|
| 1 | R-13.18.3   |

1. **#1** — Noise through closed door with 0.5 attenuation
   - **Expected:** Effective intensity *= 0.5

### TC-13.18.3.3 Distraction Lure

| # | Requirement |
|---|-------------|
| 1 | R-13.18.3   |

1. **#1** — Throw distraction at point P
   - **Expected:** AI investigates point P

### TC-13.18.4.1 Takedown Silent

| # | Requirement |
|---|-------------|
| 1 | R-13.18.4   |

1. **#1** — Silent takedown, nearby AI 10 m away
   - **Expected:** Nearby AI remains Unaware

### TC-13.18.4.2 Takedown Loud Alerts

| # | Requirement |
|---|-------------|
| 1 | R-13.18.4   |

1. **#1** — Loud takedown, nearby AI 10 m away
   - **Expected:** Nearby AI transitions to Suspicious or Alerted

### TC-13.18.4.3 Takedown Preconditions

| # | Requirement |
|---|-------------|
| 1 | R-13.18.4   |
| 2 | R-13.18.4   |

1. **#1** — Attempt takedown from front, target aware
   - **Expected:** Takedown fails
2. **#2** — Behind target, target Unaware
   - **Expected:** Takedown succeeds

### TC-13.18.5.1 Cover Half Classification

| # | Requirement |
|---|-------------|
| 1 | R-13.18.5   |

1. **#1** — Wall height = 0.9 m (waist high)
   - **Expected:** `CoverType::Half`

### TC-13.18.5.2 Cover Full Classification

| # | Requirement |
|---|-------------|
| 1 | R-13.18.5   |

1. **#1** — Wall height = 1.8 m (standing)
   - **Expected:** `CoverType::Full`

### TC-13.18.5.3 Cover Peek Exposure

| # | Requirement |
|---|-------------|
| 1 | R-13.18.5   |

1. **#1** — Peek right from half cover
   - **Expected:** Partial body exposed; `VisibilityScore` increases

### TC-13.18.5.4 Cover Blind Fire Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-13.18.5   |

1. **#1** — Blind fire from cover
   - **Expected:** Accuracy reduced vs aimed fire

### TC-13.18.5.5 Cover To Cover Sprint

| # | Requirement |
|---|-------------|
| 1 | R-13.18.5   |

1. **#1** — Select adjacent cover, sprint
   - **Expected:** Character moves to new cover, snaps on arrival

### TC-13.18.5.6 Cover Directional Flank

| # | Requirement |
|---|-------------|
| 1 | R-13.18.5   |

1. **#1** — `Directional` cover facing north, attack from east
   - **Expected:** Cover provides no protection

## Integration Tests

### TC-13.17.1.I1 Traversal Animation Transition

| # | Requirement |
|---|-------------|
| 1 | NFR-13.17.1 |

1. **#1** — Detect vault opportunity, trigger
   - **Expected:** Animation starts within 2 frames

### TC-13.17.2.I1 Traversal 200 Interactables

| # | Requirement |
|---|-------------|
| 1 | NFR-13.17.2 |

1. **#1** — 200 interactable entities in range
   - **Expected:** Detection completes in < 1 ms

### TC-13.18.1.I1 Stealth 32 Entities

| # | Requirement |
|---|-------------|
| 1 | NFR-13.18.1 |

1. **#1** — 32 tracked entities
   - **Expected:** Visibility computation < 2 ms total

### TC-13.18.2.I1 Cover 10K Surfaces

| # | Requirement |
|---|-------------|
| 1 | NFR-13.18.2 |

1. **#1** — 10,000 surfaces at level load
   - **Expected:** Detection completes in < 500 ms

### TC-13.18.5.I1 AI Cover Scoring

| # | Requirement |
|---|-------------|
| 1 | R-13.18.5   |

1. **#1** — AI evaluates 20 cover points in combat
   - **Expected:** Selects highest-scoring cover; consistent with player rules

## Benchmarks

### TC-13.17.1.B1 Traversal Detection Per Frame

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Per-frame shape cast detection | Frame time | < 0.5 ms | NFR-13.17.1 |

### TC-13.17.2.B1 Interaction Detection

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 200 interactable entities | Detection time | < 1 ms | NFR-13.17.2 |

### TC-13.18.1.B1 Visibility Score

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 32 tracked entities | Computation time | < 2 ms | NFR-13.18.1 |

### TC-13.18.2.B1 Cover Detection At Load

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10,000 surfaces | Detection time | < 500 ms | NFR-13.18.2 |

### TC-13.18.2.B2 Cover Point Query

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Query within 30 m radius | Query time | < 0.3 ms | NFR-13.18.2 |
