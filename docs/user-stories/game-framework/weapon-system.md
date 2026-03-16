# User Stories -- Weapon Systems (13.16)

## Weapon Fire Modes (F-13.16.1)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.16.1.1 | player (P-23) | **As a** player (P-23), **I want** to toggle between semi-automatic, burst, and full-automatic fire modes on weapons that support them, **so that** I can adapt my firing to the situation. |  | F-13.16.1 | R-13.16.1 |
| US-13.16.1.2 | player (P-23) | **As a** player (P-23), **I want** each fire mode to have distinct rounds per activation, inter-round delay, and spread, **so that** modes feel mechanically different. |  | F-13.16.1 | R-13.16.1 |
| US-13.16.1.3 | gameplay director (P-3) | **As a** gameplay director (P-3), **I want** fire rate defined as rounds per minute with configurable spin-up for gatling weapons, **so that** weapon archetypes feel unique. |  | F-13.16.1 | R-13.16.1 |
| US-13.16.1.4 | designer (P-5) | **As a** designer (P-5), **I want** to configure fire mode parameters per weapon as data assets, **so that** I can iterate on weapon feel without code changes. |  | F-13.16.1 | R-13.16.1 |
| US-13.16.1.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that burst mode fires exactly N rounds per trigger press, **so that** burst count is accurate. |  | F-13.16.1 | R-13.16.1 |

## Magazine and Ammo Management (F-13.16.2a)

| US-13.16.2 | player (P-23) | **As a** player (P-23), **I want** each weapon to track magazine rounds and reserve ammo, **so that** I must manage my ammunition supply. |  | F-13.16.2 | R-13.16.2 |
| US-13.16.2 | player (P-23) | **As a** player (P-23), **I want** magazine and reserve counts displayed in the HUD, **so that** I know my ammo status at a glance. |  | F-13.16.2 | R-13.16.2 |
| US-13.16.2 | player (P-23) | **As a** player (P-23), **I want** fire input suppressed when the magazine is empty, **so that** I must reload before firing again. |  | F-13.16.2 | R-13.16.2 |
| US-13.16.2 | designer (P-5) | **As a** designer (P-5), **I want** to configure magazine capacity, reserve pool, and shared vs. per-weapon reserve per weapon type, **so that** ammo economy varies by archetype. |  | F-13.16.2 | R-13.16.2 |
| US-13.16.2 | tester (P-27) | **As a** tester (P-27), **I want** to verify that ammo is consumed from inventory when not in unlimited mode, **so that** ammo tracking is correct. |  | F-13.16.2 | R-13.16.2 |

## Reload Mechanics (F-13.16.2b)

| US-13.16.2 | player (P-23) | **As a** player (P-23), **I want** tactical reload to be faster than empty reload, **so that** I am rewarded for reloading before the magazine empties. |  | F-13.16.2 | R-13.16.2 |
| US-13.16.2 | player (P-23) | **As a** player (P-23), **I want** sequential reload to be interruptible by fire input, **so that** I can shoot a partially loaded shotgun in emergencies. |  | F-13.16.2 | R-13.16.2 |
| US-13.16.2 | player (P-23) | **As a** player (P-23), **I want** reloading to cancel sprint, **so that** reload has a movement cost. |  | F-13.16.2 | R-13.16.2 |
| US-13.16.2 | designer (P-5) | **As a** designer (P-5), **I want** to configure reload duration, animation, and sound per weapon as data assets, **so that** reload behavior is tunable per weapon. |  | F-13.16.2 | R-13.16.2 |
| US-13.16.2 | tester (P-27) | **As a** tester (P-27), **I want** to verify that interrupting a sequential reload preserves the rounds already loaded, **so that** partial reload works correctly. |  | F-13.16.2 | R-13.16.2 |

## Ammo Types (F-13.16.2c)

