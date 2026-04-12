# User Stories — Character Progression (13.12)

## Archetype Templates

| ID            | Persona              |
|---------------|----------------------|
| US-13.12.1a.1 | game designer (P-5)  |
| US-13.12.1b.1 | game designer (P-5)  |
| US-13.12.1c.1 | player (P-23)        |
| US-13.12.1c.2 | game developer (P-15)|
| US-13.12.1d.1 | player (P-23)        |

1. **US-13.12.1a.1** — **As a** game designer (P-5), **I want** data-driven archetype template
   assets specifying stat modifiers, abilities, and cosmetic constraints, **so that** character
   archetypes are authored without code.
2. **US-13.12.1b.1** — **As a** game designer (P-5), **I want** archetype templates defining
   starting abilities, level-up unlocks, and equipment restrictions, **so that** play style
   differentiation is data-driven.
3. **US-13.12.1c.1** — [game-specific] **As a** player (P-23), **I want** to switch archetypes at
   designated NPCs preserving previous progress, **so that** I can respec without losing work.
4. **US-13.12.1c.2** — **As a** game developer (P-15), **I want** archetype switching as a state
   transition preserving ECS component history, **so that** respec logic is generic.
5. **US-13.12.1d.1** — [game-specific] **As a** player (P-23), **I want** a prestige/rebirth system
   with permanent bonuses across cycles, **so that** I have long-term goals beyond max level.

## DAG Progression Graph

| ID            | Persona              |
|---------------|----------------------|
| US-13.12.2a.1 | game designer (P-5)  |
| US-13.12.2b.1 | game designer (P-5)  |
| US-13.12.2b.2 | player (P-23)        |
| US-13.12.2c.1 | game designer (P-5)  |

1. **US-13.12.2a.1** — **As a** game designer (P-5), **I want** talent trees as directed acyclic
   graphs with passive bonuses, active abilities, and prerequisite edges, **so that** progression
   paths are structured.
2. **US-13.12.2b.1** — **As a** game designer (P-5), **I want** point allocation with respec at
   currency cost, **so that** builds are changeable.
3. **US-13.12.2b.2** — [game-specific] **As a** player (P-23), **I want** to spend talent points on
   unlock nodes and respec to try different builds, **so that** experimentation is encouraged.
4. **US-13.12.2c.1** — **As a** game designer (P-5), **I want** a visual graph editor for talent
   trees with drag-and-drop nodes and edge drawing, **so that** talent authoring is visual.

## Professions and Crafting

| ID            | Persona              |
|---------------|----------------------|
| US-13.12.3a.1 | game designer (P-5)  |
| US-13.12.3b.1 | game designer (P-5)  |
| US-13.12.3c.1 | player (P-23)        |
| US-13.12.4.1  | game designer (P-5)  |
| US-13.12.4.2  | player (P-23)        |

1. **US-13.12.3a.1** — **As a** game designer (P-5), **I want** profession definitions with skill
   level, XP curves, and recipe thresholds, **so that** professions are data-driven.
2. **US-13.12.3b.1** — **As a** game designer (P-5), **I want** gathering professions to interact
   with world resource nodes with yield scaling, **so that** gathering integrates with the world.
3. **US-13.12.3c.1** — [game-specific] **As a** player (P-23), **I want** crafting to consume
   materials through recipes gated by profession level, **so that** crafting feels rewarding.
4. **US-13.12.4.1** — **As a** game designer (P-5), **I want** in-world crafting stations with
   quality tiers gating recipe access, **so that** location matters for crafting.
5. **US-13.12.4.2** — [game-specific] **As a** player (P-23), **I want** the station UI to show
   available recipes filtered by level with material counts, **so that** I know what I can craft.

## Reputation and Achievements

| ID            | Persona              |
|---------------|----------------------|
| US-13.12.5.1  | game designer (P-5)  |
| US-13.12.5.2  | player (P-23)        |
| US-13.12.6a.1 | game designer (P-5)  |
| US-13.12.6b.1 | player (P-23)        |
| US-13.12.6c.1 | game developer (P-15)|

1. **US-13.12.5.1** — **As a** game designer (P-5), **I want** per-faction reputation meters with
   tiered standings gating vendors, quests, and zone access, **so that** faction relationships drive
   content.
2. **US-13.12.5.2** — [game-specific] **As a** player (P-23), **I want** reputation from kills,
   quests, and turn-ins with asymmetric faction relationships, **so that** reputation has strategic
   depth.
3. **US-13.12.6a.1** — **As a** game designer (P-5), **I want** data-driven achievement definitions
   with trigger conditions and progress tracking, **so that** achievements are configured without
   code.
4. **US-13.12.6b.1** — **As a** player (P-23), **I want** completed achievements to grant rewards
   and a notification popup, **so that** milestones feel celebratory.
5. **US-13.12.6c.1** — **As a** game developer (P-15), **I want** platform achievement sync via
   platform services, **so that** engine achievements map to Steam/PlayStation/Xbox.

## Item Enhancement

| ID            | Persona              |
|---------------|----------------------|
| US-13.12.7.1  | game designer (P-5)  |
| US-13.12.8a.1 | game designer (P-5)  |
| US-13.12.8b.1 | game designer (P-5)  |
| US-13.12.8c.1 | player (P-23)        |
| US-13.12.9.1  | game designer (P-5)  |
| US-13.12.9.2  | player (P-23)        |
| US-13.12.10.1 | game designer (P-5)  |

1. **US-13.12.7.1** — **As a** game designer (P-5), **I want** an enhancement system with
   configurable success probability and failure consequences per level, **so that** item progression
   has risk.
2. **US-13.12.8a.1** — **As a** game designer (P-5), **I want** item rarity tiers with stat ranges
   and color-coded display, **so that** loot quality is immediately visible.
3. **US-13.12.8b.1** — **As a** game designer (P-5), **I want** random prefix/suffix affixes from
   affix pools scaling with rarity, **so that** items have emergent variety.
4. **US-13.12.8c.1** — [game-specific] **As a** player (P-23), **I want** to re-roll affixes at
   currency cost while preserving the base item, **so that** I can improve gear incrementally.
5. **US-13.12.9.1** — **As a** game designer (P-5), **I want** equipment set definitions with
   escalating bonuses per piece count, **so that** collecting sets is strategic.
6. **US-13.12.9.2** — [game-specific] **As a** player (P-23), **I want** the character panel to show
   set progress with collected vs. missing pieces, **so that** I know what I need.
7. **US-13.12.10.1** — **As a** game designer (P-5), **I want** durability per item with
   configurable drain rates and repair costs, **so that** equipment maintenance is an economic sink.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-13.12.10 | game designer (P-5) |
| US-13.12.4 | game designer (P-5) |
| US-13.12.5 | game designer (P-5) |
| US-13.12.7 | game designer (P-5) |
| US-13.12.9 | game designer (P-5) |

1. **US-13.12.10** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.12.10.1 through US-13.12.10.1 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

2. **US-13.12.4** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.12.4.1 through US-13.12.4.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

3. **US-13.12.5** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.12.5.1 through US-13.12.5.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

4. **US-13.12.7** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.12.7.1 through US-13.12.7.1 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

5. **US-13.12.9** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.12.9.1 through US-13.12.9.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.
