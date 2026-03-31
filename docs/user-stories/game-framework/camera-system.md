# User Stories — 3D Camera System (13.25)

## Virtual Camera Entity and Priority

| ID           | Persona                |
|--------------|------------------------|
| US-13.25.1.1 | game designer (P-5)    |
| US-13.25.1.2 | game designer (P-5)    |
| US-13.25.1.3 | engine developer (P-26)|
| US-13.25.1.4 | game developer (P-15)  |

1. **US-13.25.1.1** — **As a** game designer (P-5), **I want** each camera behavior to be an ECS
   entity with a priority value, **so that** camera switching is data-driven.
2. **US-13.25.1.2** — **As a** game designer (P-5), **I want** priority modifiable at runtime by
   gameplay events, **so that** cameras activate on triggers or combat.
3. **US-13.25.1.3** — **As a** engine developer (P-26), **I want** virtual cameras to be lightweight
   descriptions feeding computed parameters to the Camera Brain, **so that** multiple cameras
   coexist cheaply.
4. **US-13.25.1.4** — **As a** game developer (P-15), **I want** to register custom camera behaviors
   as ECS components, **so that** I can extend the camera system via the plugin API.

## Camera Brain and Output Controller

| ID           | Persona              |
|--------------|----------------------|
| US-13.25.2.1 | game designer (P-5)  |
| US-13.25.2.2 | game designer (P-5)  |
| US-13.25.2.3 | game developer (P-15)|
| US-13.25.2.4 | player (P-23)        |

1. **US-13.25.2.1** — **As a** game designer (P-5), **I want** the camera brain to select the
   highest-priority camera and drive the final rendered view, **so that** camera management is
   automatic.
2. **US-13.25.2.2** — **As a** game designer (P-5), **I want** configurable update timing (late,
   fixed, manual), **so that** camera updates sync with physics or replay.
3. **US-13.25.2.3** — **As a** game developer (P-15), **I want** multiple brains for split-screen
   with channel masks, **so that** each player has an independent camera.
4. **US-13.25.2.4** — **As a** player (P-23), **I want** smooth blended transitions between cameras,
   **so that** camera changes feel seamless.

## Position Control Behaviors

| ID           | Persona              |
|--------------|----------------------|
| US-13.25.3.1 | game designer (P-5)  |
| US-13.25.4.1 | game designer (P-5)  |
| US-13.25.5.1 | game designer (P-5)  |
| US-13.25.5.2 | player (P-23)        |
| US-13.25.6.1 | game designer (P-5)  |
| US-13.25.7.1 | game designer (P-5)  |
| US-13.25.8.1 | game designer (P-5)  |

1. **US-13.25.3.1** — **As a** game designer (P-5), **I want** a fixed-offset follow with per-axis
   damping and binding modes, **so that** I control how the camera tracks.
2. **US-13.25.4.1** — **As a** game designer (P-5), **I want** an orbital follow driven by player
   input axes with recentering, **so that** the player controls orbit.
3. **US-13.25.5.1** — **As a** game designer (P-5), **I want** a third-person shoulder camera with
   collision resolution, **so that** obstacles do not occlude the player.
4. **US-13.25.5.2** — **As a** player (P-23), **I want** the camera to pull forward when a wall is
   behind me and smoothly return when clear, **so that** I always see my character.
5. **US-13.25.6.1** — **As a** game designer (P-5), **I want** a hard-lock camera that copies the
   target position with no offset, **so that** first-person views are rigid.
6. **US-13.25.7.1** — **As a** game designer (P-5), **I want** an adaptive framing composer with
   dead zone, soft zone, and hard limits, **so that** the target stays at a desired screen position.
7. **US-13.25.8.1** — **As a** game designer (P-5), **I want** a spline dolly that constrains the
   camera to a path with nearest-point-to-target tracking, **so that** cinematic rail shots are
   possible at runtime.

## Rotation Control Behaviors

| ID            | Persona              |
|---------------|----------------------|
| US-13.25.9.1  | game designer (P-5)  |
| US-13.25.10.1 | game designer (P-5)  |
| US-13.25.11.1 | game designer (P-5)  |
| US-13.25.11.2 | player (P-23)        |
| US-13.25.12.1 | game designer (P-5)  |

1. **US-13.25.9.1** — **As a** game designer (P-5), **I want** an adaptive aim composer with dead
   zone and soft zone, **so that** the camera re-aims smoothly.
2. **US-13.25.10.1** — **As a** game designer (P-5), **I want** a hard look-at that keeps the target
   centered with no damping, **so that** lock-on cameras are rigid.
