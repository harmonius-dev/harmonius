# Harmonius Game Engine -- User Stories

Exhaustive user stories for the Harmonius game engine covering every feature from multiple persona
perspectives. Each story represents a single action by a single persona in a single use case of a
single feature.

See [personas.md](personas.md) for the complete persona catalog (27 personas).

## Summary

| # | Module | Files | Stories |
|---|--------|-------|---------|
| 1 | [Core Runtime](core-runtime/) | 9 | 181 |
| 2 | [Rendering](rendering/) | 12 | 324 |
| 3 | [Geometry & World](geometry-world/) | 6 | 108 |
| 4 | [Physics](physics/) | 8 | 742 |
| 5 | [Audio](audio/) | 5 | 456 |
| 6 | [Input](input/) | 5 | 372 |
| 7 | [AI](ai/) | 8 | 804 |
| 8 | [Networking](networking/) | 8 | 106 |
| 9 | [Animation](animation/) | 6 | 119 |
| 10 | [UI & 2D](ui-2d/) | 6 | 144 |
| 11 | [VFX](vfx/) | 6 | 100 |
| 12 | [Content Pipeline](content-pipeline/) | 7 | 144 |
| 13 | [Game Framework](game-framework/) | 28 | 1524 |
| 14 | [Platform](platform/) | 6 | 91 |
| 15 | [Tools & Editor](tools-editor/) | 20 | 666 |
| | **Total** | **140** | **5,881** |

## 1. Core Runtime

| Area                     | Stories |
|--------------------------|---------|
| Entity Component System  | 53      |
| Async I/O                | 20      |
| Memory Management        | 18      |
| Serialization            | 17      |
| Events & Messaging       | 16      |
| Spatial Indexing         | 16      |
| Plugin System            | 14      |
| Reflection & Type System | 14      |
| Scene & Transforms       | 13      |

1. **[entity-component-system.md](core-runtime/entity-component-system.md)** —
   [entity-component-system.md](core-runtime/entity-component-system.md)
2. **[async-io.md](core-runtime/async-io.md)** — [async-io.md](core-runtime/async-io.md)
3. **[memory-management.md](core-runtime/memory-management.md)** —
   [memory-management.md](core-runtime/memory-management.md)
4. **[serialization.md](core-runtime/serialization.md)** —
   [serialization.md](core-runtime/serialization.md)
5. **[events-and-messaging.md](core-runtime/events-and-messaging.md)** —
   [events-and-messaging.md](core-runtime/events-and-messaging.md)
6. **[spatial-indexing.md](core-runtime/spatial-indexing.md)** —
   [spatial-indexing.md](core-runtime/spatial-indexing.md)
7. **[plugin-system.md](core-runtime/plugin-system.md)** —
   [plugin-system.md](core-runtime/plugin-system.md)
8. **[reflection-and-type-system.md](core-runtime/reflection-and-type-system.md)** —
   [reflection-and-type-system.md](core-runtime/reflection-and-type-system.md)
9. **[scene-and-transforms.md](core-runtime/scene-and-transforms.md)** —
   [scene-and-transforms.md](core-runtime/scene-and-transforms.md)

## 2. Rendering

| Area                      | Stories |
|---------------------------|---------|
| Lighting                  | 46      |
| Post-Processing           | 42      |
| Advanced Rendering        | 33      |
| Core Rendering            | 30      |
| Render Graph              | 26      |
| Advanced Materials        | 25      |
| GPU Abstraction Layer     | 25      |
| Environment               | 23      |
| Scene Rendering Pipeline  | 21      |
| Anti-Aliasing & Upscaling | 19      |
| Character Rendering       | 19      |
| Stylized Effects          | 15      |

1. **[lighting.md](rendering/lighting.md)** — [lighting.md](rendering/lighting.md)
2. **[post-processing.md](rendering/post-processing.md)** —
   [post-processing.md](rendering/post-processing.md)
3. **[advanced-rendering.md](rendering/advanced-rendering.md)** —
   [advanced-rendering.md](rendering/advanced-rendering.md)
4. **[core-rendering.md](rendering/core-rendering.md)** —
   [core-rendering.md](rendering/core-rendering.md)
5. **[render-graph.md](rendering/render-graph.md)** — [render-graph.md](rendering/render-graph.md)
6. **[advanced-materials.md](rendering/advanced-materials.md)** —
   [advanced-materials.md](rendering/advanced-materials.md)
7. **[gpu-abstraction-layer.md](rendering/gpu-abstraction-layer.md)** —
   [gpu-abstraction-layer.md](rendering/gpu-abstraction-layer.md)
8. **[environment.md](rendering/environment.md)** — [environment.md](rendering/environment.md)
9. **[scene-rendering-pipeline.md](rendering/scene-rendering-pipeline.md)** —
   [scene-rendering-pipeline.md](rendering/scene-rendering-pipeline.md)
