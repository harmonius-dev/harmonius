# Harmonius Game Engine -- Requirements

Functional and non-functional requirements for all engine subsystems. Each requirement uses the
`R-X.Y.Z` ID format where X is the domain, Y is the group, and Z is the requirement number.

## Summary

| # | Domain | Files | Requirements |
|---|--------|------:|-----------:|
| 1 | Core Runtime | 9 | 122 |
| 2 | Rendering | 14 | 131 |
| 3 | Geometry & World | 6 | 55 |
| 4 | Physics | 8 | 65 |
| 5 | Audio | 5 | 58 |
| 6 | Input | 5 | 75 |
| 7 | AI | 8 | 67 |
| 8 | Networking | 9 | 84 |
| 9 | Animation | 7 | 56 |
| 10 | UI & 2D | 6 | 75 |
| 11 | VFX | 6 | 42 |
| 12 | Content Pipeline | 8 | 87 |
| 13 | Game Framework | 27 | 236 |
| 14 | Platform | 7 | 130 |
| 15 | Tools & Editor | 24 | 204 |
| X | Cross-Cutting | 1 | 33 |
| | **Total** | **150** | **1,520** |

## 1. Core Runtime

| File | Requirements |
|------|-----------:|
| [entity-component-system.md](core-runtime/entity-component-system.md) | 39 |
| [async-io.md](core-runtime/async-io.md) | 14 |
| [serialization.md](core-runtime/serialization.md) | 12 |
| [reflection-and-type-system.md](core-runtime/reflection-and-type-system.md) | 11 |
| [spatial-indexing.md](core-runtime/spatial-indexing.md) | 11 |
| [memory-management.md](core-runtime/memory-management.md) | 10 |
| [events-and-messaging.md](core-runtime/events-and-messaging.md) | 9 |
| [plugin-system.md](core-runtime/plugin-system.md) | 8 |
| [scene-and-transforms.md](core-runtime/scene-and-transforms.md) | 8 |

## 2. Rendering

| File | Requirements |
|------|-----------:|
| [lighting.md](rendering/lighting.md) | 17 |
| [gpu-abstraction-layer.md](rendering/gpu-abstraction-layer.md) | 15 |
| [core-rendering.md](rendering/core-rendering.md) | 13 |
| [render-graph.md](rendering/render-graph.md) | 13 |
| [post-processing.md](rendering/post-processing.md) | 11 |
| [advanced-rendering.md](rendering/advanced-rendering.md) | 10 |
| [advanced-materials.md](rendering/advanced-materials.md) | 9 |
| [anti-aliasing-upscaling.md](rendering/anti-aliasing-upscaling.md) | 8 |
| [character-rendering.md](rendering/character-rendering.md) | 8 |
| [environment.md](rendering/environment.md) | 8 |
| [first-person-rendering.md](rendering/first-person-rendering.md) | 8 |
| [scene-rendering-pipeline.md](rendering/scene-rendering-pipeline.md) | 6 |
| [stylized-effects.md](rendering/stylized-effects.md) | 5 |
| [gpu-abstraction.md](rendering/gpu-abstraction.md) | 0 |

## 3. Geometry & World

| File | Requirements |
|------|-----------:|
| [terrain.md](geometry-world/terrain.md) | 14 |
| [procedural-generation.md](geometry-world/procedural-generation.md) | 13 |
| [foliage.md](geometry-world/foliage.md) | 7 |
| [meshlet-pipeline.md](geometry-world/meshlet-pipeline.md) | 7 |
| [sky-atmosphere.md](geometry-world/sky-atmosphere.md) | 7 |
| [water.md](geometry-world/water.md) | 7 |

## 4. Physics

| File | Requirements |
|------|-----------:|
| [destruction-and-fracture.md](physics/destruction-and-fracture.md) | 11 |
| [rigid-body-dynamics.md](physics/rigid-body-dynamics.md) | 10 |
| [collision-detection.md](physics/collision-detection.md) | 9 |
| [constraints-and-joints.md](physics/constraints-and-joints.md) | 8 |
| [fluid-simulation.md](physics/fluid-simulation.md) | 7 |
| [soft-body-and-cloth.md](physics/soft-body-and-cloth.md) | 7 |
| [vehicle-physics.md](physics/vehicle-physics.md) | 7 |
| [spatial-queries.md](physics/spatial-queries.md) | 6 |

## 5. Audio

