# User Stories -- Weapon Systems (13.16)

## Weapon Fire Modes (F-13.16.1)

## US-13.16.1.1
**As a** player (P-23), **I want** to toggle between semi-automatic, burst, and full-automatic fire
modes on weapons that support them, **so that** I can adapt my firing to the situation.

## US-13.16.1.2
**As a** player (P-23), **I want** each fire mode to have distinct rounds per activation,
inter-round delay, and spread, **so that** modes feel mechanically different.

## US-13.16.1.3
**As a** gameplay director (P-3), **I want** fire rate defined as rounds per minute with
configurable spin-up for gatling weapons, **so that** weapon archetypes feel unique.

## US-13.16.1.4
**As a** designer (P-5), **I want** to configure fire mode parameters per weapon as data assets,
**so that** I can iterate on weapon feel without code changes.

## US-13.16.1.5
**As a** tester (P-27), **I want** to verify that burst mode fires exactly N rounds per trigger
press, **so that** burst count is accurate.

## Magazine and Ammo Management (F-13.16.2a)

## US-13.16.2a.1
**As a** player (P-23), **I want** each weapon to track magazine rounds and reserve ammo, **so
that** I must manage my ammunition supply.

## US-13.16.2a.2
**As a** player (P-23), **I want** magazine and reserve counts displayed in the HUD, **so that** I
know my ammo status at a glance.

## US-13.16.2a.3
**As a** player (P-23), **I want** fire input suppressed when the magazine is empty, **so that** I
must reload before firing again.

## US-13.16.2a.4
**As a** designer (P-5), **I want** to configure magazine capacity, reserve pool, and shared vs.
per-weapon reserve per weapon type, **so that** ammo economy varies by archetype.

## US-13.16.2a.5
**As a** tester (P-27), **I want** to verify that ammo is consumed from inventory when not in
unlimited mode, **so that** ammo tracking is correct.

## Reload Mechanics (F-13.16.2b)

## US-13.16.2b.1
**As a** player (P-23), **I want** tactical reload to be faster than empty reload, **so that** I am
rewarded for reloading before the magazine empties.

## US-13.16.2b.2
**As a** player (P-23), **I want** sequential reload to be interruptible by fire input, **so that**
I can shoot a partially loaded shotgun in emergencies.

## US-13.16.2b.3
**As a** player (P-23), **I want** reloading to cancel sprint, **so that** reload has a movement
cost.

## US-13.16.2b.4
**As a** designer (P-5), **I want** to configure reload duration, animation, and sound per weapon as
data assets, **so that** reload behavior is tunable per weapon.

## US-13.16.2b.5
**As a** tester (P-27), **I want** to verify that interrupting a sequential reload preserves the
rounds already loaded, **so that** partial reload works correctly.

## Ammo Types (F-13.16.2c)

## US-13.16.2c.1
**As a** player (P-23), **I want** to switch ammo types at runtime via an input action, **so that**
I can adapt to different enemy resistances mid-combat.

## US-13.16.2c.2
**As a** player (P-23), **I want** armor-piercing rounds to ignore a percentage of target armor,
**so that** I can counter heavily armored enemies.

## US-13.16.2c.3
**As a** player (P-23), **I want** incendiary rounds to apply burn damage over time, **so that** I
can use fire against vulnerable targets.

## US-13.16.2c.4
**As a** designer (P-5), **I want** to define ammo types with damage modifier, armor penetration,
status effect, tracer color, and muzzle VFX in gameplay databases, **so that** I can create diverse
ammunition without code.

## US-13.16.2c.5
**As a** gameplay director (P-3), **I want** ammo types to create tactical depth by forcing loadout
decisions, **so that** weapon customization has strategic significance.

## US-13.16.2c.6
**As a** tester (P-27), **I want** to verify that switching ammo types updates the tracer color and
damage modifier immediately, **so that** the swap applies correctly.

## Recoil and Spread (F-13.16.3)

## US-13.16.3.1
**As a** player (P-23), **I want** sustained fire to kick my aim upward and sideways following a
pattern, with recovery when I stop, **so that** recoil control is a learnable skill.

## US-13.16.3.2
**As a** player (P-23), **I want** the crosshair to dynamically reflect current spread radius, **so
that** I can visually judge my accuracy.

## US-13.16.3.3
**As a** player (P-23), **I want** spread to widen with movement, hip-fire, and sustained fire, and
tighten with ADS and crouching, **so that** positioning and posture improve accuracy.

## US-13.16.3.4
**As a** designer (P-5), **I want** to author recoil patterns as 2D curves in the visual editor per
weapon, **so that** I can craft unique weapon feel without code.