10. **[anti-aliasing-upscaling.md](rendering/anti-aliasing-upscaling.md)** —
    [anti-aliasing-upscaling.md](rendering/anti-aliasing-upscaling.md)
11. **[character-rendering.md](rendering/character-rendering.md)** —
    [character-rendering.md](rendering/character-rendering.md)
12. **[stylized-effects.md](rendering/stylized-effects.md)** —
    [stylized-effects.md](rendering/stylized-effects.md)

## 3. Geometry & World

| File | Area | Stories |
|------|------|---------|
| [procedural-generation.md](geometry-world/procedural-generation.md) | Procedural Generation | 64 |
| [terrain.md](geometry-world/terrain.md) | Terrain | 15 |
| [meshlet-pipeline.md](geometry-world/meshlet-pipeline.md) | Meshlet Pipeline | 8 |
| [foliage.md](geometry-world/foliage.md) | Foliage | 7 |
| [sky-atmosphere.md](geometry-world/sky-atmosphere.md) | Sky & Atmosphere | 7 |
| [water.md](geometry-world/water.md) | Water | 7 |

## 4. Physics

| File | Area | Stories |
|------|------|---------|
| [rigid-body-dynamics.md](physics/rigid-body-dynamics.md) | Rigid Body Dynamics | 118 |
| [collision-detection.md](physics/collision-detection.md) | Collision Detection | 108 |
| [constraints-and-joints.md](physics/constraints-and-joints.md) | Constraints & Joints | 108 |
| [destruction-and-fracture.md](physics/destruction-and-fracture.md) | Destruction & Fracture | 84 |
| [fluid-simulation.md](physics/fluid-simulation.md) | Fluid Simulation | 84 |
| [soft-body-and-cloth.md](physics/soft-body-and-cloth.md) | Soft Body & Cloth | 84 |
| [vehicle-physics.md](physics/vehicle-physics.md) | Vehicle Physics | 84 |
| [spatial-queries.md](physics/spatial-queries.md) | Spatial Queries | 72 |

## 5. Audio

| File | Area | Stories |
|------|------|---------|
| [voice-and-speech.md](audio/voice-and-speech.md) | Voice & Speech | 108 |
| [dsp-and-effects.md](audio/dsp-and-effects.md) | DSP & Effects | 96 |
| [audio-engine.md](audio/audio-engine.md) | Audio Engine | 84 |
| [music-system.md](audio/music-system.md) | Music System | 84 |
| [spatial-audio.md](audio/spatial-audio.md) | Spatial Audio | 84 |

## 6. Input

| Area                    | Stories |
|-------------------------|---------|
| Input Actions & Mapping | 132     |
| Device Abstraction      | 60      |
| Gestures                | 60      |
| Haptics & Feedback      | 60      |
| VR Input                | 60      |

1. **[input-actions-and-mapping.md](input/input-actions-and-mapping.md)** —
   [input-actions-and-mapping.md](input/input-actions-and-mapping.md)
2. **[device-abstraction.md](input/device-abstraction.md)** —
   [device-abstraction.md](input/device-abstraction.md)
3. **[gestures.md](input/gestures.md)** — [gestures.md](input/gestures.md)
4. **[haptics-and-feedback.md](input/haptics-and-feedback.md)** —
   [haptics-and-feedback.md](input/haptics-and-feedback.md)
5. **[vr-input.md](input/vr-input.md)** — [vr-input.md](input/vr-input.md)

## 7. AI

| File | Area | Stories |
|------|------|---------|
| [navigation.md](ai/navigation.md) | Navigation | 180 |
| [crowd-simulation.md](ai/crowd-simulation.md) | Crowd Simulation | 132 |
| [perception.md](ai/perception.md) | Perception | 132 |
| [behavior-trees.md](ai/behavior-trees.md) | Behavior Trees | 84 |
| [goap.md](ai/goap.md) | GOAP | 72 |
| [steering-avoidance.md](ai/steering-avoidance.md) | Steering & Avoidance | 72 |
| [tactical-combat.md](ai/tactical-combat.md) | Tactical Combat | 72 |
| [utility-ai.md](ai/utility-ai.md) | Utility AI | 60 |

## 8. Networking

| File | Area | Stories |
|------|------|---------|
| [session-management.md](networking/session-management.md) | Session Management | 18 |
| [mmo-infrastructure.md](networking/mmo-infrastructure.md) | MMO Infrastructure | 16 |
| [anti-cheat.md](networking/anti-cheat.md) | Anti-Cheat | 13 |
| [transport-layer.md](networking/transport-layer.md) | Transport Layer | 13 |
| [prediction-and-rollback.md](networking/prediction-and-rollback.md) | Prediction & Rollback | 12 |
| [replay-system.md](networking/replay-system.md) | Replay System | 12 |
| [state-replication.md](networking/state-replication.md) | State Replication | 12 |
| [remote-procedure-calls.md](networking/remote-procedure-calls.md) | Remote Procedure Calls | 10 |

