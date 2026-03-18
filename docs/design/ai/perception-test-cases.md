# AI Perception Test Cases

Companion test cases for [perception.md](perception.md).

## Unit Tests

### TC-7.6.1.1 Sight Cone Boundary

| # | Requirement |
|---|-------------|
| 1 | R-7.6.1     |
| 2 | R-7.6.1     |

1. **#1** — Target at cone half-angle (45 deg), range=20m, distance=15m, LOS clear
   - **Expected:** Detected = true
2. **#2** — Target at half-angle + 1 deg (46 deg), range=20m, distance=15m, LOS clear
   - **Expected:** Detected = false

### TC-7.6.1.2 Sight LOS Blocked

| # | Requirement |
|---|-------------|
| 1 | R-7.6.1     |
| 2 | R-7.6.1     |

1. **#1** — Target inside cone (20 deg), range=20m, distance=10m, wall between
   - **Expected:** Detected = false
2. **#2** — Target inside cone (20 deg), range=20m, distance=10m, no wall
   - **Expected:** Detected = true

### TC-7.6.1.3 Sight Range Boundary

| # | Requirement |
|---|-------------|
| 1 | R-7.6.1     |
| 2 | R-7.6.1     |

1. **#1** — Target at max range (30m), inside cone, LOS clear
   - **Expected:** Detected = true
2. **#2** — Target at max range + 1m (31m), inside cone, LOS clear
   - **Expected:** Detected = false

### TC-7.6.1.4 Sight Trace Channel

| # | Requirement |
|---|-------------|
| 1 | R-7.6.1     |
| 2 | R-7.6.1     |

1. **#1** — Glass wall, trace channel A (blocks glass)
   - **Expected:** LOS blocked; detected = false
2. **#2** — Glass wall, trace channel B (ignores glass)
   - **Expected:** LOS clear; detected = true

### TC-7.6.1.5 Sight Peripheral Falloff

| # | Requirement |
|---|-------------|
| 1 | R-7.6.1     |
| 2 | R-7.6.1     |

1. **#1** — Target at cone center (0 deg), range=20m, distance=10m
   - **Expected:** Confidence = 1.0
2. **#2** — Target at cone edge (44 deg of 45 deg half-angle), same range/distance
   - **Expected:** Confidence < 0.5

### TC-7.6.2.1 Hearing Distance Attenuation

| # | Requirement |
|---|-------------|
| 1 | R-7.6.2     |
| 2 | R-7.6.2     |
| 3 | R-7.6.2     |

1. **#1** — Sound at distance=0, max_radius=50m
   - **Expected:** Confidence = 1.0
2. **#2** — Sound at distance=50m, max_radius=50m
   - **Expected:** Confidence approximately 0.0
3. **#3** — Sound at distance=25m, max_radius=50m
   - **Expected:** 0.0 < Confidence < 1.0

### TC-7.6.2.2 Hearing Occlusion

| # | Requirement |
|---|-------------|
| 1 | R-7.6.2     |
| 2 | R-7.6.2     |

1. **#1** — Sound at 10m, no wall between emitter and listener
   - **Expected:** Confidence = C1
2. **#2** — Sound at 10m, concrete wall between emitter and listener
   - **Expected:** Confidence < C1

### TC-7.6.2.3 Hearing Intensity Scaling

| # | Requirement |
|---|-------------|
| 1 | R-7.6.2     |
| 2 | R-7.6.2     |

1. **#1** — Gunshot (intensity=3.0), max_radius=50m
   - **Expected:** Detectable at 45m
2. **#2** — Footstep (intensity=1.0), max_radius=50m
   - **Expected:** Not detectable at 45m

### TC-7.6.3.1 Damage Bypasses LOS

| # | Requirement |
|---|-------------|
| 1 | R-7.6.3     |

1. **#1** — Damage from outside vision cone and beyond sight range
   - **Expected:** Detected = true (damage sense bypasses all)

