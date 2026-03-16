# Harmonius Plan

Work plan for the Harmonius game engine. Maps every feature-group from the
[domain decomposition](domain-decomposition.md) to its requirements, features, design status, and
implementation status. Organized by wave to show what can proceed in parallel at each stage.

**Current state:** Requirements and features are fully specified (1,501 requirements, 1,204 features
across 15 domains). Build system is configured (C++26, CMake, vcpkg, Metal/Vulkan/D3D12 backend
selection). No source code exists. No designs exist.

---

## Wave 0 — Platform Root

No prerequisites. These are the first feature-groups to design and implement.

| # | Feature-Group | Requirements | Features | Design | Implementation |
|---|---------------|-------------|----------|--------|----------------|
| 1 | Platform.Windowing | [window-display](../requirements/platform/window-display.md) | [window-display](../features/platform/window-display.md) | Not started | Not started |
| 2 | Platform.Threading | [threading-async](../requirements/platform/threading-async.md) | [threading-async](../features/platform/threading-async.md) | Not started | Not started |
| 3 | Platform.OS | [os-integration](../requirements/platform/os-integration.md), [crash-reporting](../requirements/platform/crash-reporting.md), [platform-services](../requirements/platform/platform-services.md), [filesystem](../requirements/platform/filesystem.md) | [os-integration](../features/platform/os-integration.md), [crash-reporting](../features/platform/crash-reporting.md), [platform-services](../features/platform/platform-services.md), [filesystem](../features/platform/filesystem.md) | Not started | Not started |

**Exit criteria:** Windows open on macOS/Linux/Windows, thread pool executes tasks, async file I/O
completes, crash handler captures dumps.

---

## Wave 1 — Core Runtime

Requires Wave 0. CoreRuntime.ECS should lead; other groups depend on its primitives.

| # | Feature-Group | Requirements | Features | Design | Implementation |
|---|---------------|-------------|----------|--------|----------------|
| 4 | CoreRuntime.ECS | [entity-component-system](../requirements/core-runtime/entity-component-system.md) | [entity-component-system](../features/core-runtime/entity-component-system.md) | Not started | Not started |
| 5 | CoreRuntime.SceneTransforms | [scene-and-transforms](../requirements/core-runtime/scene-and-transforms.md) | [scene-and-transforms](../features/core-runtime/scene-and-transforms.md) | Not started | Not started |
| 6 | CoreRuntime.ReflectionSerialization | [reflection-and-type-system](../requirements/core-runtime/reflection-and-type-system.md), [serialization](../requirements/core-runtime/serialization.md) | [reflection-and-type-system](../features/core-runtime/reflection-and-type-system.md), [serialization](../features/core-runtime/serialization.md) | Not started | Not started |
| 7 | CoreRuntime.EventsPlugins | [events-and-messaging](../requirements/core-runtime/events-and-messaging.md), [plugin-system](../requirements/core-runtime/plugin-system.md) | [events-and-messaging](../features/core-runtime/events-and-messaging.md), [plugin-system](../features/core-runtime/plugin-system.md) | Not started | Not started |
| 8 | CoreRuntime.MemoryAsyncIO | [memory-management](../requirements/core-runtime/memory-management.md), [async-io](../requirements/core-runtime/async-io.md) | [memory-management](../features/core-runtime/memory-management.md), [async-io](../features/core-runtime/async-io.md) | Not started | Not started |
| 9 | CoreRuntime.SpatialIndex | [spatial-indexing](../requirements/core-runtime/spatial-indexing.md) | [spatial-indexing](../features/core-runtime/spatial-indexing.md) | Not started | Not started |

**Exit criteria:** Entities spawn, queries iterate, systems schedule in parallel, transforms
propagate, events fire, types reflect, binary serialization round-trips, arena allocators work,
async I/O completes, spatial index inserts/queries work.

**Interoperability contracts to lock before Wave 2:**

| Contract | Defined By | Scope |
|----------|-----------|-------|
| Component trait + archetype API | CoreRuntime.ECS | How to define, register, query, and mutate components |
| Entity handle type | CoreRuntime.ECS | Generational index format, validity checking |
| System trait + scheduling API | CoreRuntime.ECS | How to declare systems, dependencies, and access patterns |
| Event channel API | CoreRuntime.EventsPlugins | How to send and receive typed events |
| Spatial index query API | CoreRuntime.SpatialIndex | Ray-cast, shape-cast, overlap, frustum, k-nearest |
| Serialization trait | CoreRuntime.ReflectionSerialization | Binary/text encode/decode for components and assets |
| Async I/O trait | CoreRuntime.MemoryAsyncIO | Read/write/cancel operations, completion callbacks |
| Transform component schema | CoreRuntime.SceneTransforms | Position, rotation, scale, parent, world matrix |

