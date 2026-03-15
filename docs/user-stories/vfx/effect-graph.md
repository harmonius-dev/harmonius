# User Stories — 11.6 Effect Graph

## US-11.6.1 Author VFX Visually and Trigger Them from Gameplay Events
**As an** artist, **I want** a node-based effect graph editor with custom nodes, exposed
parameters, event-driven spawning, and automatic LOD budgeting, **so that** I can create
and iterate on VFX entirely in the visual editor and connect them to gameplay without code.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Visual node graph compiling to GPU compute dispatches | F-11.6.1 | R-11.6.1 |
| Custom nodes via logic graph system as library assets | F-11.6.2 | R-11.6.2 |
| Typed parameter slots with per-instance overrides | F-11.6.3 | R-11.6.3 |
| Event-driven spawning from animation, physics, gameplay | F-11.6.4 | R-11.6.4 |
| Distance-based LOD with global performance budget | F-11.6.5 | R-11.6.5 |
| No-code: all VFX authored via visual editor | F-11.6.1 | R-X.9.5 |
| VFX GPU compute within 2ms budget at 60fps | F-11.6.5 | R-X.13.1 |