| US-13.16.2 | player (P-23) | **As a** player (P-23), **I want** to switch ammo types at runtime via an input action, **so that** I can adapt to different enemy resistances mid-combat. |  | F-13.16.2 | R-13.16.2 |
| US-13.16.2 | player (P-23) | **As a** player (P-23), **I want** armor-piercing rounds to ignore a percentage of target armor, **so that** I can counter heavily armored enemies. |  | F-13.16.2 | R-13.16.2 |
| US-13.16.2 | player (P-23) | **As a** player (P-23), **I want** incendiary rounds to apply burn damage over time, **so that** I can use fire against vulnerable targets. |  | F-13.16.2 | R-13.16.2 |
| US-13.16.2 | designer (P-5) | **As a** designer (P-5), **I want** to define ammo types with damage modifier, armor penetration, status effect, tracer color, and muzzle VFX in gameplay databases, **so that** I can create diverse ammunition without code. |  | F-13.16.2 | R-13.16.2 |
| US-13.16.2 | gameplay director (P-3) | **As a** gameplay director (P-3), **I want** ammo types to create tactical depth by forcing loadout decisions, **so that** weapon customization has strategic significance. |  | F-13.16.2 | R-13.16.2 |
| US-13.16.2 | tester (P-27) | **As a** tester (P-27), **I want** to verify that switching ammo types updates the tracer color and damage modifier immediately, **so that** the swap applies correctly. |  | F-13.16.2 | R-13.16.2 |

## Recoil and Spread (F-13.16.3)

| US-13.16.3.1 | player (P-23) | **As a** player (P-23), **I want** sustained fire to kick my aim upward and sideways following a pattern, with recovery when I stop, **so that** recoil control is a learnable skill. |  | F-13.16.3 | R-13.16.3 |
| US-13.16.3.2 | player (P-23) | **As a** player (P-23), **I want** the crosshair to dynamically reflect current spread radius, **so that** I can visually judge my accuracy. |  | F-13.16.3 | R-13.16.3 |
| US-13.16.3.3 | player (P-23) | **As a** player (P-23), **I want** spread to widen with movement, hip-fire, and sustained fire, and tighten with ADS and crouching, **so that** positioning and posture improve accuracy. |  | F-13.16.3 | R-13.16.3 |
| US-13.16.3.4 | designer (P-5) | **As a** designer (P-5), **I want** to author recoil patterns as 2D curves in the visual editor per weapon, **so that** I can craft unique weapon feel without code. |  | F-13.16.3 | R-13.16.3 |
| US-13.16.3.5 | gameplay director (P-3) | **As a** gameplay director (P-3), **I want** first-shot accuracy configurable per weapon type (100% for snipers, imprecise for SMGs), **so that** weapon identity is clear from the first shot. |  | F-13.16.3 | R-13.16.3 |
| US-13.16.3.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that the recoil pattern repeats identically on each spray, **so that** recoil is deterministic and learnable. |  | F-13.16.3 | R-13.16.3 |

## Projectile Ballistics (F-13.16.4a)

| US-13.16.4 | player (P-23) | **As a** player (P-23), **I want** projectiles to follow a parabolic trajectory with gravity drop and travel time, **so that** distant targets require leading and elevation. |  | F-13.16.4 | R-13.16.4 |
| US-13.16.4 | player (P-23) | **As a** player (P-23), **I want** drag to decelerate projectiles over distance, **so that** effective range has mechanical meaning. |  | F-13.16.4 | R-13.16.4 |
| US-13.16.4 | designer (P-5) | **As a** designer (P-5), **I want** to configure mass, muzzle velocity, and drag per ammo type, **so that** snipers and shotguns feel physically distinct at range. |  | F-13.16.4 | R-13.16.4 |
| US-13.16.4 | designer (P-5) | **As a** designer (P-5), **I want** to disable gravity and drag independently for arcade modes, **so that** the ballistics system supports both realistic and arcade gameplay. |  | F-13.16.4 | R-13.16.4 |
| US-13.16.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a projectile with zero drag reaches the target at the expected time based on muzzle velocity, **so that** travel time is accurate. |  | F-13.16.4 | R-13.16.4 |

