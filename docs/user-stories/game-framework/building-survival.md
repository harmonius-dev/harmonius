# User Stories -- Building and Survival (13.14)

## Building Placement

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.14.1.1  | game designer (P-5)     |
| US-13.14.1.2  | game developer (P-15)   |
| US-13.14.1.3  | player (P-23)           |
| US-13.14.1.4  | player (P-23)           |
| US-13.14.1.5  | level designer (P-6)    |
| US-13.14.2.1  | game designer (P-5)     |
| US-13.14.2.2  | player (P-23)           |
| US-13.14.2.3  | player (P-23)           |
| US-13.14.3.1  | game designer (P-5)     |
| US-13.14.3.2  | game developer (P-15)   |
| US-13.14.3.3  | player (P-23)           |
| US-13.14.4.1  | game designer (P-5)     |
| US-13.14.4.2  | player (P-23)           |
| US-13.14.4.3  | player (P-23)           |

1. **US-13.14.1.1** -- **As a** game designer (P-5), **I want** to define attachment sockets,
   rotation increments, and placement rules per building piece as data assets, **so that** new
   pieces require no code changes.

2. **US-13.14.1.2** -- **As a** game developer (P-15), **I want** placement validity checks for
   collision, slope, and adjacency to be configurable per piece, **so that** building rules adapt to
   different game modes.

3. **US-13.14.1.3** -- [game-specific] **As a** player (P-23), **I want** building pieces to snap to
   valid sockets on adjacent pieces with green/red ghost previews, **so that** I align walls and
   roofs without fiddling.

4. **US-13.14.1.4** -- [game-specific] **As a** player (P-23), **I want** a free-form placement mode
   that aligns pieces to the ground using physics, **so that** I can build on uneven terrain without
   grid constraints.

5. **US-13.14.1.5** -- **As a** level designer (P-6), **I want** to restrict placement in designated
   zones using trigger volumes, **so that** players cannot build in performance-sensitive areas.

6. **US-13.14.2.1** -- **As a** game designer (P-5), **I want** to configure construction duration,
   required materials, visual stage thresholds, and refund percentages per piece, **so that**
   building economy is data-driven.

7. **US-13.14.2.2** -- [game-specific] **As a** player (P-23), **I want** placed buildings to start
   as scaffolds requiring resources and time to complete, **so that** construction feels like a
   meaningful investment.

8. **US-13.14.2.3** -- [game-specific] **As a** player (P-23), **I want** to pause, cancel, or
   accelerate construction with partial resource refunds, **so that** I have control over my
   building projects.

9. **US-13.14.3.1** -- **As a** game designer (P-5), **I want** to configure stability values,
   material bonuses, and cascade thresholds per piece, **so that** structural integrity is tunable
   per game mode.

10. **US-13.14.3.2** -- **As a** game developer (P-15), **I want** stability computed incrementally
    on piece add, remove, or damage rather than every frame, **so that** large structures do not
    degrade performance.

11. **US-13.14.3.3** -- [game-specific] **As a** player (P-23), **I want** unsupported structures to
    display cracks and wobble before collapsing, **so that** I receive visual warnings about
    instability.

12. **US-13.14.4.1** -- **As a** game designer (P-5), **I want** upgrade tiers, material costs,
    decay rates, and repair costs per material in gameplay databases, **so that** the building
    economy is balanced.

13. **US-13.14.4.2** -- [game-specific] **As a** player (P-23), **I want** to upgrade building
    pieces from wood to stone to metal by paying material costs, **so that** I can strengthen my
    base over time.

14. **US-13.14.4.3** -- [game-specific] **As a** player (P-23), **I want** to repair damaged pieces
    by consuming materials proportional to the damage, **so that** I can restore my base after an
    attack.

## Housing and Furniture

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.14.5.1  | game designer (P-5)     |
| US-13.14.5.2  | game developer (P-15)   |
| US-13.14.5.3  | player (P-23)           |
| US-13.14.5.4  | player (P-23)           |
| US-13.14.5.5  | level designer (P-6)    |

1. **US-13.14.5.1** -- **As a** game designer (P-5), **I want** to define functional effects per
   furniture type in gameplay databases, **so that** new furniture benefits require no code.

2. **US-13.14.5.2** -- **As a** game developer (P-15), **I want** housing instances to support
   visitor permissions and persist through the save system, **so that** player homes are a permanent
   world feature.

3. **US-13.14.5.3** -- [game-specific] **As a** player (P-23), **I want** to claim a housing plot
   and set visitor permissions to public, friends, guild, or private, **so that** I control who
   enters my home.

4. **US-13.14.5.4** -- [game-specific] **As a** player (P-23), **I want** beds to set my respawn
   point, storage chests to extend inventory, and crafting stations to enable profession crafting,
   **so that** furniture provides gameplay benefits.

5. **US-13.14.5.5** -- **As a** level designer (P-6), **I want** to place housing zones in the world
   with configurable boundaries, **so that** player housing integrates with the world layout.