### TC-7.6.3.2 Damage Direction Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-7.6.3     |
| 2 | R-7.6.3     |

1. **#1** — Damage from direction (1, 0, 0); agent at origin
   - **Expected:** Reported direction within 5 degrees of (1, 0, 0)
2. **#2** — Damage from direction (-0.7, 0, 0.7); agent at origin
   - **Expected:** Reported direction within 5 degrees of actual

### TC-7.6.3.3 Damage Threshold

| # | Requirement |
|---|-------------|
| 1 | R-7.6.3     |
| 2 | R-7.6.3     |

1. **#1** — Damage amount=1.0, threshold=5.0
   - **Expected:** Ignored (below threshold)
2. **#2** — Damage amount=10.0, threshold=5.0
   - **Expected:** Detected

### TC-7.6.4.1 Faction Hostile Detected

| # | Requirement |
|---|-------------|
| 1 | R-7.6.4     |

1. **#1** — Stimulus from hostile faction, filter enabled
   - **Expected:** Stimulus not filtered; passed to sense eval

### TC-7.6.4.2 Faction Friendly Filtered

| # | Requirement |
|---|-------------|
| 1 | R-7.6.4     |
| 2 | R-7.6.4     |

1. **#1** — Stimulus from friendly faction, friendly filter enabled
   - **Expected:** Stimulus filtered out
2. **#2** — Stimulus from friendly faction, friendly filter disabled
   - **Expected:** Stimulus passed to sense eval

### TC-7.6.4.3 Faction Runtime Change

| # | Requirement |
|---|-------------|
| 1 | R-7.6.4     |

1. **#1** — Change faction B from friendly to hostile at runtime
   - **Expected:** Next perception tick treats B as hostile

### TC-7.6.4.4 Faction Override Precedence

| # | Requirement |
|---|-------------|
| 1 | R-7.6.4     |
| 2 | R-7.6.4     |

1. **#1** — Faction default=hostile; per-entity override=friendly
   - **Expected:** Entity treated as friendly
2. **#2** — Faction default=friendly; per-entity override=hostile
   - **Expected:** Entity treated as hostile

### TC-7.6.5.1 Stimulus Registration

| # | Requirement |
|---|-------------|
| 1 | R-7.6.5     |
| 2 | R-7.6.5     |

1. **#1** — Register noise stimulus at (10, 0, 10), radius=5m
   - **Expected:** Spatial query at (10, 0, 10) returns stimulus
2. **#2** — Register noise stimulus at (10, 0, 10), radius=5m
   - **Expected:** Spatial query at (100, 0, 100) returns empty

### TC-7.6.5.2 Stimulus Expiration

| # | Requirement |
|---|-------------|
| 1 | R-7.6.5     |
| 2 | R-7.6.5     |

1. **#1** — Register stimulus with TTL=2.0s; query at t=1.0
   - **Expected:** Stimulus found
2. **#2** — Register stimulus with TTL=2.0s; query at t=3.0
   - **Expected:** Stimulus expired; not found

### TC-7.6.5.3 Stimulus Spatial Filter

| # | Requirement |
|---|-------------|
| 1 | R-7.6.5     |

1. **#1** — 3 stimuli at distances 5m, 15m, 25m; query radius=10m
   - **Expected:** Returns only the 5m stimulus

### TC-7.6.5.4 Stimulus Cap Enforced

| # | Requirement |
|---|-------------|
| 1 | R-7.6.5     |
| 2 | R-7.6.5     |

1. **#1** — Capacity=1024; register 1025th stimulus
   - **Expected:** Returns None (at capacity)
2. **#2** — Capacity=1024; register 1024th stimulus
   - **Expected:** Returns Some(handle)

### TC-7.6.6.1 Memory Decay Linear

| # | Requirement |
|---|-------------|
| 1 | R-7.6.6     |
| 2 | R-7.6.6     |

1. **#1** — Initial confidence=1.0, decay_rate=0.1/s, dt=2.0s
   - **Expected:** Confidence = 0.8
