# User Stories — 6.5 VR and Spatial Input

| ID         | Persona                 |
|------------|-------------------------|
| US-6.5.1.1 | player (P-23)           |
| US-6.5.1.2 | game designer (P-5)     |
| US-6.5.1.3 | engine developer (P-26) |
| US-6.5.2.1 | player (P-23)           |
| US-6.5.2.2 | game designer (P-5)     |
| US-6.5.2.3 | engine developer (P-26) |
| US-6.5.3.1 | player (P-23)           |
| US-6.5.3.2 | game designer (P-5)     |
| US-6.5.3.3 | engine developer (P-26) |
| US-6.5.4.1 | player (P-23)           |
| US-6.5.4.2 | game designer (P-5)     |
| US-6.5.4.3 | engine developer (P-26) |
| US-6.5.5.1 | player (P-23)           |
| US-6.5.5.2 | game designer (P-5)     |
| US-6.5.5.3 | engine developer (P-26) |

1. **US-6.5.1.1** — **As a** player (P-23), **I want** the virtual world to respond instantly to
   head movement at up to 120 Hz, **so that** VR presence is convincing without sickness.
2. **US-6.5.1.2** — **As a** game designer (P-5), **I want** room-scale, seated, and standing play
   areas with configurable guardian boundaries, **so that** my VR experience fits diverse setups.
3. **US-6.5.1.3** — **As an** engine developer (P-26), **I want** late-latching pose updates
   submitted close to scanout, **so that** motion-to-photon latency stays under 20 ms.
4. **US-6.5.2.1** — **As a** player (P-23), **I want** to reach out and interact with objects using
   tracked hand controllers, **so that** VR interaction feels natural.
5. **US-6.5.2.2** — **As a** game designer (P-5), **I want** VR controller bindings configurable in
   the input action system, **so that** VR controls are customizable per game.
6. **US-6.5.2.3** — **As an** engine developer (P-26), **I want** VR controller inputs mapped to the
   same typed action system as gamepad inputs, **so that** shared mappings work across flat and VR.
7. **US-6.5.3.1** — **As a** player (P-23), **I want** to reach out and grab objects with my bare
   hands, **so that** VR interaction is controller-free.
8. **US-6.5.3.2** — **As a** game designer (P-5), **I want** custom hand gestures authored in the
   logic graph using joint angle thresholds, **so that** novel gestures are possible per game.
9. **US-6.5.3.3** — **As an** engine developer (P-26), **I want** automatic switching between
   controller and hand tracking based on controller presence, **so that** transitions are seamless.
10. **US-6.5.4.1** — **As a** player (P-23), **I want** to look at objects to select or interact
    with them via eye tracking, **so that** gaze-based UI works naturally.
11. **US-6.5.4.2** — **As a** game designer (P-5), **I want** gameplay mechanics where enemies react
    to being looked at, **so that** eye tracking adds depth to encounters.
12. **US-6.5.4.3** — **As an** engine developer (P-26), **I want** GazeRay data consumable by the
    foveated rendering system, **so that** GPU shading detail follows gaze direction.
13. **US-6.5.5.1** — **As a** player (P-23), **I want** weapon impacts to vibrate the hand holding
    the weapon, **so that** combat feedback is directional.
14. **US-6.5.5.2** — **As a** game designer (P-5), **I want** VR haptic patterns authored as assets
    in the editor, **so that** haptic design is data-driven.
15. **US-6.5.5.3** — **As an** engine developer (P-26), **I want** VR haptic output abstracted
    across OpenXR, PSVR2, and Quest APIs, **so that** authored patterns work on all platforms.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-6.5.1 | player (P-23) |
| US-6.5.2 | player (P-23) |
| US-6.5.3 | player (P-23) |
| US-6.5.4 | player (P-23) |
| US-6.5.5 | player (P-23) |

1. **US-6.5.1** -- **As a** player (P-23), **I want** the capabilities defined in sub-stories
   US-6.5.1.1 through US-6.5.1.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

2. **US-6.5.2** -- **As a** player (P-23), **I want** the capabilities defined in sub-stories
   US-6.5.2.1 through US-6.5.2.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

3. **US-6.5.3** -- **As a** player (P-23), **I want** the capabilities defined in sub-stories
   US-6.5.3.1 through US-6.5.3.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

4. **US-6.5.4** -- **As a** player (P-23), **I want** the capabilities defined in sub-stories
   US-6.5.4.1 through US-6.5.4.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

5. **US-6.5.5** -- **As a** player (P-23), **I want** the capabilities defined in sub-stories
   US-6.5.5.1 through US-6.5.5.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.
