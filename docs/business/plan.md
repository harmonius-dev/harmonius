# Harmonius Plan

Work plan for the Harmonius game engine. Maps every feature-group from the
[domain decomposition](domain-decomposition.md) to its requirements, features, design status, and
implementation status. Organized by wave to show what can proceed in parallel at each stage.

**Current state:** Requirements and features are fully specified (1,501 requirements, 1,204 features
across 15 domains). Build system is configured (Rust stable, XcodeGen for macOS/iOS,
Metal/Vulkan/D3D12 backend selection). No source code exists. No designs exist.

---

## Wave 0 — Platform Root

No prerequisites. These are the first feature-groups to design and implement.

| # | Feature-Group      | Design      | Implementation |
|---|--------------------|-------------|----------------|
| 1 | Platform.Windowing | Not started | Not started    |
| 2 | Platform.Threading | Not started | Not started    |
| 3 | Platform.OS        | Not started | Not started    |

1. **1** — [window-display](../requirements/platform/window-display.md)
   - **Features:** [window-display](../features/platform/window-display.md)
2. **2** — [threading-async](../requirements/platform/threading-async.md)
   - **Features:** [threading-async](../features/platform/threading-async.md)
3. **3** — [os-integration](../requirements/platform/os-integration.md),
   [crash-reporting](../requirements/platform/crash-reporting.md),
   [platform-services](../requirements/platform/platform-services.md),
   [filesystem](../requirements/platform/filesystem.md)
   - **Features:** [os-integration](../features/platform/os-integration.md),
     [crash-reporting](../features/platform/crash-reporting.md),
     [platform-services](../features/platform/platform-services.md),
     [filesystem](../features/platform/filesystem.md)

**Exit criteria:** Windows open on macOS/Linux/Windows, thread pool executes tasks, async file I/O
completes, crash handler captures dumps.

---

## Wave 1 — Core Runtime

Requires Wave 0. CoreRuntime.ECS should lead; other groups depend on its primitives.

| # | Feature-Group                       | Design      | Implementation |
|---|-------------------------------------|-------------|----------------|
| 4 | CoreRuntime.ECS                     | Not started | Not started    |
| 5 | CoreRuntime.SceneTransforms         | Not started | Not started    |
| 6 | CoreRuntime.ReflectionSerialization | Not started | Not started    |
| 7 | CoreRuntime.EventsPlugins           | Not started | Not started    |
| 8 | CoreRuntime.MemoryAsyncIO           | Not started | Not started    |
| 9 | CoreRuntime.SpatialIndex            | Not started | Not started    |

1. **4** — [entity-component-system](../requirements/core-runtime/entity-component-system.md)
   - **Features:** [entity-component-system](../features/core-runtime/entity-component-system.md)
2. **5** — [scene-and-transforms](../requirements/core-runtime/scene-and-transforms.md)
   - **Features:** [scene-and-transforms](../features/core-runtime/scene-and-transforms.md)
3. **6** — [reflection-and-type-system](../requirements/core-runtime/reflection-and-type-system.md),
   [serialization](../requirements/core-runtime/serialization.md)
   - **Features:**
     [reflection-and-type-system](../features/core-runtime/reflection-and-type-system.md),
     [serialization](../features/core-runtime/serialization.md)
4. **7** — [events-and-messaging](../requirements/core-runtime/events-and-messaging.md),
   [plugin-system](../requirements/core-runtime/plugin-system.md)
   - **Features:** [events-and-messaging](../features/core-runtime/events-and-messaging.md),
     [plugin-system](../features/core-runtime/plugin-system.md)
5. **8** — [memory-management](../requirements/core-runtime/memory-management.md),
   [async-io](../requirements/core-runtime/async-io.md)
   - **Features:** [memory-management](../features/core-runtime/memory-management.md),
     [async-io](../features/core-runtime/async-io.md)
6. **9** — [spatial-indexing](../requirements/core-runtime/spatial-indexing.md)
   - **Features:** [spatial-indexing](../features/core-runtime/spatial-indexing.md)

**Exit criteria:** Entities spawn, queries iterate, systems schedule in parallel, transforms
propagate, events fire, types reflect, binary serialization round-trips, arena allocators work,
async I/O completes, spatial index inserts/queries work.

**Interoperability contracts to lock before Wave 2:**

| Contract                        | Defined By                          |
|---------------------------------|-------------------------------------|
| Component trait + archetype API | CoreRuntime.ECS                     |
| Entity handle type              | CoreRuntime.ECS                     |
| System trait + scheduling API   | CoreRuntime.ECS                     |
| Event channel API               | CoreRuntime.EventsPlugins           |
| Spatial index query API         | CoreRuntime.SpatialIndex            |
| Serialization trait             | CoreRuntime.ReflectionSerialization |
| Async I/O trait                 | CoreRuntime.MemoryAsyncIO           |
| Transform component schema      | CoreRuntime.SceneTransforms         |

1. **Component trait + archetype API** — How to define, register, query, and mutate components
2. **Entity handle type** — Generational index format, validity checking
3. **System trait + scheduling API** — How to declare systems, dependencies, and access patterns
4. **Event channel API** — How to send and receive typed events
5. **Spatial index query API** — Ray-cast, shape-cast, overlap, frustum, k-nearest
6. **Serialization trait** — Binary/text encode/decode for components and assets
7. **Async I/O trait** — Read/write/cancel operations, completion callbacks
8. **Transform component schema** — Position, rotation, scale, parent, world matrix

---

## Wave 2 — Infrastructure Fan-Out