## 9. Animation

| File | Area | Stories |
|------|------|---------|
| [procedural.md](animation/procedural.md) | Procedural | 27 |
| [skeletal.md](animation/skeletal.md) | Skeletal | 25 |
| [state-machine.md](animation/state-machine.md) | State Machine | 25 |
| [cloth-hair.md](animation/cloth-hair.md) | Cloth & Hair | 15 |
| [morph.md](animation/morph.md) | Morph | 14 |
| [first-person.md](animation/first-person.md) | First Person | 13 |

## 10. UI & 2D

| File | Area | Stories |
|------|------|---------|
| [hud-and-game-ui.md](ui-2d/hud-and-game-ui.md) | HUD & Game UI | 34 |
| [2d-games.md](ui-2d/2d-games.md) | 2D Games | 33 |
| [widget-framework.md](ui-2d/widget-framework.md) | Widget Framework | 24 |
| [accessibility.md](ui-2d/accessibility.md) | Accessibility | 21 |
| [common-widgets.md](ui-2d/common-widgets.md) | Common Widgets | 18 |
| [ui-rendering.md](ui-2d/ui-rendering.md) | UI Rendering | 14 |

## 11. VFX

| File | Area | Stories |
|------|------|---------|
| [weather-environmental.md](vfx/weather-environmental.md) | Weather & Environmental | 20 |
| [destruction.md](vfx/destruction.md) | Destruction | 18 |
| [particles.md](vfx/particles.md) | Particles | 18 |
| [decals.md](vfx/decals.md) | Decals | 15 |
| [screen-effects.md](vfx/screen-effects.md) | Screen Effects | 15 |
| [effect-graph.md](vfx/effect-graph.md) | Effect Graph | 14 |

## 12. Content Pipeline

| File | Area | Stories |
|------|------|---------|
| [dcc-plugins.md](content-pipeline/dcc-plugins.md) | DCC Plugins | 43 |
| [asset-database.md](content-pipeline/asset-database.md) | Asset Database | 20 |
| [asset-processing.md](content-pipeline/asset-processing.md) | Asset Processing | 20 |
| [streaming-io.md](content-pipeline/streaming-io.md) | Streaming I/O | 20 |
| [hot-reload.md](content-pipeline/hot-reload.md) | Hot Reload | 16 |
| [asset-versioning.md](content-pipeline/asset-versioning.md) | Asset Versioning | 15 |
| [asset-import.md](content-pipeline/asset-import.md) | Asset Import | 10 |

## 13. Game Framework

| Area                    | Stories |
|-------------------------|---------|
| Camera System           | 125     |
| Building & Survival     | 96      |
| Social                  | 94      |
| Block & Voxel           | 84      |
| Monetization            | 80      |
| Progression             | 78      |
| Traversal & Interaction | 77      |
| NPC Simulation          | 75      |
| Weapon System           | 74      |
| Game Modes & Misc       | 67      |
| Gameplay Primitives     | 62      |
| Selection System        | 55      |
| Pets & Mounts           | 53      |
| Minigames               | 52      |
| Character Customization | 49      |
| Gameplay Databases      | 42      |
| Quest & Dialogue        | 41      |
| Inventory               | 38      |
| Stealth & Cover         | 35      |
| Racing                  | 33      |
| Turn-Based              | 33      |
| World Management        | 32      |
| Abilities               | 30      |
| Cinematics              | 30      |
| Advertising             | 25      |
| Save System             | 24      |
| Fog of War              | 23      |
| Scripting               | 17      |

1. **[camera-system.md](game-framework/camera-system.md)** —
   [camera-system.md](game-framework/camera-system.md)
2. **[building-survival.md](game-framework/building-survival.md)** —
   [building-survival.md](game-framework/building-survival.md)
3. **[social.md](game-framework/social.md)** — [social.md](game-framework/social.md)
4. **[block-voxel.md](game-framework/block-voxel.md)** —
   [block-voxel.md](game-framework/block-voxel.md)
5. **[monetization.md](game-framework/monetization.md)** —
   [monetization.md](game-framework/monetization.md)
6. **[progression.md](game-framework/progression.md)** —
   [progression.md](game-framework/progression.md)
7. **[traversal-interaction.md](game-framework/traversal-interaction.md)** —
   [traversal-interaction.md](game-framework/traversal-interaction.md)
8. **[npc-simulation.md](game-framework/npc-simulation.md)** —
   [npc-simulation.md](game-framework/npc-simulation.md)
9. **[weapon-system.md](game-framework/weapon-system.md)** —
   [weapon-system.md](game-framework/weapon-system.md)
