# Perception and Scoring Systems -- Test Cases

Companion test cases for [perception.md](perception.md).

## Unit Tests -- Perception

### TC-13.18.1.1 Sense Score Distance Only

| # | Requirement |
|---|-------------|
| 1 | R-13.18.1   |

1. **#1** -- Single Distance factor, target at half range
   - **Expected:** Score equals curve sample at 0.5 times weight

### TC-13.18.1.2 Sense Score Multi Factor

| # | Requirement |
|---|-------------|
| 1 | R-13.18.1   |

1. **#1** -- Four factors (Distance, Light, Speed, Cover)
   - **Expected:** Weighted sum matches expected value

### TC-13.18.1.3 Sense FoV Inside

| # | Requirement |
|---|-------------|
| 1 | R-13.18.1   |

1. **#1** -- Target within 120-degree FoV
   - **Expected:** Score is non-zero

### TC-13.18.1.4 Sense FoV Outside

| # | Requirement |
|---|-------------|
| 1 | R-13.18.1   |

1. **#1** -- Target outside FoV
   - **Expected:** Score is zero

### TC-13.18.1.5 Stealth Bonus Reduces Score

| # | Requirement |
|---|-------------|
| 1 | R-13.18.1   |

1. **#1** -- Target with `stealth_bonus = 0.3`
   - **Expected:** Final score reduced by 0.3

### TC-13.18.1.6 Score Clamp Zero One

| # | Requirement |
|---|-------------|
| 1 | R-13.18.1   |
| 2 | R-13.18.1   |

1. **#1** -- Factors sum to 1.5
   - **Expected:** Score clamped to 1.0
2. **#2** -- Factors sum to negative
   - **Expected:** Score clamped to 0.0

### TC-13.18.1.7 Curve Sample Linear

| # | Requirement |
|---|-------------|
| 1 | R-13.18.1   |

1. **#1** -- Linear curve, input 0.5
   - **Expected:** Output equals 0.5

### TC-13.18.1.8 Curve Sample Inverse

| # | Requirement |
|---|-------------|
| 1 | R-13.18.1   |

1. **#1** -- Inverse curve, input 0.3
   - **Expected:** Output equals 0.7

### TC-13.18.2.1 Awareness Unaware to Suspicious

| # | Requirement |
|---|-------------|
| 1 | R-13.18.2   |

1. **#1** -- Score exceeds suspicious threshold for N frames
   - **Expected:** Level transitions from `Unaware` to `Suspicious`

### TC-13.18.2.2 Awareness Suspicious to Searching

| # | Requirement |
|---|-------------|
| 1 | R-13.18.2   |

1. **#1** -- Sustained score above search threshold
   - **Expected:** Level transitions to `Searching`

### TC-13.18.2.3 Awareness Searching to Alerted

| # | Requirement |
|---|-------------|
| 1 | R-13.18.2   |

1. **#1** -- Score above alert threshold for N frames
   - **Expected:** Level transitions to `Alerted`

### TC-13.18.2.4 Awareness Alerted to Lost

| # | Requirement |
|---|-------------|
| 1 | R-13.18.2   |

1. **#1** -- Score decays below alert threshold
   - **Expected:** Level transitions to `LostTarget`

### TC-13.18.2.5 Awareness Lost to Unaware

| # | Requirement |
|---|-------------|
| 1 | R-13.18.2   |

1. **#1** -- Timeout expires without new stimulus
   - **Expected:** Level reverts to `Unaware`

### TC-13.18.2.6 Awareness Hysteresis Blocks

| # | Requirement |
|---|-------------|
| 1 | R-13.18.2   |

1. **#1** -- Score above threshold for fewer than N frames
   - **Expected:** No transition occurs

### TC-13.18.2.7 Awareness Hysteresis Passes

| # | Requirement |
|---|-------------|
| 1 | R-13.18.2   |

1. **#1** -- Score above threshold for exactly N frames
   - **Expected:** Transition occurs

### TC-13.18.2.8 Awareness Timeout Revert

| # | Requirement |
|---|-------------|
| 1 | R-13.18.2   |

1. **#1** -- No stimulus for `timeout_secs`
   - **Expected:** Automatic demotion to lower state

### TC-13.18.2.9 Awareness Score Decay

| # | Requirement |
|---|-------------|
| 1 | R-13.18.2   |

1. **#1** -- No stimulus; tick 1 second
   - **Expected:** Score reduced by `score_decay_rate`

### TC-13.18.3.1 Stimulus Hearing Omnidirectional

| # | Requirement |
|---|-------------|
| 1 | R-13.18.3   |

