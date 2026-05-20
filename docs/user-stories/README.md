# Harmonius Game Engine -- User Stories

User stories for the Harmonius game engine covering every feature from multiple persona
perspectives. Each story uses the `US-X.Y.Z` ID format where X is the domain, Y is the group, and Z
is the story number.

See [personas.md](personas.md) for the complete persona catalog.

## Summary

| # | Domain | Files | Stories |
|---|--------|------:|-------:|
| 1 | Core Runtime | 11 | 198 |
| 2 | Rendering | 13 | 277 |
| 3 | Geometry & World | 6 | 196 |
| 4 | Physics | 8 | 141 |
| 5 | Audio | 5 | 167 |
| 6 | Input | 5 | 93 |
| 7 | AI | 9 | 230 |
| 8 | Networking | 10 | 159 |
| 9 | Animation | 7 | 104 |
| 10 | UI & 2D | 6 | 76 |
| 11 | VFX | 6 | 119 |
| 12 | Content Pipeline | 7 | 109 |
| 13 | Game Framework | 27 | 664 |
| 14 | Platform | 7 | 175 |
| 15 | Tools & Editor | 24 | 411 |
| 16 | Data Systems | 4 | 45 |
| 17 | Simulation | 4 | 42 |
| | **Total** | **159** | **3,206** |

## 1. Core Runtime

| File | Stories |
|------|-------:|
| [entity-component-system.md](core-runtime/entity-component-system.md) | 45 |
| [serialization.md](core-runtime/serialization.md) | 22 |
| [async-io.md](core-runtime/async-io.md) | 20 |
| [memory-management.md](core-runtime/memory-management.md) | 18 |
| [reflection-and-type-system.md](core-runtime/reflection-and-type-system.md) | 18 |
| [events-and-messaging.md](core-runtime/events-and-messaging.md) | 15 |
| [spatial-indexing.md](core-runtime/spatial-indexing.md) | 15 |
| [plugin-system.md](core-runtime/plugin-system.md) | 13 |
| [scene-and-transforms.md](core-runtime/scene-and-transforms.md) | 12 |
| [game-loop.md](core-runtime/game-loop.md) | 10 |
| [algorithms.md](core-runtime/algorithms.md) | 10 |

## 2. Rendering

| File | Stories |
|------|-------:|
| [lighting.md](rendering/lighting.md) | 40 |
| [gpu-abstraction-layer.md](rendering/gpu-abstraction-layer.md) | 29 |
| [advanced-rendering.md](rendering/advanced-rendering.md) | 26 |
| [render-graph.md](rendering/render-graph.md) | 24 |
| [core-rendering.md](rendering/core-rendering.md) | 23 |
| [post-processing.md](rendering/post-processing.md) | 23 |
| [advanced-materials.md](rendering/advanced-materials.md) | 19 |
| [scene-rendering-pipeline.md](rendering/scene-rendering-pipeline.md) | 19 |
| [environment.md](rendering/environment.md) | 18 |
| [character-rendering.md](rendering/character-rendering.md) | 16 |
| [first-person-rendering.md](rendering/first-person-rendering.md) | 16 |
| [anti-aliasing-upscaling.md](rendering/anti-aliasing-upscaling.md) | 13 |
| [stylized-effects.md](rendering/stylized-effects.md) | 11 |

## 3. Geometry & World

| File | Stories |
|------|-------:|
| [procedural-generation.md](geometry/procedural-generation.md) | 66 |
| [terrain.md](geometry/terrain.md) | 33 |
| [foliage.md](geometry/foliage.md) | 29 |
| [meshlet-pipeline.md](geometry/meshlet-pipeline.md) | 24 |
| [sky-atmosphere.md](geometry/sky-atmosphere.md) | 22 |
| [water.md](geometry/water.md) | 22 |

## 4. Physics

| File | Stories |
|------|-------:|
| [collision-detection.md](physics/collision-detection.md) | 25 |
| [rigid-body-dynamics.md](physics/rigid-body-dynamics.md) | 21 |
| [constraints-and-joints.md](physics/constraints-and-joints.md) | 19 |
| [destruction-and-fracture.md](physics/destruction-and-fracture.md) | 19 |
| [vehicle-physics.md](physics/vehicle-physics.md) | 15 |
| [fluid-simulation.md](physics/fluid-simulation.md) | 14 |
| [soft-body-and-cloth.md](physics/soft-body-and-cloth.md) | 14 |
| [spatial-queries.md](physics/spatial-queries.md) | 14 |

## 5. Audio

