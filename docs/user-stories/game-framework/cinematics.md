# User Stories — 13.5 Cinematics

## F-13.5.1 Sequencer and Timeline

## US-13.5.1.1 Author a Multi-Track Cinematic Timeline

**As a** story director (P-4), **I want to** place keyframed clips across camera, animation, audio,
VFX, lighting, and gameplay trigger tracks, **so that** I can choreograph a complete cinematic
sequence visually.

## US-13.5.1.2 Nest Sub-Sequences for Reuse

**As a** story director (P-4), **I want to** embed reusable sub-sequences inside a parent sequence,
**so that** common cinematic building blocks (logo reveal, transition stinger) are authored once and
reused across cutscenes.

## US-13.5.1.3 Watch Cinematics With Deterministic Playback

**As a** player (P-23), **I want to** cutscenes to play back identically regardless of frame rate,
**so that** the intended pacing and timing are preserved on all hardware.

## US-13.5.1.4 Verify Sequencer Produces Deterministic Output

**As an** engine tester (P-27), **I want to** play a sequence at 30 and 120 FPS and verify identical
track evaluations at matching timeline positions, **so that** playback is fully frame-rate
independent.

## F-13.5.2 Cutscene Camera System

## US-13.5.2.1 Configure Cinematic Camera Modes Per Shot

**As a** story director (P-4), **I want to** assign camera modes (fixed, tracking, orbit, dolly
zoom, handheld shake) per shot with configurable transitions, **so that** each shot has the intended
framing and motion.

## US-13.5.2.2 Override Depth-of-Field Per Shot

**As a** story director (P-4), **I want to** set depth-of-field overrides and focal target tracking
per camera shot, **so that** cinematic focus draws the player's eye to the intended subject.

## US-13.5.2.3 Experience Smooth Camera Transitions

**As a** player (P-23), **I want to** camera transitions between shots to blend smoothly,
**so that** cuts and transitions feel cinematic rather than jarring.

## US-13.5.2.4 Verify Camera Blends Use Configured Transition Curves

**As an** engine tester (P-27), **I want to** verify that camera transitions use the exact easing
curve defined per shot, **so that** blend behavior matches the authored intent.

## F-13.5.3 Camera Rails and Splines

## US-13.5.3.1 Place Camera Spline Paths in the World

**As a** story director (P-4), **I want to** place Catmull-Rom or Bezier camera splines in the world
with configurable speed, acceleration, and look-at targets, **so that** cameras follow smooth
authored paths.

## US-13.5.3.2 Branch Spline Paths by Gameplay Condition

**As a** story director (P-4), **I want to** define branching camera paths where the active branch
is selected by gameplay conditions (boss phase, player choice), **so that** camera angles adapt to
the current game state.

## US-13.5.3.3 Experience Dynamic Camera Angles During Boss Fights

**As a** player (P-23), **I want to** the camera to select different cinematic angles based on the
active boss phase, **so that** dramatic moments are framed appropriately.

## US-13.5.3.4 Verify Spline Triggers Fire at Correct Positions

**As an** engine tester (P-27), **I want to** verify that spline segment triggers fire at their
authored world positions regardless of camera speed, **so that** trigger placement is reliable.

## F-13.5.4 Actor Animation Blending in Cinematics

## US-13.5.4.1 Blend Actors From Gameplay to Cinematic Poses

**As a** story director (P-4), **I want to** blend NPC and player character animations from gameplay
state into cinematic-driven poses with configurable blend durations, **so that** cutscene
transitions appear natural.

## US-13.5.4.2 Apply Partial Body Overrides During Cutscenes

**As a** story director (P-4), **I want to** override only the upper body for cinematic gestures
while the lower body continues locomotion, **so that** characters perform cinematic actions while
walking.

## US-13.5.4.3 Observe Background NPCs During Cutscenes

**As a** player (P-23), **I want to** background NPCs to continue ambient behavior while foreground
actors perform cinematic actions, **so that** the world feels alive during cutscenes.

## US-13.5.4.4 Verify Blend-Out Restores Gameplay Animation State

