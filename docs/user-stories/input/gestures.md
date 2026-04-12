# User Stories — 6.3 Gestures

| ID         | Persona                 |
|------------|-------------------------|
| US-6.3.1.1 | player (P-23)           |
| US-6.3.1.2 | game designer (P-5)     |
| US-6.3.1.3 | engine developer (P-26) |
| US-6.3.2.1 | player (P-23)           |
| US-6.3.2.2 | game designer (P-5)     |
| US-6.3.2.3 | engine developer (P-26) |
| US-6.3.3.1 | player (P-23)           |
| US-6.3.3.2 | player (P-23)           |
| US-6.3.3.3 | engine developer (P-26) |
| US-6.3.4.1 | player (P-23)           |
| US-6.3.4.2 | game designer (P-5)     |
| US-6.3.4.3 | engine developer (P-26) |
| US-6.3.5.1 | game designer (P-5)     |
| US-6.3.5.2 | player (P-23)           |
| US-6.3.5.3 | game designer (P-5)     |

1. **US-6.3.1.1** — **As a** player (P-23), **I want** to tap enemies to select them on touchscreen,
   **so that** target selection is intuitive.
2. **US-6.3.1.2** — **As a** game designer (P-5), **I want** tap timing and distance thresholds
   configurable in the editor, **so that** gesture sensitivity is tunable.
3. **US-6.3.1.3** — **As an** engine developer (P-26), **I want** gesture distance thresholds scaled
   with display DPI, **so that** recognition is consistent across screens.
4. **US-6.3.2.1** — **As a** player (P-23), **I want** to swipe to trigger dodge in the swipe
   direction, **so that** touch combat feels responsive.
5. **US-6.3.2.2** — **As a** game designer (P-5), **I want** swipe velocity thresholds tunable per
   platform, **so that** different touch surfaces are accommodated.
6. **US-6.3.2.3** — **As an** engine developer (P-26), **I want** swipe detection to filter out
   incidental movement during tap and long press, **so that** gestures do not conflict.
7. **US-6.3.3.1** — **As a** player (P-23), **I want** to pinch to zoom the camera on mobile,
   **so that** view distance is controllable by touch.
8. **US-6.3.3.2** — **As a** player (P-23), **I want** two-finger rotation to rotate the map,
   **so that** map orientation is adjustable.
9. **US-6.3.3.3** — **As an** engine developer (P-26), **I want** pinch and pan to run
   simultaneously, **so that** camera zoom-and-orbit is a single gesture.
10. **US-6.3.4.1** — **As a** player (P-23), **I want** touch input to work without gestures eating
    each other, **so that** controls are reliable.
11. **US-6.3.4.2** — **As a** game designer (P-5), **I want** conflict resolution strategies
    (require- failure, simultaneous, priority) configurable in the editor, **so that** gesture
    behavior is tunable per game.
12. **US-6.3.4.3** — **As an** engine developer (P-26), **I want** gesture recognizers as composable
    state machines with lifecycle states, **so that** recognition is well-defined and testable.
13. **US-6.3.5.1** — **As a** game designer (P-5), **I want** to define custom gesture recognizers
    visually by composing primitives on a timeline, **so that** novel gestures are authored without
    code.
14. **US-6.3.5.2** — **As a** player (P-23), **I want** to draw rune shapes on screen to cast
    spells, **so that** gesture-based magic feels immersive.
15. **US-6.3.5.3** — **As a** game designer (P-5), **I want** custom gestures evaluated at runtime
    through the logic graph system, **so that** recognition is data-driven and hot-reloadable.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-6.3.1 | player (P-23) |
| US-6.3.2 | player (P-23) |
| US-6.3.3 | player (P-23) |
| US-6.3.4 | player (P-23) |
| US-6.3.5 | game designer (P-5) |

1. **US-6.3.1** -- **As a** player (P-23), **I want** the capabilities defined in sub-stories
   US-6.3.1.1 through US-6.3.1.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

2. **US-6.3.2** -- **As a** player (P-23), **I want** the capabilities defined in sub-stories
   US-6.3.2.1 through US-6.3.2.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

3. **US-6.3.3** -- **As a** player (P-23), **I want** the capabilities defined in sub-stories
   US-6.3.3.1 through US-6.3.3.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

4. **US-6.3.4** -- **As a** player (P-23), **I want** the capabilities defined in sub-stories
   US-6.3.4.1 through US-6.3.4.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

5. **US-6.3.5** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-6.3.5.1 through US-6.3.5.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.