| File | Stories |
|------|-------:|
| [voice-and-speech.md](audio/voice-and-speech.md) | 39 |
| [audio-engine.md](audio/audio-engine.md) | 35 |
| [dsp-and-effects.md](audio/dsp-and-effects.md) | 32 |
| [music-system.md](audio/music-system.md) | 32 |
| [spatial-audio.md](audio/spatial-audio.md) | 29 |

## 6. Input

| File | Stories |
|------|-------:|
| [input-actions-and-mapping.md](input/input-actions-and-mapping.md) | 31 |
| [device-abstraction.md](input/device-abstraction.md) | 17 |
| [gestures.md](input/gestures.md) | 15 |
| [haptics-and-feedback.md](input/haptics-and-feedback.md) | 15 |
| [vr-input.md](input/vr-input.md) | 15 |

## 7. AI

| File | Stories |
|------|-------:|
| [behavior-trees.md](ai/behavior-trees.md) | 34 |
| [non-functional.md](ai/non-functional.md) | 30 |
| [navigation.md](ai/navigation.md) | 30 |
| [perception.md](ai/perception.md) | 26 |
| [crowd-simulation.md](ai/crowd-simulation.md) | 24 |
| [goap.md](ai/goap.md) | 23 |
| [steering-avoidance.md](ai/steering-avoidance.md) | 22 |
| [tactical-combat.md](ai/tactical-combat.md) | 22 |
| [utility-ai.md](ai/utility-ai.md) | 19 |

## 8. Networking

| File | Stories |
|------|-------:|
| [communication.md](networking/communication.md) | 29 |
| [non-functional.md](networking/non-functional.md) | 24 |
| [session-management.md](networking/session-management.md) | 18 |
| [transport-layer.md](networking/transport-layer.md) | 16 |
| [mmo-infrastructure.md](networking/mmo-infrastructure.md) | 15 |
| [anti-cheat.md](networking/anti-cheat.md) | 12 |
| [prediction-and-rollback.md](networking/prediction-and-rollback.md) | 12 |
| [replay-system.md](networking/replay-system.md) | 12 |
| [state-replication.md](networking/state-replication.md) | 11 |
| [remote-procedure-calls.md](networking/remote-procedure-calls.md) | 10 |

## 9. Animation

| File | Stories |
|------|-------:|
| [skeletal.md](animation/skeletal.md) | 20 |
| [state-machine.md](animation/state-machine.md) | 18 |
| [motion-matching.md](animation/motion-matching.md) | 17 |
| [procedural.md](animation/procedural.md) | 17 |
| [cloth-hair.md](animation/cloth-hair.md) | 13 |
| [morph.md](animation/morph.md) | 11 |
| [first-person.md](animation/first-person.md) | 8 |

## 10. UI & 2D

| File | Stories |
|------|-------:|
| [hud-and-game-ui.md](ui/hud-and-game-ui.md) | 18 |
| [2d-games.md](ui/2d-games.md) | 17 |
| [widget-framework.md](ui/widget-framework.md) | 14 |
| [accessibility.md](ui/accessibility.md) | 10 |
| [common-widgets.md](ui/common-widgets.md) | 10 |
| [ui-rendering.md](ui/ui-rendering.md) | 7 |

## 11. VFX

| File | Stories |
|------|-------:|
| [destruction.md](vfx/destruction.md) | 28 |
| [particles.md](vfx/particles.md) | 21 |
| [weather-environmental.md](vfx/weather-environmental.md) | 21 |
| [screen-effects.md](vfx/screen-effects.md) | 18 |
| [decals.md](vfx/decals.md) | 16 |
| [effect-graph.md](vfx/effect-graph.md) | 15 |

## 12. Content Pipeline

| File | Stories |
|------|-------:|
| [content-plugins.md](content-pipeline/content-plugins.md) | 27 |
| [asset-processing.md](content-pipeline/asset-processing.md) | 17 |
| [asset-database.md](content-pipeline/asset-database.md) | 15 |
| [streaming-io.md](content-pipeline/streaming-io.md) | 15 |
| [asset-import.md](content-pipeline/asset-import.md) | 12 |
| [asset-versioning.md](content-pipeline/asset-versioning.md) | 12 |
| [hot-reload.md](content-pipeline/hot-reload.md) | 11 |

## 13. Game Framework

