# User Stories — Quest and Dialogue (13.6)

## Objective Graph

| ID          | Persona              |
|-------------|----------------------|
| US-13.6.1.1 | game designer (P-5)  |
| US-13.6.1.2 | game developer (P-15)|
| US-13.6.2.1 | game designer (P-5)  |
| US-13.6.3.1 | player (P-23)        |
| US-13.6.3.2 | player (P-23)        |

1. **US-13.6.1.1** — **As a** game designer (P-5), **I want** quests as directed acyclic graphs of
   objectives with branching paths, **so that** quest flow is non-linear.
2. **US-13.6.1.2** — **As a** game developer (P-15), **I want** the quest graph evaluated
   server-authoritatively, **so that** completion exploits are prevented.
3. **US-13.6.2.1** — **As a** game designer (P-5), **I want** prerequisite conditions (completed
   quests, level, reputation, items) with boolean composition, **so that** quest gating is
   data-driven.
4. **US-13.6.3.1** — **As a** player (P-23), **I want** a quest journal with active quests,
   objectives, progress counters, and waypoint markers, **so that** I track what to do next.
5. **US-13.6.3.2** — **As a** player (P-23), **I want** quest categories (main story, side, daily)
   with sorting and filtering, **so that** I find relevant quests quickly.

## Branching Dialogue

| ID           | Persona              |
|--------------|----------------------|
| US-13.6.4.1  | writer (P-17)        |
| US-13.6.4.2  | story director (P-4) |
| US-13.6.4.3  | player (P-23)        |
| US-13.6.5a.1 | story director (P-4) |
| US-13.6.5b.1 | game designer (P-5)  |
| US-13.6.5c.1 | player (P-23)        |

1. **US-13.6.4.1** — **As a** writer (P-17), **I want** branching dialogue trees with conditional
   branches checking quest state, reputation, and inventory, **so that** conversations adapt to the
   player.
2. **US-13.6.4.2** — **As a** story director (P-4), **I want** dialogue nodes to trigger side
   effects (quest accept, item grant, reputation change), **so that** conversations drive gameplay.
3. **US-13.6.4.3** — **As a** player (P-23), **I want** player response options that meaningfully
   affect the conversation outcome, **so that** dialogue choices matter.
4. **US-13.6.5a.1** — **As a** story director (P-4), **I want** conversation camera management with
   over-the-shoulder shots and automatic speaker switching, **so that** dialogues are visually
   engaging.
5. **US-13.6.5b.1** — **As a** game designer (P-5), **I want** configurable gameplay state changes
   during conversation (HUD suppression, input suppression, audio ducking), **so that** dialogue
   focus is controlled.
6. **US-13.6.5c.1** — **As a** player (P-23), **I want** interrupted conversations to resume from
   the last node when I return, **so that** I do not lose progress.

## Rewards and World Events

| ID           | Persona                    |
|--------------|----------------------------|
| US-13.6.6.1  | game designer (P-5)        |
| US-13.6.6.2  | player (P-23)              |
| US-13.6.7a.1 | server administrator (P-22)|
| US-13.6.7a.2 | game designer (P-5)        |
| US-13.6.7b.1 | game designer (P-5)        |
| US-13.6.7b.2 | player (P-23)              |

1. **US-13.6.6.1** — **As a** game designer (P-5), **I want** per-quest reward tables with XP,
   currency, items, and reputation, **so that** rewards are data-driven.
2. **US-13.6.6.2** — **As a** player (P-23), **I want** quest rewards granted transactionally and
   server-authoritatively, **so that** duplication exploits are prevented.
3. **US-13.6.7a.1** — **As a** server administrator (P-22), **I want** server-driven world events
   triggered by conditions (time, player count), **so that** live events can be scheduled.
4. **US-13.6.7a.2** — **As a** game designer (P-5), **I want** world events to alter zone state for
   all players simultaneously, **so that** shared world moments are impactful.
5. **US-13.6.7b.1** — **As a** game designer (P-5), **I want** quest phasing to show different zone
   versions per player's quest stage, **so that** story progression is visible in the world.
6. **US-13.6.7b.2** — [game-specific] **As a** player (P-23), **I want** to see the town rebuilt
   after completing a defense quest, **so that** my actions visibly change the world.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-13.6.1 | game designer (P-5) |
| US-13.6.2 | game designer (P-5) |
| US-13.6.3 | player (P-23) |
| US-13.6.4 | writer (P-17) |
| US-13.6.6 | game designer (P-5) |

1. **US-13.6.1** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-13.6.1.1 through US-13.6.1.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

2. **US-13.6.2** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-13.6.2.1 through US-13.6.2.1 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

3. **US-13.6.3** -- **As a** player (P-23), **I want** the capabilities defined in sub-stories
   US-13.6.3.1 through US-13.6.3.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

4. **US-13.6.4** -- **As a** writer (P-17), **I want** the capabilities defined in sub-stories
   US-13.6.4.1 through US-13.6.4.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

5. **US-13.6.6** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-13.6.6.1 through US-13.6.6.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.
