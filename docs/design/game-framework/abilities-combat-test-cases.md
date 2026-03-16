# Abilities and Combat Test Cases

Companion test cases for [abilities-combat.md](abilities-combat.md).

## Unit Tests

### TC-13.10.1.1 Tag Set Operations

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `insert(tag_0); contains(tag_0)` | `true` | R-13.10.1 |
| 2 | `insert(tag_0); remove(tag_0); contains(tag_0)` | `false` | R-13.10.1 |
| 3 | `{tag_0, tag_1}.union({tag_1, tag_2})` | `{tag_0, tag_1, tag_2}` | R-13.10.1 |
| 4 | `{tag_0, tag_1}.intersection({tag_1, tag_2})` | `{tag_1}` | R-13.10.1 |
| 5 | `{tag_0, tag_1}.has_any({tag_2, tag_3})` | `false` | R-13.10.1 |
| 6 | `{tag_0, tag_1}.has_all({tag_0, tag_1})` | `true` | R-13.10.1 |

### TC-13.10.2.1 Ability Activation Press Mode

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `ActivateAbilityRequest(entity, press_ability)` with 100 mana, cooldown ready | `ActivationResult::Activated`; mana = 70; cooldown started | R-13.10.2 |
| 2 | `ActivateAbilityRequest(entity, press_ability)` while on cooldown (remaining=30) | `ActivationRejection::OnCooldown { remaining_ticks: 30 }` | R-13.10.2 |
| 3 | `ActivateAbilityRequest(entity, press_ability)` with mana=10, cost=30 | `ActivationRejection::InsufficientResource { required: 30.0, current: 10.0 }` | R-13.10.2 |
| 4 | `ActivateAbilityRequest(entity, press_ability)` with blocked_tag=Stun active | `ActivationRejection::BlockedByTag { tag: Stun }` | R-13.10.2 |

### TC-13.10.2.2 Hold Activation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hold input for 60 ticks | Ability active for 60 ticks | R-13.10.2 |
| 2 | Release input after 60 ticks | Ability deactivates immediately | R-13.10.2 |

### TC-13.10.2.3 Charge Activation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Charge for `min_charge_ticks` (30), release | Power = minimum scale (0.0) | R-13.10.2 |
| 2 | Charge for `max_charge_ticks` (120), release | Power = maximum scale (1.0) | R-13.10.2 |
| 3 | Charge for 75 ticks (midpoint), release | Power = 0.5 (linear interpolation) | R-13.10.2 |

### TC-13.10.2.4 Toggle Activation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Press toggle ability once | Ability state = active | R-13.10.2 |
| 2 | Press toggle ability twice | Ability state = inactive | R-13.10.2 |

### TC-13.10.2.5 Combo Chain Advance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Press within window at step 0 | Advances to step 1 (Heavy Slash) | R-13.10.2 |
| 2 | Press within window at step 1 | Advances to step 2 (Finisher Slam) | R-13.10.2 |
| 3 | No input for `window_ticks` after step 0 | Reset to step 0 | R-13.10.2 |

### TC-13.10.2.6 AI Synthetic Input

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | AI sends `ActivateAbilityRequest(entity, ability)` | Same `ActivationResult` as player input | R-13.10.2 |

### TC-13.10.3.1 Instant Damage Effect

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply `GameplayEffect { type: Damage(Physical), magnitude: 50 }` to target with HP=100 | Target HP = 50 | R-13.10.3 |

### TC-13.10.3.2 Duration Buff Effect

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply 30% speed buff, duration=300 ticks | Speed increased by 30% | R-13.10.3 |
| 2 | After 300 ticks | Speed restored to base | R-13.10.3 |

### TC-13.10.3.3 Periodic Heal Effect

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply heal 10 HP, period=120 ticks, duration=600 ticks | Exactly 5 heal ticks | R-13.10.3 |

### TC-13.10.3.4 Stacking Additive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply +10 damage bonus twice (Additive stacking) | Total bonus = +20 | R-13.10.3 |

### TC-13.10.3.5 Stacking Multiplicative

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply 1.2x and 1.5x multipliers | Total multiplier = 1.8x | R-13.10.3 |

### TC-13.10.3.6 Stacking Highest Wins

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply +15% and +25% buffs (HighestWins) | Only +25% applies | R-13.10.3 |

### TC-13.10.3.7 Stacking Non-Stacking

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply debuff (duration=100), reapply same debuff | Duration refreshed to 100; magnitude unchanged | R-13.10.3 |

### TC-13.10.3.8 Tag Interaction Fire vs Frozen

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Target has Frozen debuff; apply Fire damage | Frozen debuff removed | R-13.10.3 |

### TC-13.10.4.1 Hitbox Activation Timing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Play attack animation; query hitbox at frame 10 (before window) | `hitbox.active = false` | R-13.10.4 |
| 2 | Query hitbox at frame 20 (inside window) | `hitbox.active = true` | R-13.10.4 |
| 3 | Query hitbox at frame 40 (after window) | `hitbox.active = false` | R-13.10.4 |

