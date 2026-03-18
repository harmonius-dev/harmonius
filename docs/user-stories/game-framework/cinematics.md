# User Stories — 13.5 Cinematics

## F-13.5.1 Sequencer and Timeline

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-13.5.1.1 | story director (P-4) | F-13.5.1 | R-13.5.1     |
| US-13.5.1.2 | story director (P-4) | F-13.5.1 | R-13.5.1     |
| US-13.5.1.3 | player (P-23)        | F-13.5.1 | R-13.5.1     |
| US-13.5.1.4 | engine tester (P-27) | F-13.5.1 | R-13.5.1     |

1. **US-13.5.1.1** — I want to place keyframed clips across camera, animation, audio, VFX, lighting,
   and gameplay trigger tracks so that I can choreograph a complete cinematic sequence visually
2. **US-13.5.1.2** — I want to embed reusable sub-sequences inside a parent sequence so that common
   cinematic building blocks (logo reveal, transition stinger) are authored once and reused across
   cutscenes
3. **US-13.5.1.3** — I want to cutscenes to play back identically regardless of frame rate so that
   the intended pacing and timing are preserved on all hardware
4. **US-13.5.1.4** — I want to play a sequence at 30 and 120 FPS and verify identical track
   evaluations at matching timeline positions so that playback is fully frame-rate independent

## F-13.5.2 Cutscene Camera System

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-13.5.2.1 | story director (P-4) | F-13.5.2 | R-13.5.2     |
| US-13.5.2.2 | story director (P-4) | F-13.5.2 | R-13.5.2     |
| US-13.5.2.3 | player (P-23)        | F-13.5.2 | R-13.5.2     |
| US-13.5.2.4 | engine tester (P-27) | F-13.5.2 | R-13.5.2     |

1. **US-13.5.2.1** — I want to assign camera modes (fixed, tracking, orbit, dolly zoom, handheld
   shake) per shot with configurable transitions so that each shot has the intended framing and
   motion
2. **US-13.5.2.2** — I want to set depth-of-field overrides and focal target tracking per camera
   shot so that cinematic focus draws the player's eye to the intended subject
3. **US-13.5.2.3** — I want to camera transitions between shots to blend smoothly so that cuts and
   transitions feel cinematic rather than jarring
4. **US-13.5.2.4** — I want to verify that camera transitions use the exact easing curve defined per
   shot so that blend behavior matches the authored intent

## F-13.5.3 Camera Rails and Splines

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-13.5.3.1 | story director (P-4) | F-13.5.3 | R-13.5.3     |
| US-13.5.3.2 | story director (P-4) | F-13.5.3 | R-13.5.3     |
| US-13.5.3.3 | player (P-23)        | F-13.5.3 | R-13.5.3     |
| US-13.5.3.4 | engine tester (P-27) | F-13.5.3 | R-13.5.3     |

1. **US-13.5.3.1** — I want to place Catmull-Rom or Bezier camera splines in the world with
   configurable speed, acceleration, and look-at targets so that cameras follow smooth authored
   paths
2. **US-13.5.3.2** — I want to define branching camera paths where the active branch is selected by
   gameplay conditions (boss phase, player choice) so that camera angles adapt to the current game
   state
3. **US-13.5.3.3** — I want to the camera to select different cinematic angles based on the active
   boss phase so that dramatic moments are framed appropriately
4. **US-13.5.3.4** — I want to verify that spline segment triggers fire at their authored world
   positions regardless of camera speed so that trigger placement is reliable

## F-13.5.4 Actor Animation Blending in Cinematics

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-13.5.4.1 | story director (P-4) | F-13.5.4 | R-13.5.4     |
| US-13.5.4.2 | story director (P-4) | F-13.5.4 | R-13.5.4     |
| US-13.5.4.3 | player (P-23)        | F-13.5.4 | R-13.5.4     |
| US-13.5.4.4 | engine tester (P-27) | F-13.5.4 | R-13.5.4     |

1. **US-13.5.4.1** — I want to blend NPC and player character animations from gameplay state into
   cinematic-driven poses with configurable blend durations so that cutscene transitions appear
   natural
2. **US-13.5.4.2** — I want to override only the upper body for cinematic gestures while the lower
   body continues locomotion so that characters perform cinematic actions while walking
3. **US-13.5.4.3** — I want to background NPCs to continue ambient behavior while foreground actors
   perform cinematic actions so that the world feels alive during cutscenes
