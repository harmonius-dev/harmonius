# Traversal and Stealth System Test Cases

Companion test cases for [traversal-stealth.md](traversal-stealth.md).

## Unit Tests

### TC-13.17.4a.1 Vault Height Classification

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Obstacle height = 0.5 m | `TraversalType::Vault` | R-13.17.4a |

### TC-13.17.4a.2 Mantle Height Classification

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Obstacle height = 1.5 m | `TraversalType::Mantle` | R-13.17.4a |

### TC-13.17.4a.3 Auto Detection No Tags

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Surface without editor tags, height = 0.6 m | `TraversalOpportunity` generated as Vault | R-13.17.4a |

### TC-13.17.4b.1 Vault Stamina Deduction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Vault with `vault_stamina_cost = 10.0`, stamina = 50.0 | Stamina = 40.0 after vault | R-13.17.4b |

### TC-13.17.4b.2 Vault Fails Low Stamina

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `vault_stamina_cost = 10.0`, stamina = 5.0 | Vault blocked; stamina unchanged | R-13.17.4b |

### TC-13.17.4b.3 Vault Min Approach Speed

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `min_approach_speed = 3.0`, current speed = 2.0 | Vault not triggered | R-13.17.4b |

### TC-13.17.4c.1 Wall Run Gravity Timer

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `max_duration = 1.5`, run for 1.6 seconds | Wall run terminates at 1.5s | R-13.17.4c |

### TC-13.17.4c.2 Wall Run Jump Off Angle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `jump_off_angle = 45.0` degrees, jump off wall | Launch direction 45 degrees from wall normal | R-13.17.4c |

### TC-13.17.4d.1 Slide Distance Scales

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Entry speed 5 m/s vs 10 m/s | 10 m/s slide covers greater distance | R-13.17.4d |

### TC-13.17.4d.2 Slide Downhill Extends

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Flat slide vs 30-degree downhill, same entry speed | Downhill slide covers greater distance | R-13.17.4d |

### TC-13.17.4e.1 Balance Fall On Speed

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Narrow surface, movement speed above threshold | Character falls off | R-13.17.4e |

### TC-13.17.5a.1 Climb Stamina Drain

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `stamina_drain = 5.0/s`, climb for 10s | Stamina depleted; character falls at 0 | R-13.17.5a |

### TC-13.17.5a.2 Climb Rest Point

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Reach `ClimbRestPoint` entity | Stamina drain paused | R-13.17.5a |

### TC-13.17.5b.1 Ladder No Stamina

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Climb ladder for 30s | Stamina unchanged | R-13.17.5b |

### TC-13.17.5c.1 Ledge Grab Airborne

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Airborne near ledge edge | `LedgeGrab` triggers | R-13.17.5c |
| 2 | Grounded near ledge edge | `LedgeGrab` does not trigger | R-13.17.5c |

### TC-13.17.6.1 Swim Oxygen Drain

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `oxygen_drain = 10.0/s`, max_oxygen = 100, submerge 5s | `SwimState.oxygen == 50.0` | R-13.17.6 |

### TC-13.17.6.2 Drowning Damage

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Oxygen = 0, `drowning_damage = 5.0/s`, 2 seconds | 10.0 damage applied to Health | R-13.17.6 |

### TC-13.17.7.1 Grapple Range Limit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Target at 50 m, `range = 30 m` | Hook fails to attach | R-13.17.7 |
| 2 | Target at 25 m, `range = 30 m` | Hook attaches | R-13.17.7 |

### TC-13.17.1.1 Interaction Instant

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Press interact on `InteractionType::Instant` entity | Logic graph executes immediately | R-13.17.1 |

### TC-13.17.1.2 Interaction Channeled

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hold interact for `channel_duration = 3.0s` | Completes at 3.0s | R-13.17.1 |

### TC-13.17.1.3 Interaction Cancel

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Start channel, move during channel | Channel cancelled, interaction not completed | R-13.17.1 |

### TC-13.17.2.1 Door Key Required

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Locked door, player has no key | Door remains locked | R-13.17.2 |

### TC-13.17.2.2 Door NPC Key Usage

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | NPC with matching key approaches locked door | Door opens | R-13.17.2 |

