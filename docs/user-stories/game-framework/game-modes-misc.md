# User Stories -- Game Modes and Systems (13.24)

## Wave and Tower Defense

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.24.1.1  | game designer (P-5)     |
| US-13.24.1.2  | game developer (P-15)   |
| US-13.24.1.3  | player (P-23)           |
| US-13.24.1.4  | player (P-23)           |
| US-13.24.2.1  | game designer (P-5)     |
| US-13.24.2.2  | game developer (P-15)   |
| US-13.24.2.3  | player (P-23)           |
| US-13.24.2.4  | player (P-23)           |

1. **US-13.24.1.1** -- **As a** game designer (P-5), **I want** to configure enemy composition,
   spawn points, timing, inter-wave delay, and difficulty scaling per wave, **so that** wave design
   is data-driven.

2. **US-13.24.1.2** -- **As a** game developer (P-15), **I want** endless mode to procedurally
   generate waves using configurable escalation rules, **so that** infinite play is supported
   without manual wave authoring.

3. **US-13.24.1.3** -- [game-specific] **As a** player (P-23), **I want** enemy waves to spawn with
   escalating difficulty, **so that** survival modes provide increasing challenge.

4. **US-13.24.1.4** -- [game-specific] **As a** player (P-23), **I want** wave state displayed in
   the HUD, **so that** I track current wave, enemies remaining, and next wave timer.

5. **US-13.24.2.1** -- **As a** game designer (P-5), **I want** to define tower stats, targeting
   modes, upgrade tiers, and costs in gameplay databases, **so that** tower design is data-driven.

6. **US-13.24.2.2** -- **As a** game developer (P-15), **I want** tower targeting to use
   configurable scoring modes (nearest, strongest, fastest), **so that** targeting logic is reusable
   across game types.

7. **US-13.24.2.3** -- [game-specific] **As a** player (P-23), **I want** towers to auto-fire at
   targets within range, **so that** tower placement is the strategic decision rather than manual
   aiming.

8. **US-13.24.2.4** -- [game-specific] **As a** player (P-23), **I want** to upgrade towers along
   linear or branching paths with per-tier stat improvements, **so that** towers evolve during the
   match.

## Score and Feedback

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.24.3.1  | game designer (P-5)     |
| US-13.24.3.2  | game developer (P-15)   |
| US-13.24.3.3  | player (P-23)           |
| US-13.24.4.1  | game designer (P-5)     |
| US-13.24.4.2  | game designer (P-5)     |
| US-13.24.4.3  | game developer (P-15)   |
| US-13.24.4.4  | player (P-23)           |
| US-13.24.4.5  | player (P-23)           |

1. **US-13.24.3.1** -- **As a** game designer (P-5), **I want** to configure per-event point values,
   combo window, and grade thresholds, **so that** scoring is tunable per game mode.

2. **US-13.24.3.2** -- **As a** game developer (P-15), **I want** the scoring framework to be
   reusable across action, rhythm, racing, and arcade modes, **so that** one system serves all
   genres.

3. **US-13.24.3.3** -- [game-specific] **As a** player (P-23), **I want** combo counters to
   increment on successive scoring events and multiply points, **so that** sustained performance is
   rewarded.

4. **US-13.24.4.1** -- **As a** game designer (P-5), **I want** to compose feedback stacks from
   reusable entries with timing, intensity, and execution flags, **so that** I iterate on game feel
   without code.

5. **US-13.24.4.2** -- **As a** game designer (P-5), **I want** a library of built-in feedback entry
   types (shake, haptic, particle, sound, flash) with configurable parameter schemas, **so that**
   effects use standard building blocks.

6. **US-13.24.4.3** -- **As a** game developer (P-15), **I want** hit-stop to freeze entities for a
   configurable frame count and slow-motion to interpolate time scale, **so that** time manipulation
   is frame-precise.

7. **US-13.24.4.4** -- [game-specific] **As a** player (P-23), **I want** gameplay events like hit
   confirmation to trigger multi-sensory feedback, **so that** impactful moments feel satisfying.

8. **US-13.24.4.5** -- [game-specific] **As a** player (P-23), **I want** slow-motion ramps during
   key moments like finishing blows, **so that** climactic actions feel cinematic.

