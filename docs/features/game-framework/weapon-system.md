# 13.16 — Weapon Systems

## Fire Modes and Ammunition

| ID         | Feature                          |
|------------|----------------------------------|
| F-13.16.1  | Weapon Fire Mode System          |
| F-13.16.2a | Magazine and Ammo Management     |
| F-13.16.2b | Reload Mechanics                 |
| F-13.16.2c | Ammo Type System                 |
| F-13.16.3  | Recoil Pattern and Weapon Spread |

1. **F-13.16.1** — Per-weapon configurable fire modes: semi-automatic (one shot per trigger press),
   burst (N rounds per press with configurable delay between rounds), and full-automatic (continuous
   fire while held). Players toggle between available modes via input action. Each mode defines:
   rounds per trigger activation, inter-round delay, spread modifier, and recoil modifier. The fire
   mode component is an ECS component on weapon entities, driven by the ability activation system
   (F-13.10.2). Fire rate is defined as rounds per minute with configurable spin-up for
   gatling-style weapons.
   - **Deps:** F-13.10.2 (Ability Activation), F-13.10.5 (Ranged Combat)
2. **F-13.16.2a** — Magazine-based ammunition tracking per weapon entity. Each weapon has a magazine
   capacity (maximum rounds per load), current round count, and reserve ammo pool. Ammo is consumed
   from inventory (F-13.9.1) or from an unlimited pool depending on game mode configuration.
   Magazine and reserve counts are displayed in the HUD (F-10.3.1). When the magazine is empty, fire
   input is suppressed until reload completes. Reserve ammo is shared or per-weapon depending on
   configuration.
   - **Deps:** F-13.16.1, F-13.9.1 (Inventory), F-10.3.1 (HUD)
3. **F-13.16.2b** — Reload behaviors driven by weapon configuration. Tactical reload (magazine not
   empty) is faster than empty reload (magazine depleted). Sequential reload (shotgun
   shell-by-shell) can be interrupted by fire input to shoot with a partially loaded magazine.
   Reload cancels sprint. Reload duration, animation, and sound are per-weapon data assets. The
   reload state is an ECS component driven by the ability activation system (F-13.10.2).
   - **Deps:** F-13.16.2a, F-13.10.2 (Ability Activation), F-9.4.1 (Animation)
4. **F-13.16.2c** — Per-weapon swappable ammunition types with distinct gameplay properties. Ammo
   types include standard, armor-piercing (ignores a percentage of target armor), incendiary
   (applies burn damage over time), and hollow-point (increased damage vs. unarmored, decreased vs.
   armored). Players switch ammo type at runtime via an input action. Each ammo type defines damage
   modifier, armor penetration, status effect, tracer color, and muzzle VFX. Ammo types are gameplay
   database entries.
   - **Deps:** F-13.16.2a, F-13.7.1 (Table Schema), F-13.10.3 (Gameplay Effects)
5. **F-13.16.3** — Per-weapon recoil patterns defined as curves that shift aim direction over
   sustained fire. Recoil kicks the camera/aim point upward and sideways following the pattern;
   recovery pulls back toward center when firing stops. Spread defines a cone of inaccuracy that
   widens with: sustained fire, movement, hip-fire, and jumping; and tightens with: ADS, crouching,
   and burst discipline. First-shot accuracy can be 100% (sniper) or imprecise (SMG). Recoil
   patterns are data assets authored as 2D curves in the visual editor. The crosshair dynamically
   reflects current spread radius.
   - **Deps:** F-13.16.1, F-9.4.9 (Aim Offsets)

## Ballistics

| ID         | Feature                          |
|------------|----------------------------------|
| F-13.16.4a | Projectile Drop and Travel Time  |
| F-13.16.4b | Wind Deflection                  |
| F-13.16.4c | Surface Penetration and Ricochet |
| F-13.16.4d | Weapon Zeroing                   |

1. **F-13.16.4a** — Projectile physics for gravity-based bullet drop and finite travel time. Each
   ammo type defines mass, muzzle velocity, and aerodynamic drag coefficient. Projectiles follow a
   parabolic trajectory computed per physics tick: gravity pulls the projectile downward while drag
   decelerates it over distance. Travel time is derived from muzzle velocity minus drag — distant
   targets require leading. All parameters are per-ammo-type data in gameplay databases. Arcade
   modes can disable gravity and/or drag individually.
   - **Deps:** F-13.10.5 (Ranged Combat), F-4.1.1 (Rigid Body ECS)
2. **F-13.16.4b** — Lateral wind deflection applied to in-flight projectiles. The wind vector is
   sourced from the weather system (F-11.4.1) and applies a cross-force proportional to projectile
   cross-section and wind speed. Wind strength and direction can vary by world zone or change over
   time. A per-ammo wind sensitivity scalar controls how much each round is affected. Wind
   indicators can be displayed in sniper scope UI. Disableable independently for arcade modes.
   - **Deps:** F-13.16.4a, F-11.4.1 (Weather)
