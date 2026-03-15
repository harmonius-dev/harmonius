# 13.16 — Weapon Systems

## Fire Modes and Ammunition

### F-13.16.1 Weapon Fire Mode System

Per-weapon configurable fire modes: semi-automatic (one shot per trigger press), burst (N rounds per
press with configurable delay between rounds), and full-automatic (continuous fire while held).
Players toggle between available modes via input action. Each mode defines: rounds per trigger
activation, inter-round delay, spread modifier, and recoil modifier. The fire mode component is an
ECS component on weapon entities, driven by the ability activation system (F-13.10.2). Fire rate is
defined as rounds per minute with configurable spin-up for gatling-style weapons.

- **Requirements:** R-13.16.1
- **Dependencies:** F-13.10.2 (Ability Activation), F-13.10.5 (Ranged Combat)
- **Platform notes:** None

### F-13.16.2a Magazine and Ammo Management

Magazine-based ammunition tracking per weapon entity. Each weapon has a magazine capacity (maximum
rounds per load), current round count, and reserve ammo pool. Ammo is consumed from inventory
(F-13.9.1) or from an unlimited pool depending on game mode configuration. Magazine and reserve
counts are displayed in the HUD (F-10.3.1). When the magazine is empty, fire input is suppressed
until reload completes. Reserve ammo is shared or per-weapon depending on configuration.

- **Requirements:** R-13.16.2a
- **Dependencies:** F-13.16.1, F-13.9.1 (Inventory), F-10.3.1 (HUD)
- **Platform notes:** None
- **User story:** As a designer, I want to configure magazine capacity and reserve pools per weapon
  so that ammo economy varies by weapon archetype.

### F-13.16.2b Reload Mechanics

Reload behaviors driven by weapon configuration. Tactical reload (magazine not empty) is faster than
empty reload (magazine depleted). Sequential reload (shotgun shell-by-shell) can be interrupted by
fire input to shoot with a partially loaded magazine. Reload cancels sprint. Reload duration,
animation, and sound are per-weapon data assets. The reload state is an ECS component driven by the
ability activation system (F-13.10.2).

- **Requirements:** R-13.16.2b
- **Dependencies:** F-13.16.2a, F-13.10.2 (Ability Activation), F-9.4.1 (Animation)
- **Platform notes:** None
- **User story:** As a player, I want tactical and empty reloads to feel distinct so that I am
  rewarded for reloading before my magazine empties.

### F-13.16.2c Ammo Type System

Per-weapon swappable ammunition types with distinct gameplay properties. Ammo types include
standard, armor-piercing (ignores a percentage of target armor), incendiary (applies burn damage
over time), and hollow-point (increased damage vs. unarmored, decreased vs. armored). Players switch
ammo type at runtime via an input action. Each ammo type defines damage modifier, armor penetration,
status effect, tracer color, and muzzle VFX. Ammo types are gameplay database entries.

- **Requirements:** R-13.16.2c
- **Dependencies:** F-13.16.2a, F-13.7.1 (Table Schema), F-13.10.3 (Gameplay Effects)
- **Platform notes:** None
- **User story:** As a player, I want to switch ammo types on the fly so that I can adapt to
  different enemy resistances mid-combat.

### F-13.16.3 Recoil Pattern and Weapon Spread

Per-weapon recoil patterns defined as curves that shift aim direction over sustained fire. Recoil
kicks the camera/aim point upward and sideways following the pattern; recovery pulls back toward
center when firing stops. Spread defines a cone of inaccuracy that widens with: sustained fire,
movement, hip-fire, and jumping; and tightens with: ADS, crouching, and burst discipline. First-shot
accuracy can be 100% (sniper) or imprecise (SMG). Recoil patterns are data assets authored as 2D
curves in the visual editor. The crosshair dynamically reflects current spread radius.

- **Requirements:** R-13.16.3
- **Dependencies:** F-13.16.1, F-9.4.9 (Aim Offsets)
- **Platform notes:** None

## Ballistics

### F-13.16.4a Projectile Drop and Travel Time