Requires specific Wave 1 nodes. 9 feature-groups can proceed concurrently.

| #  | Feature-Group                 | Design      | Impl        |
|----|-------------------------------|-------------|-------------|
| 10 | Rendering.GPUAbstraction      | Not started | Not started |
| 11 | ContentPipeline.AssetImport   | Not started | Not started |
| 12 | ContentPipeline.AssetDatabase | Not started | Not started |
| 13 | Physics.RigidBody             | Not started | Not started |
| 14 | Physics.SpatialQueries        | Not started | Not started |
| 15 | Input.DeviceAbstraction       | Not started | Not started |
| 16 | Audio.Engine                  | Not started | Not started |
| 17 | Animation.Skeletal            | Not started | Not started |
| 18 | Networking.Transport          | Not started | Not started |

1. **10** — [gpu-abstraction-layer](../requirements/rendering/gpu-abstraction-layer.md),
   [gpu-abstraction](../requirements/rendering/gpu-abstraction.md)
   - **Features:** [gpu-abstraction-layer](../features/rendering/gpu-abstraction-layer.md)
   - **Prereqs:** Platform.Windowing, CoreRuntime.ECS
2. **11** — [asset-import](../requirements/content-pipeline/asset-import.md)
   - **Features:** [asset-import](../features/content-pipeline/asset-import.md)
   - **Prereqs:** CoreRuntime.MemoryAsyncIO, Platform.Threading
3. **12** — [asset-database](../requirements/content-pipeline/asset-database.md)
   - **Features:** [asset-database](../features/content-pipeline/asset-database.md)
   - **Prereqs:** CoreRuntime.ReflectionSerialization
4. **13** — [rigid-body-dynamics](../requirements/physics/rigid-body-dynamics.md),
   [collision-detection](../requirements/physics/collision-detection.md)
   - **Features:** [rigid-body-dynamics](../features/physics/rigid-body-dynamics.md),
     [collision-detection](../features/physics/collision-detection.md)
   - **Prereqs:** CoreRuntime.ECS, CoreRuntime.SpatialIndex
5. **14** — [spatial-queries](../requirements/physics/spatial-queries.md)
   - **Features:** [spatial-queries](../features/physics/spatial-queries.md)
   - **Prereqs:** CoreRuntime.SpatialIndex
6. **15** — [device-abstraction](../requirements/input/device-abstraction.md)
   - **Features:** [device-abstraction](../features/input/device-abstraction.md)
   - **Prereqs:** Platform.Windowing, CoreRuntime.ECS
7. **16** — [audio-engine](../requirements/audio/audio-engine.md)
   - **Features:** [audio-engine](../features/audio/audio-engine.md)
   - **Prereqs:** Platform.OS, Platform.Threading, CoreRuntime.ECS
8. **17** — [skeletal](../requirements/animation/skeletal.md)
   - **Features:** [skeletal](../features/animation/skeletal.md)
   - **Prereqs:** CoreRuntime.ECS, CoreRuntime.SceneTransforms
9. **18** — [transport-layer](../requirements/networking/transport-layer.md)
   - **Features:** [transport-layer](../features/networking/transport-layer.md)
   - **Prereqs:** Platform.Threading, CoreRuntime.ECS

**Additional contracts to lock before Wave 3:**

| Contract          | Defined By                    |
|-------------------|-------------------------------|
| GPU backend trait | Rendering.GPUAbstraction      |
| Asset handle type | ContentPipeline.AssetDatabase |

1. **GPU backend trait** — Device, command buffer, pipeline, buffer, texture APIs
2. **Asset handle type** — Typed handle, load state, reference counting

**Exit criteria:** GPU device created and triangle renders, assets import from disk, rigid bodies
simulate, ray-casts work, keyboard/mouse/gamepad input captured, audio plays, skeleton evaluates and
skins, network connection established.

---

## Wave 3 — Intermediate Systems

Requires specific Wave 2 nodes. 12 feature-groups can proceed concurrently.

| #  | Feature-Group                   | Design      | Impl        |
|----|---------------------------------|-------------|-------------|
| 19 | Rendering.RenderGraph           | Not started | Not started |
| 20 | ContentPipeline.AssetProcessing | Not started | Not started |
| 21 | ContentPipeline.Streaming       | Not started | Not started |
| 22 | Input.ActionsMapping            | Not started | Not started |
| 23 | Audio.Spatial                   | Not started | Not started |
| 24 | Animation.StateMachine          | Not started | Not started |
| 25 | Physics.Constraints             | Not started | Not started |
| 26 | AI.Navigation                   | Not started | Not started |
| 27 | Networking.Replication          | Not started | Not started |
| 28 | GeometryWorld.MeshletPipeline   | Not started | Not started |
| 29 | UI2D.WidgetFramework            | Not started | Not started |
| 30 | VFX.Particles                   | Not started | Not started |

1. **19** — [render-graph](../requirements/rendering/render-graph.md)
   - **Features:** [render-graph](../features/rendering/render-graph.md)
   - **Prereqs:** Rendering.GPUAbstraction
2. **20** — [asset-processing](../requirements/content-pipeline/asset-processing.md)
   - **Features:** [asset-processing](../features/content-pipeline/asset-processing.md)
   - **Prereqs:** ContentPipeline.AssetImport, Rendering.GPUAbstraction
3. **21** — [streaming-io](../requirements/content-pipeline/streaming-io.md)
   - **Features:** [streaming-io](../features/content-pipeline/streaming-io.md)
   - **Prereqs:** ContentPipeline.AssetDatabase, CoreRuntime.MemoryAsyncIO