3. **F-13.16.4c** — Material-based penetration and ricochet when projectiles impact surfaces.
   Penetration: projectiles pass through surfaces with energy loss based on material density —
   drywall allows pass-through, concrete stops rounds. Remaining energy after penetration determines
   exit velocity and residual damage. Ricochet: projectiles striking hard surfaces at shallow angles
   (below a material-specific threshold) bounce with energy loss and angle randomization.
   Probability of ricochet versus absorption is computed from impact angle and material hardness.
   Both systems are disableable independently.
   - **Deps:** F-13.16.4a, F-4.2.9 (Physics Materials)
4. **F-13.16.4d** — Sight alignment adjustment for a target distance. Zeroing offsets the aim point
   vertically so that the point of aim matches the point of impact at the zeroed distance,
   compensating for bullet drop. Players adjust zeroing in discrete steps (100 m, 200 m, 300 m,
   etc.) via input action. The current zero distance is displayed in the scope UI overlay. Zeroing
   interacts with ammo type — switching ammo with different ballistic profiles shifts the effective
   zero. Only applicable to weapons with optic attachments (F-13.16.5a).
   - **Deps:** F-13.16.4a, F-13.16.5a (Attachment Slot Model)

## Weapon Customization

| ID         | Feature                       |
|------------|-------------------------------|
| F-13.16.5a | Attachment Slot Model         |
| F-13.16.5b | Attachment Visual Integration |
| F-13.16.5c | Attachment Customization UI   |

1. **F-13.16.5a** — Data-driven slot system for weapon modification. Weapons define named attachment
   slots (optic, barrel, muzzle, grip, stock, magazine, rail accessory). Each slot specifies a
   category filter that restricts which attachments can be equipped. Attachments are inventory items
   (F-13.9.5) with stat modifiers: scope zoom, suppressor reduces sound and range, grip reduces
   recoil, extended magazine increases capacity. Stat modifications are applied additively or
   multiplicatively as configured. Slot definitions and attachment stats are gameplay database
   entries.
   - **Deps:** F-13.16.1, F-13.9.5 (Item Sockets), F-13.7.1 (Table Schema)
2. **F-13.16.5b** — Visual representation of equipped attachments on weapon meshes. Each attachment
   defines a mesh asset that snaps to the weapon at a named socket transform. Socket transforms are
   authored on weapon mesh assets in the DCC pipeline. Optic attachments render scope reticles using
   the shader system with configurable reticle texture and zoom level. Barrel and muzzle attachments
   replace the default muzzle flash VFX point. Visual updates occur immediately on equip/unequip.
   The ECS-to-Renderer bridge (F-2.10.1) propagates attachment mesh changes to the render scene.
   - **Deps:** F-13.16.5a, F-2.10.1 (ECS-to-Renderer Bridge)
3. **F-13.16.5c** — Dedicated weapon customization screen for managing attachments. Displays a 3D
   preview of the weapon with interactive slot indicators highlighted on each attachment point.
   Selecting a slot opens a filtered list of compatible attachments from inventory. Stat comparison
   (before/after) is shown when hovering an attachment. Equipping or removing an attachment updates
   the 3D preview in real time. The UI is accessible from the inventory screen or a dedicated weapon
   bench interaction (F-13.17.1).
   - **Deps:** F-13.16.5a, F-13.16.5b, F-10.1.1 (Widget Tree), F-13.17.1 (Interaction)

## Surface Response

| ID         | Feature                 |
|------------|-------------------------|
| F-13.16.6a | Surface Type Tag System |
| F-13.16.6b | Impact VFX Response     |
| F-13.16.6c | Impact Audio Response   |
| F-13.16.6d | Impact Decal Response   |

1. **F-13.16.6a** — Physics material classification for impact response dispatch. Physics materials
   (F-4.2.9) carry a surface type tag enum (metal, wood, concrete, dirt, glass, water, flesh, and
   extensible custom types). Static meshes inherit surface type from their physics material. Terrain
   surfaces resolve type from the splatmap's dominant material weight at the impact point using
   bilinear sampling of splat layer weights. The surface type is returned as part of all physics
   query results (raycasts, shape casts, collision contacts) for downstream systems to consume.
   - **Deps:** F-4.2.9 (Physics Materials), F-3.2.4 (Terrain Splatmap)
2. **F-13.16.6b** — Particle effect lookup and spawning based on surface type on impact. When a
   projectile, melee weapon, or foot contacts a surface, the system looks up the VFX response table
   for the surface type: sparks for metal, dust for concrete, splinters for wood, blood for flesh.
   Each entry defines the particle asset, spawn count, scale, and lifetime. VFX response tables are
   data assets authored in the visual editor. The system integrates with event-driven VFX (F-11.6.4)
   for spawning.
   - **Deps:** F-13.16.6a, F-11.6.4 (Event-Driven VFX)