---

## Wave 2 — Infrastructure Fan-Out

Requires specific Wave 1 nodes. 9 feature-groups can proceed concurrently.

| # | Feature-Group | Requirements | Features | Prereqs | Design | Impl |
|---|---------------|-------------|----------|---------|--------|------|
| 10 | Rendering.GPUAbstraction | [gpu-abstraction-layer](../requirements/rendering/gpu-abstraction-layer.md), [gpu-abstraction](../requirements/rendering/gpu-abstraction.md) | [gpu-abstraction-layer](../features/rendering/gpu-abstraction-layer.md) | Platform.Windowing, CoreRuntime.ECS | Not started | Not started |
| 11 | ContentPipeline.AssetImport | [asset-import](../requirements/content-pipeline/asset-import.md) | [asset-import](../features/content-pipeline/asset-import.md) | CoreRuntime.MemoryAsyncIO, Platform.Threading | Not started | Not started |
| 12 | ContentPipeline.AssetDatabase | [asset-database](../requirements/content-pipeline/asset-database.md) | [asset-database](../features/content-pipeline/asset-database.md) | CoreRuntime.ReflectionSerialization | Not started | Not started |
| 13 | Physics.RigidBody | [rigid-body-dynamics](../requirements/physics/rigid-body-dynamics.md), [collision-detection](../requirements/physics/collision-detection.md) | [rigid-body-dynamics](../features/physics/rigid-body-dynamics.md), [collision-detection](../features/physics/collision-detection.md) | CoreRuntime.ECS, CoreRuntime.SpatialIndex | Not started | Not started |
| 14 | Physics.SpatialQueries | [spatial-queries](../requirements/physics/spatial-queries.md) | [spatial-queries](../features/physics/spatial-queries.md) | CoreRuntime.SpatialIndex | Not started | Not started |
| 15 | Input.DeviceAbstraction | [device-abstraction](../requirements/input/device-abstraction.md) | [device-abstraction](../features/input/device-abstraction.md) | Platform.Windowing, CoreRuntime.ECS | Not started | Not started |
| 16 | Audio.Engine | [audio-engine](../requirements/audio/audio-engine.md) | [audio-engine](../features/audio/audio-engine.md) | Platform.OS, Platform.Threading, CoreRuntime.ECS | Not started | Not started |
| 17 | Animation.Skeletal | [skeletal](../requirements/animation/skeletal.md) | [skeletal](../features/animation/skeletal.md) | CoreRuntime.ECS, CoreRuntime.SceneTransforms | Not started | Not started |
| 18 | Networking.Transport | [transport-layer](../requirements/networking/transport-layer.md) | [transport-layer](../features/networking/transport-layer.md) | Platform.Threading, CoreRuntime.ECS | Not started | Not started |

**Additional contracts to lock before Wave 3:**

| Contract | Defined By | Scope |
|----------|-----------|-------|
| GPU backend trait | Rendering.GPUAbstraction | Device, command buffer, pipeline, buffer, texture APIs |
| Asset handle type | ContentPipeline.AssetDatabase | Typed handle, load state, reference counting |

**Exit criteria:** GPU device created and triangle renders, assets import from disk, rigid bodies
simulate, ray-casts work, keyboard/mouse/gamepad input captured, audio plays, skeleton evaluates and
skins, network connection established.

---

## Wave 3 — Intermediate Systems

Requires specific Wave 2 nodes. 12 feature-groups can proceed concurrently.

