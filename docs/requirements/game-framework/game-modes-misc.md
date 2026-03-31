# R-13.24 -- Game Modes and Systems Requirements

## Wave Spawning

1. **R-13.24.1** -- The engine **SHALL** provide a configurable wave spawner with per-wave enemy
   composition, spawn timing, inter-wave delay, difficulty scaling, endless procedural generation,
   and ECS- exposed wave state for UI and gameplay logic.
   - **Rationale:** A data-driven wave spawner serves tower defense, survival, and arena game modes
     without custom code.
   - **Verification:** Configure a 5-wave sequence and verify each wave spawns the correct enemies
     at the configured intervals. Enable endless mode and verify procedural escalation.

## Auto-Targeting Entities

1. **R-13.24.2** -- The engine **SHALL** provide configurable auto- targeting for stationary
   entities with scoring modes (nearest, strongest, fastest), data-driven upgrade paths, and role
   differentiation (damage, support, slow, spawner).
   - **Rationale:** Auto-targeting with scoring modes is the engine primitive for tower defense and
     turret systems.
   - **Verification:** Set targeting to "nearest" and verify the closest enemy is always targeted.
     Upgrade a tower and verify stats change per the configured tier.

## Score and Combo

1. **R-13.24.3** -- The engine **SHALL** provide a reusable scoring framework with per-event point
   values, combo chains with timing windows, multipliers, and configurable grade thresholds.
   - **Rationale:** A generic scoring framework serves action, rhythm, racing, and arcade modes
     without per-genre code.
   - **Verification:** Score two events within the combo window and verify the multiplier
     increments. Wait beyond the window and verify the combo resets.

## Feedback Stacks

1. **R-13.24.4** -- The engine **SHALL** provide composable feedback stack assets containing ordered
   entries with type, timing, intensity, and parallel/sequential execution, triggerable by gameplay
   events. Built-in entry types **SHALL** include camera shake, haptic pulse, particle burst, sound
   cue, screen flash, and time scale manipulation (hit-stop and slow-motion).
   - **Rationale:** Composable feedback stacks enable designers to author game feel without code.
   - **Verification:** Trigger a stack with parallel entries and verify all fire simultaneously.
     Trigger hit-stop and verify selective entity freeze for the configured frame count.

## Waypoint Network

1. **R-13.24.5** -- The engine **SHALL** provide a discoverable waypoint network with configurable
   travel costs, cooldowns, and loading transitions for fast travel between unlocked points.
   - **Rationale:** A waypoint network is the engine primitive for fast travel, recall abilities,
     and teleportation systems.
   - **Verification:** Discover a waypoint and verify it appears in the travel menu. Travel to it
     and verify the player repositions correctly. Verify undiscovered waypoints are hidden.

## Respawn System

1. **R-13.24.6** -- The engine **SHALL** provide configurable respawn point selection (nearest
   unlocked), optional spirit form traversal, temporary debuffs on respawn, and per-mode respawn
   timers.
   - **Rationale:** A data-driven respawn system adapts to casual and competitive modes without code
     changes.
   - **Verification:** Die and verify the nearest graveyard is selected. Configure a 10 s respawn
     timer and verify the delay is enforced.

## Tutorial Framework

1. **R-13.24.7** -- The engine **SHALL** provide a data-driven tutorial framework with sequential
   steps, trigger conditions, visual overlays, forced input sequences, progress persistence, and a
   non-blocking toast notification system.
   - **Rationale:** Tutorials and toasts serve onboarding and status communication across all game
     types.
   - **Verification:** Start a tutorial, interrupt it, restart the game, and verify it resumes at
     the correct step. Fire two toasts simultaneously and verify they queue rather than overlap.

## Photo Mode

1. **R-13.24.8** -- The engine **SHALL** provide a free-flight camera controller with
   post-processing overrides, composition aids, screenshot capture at configurable resolution with
   supersampling, and platform screenshot API integration.
   - **Rationale:** Photo mode is a standard player feature requiring camera, post-processing, and
     capture integration.
   - **Verification:** Enter photo mode and verify gameplay pauses in single-player. Capture a 4x
     supersampled screenshot and verify the output resolution matches. Verify adjustments revert on
     exit.
