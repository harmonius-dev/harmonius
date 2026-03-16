# User Stories -- Racing Systems (13.22)

## Track and Checkpoint (F-13.22.1)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.22.1.1 | player (P-23) | **As a** player (P-23), **I want** checkpoints to enforce passage order so that skipping one invalidates the lap, **so that** shortcuts cannot bypass the track. |  | F-13.22.1 | R-13.22.1 |
| US-13.22.1.2 | player (P-23) | **As a** player (P-23), **I want** lap counting to increment when I cross the start/finish checkpoint with all intermediates registered, **so that** lap progress is accurate. |  | F-13.22.1 | R-13.22.1 |
| US-13.22.1.3 | player (P-23) | **As a** player (P-23), **I want** split times comparing my current segment to my best lap and ghost, **so that** I can track per-segment improvement. |  | F-13.22.1 | R-13.22.1 |
| US-13.22.1.4 | designer (P-5) | **As a** designer (P-5), **I want** to define track layouts by placing ordered checkpoint volumes with configurable boundaries, **so that** multiple layouts share the same world geometry. |  | F-13.22.1 | R-13.22.1 |
| US-13.22.1.5 | designer (P-5) | **As a** designer (P-5), **I want** track boundaries to apply slowdown penalties or respawn the vehicle at the last checkpoint, **so that** out-of-bounds behavior is configurable. |  | F-13.22.1 | R-13.22.1 |
| US-13.22.1.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that skipping a checkpoint prevents lap completion, **so that** checkpoint enforcement is correct. |  | F-13.22.1 | R-13.22.1 |
## Race Mode Framework (F-13.22.2)
| US-13.22.2.1 | player (P-23) | **As a** player (P-23), **I want** to race in circuit, time trial, elimination, knockout, drift challenge, drag race, and checkpoint race modes, **so that** I experience varied formats. |  | F-13.22.2 | R-13.22.2 |
| US-13.22.2.2 | designer (P-5) | **As a** designer (P-5), **I want** each race mode to define win/loss conditions, scoring, timer behavior, and reward distribution, **so that** modes are self-contained rule sets. |  | F-13.22.2 | R-13.22.2 |
| US-13.22.2.3 | designer (P-5) | **As a** designer (P-5), **I want** to assemble custom race modes from composable rule components in the visual editor, **so that** new formats require no code. |  | F-13.22.2 | R-13.22.2 |
| US-13.22.2.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that elimination mode removes the last-place racer each lap, **so that** the elimination rule triggers correctly. |  | F-13.22.2 | R-13.22.2 |
| US-13.22.2.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that time trial mode records time without other racers, **so that** solo timing is isolated. |  | F-13.22.2 | R-13.22.2 |
## Racing AI Navigation (F-13.22.3a)
| US-13.22.3 | player (P-23) | **As a** player (P-23), **I want** AI racers to navigate tracks via racing lines with braking points and speed targets, **so that** races feel competitive. |  | F-13.22.3 | R-13.22.3 |
| US-13.22.3 | designer (P-5) | **As a** designer (P-5), **I want** to author waypoint splines with per-segment racing line, braking, and speed data, **so that** AI navigation is tunable per track. |  | F-13.22.3 | R-13.22.3 |
| US-13.22.3 | designer (P-5) | **As a** designer (P-5), **I want** AI difficulty controlled by top speed limiter, braking accuracy, and line adherence, **so that** I can create easy to hard AI tiers. |  | F-13.22.3 | R-13.22.3 |
| US-13.22.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that easy AI drives slower and brakes earlier than hard AI, **so that** difficulty tiering affects race performance. |  | F-13.22.3 | R-13.22.3 |
## Rubber-Banding (F-13.22.3b)
| US-13.22.3 | player (P-23) | **As a** player (P-23), **I want** trailing AI to speed up and leading AI to slow down, **so that** races stay competitive regardless of my skill. |  | F-13.22.3 | R-13.22.3 |
| US-13.22.3 | designer (P-5) | **As a** designer (P-5), **I want** rubber-band intensity configurable per difficulty tier, **so that** casual modes keep racers close while hard modes do not. |  | F-13.22.3 | R-13.22.3 |
| US-13.22.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that AI within the configured time window of the player does not rubber-band, **so that** the system only activates beyond the gap threshold. |  | F-13.22.3 | R-13.22.3 |
## AI Racing Behavior (F-13.22.3c)
| US-13.22.3 | player (P-23) | **As a** player (P-23), **I want** AI racers to vary in aggressiveness for collision avoidance and line defense, **so that** each opponent feels unique. |  | F-13.22.3 | R-13.22.3 |
| US-13.22.3 | designer (P-5) | **As a** designer (P-5), **I want** to author AI personality profiles (cautious, balanced, aggressive) as data assets, **so that** I can diversify the field. |  | F-13.22.3 | R-13.22.3 |
| US-13.22.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that an aggressive AI defends its racing line through corners, **so that** personality behavior is observable. |  | F-13.22.3 | R-13.22.3 |
## Drift Scoring and Boost (F-13.22.4)
| US-13.22.4.1 | player (P-23) | **As a** player (P-23), **I want** sustained drifts detected from slip angle to accumulate score based on angle, speed, and duration, **so that** drifting is a skill-based scoring mechanic. |  | F-13.22.4 | R-13.22.4 |
| US-13.22.4.2 | player (P-23) | **As a** player (P-23), **I want** drift score to fill a boost meter that grants temporary speed boost when activated, **so that** skilled drifting rewards faster laps. |  | F-13.22.4 | R-13.22.4 |
| US-13.22.4.3 | player (P-23) | **As a** player (P-23), **I want** boost activation to produce nitro-style visual effects including exhaust flame and FOV widening, **so that** boost feels impactful. |  | F-13.22.4 | R-13.22.4 |
| US-13.22.4.4 | designer (P-5) | **As a** designer (P-5), **I want** to configure slip angle threshold, score scaling, boost meter size, and boost sources per game mode, **so that** drift and boost are tunable. |  | F-13.22.4 | R-13.22.4 |
| US-13.22.4.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a drift below the slip angle threshold does not trigger drift detection, **so that** the threshold is enforced. |  | F-13.22.4 | R-13.22.4 |
| US-13.22.4.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that boost speed matches the configured value and expires after the configured duration, **so that** boost parameters are correct. |  | F-13.22.4 | R-13.22.4 |
## Ghost Replay and Leaderboards (F-13.22.5)
| US-13.22.5.1 | player (P-23) | **As a** player (P-23), **I want** to race against a transparent ghost of my best run in time trial mode, **so that** I have a visual benchmark for improvement. |  | F-13.22.5 | R-13.22.5 |
| US-13.22.5.2 | player (P-23) | **As a** player (P-23), **I want** per-track leaderboards showing best times with player name, vehicle, and date, **so that** I can compete globally. |  | F-13.22.5 | R-13.22.5 |
| US-13.22.5.3 | player (P-23) | **As a** player (P-23), **I want** to download and race against other players' ghost data, **so that** I can challenge friends asynchronously. |  | F-13.22.5 | R-13.22.5 |
| US-13.22.5.4 | designer (P-5) | **As a** designer (P-5), **I want** ghost data stored as compressed input/position streams per track, **so that** ghost recordings are storage-efficient. |  | F-13.22.5 | R-13.22.5 |
| US-13.22.5.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that the ghost vehicle replays the exact recorded path, **so that** ghost accuracy is frame-precise. |  | F-13.22.5 | R-13.22.5 |
| US-13.22.5.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that leaderboard entries sync through platform services, **so that** global rankings update correctly. |  | F-13.22.5 | R-13.22.5 |