**As an** engine tester (P-27), **I want to** verify that after a cutscene ends, all actors return
to their pre-cutscene animation state machine state, **so that** cinematic blending leaves no
residual animation artifacts.

## F-13.5.5 Dialogue Triggers and Subtitles

## US-13.5.5.1 Place Dialogue Cue Points on the Timeline

**As a** story director (P-4), **I want to** place dialogue events at timeline cue points that
trigger voice-over playback, subtitle display, and lip-sync animation, **so that** spoken dialogue
is synchronized with the cinematic.

## US-13.5.5.2 Branch Dialogue Based on Player Choice

**As a** story director (P-4), **I want to** cinematic conversations to branch based on player
choices or quest state through the dialogue tree system, **so that** cutscenes support interactive
narratives.

## US-13.5.5.3 Read Localized Subtitles During Cutscenes

**As a** player (P-23), **I want to** see localized subtitles with speaker identification during
cinematics, **so that** I can follow dialogue in my preferred language.

## US-13.5.5.4 Verify Subtitle Timing Matches Audio Cues

**As an** engine tester (P-27), **I want to** verify that subtitle display and hide events align
with voice-over start and end times within 50ms, **so that** subtitles are synchronized with audio.

## F-13.5.6a Cutscene Skip System

## US-13.5.6a.1 Skip Cutscenes While Preserving Side Effects

**As a** player (P-23), **I want to** skip a cutscene and have all gameplay side effects (quest
updates, item grants, phase transitions) applied immediately, **so that** skipping does not
desynchronize my game state.

## US-13.5.6a.2 Configure Group Skip Consensus Rules

**As a** story director (P-4), **I want to** configure multiplayer skip behavior (unanimous,
majority-vote, host-only), **so that** group cutscene skipping follows the intended social contract.

## US-13.5.6a.3 Verify Skip Applies All Gameplay Triggers

**As an** engine tester (P-27), **I want to** skip a cutscene with multiple gameplay triggers and
verify all triggers fire, **so that** skipping produces the same end-state as watching.

## F-13.5.6b Cutscene Playback Speed

## US-13.5.6b.1 Fast-Forward Cutscenes at 2x and 4x Speed

**As a** player (P-23), **I want to** fast-forward cutscenes at 2x and 4x speed, **so that** I can
view the content quickly on repeated playthroughs.

## US-13.5.6b.2 Verify Gameplay Triggers Fire at Correct Positions During Fast-Forward

**As an** engine tester (P-27), **I want to** fast-forward a cutscene and verify all gameplay
triggers fire at their correct timeline positions, **so that** fast-forward does not skip triggers.

## F-13.5.6c Cutscene Pause

## US-13.5.6c.1 Pause and Resume Cutscene Playback

**As a** player (P-23), **I want to** pause cutscene playback and resume from the exact frame,
**so that** I can step away without missing content.

## US-13.5.6c.2 Verify Pause Freezes All Tracks at Current Frame

**As an** engine tester (P-27), **I want to** pause a cutscene and verify all tracks (camera,
animation, audio, VFX) freeze at the current frame with no drift, **so that** pause state is exact.

## F-13.5.7 Letterboxing and Cinematic Framing

## US-13.5.7.1 Configure Letterbox Aspect Ratios Per Sequence

**As a** story director (P-4), **I want to** apply configurable letterbox bars (2.39:1, 2.00:1,
1.85:1) with animated reveal/hide transitions per sequence, **so that** cinematic framing signals a
change in tone.

## US-13.5.7.2 Experience Cinematic Framing During Cutscenes

**As a** player (P-23), **I want to** letterbox bars, vignettes, and film grain to appear during
cinematics, **so that** cutscenes feel visually distinct from gameplay.

## US-13.5.7.3 Verify Letterboxing Hides Gameplay HUD

**As an** engine tester (P-27), **I want to** verify that letterbox activation hides all gameplay
HUD elements and suppresses gameplay inputs, **so that** cinematic mode prevents accidental gameplay
interactions.
