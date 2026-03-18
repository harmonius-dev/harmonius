# R-13.16 -- Weapon Systems Requirements

| ID         | Derived From                                                 |
|------------|--------------------------------------------------------------|
| R-13.16.1  | [F-13.16.1](../../features/game-framework/weapon-system.md)  |
| R-13.16.2a | [F-13.16.2a](../../features/game-framework/weapon-system.md) |
| R-13.16.2b | [F-13.16.2b](../../features/game-framework/weapon-system.md) |
| R-13.16.2c | [F-13.16.2c](../../features/game-framework/weapon-system.md) |
| R-13.16.3  | [F-13.16.3](../../features/game-framework/weapon-system.md)  |
| R-13.16.4a | [F-13.16.4a](../../features/game-framework/weapon-system.md) |
| R-13.16.4b | [F-13.16.4b](../../features/game-framework/weapon-system.md) |
| R-13.16.4c | [F-13.16.4c](../../features/game-framework/weapon-system.md) |
| R-13.16.4d | [F-13.16.4d](../../features/game-framework/weapon-system.md) |
| R-13.16.5a | [F-13.16.5a](../../features/game-framework/weapon-system.md) |
| R-13.16.5b | [F-13.16.5b](../../features/game-framework/weapon-system.md) |
| R-13.16.5c | [F-13.16.5c](../../features/game-framework/weapon-system.md) |
| R-13.16.6a | [F-13.16.6a](../../features/game-framework/weapon-system.md) |
| R-13.16.6b | [F-13.16.6b](../../features/game-framework/weapon-system.md) |
| R-13.16.6c | [F-13.16.6c](../../features/game-framework/weapon-system.md) |
| R-13.16.6d | [F-13.16.6d](../../features/game-framework/weapon-system.md) |

1. **R-13.16.1** — The engine **SHALL** support per-weapon configurable fire modes (semi-automatic,
   burst, full- automatic) with per-mode rounds per activation, inter-round delay, spread modifier,
   and recoil modifier, toggled via input action and driven by the ability activation system.
   - **Rationale:** Data-driven fire modes on ECS weapon entities let designers tune weapon feel
     without code changes while supporting diverse weapon archetypes from pistols to miniguns.
   - **Verification:** Configure a weapon with all three fire modes. Verify semi-automatic fires one
     round per press, burst fires the configured N rounds with inter-round delay, and full-
     automatic fires continuously while held. Toggle modes via input and confirm the spread and
     recoil modifiers change to match each mode's configured values.
2. **R-13.16.2a** — The engine **SHALL** track per-weapon magazine capacity, current round count,
   and reserve ammo pool, with ammo consumed from inventory or an unlimited pool per game mode, and
   magazine and reserve counts displayed in the HUD.
   - **Rationale:** Magazine tracking is the foundation for all reload and ammo systems, and HUD
     display keeps the player informed of ammunition state.
   - **Verification:** Fire a weapon and verify the round count decrements. Verify the HUD displays
     current magazine and reserve counts. Empty the magazine and confirm fire input is suppressed.
     Configure unlimited ammo mode and verify reserve never depletes.
3. **R-13.16.2b** — The engine **SHALL** implement tactical reload (faster when magazine is not
   empty), empty reload (slower when magazine is depleted), and sequential reload (interruptible
   shell-by-shell), where reload cancels sprint and duration is configurable per weapon.
   - **Rationale:** Reload variants add tactical depth — rewarding players for reloading before
     empty and allowing shotgun users to interrupt reloads for emergency shots.
   - **Verification:** Fire a weapon partially and reload. Verify tactical reload completes faster
     than empty reload. Load a shotgun shell-by-shell, interrupt mid-reload by firing, and confirm
     the weapon fires with the partially loaded magazine. Verify reload cancels sprint.
4. **R-13.16.2c** — The engine **SHALL** support per-weapon swappable ammo types (standard,
   armor-piercing, incendiary, hollow-point) with per-type damage modifier, armor penetration,
   status effect, and visual properties, switchable at runtime via input action.
   - **Rationale:** Ammo type switching enables tactical adaptation to different enemy types without
     requiring weapon swaps.
   - **Verification:** Switch ammo type via input and verify the active ammo changes. Fire
     armor-piercing rounds at an armored target and verify increased penetration. Fire incendiary
     rounds and verify burn status effect applies. Confirm tracer color changes per ammo type.
