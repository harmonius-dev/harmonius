# R-13.24 — Game Modes and Systems Requirements

## Wave and Tower Defense

| ID        | Derived From                                                  |
|-----------|---------------------------------------------------------------|
| R-13.24.1 | [F-13.24.1](../../features/game-framework/game-modes-misc.md) |
| R-13.24.2 | [F-13.24.2](../../features/game-framework/game-modes-misc.md) |

1. **R-13.24.1** — The engine **SHALL** provide a data-driven wave spawning system with configurable
   enemy composition, spawn timing modes (simultaneous, staggered, continuous), inter-wave delays,
   difficulty scaling curves, endless mode with procedural escalation, and ECS-exposed wave state.
   - **Rationale:** ECS-exposed wave state enables UI and gameplay systems to query progress without
     coupling, and data-driven definitions allow designers to author wave content visually.
   - **Verification:** Integration test: define a 5-wave sequence with increasing enemy counts and
     staggered spawning. Verify each wave spawns the correct entity types and counts at configured
     intervals. Verify endless mode generates wave 6+ using escalation rules. Verify ECS resources
     report correct current wave, enemies remaining, and next wave timer values.
2. **R-13.24.2** — The engine **SHALL** provide tower entities with configurable targeting modes
   (first-in-path, nearest, strongest, weakest, most HP, least HP, fastest), auto-fire via the
   ability system, and data-driven linear or branching upgrade paths with per-tier cost, stat, and
   visual changes.
   - **Rationale:** Data-driven targeting and upgrade paths allow designers to balance tower defense
     gameplay without code changes.
   - **Verification:** Unit test: place a tower with "nearest" targeting among 5 enemies at varying
     distances; verify the closest enemy is selected. Switch to "strongest" and verify the
     highest-stat enemy is selected. Apply an upgrade and verify stat values, visual mesh, and cost
     match the tier definition. Verify all 7 targeting modes select the correct target.

## Score and Feedback

| ID         | Derived From                                                   |
|------------|----------------------------------------------------------------|
| R-13.24.3  | [F-13.24.3](../../features/game-framework/game-modes-misc.md)  |
| R-13.24.4a | [F-13.24.4a](../../features/game-framework/game-modes-misc.md) |
| R-13.24.4b | [F-13.24.4b](../../features/game-framework/game-modes-misc.md) |
| R-13.24.4c | [F-13.24.4c](../../features/game-framework/game-modes-misc.md) |

1. **R-13.24.3** — The engine **SHALL** provide a reusable scoring framework with per-event point
   values, timing-window-based combo chains with multipliers, multiplicative power-up stacking, and
   configurable grade thresholds (S/A/B/C/D or star ratings) with per-level and global high score
   tracking.
   - **Rationale:** A generic scoring framework avoids per-genre reimplementation and enables
     consistent leaderboard and progression mechanics across game modes.
   - **Verification:** Unit test: trigger 5 scoring events within the combo window and verify combo
     counter reaches 5x. Let the window expire and verify combo resets. Apply a 2x power-up
     multiplier and verify it stacks multiplicatively with combo. Verify total score maps to the
     correct grade threshold. Verify high scores persist per level.
2. **R-13.24.4a** — The engine **SHALL** provide composable feedback stack assets containing ordered
   lists of feedback entries with per-entry type, timing (delay, duration), intensity, and execution
   mode (parallel or sequential), triggered by gameplay observer events, with visual authoring.
   - **Rationale:** Composable stacks let designers author complex multi-effect feedback without
     code, improving iteration speed on game feel.
   - **Verification:** Author a feedback stack with 3 entries in parallel. Trigger via observer
     event and verify all entries fire at their configured delay offsets. Author a sequential stack
     and verify entries execute in order. Trigger two stacks concurrently and verify independent
     lifetimes.
3. **R-13.24.4b** — The engine **SHALL** provide hit-stop (freeze all or selective entities for N
   frames), slow-motion ramps (interpolate global time scale over a duration), and per-entity time
   scale overrides, as feedback entry types within feedback stacks, respecting multiplayer
   constraints.
   - **Rationale:** Frame-precise time manipulation is essential for impactful combat feedback and
     dramatic moments.
   - **Verification:** Trigger a hit-stop entry and verify the target entity freezes for exactly the
     configured frame count. Trigger a slow-motion ramp and verify the global time scale
     interpolates to the target value and back. Apply a per-entity override and verify one entity
     runs at normal speed while others are slowed.
