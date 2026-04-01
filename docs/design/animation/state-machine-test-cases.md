# Animation State Machine & Morph Targets Test Cases

Companion test cases for [state-machine.md](state-machine.md).

## Unit Tests

### TC-9.4.1.1 State Graph Basic Transition

| # | Requirement |
|---|-------------|
| 1 | R-9.4.1     |
| 2 | R-9.4.1     |

1. **#1** — 3-state graph (idle, walk, run), set "walk" trigger
   - **Expected:** State changes to "walk" within 1 frame
2. **#2** — 3-state graph, no trigger set
   - **Expected:** State remains "idle"

### TC-9.4.2.1 Transition Blend Weight Curve

| # | Requirement |
|---|-------------|
| 1 | R-9.4.2     |
| 2 | R-9.4.2     |
| 3 | R-9.4.2     |
| 4 | R-9.4.2     |
| 5 | R-9.4.2     |

1. **#1** — Linear curve, progress=0.25
   - **Expected:** Blend weight = 0.25
2. **#2** — Linear curve, progress=0.50
   - **Expected:** Blend weight = 0.50
3. **#3** — Linear curve, progress=0.75
   - **Expected:** Blend weight = 0.75
4. **#4** — Ease-in curve, progress=0.50
   - **Expected:** Blend weight < 0.50 (slow start)
5. **#5** — Ease-out curve, progress=0.50
   - **Expected:** Blend weight > 0.50 (slow end)

### TC-9.4.2.2 Per-Bone Blend Profile

| # | Requirement |
|---|-------------|
| 1 | R-9.4.2     |

1. **#1** — Upper-body duration=0.2s, lower-body duration=0.5s
   - **Expected:** Upper-body blend complete at 0.2s, lower at 0.5s

### TC-9.4.2.3 Sync Marker Alignment

| # | Requirement |
|---|-------------|
| 1 | R-9.4.2     |

1. **#1** — Walk-to-run transition with foot-contact sync markers
   - **Expected:** Foot-contact frames align within 1 frame

### TC-9.4.3.1 Sub-State Entry Exit

| # | Requirement |
|---|-------------|
| 1 | R-9.4.3     |
| 2 | R-9.4.3     |

1. **#1** — Enter "combat" sub-state via named entry
   - **Expected:** Current state = combat sub-state entry node
2. **#2** — Trigger exit condition from combat sub-state
   - **Expected:** State returns to parent graph state

### TC-9.4.3.2 Sub-State Shared Definition

| # | Requirement |
|---|-------------|
| 1 | R-9.4.3     |

1. **#1** — Same sub-state nested in two parent graphs, modify sub-state
   - **Expected:** Both parent graphs reflect the change

### TC-9.4.4.1 Layer Override Bone Mask

| # | Requirement |
|---|-------------|
| 1 | R-9.4.4     |

1. **#1** — Upper-body combat + lower-body locomotion layers
   - **Expected:** Upper bones follow combat, lower bones follow locomotion

### TC-9.4.4.2 Layer Additive Blending

| # | Requirement |
|---|-------------|
| 1 | R-9.4.4     |
| 2 | R-9.4.4     |

1. **#1** — Additive breathing layer, weight=1.0
   - **Expected:** Per-joint result = base + delta, error < 0.001
2. **#2** — Additive breathing layer, weight=0.5
   - **Expected:** Per-joint result = base + 0.5*delta, error < 0.001

### TC-9.4.4.3 Layer Weight Zero

| # | Requirement |
|---|-------------|
| 1 | R-9.4.4     |

1. **#1** — Set layer weight to 0.0
   - **Expected:** Pose reverts to base layer within 1 frame

### TC-9.4.5.1 Parameter Bool Transition

| # | Requirement |
|---|-------------|
| 1 | R-9.4.5     |
| 2 | R-9.4.5     |

1. **#1** — Set bool "is_crouching"=true, condition="is_crouching==true"
   - **Expected:** Transition fires to crouch state
2. **#2** — Set bool "is_crouching"=false
   - **Expected:** Transition does not fire

### TC-9.4.5.2 Parameter Float Compare

| # | Requirement |
|---|-------------|
| 1 | R-9.4.5     |
| 2 | R-9.4.5     |

1. **#1** — Set float "speed"=2.0, condition="speed > 1.0"
   - **Expected:** Condition evaluates true
2. **#2** — Set float "speed"=0.5, condition="speed > 1.0"
   - **Expected:** Condition evaluates false

### TC-9.4.5.3 Trigger Auto Reset

