# AI Perception Test Cases

Companion test cases for [perception.md](perception.md).

## Unit Tests

### TC-7.6.1.1 Sight Cone Boundary

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Target at cone half-angle (45 deg), range=20m, distance=15m, LOS clear | Detected = true | R-7.6.1 |
| 2 | Target at half-angle + 1 deg (46 deg), range=20m, distance=15m, LOS clear | Detected = false | R-7.6.1 |

### TC-7.6.1.2 Sight LOS Blocked

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Target inside cone (20 deg), range=20m, distance=10m, wall between | Detected = false | R-7.6.1 |
| 2 | Target inside cone (20 deg), range=20m, distance=10m, no wall | Detected = true | R-7.6.1 |

### TC-7.6.1.3 Sight Range Boundary

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Target at max range (30m), inside cone, LOS clear | Detected = true | R-7.6.1 |
| 2 | Target at max range + 1m (31m), inside cone, LOS clear | Detected = false | R-7.6.1 |

### TC-7.6.1.4 Sight Trace Channel

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Glass wall, trace channel A (blocks glass) | LOS blocked; detected = false | R-7.6.1 |
| 2 | Glass wall, trace channel B (ignores glass) | LOS clear; detected = true | R-7.6.1 |

### TC-7.6.1.5 Sight Peripheral Falloff

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Target at cone center (0 deg), range=20m, distance=10m | Confidence = 1.0 | R-7.6.1 |
| 2 | Target at cone edge (44 deg of 45 deg half-angle), same range/distance | Confidence < 0.5 | R-7.6.1 |

### TC-7.6.2.1 Hearing Distance Attenuation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sound at distance=0, max_radius=50m | Confidence = 1.0 | R-7.6.2 |
| 2 | Sound at distance=50m, max_radius=50m | Confidence approximately 0.0 | R-7.6.2 |
| 3 | Sound at distance=25m, max_radius=50m | 0.0 < Confidence < 1.0 | R-7.6.2 |

### TC-7.6.2.2 Hearing Occlusion

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sound at 10m, no wall between emitter and listener | Confidence = C1 | R-7.6.2 |
| 2 | Sound at 10m, concrete wall between emitter and listener | Confidence < C1 | R-7.6.2 |

### TC-7.6.2.3 Hearing Intensity Scaling

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Gunshot (intensity=3.0), max_radius=50m | Detectable at 45m | R-7.6.2 |
| 2 | Footstep (intensity=1.0), max_radius=50m | Not detectable at 45m | R-7.6.2 |

### TC-7.6.3.1 Damage Bypasses LOS

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Damage from outside vision cone and beyond sight range | Detected = true (damage sense bypasses all) | R-7.6.3 |

### TC-7.6.3.2 Damage Direction Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Damage from direction (1, 0, 0); agent at origin | Reported direction within 5 degrees of (1, 0, 0) | R-7.6.3 |
| 2 | Damage from direction (-0.7, 0, 0.7); agent at origin | Reported direction within 5 degrees of actual | R-7.6.3 |

### TC-7.6.3.3 Damage Threshold

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Damage amount=1.0, threshold=5.0 | Ignored (below threshold) | R-7.6.3 |
| 2 | Damage amount=10.0, threshold=5.0 | Detected | R-7.6.3 |

### TC-7.6.4.1 Faction Hostile Detected

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stimulus from hostile faction, filter enabled | Stimulus not filtered; passed to sense eval | R-7.6.4 |

### TC-7.6.4.2 Faction Friendly Filtered

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stimulus from friendly faction, friendly filter enabled | Stimulus filtered out | R-7.6.4 |
| 2 | Stimulus from friendly faction, friendly filter disabled | Stimulus passed to sense eval | R-7.6.4 |

### TC-7.6.4.3 Faction Runtime Change

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Change faction B from friendly to hostile at runtime | Next perception tick treats B as hostile | R-7.6.4 |

### TC-7.6.4.4 Faction Override Precedence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Faction default=hostile; per-entity override=friendly | Entity treated as friendly | R-7.6.4 |
| 2 | Faction default=friendly; per-entity override=hostile | Entity treated as hostile | R-7.6.4 |