4. **R-13.24.4c** — The engine **SHALL** provide built-in feedback entry types (camera shake, haptic
   pulse, particle burst, sound cue, chromatic aberration, screen flash, squash-and-stretch) with
   per-type parameter schemas for visual authoring, and support registration of custom entry types
   via the extensibility API.
   - **Rationale:** A standard library of feedback types enables rapid composition of rich
     multi-sensory effects without reimplementation.
   - **Verification:** Author a stack using camera shake, haptic pulse, and screen flash entries.
     Verify each produces its expected effect with configured parameters. Register a custom entry
     type and verify it appears in the visual editor and executes correctly in a stack.

## Exploration

| ID         | Derived From                                                   |
|------------|----------------------------------------------------------------|
| R-13.24.5  | [F-13.24.5](../../features/game-framework/game-modes-misc.md)  |
| R-13.24.6  | [F-13.24.6](../../features/game-framework/game-modes-misc.md)  |
| R-13.24.7a | [F-13.24.7a](../../features/game-framework/game-modes-misc.md) |
| R-13.24.7b | [F-13.24.7b](../../features/game-framework/game-modes-misc.md) |
| R-13.24.7c | [F-13.24.7c](../../features/game-framework/game-modes-misc.md) |
| R-13.24.8a | [F-13.24.8a](../../features/game-framework/game-modes-misc.md) |
| R-13.24.8b | [F-13.24.8b](../../features/game-framework/game-modes-misc.md) |
| R-13.24.8c | [F-13.24.8c](../../features/game-framework/game-modes-misc.md) |

1. **R-13.24.5** — The engine **SHALL** provide a waypoint-based fast travel network where named
   waypoints are discovered through trigger volumes, displayed on the map UI with optional travel
   costs and cooldowns, and teleport the player to the destination with a loading transition or
   seamless zone streaming.
   - **Rationale:** Discovery-gated fast travel rewards exploration while reducing backtracking in
     open-world games.
   - **Verification:** Integration test: place 3 waypoints with discovery triggers. Enter the first
     trigger and verify the waypoint unlocks. Open the map UI and verify only discovered waypoints
     appear. Fast travel to a discovered waypoint and verify player position matches the
     destination. Verify undiscovered waypoints are not selectable. Verify travel cost is deducted
     when configured.
2. **R-13.24.6** — The engine **SHALL** provide a respawn system that selects the nearest unlocked
   graveyard on death, supports optional corpse-run ghost form with restricted abilities,
   resurrection sickness debuffs, player-initiated resurrection, and configurable per-game-mode
   respawn timers.
   - **Rationale:** Configurable respawn mechanics enable different death penalties across game
     modes (casual instant respawn vs. competitive delayed respawn with corpse run).
   - **Verification:** Integration test: place 3 graveyards at known positions. Kill the player and
     verify the nearest unlocked graveyard is selected. Verify ghost form restricts movement speed
     and disables combat abilities. Respawn at the graveyard and verify resurrection sickness debuff
     applies with correct stat penalties and duration. Verify another player can resurrect the
     fallen player, bypassing the graveyard.
3. **R-13.24.7a** — The engine **SHALL** provide a data-driven tutorial framework with sequential
   step definitions, configurable trigger conditions, required-action completion gates,
   cross-session progress persistence, skip functionality, and logic graph integration for custom
   conditions.
   - **Rationale:** Data-driven tutorial steps let designers author and iterate on onboarding flows
     without code changes.
   - **Verification:** Author a 3-step tutorial. Trigger step 1 on zone entry and verify it
     activates. Verify the sequence blocks until the required input. Interrupt mid-tutorial,
     relaunch, and verify progress resumes at the correct step. Verify skip dismisses the entire
     tutorial.
