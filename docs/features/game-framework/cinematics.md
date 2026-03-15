# 13.5 — Cinematics

## Sequencer

### F-13.5.1 Sequencer and Timeline

A multi-track timeline system for authoring and playing back cinematics. Each track controls a
specific aspect: camera, actor animation, audio, VFX, lighting overrides, gameplay triggers, and UI
overlays. Tracks contain keyframed clips with configurable easing curves. The sequencer evaluates
all active tracks each frame and produces deterministic output regardless of framerate. Supports
nested sub-sequences for reusable cinematic building blocks.

- **Requirements:** R-13.5.1
- **Dependencies:** F-1.5.1 (Typed Event Channels), F-9.4.1 (Animation State Graph)
- **Platform notes:** None

### F-13.5.2 Cutscene Camera System

Provides cinematic camera modes that override the gameplay camera: fixed shot, tracking shot (follow
target with offset), orbit, dolly zoom, and handheld shake. Cameras blend between modes using
configurable transition curves. Supports depth-of-field overrides, focal target tracking, and aspect
ratio changes (letterboxing) per shot. Camera cuts and transitions are driven by the sequencer
timeline.

- **Requirements:** R-13.5.2
- **Dependencies:** F-13.5.1
- **Platform notes:** None

## Camera Motion

### F-13.5.3 Camera Rails and Splines

Defines camera paths as Catmull-Rom or Bezier splines placed in the world. The camera follows the
spline with configurable speed, acceleration, and look-at targets. Spline segments can be triggered
by sequencer events, player proximity, or script commands. Supports branching paths where the active
branch is selected by gameplay conditions (e.g., different camera angles depending on which boss
phase is active).

- **Requirements:** R-13.5.3
- **Dependencies:** F-13.5.2
- **Platform notes:** None

## Actor Integration

### F-13.5.4 Actor Animation Blending in Cinematics

Blends NPC and player character animations from their gameplay state machine into cinematic-driven
poses when a cutscene begins, and back again when it ends. Blend-in and blend-out durations are
per-actor and per-track. Supports partial body overrides so a character's lower body can continue
locomotion while the upper body performs a cinematic gesture. Handles the case where hundreds of
background NPCs continue ambient behavior while foreground actors are cinematically controlled.

- **Requirements:** R-13.5.4
- **Dependencies:** F-13.5.1, F-9.4.4 (Animation Layers), F-9.4.7 (Montages)
- **Platform notes:** None

### F-13.5.5 Dialogue Triggers and Subtitles

Fires dialogue events at timeline-specified cue points, triggering voice-over playback, subtitle
display, and lip-sync animation. Subtitles support localized text, speaker identification, and
configurable display duration. Dialogue events integrate with the dialogue tree system so cinematic
conversations can branch based on player choices or quest state.

- **Requirements:** R-13.5.5
- **Dependencies:** F-13.5.1, F-13.6.4 (Dialogue Trees), F-5.5.1 (Voice)
- **Platform notes:** None

## Player Control

### F-13.5.6a Cutscene Skip System

Allows players to skip cutscenes entirely, jumping to an end-state that applies all gameplay side
effects (quest updates, item grants, phase transitions) without desynchronizing server state. In
multiplayer instanced content, skip requires consensus from all group members or uses a
majority-vote timeout.

- **Requirements:** R-13.5.6a
- **Dependencies:** F-13.5.1, F-13.1.2 (Game State Manager)
- **Platform notes:** None

### F-13.5.6b Cutscene Playback Speed

Supports fast-forward playback at 2x and 4x speed. Fast-forward accelerates all tracks (camera,
animation, audio, VFX) uniformly. Audio pitch-shifts or mutes during fast-forward as configured per
sequence. All gameplay triggers fire at their correct timeline positions regardless of playback
speed.

- **Requirements:** R-13.5.6b
- **Dependencies:** F-13.5.1
- **Platform notes:** None

### F-13.5.6c Cutscene Pause

Allows players to pause cutscene playback, freezing all tracks at the current frame. Pausing
displays a configurable overlay (dim, blur, or pause icon). Unpausing resumes from the exact frame.
In multiplayer, pause behavior is configurable: disabled, host-only, or any-player.

- **Requirements:** R-13.5.6c
- **Dependencies:** F-13.5.1, F-13.1.2 (Game State Manager)
- **Platform notes:** None

### F-13.5.7 Letterboxing and Cinematic Framing

Renders configurable letterbox bars (aspect ratios: 2.39:1, 2.00:1, 1.85:1) with animated
reveal/hide transitions to signal cinematic mode. Letterboxing coordinates with the UI system to
hide gameplay HUD elements and with the input system to suppress gameplay inputs during cutscenes.
Supports custom vignette and film grain overlays per sequence for stylistic framing.

- **Requirements:** R-13.5.7
- **Dependencies:** F-13.5.1, F-10.1.1 (UI Widget Framework), F-2.4.1 (Post-Processing)
- **Platform notes:** None