## Wind Deflection (F-13.16.4b)

| US-13.16.4 | player (P-23) | **As a** player (P-23), **I want** wind to deflect my shots laterally, **so that** long-range marksmanship requires reading environmental conditions. |  | F-13.16.4 | R-13.16.4 |
| US-13.16.4 | player (P-23) | **As a** player (P-23), **I want** wind indicators in the sniper scope UI, **so that** I can compensate for wind at long range. |  | F-13.16.4 | R-13.16.4 |
| US-13.16.4 | designer (P-5) | **As a** designer (P-5), **I want** to configure per-ammo wind sensitivity and enable wind per zone, **so that** I can create varying wind conditions across the map. |  | F-13.16.4 | R-13.16.4 |
| US-13.16.4 | designer (P-5) | **As a** designer (P-5), **I want** wind deflection to be disableable independently for arcade modes, **so that** casual modes skip the complexity. |  | F-13.16.4 | R-13.16.4 |
| US-13.16.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a projectile fired perpendicular to wind deflects by the expected distance at a given range, **so that** deflection math is correct. |  | F-13.16.4 | R-13.16.4 |

## Surface Penetration and Ricochet (F-13.16.4c)

| US-13.16.4 | player (P-23) | **As a** player (P-23), **I want** bullets to penetrate thin materials like drywall with energy loss, **so that** cover material matters tactically. |  | F-13.16.4 | R-13.16.4 |
| US-13.16.4 | player (P-23) | **As a** player (P-23), **I want** bullets to ricochet off hard surfaces at shallow angles, **so that** metal walls deflect shots unpredictably. |  | F-13.16.4 | R-13.16.4 |
| US-13.16.4 | designer (P-5) | **As a** designer (P-5), **I want** to configure penetration and ricochet thresholds per physics material, **so that** each surface type responds differently. |  | F-13.16.4 | R-13.16.4 |
| US-13.16.4 | gameplay director (P-3) | **As a** gameplay director (P-3), **I want** penetration and ricochet systems to be independently disableable, **so that** I can control tactical complexity per game mode. |  | F-13.16.4 | R-13.16.4 |
| US-13.16.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a bullet hitting concrete at a steep angle stops rather than ricocheting, **so that** angle thresholds are enforced. |  | F-13.16.4 | R-13.16.4 |

## Weapon Zeroing (F-13.16.4d)

| US-13.16.4 | player (P-23) | **As a** player (P-23), **I want** to adjust my scope zeroing in discrete distance steps, **so that** point of aim matches point of impact at the zeroed range. |  | F-13.16.4 | R-13.16.4 |
| US-13.16.4 | player (P-23) | **As a** player (P-23), **I want** the current zero distance displayed in my scope UI overlay, **so that** I know my current zeroing at a glance. |  | F-13.16.4 | R-13.16.4 |
| US-13.16.4 | player (P-23) | **As a** player (P-23), **I want** switching ammo types to shift the effective zero, **so that** I must re-zero when changing ammunition. |  | F-13.16.4 | R-13.16.4 |
| US-13.16.4 | designer (P-5) | **As a** designer (P-5), **I want** zeroing available only on weapons with optic attachments, **so that** iron-sight weapons cannot be zeroed. |  | F-13.16.4 | R-13.16.4 |
| US-13.16.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a weapon zeroed to 200 m hits dead center at 200 m with the configured ammo type, **so that** zeroing calibration is accurate. |  | F-13.16.4 | R-13.16.4 |

## Attachment Slot Model (F-13.16.5a)