5. **R-13.16.3** — The engine **SHALL** apply per-weapon recoil patterns defined as 2D data curves
   that shift aim direction over sustained fire, with spread widening from movement, hip-fire, and
   sustained fire, and tightening from ADS, crouching, and burst discipline, reflected dynamically
   in the crosshair.
   - **Rationale:** Learnable recoil patterns and context-sensitive spread reward skill mastery
     while allowing designers to differentiate weapon handling characteristics through data.
   - **Verification:** Fire a weapon in sustained full-auto and record aim offset per frame. Verify
     the offset follows the authored 2D recoil curve. Measure spread cone while stationary ADS
     versus sprinting hip-fire and confirm the stationary ADS cone is smaller by the configured
     modifier ratio. Verify the crosshair radius matches the current spread value.
6. **R-13.16.4a** — The engine **SHALL** simulate gravity-based bullet drop and finite travel time
   with per-ammo-type mass, muzzle velocity, and drag coefficient, following a parabolic trajectory,
   with gravity and drag independently disableable for arcade modes.
   - **Rationale:** Gravity drop and travel time are the foundational ballistic physics that
     differentiate weapon archetypes at range.
   - **Verification:** Fire a round at a target 500 m away and verify bullet drop matches the
     expected parabolic trajectory within 5% tolerance. Disable gravity and confirm the projectile
     travels in a straight line. Measure travel time and verify it matches muzzle velocity minus
     drag over the distance.
7. **R-13.16.4b** — The engine **SHALL** apply lateral wind deflection to in-flight projectiles
   based on a wind vector from the weather system, with per-ammo wind sensitivity and independent
   disableability.
   - **Rationale:** Wind deflection adds environmental skill expression to long-range shooting
     without affecting close-range combat.
   - **Verification:** Fire at a distant target with wind active and verify lateral deviation
     matches the wind vector and ammo sensitivity. Disable wind and confirm no lateral deflection.
     Change wind direction mid-flight and verify the projectile adjusts.
8. **R-13.16.4c** — The engine **SHALL** simulate surface penetration with material-density-based
   energy loss and ricochet at shallow angles with probability based on impact angle and material
   hardness, both independently disableable.
   - **Rationale:** Penetration and ricochet make cover material tactically meaningful and create
     emergent combat scenarios.
   - **Verification:** Fire through a drywall surface and confirm the projectile passes through with
     reduced energy. Fire at concrete and confirm the projectile stops. Fire at a metal surface at a
     shallow angle and verify ricochet occurs. Disable penetration and confirm no pass-through.
9. **R-13.16.4d** — The engine **SHALL** support adjustable sight zeroing in discrete distance steps
   that offset the aim point to compensate for bullet drop, displayed in the scope UI, and
   interacting with ammo type ballistic profiles.
   - **Rationale:** Zeroing enables accurate first shots at known distances without manual holdover,
     critical for sniper-focused gameplay.
   - **Verification:** Zero a scoped weapon to 300 m and fire at a 300 m target. Verify point of aim
     matches point of impact. Fire at 100 m with 300 m zero and verify the round impacts high.
     Switch ammo type and verify the effective zero shifts due to different ballistics.
10. **R-13.16.5a** — The engine **SHALL** provide a data-driven slot system where weapons define
    named attachment slots with category filters, and attachments modify weapon stats additively or
    multiplicatively as configured, with slots and stats defined in gameplay databases.
    - **Rationale:** Data-driven slot definitions and stat modifiers let designers extend weapon
      customization without code changes.
    - **Verification:** Define a weapon with optic, muzzle, and grip slots. Equip a suppressor to
      the muzzle slot and verify sound and range decrease by configured amounts. Equip an extended
      magazine and verify capacity increases. Attempt to equip an optic in the grip slot and verify
      the category filter rejects it.
11. **R-13.16.5b** — The engine **SHALL** render equipped attachments at named socket transforms on
    weapon meshes, with optic attachments rendering scope reticles and barrel attachments replacing
    the muzzle flash VFX point.
    - **Rationale:** Visual attachment representation gives customization tangible in-game presence
      and reinforces player investment in weapon modification.
    - **Verification:** Equip a suppressor, optic, and grip. Verify each attachment mesh appears at
      its socket transform. Verify the optic renders its reticle at the configured zoom level.
      Remove an attachment and confirm the mesh disappears immediately.
12. **R-13.16.5c** — The engine **SHALL** provide a weapon customization screen with a 3D weapon
    preview, interactive slot indicators, filtered attachment lists from inventory, and before/after
    stat comparison on hover.
    - **Rationale:** A visual customization UI lets players make informed attachment decisions by
      previewing both visual and stat effects before committing.
    - **Verification:** Open the customization UI and verify the 3D weapon preview renders with
      current attachments. Select a slot and verify only compatible attachments appear. Hover an
      attachment and verify stat comparison shows before/after values. Equip and confirm the 3D
      preview updates in real time.