| # | Feature-Group | Requirements | Features | Prereqs | Design | Impl |
|---|---------------|-------------|----------|---------|--------|------|
| 19 | Rendering.RenderGraph | [render-graph](../requirements/rendering/render-graph.md) | [render-graph](../features/rendering/render-graph.md) | Rendering.GPUAbstraction | Not started | Not started |
| 20 | ContentPipeline.AssetProcessing | [asset-processing](../requirements/content-pipeline/asset-processing.md) | [asset-processing](../features/content-pipeline/asset-processing.md) | ContentPipeline.AssetImport, Rendering.GPUAbstraction | Not started | Not started |
| 21 | ContentPipeline.Streaming | [streaming-io](../requirements/content-pipeline/streaming-io.md) | [streaming-io](../features/content-pipeline/streaming-io.md) | ContentPipeline.AssetDatabase, CoreRuntime.MemoryAsyncIO | Not started | Not started |
| 22 | Input.ActionsMapping | [input-actions-and-mapping](../requirements/input/input-actions-and-mapping.md) | [input-actions-and-mapping](../features/input/input-actions-and-mapping.md) | Input.DeviceAbstraction | Not started | Not started |
| 23 | Audio.Spatial | [spatial-audio](../requirements/audio/spatial-audio.md) | [spatial-audio](../features/audio/spatial-audio.md) | Audio.Engine, CoreRuntime.SpatialIndex | Not started | Not started |
| 24 | Animation.StateMachine | [state-machine](../requirements/animation/state-machine.md) | [state-machine](../features/animation/state-machine.md) | Animation.Skeletal | Not started | Not started |
| 25 | Physics.Constraints | [constraints-and-joints](../requirements/physics/constraints-and-joints.md) | [constraints-and-joints](../features/physics/constraints-and-joints.md) | Physics.RigidBody | Not started | Not started |
| 26 | AI.Navigation | [navigation](../requirements/ai/navigation.md) | [navigation](../features/ai/navigation.md) | CoreRuntime.SpatialIndex, Physics.SpatialQueries | Not started | Not started |
| 27 | Networking.Replication | [state-replication](../requirements/networking/state-replication.md) | [state-replication](../features/networking/state-replication.md) | Networking.Transport, CoreRuntime.ReflectionSerialization, CoreRuntime.SpatialIndex | Not started | Not started |
| 28 | GeometryWorld.MeshletPipeline | [meshlet-pipeline](../requirements/geometry-world/meshlet-pipeline.md) | [meshlet-pipeline](../features/geometry-world/meshlet-pipeline.md) | Rendering.GPUAbstraction, CoreRuntime.ECS | Not started | Not started |
| 29 | UI2D.WidgetFramework | [widget-framework](../requirements/ui-2d/widget-framework.md) | [widget-framework](../features/ui-2d/widget-framework.md) | CoreRuntime.ECS, Input.DeviceAbstraction | Not started | Not started |
| 30 | VFX.Particles | [particles](../requirements/vfx/particles.md) | [particles](../features/vfx/particles.md) | Rendering.GPUAbstraction, CoreRuntime.ECS | Not started | Not started |

**Additional contract to lock before Wave 4:**

| Contract | Defined By | Scope |
|----------|-----------|-------|
| Render graph pass API | Rendering.RenderGraph | How to declare passes, read/write resources, submit commands |

**Exit criteria:** Render graph executes passes with automatic barriers, shaders compile via
DXC/MSC, assets stream from disk, input actions map to gameplay, spatial audio occludes, animation
state machines transition, joints constrain bodies, NavMesh generates and pathfinds, components
replicate over network, meshlets render, widget tree lays out, particles simulate on GPU.

---

## Wave 4 — Integration Layer

Requires specific Wave 3 nodes. 19 feature-groups can proceed concurrently.

