# User Stories — Abilities and Combat (13.10)

## Ability Definition and Composition

| ID           | Persona              |
|--------------|----------------------|
| US-13.10.1.1 | game designer (P-5)  |
| US-13.10.1.2 | game designer (P-5)  |
| US-13.10.1.3 | game developer (P-15)|
| US-13.10.1.4 | player (P-23)        |
| US-13.10.1.5 | engine developer (P-26)|

1. **US-13.10.1.1** — **As a** game designer (P-5), **I want** to compose abilities from reusable
   building blocks such as activation conditions, costs, targeting, and effects, **so that** I share
   common components across many abilities.
2. **US-13.10.1.2** — **As a** game designer (P-5), **I want** to author abilities as visual assets
   in the editor without writing code, **so that** new abilities are created entirely through data.
3. **US-13.10.1.3** — **As a** game developer (P-15), **I want** the ability composition API to
   expose typed slots for each building block, **so that** I can extend the system with custom block
   types via the plugin API.
4. **US-13.10.1.4** — **As a** player (P-23), **I want** abilities to produce coordinated animation,
   VFX, and audio when activated, **so that** ability use feels impactful.
5. **US-13.10.1.5** — **As a** engine developer (P-26), **I want** ability assets validated at load
   time against their schema, **so that** incomplete or malformed abilities are caught before
   runtime.

## Ability Activation and Input Integration

| ID           | Persona              |
|--------------|----------------------|
| US-13.10.2.1 | game designer (P-5)  |
| US-13.10.2.2 | game designer (P-5)  |
| US-13.10.2.3 | game developer (P-15)|
| US-13.10.2.4 | player (P-23)        |
| US-13.10.2.5 | player (P-23)        |

1. **US-13.10.2.1** — **As a** game designer (P-5), **I want** to configure activation modes (press,
   hold, charge, combo, toggle) per ability, **so that** each ability responds to input in its
   intended way.
2. **US-13.10.2.2** — **As a** game designer (P-5), **I want** activation to validate cooldowns,
   resource costs, and cast conditions before execution, **so that** invalid inputs are rejected
   with clear feedback.
3. **US-13.10.2.3** — **As a** game developer (P-15), **I want** AI agents to activate abilities
   through the same API as player input using synthetic events, **so that** NPC and player ability
   execution is identical.
4. **US-13.10.2.4** — **As a** player (P-23), **I want** to execute combo chains by pressing the
   ability button within timing windows, **so that** skilled play is rewarded with extended combos.
5. **US-13.10.2.5** — **As a** player (P-23), **I want** a clear message when an ability cannot
   activate due to cooldown or missing resources, **so that** I understand why the action failed.

## Gameplay Effect System

| ID           | Persona              |
|--------------|----------------------|
| US-13.10.3.1 | game designer (P-5)  |
| US-13.10.3.2 | game designer (P-5)  |
| US-13.10.3.3 | game developer (P-15)|
| US-13.10.3.4 | player (P-23)        |

1. **US-13.10.3.1** — **As a** game designer (P-5), **I want** to define instant, duration,
   periodic, and permanent effects with configurable stacking rules, **so that** effect behavior
   matches my balance design.
2. **US-13.10.3.2** — **As a** game designer (P-5), **I want** to attach metadata (source entity,
   damage type, hierarchical tags) to effects, **so that** cross-effect interactions like fire
   removing frozen are data-driven.
3. **US-13.10.3.3** — **As a** game developer (P-15), **I want** the effect pipeline to expose hooks
   for custom stacking rules, **so that** I can implement game-specific modifier logic.
4. **US-13.10.3.4** — **As a** player (P-23), **I want** active effects displayed as icons with
   duration timers in the HUD, **so that** I know what buffs and debuffs affect my character.

## Melee Combat System

| ID           | Persona              |
|--------------|----------------------|
| US-13.10.4.1 | game designer (P-5)  |
| US-13.10.4.2 | game developer (P-15)|
| US-13.10.4.3 | player (P-23)        |
| US-13.10.4.4 | player (P-23)        |

1. **US-13.10.4.1** — **As a** game designer (P-5), **I want** to define weapon hitboxes as
   collision shapes attached to skeleton bones activated during attack windows, **so that** melee
   hit detection matches weapon motion.
2. **US-13.10.4.2** — **As a** game developer (P-15), **I want** the combat system to expose hit
   events with damage type, hit bone, and direction, **so that** I can build game-specific hit
   reactions.
3. **US-13.10.4.3** — **As a** player (P-23), **I want** melee attacks to produce weapon trails,
   impact VFX, and hit-stop on impact, **so that** combat feels weighty and responsive.