| File | Requirements |
|------|-----------:|
| [audio-engine.md](audio/audio-engine.md) | 15 |
| [spatial-audio.md](audio/spatial-audio.md) | 13 |
| [voice-and-speech.md](audio/voice-and-speech.md) | 12 |
| [dsp-and-effects.md](audio/dsp-and-effects.md) | 10 |
| [music-system.md](audio/music-system.md) | 8 |

## 6. Input

| File | Requirements |
|------|-----------:|
| [input-actions-and-mapping.md](input/input-actions-and-mapping.md) | 25 |
| [device-abstraction.md](input/device-abstraction.md) | 15 |
| [vr-input.md](input/vr-input.md) | 15 |
| [gestures.md](input/gestures.md) | 10 |
| [haptics-and-feedback.md](input/haptics-and-feedback.md) | 10 |

## 7. AI

| File | Requirements |
|------|-----------:|
| [navigation.md](ai/navigation.md) | 15 |
| [crowd-simulation.md](ai/crowd-simulation.md) | 11 |
| [perception.md](ai/perception.md) | 11 |
| [behavior-trees.md](ai/behavior-trees.md) | 7 |
| [goap.md](ai/goap.md) | 6 |
| [steering-avoidance.md](ai/steering-avoidance.md) | 6 |
| [tactical-combat.md](ai/tactical-combat.md) | 6 |
| [utility-ai.md](ai/utility-ai.md) | 5 |

## 8. Networking

| File | Requirements |
|------|-----------:|
| [communication.md](networking/communication.md) | 18 |
| [session-management.md](networking/session-management.md) | 13 |
| [mmo-infrastructure.md](networking/mmo-infrastructure.md) | 11 |
| [transport-layer.md](networking/transport-layer.md) | 10 |
| [state-replication.md](networking/state-replication.md) | 8 |
| [prediction-and-rollback.md](networking/prediction-and-rollback.md) | 7 |
| [anti-cheat.md](networking/anti-cheat.md) | 6 |
| [remote-procedure-calls.md](networking/remote-procedure-calls.md) | 6 |
| [replay-system.md](networking/replay-system.md) | 5 |

## 9. Animation

| File | Requirements |
|------|-----------:|
| [motion-matching.md](animation/motion-matching.md) | 11 |
| [procedural.md](animation/procedural.md) | 10 |
| [skeletal.md](animation/skeletal.md) | 10 |
| [state-machine.md](animation/state-machine.md) | 10 |
| [cloth-hair.md](animation/cloth-hair.md) | 6 |
| [morph.md](animation/morph.md) | 5 |
| [first-person.md](animation/first-person.md) | 4 |

## 10. UI & 2D

| File | Requirements |
|------|-----------:|
| [2d-games.md](ui-2d/2d-games.md) | 24 |
| [hud-and-game-ui.md](ui-2d/hud-and-game-ui.md) | 15 |
| [widget-framework.md](ui-2d/widget-framework.md) | 14 |
| [common-widgets.md](ui-2d/common-widgets.md) | 8 |
| [accessibility.md](ui-2d/accessibility.md) | 7 |
| [ui-rendering.md](ui-2d/ui-rendering.md) | 7 |

## 11. VFX

| File | Requirements |
|------|-----------:|
| [destruction.md](vfx/destruction.md) | 11 |
| [particles.md](vfx/particles.md) | 7 |
| [weather-environmental.md](vfx/weather-environmental.md) | 7 |
| [decals.md](vfx/decals.md) | 6 |
| [screen-effects.md](vfx/screen-effects.md) | 6 |
| [effect-graph.md](vfx/effect-graph.md) | 5 |

## 12. Content Pipeline

| File | Requirements |
|------|-----------:|
| [dcc-plugins.md](content-pipeline/dcc-plugins.md) | 26 |
| [content-plugins.md](content-pipeline/content-plugins.md) | 12 |
| [asset-database.md](content-pipeline/asset-database.md) | 10 |
| [streaming-io.md](content-pipeline/streaming-io.md) | 10 |
| [asset-processing.md](content-pipeline/asset-processing.md) | 9 |
| [asset-versioning.md](content-pipeline/asset-versioning.md) | 8 |
| [hot-reload.md](content-pipeline/hot-reload.md) | 7 |
| [asset-import.md](content-pipeline/asset-import.md) | 5 |

## 13. Game Framework

