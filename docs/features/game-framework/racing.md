# 13.22 — Racing Systems

### F-13.22.1 Track and Checkpoint System

Define race tracks using ordered checkpoint volumes placed in the world. Checkpoints are trigger
volumes (F-4.2.8) that record passage order and timestamps. The system enforces checkpoint order —
skipping a checkpoint invalidates the lap. Lap counting increments when the start/finish checkpoint
is crossed with all intermediate checkpoints registered. Split times compare current lap segment to
best lap and ghost. Track boundaries use trigger volumes that apply slowdown penalties or respawn
the vehicle at the last valid checkpoint. Multiple track layouts can share the same world geometry
with different checkpoint configurations.

- **Requirements:** R-13.22.1
- **Dependencies:** F-4.2.8 (Trigger Volumes), F-4.5.1 (Vehicle Physics)
- **Platform notes:** None

### F-13.22.2 Race Mode Framework

Configurable race modes: circuit (N laps, first to finish wins), time trial (solo, beat the clock),
elimination (last place eliminated each lap), knockout (timed rounds, lowest scorer eliminated),
drift challenge (score points for sustained drifts), drag race (quarter-mile acceleration), and
checkpoint race (reach checkpoints before time expires). Each mode defines: win/loss conditions,
scoring rules, timer behavior, elimination rules, and reward distribution. The mode framework is
data-driven — custom modes are assembled from composable rule components in the visual editor.

- **Requirements:** R-13.22.2
- **Dependencies:** F-13.22.1, F-13.1.1 (Game Mode Framework)
- **Platform notes:** None

### F-13.22.3a Racing AI Navigation

AI-controlled racers navigate tracks via waypoint splines with configurable racing lines,
braking points, and speed targets per segment. AI difficulty is controlled by top speed
limiter, braking accuracy, and racing line adherence. Spline data and per-segment parameters
are authored as data assets per track.

- **Requirements:** R-13.22.3a
- **Dependencies:** F-13.22.1, F-7.2.1 (Steering Behaviors)
- **Platform notes:** None

### F-13.22.3b Rubber-Banding Difficulty

Rubber-banding dynamically adjusts AI speed based on position relative to the player.
Trailing AI speeds up and leading AI slows down to maintain competitive gaps. Aggressive
rubber-banding keeps all racers within a configurable time window of the player. Rubber-band
intensity is a per-difficulty-tier parameter.

- **Requirements:** R-13.22.3b
- **Dependencies:** F-13.22.3a
- **Platform notes:** None

### F-13.22.3c AI Racing Behavior

AI racers avoid collisions using simplified avoidance steering, jostle for position on
straights, and defend racing lines through corners. Behavior aggressiveness is configurable
per AI personality profile (cautious, balanced, aggressive). Personality profiles are
data-driven assets authored in the visual editor.

- **Requirements:** R-13.22.3c
- **Dependencies:** F-13.22.3a, F-7.2.1 (Steering Behaviors)
- **Platform notes:** None

### F-13.22.4 Drift Scoring and Boost System

Detect and score sustained drifts based on slip angle, speed, and duration. Drift detection uses
the vehicle's lateral velocity relative to its forward direction — exceeding a slip angle threshold
triggers drift state. Drift score accumulates based on angle (higher angle = more points), speed
(faster = more points), and duration (combo multiplier). Drift score can fill a boost meter that
grants a temporary speed boost when activated. The boost system supports configurable boost sources:
drift-earned, collectible pickups, or recharging cooldown. Boost activates with nitro-style visual
effects (exhaust flame, motion lines, FOV widening).

- **Requirements:** R-13.22.4
- **Dependencies:** F-4.5.1 (Vehicle Physics), F-11.6.1 (Effect Graph)
- **Platform notes:** None

### F-13.22.5 Ghost Replay and Leaderboards

Record the player's best run as a ghost vehicle — a transparent replay of inputs and positions that
races alongside the player in time trial mode. Ghosts are stored as compressed input/position
streams referenced by track and player. Per-track leaderboards display best times with player
names, vehicle used, and date. Online leaderboards sync through the platform services (F-14.5.2)
for global competition. Ghost data can be shared between players for competitive challenges. The
ghost system reuses the replay recording infrastructure (F-8.6.1) with a lightweight vehicle-only
subset.

- **Requirements:** R-13.22.5
- **Dependencies:** F-8.6.1 (Replay Recording), F-14.5.2 (Leaderboards)
- **Platform notes:** None