1. **#1** -- Hearing sense with no FoV; target behind source
   - **Expected:** Target is detected

### TC-13.18.3.2 Stimulus Intensity Falloff

| # | Requirement |
|---|-------------|
| 1 | R-13.18.3   |

1. **#1** -- Stimulus at half radius
   - **Expected:** Intensity reduced by distance curve

### TC-13.18.3.3 Stimulus Stacks With Sense

| # | Requirement |
|---|-------------|
| 1 | R-13.18.3   |

1. **#1** -- Stimulus + continuous sense evaluation
   - **Expected:** Score is sum of both contributions

### TC-13.20.2.1 Faction Vision Merge

| # | Requirement |
|---|-------------|
| 1 | R-13.20.2   |

1. **#1** -- Two sources, same faction, different targets
   - **Expected:** `FactionVisionMap` contains union of targets

### TC-13.20.2.2 Faction Vision Any Source

| # | Requirement |
|---|-------------|
| 1 | R-13.20.2   |

1. **#1** -- One source sees target, another does not
   - **Expected:** `is_visible(target)` returns `true`

### TC-13.20.2.3 Faction Highest Awareness

| # | Requirement |
|---|-------------|
| 1 | R-13.20.2   |

1. **#1** -- Source A at `Suspicious`, source B at `Alerted`
   - **Expected:** `highest_awareness` returns `Alerted`

### TC-13.18.1.9 LOD Full Every Frame

| # | Requirement |
|---|-------------|
| 1 | R-13.18.1   |

1. **#1** -- Source at `Full` LOD tier
   - **Expected:** Sense evaluated every frame

### TC-13.18.1.10 LOD Reduced Skip Frames

| # | Requirement |
|---|-------------|
| 1 | R-13.18.1   |

1. **#1** -- Source at `Reduced` LOD tier
   - **Expected:** Sense evaluated every 2nd frame only

### TC-13.18.1.11 LOD Dormant No Eval

| # | Requirement |
|---|-------------|
| 1 | R-13.18.1   |

1. **#1** -- Source at `Dormant` LOD tier
   - **Expected:** No evaluation occurs

## Unit Tests -- Scored List

### TC-13.19.6.1 Scored List Add

| # | Requirement |
|---|-------------|
| 1 | R-13.19.6   |

1. **#1** -- `apply(entity, Add, 50.0)`
   - **Expected:** Score increases by 50

### TC-13.19.6.2 Scored List Set

| # | Requirement |
|---|-------------|
| 1 | R-13.19.6   |

1. **#1** -- `apply(entity, Set, 100.0)` on entry with score 30
   - **Expected:** Score is exactly 100 regardless of previous

### TC-13.19.6.3 Scored List Multiply

| # | Requirement |
|---|-------------|
| 1 | R-13.19.6   |

1. **#1** -- `apply(entity, Multiply, 2.0)` on entry with score 25
   - **Expected:** Score is 50.0

### TC-13.19.6.4 Scored List Decay

| # | Requirement |
|---|-------------|
| 1 | R-13.19.6   |

1. **#1** -- Decay at 5/s for 1 second
   - **Expected:** Score reduced by 5

### TC-13.19.6.5 Scored List Decay Minimum

| # | Requirement |
|---|-------------|
| 1 | R-13.19.6   |

1. **#1** -- Decay below minimum (minimum = 10.0)
   - **Expected:** Score clamped to 10.0

### TC-13.19.6.6 Scored List Remove at Zero

| # | Requirement |
|---|-------------|
| 1 | R-13.19.6   |

1. **#1** -- Entry decays to zero; `remove_at_zero = true`
   - **Expected:** Entry removed from list

### TC-13.19.6.7 Scored List Capacity Eviction

| # | Requirement |
|---|-------------|
| 1 | R-13.19.6   |

1. **#1** -- List at capacity 3; insert 4th entry
   - **Expected:** Lowest-ranked entry evicted

### TC-13.19.6.8 Scored List Sort Descending

| # | Requirement |
|---|-------------|
| 1 | R-13.19.6   |

1. **#1** -- Three entries with scores 10, 30, 20
   - **Expected:** Sorted order is [30, 20, 10]

### TC-13.19.6.9 Scored List Sort Ascending

| # | Requirement |
|---|-------------|
| 1 | R-13.19.6   |

1. **#1** -- Three entries with scores 10, 30, 20
   - **Expected:** Sorted order is [10, 20, 30]

### TC-13.19.6.10 Scored List Top

| # | Requirement |
|---|-------------|
| 1 | R-13.19.6   |

1. **#1** -- Insert entries with scores 10, 50, 30
   - **Expected:** `top()` returns entry with score 50