3. **US-13.25.11.1** — **As a** game designer (P-5), **I want** input-driven pan and tilt with
   configurable range and recentering, **so that** the player controls rotation.
4. **US-13.25.11.2** — **As a** player (P-23), **I want** camera rotation to recenter automatically
   when I stop looking around, **so that** I face forward by default.
5. **US-13.25.12.1** — **As a** game designer (P-5), **I want** a rotation mode that copies the
   target's facing, **so that** vehicle cameras match heading.

## Spring Arm and Collision

| ID            | Persona              |
|---------------|----------------------|
| US-13.25.13.1 | game designer (P-5)  |
| US-13.25.14.1 | game designer (P-5)  |
| US-13.25.15.1 | game designer (P-5)  |
| US-13.25.15.2 | player (P-23)        |

1. **US-13.25.13.1** — **As a** game designer (P-5), **I want** a spring arm that retracts on
   collision and extends when clear with configurable lag, **so that** the camera avoids clipping
   through geometry.
2. **US-13.25.14.1** — **As a** game designer (P-5), **I want** a deoccluder that repositions the
   camera to maintain line of sight to the target, **so that** obstacles never hide the focus
   subject.
3. **US-13.25.15.1** — **As a** game designer (P-5), **I want** a decollider that prevents the
   camera from entering solid geometry, **so that** the view never shows inside walls.
4. **US-13.25.15.2** — **As a** player (P-23), **I want** the camera to never clip inside terrain or
   walls, **so that** the view stays usable in tight spaces.

## Camera Blending and Transitions

| ID            | Persona              |
|---------------|----------------------|
| US-13.25.16.1 | game designer (P-5)  |
| US-13.25.16.2 | game designer (P-5)  |
| US-13.25.17.1 | story director (P-4) |

1. **US-13.25.16.1** — **As a** game designer (P-5), **I want** configurable blend curves (cut,
   ease-in-out, linear, custom) between cameras, **so that** transitions match the desired pacing.
2. **US-13.25.16.2** — **As a** game designer (P-5), **I want** per-camera-pair custom blend rules
   with wildcard fallback, **so that** specific transitions are individually tuned.
3. **US-13.25.17.1** — **As a** story director (P-4), **I want** weighted multi-camera blending of
   up to eight cameras, **so that** smooth multi-perspective shots are possible.

## Camera Shake and Noise

| ID            | Persona              |
|---------------|----------------------|
| US-13.25.18.1 | game designer (P-5)  |
| US-13.25.19.1 | game designer (P-5)  |
| US-13.25.20.1 | game designer (P-5)  |
| US-13.25.21.1 | game designer (P-5)  |
| US-13.25.22.1 | story director (P-4) |
| US-13.25.19.2 | player (P-23)        |

1. **US-13.25.18.1** — **As a** game designer (P-5), **I want** multi-channel Perlin noise profiles
   with animatable amplitude and frequency, **so that** camera noise feels organic and tunable.
2. **US-13.25.19.1** — **As a** game designer (P-5), **I want** impulse sources that emit shake
   signals propagating outward from a world position with attenuation, **so that** explosions shake
   nearby cameras.
3. **US-13.25.19.2** — **As a** player (P-23), **I want** camera shake intensity to diminish with
   distance from the source, **so that** distant events feel appropriately subtle.
4. **US-13.25.20.1** — **As a** game designer (P-5), **I want** sinusoidal wave oscillation with
   per-axis amplitude and frequency, **so that** rocking and breathing effects are smooth and
   periodic.
5. **US-13.25.21.1** — **As a** game designer (P-5), **I want** composite shake patterns layering
   noise, wave, and animation, **so that** complex effects combine multiple shake types.
6. **US-13.25.22.1** — **As a** story director (P-4), **I want** cinematics-editor-driven shake from
   keyframed animation sequences, **so that** choreographed shake is precise.

## Camera Intelligence

| ID            | Persona              |
|---------------|----------------------|
| US-13.25.23.1 | game designer (P-5)  |
| US-13.25.24.1 | game designer (P-5)  |
| US-13.25.25.1 | game developer (P-15)|
| US-13.25.26.1 | game designer (P-5)  |

1. **US-13.25.23.1** — **As a** game designer (P-5), **I want** state-driven camera switching mapped
   to animation states, **so that** cameras change automatically with gameplay.
2. **US-13.25.24.1** — **As a** game designer (P-5), **I want** a clear-shot manager that evaluates
   shot quality and selects the best unobstructed camera, **so that** the view is always useful.
3. **US-13.25.25.1** — **As a** game developer (P-15), **I want** a shot quality evaluator returning
   a 0-1 score based on target visibility and distance, **so that** I can build custom camera
   selection logic.
