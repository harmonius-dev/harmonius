# Save System & Cinematics Test Cases

Companion test cases for [save-cinematics.md](save-cinematics.md).

## Unit Tests

### TC-13.3.1.1 Serialize Full Character

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Entity with 500 items, 50 quests, 200 achievements | Serialize then deserialize; all fields match via `Reflect::reflect_partial_eq` | R-13.3.1 |

### TC-13.3.1.2 Serialize Dirty Only

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Modify 1 field on 1 of 100 entities | Only 1 entity serialized; output < 1% of full size | R-13.3.1 |

### TC-13.3.1.3 Reflect Auto Serialize

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | New `#[derive(Reflect)]` component, no custom code | Component serializes and deserializes correctly | R-13.3.1 |

### TC-13.3.2.1 Migration V1 to V3

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | v1 save, v1->v2 (AddField), v2->v3 (RenameField) | Loaded v3 data has new field and renamed field correct | R-13.3.2 |

### TC-13.3.2.2 Migration Failure Preserves Original

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register failing Custom migration step | `Err(StepFailed)`, original `DynamicValue` unchanged | R-13.3.2 |

### TC-13.3.2.3 Migration No Path

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Request migrate v1 to v5, only v1->v2 registered | `Err(NoPath { from: v1, to: v5 })` | R-13.3.2 |

### TC-13.3.3.1 Checkpoint Trigger

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Emit zone-transition event | Autosave triggered; correct rotating slot used | R-13.3.3 |

### TC-13.3.3.2 Autosave Rotation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | N=3 rotating slots, trigger 4 autosaves | Slot 0 overwritten by 4th save | R-13.3.3 |

### TC-13.3.3.3 Autosave Crash Midwrite

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Interrupt after temp write, before rename | Previous slot file intact, no corrupt final file | R-13.3.3 |

### TC-13.3.4.1 Slot Metadata

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create slot "Save1", save level-20 character | `name == "Save1"`, `level == 20`, timestamp set | R-13.3.4 |

### TC-13.3.4.2 Slot Copy Transactional

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Copy slot, interrupt mid-copy | No partial copy file exists | R-13.3.4 |

### TC-13.3.4.3 Slot Delete

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Delete slot 2 | File removed, metadata cleared, `list_slots()` excludes slot 2 | R-13.3.4 |

### TC-13.3.4.4 Slot Export Import

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Export slot, delete, import back | Imported data identical to original | R-13.3.4 |

### TC-13.3.6.1 Pipeline Compress Encrypt Checksum

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write 1 MB payload, read back | Data matches; corrupt 1 byte -> `ChecksumMismatch` | R-13.3.6 |

### TC-13.3.6.2 Pipeline Atomic Rename

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Kill after temp write, before rename | No corrupt final file; temp file orphaned | R-13.3.6 |

### TC-13.3.6.3 Pipeline Priority Ordering

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Submit Background autosave then Normal explicit save | Normal completes before Background | R-13.3.6 |

### TC-13.3.6.4 Pipeline LZ4 vs Zstd

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compress same 5 MB payload with both | LZ4 faster; Zstd smaller output | R-13.3.6 |

### TC-13.3.6.5 Encryption Wrong Key

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Encrypt with key A, decrypt with key B | `Err(DecryptionFailed)` | R-13.3.6 |

### TC-13.5.1.1 Timeline Deterministic 30/60/120

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Play at 30, 60, 120 fps; sample t=1.0, 2.5, 5.0 | Identical track outputs at all framerates | R-13.5.1 |

### TC-13.5.1.2 Nested Sub Sequence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Parent with child sub-sequence at t=2.0s | Child plays at correct position within parent timeline | R-13.5.1 |

### TC-13.5.1.3 Keyframe Interpolation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3 keyframes at 0.0, 0.5, 1.0; sample at 0.25, 0.75 | Values match Bezier interpolation within 0.001 tolerance | R-13.5.1 |

### TC-13.5.2.1 Camera Mode Fixed

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fixed camera at pos=(1,2,3), rot=(0,0,0,1) | Camera position and rotation match exactly | R-13.5.2 |

### TC-13.5.2.2 Camera Mode Tracking

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Tracking shot, target moves from (0,0,0) to (10,0,0) | Camera follows with configured offset | R-13.5.2 |

### TC-13.5.2.3 Camera Blend Transition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fixed->Tracking with EaseInOut, sample at 50% | Blended position between fixed and tracking | R-13.5.2 |

### TC-13.5.2.4 Camera DOF Override

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set `DepthOfField { focal: 5.0, aperture: 2.8 }` | `DepthOfField` component values match | R-13.5.2 |

### TC-13.5.3.1 Spline Catmull Rom

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 4 control points, evaluate t=0.5 | Position on Catmull-Rom curve within 0.01 tolerance | R-13.5.3 |

### TC-13.5.3.2 Spline Branching

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 2 branches gated on bool condition | Correct branch selected per condition value | R-13.5.3 |

### TC-13.5.4.1 Actor Blend In

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1.0s blend-in, sample at 0.5s | `blend_weight == 0.5` | R-13.5.4 |

### TC-13.5.4.2 Actor Blend Out

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | End cutscene with 1.0s blend-out | Gameplay animation fully restored after blend | R-13.5.4 |