| # | Requirement |
|---|-------------|
| 1 | R-9.4.5     |
| 2 | R-9.4.5     |

1. **#1** — Set trigger "attack", advance 2 frames
   - **Expected:** Trigger fires on frame 1, auto-resets on frame 2
2. **#2** — Check trigger state after consumption
   - **Expected:** Trigger value = false

### TC-9.4.5.4 Trigger Same Frame

| # | Requirement |
|---|-------------|
| 1 | R-9.4.5     |

1. **#1** — Set trigger "attack" twice on same frame
   - **Expected:** Only one transition fires

### TC-9.4.6.1 Sync Group Phase Alignment

| # | Requirement |
|---|-------------|
| 1 | R-9.4.6     |

1. **#1** — Walk (1.0s) and run (0.6s) in sync group, 50% blend, full cycle
   - **Expected:** Sync markers align within 1 frame throughout

### TC-9.4.7.1 Montage Override Bones

| # | Requirement |
|---|-------------|
| 1 | R-9.4.7     |

1. **#1** — Upper-body attack montage playing
   - **Expected:** Upper bones follow montage, lower bones follow state machine

### TC-9.4.7.2 Montage Section Branch

| # | Requirement |
|---|-------------|
| 1 | R-9.4.7     |
| 2 | R-9.4.7     |

1. **#1** — 3-section montage, branch at section 2 to section 3
   - **Expected:** Playback jumps from section 2 to section 3
2. **#2** — 3-section montage, no branch
   - **Expected:** Plays sections 1, 2, 3 sequentially

### TC-9.4.7.3 Montage Blend Out

| # | Requirement |
|---|-------------|
| 1 | R-9.4.7     |

1. **#1** — Stop montage, blend_out_duration=0.3s
   - **Expected:** Blend completes in 0.3s, state machine resumes

### TC-9.4.7.4 Montage Notify Fires

| # | Requirement |
|---|-------------|
| 1 | R-9.4.7     |

1. **#1** — Notify at time 0.5s in montage, play montage
   - **Expected:** Notify fires exactly once when time passes 0.5s

### TC-9.4.7.5 Montage Scoped Notify

| # | Requirement |
|---|-------------|
| 1 | R-9.4.7     |

1. **#1** — Montage-scoped notify, state machine evaluation (no montage)
   - **Expected:** Notify does not fire

### TC-9.4.8.1 Blend Space 1D Interpolation

| # | Requirement |
|---|-------------|
| 1 | R-9.4.8     |
| 2 | R-9.4.8     |

1. **#1** — 3 samples at positions 0.0, 0.5, 1.0; evaluate at 0.25
   - **Expected:** Weights: sample[0]=0.5, sample[1]=0.5
2. **#2** — Evaluate at 0.0
   - **Expected:** Weights: sample[0]=1.0, others=0.0

### TC-9.4.8.2 Blend Space 2D Barycentric

| # | Requirement |
|---|-------------|
| 1 | R-9.4.8     |

1. **#1** — 9 samples in 2D grid, evaluate at triangle center
   - **Expected:** Barycentric weights within 0.001 of expected

### TC-9.4.8.3 Blend Space 2D Continuity

| # | Requirement |
|---|-------------|
| 1 | R-9.4.8     |

1. **#1** — Sweep parameter 0.0 to 1.0 in 100 steps
   - **Expected:** No discontinuities in output weights (max delta < 0.02)

### TC-9.4.9.1 Aim Offset Additive

| # | Requirement |
|---|-------------|
| 1 | R-9.4.9     |

1. **#1** — pitch=45 deg, yaw=30 deg
   - **Expected:** Weapon direction within 2 degrees of target

### TC-9.4.9.2 Aim Offset Lower Body Unaffected

| # | Requirement |
|---|-------------|
| 1 | R-9.4.9     |

1. **#1** — Aim offset active, lower-body bones
   - **Expected:** Per-bone delta from base pose < 0.001 units

### TC-9.2.1.1 Morph GPU Accumulation

| # | Requirement |
|---|-------------|
| 1 | R-9.2.1     |
| 2 | R-9.2.1     |

1. **#1** — 16 morph targets applied, 10K vertices
   - **Expected:** GPU output within 0.001 of CPU reference per vertex
2. **#2** — 1 morph target, weight=0.0
   - **Expected:** Output matches base mesh exactly

### TC-9.2.1.2 Morph Sparse Storage

| # | Requirement |
|---|-------------|
| 1 | R-9.2.1     |

1. **#1** — Morph with 100 active deltas on 10K vertex mesh
   - **Expected:** Memory proportional to 100 deltas, not 10K vertices

