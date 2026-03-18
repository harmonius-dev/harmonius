# Abilities and Combat Test Cases

Companion test cases for [abilities-combat.md](abilities-combat.md).

## Unit Tests

### TC-13.10.1.1 Tag Set Operations

| # | Requirement |
|---|-------------|
| 1 | R-13.10.1   |
| 2 | R-13.10.1   |
| 3 | R-13.10.1   |
| 4 | R-13.10.1   |
| 5 | R-13.10.1   |
| 6 | R-13.10.1   |

1. **#1** — `insert(tag_0); contains(tag_0)`
   - **Expected:** `true`
2. **#2** — `insert(tag_0); remove(tag_0); contains(tag_0)`
   - **Expected:** `false`
3. **#3** — `{tag_0, tag_1}.union({tag_1, tag_2})`
   - **Expected:** `{tag_0, tag_1, tag_2}`
4. **#4** — `{tag_0, tag_1}.intersection({tag_1, tag_2})`
   - **Expected:** `{tag_1}`
5. **#5** — `{tag_0, tag_1}.has_any({tag_2, tag_3})`
   - **Expected:** `false`
6. **#6** — `{tag_0, tag_1}.has_all({tag_0, tag_1})`
   - **Expected:** `true`

### TC-13.10.2.1 Ability Activation Press Mode

| # | Requirement |
|---|-------------|
| 1 | R-13.10.2   |
| 2 | R-13.10.2   |
| 3 | R-13.10.2   |
| 4 | R-13.10.2   |

1. **#1** — `ActivateAbilityRequest(entity, press_ability)` with 100 mana, cooldown ready
   - **Expected:** `ActivationResult::Activated`; mana = 70; cooldown started
2. **#2** — `ActivateAbilityRequest(entity, press_ability)` while on cooldown (remaining=30)
   - **Expected:** `ActivationRejection::OnCooldown { remaining_ticks: 30 }`
3. **#3** — `ActivateAbilityRequest(entity, press_ability)` with mana=10, cost=30
   - **Expected:** `ActivationRejection::InsufficientResource { required: 30.0, current: 10.0 }`
4. **#4** — `ActivateAbilityRequest(entity, press_ability)` with blocked_tag=Stun active
   - **Expected:** `ActivationRejection::BlockedByTag { tag: Stun }`

### TC-13.10.2.2 Hold Activation

| # | Requirement |
|---|-------------|
| 1 | R-13.10.2   |
| 2 | R-13.10.2   |

1. **#1** — Hold input for 60 ticks
   - **Expected:** Ability active for 60 ticks
2. **#2** — Release input after 60 ticks
   - **Expected:** Ability deactivates immediately

### TC-13.10.2.3 Charge Activation

| # | Requirement |
|---|-------------|
| 1 | R-13.10.2   |
| 2 | R-13.10.2   |
| 3 | R-13.10.2   |

1. **#1** — Charge for `min_charge_ticks` (30), release
   - **Expected:** Power = minimum scale (0.0)
2. **#2** — Charge for `max_charge_ticks` (120), release
   - **Expected:** Power = maximum scale (1.0)
3. **#3** — Charge for 75 ticks (midpoint), release
   - **Expected:** Power = 0.5 (linear interpolation)

### TC-13.10.2.4 Toggle Activation

| # | Requirement |
|---|-------------|
| 1 | R-13.10.2   |
| 2 | R-13.10.2   |

1. **#1** — Press toggle ability once
   - **Expected:** Ability state = active
2. **#2** — Press toggle ability twice
   - **Expected:** Ability state = inactive

### TC-13.10.2.5 Combo Chain Advance

| # | Requirement |
|---|-------------|
| 1 | R-13.10.2   |
| 2 | R-13.10.2   |
| 3 | R-13.10.2   |

1. **#1** — Press within window at step 0
   - **Expected:** Advances to step 1 (Heavy Slash)
2. **#2** — Press within window at step 1
   - **Expected:** Advances to step 2 (Finisher Slam)
3. **#3** — No input for `window_ticks` after step 0
   - **Expected:** Reset to step 0

### TC-13.10.2.6 AI Synthetic Input