| # | Feature-Group | Requirements | Features | Prereqs | Design | Impl |
|---|---------------|-------------|----------|---------|--------|------|
| 31 | Rendering.CoreRendering | [core-rendering](../requirements/rendering/core-rendering.md), [scene-rendering-pipeline](../requirements/rendering/scene-rendering-pipeline.md) | [core-rendering](../features/rendering/core-rendering.md), [scene-rendering-pipeline](../features/rendering/scene-rendering-pipeline.md) | Rendering.RenderGraph, CoreRuntime.SpatialIndex, Animation.Skeletal | Not started | Not started |
| 32 | Rendering.Lighting | [lighting](../requirements/rendering/lighting.md) | [lighting](../features/rendering/lighting.md) | Rendering.CoreRendering | Not started | Not started |
| 33 | ContentPipeline.HotReload | [hot-reload](../requirements/content-pipeline/hot-reload.md) | [hot-reload](../features/content-pipeline/hot-reload.md) | ContentPipeline.AssetDatabase, ContentPipeline.AssetProcessing | Not started | Not started |
| 34 | AI.Steering | [steering-avoidance](../requirements/ai/steering-avoidance.md) | [steering-avoidance](../features/ai/steering-avoidance.md) | AI.Navigation | Not started | Not started |
| 35 | AI.Perception | [perception](../requirements/ai/perception.md) | [perception](../features/ai/perception.md) | CoreRuntime.SpatialIndex, AI.Navigation | Not started | Not started |
| 36 | AI.BehaviorDecision | [behavior-trees](../requirements/ai/behavior-trees.md), [utility-ai](../requirements/ai/utility-ai.md), [goap](../requirements/ai/goap.md) | [behavior-trees](../features/ai/behavior-trees.md), [utility-ai](../features/ai/utility-ai.md), [goap](../features/ai/goap.md) | AI.Perception | Not started | Not started |
| 37 | Animation.Procedural | [procedural](../requirements/animation/procedural.md) | [procedural](../features/animation/procedural.md) | Animation.Skeletal, Physics.RigidBody, Physics.Constraints | Not started | Not started |
| 38 | Audio.DSP | [dsp-and-effects](../requirements/audio/dsp-and-effects.md) | [dsp-and-effects](../features/audio/dsp-and-effects.md) | Audio.Engine | Not started | Not started |
| 39 | Networking.PredictionRollback | [prediction-and-rollback](../requirements/networking/prediction-and-rollback.md) | [prediction-and-rollback](../features/networking/prediction-and-rollback.md) | Networking.Replication, Physics.RigidBody | Not started | Not started |
| 40 | Networking.Session | [session-management](../requirements/networking/session-management.md) | [session-management](../features/networking/session-management.md) | Networking.Transport | Not started | Not started |
| 41 | GameFramework.GameplayPrimitives | [gameplay-primitives](../requirements/game-framework/gameplay-primitives.md) | [gameplay-primitives](../features/game-framework/gameplay-primitives.md) | CoreRuntime.ECS, CoreRuntime.SpatialIndex, Physics.RigidBody, Physics.SpatialQueries, Input.ActionsMapping | Not started | Not started |
| 42 | GameFramework.Camera | [camera-system](../requirements/game-framework/camera-system.md) | [camera-system](../features/game-framework/camera-system.md) | CoreRuntime.SceneTransforms, Physics.SpatialQueries | Not started | Not started |
| 43 | UI2D.UIRendering | [ui-rendering](../requirements/ui-2d/ui-rendering.md) | [ui-rendering](../features/ui-2d/ui-rendering.md) | UI2D.WidgetFramework, Rendering.RenderGraph, Rendering.GPUAbstraction | Not started | Not started |
| 44 | UI2D.GameUI | [hud-and-game-ui](../requirements/ui-2d/hud-and-game-ui.md) | [hud-and-game-ui](../features/ui-2d/hud-and-game-ui.md) | UI2D.WidgetFramework, UI2D.UIRendering | Not started | Not started |
| 45 | VFX.Decals | [decals](../requirements/vfx/decals.md) | [decals](../features/vfx/decals.md) | Rendering.RenderGraph | Not started | Not started |
| 46 | VFX.ScreenEffects | [screen-effects](../requirements/vfx/screen-effects.md) | [screen-effects](../features/vfx/screen-effects.md) | Rendering.CoreRendering | Not started | Not started |
| 47 | GeometryWorld.Terrain | [terrain](../requirements/geometry-world/terrain.md) | [terrain](../features/geometry-world/terrain.md) | Rendering.CoreRendering, ContentPipeline.Streaming, Physics.RigidBody | Not started | Not started |
| 48 | GeometryWorld.Foliage | [foliage](../requirements/geometry-world/foliage.md) | [foliage](../features/geometry-world/foliage.md) | Rendering.CoreRendering, CoreRuntime.SpatialIndex, ContentPipeline.Streaming | Not started | Not started |
| 49 | Input.Gestures | [gestures](../requirements/input/gestures.md) | [gestures](../features/input/gestures.md) | Input.DeviceAbstraction | Not started | Not started |

**Exit criteria:** Lit PBR scene renders with shadows, hot reload works for all asset types, AI
agents navigate and perceive, procedural animation (IK, ragdoll blend) works, audio has DSP chains,
client-side prediction works, gameplay primitives exist, camera follows player, UI renders in-game
HUD, decals and screen effects render, terrain and foliage render with streaming.

---

## Wave 5 — Advanced Systems

Requires specific Wave 4 nodes. 20 feature-groups can proceed concurrently.