4. **US-13.10.4.4** — [game-specific] **As a** player (P-23), **I want** blocking, parrying, and
   dodge to grant invulnerability frames during active windows, **so that** defensive play is
   rewarded with precise timing.

## Ranged Combat and Projectile System

| ID           | Persona              |
|--------------|----------------------|
| US-13.10.5.1 | game designer (P-5)  |
| US-13.10.5.2 | game developer (P-15)|
| US-13.10.5.3 | player (P-23)        |
| US-13.10.5.4 | player (P-23)        |

1. **US-13.10.5.1** — **As a** game designer (P-5), **I want** to configure projectile types
   (linear, arced, homing, beam, spread) with speed and gravity parameters, **so that** ranged
   combat offers diverse weapon feel.
2. **US-13.10.5.2** — **As a** game developer (P-15), **I want** projectiles to be ECS entities with
   standard physics components, **so that** I can extend projectile behavior with custom systems.
3. **US-13.10.5.3** — **As a** player (P-23), **I want** projectiles to travel with realistic
   physics and apply damage effects on impact with VFX feedback, **so that** ranged attacks feel
   satisfying.
4. **US-13.10.5.4** — **As a** player (P-23), **I want** aim assist on gamepad with configurable
   magnetism and friction, **so that** ranged combat is accessible across input devices.

## Hitbox and Hurtbox System

| ID           | Persona              |
|--------------|----------------------|
| US-13.10.6.1 | game designer (P-5)  |
| US-13.10.6.2 | game developer (P-15)|
| US-13.10.6.3 | player (P-23)        |

1. **US-13.10.6.1** — **As a** game designer (P-5), **I want** to author hitbox and hurtbox shapes
   visually on the skeleton in the animation editor, **so that** hit regions are defined without
   code.
2. **US-13.10.6.2** — **As a** game developer (P-15), **I want** per-bone hurtboxes with damage
   multipliers and a debug overlay to visualize them at runtime, **so that** I can verify hit
   detection matches animations.
3. **US-13.10.6.3** — **As a** player (P-23), **I want** hit detection to feel fair in multiplayer
   with lag compensation, **so that** hits register where my crosshair was aimed regardless of
   latency.

## Area-of-Effect Targeting

| ID           | Persona              |
|--------------|----------------------|
| US-13.10.7.1 | game designer (P-5)  |
| US-13.10.7.2 | game designer (P-5)  |
| US-13.10.7.3 | player (P-23)        |

1. **US-13.10.7.1** — **As a** game designer (P-5), **I want** to configure AoE shapes (cone,
   sphere, line, ring) with radius and range parameters, **so that** each AoE ability has a distinct
   area profile.
2. **US-13.10.7.2** — **As a** game designer (P-5), **I want** a per-ability friendly fire toggle,
   **so that** I can balance cooperative and competitive AoE independently.
3. **US-13.10.7.3** — **As a** player (P-23), **I want** to see a ground reticle previewing the
   affected area before confirming placement, **so that** I can aim area attacks precisely.

## Periodic Effects (HoT and DoT)

| ID           | Persona              |
|--------------|----------------------|
| US-13.10.8.1 | game designer (P-5)  |
| US-13.10.8.2 | game designer (P-5)  |
| US-13.10.8.3 | player (P-23)        |
| US-13.10.8.4 | player (P-23)        |

1. **US-13.10.8.1** — **As a** game designer (P-5), **I want** to configure periodic effect tick
   rate, duration, and stacking rules (refresh, stack count, pandemic), **so that** each periodic
   effect has distinct behavior.
2. **US-13.10.8.2** — **As a** game designer (P-5), **I want** periodic effects to snapshot caster
   stats at application time, **so that** tick values remain predictable even after temporary buffs
   expire.
3. **US-13.10.8.3** — **As a** player (P-23), **I want** active periodic effects shown as
   buff/debuff icons with duration timers and stack counts in the HUD, **so that** I know how much
   healing or damage is active.
4. **US-13.10.8.4** — **As a** player (P-23), **I want** cleanse abilities to remove periodic
   effects matching specific damage type tags, **so that** I can counter effects strategically.

## Status Ailments

| ID            | Persona              |
|---------------|----------------------|
| US-13.10.10.1 | game designer (P-5)  |
| US-13.10.10.2 | game designer (P-5)  |
| US-13.10.10.3 | player (P-23)        |

1. **US-13.10.10.1** — **As a** game designer (P-5), **I want** to configure duration, post-immunity
   window, and diminishing returns per status ailment, **so that** crowd control is tunable per
   effect type.
