# Skeletal Animation Test Cases

Companion test cases for [skeletal.md](skeletal.md).

## Unit Tests

### TC-9.1.2.1 Hermite Interpolation Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hermite curve, sample at t=0.5 | Result within 0.001 units of analytical value | R-9.1.2 |
| 2 | Hermite curve, sample at t=0.0 | Result equals start keyframe exactly | R-9.1.2 |
| 3 | Hermite curve, sample at t=1.0 | Result equals end keyframe exactly | R-9.1.2 |

### TC-9.1.2.2 Playback Mode Loop

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1.0s clip, PlaybackMode::Loop, time=1.5s | Wrapped time = 0.5s | R-9.1.2 |
| 2 | 1.0s clip, PlaybackMode::Loop, time=3.7s | Wrapped time = 0.7s | R-9.1.2 |

### TC-9.1.2.3 Playback Mode Clamp

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1.0s clip, PlaybackMode::Clamp, time=1.5s | Clamped time = 1.0s | R-9.1.2 |
| 2 | 1.0s clip, PlaybackMode::Clamp, time=-0.5s | Clamped time = 0.0s | R-9.1.2 |

### TC-9.1.2.4 Playback Mode PingPong

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1.0s clip, PlaybackMode::PingPong, time=1.5s | Reversed time = 0.5s | R-9.1.2 |
| 2 | 1.0s clip, PlaybackMode::PingPong, time=2.5s | Forward time = 0.5s (second cycle) | R-9.1.2 |

### TC-9.1.3.1 Blend 8 Clips Equal Weight

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 8 clips, each weight=0.125, 256 bones | Per-joint result within 0.001 of CPU reference | R-9.1.3 |
| 2 | 2 clips, weights 0.7 and 0.3, 256 bones | Per-joint result within 0.001 of weighted average | R-9.1.3 |

### TC-9.1.3.2 Cubic Blend Continuity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cubic blend, sample at 10 intermediate weights 0.0 to 1.0 | Second-derivative continuous (no sudden jumps) | R-9.1.3 |

### TC-9.1.4.1 Bone Mask Subtree

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mask from upper_body root bone (spine_01) | Masked set includes all spine, arm, hand, head bones | R-9.1.4 |
| 2 | Mask from left_arm root bone | Masked set includes left shoulder, arm, forearm, hand | R-9.1.4 |

### TC-9.1.4.2 Additive Layer

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Base pose + additive breathing delta, weight=1.0 | Result = base + delta, per-joint error < 0.001 | R-9.1.4 |
| 2 | Base pose + additive delta, weight=0.5 | Result = base + 0.5*delta, per-joint error < 0.001 | R-9.1.4 |

### TC-9.1.4.3 Override Layer

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Override layer with upper-body mask, weight=1.0 | Masked bones follow override pose exclusively | R-9.1.4 |
| 2 | Override layer with upper-body mask, weight=1.0 | Non-masked bones follow base layer exclusively | R-9.1.4 |

### TC-9.1.6.1 Root Motion Delta

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Dodge roll clip, extract root motion | Displacement matches root bone delta within 0.01 units | R-9.1.6 |
| 2 | Idle clip, extract root motion | Root displacement = (0,0,0) | R-9.1.6 |

### TC-9.1.6.2 Root Motion Zeroing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | After root extraction from dodge roll | Root bone local transform = identity for extracted channels | R-9.1.6 |

### TC-9.1.7.1 Compression Ratio 10x

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Humanoid walk cycle, 256 bones, 120 frames | Compression ratio >= 10:1 | R-9.1.7 |

### TC-9.1.7.2 Compression Error Threshold

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compress and decompress humanoid walk | Per-joint error < 0.5 mm | R-9.1.7 |
| 2 | Compress and decompress combat animation | Per-joint error < 0.5 mm | R-9.1.7 |

