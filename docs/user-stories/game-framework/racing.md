# User Stories -- Racing Systems (13.22)

## Track and Race Modes

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.22.1.1  | game designer (P-5)     |
| US-13.22.1.2  | game developer (P-15)   |
| US-13.22.1.3  | player (P-23)           |
| US-13.22.1.4  | level designer (P-6)    |
| US-13.22.2.1  | game designer (P-5)     |
| US-13.22.2.2  | player (P-23)           |
| US-13.22.2.3  | player (P-23)           |

1. **US-13.22.1.1** -- **As a** game designer (P-5), **I want** track boundaries to apply slowdown
   penalties or respawn the vehicle at the last checkpoint, **so that** out-of-bounds behavior is
   configurable.

2. **US-13.22.1.2** -- **As a** game developer (P-15), **I want** checkpoints to enforce passage
   order and record timestamps for split times, **so that** lap validation and timing are
   deterministic.

3. **US-13.22.1.3** -- [game-specific] **As a** player (P-23), **I want** split times comparing my
   current segment to best lap and ghost, **so that** I track per-segment improvement.

4. **US-13.22.1.4** -- **As a** level designer (P-6), **I want** to define track layouts by placing
   ordered checkpoint volumes, **so that** multiple layouts share the same world geometry.

5. **US-13.22.2.1** -- **As a** game designer (P-5), **I want** each race mode to define win/loss
   conditions, scoring, timer behavior, and reward distribution as composable rules, **so that** new
   modes require no code.

6. **US-13.22.2.2** -- [game-specific] **As a** player (P-23), **I want** to race in circuit, time
   trial, elimination, drift challenge, and drag race modes, **so that** I experience varied
   formats.

7. **US-13.22.2.3** -- [game-specific] **As a** player (P-23), **I want** elimination mode to remove
   the last-place racer each lap, **so that** each lap raises the stakes.

## Racing AI

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.22.3.1  | game designer (P-5)     |
| US-13.22.3.2  | game designer (P-5)     |
| US-13.22.3.3  | game developer (P-15)   |
| US-13.22.3.4  | player (P-23)           |

1. **US-13.22.3.1** -- **As a** game designer (P-5), **I want** to author waypoint splines with
   per-segment racing line, braking, and speed data, **so that** AI navigation is tunable per track.

2. **US-13.22.3.2** -- **As a** game designer (P-5), **I want** rubber-band intensity configurable
   per difficulty tier, **so that** casual modes keep racers close while hard modes do not.

3. **US-13.22.3.3** -- **As a** game developer (P-15), **I want** AI racers to use configurable
   personality profiles (cautious, balanced, aggressive), **so that** each opponent feels unique.

4. **US-13.22.3.4** -- [game-specific] **As a** player (P-23), **I want** AI racers to navigate
   tracks via racing lines and jostle for position, **so that** races feel competitive.

## Drift, Boost, and Leaderboards

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.22.4.1  | game designer (P-5)     |
| US-13.22.4.2  | player (P-23)           |
| US-13.22.4.3  | player (P-23)           |
| US-13.22.5.1  | game developer (P-15)   |
| US-13.22.5.2  | player (P-23)           |
| US-13.22.5.3  | player (P-23)           |

1. **US-13.22.4.1** -- **As a** game designer (P-5), **I want** to configure slip angle threshold,
   score scaling, boost meter size, and boost sources, **so that** drift and boost are tunable per
   game mode.

2. **US-13.22.4.2** -- [game-specific] **As a** player (P-23), **I want** sustained drifts to
   accumulate score based on angle, speed, and duration, **so that** drifting is a skill-based
   scoring mechanic.

3. **US-13.22.4.3** -- [game-specific] **As a** player (P-23), **I want** drift score to fill a
   boost meter that grants temporary speed boost, **so that** skilled drifting rewards faster laps.

4. **US-13.22.5.1** -- **As a** game developer (P-15), **I want** ghost data stored as compressed
   input/position streams per track, **so that** ghost recordings are storage-efficient.

5. **US-13.22.5.2** -- [game-specific] **As a** player (P-23), **I want** to race against a
   transparent ghost of my best run in time trial mode, **so that** I have a visual benchmark for
   improvement.

6. **US-13.22.5.3** -- [game-specific] **As a** player (P-23), **I want** per-track leaderboards
   showing best times with player name, vehicle, and date, **so that** I can compete globally.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-13.22.1 | game designer (P-5) |
| US-13.22.2 | game designer (P-5) |
| US-13.22.3 | game designer (P-5) |
| US-13.22.4 | game designer (P-5) |
| US-13.22.5 | game developer (P-15) |

1. **US-13.22.1** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.22.1.1 through US-13.22.1.4 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

2. **US-13.22.2** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.22.2.1 through US-13.22.2.3 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

3. **US-13.22.3** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.22.3.1 through US-13.22.3.4 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

4. **US-13.22.4** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.22.4.1 through US-13.22.4.3 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

5. **US-13.22.5** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-13.22.5.1 through US-13.22.5.3 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.