## Exploration

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.24.5.1  | game designer (P-5)     |
| US-13.24.5.2  | level designer (P-6)    |
| US-13.24.5.3  | player (P-23)           |
| US-13.24.5.4  | player (P-23)           |
| US-13.24.6.1  | game designer (P-5)     |
| US-13.24.6.2  | player (P-23)           |
| US-13.24.6.3  | player (P-23)           |
| US-13.24.7.1  | game designer (P-5)     |
| US-13.24.7.2  | game developer (P-15)   |
| US-13.24.7.3  | player (P-23)           |
| US-13.24.7.4  | player (P-23)           |
| US-13.24.8.1  | game designer (P-5)     |
| US-13.24.8.2  | game developer (P-15)   |
| US-13.24.8.3  | player (P-23)           |
| US-13.24.8.4  | player (P-23)           |

1. **US-13.24.5.1** -- **As a** game designer (P-5), **I want** to configure waypoint discovery
   triggers, travel costs, and cooldowns, **so that** I balance fast travel convenience via data.

2. **US-13.24.5.2** -- **As a** level designer (P-6), **I want** to place waypoint entities with
   discovery trigger volumes, **so that** fast travel points integrate with world design.

3. **US-13.24.5.3** -- [game-specific] **As a** player (P-23), **I want** to discover waypoints
   through exploration and fast travel between unlocked ones, **so that** backtracking is reduced.

4. **US-13.24.5.4** -- [game-specific] **As a** player (P-23), **I want** to select a destination
   from the map UI with names, icons, and optional travel costs, **so that** I choose my destination
   easily.

5. **US-13.24.6.1** -- **As a** game designer (P-5), **I want** to configure respawn timers per game
   mode, **so that** death penalty fits casual and competitive modes differently.

6. **US-13.24.6.2** -- [game-specific] **As a** player (P-23), **I want** to respawn at the nearest
   unlocked graveyard on death, **so that** I resume play near where I died.

7. **US-13.24.6.3** -- [game-specific] **As a** player (P-23), **I want** an optional spirit form to
   navigate back to my corpse for revival, **so that** death has a recovery mechanic.

8. **US-13.24.7.1** -- **As a** game designer (P-5), **I want** to author tutorial steps as data
   assets with trigger conditions, instruction text, and completion actions, **so that** tutorials
   are data-driven.

9. **US-13.24.7.2** -- **As a** game developer (P-15), **I want** tutorial progress to persist
   across sessions with per-step logic graph execution, **so that** interrupted tutorials resume
   correctly.

10. **US-13.24.7.3** -- [game-specific] **As a** player (P-23), **I want** tutorials to guide me
    with sequential steps that resume after interruption, **so that** onboarding is smooth.

11. **US-13.24.7.4** -- [game-specific] **As a** player (P-23), **I want** non-blocking toast
    notifications for achievements, loot, and quest updates, **so that** I am informed without
    interrupting gameplay.

12. **US-13.24.8.1** -- **As a** game designer (P-5), **I want** all photo mode adjustments to be
    temporary and revert on exit, **so that** gameplay settings are not permanently altered.

13. **US-13.24.8.2** -- **As a** game developer (P-15), **I want** screenshot capture at
    configurable resolution with platform API integration, **so that** captures feed into platform
    galleries.

14. **US-13.24.8.3** -- [game-specific] **As a** player (P-23), **I want** a free-flight camera for
    virtual photography with position, rotation, and FOV controls, **so that** I frame screenshots
    from any angle.

15. **US-13.24.8.4** -- [game-specific] **As a** player (P-23), **I want** depth of field, exposure,
    color grading filters, and composition aids in photo mode, **so that** I create visually
    polished screenshots.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-13.24.1 | game designer (P-5) |
| US-13.24.2 | game designer (P-5) |
| US-13.24.3 | game designer (P-5) |
| US-13.24.4 | game designer (P-5) |
| US-13.24.5 | game designer (P-5) |
| US-13.24.6 | game designer (P-5) |
| US-13.24.7 | game designer (P-5) |
| US-13.24.8 | game designer (P-5) |

1. **US-13.24.1** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.24.1.1 through US-13.24.1.4 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

2. **US-13.24.2** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.24.2.1 through US-13.24.2.4 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

3. **US-13.24.3** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.24.3.1 through US-13.24.3.3 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

4. **US-13.24.4** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.24.4.1 through US-13.24.4.5 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

5. **US-13.24.5** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.24.5.1 through US-13.24.5.4 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

6. **US-13.24.6** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.24.6.1 through US-13.24.6.3 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

7. **US-13.24.7** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.24.7.1 through US-13.24.7.4 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

8. **US-13.24.8** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.24.8.1 through US-13.24.8.4 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.