### TC-13.17.3.1 Pickup Hold Point

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pick up `Grabbable` object | Object at `hold_distance` from character | R-13.17.3 |

### TC-13.17.3.2 Throw Damage

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Throw held object at target | Damage applied based on `throw_strength` and mass | R-13.17.3 |

### TC-13.18.1.1 Visibility Darkness

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `light_level = 0.1`, standing, no gear | `score < 0.15` | R-13.18.1 |

### TC-13.18.1.2 Visibility Crouch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same conditions, standing vs crouching | Crouching score < standing score | R-13.18.1 |

### TC-13.18.1.3 Visibility Override

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `ability_override = Some(0.0)` (invisibility) | `score == 0.0` regardless of other factors | R-13.18.1 |

### TC-13.18.2.1 Alert Hysteresis

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Brief 0.5s glimpse, `alert_threshold = 2.0` | `AlertLevel::Suspicious`, not `Alerted` | R-13.18.2 |

### TC-13.18.2.2 Alert Sustained Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sustained detection for 3.0s, `alert_threshold = 2.0` | Transitions to `AlertLevel::Alerted` | R-13.18.2 |

### TC-13.18.2.3 Alert Search Timeout

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Searching for `search_timeout` seconds, no stimulus | Transitions to `LostTarget` then `Unaware` | R-13.18.2 |

### TC-13.18.3.1 Noise Distance Attenuation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Noise intensity 1.0 at 10 m, AI at 20 m, range 30 m | Effective intensity = 1.0 * (1 - 20/30) = 0.33 | R-13.18.3 |

### TC-13.18.3.2 Noise Wall Occlusion

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Noise through closed door with 0.5 attenuation | Effective intensity *= 0.5 | R-13.18.3 |

### TC-13.18.3.3 Distraction Lure

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Throw distraction at point P | AI investigates point P | R-13.18.3 |

### TC-13.18.4.1 Takedown Silent

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Silent takedown, nearby AI 10 m away | Nearby AI remains Unaware | R-13.18.4 |

### TC-13.18.4.2 Takedown Loud Alerts

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Loud takedown, nearby AI 10 m away | Nearby AI transitions to Suspicious or Alerted | R-13.18.4 |

### TC-13.18.4.3 Takedown Preconditions

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Attempt takedown from front, target aware | Takedown fails | R-13.18.4 |
| 2 | Behind target, target Unaware | Takedown succeeds | R-13.18.4 |

### TC-13.18.5.1 Cover Half Classification

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Wall height = 0.9 m (waist high) | `CoverType::Half` | R-13.18.5 |

### TC-13.18.5.2 Cover Full Classification

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Wall height = 1.8 m (standing) | `CoverType::Full` | R-13.18.5 |

### TC-13.18.5.3 Cover Peek Exposure

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Peek right from half cover | Partial body exposed; `VisibilityScore` increases | R-13.18.5 |

### TC-13.18.5.4 Cover Blind Fire Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Blind fire from cover | Accuracy reduced vs aimed fire | R-13.18.5 |

### TC-13.18.5.5 Cover To Cover Sprint

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Select adjacent cover, sprint | Character moves to new cover, snaps on arrival | R-13.18.5 |

### TC-13.18.5.6 Cover Directional Flank

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `Directional` cover facing north, attack from east | Cover provides no protection | R-13.18.5 |

## Integration Tests

### TC-13.17.1.I1 Traversal Animation Transition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Detect vault opportunity, trigger | Animation starts within 2 frames | NFR-13.17.1 |

### TC-13.17.2.I1 Traversal 200 Interactables

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 200 interactable entities in range | Detection completes in < 1 ms | NFR-13.17.2 |

### TC-13.18.1.I1 Stealth 32 Entities

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 32 tracked entities | Visibility computation < 2 ms total | NFR-13.18.1 |

### TC-13.18.2.I1 Cover 10K Surfaces

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10,000 surfaces at level load | Detection completes in < 500 ms | NFR-13.18.2 |

### TC-13.18.5.I1 AI Cover Scoring

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | AI evaluates 20 cover points in combat | Selects highest-scoring cover; consistent with player rules | R-13.18.5 |

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