| # | Feature-Group | Requirements | Features | Prereqs | Design | Impl |
|---|---------------|-------------|----------|---------|--------|------|
| 50 | Rendering.AdvancedRendering | [advanced-rendering](../requirements/rendering/advanced-rendering.md), [advanced-materials](../requirements/rendering/advanced-materials.md), [character-rendering](../requirements/rendering/character-rendering.md), [environment](../requirements/rendering/environment.md), [post-processing](../requirements/rendering/post-processing.md), [anti-aliasing-upscaling](../requirements/rendering/anti-aliasing-upscaling.md), [stylized-effects](../requirements/rendering/stylized-effects.md) | [advanced-rendering](../features/rendering/advanced-rendering.md), [advanced-materials](../features/rendering/advanced-materials.md), [character-rendering](../features/rendering/character-rendering.md), [environment](../features/rendering/environment.md), [post-processing](../features/rendering/post-processing.md), [anti-aliasing-upscaling](../features/rendering/anti-aliasing-upscaling.md), [stylized-effects](../features/rendering/stylized-effects.md) | Rendering.Lighting | Not started | Not started |
| 51 | ToolsEditor.EditorFramework | [editor-framework](../requirements/tools-editor/editor-framework.md) | [editor-framework](../features/tools-editor/editor-framework.md) | UI2D.WidgetFramework, CoreRuntime.ReflectionSerialization, ContentPipeline.HotReload, Rendering.CoreRendering, Input.ActionsMapping | Not started | Not started |
| 52 | ToolsEditor.LogicGraph | [logic-graph](../requirements/tools-editor/logic-graph.md) | [logic-graph](../features/tools-editor/logic-graph.md) | CoreRuntime.ECS, CoreRuntime.EventsPlugins, ContentPipeline.HotReload | Not started | Not started |
| 53 | ToolsEditor.MaterialEditor | [material-editor](../requirements/tools-editor/material-editor.md) | [material-editor](../features/tools-editor/material-editor.md) | Rendering.Lighting, ContentPipeline.HotReload | Not started | Not started |
| 54 | ToolsEditor.Profiling | [profiling-tools](../requirements/tools-editor/profiling-tools.md) | [profiling-tools](../features/tools-editor/profiling-tools.md) | CoreRuntime.ECS, Platform.Threading | Not started | Not started |
| 55 | GameFramework.SaveSystem | [save-system](../requirements/game-framework/save-system.md) | [save-system](../features/game-framework/save-system.md) | CoreRuntime.ReflectionSerialization, GameFramework.GameplayPrimitives | Not started | Not started |
| 56 | GameFramework.Abilities | [abilities](../requirements/game-framework/abilities.md) | [abilities](../features/game-framework/abilities.md) | GameFramework.GameplayPrimitives, Animation.StateMachine, Physics.SpatialQueries | Not started | Not started |
| 57 | GameFramework.WeaponSystem | [weapon-system](../requirements/game-framework/weapon-system.md) | [weapon-system](../features/game-framework/weapon-system.md) | GameFramework.GameplayPrimitives, Physics.SpatialQueries, Animation.StateMachine | Not started | Not started |
| 58 | GameFramework.Progression | [progression](../requirements/game-framework/progression.md) | [progression](../features/game-framework/progression.md) | GameFramework.GameplayPrimitives | Not started | Not started |
| 59 | GameFramework.WorldManagement | [world-management](../requirements/game-framework/world-management.md) | [world-management](../features/game-framework/world-management.md) | CoreRuntime.ECS, ContentPipeline.Streaming | Not started | Not started |
| 60 | Physics.SoftBodyCloth | [soft-body-and-cloth](../requirements/physics/soft-body-and-cloth.md) | [soft-body-and-cloth](../features/physics/soft-body-and-cloth.md) | Physics.RigidBody, Animation.Skeletal | Not started | Not started |
| 61 | Physics.Destruction | [destruction-and-fracture](../requirements/physics/destruction-and-fracture.md) | [destruction-and-fracture](../features/physics/destruction-and-fracture.md) | Physics.RigidBody, Physics.Constraints | Not started | Not started |
| 62 | GeometryWorld.Water | [water](../requirements/geometry-world/water.md) | [water](../features/geometry-world/water.md) | Rendering.CoreRendering, Physics.RigidBody | Not started | Not started |
| 63 | Audio.MusicSystem | [music-system](../requirements/audio/music-system.md) | [music-system](../features/audio/music-system.md) | Audio.Engine, Audio.DSP | Not started | Not started |
| 64 | Audio.VoiceSpeech | [voice-and-speech](../requirements/audio/voice-and-speech.md) | [voice-and-speech](../features/audio/voice-and-speech.md) | Audio.Engine, Networking.Transport | Not started | Not started |
| 65 | AI.CrowdTactical | [crowd-simulation](../requirements/ai/crowd-simulation.md), [tactical-combat](../requirements/ai/tactical-combat.md) | [crowd-simulation](../features/ai/crowd-simulation.md), [tactical-combat](../features/ai/tactical-combat.md) | AI.BehaviorDecision, AI.Steering | Not started | Not started |
| 66 | VFX.Weather | [weather-environmental](../requirements/vfx/weather-environmental.md) | [weather-environmental](../features/vfx/weather-environmental.md) | VFX.Particles, Audio.Spatial | Not started | Not started |
| 67 | VFX.EffectGraph | [effect-graph](../requirements/vfx/effect-graph.md) | [effect-graph](../features/vfx/effect-graph.md) | VFX.Particles, ToolsEditor.LogicGraph | Not started | Not started |
| 68 | UI2D.2DGames | [2d-games](../requirements/ui-2d/2d-games.md) | [2d-games](../features/ui-2d/2d-games.md) | UI2D.WidgetFramework, Physics.RigidBody, Rendering.CoreRendering | Not started | Not started |
| 69 | Networking.AntiCheat | [anti-cheat](../requirements/networking/anti-cheat.md) | [anti-cheat](../features/networking/anti-cheat.md) | Networking.Replication, GameFramework.GameplayPrimitives | Not started | Not started |

