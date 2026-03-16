# Animation State Machine & Morph Targets Test Cases

Companion test cases for [state-machine.md](state-machine.md).

## Unit Tests

### TC-9.4.1.1 State Graph Basic Transition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3-state graph (idle, walk, run), set "walk" trigger | State changes to "walk" within 1 frame | R-9.4.1 |
| 2 | 3-state graph, no trigger set | State remains "idle" | R-9.4.1 |

### TC-9.4.2.1 Transition Blend Weight Curve

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Linear curve, progress=0.25 | Blend weight = 0.25 | R-9.4.2 |
| 2 | Linear curve, progress=0.50 | Blend weight = 0.50 | R-9.4.2 |
| 3 | Linear curve, progress=0.75 | Blend weight = 0.75 | R-9.4.2 |
| 4 | Ease-in curve, progress=0.50 | Blend weight < 0.50 (slow start) | R-9.4.2 |
| 5 | Ease-out curve, progress=0.50 | Blend weight > 0.50 (slow end) | R-9.4.2 |

### TC-9.4.2.2 Per-Bone Blend Profile

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Upper-body duration=0.2s, lower-body duration=0.5s | Upper-body blend complete at 0.2s, lower at 0.5s | R-9.4.2 |

### TC-9.4.2.3 Sync Marker Alignment

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Walk-to-run transition with foot-contact sync markers | Foot-contact frames align within 1 frame | R-9.4.2 |

### TC-9.4.3.1 Sub-State Entry Exit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enter "combat" sub-state via named entry | Current state = combat sub-state entry node | R-9.4.3 |
| 2 | Trigger exit condition from combat sub-state | State returns to parent graph state | R-9.4.3 |

### TC-9.4.3.2 Sub-State Shared Definition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same sub-state nested in two parent graphs, modify sub-state | Both parent graphs reflect the change | R-9.4.3 |

### TC-9.4.4.1 Layer Override Bone Mask

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Upper-body combat + lower-body locomotion layers | Upper bones follow combat, lower bones follow locomotion | R-9.4.4 |

### TC-9.4.4.2 Layer Additive Blending

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Additive breathing layer, weight=1.0 | Per-joint result = base + delta, error < 0.001 | R-9.4.4 |
| 2 | Additive breathing layer, weight=0.5 | Per-joint result = base + 0.5*delta, error < 0.001 | R-9.4.4 |

### TC-9.4.4.3 Layer Weight Zero

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set layer weight to 0.0 | Pose reverts to base layer within 1 frame | R-9.4.4 |

### TC-9.4.5.1 Parameter Bool Transition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set bool "is_crouching"=true, condition="is_crouching==true" | Transition fires to crouch state | R-9.4.5 |
| 2 | Set bool "is_crouching"=false | Transition does not fire | R-9.4.5 |

### TC-9.4.5.2 Parameter Float Compare

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set float "speed"=2.0, condition="speed > 1.0" | Condition evaluates true | R-9.4.5 |
| 2 | Set float "speed"=0.5, condition="speed > 1.0" | Condition evaluates false | R-9.4.5 |

### TC-9.4.5.3 Trigger Auto Reset

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set trigger "attack", advance 2 frames | Trigger fires on frame 1, auto-resets on frame 2 | R-9.4.5 |
| 2 | Check trigger state after consumption | Trigger value = false | R-9.4.5 |

### TC-9.4.5.4 Trigger Same Frame

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set trigger "attack" twice on same frame | Only one transition fires | R-9.4.5 |

### TC-9.4.6.1 Sync Group Phase Alignment

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Walk (1.0s) and run (0.6s) in sync group, 50% blend, full cycle | Sync markers align within 1 frame throughout | R-9.4.6 |

### TC-9.4.7.1 Montage Override Bones

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Upper-body attack montage playing | Upper bones follow montage, lower bones follow state machine | R-9.4.7 |

### TC-9.4.7.2 Montage Section Branch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3-section montage, branch at section 2 to section 3 | Playback jumps from section 2 to section 3 | R-9.4.7 |
| 2 | 3-section montage, no branch | Plays sections 1, 2, 3 sequentially | R-9.4.7 |

### TC-9.4.7.3 Montage Blend Out

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stop montage, blend_out_duration=0.3s | Blend completes in 0.3s, state machine resumes | R-9.4.7 |

### TC-9.4.7.4 Montage Notify Fires

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Notify at time 0.5s in montage, play montage | Notify fires exactly once when time passes 0.5s | R-9.4.7 |

### TC-9.4.7.5 Montage Scoped Notify

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Montage-scoped notify, state machine evaluation (no montage) | Notify does not fire | R-9.4.7 |

### TC-9.4.8.1 Blend Space 1D Interpolation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3 samples at positions 0.0, 0.5, 1.0; evaluate at 0.25 | Weights: sample[0]=0.5, sample[1]=0.5 | R-9.4.8 |
| 2 | Evaluate at 0.0 | Weights: sample[0]=1.0, others=0.0 | R-9.4.8 |

### TC-9.4.8.2 Blend Space 2D Barycentric

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 9 samples in 2D grid, evaluate at triangle center | Barycentric weights within 0.001 of expected | R-9.4.8 |

### TC-9.4.8.3 Blend Space 2D Continuity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sweep parameter 0.0 to 1.0 in 100 steps | No discontinuities in output weights (max delta < 0.02) | R-9.4.8 |

### TC-9.4.9.1 Aim Offset Additive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | pitch=45 deg, yaw=30 deg | Weapon direction within 2 degrees of target | R-9.4.9 |

