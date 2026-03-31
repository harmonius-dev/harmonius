# User Stories — 10.5 2D Game Support

| ID         | Persona                 |
|------------|-------------------------|
| US-10.5.1  | game developer (P-15)   |
| US-10.5.2  | game designer (P-5)     |
| US-10.5.3  | technical artist (P-13) |
| US-10.5.4  | game developer (P-15)   |
| US-10.5.5  | game designer (P-5)     |
| US-10.5.6  | game designer (P-5)     |
| US-10.5.7  | game developer (P-15)   |
| US-10.5.8  | game developer (P-15)   |
| US-10.5.9  | game developer (P-15)   |
| US-10.5.10 | game developer (P-15)   |
| US-10.5.11 | technical artist (P-13) |
| US-10.5.12 | game designer (P-5)     |
| US-10.5.13 | player (P-23)           |
| US-10.5.14 | game developer (P-15)   |
| US-10.5.15 | game developer (P-15)   |
| US-10.5.16 | player (P-23)           |
| US-10.5.17 | game developer (P-15)   |

1. **US-10.5.1** — **As a** game developer (P-15), **I want** sprites rendered as instanced textured
   quads batched by atlas page with z-order sorting, **so that** thousands of sprites draw
   efficiently.
2. **US-10.5.2** — **As a** game designer (P-5), **I want** frame-based sprite animation with
   looping modes and animation events, **so that** characters animate with precise timing.
3. **US-10.5.3** — **As a** technical artist (P-13), **I want** bone-based 2D skeletal animation
   with mesh deformation and Spine/DragonBones import, **so that** characters move smoothly beyond
   frame-by-frame.
4. **US-10.5.4** — **As a** game developer (P-15), **I want** 2D shapes rendered from vector paths
   at any resolution without pixelation, **so that** scalable elements work across devices.
5. **US-10.5.5** — **As a** game designer (P-5), **I want** chunked tilemap rendering with
   auto-tiling, animated tiles, and viewport culling, **so that** large tile worlds stream
   efficiently.
6. **US-10.5.6** — **As a** game designer (P-5), **I want** isometric and hexagonal grid layouts
   with correct depth sorting and coordinate conversion, **so that** I can build strategy and RPG
   maps.
7. **US-10.5.7** — **As a** game developer (P-15), **I want** procedural tilemap generation with
   WFC, cellular automata, and BSP dungeons, **so that** roguelikes have explorable content.
8. **US-10.5.8** — **As a** game developer (P-15), **I want** a 2D camera with parallax scrolling,
   smoothing, shake, and split-screen, **so that** any 2D genre has proper camera behavior.
9. **US-10.5.9** — **As a** game developer (P-15), **I want** 2D rigid body physics as ECS
   components with CCD, one-way platforms, and determinism, **so that** 2D games have reliable
   physics.
10. **US-10.5.10** — **As a** game developer (P-15), **I want** 2D collider shapes with tilemap
    collision auto-generating optimized edge chains, **so that** worlds have efficient collision.
11. **US-10.5.11** — **As a** technical artist (P-13), **I want** dynamic 2D lighting with point
    lights, shadows from sprite edges, and normal-mapped sprites, **so that** 2D scenes have
    atmosphere.
12. **US-10.5.12** — **As a** game designer (P-5), **I want** on-screen virtual joystick, D-pad, and
    action buttons feeding into the input action system, **so that** mobile players have responsive
    controls.
13. **US-10.5.13** — **As a** player (P-23), **I want** pinch-to-zoom, pan, swipe, and long-press
    mapped to 2D game actions, **so that** I control the game with natural gestures.
14. **US-10.5.14** — **As a** game developer (P-15), **I want** 2D state replication optimized for
    Transform2D delta compression and 2D relevancy, **so that** 2D multiplayer is
    bandwidth-efficient.
15. **US-10.5.15** — **As a** game developer (P-15), **I want** rollback netcode saving and
    re-simulating 2D physics state on misprediction, **so that** competitive 2D games stay
    synchronized.
16. **US-10.5.16** — **As a** player (P-23), **I want** 3D characters composited into a 2D scene
    with correct depth ordering, **so that** 2.5D games look and play correctly.
17. **US-10.5.17** — **As a** game developer (P-15), **I want** arbitrary layering of 2D and 3D
    assets with configurable depth and blend modes, **so that** hybrid scenes have correct
    occlusion.