---

## Wave 6 — Capstone Systems

Requires specific Wave 5 nodes. 18 feature-groups can proceed concurrently.

| # | Feature-Group | Requirements | Features | Prereqs | Design | Impl |
|---|---------------|-------------|----------|---------|--------|------|
| 70 | ToolsEditor.AnimationEditor | [animation-editor](../requirements/tools-editor/animation-editor.md) | [animation-editor](../features/tools-editor/animation-editor.md) | ToolsEditor.EditorFramework, Animation.StateMachine | Not started | Not started |
| 71 | ToolsEditor.WorldBuilding | [world-building](../requirements/tools-editor/world-building.md) | [world-building](../features/tools-editor/world-building.md) | ToolsEditor.EditorFramework, GeometryWorld.Terrain, GeometryWorld.Foliage | Not started | Not started |
| 72 | ToolsEditor.AdvancedTools | [version-control](../requirements/tools-editor/version-control.md), [remote-editing](../requirements/tools-editor/remote-editing.md), [ai-assistant](../requirements/tools-editor/ai-assistant.md), [ai-governance](../requirements/tools-editor/ai-governance.md), [deployment](../requirements/tools-editor/deployment.md), [launcher](../requirements/tools-editor/launcher.md), [localization-editor](../requirements/tools-editor/localization-editor.md), [mod-support](../requirements/tools-editor/mod-support.md), [shared-cache](../requirements/tools-editor/shared-cache.md) | [version-control](../features/tools-editor/version-control.md), [remote-editing](../features/tools-editor/remote-editing.md), [deployment](../features/tools-editor/deployment.md), [launcher](../features/tools-editor/launcher.md), [localization-editor](../features/tools-editor/localization-editor.md), [mod-support](../features/tools-editor/mod-support.md) | ToolsEditor.EditorFramework, Networking.Replication | Not started | Not started |
| 73 | ContentPipeline.DCCPlugins | [dcc-plugins](../requirements/content-pipeline/dcc-plugins.md) | [dcc-plugins](../features/content-pipeline/dcc-plugins.md) | ContentPipeline.AssetImport, ContentPipeline.AssetProcessing | Not started | Not started |
| 74 | Animation.ClothHair | [cloth-hair](../requirements/animation/cloth-hair.md) | [cloth-hair](../features/animation/cloth-hair.md) | Physics.SoftBodyCloth, Audio.VoiceSpeech, Rendering.AdvancedRendering | Not started | Not started |
| 75 | Animation.FirstPerson | [first-person](../requirements/animation/first-person.md) | [first-person](../features/animation/first-person.md) | Animation.Skeletal, GameFramework.WeaponSystem | Not started | Not started |
| 76 | VFX.Destruction | [destruction](../requirements/vfx/destruction.md) | [destruction](../features/vfx/destruction.md) | Physics.Destruction, VFX.Particles | Not started | Not started |
| 77 | GameFramework.NPCSimulation | [npc-simulation](../requirements/game-framework/npc-simulation.md) | [npc-simulation](../features/game-framework/npc-simulation.md) | AI.BehaviorDecision, GameFramework.GameplayPrimitives | Not started | Not started |
| 78 | GameFramework.Social | [social](../requirements/game-framework/social.md) | [social](../features/game-framework/social.md) | Networking.Session, GameFramework.GameplayPrimitives | Not started | Not started |
| 79 | GameFramework.GameModes | [game-modes-misc](../requirements/game-framework/game-modes-misc.md) | [game-modes-misc](../features/game-framework/game-modes-misc.md) | Networking.Session, GameFramework.GameplayPrimitives | Not started | Not started |
| 80 | GameFramework.StealthCover | [stealth-cover](../requirements/game-framework/stealth-cover.md) | [stealth-cover](../features/game-framework/stealth-cover.md) | AI.Perception, GameFramework.GameplayPrimitives | Not started | Not started |
| 81 | GameFramework.GenreSpecific | [building-survival](../requirements/game-framework/building-survival.md), [racing](../requirements/game-framework/racing.md), [turn-based](../requirements/game-framework/turn-based.md), [fog-of-war](../requirements/game-framework/fog-of-war.md), [minigames](../requirements/game-framework/minigames.md), [pets-mounts](../requirements/game-framework/pets-mounts.md), [monetization](../requirements/game-framework/monetization.md), [traversal-interaction](../requirements/game-framework/traversal-interaction.md), [selection-system](../requirements/game-framework/selection-system.md) | [building-survival](../features/game-framework/building-survival.md), [racing](../features/game-framework/racing.md), [turn-based](../features/game-framework/turn-based.md), [fog-of-war](../features/game-framework/fog-of-war.md), [minigames](../features/game-framework/minigames.md), [pets-mounts](../features/game-framework/pets-mounts.md), [monetization](../features/game-framework/monetization.md), [traversal-interaction](../features/game-framework/traversal-interaction.md), [selection-system](../features/game-framework/selection-system.md) | GameFramework.GameplayPrimitives, GameFramework.Abilities, Physics.RigidBody, AI.Navigation | Not started | Not started |
| 82 | GeometryWorld.TerrainVoxel | [terrain](../requirements/geometry-world/terrain.md) (voxel section) | [terrain](../features/geometry-world/terrain.md) (voxel section) | GeometryWorld.Terrain, Physics.Destruction | Not started | Not started |
| 83 | GeometryWorld.ProceduralGeneration | [procedural-generation](../requirements/geometry-world/procedural-generation.md) | [procedural-generation](../features/geometry-world/procedural-generation.md) | GeometryWorld.Terrain, Rendering.AdvancedRendering, ContentPipeline.Streaming | Not started | Not started |
| 84 | Physics.Fluid | [fluid-simulation](../requirements/physics/fluid-simulation.md) | [fluid-simulation](../features/physics/fluid-simulation.md) | Physics.RigidBody, GeometryWorld.Water | Not started | Not started |
| 85 | Networking.MMO | [mmo-infrastructure](../requirements/networking/mmo-infrastructure.md) | [mmo-infrastructure](../features/networking/mmo-infrastructure.md) | Networking.Replication, GameFramework.WorldManagement, GameFramework.SaveSystem | Not started | Not started |
| 86 | UI2D.Accessibility | [accessibility](../requirements/ui-2d/accessibility.md) | [accessibility](../features/ui-2d/accessibility.md) | UI2D.WidgetFramework, Audio.Engine, Input.ActionsMapping | Not started | Not started |
| 87 | Input.VRInput | [vr-input](../requirements/input/vr-input.md) | [vr-input](../features/input/vr-input.md) | Input.DeviceAbstraction, Rendering.AdvancedRendering | Not started | Not started |

