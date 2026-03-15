# Harmonius Game Engine -- User Stories

Exhaustive user stories for the Harmonius game engine covering every feature from multiple
persona perspectives. Each story represents a single action by a single persona in a single
use case of a single feature.

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
| 15 | [Tools & Editor](tools-editor/) | 19 | 644 |
| | **Total** | **139** | **5,859** |

## 1. Core Runtime

| File | Area | Stories |
|------|------|---------|
| [entity-component-system.md](core-runtime/entity-component-system.md) | Entity Component System | 53 |
| [async-io.md](core-runtime/async-io.md) | Async I/O | 20 |
| [memory-management.md](core-runtime/memory-management.md) | Memory Management | 18 |
| [serialization.md](core-runtime/serialization.md) | Serialization | 17 |
| [events-and-messaging.md](core-runtime/events-and-messaging.md) | Events & Messaging | 16 |
| [spatial-indexing.md](core-runtime/spatial-indexing.md) | Spatial Indexing | 16 |
| [plugin-system.md](core-runtime/plugin-system.md) | Plugin System | 14 |
| [reflection-and-type-system.md](core-runtime/reflection-and-type-system.md) | Reflection & Type System | 14 |
| [scene-and-transforms.md](core-runtime/scene-and-transforms.md) | Scene & Transforms | 13 |

## 2. Rendering

| File | Area | Stories |
|------|------|---------|
| [lighting.md](rendering/lighting.md) | Lighting | 46 |
| [post-processing.md](rendering/post-processing.md) | Post-Processing | 42 |
| [advanced-rendering.md](rendering/advanced-rendering.md) | Advanced Rendering | 33 |
| [core-rendering.md](rendering/core-rendering.md) | Core Rendering | 30 |
| [render-graph.md](rendering/render-graph.md) | Render Graph | 26 |
| [advanced-materials.md](rendering/advanced-materials.md) | Advanced Materials | 25 |
| [gpu-abstraction-layer.md](rendering/gpu-abstraction-layer.md) | GPU Abstraction Layer | 25 |
| [environment.md](rendering/environment.md) | Environment | 23 |
| [scene-rendering-pipeline.md](rendering/scene-rendering-pipeline.md) | Scene Rendering Pipeline | 21 |
| [anti-aliasing-upscaling.md](rendering/anti-aliasing-upscaling.md) | Anti-Aliasing & Upscaling | 19 |
| [character-rendering.md](rendering/character-rendering.md) | Character Rendering | 19 |
| [stylized-effects.md](rendering/stylized-effects.md) | Stylized Effects | 15 |

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

| File | Area | Stories |
|------|------|---------|
| [input-actions-and-mapping.md](input/input-actions-and-mapping.md) | Input Actions & Mapping | 132 |
| [device-abstraction.md](input/device-abstraction.md) | Device Abstraction | 60 |
| [gestures.md](input/gestures.md) | Gestures | 60 |
| [haptics-and-feedback.md](input/haptics-and-feedback.md) | Haptics & Feedback | 60 |
| [vr-input.md](input/vr-input.md) | VR Input | 60 |

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

| File | Area | Stories |
|------|------|---------|
| [camera-system.md](game-framework/camera-system.md) | Camera System | 125 |
| [building-survival.md](game-framework/building-survival.md) | Building & Survival | 96 |
| [social.md](game-framework/social.md) | Social | 94 |
| [block-voxel.md](game-framework/block-voxel.md) | Block & Voxel | 84 |
| [monetization.md](game-framework/monetization.md) | Monetization | 80 |
| [progression.md](game-framework/progression.md) | Progression | 78 |
| [traversal-interaction.md](game-framework/traversal-interaction.md) | Traversal & Interaction | 77 |
| [npc-simulation.md](game-framework/npc-simulation.md) | NPC Simulation | 75 |
| [weapon-system.md](game-framework/weapon-system.md) | Weapon System | 74 |
| [game-modes-misc.md](game-framework/game-modes-misc.md) | Game Modes & Misc | 67 |
| [gameplay-primitives.md](game-framework/gameplay-primitives.md) | Gameplay Primitives | 62 |
| [selection-system.md](game-framework/selection-system.md) | Selection System | 55 |
| [pets-mounts.md](game-framework/pets-mounts.md) | Pets & Mounts | 53 |
| [minigames.md](game-framework/minigames.md) | Minigames | 52 |
| [character-customization.md](game-framework/character-customization.md) | Character Customization | 49 |
| [gameplay-databases.md](game-framework/gameplay-databases.md) | Gameplay Databases | 42 |
| [quest-dialogue.md](game-framework/quest-dialogue.md) | Quest & Dialogue | 41 |
| [inventory.md](game-framework/inventory.md) | Inventory | 38 |
| [stealth-cover.md](game-framework/stealth-cover.md) | Stealth & Cover | 35 |
| [racing.md](game-framework/racing.md) | Racing | 33 |
| [turn-based.md](game-framework/turn-based.md) | Turn-Based | 33 |
| [world-management.md](game-framework/world-management.md) | World Management | 32 |
| [abilities.md](game-framework/abilities.md) | Abilities | 30 |
| [cinematics.md](game-framework/cinematics.md) | Cinematics | 30 |
| [advertising.md](game-framework/advertising.md) | Advertising | 25 |
| [save-system.md](game-framework/save-system.md) | Save System | 24 |
| [fog-of-war.md](game-framework/fog-of-war.md) | Fog of War | 23 |
| [scripting.md](game-framework/scripting.md) | Scripting | 17 |

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
