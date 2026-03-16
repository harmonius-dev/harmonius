# Weapon System Test Cases

Companion test cases for [weapons.md](weapons.md).

## Unit Tests

### TC-13.16.1.1 Semi Auto One Round

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fire semi-auto weapon, single press | Exactly 1 round fired | R-13.16.1 |

### TC-13.16.1.2 Burst N Rounds

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fire burst-mode (N=3), single press | Exactly 3 rounds with inter-round delay | R-13.16.1 |

### TC-13.16.1.3 Full Auto Continuous

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hold fire on full-auto, 600 RPM, 1 second | ~10 rounds fired (600/60) | R-13.16.1 |

### TC-13.16.1.4 Fire Mode Toggle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Toggle semi->burst->auto | Spread and recoil modifiers change per mode | R-13.16.1 |

### TC-13.16.1.5 Spin Up Gatling

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `spin_up_time_ms = 500`, press fire | No rounds for 500 ms, then full RPM | R-13.16.1 |

### TC-13.16.2a.1 Magazine Decrement

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fire 1 round, magazine had 30 | `current_rounds == 29` | R-13.16.2a |

### TC-13.16.2a.2 Empty Magazine Blocks Fire

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `current_rounds == 0`, press fire | No round fired | R-13.16.2a |

### TC-13.16.2a.3 Unlimited Ammo

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `unlimited = true`, fire 100 rounds | `reserve_ammo` unchanged | R-13.16.2a |

### TC-13.16.2b.1 Tactical Reload Faster

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Reload with 10 rounds remaining | Duration = `tactical_duration_ticks` (shorter) | R-13.16.2b |

### TC-13.16.2b.2 Empty Reload Slower

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Reload with 0 rounds | Duration = `empty_duration_ticks` (longer) | R-13.16.2b |

### TC-13.16.2b.3 Sequential Reload Interrupt

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load 3 shells, press fire | Magazine = 3, weapon fires | R-13.16.2b |

### TC-13.16.2b.4 Reload Cancels Sprint

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sprinting, press reload | Sprint cancelled, reload begins | R-13.16.2b |

### TC-13.16.2c.1 Ammo Type Switch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Switch from Standard to AP ammo | `damage_modifier` and `armor_penetration` change | R-13.16.2c |

### TC-13.16.2c.2 Incendiary Status Effect

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fire incendiary round, hit target | Burn DoT `status_effect` applied | R-13.16.2c |

### TC-13.16.2c.3 Tracer Color Change

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Switch ammo type | `tracer_color` matches new ammo config | R-13.16.2c |

### TC-13.16.3.1 Recoil Pattern Deterministic

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fire 10 rounds | Aim offset matches `curve_points[0..10]` exactly | R-13.16.3 |

### TC-13.16.3.2 Recoil Recovery

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fire 5 rounds, stop, wait 2 seconds | `accumulated_offset` converges toward zero | R-13.16.3 |

### TC-13.16.3.3 Spread ADS Tighter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compare spread ADS vs hip-fire | ADS spread = base * `ads_modifier` (< 1.0) | R-13.16.3 |

### TC-13.16.3.4 Spread Movement Wider

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compare spread stationary vs moving | Moving spread = base * `move_modifier` (> 1.0) | R-13.16.3 |

### TC-13.16.3.5 First Shot Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sniper with `first_shot_accuracy = 0.0` | First shot spread = 0 degrees | R-13.16.3 |

### TC-13.16.3.6 Crosshair Reflects Spread

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fire sustained, read crosshair radius | Radius matches `current_spread` | R-13.16.3 |

### TC-13.16.4a.1 Bullet Drop Parabolic

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fire at 500 m, measure drop | Drop within 5% of expected parabola | R-13.16.4a |

### TC-13.16.4a.2 Drag Decelerates

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Measure velocity at 100 m vs 500 m | Velocity at 500 m < velocity at 100 m | R-13.16.4a |

### TC-13.16.4a.3 Gravity Disable Straight

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `gravity_enabled = false`, fire | Projectile travels in straight line (drop = 0) | R-13.16.4a |

### TC-13.16.4a.4 Travel Time Matches Velocity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Muzzle velocity 400 m/s, target at 100 m | Arrival time matches expected from velocity + drag | R-13.16.4a |

### TC-13.16.4b.1 Wind Lateral Deflection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fire perpendicular to 10 m/s wind at 300 m | Lateral offset matches `wind_sensitivity * wind * distance` | R-13.16.4b |