---

## Requirements and Features Not Yet Mapped to Feature-Groups

The following requirement/feature docs exist but are not explicitly covered by the 87 feature-groups
above. They should be folded into existing groups or promoted to new groups.

### Unmapped Requirements

| File | Candidate Feature-Group |
|------|------------------------|
| [cross-cutting](../requirements/cross-cutting.md) | Spans all groups — constraints on all waves |
| [animation/morph](../requirements/animation/morph.md) | Animation.Skeletal (morph targets are a skeletal subsystem) |
| [input/haptics-and-feedback](../requirements/input/haptics-and-feedback.md) | Input.DeviceAbstraction or Input.ActionsMapping |
| [physics/vehicle-physics](../requirements/physics/vehicle-physics.md) | New group: Physics.Vehicles, or fold into GameFramework.GenreSpecific |
| [networking/remote-procedure-calls](../requirements/networking/remote-procedure-calls.md) | Networking.Replication (RPCs are a replication mechanism) |
| [networking/replay-system](../requirements/networking/replay-system.md) | Networking.PredictionRollback (replay is a rollback byproduct) |
| [content-pipeline/asset-versioning](../requirements/content-pipeline/asset-versioning.md) | ContentPipeline.AssetDatabase |
| [geometry-world/sky-atmosphere](../requirements/geometry-world/sky-atmosphere.md) | Rendering.AdvancedRendering (sky is a rendering feature) or new GeometryWorld group |
| [game-framework/inventory](../requirements/game-framework/inventory.md) | GameFramework.GenreSpecific or GameFramework.GameplayPrimitives |
| [game-framework/quest-dialogue](../requirements/game-framework/quest-dialogue.md) | GameFramework.GenreSpecific |
| [game-framework/character-customization](../requirements/game-framework/character-customization.md) | GameFramework.GenreSpecific |
| [game-framework/cinematics](../requirements/game-framework/cinematics.md) | GameFramework.GenreSpecific or ToolsEditor group |
| [game-framework/scripting](../requirements/game-framework/scripting.md) | ToolsEditor.LogicGraph (scripting = logic graph in no-code engine) |
| [game-framework/gameplay-databases](../requirements/game-framework/gameplay-databases.md) | GameFramework.GameplayPrimitives |
| [game-framework/advertising](../requirements/game-framework/advertising.md) | GameFramework.GenreSpecific |
| [game-framework/block-voxel](../requirements/game-framework/block-voxel.md) | GeometryWorld.TerrainVoxel or GameFramework.GenreSpecific |
| [tools-editor/level-editor](../requirements/tools-editor/level-editor.md) | ToolsEditor.EditorFramework or ToolsEditor.WorldBuilding |
| [tools-editor/asset-store](../requirements/tools-editor/asset-store.md) | ToolsEditor.AdvancedTools |
| [tools-editor/server-infrastructure](../requirements/tools-editor/server-infrastructure.md) | ToolsEditor.AdvancedTools |
| [ui-2d/common-widgets](../requirements/ui-2d/common-widgets.md) | UI2D.WidgetFramework |
| [ai/non-functional](../requirements/ai/non-functional.md) | Spans AI groups — performance constraints |
| [networking/non-functional](../requirements/networking/non-functional.md) | Spans Networking groups — performance constraints |
| [rendering/gpu-abstraction](../requirements/rendering/gpu-abstraction.md) | Rendering.GPUAbstraction (possibly duplicate of gpu-abstraction-layer) |

