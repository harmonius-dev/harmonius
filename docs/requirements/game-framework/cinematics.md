# R-13.5 — Cinematics Requirements

## Sequencer

### R-13.5.1 Sequencer and Timeline

The engine **SHALL** provide a multi-track timeline system for authoring and playing back
cinematics, with tracks controlling camera, actor animation, audio, VFX, lighting overrides,
gameplay triggers, and UI overlays via keyframed clips with configurable easing curves, producing
deterministic output regardless of framerate, and supporting nested sub-sequences for reusable
cinematic building blocks.

- **Derived from:** [F-13.5.1](../../features/game-framework/cinematics.md)
- **Rationale:** A deterministic multi-track sequencer enables designers to author complex
  cinematics visually without code, with reusable sub-sequences reducing duplication.
- **Verification:** Integration test: create a sequence with camera, animation, audio, and VFX
  tracks. Play it back at 30 fps and 60 fps and verify identical output at matching timestamps.
  Embed a nested sub-sequence and verify it plays as part of the parent sequence. Verify all track
  types produce correct results at keyframed cue points.

### R-13.5.2 Cutscene Camera System

The engine **SHALL** provide cinematic camera modes (fixed shot, tracking shot, orbit, dolly zoom,
handheld shake) that override the gameplay camera, blending between modes using configurable
transition curves, with per-shot depth-of-field overrides, focal target tracking, and aspect ratio
changes (letterboxing), all driven by the sequencer timeline.

- **Derived from:** [F-13.5.2](../../features/game-framework/cinematics.md)
- **Rationale:** Cinematic camera modes with smooth blending produce film-quality shots that enhance
  narrative delivery beyond what gameplay cameras provide.
- **Verification:** Integration test: configure a sequence with fixed, tracking, and orbit camera
  modes. Verify each mode produces correct camera behavior. Verify blend transitions between modes
  use the configured curve. Verify depth-of-field overrides and letterboxing apply per-shot as
  specified.

## Camera Motion

### R-13.5.3 Camera Rails and Splines

The engine **SHALL** support camera paths defined as Catmull-Rom or Bezier splines with configurable
speed, acceleration, and look-at targets, triggered by sequencer events, player proximity, or visual
logic graph commands, with branching paths where the active branch is selected by gameplay
conditions.

- **Derived from:** [F-13.5.3](../../features/game-framework/cinematics.md)
- **Rationale:** Spline-based camera paths give designers precise control over cinematic camera
  motion with dynamic branching for context-sensitive framing.
- **Verification:** Integration test: define a Catmull-Rom spline path with a look-at target.
  Trigger it from the sequencer and verify the camera follows the spline at the configured speed.
  Define a branching path with two branches gated on a gameplay condition, toggle the condition, and
  verify the correct branch is selected.

## Actor Integration

### R-13.5.4 Actor Animation Blending in Cinematics

The engine **SHALL** blend NPC and player character animations from gameplay state machines into
cinematic-driven poses at cutscene start and back again at cutscene end, with per-actor and
per-track blend durations, partial body overrides (e.g., upper body cinematic while lower body
continues locomotion), and support for background NPCs continuing ambient behavior while foreground
actors are cinematically controlled.

- **Derived from:** [F-13.5.4](../../features/game-framework/cinematics.md)
- **Rationale:** Seamless animation blending between gameplay and cinematic states prevents visual
  discontinuities at cutscene boundaries and maintains world immersion.
- **Verification:** Integration test: start a cutscene on an actor in locomotion and verify the
  blend-in transition is smooth over the configured duration. Apply a partial upper-body override
  and verify the lower body continues locomotion. End the cutscene and verify blend-out returns the
  actor to gameplay animation. Verify background NPCs continue ambient animations throughout.

### R-13.5.5 Dialogue Triggers and Subtitles

The engine **SHALL** fire dialogue events at timeline-specified cue points, triggering voice-over
playback, localized subtitle display with speaker identification and configurable display duration,
and lip-sync animation, with integration into the dialogue tree system for branching conversations
based on player choices or quest state.

- **Derived from:** [F-13.5.5](../../features/game-framework/cinematics.md)
- **Rationale:** Integrated dialogue triggers ensure voice, subtitles, and lip-sync are
  frame-accurately synchronized, and dialogue tree integration enables narrative branching.