2. **#2** — Initial confidence=1.0, decay_rate=0.5/s, dt=1.0s
   - **Expected:** Confidence = 0.5

### TC-7.6.6.2 Memory Refresh Resets

| # | Requirement |
|---|-------------|
| 1 | R-7.6.6     |

1. **#1** — Confidence decayed to 0.3; re-confirmation occurs
   - **Expected:** Confidence reset to 1.0

### TC-7.6.6.3 Memory Expiry

| # | Requirement |
|---|-------------|
| 1 | R-7.6.6     |
| 2 | R-7.6.6     |

1. **#1** — Confidence = 0.01, decay_rate=0.1/s, dt=0.2s
   - **Expected:** Entry removed (confidence <= 0)
2. **#2** — Confidence = 0.5, decay_rate=0.1/s, dt=0.2s
   - **Expected:** Entry retained (confidence > 0)

### TC-7.6.6.4 Memory Archetype Duration

| # | Requirement |
|---|-------------|
| 1 | R-7.6.6     |

1. **#1** — Archetype A: memory_duration=10s; Archetype B: memory_duration=20s; same decay rate
   - **Expected:** B entry retained 2x longer than A

### TC-7.6.7.1 Custom Sense Registration

| # | Requirement |
|---|-------------|
| 1 | R-7.6.7     |

1. **#1** — Register custom "tremor" sense with ID=100
   - **Expected:** Sense invoked during evaluation cycle

### TC-7.6.7.2 Custom Sense Duplicate Rejected

| # | Requirement |
|---|-------------|
| 1 | R-7.6.7     |

1. **#1** — Register sense ID=100; register sense ID=100 again
   - **Expected:** Error::DuplicateSenseId

### TC-7.6.7.3 Budget High Priority Always Runs

| # | Requirement |
|---|-------------|
| 1 | R-7.6.7     |

1. **#1** — 200 agents, priority-0 agent with budget=250us
   - **Expected:** Priority-0 agent evaluated every tick

### TC-7.6.7.4 Budget Low Priority Deferred

| # | Requirement |
|---|-------------|
| 1 | R-7.6.7     |

1. **#1** — 200 agents, budget=250us, 50 low-priority agents
   - **Expected:** Low-priority agents deferred when budget exhausted

### TC-7.6.8.1 Scent Deposit and Query

| # | Requirement |
|---|-------------|
| 1 | R-7.6.8     |
| 2 | R-7.6.8     |

1. **#1** — Deposit scent intensity=1.0 at cell (5, 0, 5); query cell (5, 0, 5)
   - **Expected:** Returns intensity > 0
2. **#2** — Deposit scent at cell (5, 0, 5); query cell (50, 0, 50)
   - **Expected:** Returns intensity = 0

### TC-7.6.8.2 Scent Decay

| # | Requirement |
|---|-------------|
| 1 | R-7.6.8     |
| 2 | R-7.6.8     |

1. **#1** — Scent intensity=1.0, decay_rate=0.2/s, dt=3.0s
   - **Expected:** Intensity = 0.4
2. **#2** — Scent intensity=1.0, decay_rate=1.0/s, dt=1.5s
   - **Expected:** Intensity = 0.0 (fully decayed)

### TC-7.6.8.3 Scent Wind Drift

| # | Requirement |
|---|-------------|
| 1 | R-7.6.8     |
| 2 | R-7.6.8     |

1. **#1** — Scent at origin, wind direction=(1,0,0); query at (10,0,0)
   - **Expected:** Detectable (downwind)
2. **#2** — Scent at origin, wind direction=(1,0,0); query at (-10,0,0)
   - **Expected:** Not detectable (upwind)

### TC-7.6.8.4 Scent Blocked by Door

| # | Requirement |
|---|-------------|
| 1 | R-7.6.8     |
| 2 | R-7.6.8     |