### TC-9.2.2.1 Corrective Shape Activation

| # | Requirement |
|---|-------------|
| 1 | R-9.2.2     |
| 2 | R-9.2.2     |
| 3 | R-9.2.2     |
| 4 | R-9.2.2     |

1. **#1** — Elbow at 0 degrees
   - **Expected:** Corrective weight = 0.0
2. **#2** — Elbow at 120 degrees
   - **Expected:** Corrective weight = 0.0
3. **#3** — Elbow at 180 degrees
   - **Expected:** Corrective weight = 1.0
4. **#4** — Elbow at 150 degrees
   - **Expected:** Corrective weight between 0.0 and 1.0 (smooth ramp)

### TC-9.2.2.2 Corrective Volume Restoration

| # | Requirement |
|---|-------------|
| 1 | R-9.2.2     |

1. **#1** — Elbow at 150 degrees, corrective active
   - **Expected:** Mesh volume >= 90% of expected reference volume

### TC-9.2.3.1 Facial Action Unit Mapping

| # | Requirement |
|---|-------------|
| 1 | R-9.2.3     |

1. **#1** — Performance capture with 52 action units
   - **Expected:** All action units map to correct blend shapes

### TC-9.2.3.2 Facial 100 NPCs

| # | Requirement |
|---|-------------|
| 1 | R-9.2.3     |

1. **#1** — 100 NPCs with unique AU weight sets
   - **Expected:** All 100 display distinct facial poses

### TC-9.2.4.1 VAT Playback Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-9.2.4     |

1. **#1** — 120-frame VAT, sample each frame
   - **Expected:** Deformed mesh within 0.5 mm per vertex of source

### TC-9.2.4.2 VAT Zero CPU Cost

| # | Requirement |
|---|-------------|
| 1 | R-9.2.4     |

1. **#1** — 100 VAT meshes, measure CPU time
   - **Expected:** CPU time increase over static mesh baseline < 0.01 ms

### TC-9.2.5.1 Morph Stream LRU Eviction

| # | Requirement |
|---|-------------|
| 1 | R-9.2.5     |

1. **#1** — Fill morph cache to capacity, access new target
   - **Expected:** LRU target evicted, new target loaded

### TC-9.2.5.2 Morph Stream Visible Only

| # | Requirement |
|---|-------------|
| 1 | R-9.2.5     |

1. **#1** — 10 characters, 5 off-screen
   - **Expected:** Only 5 visible characters have GPU-resident morph sets

## Integration Tests

### TC-9.4.1.I1 1000 Instances Under 1ms

| # | Requirement |
|---|-------------|
| 1 | US-9.4.1.3  |

1. **#1** — 1000 entities with active state graphs, 60 frames
   - **Expected:** CPU eval time < 1 ms per frame

### TC-9.4.10.I1 500 AI Agents Under 2ms

| # | Requirement |
|---|-------------|
| 1 | US-9.4.10.3 |

1. **#1** — 500 AI agents driving animation via behavior trees
   - **Expected:** Combined cost < 2 ms per frame

### TC-9.4.1.I2 Per Instance Memory Under 1KB

| # | Requirement |
|---|-------------|
| 1 | R-9.4.1     |

1. **#1** — Measure StateInstance size (excluding clip data)
   - **Expected:** Size < 1024 bytes

### TC-9.4.2.I1 Foot Sliding Under 1cm

| # | Requirement |
|---|-------------|
| 1 | US-9.4.2.2  |

1. **#1** — Walk-to-run with sync markers, measure foot displacement
   - **Expected:** Foot sliding distance < 1 cm

### TC-9.4.4.I1 Layer Count Per Platform

| # | Requirement |
|---|-------------|
| 1 | US-9.4.4.3  |
| 2 | US-9.4.4.3  |
| 3 | US-9.4.4.3  |

1. **#1** — Mobile config
   - **Expected:** Max layers = 2
2. **#2** — Switch config
   - **Expected:** Max layers = 3
3. **#3** — Desktop config
   - **Expected:** Max layers >= 4

### TC-9.4.8.I1 Blend Space Samples Per Platform

| # | Requirement |
|---|-------------|
| 1 | US-9.4.8.3  |
| 2 | US-9.4.8.3  |
| 3 | US-9.4.8.3  |

1. **#1** — Mobile config
   - **Expected:** Max samples = 6-8
2. **#2** — Switch config
   - **Expected:** Max samples = 12
3. **#3** — Desktop config
   - **Expected:** Max samples >= 16