13. **R-13.16.6a** — The engine **SHALL** assign a surface type tag to all physics materials,
    resolve terrain surface type from the splatmap's dominant material weight, and include the
    surface type in all physics query results.
    - **Rationale:** Surface type tagging is the foundation that enables all downstream impact
      response systems to dispatch context-appropriate feedback.
    - **Verification:** Perform a raycast against metal, wood, and concrete surfaces. Verify each
      returns the correct surface type tag. Raycast terrain and confirm the tag matches the dominant
      splatmap material at the hit point. Add a custom surface type and verify it propagates through
      physics queries.
14. **R-13.16.6b** — The engine **SHALL** spawn particle effects on surface impact by looking up the
    VFX response table for the surface type, with configurable particle asset, spawn count, scale,
    and lifetime per surface type.
    - **Rationale:** Per-surface VFX provides immediate visual feedback that communicates impact
      context to the player.
    - **Verification:** Fire a projectile at metal, wood, and concrete. Verify sparks spawn for
      metal, splinters for wood, and dust for concrete as defined in the VFX response table. Verify
      particle count and lifetime match configured values.
15. **R-13.16.6c** — The engine **SHALL** play spatial audio on surface impact by looking up the
    audio response table for the surface type, with random variation from a sound pool and
    volume/pitch scaling with impact force.
    - **Rationale:** Per-surface audio reinforces the physical context of impacts and prevents
      repetitive sound when using random variation pools.
    - **Verification:** Fire at metal, wood, and concrete. Verify each produces the correct impact
      sound. Fire multiple rounds at the same surface and verify random variation produces different
      sounds from the pool. Verify louder impacts produce louder audio.
16. **R-13.16.6d** — The engine **SHALL** place decals on surface impact by looking up the decal
    response table for the surface type, projected onto the surface at the impact point and oriented
    along the surface normal, with configurable lifetime and maximum count.
    - **Rationale:** Impact decals leave visible environmental traces that communicate combat
      history and enhance immersion.
    - **Verification:** Fire at metal, wood, and flesh surfaces. Verify bullet hole, splinter hole,
      and blood spatter decals appear respectively. Verify decals are oriented along the surface
      normal. Exceed the maximum decal count and verify the oldest decals are removed.

## Non-Functional Requirements

| ID          | Derived From |
|-------------|--------------|
| NFR-13.16.1 |              |
| NFR-13.16.2 |              |

1. **NFR-13.16.1** — The advanced ballistic simulation **SHALL** process up to 256 simultaneous
   in-flight projectiles within 1ms per physics tick. Per-projectile penetration and ricochet
   calculations **SHALL** complete within 0.01ms each.
   - **Rationale:** High fire-rate weapons and multiplayer scenarios can produce large numbers of
     simultaneous projectiles that must be simulated without frame rate impact.
   - **Verification:** Spawn 256 simultaneous projectiles with gravity, drag, and penetration
     enabled. Measure total ballistic simulation time per physics tick. Verify it stays under 1ms.
2. **NFR-13.16.2** — Time from trigger input to visual muzzle flash, audio report, and recoil camera
   kick **SHALL** be under 16ms (1 frame at 60 fps). Reload animations and fire mode toggle
   **SHALL** respond within 1 frame of input.
   - **Rationale:** Weapon responsiveness is the most critical feel metric in shooter games. Any
     perceptible input-to-feedback latency degrades player experience.
   - **Verification:** Measure time from fire input event to muzzle flash render and audio playback
     start. Verify total latency is under 16ms. Measure fire mode toggle response time and verify it
     is within 1 frame.

## Projectile Archetypes and Interactions

| ID         | Derived From                                                 |
|------------|--------------------------------------------------------------|
| R-13.16.7  | [F-13.16.7](../../features/game-framework/weapon-system.md)  |
| R-13.16.8  | [F-13.16.8](../../features/game-framework/weapon-system.md)  |
| R-13.16.9  | [F-13.16.9](../../features/game-framework/weapon-system.md)  |
| R-13.16.10 | [F-13.16.10](../../features/game-framework/weapon-system.md) |

