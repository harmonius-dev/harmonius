# User Stories -- Weapon Systems (13.16)

## Fire Modes and Ammunition

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.16.1.1  | game designer (P-5)     |
| US-13.16.1.2  | player (P-23)           |
| US-13.16.1.3  | player (P-23)           |
| US-13.16.2.1  | game designer (P-5)     |
| US-13.16.2.2  | game developer (P-15)   |
| US-13.16.2.3  | player (P-23)           |
| US-13.16.2.4  | player (P-23)           |
| US-13.16.3.1  | game designer (P-5)     |
| US-13.16.3.2  | game developer (P-15)   |
| US-13.16.3.3  | player (P-23)           |

1. **US-13.16.1.1** -- **As a** game designer (P-5), **I want** to configure fire mode parameters
   (rounds per activation, inter-round delay, spread) per weapon as data assets, **so that** weapon
   feel is code-free.

2. **US-13.16.1.2** -- [game-specific] **As a** player (P-23), **I want** to toggle between
   semi-automatic, burst, and full-automatic fire modes, **so that** I adapt my firing to the
   situation.

3. **US-13.16.1.3** -- [game-specific] **As a** player (P-23), **I want** each fire mode to feel
   mechanically different in rounds per trigger press and recoil, **so that** modes have distinct
   tactical roles.

4. **US-13.16.2.1** -- **As a** game designer (P-5), **I want** to define ammo types with damage
   modifier, armor penetration, status effect, tracer color, and muzzle VFX in gameplay databases,
   **so that** ammunition is data-driven.

5. **US-13.16.2.2** -- **As a** game developer (P-15), **I want** magazine- based ammunition
   tracking with tactical/empty reload behaviors and sequential reload interruption, **so that**
   reload mechanics are consistent and configurable.

6. **US-13.16.2.3** -- [game-specific] **As a** player (P-23), **I want** to switch ammo types at
   runtime via an input action, **so that** I adapt to different enemy resistances mid-combat.

7. **US-13.16.2.4** -- [game-specific] **As a** player (P-23), **I want** magazine and reserve
   counts displayed in the HUD, **so that** I know my ammo status at a glance.

8. **US-13.16.3.1** -- **As a** game designer (P-5), **I want** to author recoil patterns as 2D
   curves in the visual editor per weapon, **so that** weapon feel is tunable without code.

9. **US-13.16.3.2** -- **As a** game developer (P-15), **I want** spread to widen with movement,
   hip-fire, and sustained fire, and tighten with ADS and crouching, **so that** accuracy reflects
   player positioning.

10. **US-13.16.3.3** -- [game-specific] **As a** player (P-23), **I want** the crosshair to
    dynamically reflect current spread radius, **so that** I visually judge my accuracy.

## Ballistics

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.16.4.1  | game designer (P-5)     |
| US-13.16.4.2  | game designer (P-5)     |
| US-13.16.4.3  | game developer (P-15)   |
| US-13.16.4.4  | player (P-23)           |
| US-13.16.4.5  | player (P-23)           |

1. **US-13.16.4.1** -- **As a** game designer (P-5), **I want** to configure mass, muzzle velocity,
   and drag per ammo type with gravity and drag independently disableable, **so that** ballistics
   supports both realistic and arcade modes.

2. **US-13.16.4.2** -- **As a** game designer (P-5), **I want** penetration and ricochet thresholds
   per physics material with independent enable toggles, **so that** surface response is
   data-driven.

3. **US-13.16.4.3** -- **As a** game developer (P-15), **I want** projectile physics computed per
   tick with parabolic trajectory, finite travel time, and wind deflection, **so that** ballistics
   are physically grounded.

4. **US-13.16.4.4** -- [game-specific] **As a** player (P-23), **I want** distant targets to require
   leading and elevation compensation, **so that** long-range marksmanship is skillful.

5. **US-13.16.4.5** -- [game-specific] **As a** player (P-23), **I want** to adjust my scope zeroing
   in discrete distance steps, **so that** point of aim matches point of impact at the zeroed range.

## Weapon Customization

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.16.5.1  | game designer (P-5)     |
| US-13.16.5.2  | game developer (P-15)   |
| US-13.16.5.3  | player (P-23)           |
| US-13.16.5.4  | player (P-23)           |

1. **US-13.16.5.1** -- **As a** game designer (P-5), **I want** to define slot categories,
   attachment stats, and modifier modes in gameplay databases, **so that** weapon customization is
   extensible without code.