1. **#1** — Scent in room A; closed door between room A and B
   - **Expected:** No scent propagation into room B
2. **#2** — Scent in room A; open door between rooms
   - **Expected:** Scent propagates into room B

### TC-7.6.8.5 Scent Rain Dilution

| # | Requirement |
|---|-------------|
| 1 | R-7.6.8     |
| 2 | R-7.6.8     |

1. **#1** — Scent decay_rate=0.2/s, rain_multiplier=2.0, dt=1.0s
   - **Expected:** Effective decay = 0.4 (accelerated)
2. **#2** — Scent decay_rate=0.2/s, no rain, dt=1.0s
   - **Expected:** Effective decay = 0.2 (normal)

### TC-7.6.9.1 Evidence Footprint Spawn

| # | Requirement |
|---|-------------|
| 1 | R-7.6.9     |
| 2 | R-7.6.9     |

1. **#1** — Agent steps on deformable surface (snow)
   - **Expected:** Footprint evidence entity spawned
2. **#2** — Agent steps on hard surface (stone)
   - **Expected:** No footprint spawned

### TC-7.6.9.2 Evidence Decay

| # | Requirement |
|---|-------------|
| 1 | R-7.6.9     |
| 2 | R-7.6.9     |

1. **#1** — Footprint with decay_timer=30s; advance 31s
   - **Expected:** Evidence entity removed
2. **#2** — Footprint with decay_timer=30s; advance 10s
   - **Expected:** Evidence entity still present

### TC-7.6.9.3 Evidence Direction Inference

| # | Requirement |
|---|-------------|
| 1 | R-7.6.9     |

1. **#1** — 5 footprints in a line heading north
   - **Expected:** TrackingSense infers direction = north

### TC-7.6.10.1 Investigation Claim

| # | Requirement |
|---|-------------|
| 1 | R-7.6.10    |

1. **#1** — Stimulus emitted; Agent A and B both nearby
   - **Expected:** Agent A claims; Agent B continues patrol

### TC-7.6.10.2 Investigation Timeout

| # | Requirement |
|---|-------------|
| 1 | R-7.6.10    |

1. **#1** — Agent investigating, timeout=10s, advance 11s
   - **Expected:** Investigation ends; state = Unaware

### TC-7.6.10.3 Alert State Suspicious

| # | Requirement |
|---|-------------|
| 1 | R-7.6.10    |

1. **#1** — Below-threshold stimulus (confidence=0.3, threshold=0.5)
   - **Expected:** Alert state transitions to Suspicious

### TC-7.6.10.4 Alert State Alerted

| # | Requirement |
|---|-------------|
| 1 | R-7.6.10    |

1. **#1** — Confirmed threat (confidence=0.9, threshold=0.5)
   - **Expected:** Alert state transitions to Alerted

### TC-7.6.11.1 Tracking Method Transition

| # | Requirement |
|---|-------------|
| 1 | R-7.6.11    |
| 2 | R-7.6.11    |

1. **#1** — Visual tracking active; target goes behind wall
   - **Expected:** Tracking switches to audio (if audible)
2. **#2** — Audio tracking active; target goes silent
   - **Expected:** Tracking switches to scent (if trail exists)

### TC-7.6.11.2 Tracking Confidence Decay

| # | Requirement |
|---|-------------|
| 1 | R-7.6.11    |

1. **#1** — Tracking confidence=0.8, no refresh for 5s, decay=0.1/s
   - **Expected:** Confidence = 0.3

### TC-7.6.11.3 Tracking Pack Sharing

| # | Requirement |
|---|-------------|
| 1 | R-7.6.11    |

1. **#1** — Pack member A sights target at (10, 0, 20)
   - **Expected:** All pack members update last-known position to (10, 0, 20)

### TC-7.6.4.5 Threat Score Ranking

| # | Requirement |
|---|-------------|
| 1 | R-7.6.4     |

1. **#1** — Hostile A at 5m; Hostile B at 20m
   - **Expected:** A ranked higher (closer = more threatening)

