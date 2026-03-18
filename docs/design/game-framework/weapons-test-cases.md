# Weapon System Test Cases

Companion test cases for [weapons.md](weapons.md).

## Unit Tests

### TC-13.16.1.1 Semi Auto One Round

| # | Requirement |
|---|-------------|
| 1 | R-13.16.1   |

1. **#1** — Fire semi-auto weapon, single press
   - **Expected:** Exactly 1 round fired

### TC-13.16.1.2 Burst N Rounds

| # | Requirement |
|---|-------------|
| 1 | R-13.16.1   |

1. **#1** — Fire burst-mode (N=3), single press
   - **Expected:** Exactly 3 rounds with inter-round delay

### TC-13.16.1.3 Full Auto Continuous

| # | Requirement |
|---|-------------|
| 1 | R-13.16.1   |

1. **#1** — Hold fire on full-auto, 600 RPM, 1 second
   - **Expected:** ~10 rounds fired (600/60)

### TC-13.16.1.4 Fire Mode Toggle

| # | Requirement |
|---|-------------|
| 1 | R-13.16.1   |

1. **#1** — Toggle semi->burst->auto
   - **Expected:** Spread and recoil modifiers change per mode

### TC-13.16.1.5 Spin Up Gatling

| # | Requirement |
|---|-------------|
| 1 | R-13.16.1   |

1. **#1** — `spin_up_time_ms = 500`, press fire
   - **Expected:** No rounds for 500 ms, then full RPM

### TC-13.16.2a.1 Magazine Decrement

| # | Requirement |
|---|-------------|
| 1 | R-13.16.2a  |

1. **#1** — Fire 1 round, magazine had 30
   - **Expected:** `current_rounds == 29`

### TC-13.16.2a.2 Empty Magazine Blocks Fire

| # | Requirement |
|---|-------------|
| 1 | R-13.16.2a  |

1. **#1** — `current_rounds == 0`, press fire
   - **Expected:** No round fired

### TC-13.16.2a.3 Unlimited Ammo

| # | Requirement |
|---|-------------|
| 1 | R-13.16.2a  |

1. **#1** — `unlimited = true`, fire 100 rounds
   - **Expected:** `reserve_ammo` unchanged

### TC-13.16.2b.1 Tactical Reload Faster

| # | Requirement |
|---|-------------|
| 1 | R-13.16.2b  |

1. **#1** — Reload with 10 rounds remaining
   - **Expected:** Duration = `tactical_duration_ticks` (shorter)

### TC-13.16.2b.2 Empty Reload Slower

| # | Requirement |
|---|-------------|
| 1 | R-13.16.2b  |

1. **#1** — Reload with 0 rounds
   - **Expected:** Duration = `empty_duration_ticks` (longer)

### TC-13.16.2b.3 Sequential Reload Interrupt

| # | Requirement |
|---|-------------|
| 1 | R-13.16.2b  |

1. **#1** — Load 3 shells, press fire
   - **Expected:** Magazine = 3, weapon fires

### TC-13.16.2b.4 Reload Cancels Sprint

| # | Requirement |
|---|-------------|
| 1 | R-13.16.2b  |

1. **#1** — Sprinting, press reload
   - **Expected:** Sprint cancelled, reload begins

### TC-13.16.2c.1 Ammo Type Switch

| # | Requirement |
|---|-------------|
| 1 | R-13.16.2c  |

1. **#1** — Switch from Standard to AP ammo
   - **Expected:** `damage_modifier` and `armor_penetration` change

### TC-13.16.2c.2 Incendiary Status Effect

| # | Requirement |
|---|-------------|
| 1 | R-13.16.2c  |

1. **#1** — Fire incendiary round, hit target
   - **Expected:** Burn DoT `status_effect` applied

### TC-13.16.2c.3 Tracer Color Change

| # | Requirement |
|---|-------------|
| 1 | R-13.16.2c  |

1. **#1** — Switch ammo type
   - **Expected:** `tracer_color` matches new ammo config

### TC-13.16.3.1 Recoil Pattern Deterministic

| # | Requirement |
|---|-------------|
| 1 | R-13.16.3   |

1. **#1** — Fire 10 rounds
   - **Expected:** Aim offset matches `curve_points[0..10]` exactly

### TC-13.16.3.2 Recoil Recovery

| # | Requirement |
|---|-------------|
| 1 | R-13.16.3   |

1. **#1** — Fire 5 rounds, stop, wait 2 seconds
   - **Expected:** `accumulated_offset` converges toward zero

### TC-13.16.3.3 Spread ADS Tighter

| # | Requirement |
|---|-------------|
| 1 | R-13.16.3   |

1. **#1** — Compare spread ADS vs hip-fire
   - **Expected:** ADS spread = base * `ads_modifier` (< 1.0)

### TC-13.16.3.4 Spread Movement Wider

| # | Requirement |
|---|-------------|
| 1 | R-13.16.3   |

1. **#1** — Compare spread stationary vs moving
   - **Expected:** Moving spread = base * `move_modifier` (> 1.0)

### TC-13.16.3.5 First Shot Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-13.16.3   |