### TC-13.16.4b.2 Wind Disable No Deflection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `wind_sensitivity = 0.0` | Zero lateral deflection | R-13.16.4b |

### TC-13.16.4c.1 Penetration Drywall

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fire through drywall (low density) | `SurfaceInteraction::Penetrated` with reduced velocity | R-13.16.4c |

### TC-13.16.4c.2 Penetration Concrete Stops

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fire at concrete (high density) | `SurfaceInteraction::Absorbed` | R-13.16.4c |

### TC-13.16.4c.3 Ricochet Shallow Angle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fire at metal at 15-degree angle (below threshold) | `SurfaceInteraction::Ricocheted` | R-13.16.4c |

### TC-13.16.4c.4 Ricochet Steep Absorbs

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fire at metal at 80-degree angle (above threshold) | `SurfaceInteraction::Absorbed` | R-13.16.4c |

### TC-13.16.4d.1 Zeroing 300m On Target

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Zero to 300 m, fire at 300 m target | Point-of-aim matches point-of-impact | R-13.16.4d |

### TC-13.16.4d.2 Zeroing Wrong Distance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Zero to 300 m, fire at 100 m target | Round impacts high of aim point | R-13.16.4d |

### TC-13.16.4d.3 Zeroing Ammo Shift

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Switch ammo type (different muzzle velocity) | Effective zero distance shifts | R-13.16.4d |

### TC-13.16.5a.1 Attachment Equip Stat

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Equip suppressor with `-20% SoundRadius`, `-10% Range` | Stats modified by configured amounts | R-13.16.5a |

### TC-13.16.5a.2 Attachment Category Filter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Equip Scope in Grip slot | Rejected; slot category mismatch | R-13.16.5a |

### TC-13.16.5b.1 Attachment Visual Snap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Equip red dot sight | Mesh appears at optic socket transform | R-13.16.5b |

### TC-13.16.5b.2 Attachment Unequip Visual

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Remove attachment | Mesh disappears from weapon model | R-13.16.5b |

### TC-13.16.5b.3 Optic Reticle Render

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Equip 4x scope | Reticle renders at 4x zoom | R-13.16.5b |

### TC-13.16.6a.1 Surface Tag Metal

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Raycast hits metal surface | `SurfaceTypeTag::Metal` returned | R-13.16.6a |

### TC-13.16.6a.2 Surface Tag Terrain

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Raycast hits terrain with dirt dominant splatmap | `SurfaceTypeTag::Dirt` returned | R-13.16.6a |

### TC-13.16.6b.1 Impact VFX Metal Sparks

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Shoot metal surface | Sparks VFX spawned from response table | R-13.16.6b |

### TC-13.16.6c.1 Impact Audio Wood Thud

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Shoot wood surface | Thud sound plays from sound pool | R-13.16.6c |

### TC-13.16.6c.2 Impact Audio Variation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Shoot same surface 10 times | At least 2 distinct sounds from pool | R-13.16.6c |

### TC-13.16.6d.1 Impact Decal Placement

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Shoot surface | Decal placed at impact point, oriented along normal | R-13.16.6d |

### TC-13.16.6d.2 Decal Max Count Eviction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Exceed `max_count` decals on surface | Oldest decal removed | R-13.16.6d |

## Integration Tests

### TC-13.16.1.I1 Fire Full Pipeline

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Press fire | Ammo consumed, recoil applied, projectile spawned, muzzle VFX plays -- within 1 frame | R-13.16.1 |

### TC-13.16.4a.I1 Projectile Impact Full

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Projectile hits surface | Damage applied, VFX spawns, audio plays, decal placed | R-13.16.4a |

### TC-13.16.4c.I1 Penetration Chain

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fire through drywall, hit target behind | Target takes damage with reduced energy | R-13.16.4c |

### TC-13.16.4c.I2 Ricochet Kills

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Ricochet off metal into target | Target takes damage with reduced energy | R-13.16.4c |

### TC-13.16.5.I1 Attachment Full Cycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Equip in UI, verify stat, verify mesh, verify UI comparison | All 3 checks pass | R-13.16.5a |

### TC-NFR-13.16.1.I1 256 Projectiles Perf

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 256 simultaneous projectiles with full ballistics | Under 1 ms per physics tick | NFR-13.16.1 |

### TC-NFR-13.16.2.I1 Weapon Feedback Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Measure input-to-muzzle-flash over 100 fires | All under 16 ms | NFR-13.16.2 |

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