2. **US-13.10.10.2** — **As a** game designer (P-5), **I want** named status conditions (stun, slow,
   root, silence) to be defined as data-driven hierarchical-tag effects, **so that** I can create
   new ailments without code.
3. **US-13.10.10.3** — **As a** player (P-23), **I want** status ailment debuff icons with remaining
   duration on my HUD, **so that** I know when crowd control will expire.

## Usable Items

| ID            | Persona              |
|---------------|----------------------|
| US-13.10.11.1 | game designer (P-5)  |
| US-13.10.11.2 | player (P-23)        |
| US-13.10.11.3 | player (P-23)        |

1. **US-13.10.11.1** — **As a** game designer (P-5), **I want** to configure charge count, cooldown,
   shared cooldown group, and cast time per usable item, **so that** each consumable has distinct
   usage constraints.
2. **US-13.10.11.2** — **As a** player (P-23), **I want** to use a healing potion from my quick-slot
   and see my health restore, **so that** consumables provide immediate gameplay benefit.
3. **US-13.10.11.3** — **As a** player (P-23), **I want** potions to share a cooldown group,
   **so that** I cannot chain-consume multiple potions instantly.

## Ability Conditions

| ID            | Persona              |
|---------------|----------------------|
| US-13.10.12.1 | game designer (P-5)  |
| US-13.10.12.2 | game designer (P-5)  |
| US-13.10.12.3 | player (P-23)        |

1. **US-13.10.12.1** — **As a** game designer (P-5), **I want** to configure ability prerequisites
   (weapon type, stance, resource threshold, buff, talent) with AND/OR/NOT logic, **so that**
   complex activation rules are data-driven.
2. **US-13.10.12.2** — **As a** game designer (P-5), **I want** the system to report the specific
   failing condition to the UI when activation is denied, **so that** players get clear feedback.
3. **US-13.10.12.3** — **As a** player (P-23), **I want** to see a message like "Requires a sword"
   when I try to use an ability without meeting its prerequisites, **so that** I know what to
   change.

## Combo System

| ID            | Persona              |
|---------------|----------------------|
| US-13.10.13.1 | game designer (P-5)  |
| US-13.10.13.2 | game designer (P-5)  |
| US-13.10.13.3 | player (P-23)        |

1. **US-13.10.13.1** — **As a** game designer (P-5), **I want** to define sequential combo chains
   with per-step timing windows and branching paths, **so that** combos offer meaningful input
   choices.
2. **US-13.10.13.2** — **As a** game designer (P-5), **I want** configurable combo point generation
   per step and finisher abilities that consume points with scaling effects, **so that** I can tune
   combo reward independently.
3. **US-13.10.13.3** — **As a** player (P-23), **I want** to chain abilities in sequence within
   timing windows for escalating damage, **so that** skilled play is rewarded.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-13.10.1 | game designer (P-5) |
| US-13.10.10 | game designer (P-5) |
| US-13.10.11 | game designer (P-5) |
| US-13.10.12 | game designer (P-5) |
| US-13.10.13 | game designer (P-5) |
| US-13.10.2 | game designer (P-5) |
| US-13.10.3 | game designer (P-5) |
| US-13.10.4 | game designer (P-5) |
| US-13.10.5 | game designer (P-5) |
| US-13.10.6 | game designer (P-5) |
| US-13.10.7 | game designer (P-5) |
| US-13.10.8 | game designer (P-5) |

1. **US-13.10.1** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.10.1.1 through US-13.10.1.5 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

2. **US-13.10.10** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.10.10.1 through US-13.10.10.3 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

3. **US-13.10.11** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.10.11.1 through US-13.10.11.3 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

4. **US-13.10.12** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.10.12.1 through US-13.10.12.3 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

5. **US-13.10.13** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.10.13.1 through US-13.10.13.3 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

6. **US-13.10.2** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.10.2.1 through US-13.10.2.5 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

7. **US-13.10.3** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.10.3.1 through US-13.10.3.4 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

8. **US-13.10.4** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.10.4.1 through US-13.10.4.4 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

9. **US-13.10.5** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.10.5.1 through US-13.10.5.4 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

10. **US-13.10.6** -- **As a** game designer (P-5), **I want** the capabilities defined in
    sub-stories
US-13.10.6.1 through US-13.10.6.3 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

11. **US-13.10.7** -- **As a** game designer (P-5), **I want** the capabilities defined in
    sub-stories
US-13.10.7.1 through US-13.10.7.3 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

12. **US-13.10.8** -- **As a** game designer (P-5), **I want** the capabilities defined in
    sub-stories
US-13.10.8.1 through US-13.10.8.4 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.