1. **#1** — Sniper with `first_shot_accuracy = 0.0`
   - **Expected:** First shot spread = 0 degrees

### TC-13.16.3.6 Crosshair Reflects Spread

| # | Requirement |
|---|-------------|
| 1 | R-13.16.3   |

1. **#1** — Fire sustained, read crosshair radius
   - **Expected:** Radius matches `current_spread`

### TC-13.16.4a.1 Bullet Drop Parabolic

| # | Requirement |
|---|-------------|
| 1 | R-13.16.4a  |

1. **#1** — Fire at 500 m, measure drop
   - **Expected:** Drop within 5% of expected parabola

### TC-13.16.4a.2 Drag Decelerates

| # | Requirement |
|---|-------------|
| 1 | R-13.16.4a  |

1. **#1** — Measure velocity at 100 m vs 500 m
   - **Expected:** Velocity at 500 m < velocity at 100 m

### TC-13.16.4a.3 Gravity Disable Straight

| # | Requirement |
|---|-------------|
| 1 | R-13.16.4a  |

1. **#1** — `gravity_enabled = false`, fire
   - **Expected:** Projectile travels in straight line (drop = 0)

### TC-13.16.4a.4 Travel Time Matches Velocity

| # | Requirement |
|---|-------------|
| 1 | R-13.16.4a  |

1. **#1** — Muzzle velocity 400 m/s, target at 100 m
   - **Expected:** Arrival time matches expected from velocity + drag

### TC-13.16.4b.1 Wind Lateral Deflection

| # | Requirement |
|---|-------------|
| 1 | R-13.16.4b  |

1. **#1** — Fire perpendicular to 10 m/s wind at 300 m
   - **Expected:** Lateral offset matches `wind_sensitivity * wind * distance`

### TC-13.16.4b.2 Wind Disable No Deflection

| # | Requirement |
|---|-------------|
| 1 | R-13.16.4b  |

1. **#1** — `wind_sensitivity = 0.0`
   - **Expected:** Zero lateral deflection

### TC-13.16.4c.1 Penetration Drywall

| # | Requirement |
|---|-------------|
| 1 | R-13.16.4c  |

1. **#1** — Fire through drywall (low density)
   - **Expected:** `SurfaceInteraction::Penetrated` with reduced velocity

### TC-13.16.4c.2 Penetration Concrete Stops

| # | Requirement |
|---|-------------|
| 1 | R-13.16.4c  |

1. **#1** — Fire at concrete (high density)
   - **Expected:** `SurfaceInteraction::Absorbed`

### TC-13.16.4c.3 Ricochet Shallow Angle

| # | Requirement |
|---|-------------|
| 1 | R-13.16.4c  |

1. **#1** — Fire at metal at 15-degree angle (below threshold)
   - **Expected:** `SurfaceInteraction::Ricocheted`

### TC-13.16.4c.4 Ricochet Steep Absorbs

| # | Requirement |
|---|-------------|
| 1 | R-13.16.4c  |

1. **#1** — Fire at metal at 80-degree angle (above threshold)
   - **Expected:** `SurfaceInteraction::Absorbed`

### TC-13.16.4d.1 Zeroing 300m On Target

| # | Requirement |
|---|-------------|
| 1 | R-13.16.4d  |

1. **#1** — Zero to 300 m, fire at 300 m target
   - **Expected:** Point-of-aim matches point-of-impact

### TC-13.16.4d.2 Zeroing Wrong Distance

| # | Requirement |
|---|-------------|
| 1 | R-13.16.4d  |

1. **#1** — Zero to 300 m, fire at 100 m target
   - **Expected:** Round impacts high of aim point

### TC-13.16.4d.3 Zeroing Ammo Shift

| # | Requirement |
|---|-------------|
| 1 | R-13.16.4d  |

1. **#1** — Switch ammo type (different muzzle velocity)
   - **Expected:** Effective zero distance shifts

### TC-13.16.5a.1 Attachment Equip Stat

| # | Requirement |
|---|-------------|
| 1 | R-13.16.5a  |

1. **#1** — Equip suppressor with `-20% SoundRadius`, `-10% Range`
   - **Expected:** Stats modified by configured amounts

### TC-13.16.5a.2 Attachment Category Filter

| # | Requirement |
|---|-------------|
| 1 | R-13.16.5a  |

1. **#1** — Equip Scope in Grip slot
   - **Expected:** Rejected; slot category mismatch

### TC-13.16.5b.1 Attachment Visual Snap

| # | Requirement |
|---|-------------|
| 1 | R-13.16.5b  |

1. **#1** — Equip red dot sight
   - **Expected:** Mesh appears at optic socket transform

### TC-13.16.5b.2 Attachment Unequip Visual

| # | Requirement |
|---|-------------|
| 1 | R-13.16.5b  |

1. **#1** — Remove attachment
   - **Expected:** Mesh disappears from weapon model

### TC-13.16.5b.3 Optic Reticle Render

| # | Requirement |
|---|-------------|
| 1 | R-13.16.5b  |

1. **#1** — Equip 4x scope
   - **Expected:** Reticle renders at 4x zoom

