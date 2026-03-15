# R-13.16 -- Weapon Systems Requirements

## R-13.16.1 Weapon Fire Mode System

The engine **SHALL** support per-weapon configurable fire modes (semi-automatic, burst, full-
automatic) with per-mode rounds per activation, inter-round delay, spread modifier, and recoil
modifier, toggled via input action and driven by the ability activation system.

- **Derived from:** [F-13.16.1](../../features/game-framework/weapon-system.md)
- **Rationale:** Data-driven fire modes on ECS weapon entities let designers tune weapon feel
  without code changes while supporting diverse weapon archetypes from pistols to miniguns.
- **Verification:** Configure a weapon with all three fire modes. Verify semi-automatic fires one
  round per press, burst fires the configured N rounds with inter-round delay, and full- automatic
  fires continuously while held. Toggle modes via input and confirm the spread and recoil modifiers
  change to match each mode's configured values.

## R-13.16.2a Magazine and Ammo Management

The engine **SHALL** track per-weapon magazine capacity, current round count, and reserve ammo pool,
with ammo consumed from inventory or an unlimited pool per game mode, and magazine and reserve
counts displayed in the HUD.

- **Derived from:** [F-13.16.2a](../../features/game-framework/weapon-system.md)
- **Rationale:** Magazine tracking is the foundation for all reload and ammo systems, and HUD
  display keeps the player informed of ammunition state.
- **Verification:** Fire a weapon and verify the round count decrements. Verify the HUD displays
  current magazine and reserve counts. Empty the magazine and confirm fire input is suppressed.
  Configure unlimited ammo mode and verify reserve never depletes.

## R-13.16.2b Reload Mechanics

The engine **SHALL** implement tactical reload (faster when magazine is not empty), empty reload
(slower when magazine is depleted), and sequential reload (interruptible shell-by-shell), where
reload cancels sprint and duration is configurable per weapon.

- **Derived from:** [F-13.16.2b](../../features/game-framework/weapon-system.md)
- **Rationale:** Reload variants add tactical depth — rewarding players for reloading before empty
  and allowing shotgun users to interrupt reloads for emergency shots.
- **Verification:** Fire a weapon partially and reload. Verify tactical reload completes faster than
  empty reload. Load a shotgun shell-by-shell, interrupt mid-reload by firing, and confirm the
  weapon fires with the partially loaded magazine. Verify reload cancels sprint.

## R-13.16.2c Ammo Type System

The engine **SHALL** support per-weapon swappable ammo types (standard, armor-piercing, incendiary,
hollow-point) with per-type damage modifier, armor penetration, status effect, and visual
properties, switchable at runtime via input action.

- **Derived from:** [F-13.16.2c](../../features/game-framework/weapon-system.md)
- **Rationale:** Ammo type switching enables tactical adaptation to different enemy types without
  requiring weapon swaps.
- **Verification:** Switch ammo type via input and verify the active ammo changes. Fire
  armor-piercing rounds at an armored target and verify increased penetration. Fire incendiary
  rounds and verify burn status effect applies. Confirm tracer color changes per ammo type.

## R-13.16.3 Recoil Pattern and Weapon Spread

The engine **SHALL** apply per-weapon recoil patterns defined as 2D data curves that shift aim
direction over sustained fire, with spread widening from movement, hip-fire, and sustained fire, and
tightening from ADS, crouching, and burst discipline, reflected dynamically in the crosshair.

- **Derived from:** [F-13.16.3](../../features/game-framework/weapon-system.md)
- **Rationale:** Learnable recoil patterns and context-sensitive spread reward skill mastery while
  allowing designers to differentiate weapon handling characteristics through data.
- **Verification:** Fire a weapon in sustained full-auto and record aim offset per frame. Verify the
  offset follows the authored 2D recoil curve. Measure spread cone while stationary ADS versus
  sprinting hip-fire and confirm the stationary ADS cone is smaller by the configured modifier
  ratio. Verify the crosshair radius matches the current spread value.

## R-13.16.4a Projectile Drop and Travel Time

The engine **SHALL** simulate gravity-based bullet drop and finite travel time with per-ammo-type
mass, muzzle velocity, and drag coefficient, following a parabolic trajectory, with gravity and drag
independently disableable for arcade modes.