| File | Requirements |
|------|-----------:|
| [traversal-interaction.md](game-framework/traversal-interaction.md) | 17 |
| [abilities.md](game-framework/abilities.md) | 15 |
| [character-customization.md](game-framework/character-customization.md) | 15 |
| [camera-system.md](game-framework/camera-system.md) | 12 |
| [monetization.md](game-framework/monetization.md) | 12 |
| [npc-simulation.md](game-framework/npc-simulation.md) | 11 |
| [gameplay-primitives.md](game-framework/gameplay-primitives.md) | 10 |
| [inventory.md](game-framework/inventory.md) | 10 |
| [progression.md](game-framework/progression.md) | 10 |
| [social.md](game-framework/social.md) | 10 |
| [weapon-system.md](game-framework/weapon-system.md) | 10 |
| [building-survival.md](game-framework/building-survival.md) | 9 |
| [gameplay-databases.md](game-framework/gameplay-databases.md) | 9 |
| [block-voxel.md](game-framework/block-voxel.md) | 8 |
| [game-modes-misc.md](game-framework/game-modes-misc.md) | 8 |
| [minigames.md](game-framework/minigames.md) | 8 |
| [selection-system.md](game-framework/selection-system.md) | 8 |
| [cinematics.md](game-framework/cinematics.md) | 7 |
| [quest-dialogue.md](game-framework/quest-dialogue.md) | 7 |
| [world-management.md](game-framework/world-management.md) | 7 |
| [save-system.md](game-framework/save-system.md) | 6 |
| [pets-mounts.md](game-framework/pets-mounts.md) | 5 |
| [racing.md](game-framework/racing.md) | 5 |
| [stealth-cover.md](game-framework/stealth-cover.md) | 5 |
| [turn-based.md](game-framework/turn-based.md) | 5 |
| [fog-of-war.md](game-framework/fog-of-war.md) | 4 |
| [scripting.md](game-framework/scripting.md) | 3 |

## 14. Platform

| File | Requirements |
|------|-----------:|
| [sdk-integration.md](platform/sdk-integration.md) | 60 |
| [threading-async.md](platform/threading-async.md) | 16 |
| [platform-services.md](platform/platform-services.md) | 12 |
| [crash-reporting.md](platform/crash-reporting.md) | 11 |
| [filesystem.md](platform/filesystem.md) | 11 |
| [window-display.md](platform/window-display.md) | 11 |
| [os-integration.md](platform/os-integration.md) | 9 |

## 15. Tools & Editor

| File | Requirements |
|------|-----------:|
| [logic-graph.md](tools-editor/logic-graph.md) | 14 |
| [remote-editing.md](tools-editor/remote-editing.md) | 14 |
| [specialized-editors.md](tools-editor/specialized-editors.md) | 12 |
| [world-building.md](tools-editor/world-building.md) | 12 |
| [ai-assistant.md](tools-editor/ai-assistant.md) | 10 |
| [asset-store.md](tools-editor/asset-store.md) | 10 |
| [editor-plugins.md](tools-editor/editor-plugins.md) | 10 |
| [server-infrastructure.md](tools-editor/server-infrastructure.md) | 10 |
| [deployment.md](tools-editor/deployment.md) | 9 |
| [editor-framework.md](tools-editor/editor-framework.md) | 9 |
| [ai-cloud-backend.md](tools-editor/ai-cloud-backend.md) | 8 |
| [ai-governance.md](tools-editor/ai-governance.md) | 8 |
| [shared-cache.md](tools-editor/shared-cache.md) | 8 |
| [version-control.md](tools-editor/version-control.md) | 8 |
| [animation-editor.md](tools-editor/animation-editor.md) | 7 |
| [cloud-build.md](tools-editor/cloud-build.md) | 7 |
| [documentation.md](tools-editor/documentation.md) | 7 |
| [level-editor.md](tools-editor/level-editor.md) | 7 |
| [profiling-tools.md](tools-editor/profiling-tools.md) | 7 |
| [launcher.md](tools-editor/launcher.md) | 6 |
| [material-editor.md](tools-editor/material-editor.md) | 6 |
| [mod-support.md](tools-editor/mod-support.md) | 6 |
| [vr-editor.md](tools-editor/vr-editor.md) | 6 |
| [localization-editor.md](tools-editor/localization-editor.md) | 3 |

## X. Cross-Cutting

| File | Requirements |
|------|-----------:|
| [cross-cutting.md](cross-cutting.md) | 33 |