### TC-13.16.6a.1 Surface Tag Metal

| # | Requirement |
|---|-------------|
| 1 | R-13.16.6a  |

1. **#1** — Raycast hits metal surface
   - **Expected:** `SurfaceTypeTag::Metal` returned

### TC-13.16.6a.2 Surface Tag Terrain

| # | Requirement |
|---|-------------|
| 1 | R-13.16.6a  |

1. **#1** — Raycast hits terrain with dirt dominant splatmap
   - **Expected:** `SurfaceTypeTag::Dirt` returned

### TC-13.16.6b.1 Impact VFX Metal Sparks

| # | Requirement |
|---|-------------|
| 1 | R-13.16.6b  |

1. **#1** — Shoot metal surface
   - **Expected:** Sparks VFX spawned from response table

### TC-13.16.6c.1 Impact Audio Wood Thud

| # | Requirement |
|---|-------------|
| 1 | R-13.16.6c  |

1. **#1** — Shoot wood surface
   - **Expected:** Thud sound plays from sound pool

### TC-13.16.6c.2 Impact Audio Variation

| # | Requirement |
|---|-------------|
| 1 | R-13.16.6c  |

1. **#1** — Shoot same surface 10 times
   - **Expected:** At least 2 distinct sounds from pool

### TC-13.16.6d.1 Impact Decal Placement

| # | Requirement |
|---|-------------|
| 1 | R-13.16.6d  |

1. **#1** — Shoot surface
   - **Expected:** Decal placed at impact point, oriented along normal

### TC-13.16.6d.2 Decal Max Count Eviction

| # | Requirement |
|---|-------------|
| 1 | R-13.16.6d  |

1. **#1** — Exceed `max_count` decals on surface
   - **Expected:** Oldest decal removed

## Integration Tests

### TC-13.16.1.I1 Fire Full Pipeline

| # | Requirement |
|---|-------------|
| 1 | R-13.16.1   |

1. **#1** — Press fire
   - **Expected:** Ammo consumed, recoil applied, projectile spawned, muzzle VFX plays -- within 1
     frame

### TC-13.16.4a.I1 Projectile Impact Full

| # | Requirement |
|---|-------------|
| 1 | R-13.16.4a  |

1. **#1** — Projectile hits surface
   - **Expected:** Damage applied, VFX spawns, audio plays, decal placed

### TC-13.16.4c.I1 Penetration Chain

| # | Requirement |
|---|-------------|
| 1 | R-13.16.4c  |

1. **#1** — Fire through drywall, hit target behind
   - **Expected:** Target takes damage with reduced energy

### TC-13.16.4c.I2 Ricochet Kills

| # | Requirement |
|---|-------------|
| 1 | R-13.16.4c  |

1. **#1** — Ricochet off metal into target
   - **Expected:** Target takes damage with reduced energy

### TC-13.16.5.I1 Attachment Full Cycle

| # | Requirement |
|---|-------------|
| 1 | R-13.16.5a  |

1. **#1** — Equip in UI, verify stat, verify mesh, verify UI comparison
   - **Expected:** All 3 checks pass

### TC-NFR-13.16.1.I1 256 Projectiles Perf

| # | Requirement |
|---|-------------|
| 1 | NFR-13.16.1 |

1. **#1** — 256 simultaneous projectiles with full ballistics
   - **Expected:** Under 1 ms per physics tick

### TC-NFR-13.16.2.I1 Weapon Feedback Latency

| # | Requirement |
|---|-------------|
| 1 | NFR-13.16.2 |

1. **#1** — Measure input-to-muzzle-flash over 100 fires
   - **Expected:** All under 16 ms

## Benchmarks

### TC-NFR-13.16.1.B1 Ballistic Simulation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 256 projectiles per tick | Simulation time | < 1 ms | NFR-13.16.1 |

### TC-NFR-13.16.1.B2 Per Projectile Penetration

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single penetration/ricochet eval | Evaluation time | < 0.01 ms | NFR-13.16.1 |

### TC-NFR-13.16.2.B1 Input To Muzzle Flash

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Fire input to VFX display | Latency | < 16 ms | NFR-13.16.2 |

### TC-13.16.2b.B1 Reload State Transition

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | State machine transition | Transition time | < 0.01 ms | R-13.16.2b |

### TC-13.16.3.B1 Recoil Pattern Lookup

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Pattern index lookup + apply | Per-round time | < 0.001 ms | R-13.16.3 |

### TC-13.16.3.B2 Spread Recalculation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full spread recalc (movement + posture + ADS) | Recalc time | < 0.001 ms | R-13.16.3 |

### TC-13.16.5a.B1 Attachment Stat Aggregation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Aggregate 6 attachment stat modifiers | Aggregation time | < 0.05 ms | R-13.16.5a |

### TC-13.16.6a.B1 Surface Type Resolve

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Terrain splatmap sample at impact | Resolve time | < 0.01 ms | R-13.16.6a |

### TC-13.16.6b.B1 Impact Response Dispatch

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | VFX + audio + decal spawn | Dispatch time | < 0.1 ms | R-13.16.6b |