| US-13.16.5 | player (P-23) | **As a** player (P-23), **I want** weapons to have named attachment slots (optic, barrel, muzzle, grip, stock, magazine, rail), **so that** I can customize my weapon. |  | F-13.16.5 | R-13.16.5 |
| US-13.16.5 | player (P-23) | **As a** player (P-23), **I want** each slot to filter compatible attachments by category, **so that** only valid attachments can be equipped. |  | F-13.16.5 | R-13.16.5 |
| US-13.16.5 | designer (P-5) | **As a** designer (P-5), **I want** to define slot categories, attachment stats, and modifier modes (additive or multiplicative) in gameplay databases, **so that** weapon customization is extensible without code. |  | F-13.16.5 | R-13.16.5 |
| US-13.16.5 | gameplay director (P-3) | **As a** gameplay director (P-3), **I want** attachment stat modifiers to create meaningful build choices, **so that** customization affects gameplay strategy. |  | F-13.16.5 | R-13.16.5 |
| US-13.16.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that an optic attachment cannot be placed in a grip slot, **so that** slot category filtering is enforced. |  | F-13.16.5 | R-13.16.5 |

## Attachment Visuals (F-13.16.5b)

| US-13.16.5 | player (P-23) | **As a** player (P-23), **I want** equipped attachments to appear on my weapon model at the correct socket positions, **so that** customization has visible impact. |  | F-13.16.5 | R-13.16.5 |
| US-13.16.5 | player (P-23) | **As a** player (P-23), **I want** optic attachments to render scope reticles with configurable zoom, **so that** optics feel functional in ADS. |  | F-13.16.5 | R-13.16.5 |
| US-13.16.5 | designer (P-5) | **As a** designer (P-5), **I want** to author attachment meshes that snap to named socket transforms on the weapon, **so that** visual integration is consistent. |  | F-13.16.5 | R-13.16.5 |
| US-13.16.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that equipping and unequipping an attachment updates the weapon mesh immediately, **so that** visual changes are instant. |  | F-13.16.5 | R-13.16.5 |

## Attachment Customization UI (F-13.16.5c)

| US-13.16.5 | player (P-23) | **As a** player (P-23), **I want** a dedicated weapon customization screen with a 3D weapon preview and interactive slot indicators, **so that** I can browse and compare attachments. |  | F-13.16.5 | R-13.16.5 |
| US-13.16.5 | player (P-23) | **As a** player (P-23), **I want** stat comparison shown when hovering an attachment, **so that** I can evaluate before equipping. |  | F-13.16.5 | R-13.16.5 |
| US-13.16.5 | player (P-23) | **As a** player (P-23), **I want** the 3D preview to update in real time when I equip or remove an attachment, **so that** I see the visual result immediately. |  | F-13.16.5 | R-13.16.5 |
| US-13.16.5 | designer (P-5) | **As a** designer (P-5), **I want** the customization screen accessible from inventory or a weapon bench interaction, **so that** the UI is reachable from multiple contexts. |  | F-13.16.5 | R-13.16.5 |
| US-13.16.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that the stat comparison accurately reflects the before and after values, **so that** the preview is trustworthy. |  | F-13.16.5 | R-13.16.5 |

## Surface Type Tags (F-13.16.6a)

| US-13.16.6 | player (P-23) | **As a** player (P-23), **I want** surfaces to carry material type tags, **so that** impacts produce appropriate responses for metal, wood, concrete, etc. |  | F-13.16.6 | R-13.16.6 |
| US-13.16.6 | designer (P-5) | **As a** designer (P-5), **I want** surface type tags on physics materials and terrain splatmap layers, **so that** every surface in the game has a classifiable material. |  | F-13.16.6 | R-13.16.6 |
| US-13.16.6 | designer (P-5) | **As a** designer (P-5), **I want** extensible custom surface types beyond the built-in set, **so that** I can define new materials for unique environments. |  | F-13.16.6 | R-13.16.6 |
| US-13.16.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that terrain impact resolves the correct surface type from the dominant splatmap layer, **so that** terrain material classification is accurate. |  | F-13.16.6 | R-13.16.6 |

