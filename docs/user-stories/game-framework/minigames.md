# User Stories -- Minigame Framework (13.26)

## Minigame Context

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.26.1.1  | game designer (P-5)     |
| US-13.26.1.2  | game developer (P-15)   |
| US-13.26.1.3  | player (P-23)           |
| US-13.26.1.4  | modder (P-24)           |
| US-13.26.2.1  | game designer (P-5)     |
| US-13.26.2.2  | player (P-23)           |
| US-13.26.2.3  | player (P-23)           |
| US-13.26.3.1  | game designer (P-5)     |
| US-13.26.3.2  | game developer (P-15)   |
| US-13.26.3.3  | player (P-23)           |
| US-13.26.3.4  | player (P-23)           |

1. **US-13.26.1.1** -- **As a** game designer (P-5), **I want** minigame sessions authored as
   self-contained project assets with typed result contracts, **so that** minigame creation requires
   no code.

2. **US-13.26.1.2** -- **As a** game developer (P-15), **I want** entering a minigame to create an
   isolated ECS world with its own input, camera, and UI, **so that** minigame state is separated
   from the outer game.

3. **US-13.26.1.3** -- [game-specific] **As a** player (P-23), **I want** the outer game to
   optionally pause or continue at reduced tick rate during a minigame, **so that** the world
   behaves appropriately while I play.

4. **US-13.26.1.4** -- **As a** modder (P-24), **I want** to create custom minigame sessions using
   the same visual authoring tools, **so that** modded minigames integrate seamlessly.

5. **US-13.26.2.1** -- **As a** game designer (P-5), **I want** to choose between world-space,
   fullscreen overlay, split-view, and diegetic presentation modes per minigame, **so that**
   presentation matches the activity.

6. **US-13.26.2.2** -- [game-specific] **As a** player (P-23), **I want** minigames to render on
   in-world surfaces like arcade screens, **so that** minigames feel physically embedded in the
   world.

7. **US-13.26.2.3** -- [game-specific] **As a** player (P-23), **I want** diegetic mode rendering
   minigame elements as 3D objects in the world, **so that** chess pieces on a table look natural.

8. **US-13.26.3.1** -- **As a** game designer (P-5), **I want** to define result contracts with
   entry conditions, output results, and side effects per minigame, **so that** each minigame's
   integration is explicit.

9. **US-13.26.3.2** -- **As a** game developer (P-15), **I want** the lifecycle to enforce setup,
   play, result, and teardown phases with clean outer-world state restoration, **so that** minigames
   never corrupt the main game.

10. **US-13.26.3.3** -- [game-specific] **As a** player (P-23), **I want** results displayed on
    completion with earned items, currency, and score, **so that** I see what I won.

11. **US-13.26.3.4** -- [game-specific] **As a** player (P-23), **I want** quitting mid-minigame to
    follow the contract's quit rule, **so that** quit behavior is predictable.

## Minigame Types

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.26.4.1  | game designer (P-5)     |
| US-13.26.4.2  | player (P-23)           |
| US-13.26.5.1  | game designer (P-5)     |
| US-13.26.5.2  | game developer (P-15)   |
| US-13.26.5.3  | player (P-23)           |
| US-13.26.5.4  | modder (P-24)           |
| US-13.26.6.1  | game designer (P-5)     |
| US-13.26.6.2  | player (P-23)           |

1. **US-13.26.4.1** -- **As a** game designer (P-5), **I want** to author note patterns, audio
   tracks, and timing tolerances as visual assets, **so that** rhythm content is data-driven.

2. **US-13.26.4.2** -- [game-specific] **As a** player (P-23), **I want** input windows with
   perfect/great/good/miss thresholds, **so that** precision is rewarded with higher scores.

3. **US-13.26.5.1** -- **As a** game designer (P-5), **I want** a configurable NxM grid engine with
   cell types, turn-based or real-time modes, and piece management, **so that** the grid supports
   diverse game types.

4. **US-13.26.5.2** -- **As a** game developer (P-15), **I want** match detection algorithms for
   3-in-a-row, poker hands, and custom logic graph rules with recursive cascading, **so that** grid
   mechanics are reusable.

5. **US-13.26.5.3** -- [game-specific] **As a** player (P-23), **I want** AI opponents with easy,
   medium, and hard difficulty levels, **so that** board minigames are challenging at every skill
   level.

6. **US-13.26.5.4** -- **As a** modder (P-24), **I want** to create custom board layouts and piece
   types as mod assets, **so that** modded board games use the same grid engine.

7. **US-13.26.6.1** -- **As a** game designer (P-5), **I want** to configure physics parameters,
   interaction rules, and scoring per physics minigame, **so that** skill-based activities are
   data-driven.

8. **US-13.26.6.2** -- [game-specific] **As a** player (P-23), **I want** analog input for nuanced
   control in physics-driven activities like fishing and crane machines, **so that** physics
   minigames reward precision.

## Multiplayer and Discovery

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.26.7.1  | game designer (P-5)     |
| US-13.26.7.2  | player (P-23)           |
| US-13.26.7.3  | player (P-23)           |
| US-13.26.8.1  | game designer (P-5)     |
| US-13.26.8.2  | player (P-23)           |
| US-13.26.8.3  | player (P-23)           |
| US-13.26.8.4  | modder (P-24)           |

1. **US-13.26.7.1** -- **As a** game designer (P-5), **I want** turn-based minigames to use the turn
   manager and real-time ones to use prediction and rollback, **so that** networking matches the
   minigame type.

2. **US-13.26.7.2** -- [game-specific] **As a** player (P-23), **I want** to play minigames with
   other players locally or online, **so that** minigames support social play.

3. **US-13.26.7.3** -- [game-specific] **As a** player (P-23), **I want** spectators to watch
   minigame sessions in progress, **so that** I can observe competitive matches.

4. **US-13.26.8.1** -- **As a** game designer (P-5), **I want** a runtime registry tracking
   available minigames with metadata (category, player count, difficulty), **so that** discovery and
   sorting are data-driven.

5. **US-13.26.8.2** -- [game-specific] **As a** player (P-23), **I want** to discover minigames
   through world interactions and access them from a collectible menu, **so that** minigames are
   explorable.

6. **US-13.26.8.3** -- [game-specific] **As a** player (P-23), **I want** to replay discovered
   minigames from the menu without returning to the world location, **so that** replaying is
   convenient.

7. **US-13.26.8.4** -- **As a** modder (P-24), **I want** modded minigames to register in the same
   library, **so that** community minigames are discoverable alongside official ones.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-13.26.1 | game designer (P-5) |
| US-13.26.2 | game designer (P-5) |
| US-13.26.3 | game designer (P-5) |
| US-13.26.4 | game designer (P-5) |
| US-13.26.5 | game designer (P-5) |
| US-13.26.6 | game designer (P-5) |
| US-13.26.7 | game designer (P-5) |
| US-13.26.8 | game designer (P-5) |

1. **US-13.26.1** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.26.1.1 through US-13.26.1.4 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

2. **US-13.26.2** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.26.2.1 through US-13.26.2.3 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

3. **US-13.26.3** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.26.3.1 through US-13.26.3.4 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

4. **US-13.26.4** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.26.4.1 through US-13.26.4.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

5. **US-13.26.5** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.26.5.1 through US-13.26.5.4 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

6. **US-13.26.6** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.26.6.1 through US-13.26.6.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

7. **US-13.26.7** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.26.7.1 through US-13.26.7.3 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

8. **US-13.26.8** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.26.8.1 through US-13.26.8.4 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.