Projectile physics for gravity-based bullet drop and finite travel time. Each ammo type defines
mass, muzzle velocity, and aerodynamic drag coefficient. Projectiles follow a parabolic trajectory
computed per physics tick: gravity pulls the projectile downward while drag decelerates it over
distance. Travel time is derived from muzzle velocity minus drag — distant targets require leading.
All parameters are per-ammo-type data in gameplay databases. Arcade modes can disable gravity and/or
drag individually.

- **Requirements:** R-13.16.4a
- **Dependencies:** F-13.10.5 (Ranged Combat), F-4.1.1 (Rigid Body ECS)
- **Platform notes:** None
- **User story:** As a designer, I want to configure bullet drop and drag per ammo type so that
  snipers and shotguns feel physically distinct at range.

### F-13.16.4b Wind Deflection

Lateral wind deflection applied to in-flight projectiles. The wind vector is sourced from the
weather system (F-11.4.1) and applies a cross-force proportional to projectile cross-section and
wind speed. Wind strength and direction can vary by world zone or change over time. A per-ammo wind
sensitivity scalar controls how much each round is affected. Wind indicators can be displayed in
sniper scope UI. Disableable independently for arcade modes.

- **Requirements:** R-13.16.4b
- **Dependencies:** F-13.16.4a, F-11.4.1 (Weather)
- **Platform notes:** None
- **User story:** As a player, I want wind to deflect my shots so that long-range marksmanship
  requires reading environmental conditions.

### F-13.16.4c Surface Penetration and Ricochet

Material-based penetration and ricochet when projectiles impact surfaces. Penetration: projectiles
pass through surfaces with energy loss based on material density — drywall allows pass-through,
concrete stops rounds. Remaining energy after penetration determines exit velocity and residual
damage. Ricochet: projectiles striking hard surfaces at shallow angles (below a material-specific
threshold) bounce with energy loss and angle randomization. Probability of ricochet versus
absorption is computed from impact angle and material hardness. Both systems are disableable
independently.

- **Requirements:** R-13.16.4c
- **Dependencies:** F-13.16.4a, F-4.2.9 (Physics Materials)
- **Platform notes:** None
- **User story:** As a player, I want bullets to penetrate thin walls and ricochet off metal so that
  cover material matters tactically.

### F-13.16.4d Weapon Zeroing

Sight alignment adjustment for a target distance. Zeroing offsets the aim point vertically so that
the point of aim matches the point of impact at the zeroed distance, compensating for bullet drop.
Players adjust zeroing in discrete steps (100 m, 200 m, 300 m, etc.) via input action. The current
zero distance is displayed in the scope UI overlay. Zeroing interacts with ammo type — switching
ammo with different ballistic profiles shifts the effective zero. Only applicable to weapons with
optic attachments (F-13.16.5a).

- **Requirements:** R-13.16.4d
- **Dependencies:** F-13.16.4a, F-13.16.5a (Attachment Slot Model)
- **Platform notes:** None
- **User story:** As a player, I want to zero my scope to a specific range so that I can make
  accurate first shots at known distances.

## Weapon Customization

### F-13.16.5a Attachment Slot Model

Data-driven slot system for weapon modification. Weapons define named attachment slots (optic,
barrel, muzzle, grip, stock, magazine, rail accessory). Each slot specifies a category filter that
restricts which attachments can be equipped. Attachments are inventory items (F-13.9.5) with stat
modifiers: scope zoom, suppressor reduces sound and range, grip reduces recoil, extended magazine
increases capacity. Stat modifications are applied additively or multiplicatively as configured.
Slot definitions and attachment stats are gameplay database entries.

- **Requirements:** R-13.16.5a
- **Dependencies:** F-13.16.1, F-13.9.5 (Item Sockets), F-13.7.1 (Table Schema)
- **Platform notes:** None
- **User story:** As a designer, I want to define attachment slots and stat modifiers in data so
  that weapon customization is extensible without code changes.

### F-13.16.5b Attachment Visual Integration

Visual representation of equipped attachments on weapon meshes. Each attachment defines a mesh asset
that snaps to the weapon at a named socket transform. Socket transforms are authored on weapon mesh
assets in the DCC pipeline. Optic attachments render scope reticles using the shader system with
configurable reticle texture and zoom level. Barrel and muzzle attachments replace the default
muzzle flash VFX point. Visual updates occur immediately on equip/unequip. The ECS-to-Renderer
bridge (F-2.10.1) propagates attachment mesh changes to the render scene.