| # | Requirement |
|---|-------------|
| 1 | R-13.10.2   |

1. **#1** — AI sends `ActivateAbilityRequest(entity, ability)`
   - **Expected:** Same `ActivationResult` as player input

### TC-13.10.3.1 Instant Damage Effect

| # | Requirement |
|---|-------------|
| 1 | R-13.10.3   |

1. **#1** — Apply `GameplayEffect { type: Damage(Physical), magnitude: 50 }` to target with HP=100
   - **Expected:** Target HP = 50

### TC-13.10.3.2 Duration Buff Effect

| # | Requirement |
|---|-------------|
| 1 | R-13.10.3   |
| 2 | R-13.10.3   |

1. **#1** — Apply 30% speed buff, duration=300 ticks
   - **Expected:** Speed increased by 30%
2. **#2** — After 300 ticks
   - **Expected:** Speed restored to base

### TC-13.10.3.3 Periodic Heal Effect

| # | Requirement |
|---|-------------|
| 1 | R-13.10.3   |

1. **#1** — Apply heal 10 HP, period=120 ticks, duration=600 ticks
   - **Expected:** Exactly 5 heal ticks

### TC-13.10.3.4 Stacking Additive

| # | Requirement |
|---|-------------|
| 1 | R-13.10.3   |

1. **#1** — Apply +10 damage bonus twice (Additive stacking)
   - **Expected:** Total bonus = +20

### TC-13.10.3.5 Stacking Multiplicative

| # | Requirement |
|---|-------------|
| 1 | R-13.10.3   |

1. **#1** — Apply 1.2x and 1.5x multipliers
   - **Expected:** Total multiplier = 1.8x

### TC-13.10.3.6 Stacking Highest Wins

| # | Requirement |
|---|-------------|
| 1 | R-13.10.3   |

1. **#1** — Apply +15% and +25% buffs (HighestWins)
   - **Expected:** Only +25% applies

### TC-13.10.3.7 Stacking Non-Stacking

| # | Requirement |
|---|-------------|
| 1 | R-13.10.3   |

1. **#1** — Apply debuff (duration=100), reapply same debuff
   - **Expected:** Duration refreshed to 100; magnitude unchanged

### TC-13.10.3.8 Tag Interaction Fire vs Frozen

| # | Requirement |
|---|-------------|
| 1 | R-13.10.3   |

1. **#1** — Target has Frozen debuff; apply Fire damage
   - **Expected:** Frozen debuff removed

### TC-13.10.4.1 Hitbox Activation Timing

| # | Requirement |
|---|-------------|
| 1 | R-13.10.4   |
| 2 | R-13.10.4   |
| 3 | R-13.10.4   |

1. **#1** — Play attack animation; query hitbox at frame 10 (before window)
   - **Expected:** `hitbox.active = false`
2. **#2** — Query hitbox at frame 20 (inside window)
   - **Expected:** `hitbox.active = true`
3. **#3** — Query hitbox at frame 40 (after window)
   - **Expected:** `hitbox.active = false`

### TC-13.10.4.2 Hurtbox Region Multipliers

| # | Requirement |
|---|-------------|
| 1 | R-13.10.4   |
| 2 | R-13.10.4   |

1. **#1** — Strike head hurtbox (multiplier=2.0), base damage=50
   - **Expected:** Damage = 100
2. **#2** — Strike limb hurtbox (multiplier=0.75), base damage=100
   - **Expected:** Damage = 75

### TC-13.10.4.3 Directional Back Bonus

| # | Requirement |
|---|-------------|
| 1 | R-13.10.4   |

1. **#1** — Attacker behind target (back multiplier=1.5), base=100
   - **Expected:** Damage = 150

### TC-13.10.4.4 Multi-Hit Prevention

| # | Requirement |
|---|-------------|
| 1 | R-13.10.4   |

1. **#1** — Swing through same entity twice in one swing
   - **Expected:** `SwingTracker.hit_entities` contains entity once; 1 hit

### TC-13.10.4.5 Hit-Stop Time Dilation

| # | Requirement |
|---|-------------|
| 1 | R-13.10.4   |