### TC-7.6.5.1 Stimulus Registration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register noise stimulus at (10, 0, 10), radius=5m | Spatial query at (10, 0, 10) returns stimulus | R-7.6.5 |
| 2 | Register noise stimulus at (10, 0, 10), radius=5m | Spatial query at (100, 0, 100) returns empty | R-7.6.5 |

### TC-7.6.5.2 Stimulus Expiration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register stimulus with TTL=2.0s; query at t=1.0 | Stimulus found | R-7.6.5 |
| 2 | Register stimulus with TTL=2.0s; query at t=3.0 | Stimulus expired; not found | R-7.6.5 |

### TC-7.6.5.3 Stimulus Spatial Filter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3 stimuli at distances 5m, 15m, 25m; query radius=10m | Returns only the 5m stimulus | R-7.6.5 |

### TC-7.6.5.4 Stimulus Cap Enforced

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Capacity=1024; register 1025th stimulus | Returns None (at capacity) | R-7.6.5 |
| 2 | Capacity=1024; register 1024th stimulus | Returns Some(handle) | R-7.6.5 |

### TC-7.6.6.1 Memory Decay Linear

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Initial confidence=1.0, decay_rate=0.1/s, dt=2.0s | Confidence = 0.8 | R-7.6.6 |
| 2 | Initial confidence=1.0, decay_rate=0.5/s, dt=1.0s | Confidence = 0.5 | R-7.6.6 |

### TC-7.6.6.2 Memory Refresh Resets

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Confidence decayed to 0.3; re-confirmation occurs | Confidence reset to 1.0 | R-7.6.6 |

### TC-7.6.6.3 Memory Expiry

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Confidence = 0.01, decay_rate=0.1/s, dt=0.2s | Entry removed (confidence <= 0) | R-7.6.6 |
| 2 | Confidence = 0.5, decay_rate=0.1/s, dt=0.2s | Entry retained (confidence > 0) | R-7.6.6 |

### TC-7.6.6.4 Memory Archetype Duration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Archetype A: memory_duration=10s; Archetype B: memory_duration=20s; same decay rate | B entry retained 2x longer than A | R-7.6.6 |

### TC-7.6.7.1 Custom Sense Registration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register custom "tremor" sense with ID=100 | Sense invoked during evaluation cycle | R-7.6.7 |

### TC-7.6.7.2 Custom Sense Duplicate Rejected

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register sense ID=100; register sense ID=100 again | Error::DuplicateSenseId | R-7.6.7 |

### TC-7.6.7.3 Budget High Priority Always Runs

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 200 agents, priority-0 agent with budget=250us | Priority-0 agent evaluated every tick | R-7.6.7 |

### TC-7.6.7.4 Budget Low Priority Deferred

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 200 agents, budget=250us, 50 low-priority agents | Low-priority agents deferred when budget exhausted | R-7.6.7 |

### TC-7.6.8.1 Scent Deposit and Query

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Deposit scent intensity=1.0 at cell (5, 0, 5); query cell (5, 0, 5) | Returns intensity > 0 | R-7.6.8 |
| 2 | Deposit scent at cell (5, 0, 5); query cell (50, 0, 50) | Returns intensity = 0 | R-7.6.8 |

### TC-7.6.8.2 Scent Decay

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Scent intensity=1.0, decay_rate=0.2/s, dt=3.0s | Intensity = 0.4 | R-7.6.8 |
| 2 | Scent intensity=1.0, decay_rate=1.0/s, dt=1.5s | Intensity = 0.0 (fully decayed) | R-7.6.8 |

### TC-7.6.8.3 Scent Wind Drift

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Scent at origin, wind direction=(1,0,0); query at (10,0,0) | Detectable (downwind) | R-7.6.8 |
| 2 | Scent at origin, wind direction=(1,0,0); query at (-10,0,0) | Not detectable (upwind) | R-7.6.8 |

### TC-7.6.8.4 Scent Blocked by Door

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Scent in room A; closed door between room A and B | No scent propagation into room B | R-7.6.8 |
| 2 | Scent in room A; open door between rooms | Scent propagates into room B | R-7.6.8 |

### TC-7.6.8.5 Scent Rain Dilution

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Scent decay_rate=0.2/s, rain_multiplier=2.0, dt=1.0s | Effective decay = 0.4 (accelerated) | R-7.6.8 |
| 2 | Scent decay_rate=0.2/s, no rain, dt=1.0s | Effective decay = 0.2 (normal) | R-7.6.8 |