- **Requirements:** R-13.16.5b
- **Dependencies:** F-13.16.5a, F-2.10.1 (ECS-to-Renderer Bridge)
- **Platform notes:** None
- **User story:** As a player, I want to see attachments on my weapon model so that customization
  has visible impact in gameplay.

### F-13.16.5c Attachment Customization UI

Dedicated weapon customization screen for managing attachments. Displays a 3D preview of the weapon
with interactive slot indicators highlighted on each attachment point. Selecting a slot opens a
filtered list of compatible attachments from inventory. Stat comparison (before/after) is shown when
hovering an attachment. Equipping or removing an attachment updates the 3D preview in real time. The
UI is accessible from the inventory screen or a dedicated weapon bench interaction (F-13.17.1).

- **Requirements:** R-13.16.5c
- **Dependencies:** F-13.16.5a, F-13.16.5b, F-10.1.1 (Widget Tree), F-13.17.1 (Interaction)
- **Platform notes:** None
- **User story:** As a player, I want a visual weapon customization screen so that I can preview
  attachments on a 3D model before committing changes.

## Surface Response

### F-13.16.6a Surface Type Tag System

Physics material classification for impact response dispatch. Physics materials (F-4.2.9) carry a
surface type tag enum (metal, wood, concrete, dirt, glass, water, flesh, and extensible custom
types). Static meshes inherit surface type from their physics material. Terrain surfaces resolve
type from the splatmap's dominant material weight at the impact point using bilinear sampling of
splat layer weights. The surface type is returned as part of all physics query results (raycasts,
shape casts, collision contacts) for downstream systems to consume.

- **Requirements:** R-13.16.6a
- **Dependencies:** F-4.2.9 (Physics Materials), F-3.2.4 (Terrain Splatmap)
- **Platform notes:** None
- **User story:** As a designer, I want every surface to carry a material type tag so that impacts
  produce context-appropriate responses automatically.

### F-13.16.6b Impact VFX Response

Particle effect lookup and spawning based on surface type on impact. When a projectile, melee
weapon, or foot contacts a surface, the system looks up the VFX response table for the surface type:
sparks for metal, dust for concrete, splinters for wood, blood for flesh. Each entry defines the
particle asset, spawn count, scale, and lifetime. VFX response tables are data assets authored in
the visual editor. The system integrates with event-driven VFX (F-11.6.4) for spawning.

- **Requirements:** R-13.16.6b
- **Dependencies:** F-13.16.6a, F-11.6.4 (Event-Driven VFX)
- **Platform notes:** None
- **User story:** As a player, I want to see sparks on metal and dust on concrete so that impacts
  feel physically grounded.

### F-13.16.6c Impact Audio Response

Sound effect lookup and playback based on surface type on impact. The impact audio table maps each
surface type to a sound asset (clang for metal, thud for wood, crack for concrete). Each entry
supports random variation from a sound pool to avoid repetition. Volume and pitch scale with impact
force magnitude. Audio response tables are authored alongside VFX tables in the visual editor.
Playback uses the spatial audio system (F-5.1.1) positioned at the impact point.

- **Requirements:** R-13.16.6c
- **Dependencies:** F-13.16.6a, F-5.1.1 (Audio Engine)
- **Platform notes:** None
- **User story:** As a player, I want to hear distinct impact sounds per surface material so that
  audio reinforces the physical context of each hit.

### F-13.16.6d Impact Decal Response

Decal placement based on surface type on impact. The impact decal table maps each surface type to a
decal asset (bullet holes for metal, splintered holes for wood, blood splatters for flesh). Decals
are projected onto the surface at the impact point and oriented along the surface normal. Decal
lifetime and maximum count are configurable to manage rendering budget. The system uses the decal
rendering system (F-11.2.1) for placement and rendering. Decal tables are authored in the visual
editor.

- **Requirements:** R-13.16.6d
- **Dependencies:** F-13.16.6a, F-11.2.1 (Decals)
- **Platform notes:** None
- **User story:** As a player, I want to see bullet holes and damage marks on surfaces so that
  combat leaves visible environmental traces.