### TC-13.5.4.3 Actor Partial Body

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `UpperBodyOnly` mask override | Lower body weight = 0.0, upper body weight = 1.0 | R-13.5.4 |

### TC-13.5.5.1 Dialogue Cue Timing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cue at t=3.0s, play to t=3.0s | `DialogueEvent` emitted with correct cue data | R-13.5.5 |

### TC-13.5.5.2 Subtitle Localization

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Dialogue cue with key "line_01" | `SubtitleEvent.text_key == "line_01"` | R-13.5.5 |

### TC-13.5.6a.1 Skip Applies All Effects

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Timeline with 5 side effects, skip at t=1.0s | All 5 side effects applied | R-13.5.6a |

### TC-13.5.6a.2 Skip Multiplayer Unanimous

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 4 players, Unanimous policy, 3 vote | `SkipDecision::Pending { votes_for: 3, needed: 4 }` | R-13.5.6a |
| 2 | 4th player votes | `SkipDecision::Approved` | R-13.5.6a |

### TC-13.5.6a.3 Skip Multiplayer Timeout

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | MajorityVote, 3/4 vote, timeout expires | `SkipDecision::Approved` (majority met) | R-13.5.6a |

### TC-13.5.6b.1 Fast Forward 2x

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set speed=2.0, tick 1.0s real time | Clock at 2.0s; all triggers in [0, 2.0] fired | R-13.5.6b |

### TC-13.5.6b.2 Fast Forward 4x Trigger Order

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10 triggers at 1s intervals, play at 4x | All 10 fire in order | R-13.5.6b |

### TC-13.5.6c.1 Pause Freeze

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pause at t=2.5s, wait 100 frames | `position == 2.5` | R-13.5.6c |

### TC-13.5.6c.2 Resume Exact Frame

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pause at t=2.5s, resume, next frame dt=0.016 | `position == 2.516` | R-13.5.6c |

### TC-13.5.7.1 Letterbox Aspect Ratios

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply Anamorphic (2.39:1) on 16:9 display | Bar height = (1 - (16/9) / 2.39) / 2 * screen_height | R-13.5.7 |

### TC-13.5.7.2 Letterbox Hides HUD

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Activate letterbox | `suppress_hud == true`, `suppress_input == true` | R-13.5.7 |

## Integration Tests

### TC-13.3.1.I1 Save Load Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Full save-load cycle: serialize, pipeline write, read, deserialize | World state matches original | R-13.3.1 |

### TC-13.3.NF1.I1 Save No Frame Drop

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger save during 60 fps gameplay | Zero frames exceed 16.67 ms | R-13.3.NF1 |

### TC-13.3.NF1.I2 Save Under 100ms

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Max character (500 items, 50 quests, 200 achievements) | p99 save time < 100 ms | R-13.3.NF1 |

### TC-13.3.NF2.I1 Save File Under 10MB

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Max-progression character, compressed | File size < 10 MB | R-13.3.NF2 |

### TC-13.3.NF3.I1 Crash Safety 10 Points

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Process kill at 10 random pipeline stages | No data loss in any case | R-13.3.NF3 |

### TC-13.3.5.I1 Cloud Sync Upload

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Save locally, upload to cloud | Remote hash matches local hash | R-13.3.5 |

### TC-13.3.5.I2 Cloud Sync Conflict

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Modify both local and remote | `SyncResult::Conflict` returned | R-13.3.5 |

### TC-13.3.5.I3 Cloud Sync No Block

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Upload during gameplay | Game thread never blocks > 1 ms | R-13.3.5 |

### TC-13.5.NF1.I1 Sequencer 32 Tracks

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 32 simultaneous tracks evaluated | Per-frame evaluation < 0.5 ms | R-13.5.NF1 |

### TC-13.5.NF2.I1 Skip Side Effects Under 1 Frame

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 20 side effects, skip | Total application < 16.67 ms | R-13.5.NF2 |

### TC-13.5.1.I1 Cinematic End To End

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Play sequence with camera, animation, audio, dialogue, letterbox tracks | Correct output per track; skip applies all effects | R-13.5.1 |

## Benchmarks

### TC-13.3.NF1.B1 Full Save Max Character

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Max character save | p99 latency | < 100 ms | R-13.3.NF1 |

### TC-13.3.1.B1 Incremental Save 10 Dirty

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10 dirty entities out of 1000 | p99 latency | < 10 ms | R-13.3.1 |

### TC-13.3.6.B1 LZ4 Compress 5MB

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 5 MB payload LZ4 compression | Compression time | < 5 ms | R-13.3.6 |

### TC-13.3.6.B2 AES-256-GCM Encrypt 5MB

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 5 MB payload encryption | Encryption time | < 10 ms | R-13.3.6 |

### TC-13.3.NF2.B1 Save File Size

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Max character compressed | File size | < 10 MB | R-13.3.NF2 |

### TC-13.5.NF1.B1 Sequencer Eval 32 Tracks

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 32 active tracks | Evaluation time | < 0.5 ms | R-13.5.NF1 |

### TC-13.5.NF2.B1 Skip Effect Application

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 20 side effects on skip | Application time | < 16.67 ms | R-13.5.NF2 |

### TC-13.5.1.B1 Keyframe Interpolation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1000 keyframe interpolations | Total time | < 0.1 ms | R-13.5.1 |
