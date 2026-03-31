# R-13.22 -- Racing Systems Requirements

## Spline Track and Checkpoints

1. **R-13.22.1** -- The engine **SHALL** provide ordered checkpoint volumes that enforce passage
   order, record timestamps for split times, validate lap completion, and apply configurable
   boundary penalties (slowdown or respawn at last checkpoint).
   - **Rationale:** Checkpoint enforcement is the engine primitive for any track-based progression
     or racing system.
   - **Verification:** Skip a checkpoint and verify the lap is invalidated. Cross all checkpoints
     and verify lap completion. Exit a boundary and verify the configured penalty applies.

2. **R-13.22.2** -- The engine **SHALL** provide a composable race mode framework where win/loss
   conditions, scoring rules, timer behavior, elimination rules, and reward distribution are
   assembled from data- driven rule components.
   - **Rationale:** Composable rules enable custom race formats without per-mode code.
   - **Verification:** Configure an elimination mode and verify the last-place racer is removed each
     lap. Configure a time trial and verify solo timing without opponents.

## Spline AI Navigation

1. **R-13.22.3** -- The engine **SHALL** provide AI navigation along waypoint splines with
   per-segment racing line, braking, and speed targets; difficulty-tiered top speed, braking
   accuracy, and line adherence; rubber-banding with configurable intensity per tier; and
   personality profiles controlling aggressiveness.
   - **Rationale:** Spline-based AI with rubber-banding and personality profiles creates competitive
     and varied AI opponents.
   - **Verification:** Verify easy AI drives slower than hard AI. Let AI fall behind and verify
     rubber-banding activates beyond the configured gap threshold.

## Drift Detection and Boost

1. **R-13.22.4** -- The engine **SHALL** detect sustained drifts from slip angle and accumulate
   score based on angle, speed, and duration, with configurable boost meter fill and temporary speed
   boost activation.
   - **Rationale:** Drift detection with boost reward is the engine primitive for arcade racing
     skill expression.
   - **Verification:** Drift below the slip angle threshold and verify no detection. Drift above the
     threshold and verify score accumulates. Activate boost and verify the speed increase matches
     the configured value.

## Ghost Replay

1. **R-13.22.5** -- The engine **SHALL** record player runs as compressed input/position streams,
   replay them as ghost entities in time trial mode, and integrate with per-track leaderboards via
   platform services (F-14.5.2).
   - **Rationale:** Ghost replay and leaderboards provide asynchronous competition and
     self-improvement tools.
   - **Verification:** Record a run and verify the ghost replays the exact path. Submit a time and
     verify the leaderboard entry appears.
