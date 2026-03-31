# R-13.16 -- Weapon Systems Requirements

## State Mode Selector

1. **R-13.16.1** -- The engine **SHALL** provide per-entity configurable state modes with distinct
   parameters (rounds per activation, inter-round delay, spread modifier, recoil modifier)
   selectable at runtime via input action.
   - **Rationale:** A state mode selector is the engine primitive for fire modes, stance systems,
     and any togglable behavior set.
   - **Verification:** Configure three modes on an entity and verify toggling cycles through them
     with correct per-mode parameters.

## Consumable Resource Pool

1. **R-13.16.2** -- The engine **SHALL** provide a consumable resource pool with capacity, current
   count, reserve pool, consumption per activation, and configurable reload behaviors (tactical,
   empty, sequential with interrupt).
   - **Rationale:** A consumable resource pool is the engine primitive for ammunition, charges,
     fuel, and similar expendable resources.
   - **Verification:** Deplete the pool and verify activation is blocked until reload completes.
     Interrupt a sequential reload and verify partial fill is preserved.

## Spread and Recoil

1. **R-13.16.3** -- The engine **SHALL** apply per-entity recoil patterns defined as 2D curves with
   recovery, and dynamic spread influenced by movement, posture, and sustained activation, with
   crosshair reflecting current spread radius.
   - **Rationale:** Recoil patterns and dynamic spread provide skill- based accuracy control as an
     engine primitive.
   - **Verification:** Fire sustained bursts and verify the recoil pattern repeats identically.
     Verify spread widens with movement and tightens with crouching.

## Projectile Physics

1. **R-13.16.4** -- The engine **SHALL** compute projectile trajectories per physics tick with
   configurable gravity, drag, wind deflection, material-based penetration, ricochet, and sight
   zeroing, with each parameter independently disableable.
   - **Rationale:** Modular projectile physics supports both realistic and arcade ballistics from
     the same system.
   - **Verification:** Fire with zero drag and verify arrival time matches muzzle velocity. Enable
     wind and verify lateral deflection at range. Zero to 200 m and verify point-of-aim matches
     point-of-impact at that distance.

## Attachment Slot System

1. **R-13.16.5** -- The engine **SHALL** provide a data-driven slot system with named slots,
   category-filtered attachments, additive or multiplicative stat modifiers, visual socket snapping,
   and a customization UI with 3D preview and stat comparison.
   - **Rationale:** An attachment slot system is the engine primitive for any entity customization
     with modular stat modifiers.
   - **Verification:** Attempt to place an optic in a grip slot and verify rejection. Equip an
     attachment and verify stat modifiers apply and the 3D preview updates.

## Surface Response Dispatch

1. **R-13.16.6** -- The engine **SHALL** classify surfaces by material type tag (on physics
   materials and terrain splatmaps) and dispatch VFX, audio, and decal responses from configurable
   response tables per surface type on impact.
   - **Rationale:** Surface response dispatch is the engine primitive for impact feedback across all
     collision-producing systems.
   - **Verification:** Impact a metal surface and verify the metal VFX, audio, and decal from the
     response table spawn at the correct position and orientation.

## Projectile Archetypes

1. **R-13.16.7** -- The engine **SHALL** provide named projectile archetypes with preset physics
   profiles, visual behaviors (tracer, exhaust trail, elemental VFX), and configurable per-archetype
   parameters (homing, fuse mode, bounce, embed-on-hit).
   - **Rationale:** Projectile archetypes are the engine primitive for diverse ranged attacks
     without per-weapon physics tuning.
   - **Verification:** Fire each archetype and verify trajectory matches the preset physics profile.

2. **R-13.16.8** -- The engine **SHALL** support area damage on projectile detonation with
   configurable blast radius, damage falloff curve, optional shrapnel sub-projectiles, environmental
   destruction (F-11.5.1), and physics impulse to dynamic bodies.
   - **Rationale:** Area damage with destruction and physics impulse is the engine primitive for
     explosive weapons and abilities.
   - **Verification:** Detonate at epicenter and verify full damage. Verify entities at the edge
     receive reduced damage matching the falloff curve.

3. **R-13.16.9** -- The engine **SHALL** manage projectile lifetimes with configurable max range,
   max time, despawn behavior, and entity pooling with per-archetype pool sizes.
   - **Rationale:** Pooling with lifetime limits prevents allocation overhead from high-fire-rate
     entities.
   - **Verification:** Fire a projectile and verify despawn at max range. Verify the entity is
     returned to the pool and reused on the next spawn.

4. **R-13.16.10** -- The engine **SHALL** support projectile-vs- projectile and
   projectile-vs-barrier interactions (shoot-down, reflection with ownership transfer, absorption
   with energy gain) configurable per archetype and barrier type.
   - **Rationale:** Projectile interactions enable counter-play mechanics as an engine primitive.
   - **Verification:** Fire at a reflective barrier and verify the projectile reverses direction
     with ownership transferred.
