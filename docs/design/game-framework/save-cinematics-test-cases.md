# Save System & Cinematics Test Cases

Companion test cases for [save-cinematics.md](save-cinematics.md).

## Unit Tests

### TC-13.3.1.1 Serialize Full Character

| # | Requirement |
|---|-------------|
| 1 | R-13.3.1    |

1. **#1** — Entity with 500 items, 50 quests, 200 achievements
   - **Expected:** Serialize then deserialize; all fields match via `Reflect::reflect_partial_eq`

### TC-13.3.1.2 Serialize Dirty Only

| # | Requirement |
|---|-------------|
| 1 | R-13.3.1    |

1. **#1** — Modify 1 field on 1 of 100 entities
   - **Expected:** Only 1 entity serialized; output < 1% of full size

### TC-13.3.1.3 Reflect Auto Serialize

| # | Requirement |
|---|-------------|
| 1 | R-13.3.1    |

1. **#1** — New `#[derive(Reflect)]` component, no custom code
   - **Expected:** Component serializes and deserializes correctly

### TC-13.3.2.1 Migration V1 to V3

| # | Requirement |
|---|-------------|
| 1 | R-13.3.2    |

1. **#1** — v1 save, v1->v2 (AddField), v2->v3 (RenameField)
   - **Expected:** Loaded v3 data has new field and renamed field correct

### TC-13.3.2.2 Migration Failure Preserves Original

| # | Requirement |
|---|-------------|
| 1 | R-13.3.2    |

1. **#1** — Register failing Custom migration step
   - **Expected:** `Err(StepFailed)`, original `DynamicValue` unchanged

### TC-13.3.2.3 Migration No Path

| # | Requirement |
|---|-------------|
| 1 | R-13.3.2    |

1. **#1** — Request migrate v1 to v5, only v1->v2 registered
   - **Expected:** `Err(NoPath { from: v1, to: v5 })`

### TC-13.3.3.1 Checkpoint Trigger

| # | Requirement |
|---|-------------|
| 1 | R-13.3.3    |

1. **#1** — Emit zone-transition event
   - **Expected:** Autosave triggered; correct rotating slot used

### TC-13.3.3.2 Autosave Rotation

| # | Requirement |
|---|-------------|
| 1 | R-13.3.3    |

1. **#1** — N=3 rotating slots, trigger 4 autosaves
   - **Expected:** Slot 0 overwritten by 4th save

### TC-13.3.3.3 Autosave Crash Midwrite

| # | Requirement |
|---|-------------|
| 1 | R-13.3.3    |

1. **#1** — Interrupt after temp write, before rename
   - **Expected:** Previous slot file intact, no corrupt final file

### TC-13.3.4.1 Slot Metadata

| # | Requirement |
|---|-------------|
| 1 | R-13.3.4    |

1. **#1** — Create slot "Save1", save level-20 character
   - **Expected:** `name == "Save1"`, `level == 20`, timestamp set

### TC-13.3.4.2 Slot Copy Transactional

| # | Requirement |
|---|-------------|
| 1 | R-13.3.4    |

1. **#1** — Copy slot, interrupt mid-copy
   - **Expected:** No partial copy file exists

### TC-13.3.4.3 Slot Delete

| # | Requirement |
|---|-------------|
| 1 | R-13.3.4    |

1. **#1** — Delete slot 2
   - **Expected:** File removed, metadata cleared, `list_slots()` excludes slot 2

### TC-13.3.4.4 Slot Export Import

| # | Requirement |
|---|-------------|
| 1 | R-13.3.4    |

1. **#1** — Export slot, delete, import back
   - **Expected:** Imported data identical to original

### TC-13.3.6.1 Pipeline Compress Encrypt Checksum

| # | Requirement |
|---|-------------|
| 1 | R-13.3.6    |

1. **#1** — Write 1 MB payload, read back
   - **Expected:** Data matches; corrupt 1 byte -> `ChecksumMismatch`

### TC-13.3.6.2 Pipeline Atomic Rename

| # | Requirement |
|---|-------------|
| 1 | R-13.3.6    |

1. **#1** — Kill after temp write, before rename
   - **Expected:** No corrupt final file; temp file orphaned

### TC-13.3.6.3 Pipeline Priority Ordering

| # | Requirement |
|---|-------------|
| 1 | R-13.3.6    |

1. **#1** — Submit Background autosave then Normal explicit save
   - **Expected:** Normal completes before Background

### TC-13.3.6.4 Pipeline LZ4 vs Zstd

| # | Requirement |
|---|-------------|
| 1 | R-13.3.6    |

1. **#1** — Compress same 5 MB payload with both
   - **Expected:** LZ4 faster; Zstd smaller output

### TC-13.3.6.5 Encryption Wrong Key

