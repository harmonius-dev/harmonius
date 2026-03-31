# User Stories — Cinematics (13.5)

## Cinematics Editor and Timeline

| ID          | Persona              |
|-------------|----------------------|
| US-13.5.1.1 | story director (P-4) |
| US-13.5.1.2 | story director (P-4) |
| US-13.5.1.3 | game designer (P-5)  |
| US-13.5.1.4 | game developer (P-15)|

1. **US-13.5.1.1** — **As a** story director (P-4), **I want** a multi-track timeline with tracks
   for camera, animation, audio, VFX, lighting, and gameplay triggers, **so that** I author
   cinematics visually.
2. **US-13.5.1.2** — **As a** story director (P-4), **I want** nested sub-sequences for reusable
   cinematic building blocks, **so that** common sequences are shared across cutscenes.
3. **US-13.5.1.3** — **As a** game designer (P-5), **I want** deterministic evaluation regardless of
   framerate, **so that** cinematics play identically on all hardware.
4. **US-13.5.1.4** — **As a** game developer (P-15), **I want** gameplay trigger tracks that fire
   events at timeline positions, **so that** cinematics drive game state.

## Cutscene Camera System

| ID          | Persona              |
|-------------|----------------------|
| US-13.5.2.1 | story director (P-4) |
| US-13.5.2.2 | story director (P-4) |
| US-13.5.2.3 | player (P-23)        |

1. **US-13.5.2.1** — **As a** story director (P-4), **I want** cinematic camera modes (fixed,
   tracking, orbit, dolly zoom, handheld shake), **so that** I create varied shots.
2. **US-13.5.2.2** — **As a** story director (P-4), **I want** depth-of-field overrides and focal
   target tracking per shot, **so that** each camera angle has its own look.
3. **US-13.5.2.3** — **As a** player (P-23), **I want** smooth transitions between cutscene cameras,
   **so that** cinematic scenes feel polished.

## Camera Rails and Splines

| ID          | Persona              |
|-------------|----------------------|
| US-13.5.3.1 | story director (P-4) |
| US-13.5.3.2 | game designer (P-5)  |

1. **US-13.5.3.1** — **As a** story director (P-4), **I want** to place spline camera paths in the
   world, **so that** cameras follow authored paths.
2. **US-13.5.3.2** — **As a** game designer (P-5), **I want** branching spline paths selectable by
   gameplay conditions, **so that** camera angles adapt to game state.

## Actor Animation Blending

| ID          | Persona              |
|-------------|----------------------|
| US-13.5.4.1 | story director (P-4) |
| US-13.5.4.2 | game designer (P-5)  |
| US-13.5.4.3 | player (P-23)        |

1. **US-13.5.4.1** — **As a** story director (P-4), **I want** smooth blend-in and blend-out between
   gameplay and cinematic poses per actor, **so that** cutscene entries are seamless.
2. **US-13.5.4.2** — **As a** game designer (P-5), **I want** partial body overrides so lower body
   continues locomotion while upper body performs cinematic gestures, **so that** in-game cinematics
   feel natural.
3. **US-13.5.4.3** — **As a** player (P-23), **I want** background NPCs to continue ambient behavior
   during cutscenes, **so that** the world feels alive.

## Dialogue Triggers and Subtitles

| ID          | Persona              |
|-------------|----------------------|
| US-13.5.5.1 | writer (P-17)        |
| US-13.5.5.2 | writer (P-17)        |
| US-13.5.5.3 | player (P-23)        |

1. **US-13.5.5.1** — **As a** writer (P-17), **I want** dialogue events at timeline cue points
   triggering voice-over and subtitles, **so that** dialogue syncs with the cinematic.
2. **US-13.5.5.2** — **As a** writer (P-17), **I want** localized subtitle text with speaker
   identification, **so that** dialogue is accessible in all languages.
3. **US-13.5.5.3** — **As a** player (P-23), **I want** cinematic conversations to branch based on
   my prior choices, **so that** cutscenes reflect my decisions.

## Player Control

| ID           | Persona              |
|--------------|----------------------|
| US-13.5.6a.1 | player (P-23)        |
| US-13.5.6a.2 | game designer (P-5)  |
| US-13.5.6b.1 | player (P-23)        |
| US-13.5.6c.1 | player (P-23)        |
| US-13.5.7.1  | story director (P-4) |

1. **US-13.5.6a.1** — **As a** player (P-23), **I want** to skip cutscenes while all gameplay side
   effects still apply, **so that** skipping does not desynchronize state.
2. **US-13.5.6a.2** — **As a** game designer (P-5), **I want** multiplayer skip to require consensus
   or a majority-vote timeout, **so that** one player cannot skip for everyone.
3. **US-13.5.6b.1** — **As a** player (P-23), **I want** fast-forward playback at 2x and 4x speed,
   **so that** I speed through already-seen cutscenes.
4. **US-13.5.6c.1** — **As a** player (P-23), **I want** to pause cutscene playback and resume from
   the exact frame, **so that** I can attend to interruptions.
5. **US-13.5.7.1** — **As a** story director (P-4), **I want** configurable letterbox bars with
   animated reveal/hide, **so that** cinematic framing signals mode change.
