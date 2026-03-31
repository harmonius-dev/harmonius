# R-13.26 -- Minigame Framework Requirements

## Embedded Scene

1. **R-13.26.1** -- The engine **SHALL** support isolated embedded scene contexts with their own ECS
   world, input mapping, camera, and UI layer, with a typed contract defining read/write access to
   the outer world and configurable outer-world tick behavior (pause, reduced, normal).
   - **Rationale:** An isolated embedded scene is the engine primitive for minigames, sub-games, and
     nested gameplay contexts.
   - **Verification:** Enter an embedded scene and verify it cannot access arbitrary outer world
     state. Verify the outer world resumes correctly after teardown.

2. **R-13.26.2** -- The engine **SHALL** render embedded scenes in configurable presentation modes:
   world-space render-to-texture, fullscreen overlay, split-view, and diegetic 3D objects.
   - **Rationale:** Multiple presentation modes enable embedded scenes to fit different gameplay
     contexts.
   - **Verification:** Render in world-space mode and verify correct display on the designated
     surface. Switch to fullscreen and verify the outer world is dimmed.

## Rule State Machine

1. **R-13.26.3** -- The engine **SHALL** enforce a lifecycle of setup, play, result, and teardown
   phases with typed result contracts specifying entry conditions, outputs, side effects, and quit
   rules, with results persisted through the save system.
   - **Rationale:** A formal lifecycle with typed contracts prevents state corruption between
     embedded and outer scenes.
   - **Verification:** Complete an embedded scene and verify the result contract applies to the
     outer world. Quit mid-play and verify the configured quit rule (loss, refund, or no effect) is
     enforced.

## Timing Template

1. **R-13.26.4** -- The engine **SHALL** provide a reusable timing template with audio-synchronized
   beat markers, configurable input windows (perfect/great/good/miss), and score accumulation with
   combo multipliers.
   - **Rationale:** A timing template serves rhythm games, QTE sequences, and precision-timing
     challenges.
   - **Verification:** Hit a beat at perfect timing and verify maximum score. Miss the window and
     verify zero score. Verify combo multiplier increments on successive hits.

## Grid Engine

1. **R-13.26.5** -- The engine **SHALL** provide a configurable NxM grid engine with cell types,
   turn-based or real-time modes, piece management, match detection algorithms, cascading
   resolution, and AI opponents with configurable difficulty tiers.
   - **Rationale:** A grid engine is the engine primitive for card games, board games, match-3, and
     tile-based challenges.
   - **Verification:** Place three matching tiles in a row and verify match detection. Verify AI
     decision latency stays under the configured cap. Verify cascading resolves before win/loss
     check.

## Physics Sandbox

1. **R-13.26.6** -- The engine **SHALL** provide a physics sandbox template with configurable
   gravity, constraints, analog input mapping, skill-check resolution, and progressive difficulty
   scaling.
   - **Rationale:** A physics sandbox template serves fishing, throwing, crane machines, and physics
     puzzles.
   - **Verification:** Configure a throwing sandbox and verify throw power scales with input
     magnitude. Increase difficulty and verify the skill-check threshold tightens.

## Embedded Scene Multiplayer

1. **R-13.26.7** -- The engine **SHALL** support multiplayer embedded scene sessions with
   server-authoritative state replication, turn synchronization for turn-based scenes, prediction
   and rollback for real-time scenes, and spectator support.
   - **Rationale:** Multiplayer embedded scenes enable social play within nested gameplay contexts.
   - **Verification:** Play a turn-based embedded scene across two clients and verify turn
     synchronization. Verify a spectator can observe without affecting state.

## Embedded Scene Registry

1. **R-13.26.8** -- The engine **SHALL** provide a runtime registry of available embedded scenes
   with metadata (category, player count, difficulty, unlock conditions), discovery tracking, replay
   from menu, and integration with the achievement system.
   - **Rationale:** A registry with discovery tracking enables progressive content unlock and replay
     convenience.
   - **Verification:** Discover an embedded scene and verify it appears in the registry menu. Replay
     from the menu and verify it launches without returning to the world location.