1. **#1** — Land melee hit with `HitStopConfig { duration_ticks: 5, time_scale: 0.1 }`
   - **Expected:** Time scale = 0.1 for 5 ticks

### TC-13.10.5.1 Projectile Linear

| # | Requirement |
|---|-------------|
| 1 | R-13.10.5   |

1. **#1** — Fire `TrajectoryType::Linear` projectile, velocity=(0,0,10)
   - **Expected:** After 60 ticks: position.z = 600 (straight line)

### TC-13.10.5.2 Projectile Arced

| # | Requirement |
|---|-------------|
| 1 | R-13.10.5   |

1. **#1** — Fire `TrajectoryType::Arced`, gravity_scale=1.0
   - **Expected:** Position.y decreases parabolically

### TC-13.10.5.3 Projectile Homing

| # | Requirement |
|---|-------------|
| 1 | R-13.10.5   |

1. **#1** — Fire homing projectile at moving target
   - **Expected:** Heading converges toward target each tick

### TC-13.10.5.4 Projectile Spread

| # | Requirement |
|---|-------------|
| 1 | R-13.10.5   |

1. **#1** — Fire `Spread { pellet_count: 8, cone_angle: 15.0 }`
   - **Expected:** 8 pellets within 15-degree cone

### TC-13.10.5.5 Projectile CCD

| # | Requirement |
|---|-------------|
| 1 | R-13.10.5   |

1. **#1** — Fire fast projectile (speed=500) at thin wall (0.1m)
   - **Expected:** CCD detects collision; no tunneling

### TC-13.10.5.6 Projectile Effect Payload

| # | Requirement |
|---|-------------|
| 1 | R-13.10.5   |

1. **#1** — Projectile impacts target; payload = 30 damage
   - **Expected:** Target HP reduced by 30

### TC-13.10.5.7 Aim Assist Magnetism

| # | Requirement |
|---|-------------|
| 1 | R-13.10.5   |

1. **#1** — Aim within `magnetism_radius` of target with gamepad
   - **Expected:** Aim snaps to target center

## Integration Tests

### TC-13.10.1.I1 Ability Full Cycle

| # | Requirement            |
|---|------------------------|
| 1 | R-13.10.1, R-13.10.NF2 |

1. **#1** — Activate ability; observe animation, VFX, effect
   - **Expected:** All trigger within 1 frame of input

### TC-13.10.4.I1 Melee Full Pipeline

| # | Requirement            |
|---|------------------------|
| 1 | R-13.10.4, R-13.10.NF3 |

1. **#1** — Swing weapon; hitbox activates; spatial query; damage applies; hit reaction plays
   - **Expected:** All within 1 frame of anim event

### TC-13.10.5.I1 Ranged Full Pipeline

| # | Requirement |
|---|-------------|
| 1 | R-13.10.5   |

1. **#1** — Fire weapon; projectile flies; impacts target
   - **Expected:** Effect applies; surface VFX spawns

### TC-13.10.2.I1 Combo Into Ranged

| # | Requirement |
|---|-------------|
| 1 | R-13.10.2   |

1. **#1** — Execute combo chain; final step triggers projectile
   - **Expected:** Combo completes; projectile spawns correctly

### TC-13.10.NF1.I1 64 Concurrent Effects

| # | Requirement |
|---|-------------|
| 1 | R-13.10.NF1 |

1. **#1** — Apply 64 effects to one entity
   - **Expected:** All evaluate correctly within 0.1 ms

### TC-13.10.NF1.I2 40-Entity Raid

| # | Requirement |
|---|-------------|
| 1 | R-13.10.NF1 |

1. **#1** — 40 entities with 64 effects each
   - **Expected:** Total evaluation under 4 ms

### TC-13.10.6.I1 Lag Compensation 100ms

| # | Requirement |
|---|-------------|
| 1 | R-13.10.6   |

1. **#1** — Simulate 100 ms latency; client fires at target
   - **Expected:** Server resolves hit against historical snapshot at client fire time

### TC-13.10.1.I2 Shared Effect Component

| # | Requirement |
|---|-------------|
| 1 | R-13.10.1   |

1. **#1** — Two abilities share "Fire Damage" effect; activate each
   - **Expected:** Both produce identical damage values

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