- **Verification:** Integration test: place a dialogue cue in a sequence and verify voice-over,
  subtitles, and lip-sync trigger simultaneously at the cue point. Verify subtitles display the
  correct localized text with speaker name. Define a branching dialogue point and verify the branch
  selected by player choice produces the correct continuation.

## Player Control

### R-13.5.6a Cutscene Skip System

The engine **SHALL** allow players to skip cutscenes, jumping to an end-state that applies all
gameplay side effects (quest updates, item grants, phase transitions) without desynchronizing server
state. Multiplayer skip **SHALL** require group consensus or majority-vote timeout.

- **Derived from:** [F-13.5.6a](../../features/game-framework/cinematics.md)
- **Rationale:** Skip respects player time while ensuring all gameplay-critical side effects apply
  correctly, preventing state desynchronization.
- **Verification:** Play a cutscene that grants a quest reward and triggers a phase transition. Skip
  it and verify the reward is granted and the phase transition applies. In multiplayer, verify skip
  requires consensus from all group members.

### R-13.5.6b Cutscene Playback Speed

The engine **SHALL** support fast-forward cutscene playback at 2x and 4x speed, accelerating all
tracks uniformly while firing all gameplay triggers at their correct timeline positions.

- **Derived from:** [F-13.5.6b](../../features/game-framework/cinematics.md)
- **Rationale:** Fast-forward lets players experience cutscene content at reduced time without
  missing any gameplay triggers or story beats.
- **Verification:** Fast-forward at 4x and verify playback speed increases without skipping events.
  Verify all gameplay triggers fire at correct positions.

### R-13.5.6c Cutscene Pause

The engine **SHALL** allow players to pause cutscene playback, freezing all tracks at the current
frame with a configurable overlay, and resuming from the exact frame on unpause.

- **Derived from:** [F-13.5.6c](../../features/game-framework/cinematics.md)
- **Rationale:** Pause lets players attend to real-world interruptions without losing their place in
  narrative content.
- **Verification:** Pause a cutscene mid-playback and verify all tracks freeze. Unpause and verify
  playback resumes from the exact frame. Verify the configured overlay displays during pause.

### R-13.5.7 Letterboxing and Cinematic Framing

The engine **SHALL** render configurable letterbox bars (aspect ratios: 2.39:1, 2.00:1, 1.85:1) with
animated reveal/hide transitions, coordinating with the UI system to hide gameplay HUD elements and
with the input system to suppress gameplay inputs during cutscenes, with support for custom vignette
and film grain overlays per sequence.

- **Derived from:** [F-13.5.7](../../features/game-framework/cinematics.md)
- **Rationale:** Letterboxing and HUD suppression create a distinct cinematic presentation that
  signals a shift from gameplay to narrative, enhancing immersion.
- **Verification:** Integration test: start a cutscene with 2.39:1 letterboxing and verify bars
  animate in, gameplay HUD hides, and gameplay inputs are suppressed. End the cutscene and verify
  bars animate out, HUD restores, and inputs re-enable. Verify vignette and film grain overlays
  apply when configured.

## Non-Functional Requirements

### R-13.5.NF1 Sequencer Playback Overhead

The engine **SHALL** evaluate all active sequencer tracks with less than 0.5 ms of CPU time per
frame for a sequence with up to 32 simultaneous tracks, ensuring cinematic playback does not reduce
frame rate below the target.

- **Derived from:** F-13.5.1
- **Rationale:** Cinematic playback occurs during gameplay; excessive evaluation overhead would
  cause frame drops that undermine the visual quality of the cutscene itself.
- **Verification:** Play a sequence with 32 tracks (camera, 8 actor animations, 8 audio, 8 VFX, 4
  lighting, 2 gameplay triggers, 2 UI overlays). Measure per-frame evaluation time and verify it
  stays under 0.5 ms.

### R-13.5.NF2 Skip Side-Effect Application Time

The engine **SHALL** apply all gameplay side effects (quest updates, item grants, phase transitions)
from a skipped cutscene within 1 frame (16.67 ms at 60 fps), so that skipping feels instantaneous to
the player.

- **Derived from:** F-13.5.6
- **Rationale:** Slow skip processing forces players to wait after pressing skip, defeating the
  purpose of the skip feature and frustrating repeat viewers.
- **Verification:** Create a cutscene with 20 gameplay side-effect triggers (quest updates, item
  grants, phase transitions). Skip the cutscene and measure time to apply all effects. Verify total
  application time is under 16.67 ms.