4. **22** — [input-actions-and-mapping](../requirements/input/input-actions-and-mapping.md)
   - **Features:** [input-actions-and-mapping](../features/input/input-actions-and-mapping.md)
   - **Prereqs:** Input.DeviceAbstraction
5. **23** — [spatial-audio](../requirements/audio/spatial-audio.md)
   - **Features:** [spatial-audio](../features/audio/spatial-audio.md)
   - **Prereqs:** Audio.Engine, CoreRuntime.SpatialIndex
6. **24** — [state-machine](../requirements/animation/state-machine.md)
   - **Features:** [state-machine](../features/animation/state-machine.md)
   - **Prereqs:** Animation.Skeletal
7. **25** — [constraints-and-joints](../requirements/physics/constraints-and-joints.md)
   - **Features:** [constraints-and-joints](../features/physics/constraints-and-joints.md)
   - **Prereqs:** Physics.RigidBody
8. **26** — [navigation](../requirements/ai/navigation.md)
   - **Features:** [navigation](../features/ai/navigation.md)
   - **Prereqs:** CoreRuntime.SpatialIndex, Physics.SpatialQueries
9. **27** — [state-replication](../requirements/networking/state-replication.md)
   - **Features:** [state-replication](../features/networking/state-replication.md)
   - **Prereqs:** Networking.Transport, CoreRuntime.ReflectionSerialization,
     CoreRuntime.SpatialIndex
10. **28** — [meshlet-pipeline](../requirements/geometry/meshlet-pipeline.md)
    - **Features:** [meshlet-pipeline](../features/geometry/meshlet-pipeline.md)
    - **Prereqs:** Rendering.GPUAbstraction, CoreRuntime.ECS
11. **29** — [widget-framework](../requirements/ui/widget-framework.md)
    - **Features:** [widget-framework](../features/ui/widget-framework.md)
    - **Prereqs:** CoreRuntime.ECS, Input.DeviceAbstraction
12. **30** — [particles](../requirements/vfx/particles.md)
    - **Features:** [particles](../features/vfx/particles.md)
    - **Prereqs:** Rendering.GPUAbstraction, CoreRuntime.ECS

**Additional contract to lock before Wave 4:**

| Contract              | Defined By            |
|-----------------------|-----------------------|
| Render graph pass API | Rendering.RenderGraph |

1. **Render graph pass API** — How to declare passes, read/write resources, submit commands

**Exit criteria:** Render graph executes passes with automatic barriers, shaders compile via
DXC/MSC, assets stream from disk, input actions map to gameplay, spatial audio occludes, animation
state machines transition, joints constrain bodies, NavMesh generates and pathfinds, components
replicate over network, meshlets render, widget tree lays out, particles simulate on GPU.

---

## Wave 4 — Integration Layer

Requires specific Wave 3 nodes. 19 feature-groups can proceed concurrently.

| #  | Feature-Group                    | Design      | Impl        |
|----|----------------------------------|-------------|-------------|
| 31 | Rendering.CoreRendering          | Not started | Not started |
| 32 | Rendering.Lighting               | Not started | Not started |
| 33 | ContentPipeline.HotReload        | Not started | Not started |
| 34 | AI.Steering                      | Not started | Not started |
| 35 | AI.Perception                    | Not started | Not started |
| 36 | AI.BehaviorDecision              | Not started | Not started |
| 37 | Animation.Procedural             | Not started | Not started |
| 38 | Audio.DSP                        | Not started | Not started |
| 39 | Networking.PredictionRollback    | Not started | Not started |
| 40 | Networking.Session               | Not started | Not started |
| 41 | GameFramework.GameplayPrimitives | Not started | Not started |
| 42 | GameFramework.Camera             | Not started | Not started |
| 43 | UI2D.UIRendering                 | Not started | Not started |
| 44 | UI2D.GameUI                      | Not started | Not started |
| 45 | VFX.Decals                       | Not started | Not started |
| 46 | VFX.ScreenEffects                | Not started | Not started |
| 47 | GeometryWorld.Terrain            | Not started | Not started |
| 48 | GeometryWorld.Foliage            | Not started | Not started |
| 49 | Input.Gestures                   | Not started | Not started |

1. **31** — [core-rendering](../requirements/rendering/core-rendering.md),
   [scene-rendering-pipeline](../requirements/rendering/scene-rendering-pipeline.md)
   - **Features:** [core-rendering](../features/rendering/core-rendering.md),
     [scene-rendering-pipeline](../features/rendering/scene-rendering-pipeline.md)
   - **Prereqs:** Rendering.RenderGraph, CoreRuntime.SpatialIndex, Animation.Skeletal
2. **32** — [lighting](../requirements/rendering/lighting.md)
   - **Features:** [lighting](../features/rendering/lighting.md)
   - **Prereqs:** Rendering.CoreRendering
3. **33** — [hot-reload](../requirements/content-pipeline/hot-reload.md)
   - **Features:** [hot-reload](../features/content-pipeline/hot-reload.md)
   - **Prereqs:** ContentPipeline.AssetDatabase, ContentPipeline.AssetProcessing
4. **34** — [steering-avoidance](../requirements/ai/steering-avoidance.md)
   - **Features:** [steering-avoidance](../features/ai/steering-avoidance.md)
   - **Prereqs:** AI.Navigation
5. **35** — [perception](../requirements/ai/perception.md)
   - **Features:** [perception](../features/ai/perception.md)
   - **Prereqs:** CoreRuntime.SpatialIndex, AI.Navigation
