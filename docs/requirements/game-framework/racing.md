# R-13.22 -- Racing Systems Requirements

## R-13.22.1 Track and Checkpoint System

The engine **SHALL** define race tracks as ordered sequences of checkpoint trigger volumes that
record passage order and timestamps, enforce checkpoint ordering to invalidate skipped checkpoints,
count laps on start/finish crossing with all intermediates registered, compute split times against
best lap and ghost, and apply slowdown or respawn penalties at track boundary volumes.

- **Derived from:** [F-13.22.1](../../features/game-framework/racing.md)
- **Rationale:** Ordered checkpoint enforcement prevents shortcut exploits and provides the timing
  infrastructure required for lap counting, split times, and leaderboard accuracy.
- **Verification:** Place 5 checkpoints on a circuit. Drive through all in order; verify lap count
  increments on crossing start/finish. Skip checkpoint 3 and cross start/finish; verify the lap does
  not count. Verify split times compute as the delta between current and best-lap timestamps at each
  checkpoint. Drive out of bounds; verify the vehicle respawns at the last valid checkpoint.

## R-13.22.2 Race Mode Framework

The engine **SHALL** provide a data-driven race mode framework supporting circuit, time trial,
elimination, knockout, drift challenge, drag race, and checkpoint race modes, each defining win/loss
conditions, scoring rules, timer behavior, elimination rules, and reward distribution as composable
rule components configurable in the visual editor.

- **Derived from:** [F-13.22.2](../../features/game-framework/racing.md)
- **Rationale:** A composable mode framework lets designers create and iterate on diverse race
  formats without code changes, supporting the full breadth of racing game types.
- **Verification:** Configure a 3-lap circuit mode; verify the race ends when a racer completes 3
  laps. Configure an elimination mode with 4 racers; verify the last-place racer is eliminated each
  lap. Configure a time trial; verify the race is solo with a countdown timer. Create a custom mode
  combining elimination with drift scoring in the visual editor; verify both rules activate
  correctly.

## R-13.22.3a Racing AI Navigation

The engine **SHALL** drive AI racers along waypoint splines with configurable racing lines, braking
points, and speed targets per segment, controlling difficulty via top speed limiting, braking
accuracy, and racing line adherence.

- **Derived from:** [F-13.22.3a](../../features/game-framework/racing.md)
- **Rationale:** Waypoint-based navigation gives designers precise control over AI racing behavior
  while difficulty parameters enable tuning per skill level.
- **Verification:** Run a race with 3 AI racers at medium difficulty. Verify AI follows the
  configured racing line within 1 m tolerance.

## R-13.22.3b Rubber-Banding Difficulty

The engine **SHALL** apply rubber-banding that dynamically adjusts trailing and leading AI speed to
maintain competitive gaps within a configurable time window, with per-difficulty-tier intensity
parameters.

- **Derived from:** [F-13.22.3b](../../features/game-framework/racing.md)
- **Rationale:** Rubber-banding keeps races competitive across skill levels without feeling trivial
  or insurmountable.
- **Verification:** Stop the player vehicle; verify trailing AI slows and leading AI does not exceed
  the configured maximum gap. Set rubber-banding to aggressive; verify all racers stay within the
  configured time window. Disable rubber-banding; verify AI maintains constant speed targets.

## R-13.22.3c AI Racing Behavior

The engine **SHALL** support AI racer behaviors including collision avoidance, position jostling on
straights, and racing line defense through corners, with per-personality-profile aggressiveness
configuration.

- **Derived from:** [F-13.22.3c](../../features/game-framework/racing.md)
- **Rationale:** Behavioral personality adds variety to AI racers beyond raw speed, making races
  feel competitive and dynamic.
- **Verification:** Verify AI racers avoid collisions on straights. Verify aggressive AI defends
  racing lines through corners. Verify cautious AI yields position when pressured.

## R-13.22.4 Drift Scoring and Boost System

The engine **SHALL** detect drifts when lateral slip angle exceeds a configurable threshold,
accumulate drift score based on slip angle, speed, and duration with combo multipliers, fill a boost
meter from drift score and configurable additional sources (collectibles, cooldown recharge), and
grant a temporary speed boost on activation with associated visual effects.

- **Derived from:** [F-13.22.4](../../features/game-framework/racing.md)
- **Rationale:** Drift-to-boost creates a skill-rewarding feedback loop that encourages aggressive
  cornering and adds a layer of mastery beyond optimal racing lines.
- **Verification:** Drive a vehicle into a sustained drift exceeding the slip angle threshold;
  verify drift state activates and score accumulates. Verify higher slip angle and speed yield
  proportionally higher score. Maintain the drift for 3 seconds; verify the combo multiplier
  increases. Verify boost meter fills proportionally to drift score. Activate boost; verify speed
  increases by the configured amount for the configured duration and visual effects trigger.

## R-13.22.5 Ghost Replay and Leaderboards

The engine **SHALL** record the player's best run as a compressed input/position stream, replay it
as a transparent ghost vehicle in time trial mode, persist per-track leaderboards with best times,
vehicle, and date, sync leaderboards through platform services for global competition, and support
sharing ghost data between players.

- **Derived from:** [F-13.22.5](../../features/game-framework/racing.md)
- **Rationale:** Ghost replays give players a tangible benchmark to race against, while leaderboards
  drive long-term competitive engagement and replayability.
- **Verification:** Complete a time trial lap; verify a ghost recording is saved. Start a new time
  trial; verify the ghost vehicle replays the recorded run as a transparent model. Beat the ghost's
  time; verify the ghost updates to the new best. Check the leaderboard; verify the entry shows
  correct time, vehicle, and date. Simulate a second player's ghost; verify it can be loaded and
  raced against.

## Non-Functional Requirements

### NFR-13.22.1 Racing Physics Tick Rate

Vehicle physics and checkpoint detection **SHALL** run at a fixed 120 Hz tick rate to ensure
consistent lap timing precision to the millisecond. Drift detection and boost meter updates
**SHALL** process within the same tick. Ghost replay synchronization **SHALL** maintain sub-frame
accuracy relative to the recorded run.

- **Rationale:** Racing games require high-precision timing for competitive fairness. Low tick rates
  cause inconsistent checkpoint detection and lap times.
- **Verification:** Complete 10 identical-input laps and verify lap time variance is under 1ms.
  Verify checkpoint timestamps are recorded at 120 Hz precision. Verify ghost replay positions match
  the recorded run within 0.01 world units.

### NFR-13.22.2 Ghost Replay Storage Efficiency

Ghost replay data **SHALL** consume no more than 10 KB per minute of recorded racing at 120 Hz
sample rate, using delta compression on position and input streams. Per-track leaderboard entries
**SHALL** consume no more than 256 bytes each.

- **Rationale:** Players accumulate many ghost replays across tracks. Compact storage prevents
  excessive disk and network bandwidth usage.
- **Verification:** Record a 5-minute ghost replay. Verify compressed size is under 50 KB. Verify a
  leaderboard entry serializes to no more than 256 bytes.