### TC-7.6.6.5 Awareness Level Thresholds

| # | Requirement |
|---|-------------|
| 1 | R-7.6.6     |
| 2 | R-7.6.6     |
| 3 | R-7.6.6     |

1. **#1** — Confidence=0.9, thresholds: [Known>=0.8, Suspected>=0.4, Unknown>=0.0]
   - **Expected:** Awareness = Known
2. **#2** — Confidence=0.5
   - **Expected:** Awareness = Suspected
3. **#3** — Confidence=0.1
   - **Expected:** Awareness = Unknown

## Integration Tests

### TC-7.6.1.I1 Full Perception Pipeline

| # | Requirement |
|---|-------------|
| 1 | R-7.6.1     |

1. **#1** — Register stimulus; run scheduler; check PerceivedEntities
   - **Expected:** PerceivedEntities contains stimulus source entity

### TC-7.6.1.I2 Parallel 500 Agents

| # | Requirement |
|---|-------------|
| 1 | R-7.6.1     |

1. **#1** — 500 agents with sight + hearing, parallel evaluation
   - **Expected:** All agents have correct PerceivedEntities; no data races

### TC-7.6.10.I1 Investigation to Alert Cycle

| # | Requirement |
|---|-------------|
| 1 | R-7.6.10    |

1. **#1** — Sound -> Suspicious -> investigate -> find target -> Alerted -> lose target -> Unaware
   - **Expected:** Full state cycle completes correctly

### TC-7.6.11.I1 Multi-Sense Tracking

| # | Requirement |
|---|-------------|
| 1 | R-7.6.11    |

1. **#1** — Visual -> target hides -> audio -> target quiet -> scent -> found
   - **Expected:** Seamless transitions between sense modes

### TC-7.6.7.I1 Mobile Budget Compliance

| # | Requirement |
|---|-------------|
| 1 | R-7.6.7     |

1. **#1** — Budget=250us, 200 agents
   - **Expected:** High-priority agents run; low-priority deferred; total < 250us

### TC-7.6.8.I1 Scent Trail Following

| # | Requirement |
|---|-------------|
| 1 | R-7.6.8     |

1. **#1** — Entity walks 50m path; tracker starts at origin
   - **Expected:** Tracker follows scent trail to entity's position

### TC-7.6.9.I1 Footprint Tracking

| # | Requirement |
|---|-------------|
| 1 | R-7.6.9     |

1. **#1** — Entity walks through snow; tracker finds first footprint
   - **Expected:** Tracker follows chain to entity's last position

### TC-7.6.4.I1 Faction Betrayal Scenario

| # | Requirement |
|---|-------------|
| 1 | R-7.6.4     |

1. **#1** — Faction B: friendly -> hostile mid-game
   - **Expected:** Agents immediately perceive faction B as hostile

## Benchmarks

### TC-7.6.1.B1 Sight Evaluation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 agents, 1000 targets, sight eval | Total wall time | < 500 us | R-7.6.1 |

### TC-7.6.2.B1 Hearing Evaluation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 agents, 200 stimuli, hearing eval | Total wall time | < 200 us | R-7.6.2 |

### TC-7.6.5.B1 Stimulus Registry Query

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 2048 registered stimuli, single spatial query | Wall time | < 50 us | R-7.6.5 |

### TC-7.6.6.B1 Memory Decay and Expiry

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 agents, memory decay + expiry pass | Total wall time | < 100 us | R-7.6.6 |

### TC-7.6.8.B1 Scent Grid Propagation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 64x64x16 scent grid, single propagation step | Wall time | < 200 us | R-7.6.8 |

### TC-7.6.4.B1 Threat Scoring

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 agents, threat scoring pass | Total wall time | < 50 us | R-7.6.4 |

### TC-7.6.7.B1 Full Pipeline

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 agents, full perception pipeline, desktop | Total wall time | < 1000 us | R-7.6.7 |