4. **R-13.24.7b** — The engine **SHALL** provide spotlight overlays that highlight UI widgets or
   world entities, arrow callouts pointing to relevant controls, tooltip callouts anchored to
   targets, and forced input sequences that block progression until the player acts.
   - **Rationale:** Visual overlays draw player attention to the correct UI element or world object,
     making tutorials clear and unambiguous.
   - **Verification:** Trigger a tutorial step with a spotlight overlay and verify the target widget
     is highlighted while the rest of the screen dims. Verify arrow callouts point to the correct
     control. Verify forced input blocks progression until the player performs the action.
5. **R-13.24.7c** — The engine **SHALL** provide a non-blocking toast notification system with
   configurable screen position, queuing for concurrent notifications, auto-dismiss duration,
   icon/title/body content, priority-based ordering, and observer-pattern triggering from any
   gameplay system.
   - **Rationale:** Toast notifications keep players informed of achievements, loot, and status
     changes without interrupting gameplay flow.
   - **Verification:** Trigger a toast notification and verify it appears in the configured screen
     region. Trigger 3 toasts simultaneously and verify they queue in priority order. Verify
     auto-dismiss after the configured duration. Verify toasts display independently of tutorial
     state.
6. **R-13.24.8a** — The engine **SHALL** provide a free-flight camera in photo mode with position,
   rotation, roll, and FOV controls, gameplay pause in single-player, wall collision in multiplayer,
   and optional player/UI hiding.
   - **Rationale:** Free-camera control is the foundation for virtual photography, enabling players
     to frame shots from any angle.
   - **Verification:** Enter photo mode and verify free-camera movement, rotation, roll, and FOV
     slider work. Verify gameplay pauses in single-player. In multiplayer, verify the camera cannot
     pass through walls and time does not pause. Toggle UI and player visibility.
7. **R-13.24.8b** — The engine **SHALL** provide photo mode visual controls for depth of field,
   exposure, brightness/contrast/saturation, color grading filters, time-of-day override, and
   composition aids (rule-of-thirds, golden ratio), with all adjustments reverting on exit.
   - **Rationale:** Visual controls enable players to create stylized, polished screenshots that
     drive social media engagement and community content.
   - **Verification:** Adjust DoF focus distance and verify background blur changes. Apply a color
     grading filter and verify visible color shift. Enable rule-of-thirds overlay and verify grid
     appears. Exit photo mode and verify all adjustments revert to gameplay defaults.
8. **R-13.24.8c** — The engine **SHALL** capture screenshots at current or supersampled resolution
   (2x, 4x), save to disk in PNG format, store in an in-game gallery with metadata, and use platform
   screenshot APIs where available.
   - **Rationale:** High-resolution capture and an in-game gallery let players revisit and share
     their best virtual photographs.
   - **Verification:** Capture a screenshot at 2x supersampling and verify the output resolution is
     double the viewport. Verify the file is saved to disk in PNG format. Open the in-game gallery
     and verify the screenshot appears with correct metadata. Verify platform screenshot API
     integration on Steam.

## Non-Functional Requirements

| ID          | Derived From |
|-------------|--------------|
| NFR-13.24.1 |              |
| NFR-13.24.2 |              |

1. **NFR-13.24.1** — The wave spawning system **SHALL** instantiate up to 50 entities per frame
   without causing frame drops below 30 fps. Staggered spawn timing **SHALL** maintain consistent
   inter-spawn intervals within 1ms tolerance regardless of frame rate.
   - **Rationale:** Survival and tower defense modes spawn large numbers of enemies rapidly. Spawn
     throughput must not cause hitches during wave starts.
   - **Verification:** Configure a wave spawning 50 entities simultaneously. Measure frame time
     during the spawn frame. Verify it stays above 30 fps. Configure staggered spawning and verify
     inter-spawn intervals match configured values within 1ms.
2. **NFR-13.24.2** — Feedback stack effects **SHALL** begin executing within 1 frame of the
   triggering gameplay event. Hit-stop freeze frames **SHALL** activate within 1 frame and hold for
   exactly the configured frame count with zero-frame variance.
   - **Rationale:** Game feel effects must be frame-precise. Delayed or inconsistent feedback
     degrades the satisfying impact of combat and ability activation.
   - **Verification:** Trigger a feedback stack on a hit-confirmed event. Measure time from event to
     first effect activation. Verify it is within 1 frame. Measure hit-stop duration and verify it
     matches the configured frame count exactly.