### TC-9.1.7.3 Smallest-Three Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Quaternion (0.5, 0.5, 0.5, 0.5), encode/decode via smallest-three | Dot product with original > 0.9999 | R-9.1.7 |
| 2 | Quaternion (1.0, 0, 0, 0), encode/decode | Dot product with original > 0.9999 | R-9.1.7 |

### TC-9.1.8.1 Retarget Direct Copy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Finger bones in DirectCopy mode, source and target skeletons | Target transforms match source exactly (bitwise) | R-9.1.8 |

### TC-9.1.8.2 Retarget Scaled Translation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Root bone, scale_ratio=1.2 | Target translation = source * 1.2 | R-9.1.8 |
| 2 | Root bone, scale_ratio=0.8 | Target translation = source * 0.8 | R-9.1.8 |

### TC-9.1.8.3 Retarget No NaN

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Human mocap retargeted to quadruped skeleton | Zero NaN values in any output transform | R-9.1.8 |
| 2 | Human mocap retargeted to 50-bone creature | Zero NaN or Inf values in output | R-9.1.8 |

### TC-9.1.9.1 Event Fires at Frame

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Event at frame 10, advance past frame 10 | Event fires exactly once | R-9.1.9 |
| 2 | Event at frame 10, advance to frame 9 | Event does not fire | R-9.1.9 |

### TC-9.1.9.2 Window Event Active Range

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Window event at frames 20-25, advance frames 0-30 | Event active for exactly 6 frames (20,21,22,23,24,25) | R-9.1.9 |

### TC-9.1.10.1 LOD Tier Selection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Camera at 5 m | LodTier::Full | R-9.1.10 |
| 2 | Camera at 50 m | LodTier::Reduced | R-9.1.10 |
| 3 | Camera at 100 m | LodTier::HalfRate | R-9.1.10 |
| 4 | Camera at 200 m | LodTier::Vat | R-9.1.10 |

### TC-9.1.10.2 LOD Hero Bias

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | hero_bias active, camera at 200 m | LodTier::Full (bias overrides distance) | R-9.1.10 |
| 2 | hero_bias inactive, camera at 200 m | LodTier::Vat (normal tier selection) | R-9.1.10 |

## Integration Tests

### TC-9.1.1.I1 GPU Skinning LBS Twist

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Forearm mesh rotated 180 degrees, LBS mode | GPU vertex positions within 0.001 of CPU reference | R-9.1.1 |

### TC-9.1.1.I2 GPU Skinning DQS Twist

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Forearm mesh rotated 180 degrees, DQS mode | Waist cross-section area matches reference (no candy-wrapping) | R-9.1.1 |

### TC-9.1.2.I1 GPU Keyframe vs CPU

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 60-frame clip at t=15.5, GPU vs CPU | Per-joint position error < 0.001 units | R-9.1.2 |

### TC-9.1.5.I1 Instanced 1000

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000 instances sharing 10 clips, 50 random samples | All poses match single-instance evaluation | R-9.1.5 |
| 2 | 1000 instances, verify dispatch | Single GPU dispatch confirmed | R-9.1.5 |

### TC-9.1.5.I2 Instanced Frame Time

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000 instances, measure GPU time | GPU time < 2.0 ms on reference hardware | R-9.1.5 |

### TC-9.1.6.I1 Root Motion Physics

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Dodge roll with root motion + physics capsule | Capsule displacement matches root delta, collision active | R-9.1.6 |

### TC-9.1.8.I1 Retarget Cross Species

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Human walk retargeted to quadruped | No bone explosions, foot contact timing preserved | R-9.1.8 |

### TC-9.1.10.I1 LOD 200 Characters

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 200 characters at 5-500 m distances | Correct tier per distance, no visible popping | R-9.1.10 |

### TC-9.1.1.I3 Full Pipeline Frame

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 50 characters, various LOD tiers, 1 frame | No errors, all buffers written, events dispatched | R-9.1.1 through R-9.1.10 |

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