| File | Stories |
|------|-------:|
| [camera-system.md](game-framework/camera-system.md) | 54 |
| [abilities.md](game-framework/abilities.md) | 44 |
| [block-voxel.md](game-framework/block-voxel.md) | 36 |
| [traversal-interaction.md](game-framework/traversal-interaction.md) | 35 |
| [building-survival.md](game-framework/building-survival.md) | 33 |
| [game-modes-misc.md](game-framework/game-modes-misc.md) | 31 |
| [character-customization.md](game-framework/character-customization.md) | 30 |
| [monetization.md](game-framework/monetization.md) | 30 |
| [weapon-system.md](game-framework/weapon-system.md) | 28 |
| [gameplay-primitives.md](game-framework/gameplay-primitives.md) | 26 |
| [minigames.md](game-framework/minigames.md) | 26 |
| [progression.md](game-framework/progression.md) | 26 |
| [social.md](game-framework/social.md) | 25 |
| [gameplay-databases.md](game-framework/gameplay-databases.md) | 23 |
| [npc-simulation.md](game-framework/npc-simulation.md) | 22 |
| [cinematics.md](game-framework/cinematics.md) | 20 |
| [selection-system.md](game-framework/selection-system.md) | 20 |
| [inventory.md](game-framework/inventory.md) | 19 |
| [pets-mounts.md](game-framework/pets-mounts.md) | 19 |
| [quest-dialogue.md](game-framework/quest-dialogue.md) | 17 |
| [racing.md](game-framework/racing.md) | 17 |
| [stealth-cover.md](game-framework/stealth-cover.md) | 17 |
| [turn-based.md](game-framework/turn-based.md) | 16 |
| [world-management.md](game-framework/world-management.md) | 16 |
| [fog-of-war.md](game-framework/fog-of-war.md) | 15 |
| [save-system.md](game-framework/save-system.md) | 11 |
| [scripting.md](game-framework/scripting.md) | 8 |

## 14. Platform

| File | Stories |
|------|-------:|
| [sdk-integration.md](platform/sdk-integration.md) | 50 |
| [threading-async.md](platform/threading-async.md) | 26 |
| [platform-services.md](platform/platform-services.md) | 25 |
| [window-display.md](platform/window-display.md) | 23 |
| [crash-reporting.md](platform/crash-reporting.md) | 17 |
| [filesystem.md](platform/filesystem.md) | 17 |
| [os-integration.md](platform/os-integration.md) | 17 |

## 15. Tools & Editor

| File | Stories |
|------|-------:|
| [logic-graph.md](tools/logic-graph.md) | 29 |
| [remote-editing.md](tools/remote-editing.md) | 28 |
| [specialized-editors.md](tools/specialized-editors.md) | 24 |
| [world-building.md](tools/world-building.md) | 24 |
| [ai-assistant.md](tools/ai-assistant.md) | 22 |
| [asset-store.md](tools/asset-store.md) | 20 |
| [editor-plugins.md](tools/editor-plugins.md) | 20 |
| [server-infrastructure.md](tools/server-infrastructure.md) | 20 |
| [deployment.md](tools/deployment.md) | 18 |
| [editor-framework.md](tools/editor-framework.md) | 18 |
| [ai-cloud-backend.md](tools/ai-cloud-backend.md) | 16 |
| [ai-governance.md](tools/ai-governance.md) | 16 |
| [shared-cache.md](tools/shared-cache.md) | 16 |
| [version-control.md](tools/version-control.md) | 16 |
| [animation-editor.md](tools/animation-editor.md) | 14 |
| [cloud-build.md](tools/cloud-build.md) | 14 |
| [documentation.md](tools/documentation.md) | 14 |
| [level-editor.md](tools/level-editor.md) | 14 |
| [profiling-tools.md](tools/profiling-tools.md) | 14 |
| [launcher.md](tools/launcher.md) | 12 |
| [material-editor.md](tools/material-editor.md) | 12 |
| [mod-support.md](tools/mod-support.md) | 12 |
| [vr-editor.md](tools/vr-editor.md) | 12 |
| [localization-editor.md](tools/localization-editor.md) | 6 |

## 16. Data Systems

| File | Stories |
|------|-------:|
| [attributes-effects.md](data-systems/attributes-effects.md) | 12 |
| [data-tables.md](data-systems/data-tables.md) | 12 |
| [directed-graphs.md](data-systems/directed-graphs.md) | 11 |
| [containers-slots.md](data-systems/containers-slots.md) | 10 |

## 17. Simulation

| File | Stories |
|------|-------:|
| [grids-volumes.md](simulation/grids-volumes.md) | 12 |
| [timelines.md](simulation/timelines.md) | 12 |
| [spatial-awareness.md](simulation/spatial-awareness.md) | 10 |
| [event-logs.md](simulation/event-logs.md) | 8 |