## Impact VFX (F-13.16.6b)

| US-13.16.6 | player (P-23) | **As a** player (P-23), **I want** sparks on metal, dust on concrete, and splinters on wood when bullets impact, **so that** each surface reacts distinctly. |  | F-13.16.6 | R-13.16.6 |
| US-13.16.6 | designer (P-5) | **As a** designer (P-5), **I want** to configure VFX response tables mapping surface types to particle assets with spawn count, scale, and lifetime, **so that** I can author impact effects without code. |  | F-13.16.6 | R-13.16.6 |
| US-13.16.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that shooting a metal surface spawns the metal particle asset from the VFX table, **so that** the lookup is correct. |  | F-13.16.6 | R-13.16.6 |

## Impact Audio (F-13.16.6c)

| US-13.16.6 | player (P-23) | **As a** player (P-23), **I want** to hear clang on metal, thud on wood, and crack on concrete when bullets impact, **so that** audio reinforces the surface material. |  | F-13.16.6 | R-13.16.6 |
| US-13.16.6 | designer (P-5) | **As a** designer (P-5), **I want** to configure impact audio tables mapping surface types to sound pools with random variation, volume scaling, and pitch scaling, **so that** I can author impact audio without code. |  | F-13.16.6 | R-13.16.6 |
| US-13.16.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that impact sounds play at the correct 3D position using spatial audio, **so that** impact direction is audible. |  | F-13.16.6 | R-13.16.6 |

## Impact Decals (F-13.16.6d)

| US-13.16.6 | player (P-23) | **As a** player (P-23), **I want** to see bullet holes on metal, splintered holes on wood, and blood splatters on flesh after impacts, **so that** combat leaves visible environmental traces. |  | F-13.16.6 | R-13.16.6 |
| US-13.16.6 | designer (P-5) | **As a** designer (P-5), **I want** to configure impact decal tables mapping surface types to decal assets with lifetime and max count, **so that** I can manage decal quality and budget. |  | F-13.16.6 | R-13.16.6 |
| US-13.16.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that decals orient along the surface normal at the impact point, **so that** decal placement is physically correct. |  | F-13.16.6 | R-13.16.6 |

## Projectile Archetypes (F-13.16.7)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.16.7.1 | designer (P-5) | **As a** designer (P-5), **I want** to select from named projectile archetypes (Bullet, Missile, Grenade, Arrow, Spell Bolt) with preset physics profiles, **so that** I can create diverse ranged weapons without custom physics tuning. |  | F-13.16.7 | R-13.16.7 |
| US-13.16.7.2 | designer (P-5) | **As a** designer (P-5), **I want** to configure homing turn rate, fuse mode, bounce restitution, and embed-on-hit per archetype as data, **so that** I can tune projectile behavior without code. |  | F-13.16.7 | R-13.16.7 |
| US-13.16.7.3 | gameplay director (P-3) | **As a** gameplay director (P-3), **I want** each archetype to have distinct visual behaviors (tracer, exhaust trail, elemental VFX), **so that** players can identify projectile types at a glance. |  | F-13.16.7 | R-13.16.7 |
| US-13.16.7.4 | player (P-23) | **As a** player (P-23), **I want** arrows to embed in targets on hit and missiles to home toward locked targets, **so that** different projectile types feel mechanically distinct. |  | F-13.16.7 | R-13.16.7 |
| US-13.16.7.5 | engine tester (P-27) | **As a** tester (P-27), **I want** to fire each archetype and verify its trajectory matches the preset physics (linear, arcing, homing, bouncing), **so that** archetype physics are correct. |  | F-13.16.7 | R-13.16.7 |