### TC-13.10.4.2 Hurtbox Region Multipliers

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Strike head hurtbox (multiplier=2.0), base damage=50 | Damage = 100 | R-13.10.4 |
| 2 | Strike limb hurtbox (multiplier=0.75), base damage=100 | Damage = 75 | R-13.10.4 |

### TC-13.10.4.3 Directional Back Bonus

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Attacker behind target (back multiplier=1.5), base=100 | Damage = 150 | R-13.10.4 |

### TC-13.10.4.4 Multi-Hit Prevention

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Swing through same entity twice in one swing | `SwingTracker.hit_entities` contains entity once; 1 hit | R-13.10.4 |

### TC-13.10.4.5 Hit-Stop Time Dilation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Land melee hit with `HitStopConfig { duration_ticks: 5, time_scale: 0.1 }` | Time scale = 0.1 for 5 ticks | R-13.10.4 |

### TC-13.10.5.1 Projectile Linear

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fire `TrajectoryType::Linear` projectile, velocity=(0,0,10) | After 60 ticks: position.z = 600 (straight line) | R-13.10.5 |

### TC-13.10.5.2 Projectile Arced

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fire `TrajectoryType::Arced`, gravity_scale=1.0 | Position.y decreases parabolically | R-13.10.5 |

### TC-13.10.5.3 Projectile Homing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fire homing projectile at moving target | Heading converges toward target each tick | R-13.10.5 |

### TC-13.10.5.4 Projectile Spread

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fire `Spread { pellet_count: 8, cone_angle: 15.0 }` | 8 pellets within 15-degree cone | R-13.10.5 |

### TC-13.10.5.5 Projectile CCD

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fire fast projectile (speed=500) at thin wall (0.1m) | CCD detects collision; no tunneling | R-13.10.5 |

### TC-13.10.5.6 Projectile Effect Payload

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Projectile impacts target; payload = 30 damage | Target HP reduced by 30 | R-13.10.5 |

### TC-13.10.5.7 Aim Assist Magnetism

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Aim within `magnetism_radius` of target with gamepad | Aim snaps to target center | R-13.10.5 |

## Integration Tests

### TC-13.10.1.I1 Ability Full Cycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Activate ability; observe animation, VFX, effect | All trigger within 1 frame of input | R-13.10.1, R-13.10.NF2 |

### TC-13.10.4.I1 Melee Full Pipeline

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Swing weapon; hitbox activates; spatial query; damage applies; hit reaction plays | All within 1 frame of anim event | R-13.10.4, R-13.10.NF3 |

### TC-13.10.5.I1 Ranged Full Pipeline

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fire weapon; projectile flies; impacts target | Effect applies; surface VFX spawns | R-13.10.5 |

### TC-13.10.2.I1 Combo Into Ranged

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Execute combo chain; final step triggers projectile | Combo completes; projectile spawns correctly | R-13.10.2 |

### TC-13.10.NF1.I1 64 Concurrent Effects

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply 64 effects to one entity | All evaluate correctly within 0.1 ms | R-13.10.NF1 |

### TC-13.10.NF1.I2 40-Entity Raid

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 40 entities with 64 effects each | Total evaluation under 4 ms | R-13.10.NF1 |

### TC-13.10.6.I1 Lag Compensation 100ms

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Simulate 100 ms latency; client fires at target | Server resolves hit against historical snapshot at client fire time | R-13.10.6 |

### TC-13.10.1.I2 Shared Effect Component

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two abilities share "Fire Damage" effect; activate each | Both produce identical damage values | R-13.10.1 |

## Benchmarks

### TC-13.10.NF1.B1 Effect Evaluation Single Entity

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 64 effects on 1 entity | Wall-clock time | < 0.1 ms | R-13.10.NF1 |

### TC-13.10.NF1.B2 Effect Evaluation Raid

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 64 effects on 40 entities | Wall-clock time | < 4 ms | R-13.10.NF1 |

### TC-13.10.NF2.B1 Ability Activation Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Input to ability activation | Frame latency | < 1 frame (16.67 ms) | R-13.10.NF2 |

### TC-13.10.NF3.B1 Melee Hit Detection Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Animation event to hit registered | Frame latency | < 1 frame (16.67 ms) | R-13.10.NF3 |

### TC-13.10.1.B1 Tag Set Operations

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 128-bit bitmask union/intersection | Wall-clock time | < 10 ns | R-13.10.1 |

### TC-13.10.5.B1 Projectile CCD Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 256 projectiles with CCD | Wall-clock time | < 1 ms | R-13.10.5 |

### TC-13.10.2.B1 Combo Input Processing

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Combo input validation and advance | Wall-clock time | < 0.01 ms | R-13.10.2 |