2. **US-13.16.5.2** -- **As a** game developer (P-15), **I want** attachment meshes to snap to named
   socket transforms on the weapon with visual updates on equip/unequip, **so that** customization
   is visually reflected.

3. **US-13.16.5.3** -- [game-specific] **As a** player (P-23), **I want** weapons to have named
   attachment slots (optic, barrel, grip, stock), **so that** I customize my weapon's stats and
   appearance.

4. **US-13.16.5.4** -- [game-specific] **As a** player (P-23), **I want** a dedicated customization
   screen with 3D preview and stat comparison on hover, **so that** I evaluate attachments before
   equipping.

## Surface Response and Projectile Types

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.16.6.1  | game designer (P-5)     |
| US-13.16.6.2  | game developer (P-15)   |
| US-13.16.7.1  | game designer (P-5)     |
| US-13.16.7.2  | player (P-23)           |
| US-13.16.8.1  | game designer (P-5)     |
| US-13.16.8.2  | player (P-23)           |
| US-13.16.9.1  | game developer (P-15)   |
| US-13.16.10.1 | game designer (P-5)     |
| US-13.16.10.2 | player (P-23)           |

1. **US-13.16.6.1** -- **As a** game designer (P-5), **I want** to configure VFX, audio, and decal
   response tables mapping surface types to assets, **so that** impact feedback is data-driven.

2. **US-13.16.6.2** -- **As a** game developer (P-15), **I want** physics materials to carry a
   surface type tag resolved from splatmap layers on terrain, **so that** every surface has a
   classifiable material.

3. **US-13.16.7.1** -- **As a** game designer (P-5), **I want** named projectile archetypes (bullet,
   missile, grenade, arrow, spell bolt) with preset physics profiles and visual behaviors,
   **so that** diverse ranged weapons are data-driven.

4. **US-13.16.7.2** -- [game-specific] **As a** player (P-23), **I want** arrows to embed in targets
   on hit and missiles to home toward locked targets, **so that** projectile types feel mechanically
   distinct.

5. **US-13.16.8.1** -- **As a** game designer (P-5), **I want** to configure blast radius, damage
   falloff curve, and shrapnel count per explosive projectile, **so that** each explosive has a
   unique area damage profile.

6. **US-13.16.8.2** -- [game-specific] **As a** player (P-23), **I want** explosions to apply
   physics impulse that ragdolls enemies and scatters objects, **so that** explosive weapons feel
   powerful and impactful.

7. **US-13.16.9.1** -- **As a** game developer (P-15), **I want** projectile entity pooling with
   configurable max range, max time, and despawn behavior, **so that** high-fire-rate weapons avoid
   allocation overhead.

8. **US-13.16.10.1** -- **As a** game designer (P-5), **I want** to configure shoot-down,
   reflective, and absorptive projectile interaction types per archetype and barrier, **so that**
   counter-play is data-driven.

9. **US-13.16.10.2** -- [game-specific] **As a** player (P-23), **I want** to shoot down incoming
   missiles and reflect projectiles off my energy shield, **so that** I have active counter-play
   against ranged attacks.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-13.16.1 | game designer (P-5) |
| US-13.16.10 | game designer (P-5) |
| US-13.16.2 | game designer (P-5) |
| US-13.16.3 | game designer (P-5) |
| US-13.16.4 | game designer (P-5) |
| US-13.16.5 | game designer (P-5) |
| US-13.16.6 | game designer (P-5) |
| US-13.16.7 | game designer (P-5) |
| US-13.16.8 | game designer (P-5) |
| US-13.16.9 | game developer (P-15) |

1. **US-13.16.1** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.16.1.1 through US-13.16.1.3 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

2. **US-13.16.10** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.16.10.1 through US-13.16.10.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

3. **US-13.16.2** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.16.2.1 through US-13.16.2.4 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

4. **US-13.16.3** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.16.3.1 through US-13.16.3.3 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

5. **US-13.16.4** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.16.4.1 through US-13.16.4.5 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

6. **US-13.16.5** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.16.5.1 through US-13.16.5.4 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

7. **US-13.16.6** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.16.6.1 through US-13.16.6.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

8. **US-13.16.7** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.16.7.1 through US-13.16.7.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

9. **US-13.16.8** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.16.8.1 through US-13.16.8.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

10. **US-13.16.9** -- **As a** game developer (P-15), **I want** the capabilities defined in
    sub-stories
US-13.16.9.1 through US-13.16.9.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.
