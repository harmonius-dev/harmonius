# User Stories — NPC Simulation (13.19)

## Relationship and Affinity

| ID           | Persona              |
|--------------|----------------------|
| US-13.19.1.1 | game designer (P-5)  |
| US-13.19.1.2 | game developer (P-15)|
| US-13.19.1.3 | player (P-23)        |
| US-13.19.2.1 | game designer (P-5)  |
| US-13.19.2.2 | player (P-23)        |

1. **US-13.19.1.1** — **As a** game designer (P-5), **I want** per-NPC numeric affinity values
   affected by player actions, **so that** relationships evolve through gameplay.
2. **US-13.19.1.2** — **As a** game developer (P-15), **I want** affinity values stored as ECS
   components persisted through the save system, **so that** relationships survive sessions.
3. **US-13.19.1.3** — [game-specific] **As a** player (P-23), **I want** relationship tiers to
   unlock new dialogue, quests, and perks, **so that** befriending NPCs is rewarding.
4. **US-13.19.2.1** — **As a** game designer (P-5), **I want** per-NPC personality traits and
   dynamic emotional states, **so that** NPC reactions vary by character.
5. **US-13.19.2.2** — [game-specific] **As a** player (P-23), **I want** NPCs to visibly express
   emotions (happy, angry, afraid) through animations, **so that** the world feels alive.

## Memory and Reputation

| ID            | Persona              |
|---------------|----------------------|
| US-13.19.3a.1 | game designer (P-5)  |
| US-13.19.3b.1 | game designer (P-5)  |
| US-13.19.3c.1 | player (P-23)        |

1. **US-13.19.3a.1** — **As a** game designer (P-5), **I want** NPCs to witness and judge player
   actions within perception range with time-decaying memory, **so that** consequences are organic.
2. **US-13.19.3b.1** — **As a** game designer (P-5), **I want** deed memories shared between NPCs as
   gossip with degrading accuracy, **so that** reputation propagates organically.
3. **US-13.19.3c.1** — [game-specific] **As a** player (P-23), **I want** helping one NPC to
   gradually improve standing with their village, **so that** gossip creates emergent reputation.

## Daily Schedules

| ID            | Persona              |
|---------------|----------------------|
| US-13.19.4a.1 | game designer (P-5)  |
| US-13.19.4b.1 | game developer (P-15)|
| US-13.19.4c.1 | player (P-23)        |

1. **US-13.19.4a.1** — **As a** game designer (P-5), **I want** per-NPC schedule tables mapping
   time-of-day to locations and activities, **so that** routines are data-driven.
2. **US-13.19.4b.1** — **As a** game developer (P-15), **I want** the schedule system to drive NPC
   pathfinding between locations, **so that** NPCs move to their destinations on time.
3. **US-13.19.4c.1** — [game-specific] **As a** player (P-23), **I want** interactions gated by
   schedule (shopkeepers sell during work hours), **so that** schedules affect gameplay.

## Ambient Barks and Threat Tables

| ID           | Persona              |
|--------------|----------------------|
| US-13.19.5.1 | writer (P-17)        |
| US-13.19.5.2 | player (P-23)        |
| US-13.19.6.1 | game designer (P-5)  |
| US-13.19.6.2 | game developer (P-15)|

1. **US-13.19.5.1** — **As a** writer (P-17), **I want** contextual bark pools with priority and
   cooldown per NPC type, **so that** one-liner dialogue is authored easily.
2. **US-13.19.5.2** — **As a** player (P-23), **I want** NPCs to emit contextual one-liners
   (weather, combat, events) without pausing gameplay, **so that** the world feels responsive.
3. **US-13.19.6.1** — **As a** game designer (P-5), **I want** per-enemy threat tables tracking hate
   from player actions, **so that** enemies attack the highest-threat target.
4. **US-13.19.6.2** — **As a** game developer (P-15), **I want** the threat table exposed to AI
   behavior trees for target-switching, **so that** AI decisions use threat data.

## NPC Social Interactions

| ID            | Persona              |
|---------------|----------------------|
| US-13.19.7.1  | game designer (P-5)  |
| US-13.19.8.1  | game designer (P-5)  |
| US-13.19.9.1  | game developer (P-15)|
| US-13.19.10.1 | game designer (P-5)  |
| US-13.19.11.1 | story director (P-4) |
| US-13.19.12.1 | player (P-23)        |
| US-13.19.12.2 | player (P-23)        |

1. **US-13.19.7.1** — **As a** game designer (P-5), **I want** NPCs to autonomously converse with
   other NPCs exchanging topics and gossip, **so that** social simulation is emergent.
2. **US-13.19.8.1** — **As a** game designer (P-5), **I want** per-NPC memory stores of witnessed
   events with confidence decay, **so that** NPCs recall and share information organically.
3. **US-13.19.9.1** — **As a** game developer (P-15), **I want** NPCs to interact with environmental
   objects (doors, chairs, stations) using the same system as players, **so that** NPCs feel
   believable.
4. **US-13.19.10.1** — **As a** game designer (P-5), **I want** NPCs to search for targets using
   social cues (asking NPCs, checking evidence), **so that** search feels fair.
5. **US-13.19.11.1** — **As a** story director (P-4), **I want** NPCs to react to quest and story
   state changes with dialogue and behavior, **so that** the narrative feels impactful.
6. **US-13.19.12.1** — [game-specific] **As a** player (P-23), **I want** to overhear NPC
   conversations containing quest hints, **so that** exploration and attention are rewarded.
7. **US-13.19.12.2** — [game-specific] **As a** player (P-23), **I want** nearby NPCs to visibly
   react to quest events (cheer, panic), **so that** my actions affect the world.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-13.19.1 | game designer (P-5) |
| US-13.19.10 | game designer (P-5) |
| US-13.19.11 | story director (P-4) |
| US-13.19.12 | player (P-23) |
| US-13.19.2 | game designer (P-5) |
| US-13.19.5 | writer (P-17) |
| US-13.19.6 | game designer (P-5) |
| US-13.19.7 | game designer (P-5) |
| US-13.19.8 | game designer (P-5) |
| US-13.19.9 | game developer (P-15) |

1. **US-13.19.1** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.19.1.1 through US-13.19.1.3 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

2. **US-13.19.10** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.19.10.1 through US-13.19.10.1 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

3. **US-13.19.11** -- **As a** story director (P-4), **I want** the capabilities defined in
   sub-stories US-13.19.11.1 through US-13.19.11.1 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

4. **US-13.19.12** -- **As a** player (P-23), **I want** the capabilities defined in sub-stories
   US-13.19.12.1 through US-13.19.12.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

5. **US-13.19.2** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.19.2.1 through US-13.19.2.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

6. **US-13.19.5** -- **As a** writer (P-17), **I want** the capabilities defined in sub-stories
   US-13.19.5.1 through US-13.19.5.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

7. **US-13.19.6** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.19.6.1 through US-13.19.6.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

8. **US-13.19.7** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.19.7.1 through US-13.19.7.1 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

9. **US-13.19.8** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.19.8.1 through US-13.19.8.1 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

10. **US-13.19.9** -- **As a** game developer (P-15), **I want** the capabilities defined in
    sub-stories
US-13.19.9.1 through US-13.19.9.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.