4. **US-13.5.4.4** — I want to verify that after a cutscene ends, all actors return to their
   pre-cutscene animation state machine state so that cinematic blending leaves no residual
   animation artifacts

## F-13.5.5 Dialogue Triggers and Subtitles

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-13.5.5.1 | story director (P-4) | F-13.5.5 | R-13.5.5     |
| US-13.5.5.2 | story director (P-4) | F-13.5.5 | R-13.5.5     |
| US-13.5.5.3 | player (P-23)        | F-13.5.5 | R-13.5.5     |
| US-13.5.5.4 | engine tester (P-27) | F-13.5.5 | R-13.5.5     |

1. **US-13.5.5.1** — I want to place dialogue events at timeline cue points that trigger voice-over
   playback, subtitle display, and lip-sync animation so that spoken dialogue is synchronized with
   the cinematic
2. **US-13.5.5.2** — I want to cinematic conversations to branch based on player choices or quest
   state through the dialogue tree system so that cutscenes support interactive narratives
3. **US-13.5.5.3** — I want to see localized subtitles with speaker identification during cinematics
   so that I can follow dialogue in my preferred language
4. **US-13.5.5.4** — I want to verify that subtitle display and hide events align with voice-over
   start and end times within 50ms so that subtitles are synchronized with audio

## F-13.5.6a Cutscene Skip System

| ID        | Persona              | Features | Requirements |
|-----------|----------------------|----------|--------------|
| US-13.5.6 | player (P-23)        | F-13.5.6 | R-13.5.6     |
| US-13.5.6 | story director (P-4) | F-13.5.6 | R-13.5.6     |
| US-13.5.6 | engine tester (P-27) | F-13.5.6 | R-13.5.6     |

1. **US-13.5.6** — I want to skip a cutscene and have all gameplay side effects (quest updates, item
   grants, phase transitions) applied immediately so that skipping does not desynchronize my game
   state
2. **US-13.5.6** — I want to configure multiplayer skip behavior (unanimous, majority-vote,
   host-only) so that group cutscene skipping follows the intended social contract
3. **US-13.5.6** — I want to skip a cutscene with multiple gameplay triggers and verify all triggers
   fire so that skipping produces the same end-state as watching

## F-13.5.6b Cutscene Playback Speed

| ID        | Persona              | Features | Requirements |
|-----------|----------------------|----------|--------------|
| US-13.5.6 | player (P-23)        | F-13.5.6 | R-13.5.6     |
| US-13.5.6 | engine tester (P-27) | F-13.5.6 | R-13.5.6     |

1. **US-13.5.6** — I want to fast-forward cutscenes at 2x and 4x speed so that I can view the
   content quickly on repeated playthroughs
2. **US-13.5.6** — I want to fast-forward a cutscene and verify all gameplay triggers fire at their
   correct timeline positions so that fast-forward does not skip triggers

## F-13.5.6c Cutscene Pause

| ID        | Persona              | Features | Requirements |
|-----------|----------------------|----------|--------------|
| US-13.5.6 | player (P-23)        | F-13.5.6 | R-13.5.6     |
| US-13.5.6 | engine tester (P-27) | F-13.5.6 | R-13.5.6     |

1. **US-13.5.6** — I want to pause cutscene playback and resume from the exact frame so that I can
   step away without missing content
2. **US-13.5.6** — I want to pause a cutscene and verify all tracks (camera, animation, audio, VFX)
   freeze at the current frame with no drift so that pause state is exact

## F-13.5.7 Letterboxing and Cinematic Framing

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-13.5.7.1 | story director (P-4) | F-13.5.7 | R-13.5.7     |
| US-13.5.7.2 | player (P-23)        | F-13.5.7 | R-13.5.7     |
| US-13.5.7.3 | engine tester (P-27) | F-13.5.7 | R-13.5.7     |

1. **US-13.5.7.1** — I want to apply configurable letterbox bars (2.39:1, 2.00:1, 1.85:1) with
   animated reveal/hide transitions per sequence so that cinematic framing signals a change in tone
2. **US-13.5.7.2** — I want to letterbox bars, vignettes, and film grain to appear during cinematics
   so that cutscenes feel visually distinct from gameplay
3. **US-13.5.7.3** — I want to verify that letterbox activation hides all gameplay HUD elements and
   suppresses gameplay inputs so that cinematic mode prevents accidental gameplay interactions