6. **36** — [behavior-trees](../requirements/ai/behavior-trees.md),
   [utility-ai](../requirements/ai/utility-ai.md), [goap](../requirements/ai/goap.md)
   - **Features:** [behavior-trees](../features/ai/behavior-trees.md),
     [utility-ai](../features/ai/utility-ai.md), [goap](../features/ai/goap.md)
   - **Prereqs:** AI.Perception
7. **37** — [procedural](../requirements/animation/procedural.md)
   - **Features:** [procedural](../features/animation/procedural.md)
   - **Prereqs:** Animation.Skeletal, Physics.RigidBody, Physics.Constraints
8. **38** — [dsp-and-effects](../requirements/audio/dsp-and-effects.md)
   - **Features:** [dsp-and-effects](../features/audio/dsp-and-effects.md)
   - **Prereqs:** Audio.Engine
9. **39** — [prediction-and-rollback](../requirements/networking/prediction-and-rollback.md)
   - **Features:** [prediction-and-rollback](../features/networking/prediction-and-rollback.md)
   - **Prereqs:** Networking.Replication, Physics.RigidBody
10. **40** — [session-management](../requirements/networking/session-management.md)
    - **Features:** [session-management](../features/networking/session-management.md)
    - **Prereqs:** Networking.Transport
11. **41** — [gameplay-primitives](../requirements/game-framework/gameplay-primitives.md)
    - **Features:** [gameplay-primitives](../features/game-framework/gameplay-primitives.md)
    - **Prereqs:** CoreRuntime.ECS, CoreRuntime.SpatialIndex, Physics.RigidBody,
      Physics.SpatialQueries, Input.ActionsMapping
12. **42** — [camera-system](../requirements/game-framework/camera-system.md)
    - **Features:** [camera-system](../features/game-framework/camera-system.md)
    - **Prereqs:** CoreRuntime.SceneTransforms, Physics.SpatialQueries
13. **43** — [ui-rendering](../requirements/ui/ui-rendering.md)
    - **Features:** [ui-rendering](../features/ui/ui-rendering.md)
    - **Prereqs:** UI2D.WidgetFramework, Rendering.RenderGraph, Rendering.GPUAbstraction
14. **44** — [hud-and-game-ui](../requirements/ui/hud-and-game-ui.md)
    - **Features:** [hud-and-game-ui](../features/ui/hud-and-game-ui.md)
    - **Prereqs:** UI2D.WidgetFramework, UI2D.UIRendering
15. **45** — [decals](../requirements/vfx/decals.md)
    - **Features:** [decals](../features/vfx/decals.md)
    - **Prereqs:** Rendering.RenderGraph
16. **46** — [screen-effects](../requirements/vfx/screen-effects.md)
    - **Features:** [screen-effects](../features/vfx/screen-effects.md)
    - **Prereqs:** Rendering.CoreRendering
17. **47** — [terrain](../requirements/geometry/terrain.md)
    - **Features:** [terrain](../features/geometry/terrain.md)
    - **Prereqs:** Rendering.CoreRendering, ContentPipeline.Streaming, Physics.RigidBody
18. **48** — [foliage](../requirements/geometry/foliage.md)
    - **Features:** [foliage](../features/geometry/foliage.md)
    - **Prereqs:** Rendering.CoreRendering, CoreRuntime.SpatialIndex, ContentPipeline.Streaming
19. **49** — [gestures](../requirements/input/gestures.md)
    - **Features:** [gestures](../features/input/gestures.md)
    - **Prereqs:** Input.DeviceAbstraction

**Exit criteria:** Lit PBR scene renders with shadows, hot reload works for all asset types, AI
agents navigate and perceive, procedural animation (IK, ragdoll blend) works, audio has DSP chains,
client-side prediction works, gameplay primitives exist, camera follows player, UI renders in-game
HUD, decals and screen effects render, terrain and foliage render with streaming.

---

## Wave 5 — Advanced Systems

Requires specific Wave 4 nodes. 20 feature-groups can proceed concurrently.

| #  | Feature-Group                 | Design      | Impl        |
|----|-------------------------------|-------------|-------------|
| 50 | Rendering.AdvancedRendering   | Not started | Not started |
| 51 | ToolsEditor.EditorFramework   | Not started | Not started |
| 52 | ToolsEditor.LogicGraph        | Not started | Not started |
| 53 | ToolsEditor.MaterialEditor    | Not started | Not started |
| 54 | ToolsEditor.Profiling         | Not started | Not started |
| 55 | GameFramework.SaveSystem      | Not started | Not started |
| 56 | GameFramework.Abilities       | Not started | Not started |
| 57 | GameFramework.WeaponSystem    | Not started | Not started |
| 58 | GameFramework.Progression     | Not started | Not started |
| 59 | GameFramework.WorldManagement | Not started | Not started |
| 60 | Physics.SoftBodyCloth         | Not started | Not started |
| 61 | Physics.Destruction           | Not started | Not started |
| 62 | GeometryWorld.Water           | Not started | Not started |
| 63 | Audio.MusicSystem             | Not started | Not started |
| 64 | Audio.VoiceSpeech             | Not started | Not started |
| 65 | AI.CrowdTactical              | Not started | Not started |
| 66 | VFX.Weather                   | Not started | Not started |
| 67 | VFX.EffectGraph               | Not started | Not started |
| 68 | UI2D.2DGames                  | Not started | Not started |
| 69 | Networking.AntiCheat          | Not started | Not started |