## US-13.16.3.5
**As a** gameplay director (P-3), **I want** first-shot accuracy configurable per weapon type (100%
for snipers, imprecise for SMGs), **so that** weapon identity is clear from the first shot.

## US-13.16.3.6
**As a** tester (P-27), **I want** to verify that the recoil pattern repeats identically on each
spray, **so that** recoil is deterministic and learnable.

## Projectile Ballistics (F-13.16.4a)

## US-13.16.4a.1
**As a** player (P-23), **I want** projectiles to follow a parabolic trajectory with gravity drop
and travel time, **so that** distant targets require leading and elevation.

## US-13.16.4a.2
**As a** player (P-23), **I want** drag to decelerate projectiles over distance, **so that**
effective range has mechanical meaning.

## US-13.16.4a.3
**As a** designer (P-5), **I want** to configure mass, muzzle velocity, and drag per ammo type, **so
that** snipers and shotguns feel physically distinct at range.

## US-13.16.4a.4
**As a** designer (P-5), **I want** to disable gravity and drag independently for arcade modes, **so
that** the ballistics system supports both realistic and arcade gameplay.

## US-13.16.4a.5
**As a** tester (P-27), **I want** to verify that a projectile with zero drag reaches the target at
the expected time based on muzzle velocity, **so that** travel time is accurate.

## Wind Deflection (F-13.16.4b)

## US-13.16.4b.1
**As a** player (P-23), **I want** wind to deflect my shots laterally, **so that** long-range
marksmanship requires reading environmental conditions.

## US-13.16.4b.2
**As a** player (P-23), **I want** wind indicators in the sniper scope UI, **so that** I can
compensate for wind at long range.

## US-13.16.4b.3
**As a** designer (P-5), **I want** to configure per-ammo wind sensitivity and enable wind per zone,
**so that** I can create varying wind conditions across the map.

## US-13.16.4b.4
**As a** designer (P-5), **I want** wind deflection to be disableable independently for arcade
modes, **so that** casual modes skip the complexity.

## US-13.16.4b.5
**As a** tester (P-27), **I want** to verify that a projectile fired perpendicular to wind deflects
by the expected distance at a given range, **so that** deflection math is correct.

## Surface Penetration and Ricochet (F-13.16.4c)

## US-13.16.4c.1
**As a** player (P-23), **I want** bullets to penetrate thin materials like drywall with energy
loss, **so that** cover material matters tactically.

## US-13.16.4c.2
**As a** player (P-23), **I want** bullets to ricochet off hard surfaces at shallow angles, **so
that** metal walls deflect shots unpredictably.

## US-13.16.4c.3
**As a** designer (P-5), **I want** to configure penetration and ricochet thresholds per physics
material, **so that** each surface type responds differently.

## US-13.16.4c.4
**As a** gameplay director (P-3), **I want** penetration and ricochet systems to be independently
disableable, **so that** I can control tactical complexity per game mode.

## US-13.16.4c.5
**As a** tester (P-27), **I want** to verify that a bullet hitting concrete at a steep angle stops
rather than ricocheting, **so that** angle thresholds are enforced.

## Weapon Zeroing (F-13.16.4d)

## US-13.16.4d.1
**As a** player (P-23), **I want** to adjust my scope zeroing in discrete distance steps, **so
that** point of aim matches point of impact at the zeroed range.

## US-13.16.4d.2
**As a** player (P-23), **I want** the current zero distance displayed in my scope UI overlay, **so
that** I know my current zeroing at a glance.

## US-13.16.4d.3
**As a** player (P-23), **I want** switching ammo types to shift the effective zero, **so that** I
must re-zero when changing ammunition.

## US-13.16.4d.4
**As a** designer (P-5), **I want** zeroing available only on weapons with optic attachments, **so
that** iron-sight weapons cannot be zeroed.

## US-13.16.4d.5
**As a** tester (P-27), **I want** to verify that a weapon zeroed to 200 m hits dead center at 200 m
with the configured ammo type, **so that** zeroing calibration is accurate.

## Attachment Slot Model (F-13.16.5a)

## US-13.16.5a.1
**As a** player (P-23), **I want** weapons to have named attachment slots (optic, barrel, muzzle,
grip, stock, magazine, rail), **so that** I can customize my weapon.

## US-13.16.5a.2
**As a** player (P-23), **I want** each slot to filter compatible attachments by category, **so
that** only valid attachments can be equipped.

## US-13.16.5a.3
**As a** designer (P-5), **I want** to define slot categories, attachment stats, and modifier modes
(additive or multiplicative) in gameplay databases, **so that** weapon customization is extensible
without code.

