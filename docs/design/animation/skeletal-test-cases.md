# Skeletal Animation Test Cases

Companion test cases for [skeletal.md](skeletal.md).

## Unit Tests

### TC-9.1.2.1 Hermite Interpolation Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-9.1.2     |
| 2 | R-9.1.2     |
| 3 | R-9.1.2     |

1. **#1** — Hermite curve, sample at t=0.5
   - **Expected:** Result within 0.001 units of analytical value
2. **#2** — Hermite curve, sample at t=0.0
   - **Expected:** Result equals start keyframe exactly
3. **#3** — Hermite curve, sample at t=1.0
   - **Expected:** Result equals end keyframe exactly

### TC-9.1.2.2 Playback Mode Loop

| # | Requirement |
|---|-------------|
| 1 | R-9.1.2     |
| 2 | R-9.1.2     |

1. **#1** — 1.0s clip, PlaybackMode::Loop, time=1.5s
   - **Expected:** Wrapped time = 0.5s
2. **#2** — 1.0s clip, PlaybackMode::Loop, time=3.7s
   - **Expected:** Wrapped time = 0.7s

### TC-9.1.2.3 Playback Mode Clamp

| # | Requirement |
|---|-------------|
| 1 | R-9.1.2     |
| 2 | R-9.1.2     |

1. **#1** — 1.0s clip, PlaybackMode::Clamp, time=1.5s
   - **Expected:** Clamped time = 1.0s
2. **#2** — 1.0s clip, PlaybackMode::Clamp, time=-0.5s
   - **Expected:** Clamped time = 0.0s

### TC-9.1.2.4 Playback Mode PingPong

| # | Requirement |
|---|-------------|
| 1 | R-9.1.2     |
| 2 | R-9.1.2     |

1. **#1** — 1.0s clip, PlaybackMode::PingPong, time=1.5s
   - **Expected:** Reversed time = 0.5s
2. **#2** — 1.0s clip, PlaybackMode::PingPong, time=2.5s
   - **Expected:** Forward time = 0.5s (second cycle)

### TC-9.1.3.1 Blend 8 Clips Equal Weight

| # | Requirement |
|---|-------------|
| 1 | R-9.1.3     |
| 2 | R-9.1.3     |

1. **#1** — 8 clips, each weight=0.125, 256 bones
   - **Expected:** Per-joint result within 0.001 of CPU reference
2. **#2** — 2 clips, weights 0.7 and 0.3, 256 bones
   - **Expected:** Per-joint result within 0.001 of weighted average

### TC-9.1.3.2 Cubic Blend Continuity

| # | Requirement |
|---|-------------|
| 1 | R-9.1.3     |

1. **#1** — Cubic blend, sample at 10 intermediate weights 0.0 to 1.0
   - **Expected:** Second-derivative continuous (no sudden jumps)

### TC-9.1.4.1 Bone Mask Subtree

| # | Requirement |
|---|-------------|
| 1 | R-9.1.4     |
| 2 | R-9.1.4     |

1. **#1** — Mask from upper_body root bone (spine_01)
   - **Expected:** Masked set includes all spine, arm, hand, head bones
2. **#2** — Mask from left_arm root bone
   - **Expected:** Masked set includes left shoulder, arm, forearm, hand

### TC-9.1.4.2 Additive Layer

| # | Requirement |
|---|-------------|
| 1 | R-9.1.4     |
| 2 | R-9.1.4     |

1. **#1** — Base pose + additive breathing delta, weight=1.0
   - **Expected:** Result = base + delta, per-joint error < 0.001
2. **#2** — Base pose + additive delta, weight=0.5
   - **Expected:** Result = base + 0.5*delta, per-joint error < 0.001

### TC-9.1.4.3 Override Layer

| # | Requirement |
|---|-------------|
| 1 | R-9.1.4     |
| 2 | R-9.1.4     |

1. **#1** — Override layer with upper-body mask, weight=1.0
   - **Expected:** Masked bones follow override pose exclusively
2. **#2** — Override layer with upper-body mask, weight=1.0
   - **Expected:** Non-masked bones follow base layer exclusively

### TC-9.1.6.1 Root Motion Delta