1. **50** — [advanced-rendering](../requirements/rendering/advanced-rendering.md),
   [advanced-materials](../requirements/rendering/advanced-materials.md),
   [character-rendering](../requirements/rendering/character-rendering.md),
   [environment](../requirements/rendering/environment.md),
   [post-processing](../requirements/rendering/post-processing.md),
   [anti-aliasing-upscaling](../requirements/rendering/anti-aliasing-upscaling.md),
   [stylized-effects](../requirements/rendering/stylized-effects.md)
   - **Features:** [advanced-rendering](../features/rendering/advanced-rendering.md),
     [advanced-materials](../features/rendering/advanced-materials.md),
     [character-rendering](../features/rendering/character-rendering.md),
     [environment](../features/rendering/environment.md),
     [post-processing](../features/rendering/post-processing.md),
     [anti-aliasing-upscaling](../features/rendering/anti-aliasing-upscaling.md),
     [stylized-effects](../features/rendering/stylized-effects.md)
   - **Prereqs:** Rendering.Lighting
2. **51** — [editor-framework](../requirements/tools/editor-framework.md)
   - **Features:** [editor-framework](../features/tools/editor-framework.md)
   - **Prereqs:** UI2D.WidgetFramework, CoreRuntime.ReflectionSerialization,
     ContentPipeline.HotReload, Rendering.CoreRendering, Input.ActionsMapping
3. **52** — [logic-graph](../requirements/tools/logic-graph.md)
   - **Features:** [logic-graph](../features/tools/logic-graph.md)
   - **Prereqs:** CoreRuntime.ECS, CoreRuntime.EventsPlugins, ContentPipeline.HotReload
4. **53** — [material-editor](../requirements/tools/material-editor.md)
   - **Features:** [material-editor](../features/tools/material-editor.md)
   - **Prereqs:** Rendering.Lighting, ContentPipeline.HotReload
5. **54** — [profiling-tools](../requirements/tools/profiling-tools.md)
   - **Features:** [profiling-tools](../features/tools/profiling-tools.md)
   - **Prereqs:** CoreRuntime.ECS, Platform.Threading
6. **55** — [save-system](../requirements/game-framework/save-system.md)
   - **Features:** [save-system](../features/game-framework/save-system.md)
   - **Prereqs:** CoreRuntime.ReflectionSerialization, GameFramework.GameplayPrimitives
7. **56** — [abilities](../requirements/game-framework/abilities.md)
   - **Features:** [abilities](../features/game-framework/abilities.md)
   - **Prereqs:** GameFramework.GameplayPrimitives, Animation.StateMachine, Physics.SpatialQueries
8. **57** — [weapon-system](../requirements/game-framework/weapon-system.md)
   - **Features:** [weapon-system](../features/game-framework/weapon-system.md)
   - **Prereqs:** GameFramework.GameplayPrimitives, Physics.SpatialQueries, Animation.StateMachine
9. **58** — [progression](../requirements/game-framework/progression.md)
   - **Features:** [progression](../features/game-framework/progression.md)
   - **Prereqs:** GameFramework.GameplayPrimitives
10. **59** — [world-management](../requirements/game-framework/world-management.md)
    - **Features:** [world-management](../features/game-framework/world-management.md)
    - **Prereqs:** CoreRuntime.ECS, ContentPipeline.Streaming
11. **60** — [soft-body-and-cloth](../requirements/physics/soft-body-and-cloth.md)
    - **Features:** [soft-body-and-cloth](../features/physics/soft-body-and-cloth.md)
    - **Prereqs:** Physics.RigidBody, Animation.Skeletal
12. **61** — [destruction-and-fracture](../requirements/physics/destruction-and-fracture.md)
    - **Features:** [destruction-and-fracture](../features/physics/destruction-and-fracture.md)
    - **Prereqs:** Physics.RigidBody, Physics.Constraints
13. **62** — [water](../requirements/geometry/water.md)
    - **Features:** [water](../features/geometry/water.md)
    - **Prereqs:** Rendering.CoreRendering, Physics.RigidBody
14. **63** — [music-system](../requirements/audio/music-system.md)
    - **Features:** [music-system](../features/audio/music-system.md)
    - **Prereqs:** Audio.Engine, Audio.DSP
15. **64** — [voice-and-speech](../requirements/audio/voice-and-speech.md)
    - **Features:** [voice-and-speech](../features/audio/voice-and-speech.md)
    - **Prereqs:** Audio.Engine, Networking.Transport
16. **65** — [crowd-simulation](../requirements/ai/crowd-simulation.md),
    [tactical-combat](../requirements/ai/tactical-combat.md)
    - **Features:** [crowd-simulation](../features/ai/crowd-simulation.md),
      [tactical-combat](../features/ai/tactical-combat.md)
    - **Prereqs:** AI.BehaviorDecision, AI.Steering
17. **66** — [weather-environmental](../requirements/vfx/weather-environmental.md)
    - **Features:** [weather-environmental](../features/vfx/weather-environmental.md)
    - **Prereqs:** VFX.Particles, Audio.Spatial
18. **67** — [effect-graph](../requirements/vfx/effect-graph.md)
    - **Features:** [effect-graph](../features/vfx/effect-graph.md)
    - **Prereqs:** VFX.Particles, ToolsEditor.LogicGraph
19. **68** — [2d-games](../requirements/ui/2d-games.md)
    - **Features:** [2d-games](../features/ui/2d-games.md)
    - **Prereqs:** UI2D.WidgetFramework, Physics.RigidBody, Rendering.CoreRendering