- **Derived from:** [F-13.16.4a](../../features/game-framework/weapon-system.md)
- **Rationale:** Gravity drop and travel time are the foundational ballistic physics that
  differentiate weapon archetypes at range.
- **Verification:** Fire a round at a target 500 m away and verify bullet drop matches the expected
  parabolic trajectory within 5% tolerance. Disable gravity and confirm the projectile travels in a
  straight line. Measure travel time and verify it matches muzzle velocity minus drag over the
  distance.

## R-13.16.4b Wind Deflection

The engine **SHALL** apply lateral wind deflection to in-flight projectiles based on a wind vector
from the weather system, with per-ammo wind sensitivity and independent disableability.

- **Derived from:** [F-13.16.4b](../../features/game-framework/weapon-system.md)
- **Rationale:** Wind deflection adds environmental skill expression to long-range shooting without
  affecting close-range combat.
- **Verification:** Fire at a distant target with wind active and verify lateral deviation matches
  the wind vector and ammo sensitivity. Disable wind and confirm no lateral deflection. Change wind
  direction mid-flight and verify the projectile adjusts.

## R-13.16.4c Surface Penetration and Ricochet

The engine **SHALL** simulate surface penetration with material-density-based energy loss and
ricochet at shallow angles with probability based on impact angle and material hardness, both
independently disableable.

- **Derived from:** [F-13.16.4c](../../features/game-framework/weapon-system.md)
- **Rationale:** Penetration and ricochet make cover material tactically meaningful and create
  emergent combat scenarios.
- **Verification:** Fire through a drywall surface and confirm the projectile passes through with
  reduced energy. Fire at concrete and confirm the projectile stops. Fire at a metal surface at a
  shallow angle and verify ricochet occurs. Disable penetration and confirm no pass-through.

## R-13.16.4d Weapon Zeroing

The engine **SHALL** support adjustable sight zeroing in discrete distance steps that offset the aim
point to compensate for bullet drop, displayed in the scope UI, and interacting with ammo type
ballistic profiles.

- **Derived from:** [F-13.16.4d](../../features/game-framework/weapon-system.md)
- **Rationale:** Zeroing enables accurate first shots at known distances without manual holdover,
  critical for sniper-focused gameplay.
- **Verification:** Zero a scoped weapon to 300 m and fire at a 300 m target. Verify point of aim
  matches point of impact. Fire at 100 m with 300 m zero and verify the round impacts high. Switch
  ammo type and verify the effective zero shifts due to different ballistics.

## R-13.16.5a Attachment Slot Model

The engine **SHALL** provide a data-driven slot system where weapons define named attachment slots
with category filters, and attachments modify weapon stats additively or multiplicatively as
configured, with slots and stats defined in gameplay databases.

- **Derived from:** [F-13.16.5a](../../features/game-framework/weapon-system.md)
- **Rationale:** Data-driven slot definitions and stat modifiers let designers extend weapon
  customization without code changes.
- **Verification:** Define a weapon with optic, muzzle, and grip slots. Equip a suppressor to the
  muzzle slot and verify sound and range decrease by configured amounts. Equip an extended magazine
  and verify capacity increases. Attempt to equip an optic in the grip slot and verify the category
  filter rejects it.

## R-13.16.5b Attachment Visual Integration

The engine **SHALL** render equipped attachments at named socket transforms on weapon meshes, with
optic attachments rendering scope reticles and barrel attachments replacing the muzzle flash VFX
point.

- **Derived from:** [F-13.16.5b](../../features/game-framework/weapon-system.md)
- **Rationale:** Visual attachment representation gives customization tangible in-game presence and
  reinforces player investment in weapon modification.
- **Verification:** Equip a suppressor, optic, and grip. Verify each attachment mesh appears at its
  socket transform. Verify the optic renders its reticle at the configured zoom level. Remove an
  attachment and confirm the mesh disappears immediately.

## R-13.16.5c Attachment Customization UI

The engine **SHALL** provide a weapon customization screen with a 3D weapon preview, interactive
slot indicators, filtered attachment lists from inventory, and before/after stat comparison on
hover.