4. **US-13.25.26.1** — **As a** game designer (P-5), **I want** a timed camera playlist with hold
   duration per camera, **so that** attract-mode flyovers work without the full cinematics editor.

## Group and Multi-Target

| ID            | Persona              |
|---------------|----------------------|
| US-13.25.27.1 | game designer (P-5)  |
| US-13.25.28.1 | game designer (P-5)  |
| US-13.25.28.2 | player (P-23)        |

1. **US-13.25.27.1** — **As a** game designer (P-5), **I want** a target group entity that
   aggregates multiple targets for camera tracking, **so that** multi-character scenes use a single
   virtual target.
2. **US-13.25.28.1** — **As a** game designer (P-5), **I want** a group framing extension that
   adjusts zoom and position to keep all members in frame, **so that** the camera adapts to group
   spread.
3. **US-13.25.28.2** — **As a** player (P-23), **I want** the camera to zoom out as my party spreads
   apart, **so that** all characters remain visible.

## Camera Extensions

| ID            | Persona              |
|---------------|----------------------|
| US-13.25.29.1 | game designer (P-5)  |
| US-13.25.30.1 | game designer (P-5)  |
| US-13.25.31.1 | game designer (P-5)  |
| US-13.25.32.1 | game designer (P-5)  |
| US-13.25.33.1 | game developer (P-15)|
| US-13.25.34.1 | game designer (P-5)  |
| US-13.25.35.1 | story director (P-4) |
| US-13.25.36.1 | game developer (P-15)|

1. **US-13.25.29.1** — **As a** game designer (P-5), **I want** a 2D confiner that keeps screen
   edges within a polygon, **so that** the camera stays in the play area.
2. **US-13.25.30.1** — **As a** game designer (P-5), **I want** a 3D confiner that restricts camera
   position to a bounding volume, **so that** the camera stays in arenas.
3. **US-13.25.31.1** — **As a** game designer (P-5), **I want** follow zoom that adjusts FOV to
   maintain constant screen-space target size, **so that** subjects remain consistently framed.
4. **US-13.25.32.1** — **As a** game designer (P-5), **I want** auto focus that controls focus
   distance for depth-of-field, **so that** the tracked subject is always sharp.
5. **US-13.25.33.1** — **As a** game developer (P-15), **I want** a third-person aim extension that
   resolves parallax between camera and weapon origin, **so that** the player hits what the
   crosshair shows.
6. **US-13.25.34.1** — **As a** game designer (P-5), **I want** a FreeLook modifier that adjusts FOV
   and noise based on orbit position, **so that** camera feel varies by angle.
7. **US-13.25.35.1** — **As a** story director (P-4), **I want** a recomposer extension for final
   compositional tweaks (tilt, pan, dutch, zoom), **so that** I can hand-tune shots on top of
   procedural motion.
8. **US-13.25.36.1** — **As a** game developer (P-15), **I want** a modifier stack on the camera
   brain that runs custom modifiers in priority order, **so that** I can add camera effects without
   modifying the core system.

## Input Integration

| ID            | Persona              |
|---------------|----------------------|
| US-13.25.37.1 | game designer (P-5)  |
| US-13.25.37.2 | player (P-23)        |

1. **US-13.25.37.1** — **As a** game designer (P-5), **I want** a camera input controller that
   bridges player input to orbit, pan, tilt, and zoom, **so that** camera control is frame-rate
   independent.
2. **US-13.25.37.2** — **As a** player (P-23), **I want** camera input suppressed during blending,
   **so that** transitions are not disrupted by my stick input.

## Cinematic Camera Features

| ID            | Persona              |
|---------------|----------------------|
| US-13.25.38.1 | story director (P-4) |
| US-13.25.39.1 | story director (P-4) |
| US-13.25.40.1 | game designer (P-5)  |
| US-13.25.40.2 | player (P-23)        |

1. **US-13.25.38.1** — **As a** story director (P-4), **I want** physical camera simulation with
   sensor size, focal length, and aperture, **so that** cinematic rendering matches real-world
   lenses.
2. **US-13.25.39.1** — **As a** story director (P-4), **I want** pre-built dolly and crane rig
   entities with keyframeable parameters, **so that** physical camera movement is easy to author.
3. **US-13.25.40.1** — **As a** game designer (P-5), **I want** picture-in-picture rendering from a
   secondary camera, **so that** rear-view mirrors and security feeds work.
4. **US-13.25.40.2** — **As a** player (P-23), **I want** a rear-view mirror in racing or a security
   cam feed in stealth, **so that** I have awareness of multiple viewpoints simultaneously.