### TC-9.4.9.2 Aim Offset Lower Body Unaffected

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Aim offset active, lower-body bones | Per-bone delta from base pose < 0.001 units | R-9.4.9 |

### TC-9.2.1.1 Morph GPU Accumulation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 16 morph targets applied, 10K vertices | GPU output within 0.001 of CPU reference per vertex | R-9.2.1 |
| 2 | 1 morph target, weight=0.0 | Output matches base mesh exactly | R-9.2.1 |

### TC-9.2.1.2 Morph Sparse Storage

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Morph with 100 active deltas on 10K vertex mesh | Memory proportional to 100 deltas, not 10K vertices | R-9.2.1 |

### TC-9.2.2.1 Corrective Shape Activation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Elbow at 0 degrees | Corrective weight = 0.0 | R-9.2.2 |
| 2 | Elbow at 120 degrees | Corrective weight = 0.0 | R-9.2.2 |
| 3 | Elbow at 180 degrees | Corrective weight = 1.0 | R-9.2.2 |
| 4 | Elbow at 150 degrees | Corrective weight between 0.0 and 1.0 (smooth ramp) | R-9.2.2 |

### TC-9.2.2.2 Corrective Volume Restoration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Elbow at 150 degrees, corrective active | Mesh volume >= 90% of expected reference volume | R-9.2.2 |

### TC-9.2.3.1 Facial Action Unit Mapping

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Performance capture with 52 action units | All action units map to correct blend shapes | R-9.2.3 |

### TC-9.2.3.2 Facial 100 NPCs

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100 NPCs with unique AU weight sets | All 100 display distinct facial poses | R-9.2.3 |

### TC-9.2.4.1 VAT Playback Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 120-frame VAT, sample each frame | Deformed mesh within 0.5 mm per vertex of source | R-9.2.4 |

### TC-9.2.4.2 VAT Zero CPU Cost

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100 VAT meshes, measure CPU time | CPU time increase over static mesh baseline < 0.01 ms | R-9.2.4 |

### TC-9.2.5.1 Morph Stream LRU Eviction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fill morph cache to capacity, access new target | LRU target evicted, new target loaded | R-9.2.5 |

### TC-9.2.5.2 Morph Stream Visible Only

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10 characters, 5 off-screen | Only 5 visible characters have GPU-resident morph sets | R-9.2.5 |

## Integration Tests

### TC-9.4.1.I1 1000 Instances Under 1ms

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000 entities with active state graphs, 60 frames | CPU eval time < 1 ms per frame | US-9.4.1.3 |

### TC-9.4.10.I1 500 AI Agents Under 2ms

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 500 AI agents driving animation via behavior trees | Combined cost < 2 ms per frame | US-9.4.10.3 |

### TC-9.4.1.I2 Per Instance Memory Under 1KB

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Measure StateInstance size (excluding clip data) | Size < 1024 bytes | R-9.4.1 |

### TC-9.4.2.I1 Foot Sliding Under 1cm

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Walk-to-run with sync markers, measure foot displacement | Foot sliding distance < 1 cm | US-9.4.2.2 |

### TC-9.4.4.I1 Layer Count Per Platform

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile config | Max layers = 2 | US-9.4.4.3 |
| 2 | Switch config | Max layers = 3 | US-9.4.4.3 |
| 3 | Desktop config | Max layers >= 4 | US-9.4.4.3 |

### TC-9.4.8.I1 Blend Space Samples Per Platform

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile config | Max samples = 6-8 | US-9.4.8.3 |
| 2 | Switch config | Max samples = 12 | US-9.4.8.3 |
| 3 | Desktop config | Max samples >= 16 | US-9.4.8.3 |

### TC-9.2.1.I1 Morph Targets Per Platform

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile config | Max active targets = 8-16 | US-9.2.1.3 |
| 2 | Switch config | Max active targets = 16-32 | US-9.2.1.3 |
| 3 | Desktop config | Max active targets >= 64 | US-9.2.1.3 |

### TC-9.2.3.I1 Facial Units Per Platform

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile config | Action unit count = 16-24 | US-9.2.3.3 |
| 2 | Desktop config | Action unit count >= 52 | US-9.2.3.3 |

### TC-9.4.7.I1 Hundreds Concurrent Montages

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 200 simultaneous montages, 60 frames | No animation system bottleneck (frame time stable) | US-9.4.7.2 |

### TC-9.2.5.I1 Morph Stream 500 Characters

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 500 unique morph sets exceeding GPU budget | Only visible characters GPU-resident, LRU eviction active | R-9.2.5 |

### TC-9.2.5.I2 Morph Stream No Visible PopIn

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Move camera away and back to character | Morph targets streamed in before character fully visible | R-9.2.5 |

### TC-9.4.10.I2 AI Query Remaining Time

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | AI queries remaining clip time mid-playback | Accuracy within 0.01s of actual remaining | R-9.4.10 |

### TC-9.4.10.I3 AI Query Root Motion

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | AI queries root motion displacement | Accuracy within 0.01 units of actual | R-9.4.10 |

### TC-9.2.4.I1 VAT Half-Res Mobile

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile config, VAT mesh | Uses half-resolution VAT texture | US-9.2.4.3 |

### TC-9.2.2.I1 Corrective Disabled Mobile Non-Hero

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile config, non-hero character | Corrective shapes disabled | US-9.2.2.2 |

### TC-9.2.5.I3 Platform Async IO Backend

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Windows, morph streaming I/O | Uses IOCP backend | US-9.2.5.3 |
| 2 | macOS, morph streaming I/O | Uses GCD backend | US-9.2.5.3 |
| 3 | Linux, morph streaming I/O | Uses io_uring backend | US-9.2.5.3 |

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