3. **F-13.16.6c** — Sound effect lookup and playback based on surface type on impact. The impact
   audio table maps each surface type to a sound asset (clang for metal, thud for wood, crack for
   concrete). Each entry supports random variation from a sound pool to avoid repetition. Volume and
   pitch scale with impact force magnitude. Audio response tables are authored alongside VFX tables
   in the visual editor. Playback uses the spatial audio system (F-5.1.1) positioned at the impact
   point.
   - **Deps:** F-13.16.6a, F-5.1.1 (Audio Engine)
4. **F-13.16.6d** — Decal placement based on surface type on impact. The impact decal table maps
   each surface type to a decal asset (bullet holes for metal, splintered holes for wood, blood
   splatters for flesh). Decals are projected onto the surface at the impact point and oriented
   along the surface normal. Decal lifetime and maximum count are configurable to manage rendering
   budget. The system uses the decal rendering system (F-11.2.1) for placement and rendering. Decal
   tables are authored in the visual editor.
   - **Deps:** F-13.16.6a, F-11.2.1 (Decals)

## Projectile Archetypes and Interactions

| ID         | Feature                |
|------------|------------------------|
| F-13.16.7  | Projectile Archetypes  |
| F-13.16.8  | Explosive Projectiles  |
| F-13.16.9  | Projectile Lifetime    |
| F-13.16.10 | Projectile Interaction |

1. **F-13.16.7** — Named projectile categories with preset physics profiles and visual behaviors.
   Bullet: hitscan or very fast linear travel, minimal drop, tracer VFX. Missile: homing toward a
   locked target with configurable turn rate, exhaust trail particle effect, proximity or contact
   detonation. Torpedo: underwater homing with buoyancy-adjusted physics, bubble trail VFX, reduced
   turn rate compared to air missiles. RPG: arced trajectory with back-blast VFX, area damage on
   impact with configurable blast radius. Grenade: arced trajectory with bounce physics
   (configurable restitution), timed fuse or impact fuse mode, audible pin-pull and bounce sounds.
   Arrow: arced trajectory with gravity, embed-on-hit (attaches to target skeleton bone on contact),
   retrievable from corpses. Spell Bolt: homing or straight trajectory, elemental VFX (fire, ice,
   arcane) driven by gameplay tags, configurable homing strength. Each archetype is a data asset
   with all parameters exposed for designer tuning.
   - **Deps:** F-13.10.5 (Ranged Combat), F-13.16.4a (Ballistics), F-11.6.1 (Effect Graph)
2. **F-13.16.8** — Area damage on projectile detonation. Explosive projectiles define a blast
   radius, damage falloff curve (linear, quadratic, or step function from epicenter to edge), and
   optional shrapnel sub-projectiles spawned radially on detonation. Shrapnel projectiles are
   lightweight ECS entities with short lifetimes and their own collision/damage payloads. Explosions
   trigger environmental destruction (F-11.5.1) for destructible objects within the blast radius.
   Explosion VFX and audio scale with blast radius. Friendly fire for explosion damage respects the
   per-ability toggle (F-13.10.7). Explosion force applies physics impulse to dynamic rigid bodies
   within the radius, enabling ragdoll knockback and object scatter.
   - **Deps:** F-13.16.7, F-13.10.3 (Gameplay Effects), F-11.5.1 (Destruction), F-4.1.1 (Rigid Body)
3. **F-13.16.9** — Configurable projectile despawn rules for performance and gameplay. Each
   projectile defines: maximum range (distance from spawn point), maximum time alive (seconds), and
   despawn behavior (fade out, explode, or instant remove). Projectiles exceeding either limit are
   despawned. The system uses entity pooling — despawned projectile entities are recycled into an
   archetype-specific pool and reactivated on next spawn, avoiding allocation overhead. Pool size is
   configurable per archetype with automatic growth and periodic shrink. A runtime debug overlay
   shows active projectile count, pool utilization, and despawn rate per archetype.
   - **Deps:** F-13.10.5 (Ranged Combat), F-1.1.1 (ECS Core)
4. **F-13.16.10** — Projectile-versus-projectile and projectile-versus-barrier collision behaviors.
   Shoot-down: projectiles with a "shootable" tag can be destroyed by other projectiles (e.g.,
   shooting down an incoming missile), with the intercepting projectile also destroyed on contact.
   Reflection: projectiles hitting a "reflective" barrier (energy shield, magic ward) reverse
   direction and transfer ownership to the barrier's owner, applying the original effect payload
   against the original shooter. Absorption: projectiles hitting an "absorptive" barrier are
   destroyed with no effect applied, and the barrier optionally gains energy proportional to the
   absorbed projectile's damage. Each interaction type is configurable per projectile archetype and
   per barrier type via gameplay tags. Visual and audio feedback (spark, energy pulse, absorption
   glow) plays on each interaction type.
   - **Deps:** F-13.16.7, F-13.10.5 (Ranged Combat), F-4.2.1 (Broadphase)