20. **69** — [anti-cheat](../requirements/networking/anti-cheat.md)
    - **Features:** [anti-cheat](../features/networking/anti-cheat.md)
    - **Prereqs:** Networking.Replication, GameFramework.GameplayPrimitives

---

## Wave 6 — Capstone Systems

Requires specific Wave 5 nodes. 18 feature-groups can proceed concurrently.

| #  | Feature-Group                      | Design      | Impl        |
|----|------------------------------------|-------------|-------------|
| 70 | ToolsEditor.AnimationEditor        | Not started | Not started |
| 71 | ToolsEditor.WorldBuilding          | Not started | Not started |
| 72 | ToolsEditor.AdvancedTools          | Not started | Not started |
| 73 | ContentPipeline.DCCPlugins         | Not started | Not started |
| 74 | Animation.ClothHair                | Not started | Not started |
| 75 | Animation.FirstPerson              | Not started | Not started |
| 76 | VFX.Destruction                    | Not started | Not started |
| 77 | GameFramework.NPCSimulation        | Not started | Not started |
| 78 | GameFramework.Social               | Not started | Not started |
| 79 | GameFramework.GameModes            | Not started | Not started |
| 80 | GameFramework.StealthCover         | Not started | Not started |
| 81 | GameFramework.GenreSpecific        | Not started | Not started |
| 82 | GeometryWorld.TerrainVoxel         | Not started | Not started |
| 83 | GeometryWorld.ProceduralGeneration | Not started | Not started |
| 84 | Physics.Fluid                      | Not started | Not started |
| 85 | Networking.MMO                     | Not started | Not started |
| 86 | UI2D.Accessibility                 | Not started | Not started |
| 87 | Input.VRInput                      | Not started | Not started |

1. **70** — [animation-editor](../requirements/tools/animation-editor.md)
   - **Features:** [animation-editor](../features/tools/animation-editor.md)
   - **Prereqs:** ToolsEditor.EditorFramework, Animation.StateMachine
2. **71** — [world-building](../requirements/tools/world-building.md)
   - **Features:** [world-building](../features/tools/world-building.md)
   - **Prereqs:** ToolsEditor.EditorFramework, GeometryWorld.Terrain, GeometryWorld.Foliage
3. **72** — [version-control](../requirements/tools/version-control.md),
   [remote-editing](../requirements/tools/remote-editing.md),
   [ai-assistant](../requirements/tools/ai-assistant.md),
   [ai-governance](../requirements/tools/ai-governance.md),
   [deployment](../requirements/tools/deployment.md), [launcher](../requirements/tools/launcher.md),
   [localization-editor](../requirements/tools/localization-editor.md),
   [mod-support](../requirements/tools/mod-support.md),
   [shared-cache](../requirements/tools/shared-cache.md)
   - **Features:** [version-control](../features/tools/version-control.md),
     [remote-editing](../features/tools/remote-editing.md),
     [deployment](../features/tools/deployment.md), [launcher](../features/tools/launcher.md),
     [localization-editor](../features/tools/localization-editor.md),
     [mod-support](../features/tools/mod-support.md)
   - **Prereqs:** ToolsEditor.EditorFramework, Networking.Replication
4. **73** — [cloth-hair](../requirements/animation/cloth-hair.md)
   - **Features:** [cloth-hair](../features/animation/cloth-hair.md)
   - **Prereqs:** Physics.SoftBodyCloth, Audio.VoiceSpeech, Rendering.AdvancedRendering
5. **75** — [first-person](../requirements/animation/first-person.md)
   - **Features:** [first-person](../features/animation/first-person.md)
   - **Prereqs:** Animation.Skeletal, GameFramework.WeaponSystem
6. **76** — [destruction](../requirements/vfx/destruction.md)
   - **Features:** [destruction](../features/vfx/destruction.md)
   - **Prereqs:** Physics.Destruction, VFX.Particles
7. **77** — [npc-simulation](../requirements/game-framework/npc-simulation.md)
   - **Features:** [npc-simulation](../features/game-framework/npc-simulation.md)
   - **Prereqs:** AI.BehaviorDecision, GameFramework.GameplayPrimitives
8. **78** — [social](../requirements/game-framework/social.md)
   - **Features:** [social](../features/game-framework/social.md)
   - **Prereqs:** Networking.Session, GameFramework.GameplayPrimitives
9. **79** — [game-modes-misc](../requirements/game-framework/game-modes-misc.md)
   - **Features:** [game-modes-misc](../features/game-framework/game-modes-misc.md)
   - **Prereqs:** Networking.Session, GameFramework.GameplayPrimitives
10. **80** — [stealth-cover](../requirements/game-framework/stealth-cover.md)
    - **Features:** [stealth-cover](../features/game-framework/stealth-cover.md)
    - **Prereqs:** AI.Perception, GameFramework.GameplayPrimitives
11. **81** — [building-survival](../requirements/game-framework/building-survival.md),
    [racing](../requirements/game-framework/racing.md),
    [turn-based](../requirements/game-framework/turn-based.md),
    [fog-of-war](../requirements/game-framework/fog-of-war.md),
    [minigames](../requirements/game-framework/minigames.md),
    [pets-mounts](../requirements/game-framework/pets-mounts.md),
    [monetization](../requirements/game-framework/monetization.md),
    [traversal-interaction](../requirements/game-framework/traversal-interaction.md),
    [selection-system](../requirements/game-framework/selection-system.md)
    - **Features:** [building-survival](../features/game-framework/building-survival.md),
      [racing](../features/game-framework/racing.md),
      [turn-based](../features/game-framework/turn-based.md),
      [fog-of-war](../features/game-framework/fog-of-war.md),
      [minigames](../features/game-framework/minigames.md),
      [pets-mounts](../features/game-framework/pets-mounts.md),
      [monetization](../features/game-framework/monetization.md),
      [traversal-interaction](../features/game-framework/traversal-interaction.md),
      [selection-system](../features/game-framework/selection-system.md)
    - **Prereqs:** GameFramework.GameplayPrimitives, GameFramework.Abilities, Physics.RigidBody,
      AI.Navigation