## Survival Systems

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.14.6.1  | game designer (P-5)     |
| US-13.14.6.2  | game developer (P-15)   |
| US-13.14.6.3  | player (P-23)           |
| US-13.14.6.4  | player (P-23)           |
| US-13.14.6.5  | player (P-23)           |
| US-13.14.7.1  | game designer (P-5)     |
| US-13.14.7.2  | game developer (P-15)   |
| US-13.14.7.3  | player (P-23)           |
| US-13.14.8.1  | game designer (P-5)     |
| US-13.14.8.2  | player (P-23)           |
| US-13.14.8.3  | player (P-23)           |
| US-13.14.9.1  | game designer (P-5)     |
| US-13.14.9.2  | player (P-23)           |
| US-13.14.9.3  | player (P-23)           |

1. **US-13.14.6.1** -- **As a** game designer (P-5), **I want** to configure drain rates,
   restoration values, and debuff thresholds per vital in gameplay databases, **so that** survival
   difficulty is data-driven.

2. **US-13.14.6.2** -- **As a** game developer (P-15), **I want** vital meters to integrate with the
   stat and gameplay effect systems, **so that** hunger, thirst, warmth, and stamina reuse the
   engine's resource pool primitives.

3. **US-13.14.6.3** -- [game-specific] **As a** player (P-23), **I want** hunger to drain at a base
   rate accelerated by activity and eating to restore it with optional buffs, **so that** food
   quality matters.

4. **US-13.14.6.4** -- [game-specific] **As a** player (P-23), **I want** clothing insulation and
   fire proximity to protect against cold, **so that** I can prepare for harsh environments.

5. **US-13.14.6.5** -- [game-specific] **As a** player (P-23), **I want** critically low vitals to
   impose debuffs like reduced HP or slowed movement, **so that** neglecting survival needs has
   consequences.

6. **US-13.14.7.1** -- **As a** game designer (P-5), **I want** to define resource type, gather
   time, tool requirement, node HP, respawn timer, and rare yield chance per node as data,
   **so that** new nodes require no code.

7. **US-13.14.7.2** -- **As a** game developer (P-15), **I want** node placement to integrate with
   procedural generation for biome-specific distribution, **so that** resource scarcity scales with
   world design.

8. **US-13.14.7.3** -- [game-specific] **As a** player (P-23), **I want** gathering to trigger an
   interaction with a looped animation until the node depletes, **so that** gathering feels physical
   and interruptible.

9. **US-13.14.8.1** -- **As a** game designer (P-5), **I want** to configure growth stages, watering
   requirements, seasonal rules, and fertilizer effects per crop type, **so that** farming is
   balanced via data.

10. **US-13.14.8.2** -- [game-specific] **As a** player (P-23), **I want** to till soil, plant
    seeds, water crops, and harvest produce through a multi-stage growth loop, **so that** farming
    is a rewarding activity.

11. **US-13.14.8.3** -- [game-specific] **As a** player (P-23), **I want** seasonal constraints to
    restrict which crops grow in which seasons, **so that** crop planning has strategic depth.

12. **US-13.14.9.1** -- **As a** game designer (P-5), **I want** to configure production rates,
    happiness decay, and genetic rules per animal species, **so that** animal husbandry is
    data-driven.

13. **US-13.14.9.2** -- [game-specific] **As a** player (P-23), **I want** domesticated animals to
    have hunger and happiness meters that affect production rate, **so that** caring for animals is
    rewarded.

14. **US-13.14.9.3** -- [game-specific] **As a** player (P-23), **I want** to breed compatible
    animal pairs and see offspring with inherited trait variations, **so that** selective breeding
    is rewarding.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-13.14.1 | game designer (P-5) |
| US-13.14.2 | game designer (P-5) |
| US-13.14.3 | game designer (P-5) |
| US-13.14.4 | game designer (P-5) |
| US-13.14.5 | game designer (P-5) |
| US-13.14.6 | game designer (P-5) |
| US-13.14.7 | game designer (P-5) |
| US-13.14.8 | game designer (P-5) |
| US-13.14.9 | game designer (P-5) |

1. **US-13.14.1** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.14.1.1 through US-13.14.1.5 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

2. **US-13.14.2** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.14.2.1 through US-13.14.2.3 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

3. **US-13.14.3** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.14.3.1 through US-13.14.3.3 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

4. **US-13.14.4** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.14.4.1 through US-13.14.4.3 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

5. **US-13.14.5** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.14.5.1 through US-13.14.5.5 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

6. **US-13.14.6** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.14.6.1 through US-13.14.6.5 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

7. **US-13.14.7** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.14.7.1 through US-13.14.7.3 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

8. **US-13.14.8** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.14.8.1 through US-13.14.8.3 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

9. **US-13.14.9** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.14.9.1 through US-13.14.9.3 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.