## Explosive Projectiles (F-13.16.8)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.16.8.1 | designer (P-5) | **As a** designer (P-5), **I want** to configure blast radius, damage falloff curve, and shrapnel count per explosive projectile, **so that** each explosive weapon has a unique area damage profile. |  | F-13.16.8 | R-13.16.8 |
| US-13.16.8.2 | designer (P-5) | **As a** designer (P-5), **I want** explosions to trigger environmental destruction on destructible objects within the blast radius, **so that** explosive combat affects the environment. |  | F-13.16.8 | R-13.16.8 |
| US-13.16.8.3 | gameplay director (P-3) | **As a** gameplay director (P-3), **I want** explosion friendly fire to respect the per-ability toggle, **so that** I can control whether explosions damage allies in different game modes. |  | F-13.16.8 | R-13.16.8 |
| US-13.16.8.4 | player (P-23) | **As a** player (P-23), **I want** explosions to apply physics impulse that ragdolls enemies and scatters objects, **so that** explosive weapons feel powerful and impactful. |  | F-13.16.8 | R-13.16.8 |
| US-13.16.8.5 | engine tester (P-27) | **As a** tester (P-27), **I want** to detonate an explosive and verify that entities at the epicenter receive full damage while entities at the edge receive reduced damage matching the falloff curve, **so that** blast damage calculation is correct. |  | F-13.16.8 | R-13.16.8 |

## Projectile Lifetime (F-13.16.9)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.16.9.1 | designer (P-5) | **As a** designer (P-5), **I want** to configure maximum range, maximum time alive, and despawn behavior (fade, explode, instant) per projectile, **so that** projectiles do not persist indefinitely. |  | F-13.16.9 | R-13.16.9 |
| US-13.16.9.2 | designer (P-5) | **As a** designer (P-5), **I want** a runtime debug overlay showing active projectile count and pool utilization per archetype, **so that** I can monitor projectile performance budgets. |  | F-13.16.9 | R-13.16.9 |
| US-13.16.9.3 | gameplay director (P-3) | **As a** gameplay director (P-3), **I want** projectile pooling to recycle despawned entities automatically, **so that** high-fire-rate weapons do not cause allocation spikes. |  | F-13.16.9 | R-13.16.9 |
| US-13.16.9.4 | player (P-23) | **As a** player (P-23), **I want** projectiles that miss to disappear after a reasonable range or time, **so that** stray projectiles do not clutter the game world. |  | F-13.16.9 | R-13.16.9 |
| US-13.16.9.5 | engine tester (P-27) | **As a** tester (P-27), **I want** to fire a projectile and verify it despawns at the configured max range, then verify the entity is returned to the pool and reused on the next spawn, **so that** pooling and despawn work correctly. |  | F-13.16.9 | R-13.16.9 |

## Projectile Interaction (F-13.16.10)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.16.10.1 | designer (P-5) | **As a** designer (P-5), **I want** to tag projectiles as "shootable" and configure shoot-down behavior, **so that** players can intercept incoming missiles with gunfire. |  | F-13.16.10 | R-13.16.10 |
| US-13.16.10.2 | designer (P-5) | **As a** designer (P-5), **I want** to configure reflective barriers that reverse projectile direction and transfer ownership, **so that** energy shields and magic wards create counter-play opportunities. |  | F-13.16.10 | R-13.16.10 |
| US-13.16.10.3 | gameplay director (P-3) | **As a** gameplay director (P-3), **I want** absorptive barriers to destroy projectiles and optionally gain energy, **so that** defensive abilities can convert enemy fire into a resource. |  | F-13.16.10 | R-13.16.10 |
| US-13.16.10.4 | player (P-23) | **As a** player (P-23), **I want** to shoot down incoming missiles and reflect projectiles off my energy shield, **so that** I have active counter-play options against ranged attacks. |  | F-13.16.10 | R-13.16.10 |
| US-13.16.10.5 | engine tester (P-27) | **As a** tester (P-27), **I want** to fire a projectile at a reflective barrier and verify it reverses direction with ownership transferred to the barrier owner, **so that** reflection mechanics work correctly. |  | F-13.16.10 | R-13.16.10 |