1. **R-13.16.7** — The engine **SHALL** provide named projectile archetypes (Bullet, Missile,
   Torpedo, RPG, Grenade, Arrow, Spell Bolt) as data assets with per-archetype physics profiles
   (homing, arcing, bouncing, embedding), visual behaviors (tracer, exhaust trail, elemental VFX),
   and detonation modes (contact, proximity, timed fuse).
   - **Rationale:** Named archetypes with preset physics give designers a library of proven
     projectile behaviors that cover the full spectrum of ranged weapon types across game genres.
   - **Verification:** Spawn each archetype and verify its trajectory matches the preset physics:
     Bullet travels linearly with minimal drop, Missile homes toward a locked target, Grenade arcs
     and bounces, Arrow arcs with gravity and embeds on hit. Verify each archetype's VFX (tracer,
     exhaust, elemental) renders correctly.
2. **R-13.16.8** — The engine **SHALL** support explosive projectile detonation with configurable
   blast radius, damage falloff curve (linear, quadratic, step), optional shrapnel sub-projectile
   spawning, environmental destruction triggering, and physics impulse applied to dynamic bodies
   within the blast radius.
   - **Rationale:** Explosive projectiles are the foundation for grenades, rockets, and spell AoE —
     the blast radius, falloff, and shrapnel create varied tactical outcomes from a single
     detonation event.
   - **Verification:** Detonate an explosive projectile and verify entities at the epicenter receive
     full damage. Verify entities at the blast edge receive reduced damage matching the configured
     falloff curve. Verify shrapnel sub-projectiles spawn radially and deal their own damage on hit.
     Verify destructible objects within the radius are destroyed. Verify dynamic rigid bodies
     receive physics impulse.
3. **R-13.16.9** — The engine **SHALL** despawn projectiles that exceed configurable maximum range
   or maximum time alive, with configurable despawn behavior (fade, explode, instant remove), and
   **SHALL** recycle despawned entities through archetype-specific pools to avoid allocation
   overhead.
   - **Rationale:** Unbounded projectile lifetimes would leak entities and degrade performance.
     Pooling amortizes allocation cost for high-fire-rate weapons producing hundreds of projectiles
     per second.
   - **Verification:** Fire a projectile and verify it despawns at the configured max range. Fire
     another and verify it despawns at the configured max time. Verify despawned entities are
     returned to the pool. Spawn a projectile and verify it reuses a pooled entity. Enable the debug
     overlay and verify active count, pool utilization, and despawn rate are displayed.
4. **R-13.16.10** — The engine **SHALL** support projectile-versus-projectile collision
   (shoot-down), projectile reflection off reflective barriers with ownership transfer, and
   projectile absorption by absorptive barriers with optional energy gain, all configurable per
   archetype and barrier type via gameplay tags.
   - **Rationale:** Projectile interaction systems enable counter-play mechanics (shooting down
     missiles, reflecting spells) that add strategic depth beyond basic hit/miss combat.
   - **Verification:** Fire a missile, then fire a bullet at it and verify both are destroyed on
     contact (shoot-down). Fire a projectile at a reflective barrier and verify it reverses
     direction with ownership transferred to the barrier owner. Fire a projectile at an absorptive
     barrier and verify it is destroyed with no effect applied. Verify visual/audio feedback plays
     for each interaction type.

## Projectile Non-Functional Requirements

| ID          | Derived From |
|-------------|--------------|
| NFR-13.16.3 | F-13.16.9    |
| NFR-13.16.4 | F-13.16.8    |

1. **NFR-13.16.3** — The projectile pooling system **SHALL** recycle entities with zero per-spawn
   allocation after the pool is warmed, and **SHALL** support at least 512 pooled projectiles per
   archetype without exceeding 2 MB of memory per pool.
   - **Rationale:** High-fire-rate weapons and explosive shrapnel can produce hundreds of
     projectiles per frame; allocation-free recycling is critical for consistent frame times.
   - **Verification:** Warm the pool by spawning and despawning 512 projectiles. Measure per-spawn
     allocation on subsequent spawns and verify zero allocations. Measure pool memory and verify it
     stays under 2 MB.
2. **NFR-13.16.4** — Explosive detonation processing (damage falloff calculation, shrapnel spawning,
   physics impulse application, destruction triggering) **SHALL** complete within 0.5 ms per
   detonation for up to 32 entities in the blast radius.
   - **Rationale:** Multiple simultaneous explosions (grenade spam, chain detonation) must not cause
     frame spikes that break gameplay responsiveness.
   - **Verification:** Detonate an explosive with 32 entities in the blast radius. Measure total
     processing time for damage, shrapnel, impulse, and destruction. Verify it completes within 0.5
     ms. Detonate 4 explosives simultaneously and verify total processing stays under 2 ms.