| # | Requirement |
|---|-------------|
| 1 | R-9.1.6     |
| 2 | R-9.1.6     |

1. **#1** — Dodge roll clip, extract root motion
   - **Expected:** Displacement matches root bone delta within 0.01 units
2. **#2** — Idle clip, extract root motion
   - **Expected:** Root displacement = (0,0,0)

### TC-9.1.6.2 Root Motion Zeroing

| # | Requirement |
|---|-------------|
| 1 | R-9.1.6     |

1. **#1** — After root extraction from dodge roll
   - **Expected:** Root bone local transform = identity for extracted channels

### TC-9.1.7.1 Compression Ratio 10x

| # | Requirement |
|---|-------------|
| 1 | R-9.1.7     |

1. **#1** — Humanoid walk cycle, 256 bones, 120 frames
   - **Expected:** Compression ratio >= 10:1

### TC-9.1.7.2 Compression Error Threshold

| # | Requirement |
|---|-------------|
| 1 | R-9.1.7     |
| 2 | R-9.1.7     |

1. **#1** — Compress and decompress humanoid walk
   - **Expected:** Per-joint error < 0.5 mm
2. **#2** — Compress and decompress combat animation
   - **Expected:** Per-joint error < 0.5 mm

### TC-9.1.7.3 Smallest-Three Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-9.1.7     |
| 2 | R-9.1.7     |

1. **#1** — Quaternion (0.5, 0.5, 0.5, 0.5), encode/decode via smallest-three
   - **Expected:** Dot product with original > 0.9999
2. **#2** — Quaternion (1.0, 0, 0, 0), encode/decode
   - **Expected:** Dot product with original > 0.9999

### TC-9.1.8.1 Retarget Direct Copy

| # | Requirement |
|---|-------------|
| 1 | R-9.1.8     |

1. **#1** — Finger bones in DirectCopy mode, source and target skeletons
   - **Expected:** Target transforms match source exactly (bitwise)

### TC-9.1.8.2 Retarget Scaled Translation

| # | Requirement |
|---|-------------|
| 1 | R-9.1.8     |
| 2 | R-9.1.8     |

1. **#1** — Root bone, scale_ratio=1.2
   - **Expected:** Target translation = source * 1.2
2. **#2** — Root bone, scale_ratio=0.8
   - **Expected:** Target translation = source * 0.8

### TC-9.1.8.3 Retarget No NaN

| # | Requirement |
|---|-------------|
| 1 | R-9.1.8     |
| 2 | R-9.1.8     |

1. **#1** — Human mocap retargeted to quadruped skeleton
   - **Expected:** Zero NaN values in any output transform
2. **#2** — Human mocap retargeted to 50-bone creature
   - **Expected:** Zero NaN or Inf values in output

### TC-9.1.9.1 Event Fires at Frame

| # | Requirement |
|---|-------------|
| 1 | R-9.1.9     |
| 2 | R-9.1.9     |

1. **#1** — Event at frame 10, advance past frame 10
   - **Expected:** Event fires exactly once
2. **#2** — Event at frame 10, advance to frame 9
   - **Expected:** Event does not fire

### TC-9.1.9.2 Window Event Active Range

| # | Requirement |
|---|-------------|
| 1 | R-9.1.9     |

1. **#1** — Window event at frames 20-25, advance frames 0-30
   - **Expected:** Event active for exactly 6 frames (20,21,22,23,24,25)

### TC-9.1.10.1 LOD Tier Selection

| # | Requirement |
|---|-------------|
| 1 | R-9.1.10    |
| 2 | R-9.1.10    |
| 3 | R-9.1.10    |
| 4 | R-9.1.10    |

1. **#1** — Camera at 5 m
   - **Expected:** LodTier::Full
2. **#2** — Camera at 50 m
   - **Expected:** LodTier::Reduced
3. **#3** — Camera at 100 m
   - **Expected:** LodTier::HalfRate
4. **#4** — Camera at 200 m
   - **Expected:** LodTier::Vat

### TC-9.1.10.2 LOD Hero Bias

| # | Requirement |
|---|-------------|
| 1 | R-9.1.10    |
| 2 | R-9.1.10    |

1. **#1** — hero_bias active, camera at 200 m
   - **Expected:** LodTier::Full (bias overrides distance)