| # | Requirement |
|---|-------------|
| 1 | R-13.3.6    |

1. **#1** — Encrypt with key A, decrypt with key B
   - **Expected:** `Err(DecryptionFailed)`

### TC-13.5.1.1 Timeline Deterministic 30/60/120

| # | Requirement |
|---|-------------|
| 1 | R-13.5.1    |

1. **#1** — Play at 30, 60, 120 fps; sample t=1.0, 2.5, 5.0
   - **Expected:** Identical track outputs at all framerates

### TC-13.5.1.2 Nested Sub Sequence

| # | Requirement |
|---|-------------|
| 1 | R-13.5.1    |

1. **#1** — Parent with child sub-sequence at t=2.0s
   - **Expected:** Child plays at correct position within parent timeline

### TC-13.5.1.3 Keyframe Interpolation

| # | Requirement |
|---|-------------|
| 1 | R-13.5.1    |

1. **#1** — 3 keyframes at 0.0, 0.5, 1.0; sample at 0.25, 0.75
   - **Expected:** Values match Bezier interpolation within 0.001 tolerance

### TC-13.5.2.1 Camera Mode Fixed

| # | Requirement |
|---|-------------|
| 1 | R-13.5.2    |

1. **#1** — Fixed camera at pos=(1,2,3), rot=(0,0,0,1)
   - **Expected:** Camera position and rotation match exactly

### TC-13.5.2.2 Camera Mode Tracking

| # | Requirement |
|---|-------------|
| 1 | R-13.5.2    |

1. **#1** — Tracking shot, target moves from (0,0,0) to (10,0,0)
   - **Expected:** Camera follows with configured offset

### TC-13.5.2.3 Camera Blend Transition

| # | Requirement |
|---|-------------|
| 1 | R-13.5.2    |

1. **#1** — Fixed->Tracking with EaseInOut, sample at 50%
   - **Expected:** Blended position between fixed and tracking

### TC-13.5.2.4 Camera DOF Override

| # | Requirement |
|---|-------------|
| 1 | R-13.5.2    |

1. **#1** — Set `DepthOfField { focal: 5.0, aperture: 2.8 }`
   - **Expected:** `DepthOfField` component values match

### TC-13.5.3.1 Spline Catmull Rom

| # | Requirement |
|---|-------------|
| 1 | R-13.5.3    |

1. **#1** — 4 control points, evaluate t=0.5
   - **Expected:** Position on Catmull-Rom curve within 0.01 tolerance

### TC-13.5.3.2 Spline Branching

| # | Requirement |
|---|-------------|
| 1 | R-13.5.3    |

1. **#1** — 2 branches gated on bool condition
   - **Expected:** Correct branch selected per condition value

### TC-13.5.4.1 Actor Blend In

| # | Requirement |
|---|-------------|
| 1 | R-13.5.4    |

1. **#1** — 1.0s blend-in, sample at 0.5s
   - **Expected:** `blend_weight == 0.5`

### TC-13.5.4.2 Actor Blend Out

| # | Requirement |
|---|-------------|
| 1 | R-13.5.4    |

1. **#1** — End cutscene with 1.0s blend-out
   - **Expected:** Gameplay animation fully restored after blend

### TC-13.5.4.3 Actor Partial Body

| # | Requirement |
|---|-------------|
| 1 | R-13.5.4    |

1. **#1** — `UpperBodyOnly` mask override
   - **Expected:** Lower body weight = 0.0, upper body weight = 1.0

### TC-13.5.5.1 Dialogue Cue Timing

| # | Requirement |
|---|-------------|
| 1 | R-13.5.5    |

1. **#1** — Cue at t=3.0s, play to t=3.0s
   - **Expected:** `DialogueEvent` emitted with correct cue data

### TC-13.5.5.2 Subtitle Localization

| # | Requirement |
|---|-------------|
| 1 | R-13.5.5    |

1. **#1** — Dialogue cue with key "line_01"
   - **Expected:** `SubtitleEvent.text_key == "line_01"`

### TC-13.5.6a.1 Skip Applies All Effects

| # | Requirement |
|---|-------------|
| 1 | R-13.5.6a   |

1. **#1** — Timeline with 5 side effects, skip at t=1.0s
   - **Expected:** All 5 side effects applied

### TC-13.5.6a.2 Skip Multiplayer Unanimous

| # | Requirement |
|---|-------------|
| 1 | R-13.5.6a   |
| 2 | R-13.5.6a   |

1. **#1** — 4 players, Unanimous policy, 3 vote
   - **Expected:** `SkipDecision::Pending { votes_for: 3, needed: 4 }`
2. **#2** — 4th player votes
   - **Expected:** `SkipDecision::Approved`

### TC-13.5.6a.3 Skip Multiplayer Timeout