10. **[game-modes-misc.md](game-framework/game-modes-misc.md)** —
    [game-modes-misc.md](game-framework/game-modes-misc.md)
11. **[gameplay-primitives.md](game-framework/gameplay-primitives.md)** —
    [gameplay-primitives.md](game-framework/gameplay-primitives.md)
12. **[selection-system.md](game-framework/selection-system.md)** —
    [selection-system.md](game-framework/selection-system.md)
13. **[pets-mounts.md](game-framework/pets-mounts.md)** —
    [pets-mounts.md](game-framework/pets-mounts.md)
14. **[minigames.md](game-framework/minigames.md)** — [minigames.md](game-framework/minigames.md)
15. **[character-customization.md](game-framework/character-customization.md)** —
    [character-customization.md](game-framework/character-customization.md)
16. **[gameplay-databases.md](game-framework/gameplay-databases.md)** —
    [gameplay-databases.md](game-framework/gameplay-databases.md)
17. **[quest-dialogue.md](game-framework/quest-dialogue.md)** —
    [quest-dialogue.md](game-framework/quest-dialogue.md)
18. **[inventory.md](game-framework/inventory.md)** — [inventory.md](game-framework/inventory.md)
19. **[stealth-cover.md](game-framework/stealth-cover.md)** —
    [stealth-cover.md](game-framework/stealth-cover.md)
20. **[racing.md](game-framework/racing.md)** — [racing.md](game-framework/racing.md)
21. **[turn-based.md](game-framework/turn-based.md)** —
    [turn-based.md](game-framework/turn-based.md)
22. **[world-management.md](game-framework/world-management.md)** —
    [world-management.md](game-framework/world-management.md)
23. **[abilities.md](game-framework/abilities.md)** — [abilities.md](game-framework/abilities.md)
24. **[cinematics.md](game-framework/cinematics.md)** —
    [cinematics.md](game-framework/cinematics.md)
25. **[advertising.md](game-framework/advertising.md)** —
    [advertising.md](game-framework/advertising.md)
26. **[save-system.md](game-framework/save-system.md)** —
    [save-system.md](game-framework/save-system.md)
27. **[fog-of-war.md](game-framework/fog-of-war.md)** —
    [fog-of-war.md](game-framework/fog-of-war.md)
28. **[scripting.md](game-framework/scripting.md)** — [scripting.md](game-framework/scripting.md)

## 14. Platform

| File | Area | Stories |
|------|------|---------|
| [platform-services.md](platform/platform-services.md) | Platform Services | 28 |
| [window-display.md](platform/window-display.md) | Window & Display | 14 |
| [os-integration.md](platform/os-integration.md) | OS Integration | 13 |
| [crash-reporting.md](platform/crash-reporting.md) | Crash Reporting | 12 |
| [filesystem.md](platform/filesystem.md) | Filesystem | 12 |
| [threading-async.md](platform/threading-async.md) | Threading & Async | 12 |

## 15. Tools & Editor

| File | Area | Stories |
|------|------|---------|
| [logic-graph.md](tools-editor/logic-graph.md) | Logic Graph | 64 |
| [ai-assistant.md](tools-editor/ai-assistant.md) | AI Assistant | 56 |
| [remote-editing.md](tools-editor/remote-editing.md) | Remote Editing | 56 |
| [editor-framework.md](tools-editor/editor-framework.md) | Editor Framework | 40 |
| [server-infrastructure.md](tools-editor/server-infrastructure.md) | Server Infrastructure | 40 |
| [ai-governance.md](tools-editor/ai-governance.md) | AI Governance | 32 |
| [asset-store.md](tools-editor/asset-store.md) | Asset Store | 32 |
| [deployment.md](tools-editor/deployment.md) | Deployment | 32 |
| [shared-cache.md](tools-editor/shared-cache.md) | Shared Cache | 32 |
| [version-control.md](tools-editor/version-control.md) | Version Control | 32 |
| [world-building.md](tools-editor/world-building.md) | World Building | 32 |
| [animation-editor.md](tools-editor/animation-editor.md) | Animation Editor | 28 |
| [documentation.md](tools-editor/documentation.md) | Documentation | 28 |
| [level-editor.md](tools-editor/level-editor.md) | Level Editor | 28 |
| [profiling-tools.md](tools-editor/profiling-tools.md) | Profiling Tools | 28 |
| [launcher.md](tools-editor/launcher.md) | Launcher | 24 |
| [material-editor.md](tools-editor/material-editor.md) | Material Editor | 24 |
| [mod-support.md](tools-editor/mod-support.md) | Mod Support | 24 |
| [localization-editor.md](tools-editor/localization-editor.md) | Localization Editor | 12 |
| [cloud-build.md](tools-editor/cloud-build.md) | Cloud Build Service | 22 |