## US-13.16.5a.4
**As a** gameplay director (P-3), **I want** attachment stat modifiers to create meaningful build
choices, **so that** customization affects gameplay strategy.

## US-13.16.5a.5
**As a** tester (P-27), **I want** to verify that an optic attachment cannot be placed in a grip
slot, **so that** slot category filtering is enforced.

## Attachment Visuals (F-13.16.5b)

## US-13.16.5b.1
**As a** player (P-23), **I want** equipped attachments to appear on my weapon model at the correct
socket positions, **so that** customization has visible impact.

## US-13.16.5b.2
**As a** player (P-23), **I want** optic attachments to render scope reticles with configurable
zoom, **so that** optics feel functional in ADS.

## US-13.16.5b.3
**As a** designer (P-5), **I want** to author attachment meshes that snap to named socket transforms
on the weapon, **so that** visual integration is consistent.

## US-13.16.5b.4
**As a** tester (P-27), **I want** to verify that equipping and unequipping an attachment updates
the weapon mesh immediately, **so that** visual changes are instant.

## Attachment Customization UI (F-13.16.5c)

## US-13.16.5c.1
**As a** player (P-23), **I want** a dedicated weapon customization screen with a 3D weapon preview
and interactive slot indicators, **so that** I can browse and compare attachments.

## US-13.16.5c.2
**As a** player (P-23), **I want** stat comparison shown when hovering an attachment, **so that** I
can evaluate before equipping.

## US-13.16.5c.3
**As a** player (P-23), **I want** the 3D preview to update in real time when I equip or remove an
attachment, **so that** I see the visual result immediately.

## US-13.16.5c.4
**As a** designer (P-5), **I want** the customization screen accessible from inventory or a weapon
bench interaction, **so that** the UI is reachable from multiple contexts.

## US-13.16.5c.5
**As a** tester (P-27), **I want** to verify that the stat comparison accurately reflects the before
and after values, **so that** the preview is trustworthy.

## Surface Type Tags (F-13.16.6a)

## US-13.16.6a.1
**As a** player (P-23), **I want** surfaces to carry material type tags, **so that** impacts produce
appropriate responses for metal, wood, concrete, etc.

## US-13.16.6a.2
**As a** designer (P-5), **I want** surface type tags on physics materials and terrain splatmap
layers, **so that** every surface in the game has a classifiable material.

## US-13.16.6a.3
**As a** designer (P-5), **I want** extensible custom surface types beyond the built-in set, **so
that** I can define new materials for unique environments.

## US-13.16.6a.4
**As a** tester (P-27), **I want** to verify that terrain impact resolves the correct surface type
from the dominant splatmap layer, **so that** terrain material classification is accurate.

## Impact VFX (F-13.16.6b)

## US-13.16.6b.1
**As a** player (P-23), **I want** sparks on metal, dust on concrete, and splinters on wood when
bullets impact, **so that** each surface reacts distinctly.

## US-13.16.6b.2
**As a** designer (P-5), **I want** to configure VFX response tables mapping surface types to
particle assets with spawn count, scale, and lifetime, **so that** I can author impact effects
without code.

## US-13.16.6b.3
**As a** tester (P-27), **I want** to verify that shooting a metal surface spawns the metal particle
asset from the VFX table, **so that** the lookup is correct.

## Impact Audio (F-13.16.6c)

## US-13.16.6c.1
**As a** player (P-23), **I want** to hear clang on metal, thud on wood, and crack on concrete when
bullets impact, **so that** audio reinforces the surface material.

## US-13.16.6c.2
**As a** designer (P-5), **I want** to configure impact audio tables mapping surface types to sound
pools with random variation, volume scaling, and pitch scaling, **so that** I can author impact
audio without code.

## US-13.16.6c.3
**As a** tester (P-27), **I want** to verify that impact sounds play at the correct 3D position
using spatial audio, **so that** impact direction is audible.

## Impact Decals (F-13.16.6d)

## US-13.16.6d.1
**As a** player (P-23), **I want** to see bullet holes on metal, splintered holes on wood, and blood
splatters on flesh after impacts, **so that** combat leaves visible environmental traces.

## US-13.16.6d.2
**As a** designer (P-5), **I want** to configure impact decal tables mapping surface types to decal
assets with lifetime and max count, **so that** I can manage decal quality and budget.

## US-13.16.6d.3
**As a** tester (P-27), **I want** to verify that decals orient along the surface normal at the
impact point, **so that** decal placement is physically correct.