### Unmapped Features

| File | Candidate Feature-Group |
|------|------------------------|
| [animation/morph](../features/animation/morph.md) | Animation.Skeletal |
| [input/haptics-and-feedback](../features/input/haptics-and-feedback.md) | Input.DeviceAbstraction |
| [physics/vehicle-physics](../features/physics/vehicle-physics.md) | GameFramework.GenreSpecific |
| [networking/remote-procedure-calls](../features/networking/remote-procedure-calls.md) | Networking.Replication |
| [networking/replay-system](../features/networking/replay-system.md) | Networking.PredictionRollback |
| [content-pipeline/asset-versioning](../features/content-pipeline/asset-versioning.md) | ContentPipeline.AssetDatabase |
| [geometry-world/sky-atmosphere](../features/geometry-world/sky-atmosphere.md) | Rendering.AdvancedRendering |
| [game-framework/inventory](../features/game-framework/inventory.md) | GameFramework.GenreSpecific |
| [game-framework/quest-dialogue](../features/game-framework/quest-dialogue.md) | GameFramework.GenreSpecific |
| [game-framework/character-customization](../features/game-framework/character-customization.md) | GameFramework.GenreSpecific |
| [game-framework/cinematics](../features/game-framework/cinematics.md) | GameFramework.GenreSpecific |
| [game-framework/scripting](../features/game-framework/scripting.md) | ToolsEditor.LogicGraph |
| [game-framework/gameplay-databases](../features/game-framework/gameplay-databases.md) | GameFramework.GameplayPrimitives |
| [game-framework/advertising](../features/game-framework/advertising.md) | GameFramework.GenreSpecific |
| [game-framework/block-voxel](../features/game-framework/block-voxel.md) | GeometryWorld.TerrainVoxel |
| [tools-editor/level-editor](../features/tools-editor/level-editor.md) | ToolsEditor.WorldBuilding |
| [tools-editor/asset-store](../features/tools-editor/asset-store.md) | ToolsEditor.AdvancedTools |
| [tools-editor/server-infrastructure](../features/tools-editor/server-infrastructure.md) | ToolsEditor.AdvancedTools |
| [tools-editor/ai-assistant](../features/tools-editor/ai-assistant.md) | ToolsEditor.AdvancedTools |
| [tools-editor/ai-governance](../features/tools-editor/ai-governance.md) | ToolsEditor.AdvancedTools |
| [tools-editor/shared-cache](../features/tools-editor/shared-cache.md) | ToolsEditor.AdvancedTools |
| [ui-2d/common-widgets](../features/ui-2d/common-widgets.md) | UI2D.WidgetFramework |
| [audio/dsp-and-effects](../features/audio/dsp-and-effects.md) | Audio.DSP |
| [audio/music-system](../features/audio/music-system.md) | Audio.MusicSystem |