### TC-7.6.9.1 Evidence Footprint Spawn

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Agent steps on deformable surface (snow) | Footprint evidence entity spawned | R-7.6.9 |
| 2 | Agent steps on hard surface (stone) | No footprint spawned | R-7.6.9 |

### TC-7.6.9.2 Evidence Decay

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Footprint with decay_timer=30s; advance 31s | Evidence entity removed | R-7.6.9 |
| 2 | Footprint with decay_timer=30s; advance 10s | Evidence entity still present | R-7.6.9 |

### TC-7.6.9.3 Evidence Direction Inference

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5 footprints in a line heading north | TrackingSense infers direction = north | R-7.6.9 |

### TC-7.6.10.1 Investigation Claim

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stimulus emitted; Agent A and B both nearby | Agent A claims; Agent B continues patrol | R-7.6.10 |

### TC-7.6.10.2 Investigation Timeout

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Agent investigating, timeout=10s, advance 11s | Investigation ends; state = Unaware | R-7.6.10 |

### TC-7.6.10.3 Alert State Suspicious

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Below-threshold stimulus (confidence=0.3, threshold=0.5) | Alert state transitions to Suspicious | R-7.6.10 |

### TC-7.6.10.4 Alert State Alerted

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Confirmed threat (confidence=0.9, threshold=0.5) | Alert state transitions to Alerted | R-7.6.10 |

### TC-7.6.11.1 Tracking Method Transition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Visual tracking active; target goes behind wall | Tracking switches to audio (if audible) | R-7.6.11 |
| 2 | Audio tracking active; target goes silent | Tracking switches to scent (if trail exists) | R-7.6.11 |

### TC-7.6.11.2 Tracking Confidence Decay

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Tracking confidence=0.8, no refresh for 5s, decay=0.1/s | Confidence = 0.3 | R-7.6.11 |

### TC-7.6.11.3 Tracking Pack Sharing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pack member A sights target at (10, 0, 20) | All pack members update last-known position to (10, 0, 20) | R-7.6.11 |

### TC-7.6.4.5 Threat Score Ranking

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hostile A at 5m; Hostile B at 20m | A ranked higher (closer = more threatening) | R-7.6.4 |

### TC-7.6.6.5 Awareness Level Thresholds

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Confidence=0.9, thresholds: [Known>=0.8, Suspected>=0.4, Unknown>=0.0] | Awareness = Known | R-7.6.6 |
| 2 | Confidence=0.5 | Awareness = Suspected | R-7.6.6 |
| 3 | Confidence=0.1 | Awareness = Unknown | R-7.6.6 |

## Integration Tests

### TC-7.6.1.I1 Full Perception Pipeline

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register stimulus; run scheduler; check PerceivedEntities | PerceivedEntities contains stimulus source entity | R-7.6.1 |

### TC-7.6.1.I2 Parallel 500 Agents

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 500 agents with sight + hearing, parallel evaluation | All agents have correct PerceivedEntities; no data races | R-7.6.1 |

### TC-7.6.10.I1 Investigation to Alert Cycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sound -> Suspicious -> investigate -> find target -> Alerted -> lose target -> Unaware | Full state cycle completes correctly | R-7.6.10 |

### TC-7.6.11.I1 Multi-Sense Tracking

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Visual -> target hides -> audio -> target quiet -> scent -> found | Seamless transitions between sense modes | R-7.6.11 |

### TC-7.6.7.I1 Mobile Budget Compliance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Budget=250us, 200 agents | High-priority agents run; low-priority deferred; total < 250us | R-7.6.7 |

### TC-7.6.8.I1 Scent Trail Following

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Entity walks 50m path; tracker starts at origin | Tracker follows scent trail to entity's position | R-7.6.8 |

### TC-7.6.9.I1 Footprint Tracking

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Entity walks through snow; tracker finds first footprint | Tracker follows chain to entity's last position | R-7.6.9 |

### TC-7.6.4.I1 Faction Betrayal Scenario

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Faction B: friendly -> hostile mid-game | Agents immediately perceive faction B as hostile | R-7.6.4 |

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