| # | Requirement |
|---|-------------|
| 1 | R-13.5.6a   |

1. **#1** — MajorityVote, 3/4 vote, timeout expires
   - **Expected:** `SkipDecision::Approved` (majority met)

### TC-13.5.6b.1 Fast Forward 2x

| # | Requirement |
|---|-------------|
| 1 | R-13.5.6b   |

1. **#1** — Set speed=2.0, tick 1.0s real time
   - **Expected:** Clock at 2.0s; all triggers in [0, 2.0] fired

### TC-13.5.6b.2 Fast Forward 4x Trigger Order

| # | Requirement |
|---|-------------|
| 1 | R-13.5.6b   |

1. **#1** — 10 triggers at 1s intervals, play at 4x
   - **Expected:** All 10 fire in order

### TC-13.5.6c.1 Pause Freeze

| # | Requirement |
|---|-------------|
| 1 | R-13.5.6c   |

1. **#1** — Pause at t=2.5s, wait 100 frames
   - **Expected:** `position == 2.5`

### TC-13.5.6c.2 Resume Exact Frame

| # | Requirement |
|---|-------------|
| 1 | R-13.5.6c   |

1. **#1** — Pause at t=2.5s, resume, next frame dt=0.016
   - **Expected:** `position == 2.516`

### TC-13.5.7.1 Letterbox Aspect Ratios

| # | Requirement |
|---|-------------|
| 1 | R-13.5.7    |

1. **#1** — Apply Anamorphic (2.39:1) on 16:9 display
   - **Expected:** Bar height = (1 - (16/9) / 2.39) / 2 * screen_height

### TC-13.5.7.2 Letterbox Hides HUD

| # | Requirement |
|---|-------------|
| 1 | R-13.5.7    |

1. **#1** — Activate letterbox
   - **Expected:** `suppress_hud == true`, `suppress_input == true`

## Integration Tests

### TC-13.3.1.I1 Save Load Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-13.3.1    |

1. **#1** — Full save-load cycle: serialize, pipeline write, read, deserialize
   - **Expected:** World state matches original

### TC-13.3.NF1.I1 Save No Frame Drop

| # | Requirement |
|---|-------------|
| 1 | R-13.3.NF1  |

1. **#1** — Trigger save during 60 fps gameplay
   - **Expected:** Zero frames exceed 16.67 ms

### TC-13.3.NF1.I2 Save Under 100ms

| # | Requirement |
|---|-------------|
| 1 | R-13.3.NF1  |

1. **#1** — Max character (500 items, 50 quests, 200 achievements)
   - **Expected:** p99 save time < 100 ms

### TC-13.3.NF2.I1 Save File Under 10MB

| # | Requirement |
|---|-------------|
| 1 | R-13.3.NF2  |

1. **#1** — Max-progression character, compressed
   - **Expected:** File size < 10 MB

### TC-13.3.NF3.I1 Crash Safety 10 Points

| # | Requirement |
|---|-------------|
| 1 | R-13.3.NF3  |

1. **#1** — Process kill at 10 random pipeline stages
   - **Expected:** No data loss in any case

### TC-13.3.5.I1 Cloud Sync Upload

| # | Requirement |
|---|-------------|
| 1 | R-13.3.5    |

1. **#1** — Save locally, upload to cloud
   - **Expected:** Remote hash matches local hash

### TC-13.3.5.I2 Cloud Sync Conflict

| # | Requirement |
|---|-------------|
| 1 | R-13.3.5    |

1. **#1** — Modify both local and remote
   - **Expected:** `SyncResult::Conflict` returned

### TC-13.3.5.I3 Cloud Sync No Block

| # | Requirement |
|---|-------------|
| 1 | R-13.3.5    |

1. **#1** — Upload during gameplay
   - **Expected:** Game thread never blocks > 1 ms

### TC-13.5.NF1.I1 Cinematics Editor 32 Tracks

| # | Requirement |
|---|-------------|
| 1 | R-13.5.NF1  |

1. **#1** — 32 simultaneous tracks evaluated
   - **Expected:** Per-frame evaluation < 0.5 ms

### TC-13.5.NF2.I1 Skip Side Effects Under 1 Frame

| # | Requirement |
|---|-------------|
| 1 | R-13.5.NF2  |

1. **#1** — 20 side effects, skip
   - **Expected:** Total application < 16.67 ms

### TC-13.5.1.I1 Cinematic End To End

| # | Requirement |
|---|-------------|
| 1 | R-13.5.1    |

1. **#1** — Play sequence with camera, animation, audio, dialogue, letterbox tracks
   - **Expected:** Correct output per track; skip applies all effects

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

### TC-13.5.NF1.B1 Cinematics Editor Eval 32 Tracks

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