12. **82** — [terrain](../requirements/geometry/terrain.md) (voxel section)
    - **Features:** [terrain](../features/geometry/terrain.md) (voxel section)
    - **Prereqs:** GeometryWorld.Terrain, Physics.Destruction
13. **83** —
    [procedural-generation](../requirements/geometry/procedural-generation.md)
    - **Features:**
      [procedural-generation](../features/geometry/procedural-generation.md)
    - **Prereqs:** GeometryWorld.Terrain, Rendering.AdvancedRendering, ContentPipeline.Streaming
14. **84** —
    [fluid-simulation](../requirements/physics/fluid-simulation.md)
    - **Features:**
      [fluid-simulation](../features/physics/fluid-simulation.md)
    - **Prereqs:** Physics.RigidBody, GeometryWorld.Water
15. **85** —
    [mmo-infrastructure](../requirements/networking/mmo-infrastructure.md)
    - **Features:**
      [mmo-infrastructure](../features/networking/mmo-infrastructure.md)
    - **Prereqs:** Networking.Replication, GameFramework.WorldManagement, GameFramework.SaveSystem
16. **86** — [accessibility](../requirements/ui/accessibility.md)
    - **Features:** [accessibility](../features/ui/accessibility.md)
    - **Prereqs:** UI2D.WidgetFramework, Audio.Engine, Input.ActionsMapping
17. **87** — [vr-input](../requirements/input/vr-input.md)
    - **Features:** [vr-input](../features/input/vr-input.md)
    - **Prereqs:** Input.DeviceAbstraction, Rendering.AdvancedRendering

---

## Requirements and Features Not Yet Mapped to Feature-Groups

The following requirement/feature docs exist but are not explicitly covered by the 87 feature-groups
above. They should be folded into existing groups or promoted to new groups.

### Unmapped Requirements

| # | Requirement File | Group |
|---|------------------|-------|
| 1 | cross-cutting | (see below) |
| 2 | animation/morph | (see below) |
| 3 | input/haptics-and-feedback | (see below) |
| 4 | physics/vehicle-physics | (see below) |
| 5 | networking/remote-procedure-calls | (see below) |
| 6 | networking/replay-system | (see below) |
| 7 | content-pipeline/asset-versioning | (see below) |
| 8 | geometry/sky-atmosphere | (see below) |
| 9 | game-framework/inventory | (see below) |
| 10 | game-framework/quest-dialogue | (see below) |
| 11 | game-framework/character-customization | (see below) |
| 12 | game-framework/cinematics | (see below) |
| 13 | game-framework/scripting | (see below) |
| 14 | game-framework/gameplay-databases | (see below) |
| 15 | game-framework/block-voxel | (see below) |
| 16 | tools/level-editor | (see below) |
| 17 | tools/asset-store | (see below) |
| 18 | tools/server-infrastructure | (see below) |
| 19 | ui/common-widgets | (see below) |
| 20 | ai/non-functional | (see below) |
| 21 | networking/non-functional | (see below) |
| 22 | rendering/gpu-abstraction | (see below) |

Candidate feature-group mappings:

1. [cross-cutting](../requirements/cross-cutting.md) — Spans all groups; constraints on all waves.
2. [animation/morph](../requirements/animation/morph.md) — Animation.Skeletal (morph targets are a
   skeletal subsystem).
3. [input/haptics-and-feedback](../requirements/input/haptics-and-feedback.md) —
   Input.DeviceAbstraction or Input.ActionsMapping.
4. [physics/vehicle-physics](../requirements/physics/vehicle-physics.md) — New group:
   Physics.Vehicles, or fold into GameFramework.GenreSpecific.
5. [networking/remote-procedure-calls](../requirements/networking/remote-procedure-calls.md) —
   Networking.Replication (RPCs are a replication mechanism).
6. [networking/replay-system](../requirements/networking/replay-system.md) —
   Networking.PredictionRollback (replay is a rollback byproduct).
7. [content-pipeline/asset-versioning](../requirements/content-pipeline/asset-versioning.md) —
   ContentPipeline.AssetDatabase.
8. [geometry/sky-atmosphere](../requirements/geometry/sky-atmosphere.md) —
   Rendering.AdvancedRendering (sky is a rendering feature) or new GeometryWorld group.
9. [game-framework/inventory](../requirements/game-framework/inventory.md) —
   GameFramework.GenreSpecific or GameFramework.GameplayPrimitives.
10. [game-framework/quest-dialogue](../requirements/game-framework/quest-dialogue.md) —
    GameFramework.GenreSpecific.
11. [game-framework/character-customization](../requirements/game-framework/character-customization.md)
    — GameFramework.GenreSpecific.
12. [game-framework/cinematics](../requirements/game-framework/cinematics.md) —
    GameFramework.GenreSpecific or ToolsEditor group.
13. [game-framework/scripting](../requirements/game-framework/scripting.md) — ToolsEditor.LogicGraph
    (scripting = logic graph in no-code engine).
14. [game-framework/gameplay-databases](../requirements/game-framework/gameplay-databases.md) —
    GameFramework.GameplayPrimitives.