- **Derived from:** [F-13.16.5c](../../features/game-framework/weapon-system.md)
- **Rationale:** A visual customization UI lets players make informed attachment decisions by
  previewing both visual and stat effects before committing.
- **Verification:** Open the customization UI and verify the 3D weapon preview renders with current
  attachments. Select a slot and verify only compatible attachments appear. Hover an attachment and
  verify stat comparison shows before/after values. Equip and confirm the 3D preview updates in real
  time.

## R-13.16.6a Surface Type Tag System

The engine **SHALL** assign a surface type tag to all physics materials, resolve terrain surface
type from the splatmap's dominant material weight, and include the surface type in all physics query
results.

- **Derived from:** [F-13.16.6a](../../features/game-framework/weapon-system.md)
- **Rationale:** Surface type tagging is the foundation that enables all downstream impact response
  systems to dispatch context-appropriate feedback.
- **Verification:** Perform a raycast against metal, wood, and concrete surfaces. Verify each
  returns the correct surface type tag. Raycast terrain and confirm the tag matches the dominant
  splatmap material at the hit point. Add a custom surface type and verify it propagates through
  physics queries.

## R-13.16.6b Impact VFX Response

The engine **SHALL** spawn particle effects on surface impact by looking up the VFX response table
for the surface type, with configurable particle asset, spawn count, scale, and lifetime per surface
type.

- **Derived from:** [F-13.16.6b](../../features/game-framework/weapon-system.md)
- **Rationale:** Per-surface VFX provides immediate visual feedback that communicates impact context
  to the player.
- **Verification:** Fire a projectile at metal, wood, and concrete. Verify sparks spawn for metal,
  splinters for wood, and dust for concrete as defined in the VFX response table. Verify particle
  count and lifetime match configured values.

## R-13.16.6c Impact Audio Response

The engine **SHALL** play spatial audio on surface impact by looking up the audio response table for
the surface type, with random variation from a sound pool and volume/pitch scaling with impact
force.

- **Derived from:** [F-13.16.6c](../../features/game-framework/weapon-system.md)
- **Rationale:** Per-surface audio reinforces the physical context of impacts and prevents
  repetitive sound when using random variation pools.
- **Verification:** Fire at metal, wood, and concrete. Verify each produces the correct impact
  sound. Fire multiple rounds at the same surface and verify random variation produces different
  sounds from the pool. Verify louder impacts produce louder audio.

## R-13.16.6d Impact Decal Response

The engine **SHALL** place decals on surface impact by looking up the decal response table for the
surface type, projected onto the surface at the impact point and oriented along the surface normal,
with configurable lifetime and maximum count.

- **Derived from:** [F-13.16.6d](../../features/game-framework/weapon-system.md)
- **Rationale:** Impact decals leave visible environmental traces that communicate combat history
  and enhance immersion.
- **Verification:** Fire at metal, wood, and flesh surfaces. Verify bullet hole, splinter hole, and
  blood spatter decals appear respectively. Verify decals are oriented along the surface normal.
  Exceed the maximum decal count and verify the oldest decals are removed.

## Non-Functional Requirements

### NFR-13.16.1 Ballistic Simulation Performance

The advanced ballistic simulation **SHALL** process up to 256 simultaneous in-flight projectiles
within 1ms per physics tick. Per-projectile penetration and ricochet calculations **SHALL** complete
within 0.01ms each.

- **Rationale:** High fire-rate weapons and multiplayer scenarios can produce large numbers of
  simultaneous projectiles that must be simulated without frame rate impact.
- **Verification:** Spawn 256 simultaneous projectiles with gravity, drag, and penetration enabled.
  Measure total ballistic simulation time per physics tick. Verify it stays under 1ms.

### NFR-13.16.2 Weapon Feedback Latency

Time from trigger input to visual muzzle flash, audio report, and recoil camera kick **SHALL** be
under 16ms (1 frame at 60 fps). Reload animations and fire mode toggle **SHALL** respond within 1
frame of input.

- **Rationale:** Weapon responsiveness is the most critical feel metric in shooter games. Any
  perceptible input-to-feedback latency degrades player experience.
- **Verification:** Measure time from fire input event to muzzle flash render and audio playback
  start. Verify total latency is under 16ms. Measure fire mode toggle response time and verify it is
  within 1 frame.