2. **#2** — hero_bias inactive, camera at 200 m
   - **Expected:** LodTier::Vat (normal tier selection)

## Integration Tests

### TC-9.1.1.I1 GPU Skinning LBS Twist

| # | Requirement |
|---|-------------|
| 1 | R-9.1.1     |

1. **#1** — Forearm mesh rotated 180 degrees, LBS mode
   - **Expected:** GPU vertex positions within 0.001 of CPU reference

### TC-9.1.1.I2 GPU Skinning DQS Twist

| # | Requirement |
|---|-------------|
| 1 | R-9.1.1     |

1. **#1** — Forearm mesh rotated 180 degrees, DQS mode
   - **Expected:** Waist cross-section area matches reference (no candy-wrapping)

### TC-9.1.2.I1 GPU Keyframe vs CPU

| # | Requirement |
|---|-------------|
| 1 | R-9.1.2     |

1. **#1** — 60-frame clip at t=15.5, GPU vs CPU
   - **Expected:** Per-joint position error < 0.001 units

### TC-9.1.5.I1 Instanced 1000

| # | Requirement |
|---|-------------|
| 1 | R-9.1.5     |
| 2 | R-9.1.5     |

1. **#1** — 1000 instances sharing 10 clips, 50 random samples
   - **Expected:** All poses match single-instance evaluation
2. **#2** — 1000 instances, verify dispatch
   - **Expected:** Single GPU dispatch confirmed

### TC-9.1.5.I2 Instanced Frame Time

| # | Requirement |
|---|-------------|
| 1 | R-9.1.5     |

1. **#1** — 1000 instances, measure GPU time
   - **Expected:** GPU time < 2.0 ms on reference hardware

### TC-9.1.6.I1 Root Motion Physics

| # | Requirement |
|---|-------------|
| 1 | R-9.1.6     |

1. **#1** — Dodge roll with root motion + physics capsule
   - **Expected:** Capsule displacement matches root delta, collision active

### TC-9.1.8.I1 Retarget Cross Species

| # | Requirement |
|---|-------------|
| 1 | R-9.1.8     |

1. **#1** — Human walk retargeted to quadruped
   - **Expected:** No bone explosions, foot contact timing preserved

### TC-9.1.10.I1 LOD 200 Characters

| # | Requirement |
|---|-------------|
| 1 | R-9.1.10    |

1. **#1** — 200 characters at 5-500 m distances
   - **Expected:** Correct tier per distance, no visible popping

### TC-9.1.1.I3 Full Pipeline Frame

| # | Requirement              |
|---|--------------------------|
| 1 | R-9.1.1 through R-9.1.10 |

1. **#1** — 50 characters, various LOD tiers, 1 frame
   - **Expected:** No errors, all buffers written, events dispatched

## Benchmarks

### TC-9.1.2.B1 GPU Keyframe Evaluation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 256 bones, single clip | GPU time | < 0.1 ms | US-9.1.2.1 |

### TC-9.1.3.B1 GPU Blend

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 8 clips, 256 bones | GPU time | < 0.1 ms | US-9.1.3.1 |

### TC-9.1.1.B1 GPU Skinning LBS

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 50k vertices, LBS mode | GPU time | < 0.5 ms | US-9.1.1.1 |

### TC-9.1.1.B2 GPU Skinning DQS

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 50k vertices, DQS mode | GPU time | < 0.7 ms | US-9.1.1.2 |

### TC-9.1.5.B1 Instanced Evaluation 500

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 instances, shared clips | GPU time | < 1.0 ms | US-9.1.5.2 |

### TC-9.1.5.B2 Instanced Evaluation 1000

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1000 instances, shared clips | GPU time | < 2.0 ms | US-9.1.5.2 |

### TC-9.1.7.B1 Compression

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1000-frame clip compression | CPU time | < 50 ms | US-9.1.7.1 |

### TC-9.1.8.B1 Retarget

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 256-bone skeleton retarget | CPU time | < 0.05 ms | US-9.1.8.1 |

### TC-9.1.10.B1 LOD System

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 200 entities, LOD evaluation | CPU time | < 0.1 ms | US-9.1.10.1 |