15. [game-framework/block-voxel](../requirements/game-framework/block-voxel.md) —
    GeometryWorld.TerrainVoxel or GameFramework.GenreSpecific.
16. [tools/level-editor](../requirements/tools/level-editor.md) — ToolsEditor.EditorFramework or
    ToolsEditor.WorldBuilding.
17. [tools/asset-store](../requirements/tools/asset-store.md) — ToolsEditor.AdvancedTools.
18. [tools/server-infrastructure](../requirements/tools/server-infrastructure.md) —
    ToolsEditor.AdvancedTools.
19. [ui/common-widgets](../requirements/ui/common-widgets.md) — UI2D.WidgetFramework.
20. AI non-functional requirements — integrated into individual AI topic files.
21. Networking non-functional requirements — integrated into individual Networking topic files.
22. [rendering/gpu-abstraction](../requirements/rendering/gpu-abstraction.md) —
    Rendering.GPUAbstraction (possibly duplicate of gpu-abstraction-layer).

### Unmapped Features

| Candidate Feature-Group          |
|----------------------------------|
| Animation.Skeletal               |
| Input.DeviceAbstraction          |
| GameFramework.GenreSpecific      |
| Networking.Replication           |
| Networking.PredictionRollback    |
| ContentPipeline.AssetDatabase    |
| Rendering.AdvancedRendering      |
| GameFramework.GenreSpecific      |
| GameFramework.GenreSpecific      |
| GameFramework.GenreSpecific      |
| GameFramework.GenreSpecific      |
| ToolsEditor.LogicGraph           |
| GameFramework.GameplayPrimitives |
| GeometryWorld.TerrainVoxel       |
| ToolsEditor.WorldBuilding        |
| ToolsEditor.AdvancedTools        |
| ToolsEditor.AdvancedTools        |
| ToolsEditor.AdvancedTools        |
| ToolsEditor.AdvancedTools        |
| ToolsEditor.AdvancedTools        |
| UI2D.WidgetFramework             |
| Audio.DSP                        |
| Audio.MusicSystem                |

1. **[animation/morph](../features/animation/morph.md)** —
   [animation/morph](../features/animation/morph.md)
2. **[input/haptics-and-feedback](../features/input/haptics-and-feedback.md)** —
   [input/haptics-and-feedback](../features/input/haptics-and-feedback.md)
3. **[physics/vehicle-physics](../features/physics/vehicle-physics.md)** —
   [physics/vehicle-physics](../features/physics/vehicle-physics.md)
4. **[networking/remote-procedure-calls](../features/networking/remote-procedure-calls.md)** —
   [networking/remote-procedure-calls](../features/networking/remote-procedure-calls.md)
5. **[networking/replay-system](../features/networking/replay-system.md)** —
   [networking/replay-system](../features/networking/replay-system.md)
6. **[content-pipeline/asset-versioning](../features/content-pipeline/asset-versioning.md)** —
   [content-pipeline/asset-versioning](../features/content-pipeline/asset-versioning.md)
7. **[geometry/sky-atmosphere](../features/geometry/sky-atmosphere.md)** —
   [geometry/sky-atmosphere](../features/geometry/sky-atmosphere.md)
8. **[game-framework/inventory](../features/game-framework/inventory.md)** —
   [game-framework/inventory](../features/game-framework/inventory.md)
9. **[game-framework/quest-dialogue](../features/game-framework/quest-dialogue.md)** —
   [game-framework/quest-dialogue](../features/game-framework/quest-dialogue.md)
10.
**[game-framework/character-customization](../features/game-framework/character-customization.md)**
— [game-framework/character-customization](../features/game-framework/character-customization.md)
11. **[game-framework/cinematics](../features/game-framework/cinematics.md)** —
    [game-framework/cinematics](../features/game-framework/cinematics.md)
12. **[game-framework/scripting](../features/game-framework/scripting.md)** —
    [game-framework/scripting](../features/game-framework/scripting.md)
13. **[game-framework/gameplay-databases](../features/game-framework/gameplay-databases.md)** —
    [game-framework/gameplay-databases](../features/game-framework/gameplay-databases.md)
14. **[game-framework/block-voxel](../features/game-framework/block-voxel.md)** —
    [game-framework/block-voxel](../features/game-framework/block-voxel.md)
15. **[tools/level-editor](../features/tools/level-editor.md)** —
    [tools/level-editor](../features/tools/level-editor.md)
16. **[tools/asset-store](../features/tools/asset-store.md)** —
    [tools/asset-store](../features/tools/asset-store.md)
17. **[tools/server-infrastructure](../features/tools/server-infrastructure.md)** —
    [tools/server-infrastructure](../features/tools/server-infrastructure.md)
18. **[tools/ai-assistant](../features/tools/ai-assistant.md)** —
    [tools/ai-assistant](../features/tools/ai-assistant.md)
19. **[tools/ai-governance](../features/tools/ai-governance.md)** —
    [tools/ai-governance](../features/tools/ai-governance.md)
20. **[tools/shared-cache](../features/tools/shared-cache.md)** —
    [tools/shared-cache](../features/tools/shared-cache.md)
21. **[ui/common-widgets](../features/ui/common-widgets.md)** —
    [ui/common-widgets](../features/ui/common-widgets.md)
22. **[audio/dsp-and-effects](../features/audio/dsp-and-effects.md)** —
    [audio/dsp-and-effects](../features/audio/dsp-and-effects.md)
23. **[audio/music-system](../features/audio/music-system.md)** —
    [audio/music-system](../features/audio/music-system.md)