### TC-13.19.6.11 Threshold Rising

| # | Requirement |
|---|-------------|
| 1 | R-13.19.6   |

1. **#1** -- Score crosses threshold 100.0 upward
   - **Expected:** `ScoreThresholdEvent` fired with correct tag

### TC-13.19.6.12 Threshold Falling

| # | Requirement |
|---|-------------|
| 1 | R-13.19.6   |

1. **#1** -- Score crosses threshold 10.0 downward
   - **Expected:** `ScoreThresholdEvent` fired with correct tag

### TC-13.19.6.13 Threshold No Double Fire

| # | Requirement |
|---|-------------|
| 1 | R-13.19.6   |

1. **#1** -- Score oscillates around threshold boundary
   - **Expected:** Event fires once per crossing, not repeated

## Integration Tests

### TC-13.18.1.I1 Stealth Full Pipeline

| # | Requirement |
|---|-------------|
| 1 | R-13.18.1   |

1. **#1** -- Player with stealth gear; guard with vision sense
   - **Expected:** Awareness progresses through all 5 states as distance and light change

### TC-13.20.2.I1 Fog of War Faction Vision

| # | Requirement |
|---|-------------|
| 1 | R-13.20.2   |

1. **#1** -- Two units share a faction; one moves forward
   - **Expected:** Shared `FactionVisionMap` reveals new area for both units

### TC-13.19.6.I1 Threat Table Ability Modifier

| # | Requirement |
|---|-------------|
| 1 | R-13.19.6   |

1. **#1** -- Cast taunt ability targeting NPC
   - **Expected:** Caster's threat entry jumps to top of NPC's scored list

### TC-13.19.3a.I1 NPC Deed Observation Chain

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3a  |

1. **#1** -- Player performs action; NPC source detects via vision; `AwarenessTransitionEvent`
   triggers `DeedObserver`
   - **Expected:** Memory entry created in NPC deed memory

### TC-13.18.1.I2 LOD Promotion Demotion

| # | Requirement |
|---|-------------|
| 1 | R-13.18.1   |

1. **#1** -- Move player away from source past LOD boundaries
   - **Expected:** LOD demotes and eval rate decreases; moving back promotes and eval rate increases

### TC-13.19.6.I2 Perception Plus Scored List

| # | Requirement |
|---|-------------|
| 1 | R-13.19.6   |

1. **#1** -- Perception detects target; score feeds into scored list as threat
   - **Expected:** Threat entry created with correct initial score

### TC-13.18.1.I3 100 Sources 500 Targets Budget

| # | Requirement |
|---|-------------|
| 1 | NFR-PERC.1  |

1. **#1** -- Spawn 100 sources and 500 targets; measure frame
   - **Expected:** Total perception evaluation < 2 ms

### TC-13.19.6.I3 50 Lists Budget

| # | Requirement |
|---|-------------|
| 1 | NFR-PERC.2  |

1. **#1** -- 50 scored lists, 20 entries each; run modifiers and decay
   - **Expected:** Total scored list update < 0.5 ms

### TC-13.18.1.I4 Stimulus to Awareness Latency

| # | Requirement |
|---|-------------|
| 1 | NFR-PERC.3  |

1. **#1** -- Fire stimulus; measure frames until awareness transition
   - **Expected:** Transition completes within 1 frame

## Benchmarks

### TC-13.18.1.B1 Perception Throughput

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | 100 sources, 500 targets | Frame time | < 2 ms | NFR-PERC.1 |

### TC-13.19.6.B1 Scored List Throughput

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | 50 lists, 20 entries each | Frame time | < 0.5 ms | NFR-PERC.2 |

### TC-13.18.1.B2 Single Sense Eval

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | 1 sense, 4 factors, 1 target | Eval time | < 10 us | NFR-PERC.1 |

### TC-13.18.2.B1 Awareness Transition Check

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | Single awareness pair | Transition check | < 1 us | NFR-PERC.1 |

### TC-13.19.6.B2 Scored List Insert Plus Sort

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | Insert 1 entry into 20-entry list | Insert + sort time | < 5 us | NFR-PERC.2 |

### TC-13.19.6.B3 Scored List Decay

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | Decay 20 entries | Total decay time | < 2 us | NFR-PERC.2 |

### TC-13.20.2.B1 Faction Vision Merge

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | Merge vision for 1 faction | Merge time | < 100 us | R-13.20.2 |

### TC-13.18.1.B3 Curve Sample Latency

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | Single curve sample (linear interp) | Sample time | < 100 ns | NFR-PERC.1 |