### TC-9.2.1.I1 Morph Targets Per Platform

| # | Requirement |
|---|-------------|
| 1 | US-9.2.1.3  |
| 2 | US-9.2.1.3  |
| 3 | US-9.2.1.3  |

1. **#1** — Mobile config
   - **Expected:** Max active targets = 8-16
2. **#2** — Switch config
   - **Expected:** Max active targets = 16-32
3. **#3** — Desktop config
   - **Expected:** Max active targets >= 64

### TC-9.2.3.I1 Facial Units Per Platform

| # | Requirement |
|---|-------------|
| 1 | US-9.2.3.3  |
| 2 | US-9.2.3.3  |

1. **#1** — Mobile config
   - **Expected:** Action unit count = 16-24
2. **#2** — Desktop config
   - **Expected:** Action unit count >= 52

### TC-9.4.7.I1 Hundreds Concurrent Montages

| # | Requirement |
|---|-------------|
| 1 | US-9.4.7.2  |

1. **#1** — 200 simultaneous montages, 60 frames
   - **Expected:** No animation system bottleneck (frame time stable)

### TC-9.2.5.I1 Morph Stream 500 Characters

| # | Requirement |
|---|-------------|
| 1 | R-9.2.5     |

1. **#1** — 500 unique morph sets exceeding GPU budget
   - **Expected:** Only visible characters GPU-resident, LRU eviction active

### TC-9.2.5.I2 Morph Stream No Visible PopIn

| # | Requirement |
|---|-------------|
| 1 | R-9.2.5     |

1. **#1** — Move camera away and back to character
   - **Expected:** Morph targets streamed in before character fully visible

### TC-9.4.10.I2 AI Query Remaining Time

| # | Requirement |
|---|-------------|
| 1 | R-9.4.10    |

1. **#1** — AI queries remaining clip time mid-playback
   - **Expected:** Accuracy within 0.01s of actual remaining

### TC-9.4.10.I3 AI Query Root Motion

| # | Requirement |
|---|-------------|
| 1 | R-9.4.10    |

1. **#1** — AI queries root motion displacement
   - **Expected:** Accuracy within 0.01 units of actual

### TC-9.2.4.I1 VAT Half-Res Mobile

| # | Requirement |
|---|-------------|
| 1 | US-9.2.4.3  |

1. **#1** — Mobile config, VAT mesh
   - **Expected:** Uses half-resolution VAT texture

### TC-9.2.2.I1 Corrective Disabled Mobile Non-Hero

| # | Requirement |
|---|-------------|
| 1 | US-9.2.2.2  |

1. **#1** — Mobile config, non-hero character
   - **Expected:** Corrective shapes disabled

### TC-9.2.5.I3 Platform Async IO Backend

| # | Requirement |
|---|-------------|
| 1 | US-9.2.5.3  |
| 2 | US-9.2.5.3  |
| 3 | US-9.2.5.3  |

1. **#1** — Windows, morph streaming I/O
   - **Expected:** Uses Tokio (IOCP)
2. **#2** — macOS, morph streaming I/O
   - **Expected:** Uses Tokio (kqueue)
3. **#3** — Linux, morph streaming I/O
   - **Expected:** Uses Tokio (epoll)

## Benchmarks

### TC-9.4.1.B1 State Graph Evaluation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1000 instances, state graph eval | CPU time | < 1 ms | US-9.4.1.3 |

### TC-9.4.10.B1 AI Plus Animation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 AI agents + animation eval | CPU time | < 2 ms | US-9.4.10.3 |

### TC-9.4.8.B1 Blend Space 2D Lookup

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single 2D blend space lookup | CPU time | < 500 ns | R-9.4.8 |

### TC-9.4.5.B1 Transition Condition Evaluation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single transition condition check | CPU time | < 100 ns | R-9.4.5 |

### TC-9.2.1.B1 Morph GPU Accumulate

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 16 targets, 10K vertices | GPU time | < 0.5 ms | R-9.2.1 |

### TC-9.2.5.B1 Morph Stream-In Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Cold stream-in of morph target set | Latency | < 100 ms | R-9.2.5 |

### TC-9.2.4.B1 VAT Playback Overhead

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100 VAT entities, CPU overhead | Per-entity cost | < 1 us | R-9.2.4 |

### TC-9.4.4.B1 Layer Composition

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 4 layers, 256 bones | CPU time | < 50 us | R-9.4.4 |

### TC-9.4.7.B1 Montage Notify Dispatch

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single montage notify dispatch | CPU time | < 10 us | R-9.4.7 |
