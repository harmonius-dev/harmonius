# Requirements

Requirements for the Harmonius game engine — an open world MMO-scale engine.
Each requirement has a unique identifier (e.g., `R-2.3.1` is the first requirement
in category 2.3). Requirements trace to features in [docs/features/](../features/).

## Domains

| # | Domain | Description |
|---|--------|-------------|
| 1 | [Core Runtime](core-runtime/) | ECS, scene, reflection, serialization, events, plugins, memory |
| 2 | [Rendering](rendering/) | GPU abstraction, render graph, lighting, materials, post-processing |
| 3 | [Geometry & World](geometry-world/) | Meshlets, terrain, foliage, procedural generation |
| 4 | [Physics](physics/) | Rigid body, collision, constraints, vehicles, destruction, fluids |
| 5 | [Audio](audio/) | Engine, spatial, DSP, music, voice |
| 6 | [Input](input/) | Devices, actions, gestures, haptics |
| 7 | [AI](ai/) | Navigation, steering, behavior trees, perception, crowds |
| 8 | [Networking](networking/) | Transport, replication, prediction, MMO infrastructure |
| 9 | [Animation](animation/) | Skeletal, morph, procedural, state machines, cloth & hair |
| 10 | [UI & 2D](ui-2d/) | Widgets, UI rendering, 2D games, accessibility |
| 11 | [VFX](vfx/) | Particles, fluid VFX, effect graph |
| 12 | [Content Pipeline](content-pipeline/) | Asset import, processing, shaders, streaming, registry |
| 13 | [Game Framework](game-framework/) | Gameplay, camera, cinematics, scripting, save, localization |
| 14 | [Platform](platform/) | Windowing, OS integration, platform services, distribution |
| 15 | [Tools & Editor](tools-editor/) | Scene/material/animation/VFX editors, profiler, debugger |
