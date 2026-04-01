# Domain Decomposition

Decomposes all 15 engine domains into feature-groups and maps cross-domain dependency edges to
maximize parallel development pipelines. Each feature-group is a node in a directed acyclic graph.
Edges represent "must exist before" relationships and can connect any node to any node regardless of
domain or complexity level.

## Constraints

1. **Interoperability:** Any two modules where a requirement dictates interoperation must share
   contracts (component schemas, event types, API traits) before either module builds against those
   contracts.
2. **All edge types:** Cross-domain dependencies exist in all four directions —
   foundational-to-foundational, advanced-to-advanced, foundational-to-advanced, and
   advanced-to-foundational.
3. **No-code enforcement:** All user-facing authoring surfaces must be visual (R-X.9.5).
4. **ECS-first:** All simulation data lives as components, all logic as systems.
5. **Shared spatial index:** Single BVH/octree shared across physics, rendering, networking, AI,
   gameplay.
6. **Platform-native async I/O:** No stdlib file I/O — all I/O via Tokio `current_thread` runtime.

---

## Feature-Groups by Domain

### 1. Platform (Domain 14)

| Group                  | Features                     |
|------------------------|------------------------------|
| **Platform.Windowing** | F-14.1.1–F-14.1.6            |
| **Platform.Threading** | F-14.3.1–F-14.3.5            |
| **Platform.OS**        | F-14.2.x, F-14.4.x, F-14.5.x |

1. ****Platform.Windowing**** — Window creation, display modes, DPI, multi-monitor
2. ****Platform.Threading**** — Work-stealing thread pool, task graph, async I/O integration
3. ****Platform.OS**** — OS integration, crash reporting, platform services, filesystem

**Outbound edges (what these unblock):**

| From | To | Reason |
|------|-----|--------|
| Platform.Windowing | Rendering.GPUAbstraction | Swapchain needs a window surface |
| Platform.Windowing | Input.DeviceAbstraction | Input events come from the window |
| Platform.Threading | CoreRuntime.ECS | Parallel system iteration needs the thread pool |
| Platform.Threading | CoreRuntime.AsyncIO | Async I/O dispatches onto platform threads |
| Platform.Threading | ContentPipeline.AssetImport | Import jobs run on the thread pool |
| Platform.OS | ContentPipeline.Streaming | Filesystem abstraction for asset loading |
| Platform.OS | Audio.Engine | Audio output device enumeration |

**Inbound edges (what these require):** None. Platform is a root node.

---

### 2. Core Runtime (Domain 1)

| Group                                   | Features         |
|-----------------------------------------|------------------|
| **CoreRuntime.ECS**                     | F-1.1.1–F-1.1.33 |
| **CoreRuntime.SceneTransforms**         | F-1.2.1–F-1.2.8  |
| **CoreRuntime.ReflectionSerialization** | F-1.3.x, F-1.4.x |
| **CoreRuntime.EventsPlugins**           | F-1.5.x, F-1.6.x |
| **CoreRuntime.MemoryAsyncIO**           | F-1.7.x, F-1.8.x |
| **CoreRuntime.SpatialIndex**            | F-1.9.x          |

1. ****CoreRuntime.ECS**** — Archetype storage, entities, queries, systems, commands, parallel
   iteration
2. ****CoreRuntime.SceneTransforms**** — Entity hierarchy, transform propagation, spatial
   partitioning delegation
3. ****CoreRuntime.ReflectionSerialization**** — Type registry, structured descriptors, binary/text
   serialization
4. ****CoreRuntime.EventsPlugins**** — Event channels, plugin system, dependency declaration
5. ****CoreRuntime.MemoryAsyncIO**** — Arena allocator, memory budgets, async file I/O abstraction
6. ****CoreRuntime.SpatialIndex**** — Shared BVH/octree — single acceleration structure for all
   domains

**Outbound edges:**

| From                                | To                               |
|-------------------------------------|----------------------------------|
| CoreRuntime.ECS                     | *All simulation domains*         |
| CoreRuntime.SpatialIndex            | Physics.RigidBody                |
| CoreRuntime.SpatialIndex            | Rendering.CoreRendering          |
| CoreRuntime.SpatialIndex            | Networking.Replication           |
| CoreRuntime.SpatialIndex            | AI.Navigation                    |
| CoreRuntime.SpatialIndex            | Audio.Spatial                    |
| CoreRuntime.SpatialIndex            | GameFramework.GameplayPrimitives |
| CoreRuntime.ReflectionSerialization | Networking.Replication           |
| CoreRuntime.ReflectionSerialization | ContentPipeline.AssetDatabase    |
| CoreRuntime.ReflectionSerialization | GameFramework.SaveSystem         |
| CoreRuntime.EventsPlugins           | *All domains*                    |
| CoreRuntime.MemoryAsyncIO           | ContentPipeline.Streaming        |
| CoreRuntime.SceneTransforms         | Rendering.CoreRendering          |
| CoreRuntime.SceneTransforms         | Physics.RigidBody                |
| CoreRuntime.SceneTransforms         | Animation.Skeletal               |

1. **CoreRuntime.ECS** — Every domain stores data as components and runs logic as systems
2. **CoreRuntime.SpatialIndex** — Broadphase collision detection
3. **CoreRuntime.SpatialIndex** — Frustum/occlusion culling
4. **CoreRuntime.SpatialIndex** — Area-of-interest filtering
5. **CoreRuntime.SpatialIndex** — Perception queries, crowd spatial queries
6. **CoreRuntime.SpatialIndex** — Occlusion ray-casts for spatial audio
7. **CoreRuntime.SpatialIndex** — Spatial queries for selection, AOE
8. **CoreRuntime.ReflectionSerialization** — Component serialization for replication
9. **CoreRuntime.ReflectionSerialization** — Asset metadata serialization
10. **CoreRuntime.ReflectionSerialization** — World state serialization
11. **CoreRuntime.EventsPlugins** — Inter-system communication via typed events
12. **CoreRuntime.MemoryAsyncIO** — File I/O for asset loading
13. **CoreRuntime.SceneTransforms** — World-space transforms for draw calls
14. **CoreRuntime.SceneTransforms** — Transform sync between physics and scene
15. **CoreRuntime.SceneTransforms** — Bone hierarchy as scene entities

**Inbound edges:**

| From | To | Reason |
|------|-----|--------|
| Platform.Threading | CoreRuntime.ECS | Thread pool for parallel iteration |
| Platform.Threading | CoreRuntime.MemoryAsyncIO | Platform async I/O dispatch |

---

### 3. Rendering (Domain 2)

| Group                           | Features          |
|---------------------------------|-------------------|
| **Rendering.GPUAbstraction**    | F-2.1.1–F-2.1.12  |
| **Rendering.RenderGraph**       | F-2.2.1–F-2.2.5   |
| **Rendering.CoreRendering**     | F-2.3.x, F-2.10.x |
| **Rendering.Lighting**          | F-2.4.x           |
| **Rendering.AdvancedRendering** | F-2.5.x–F-2.9.x   |

1. ****Rendering.GPUAbstraction**** — Backend trait, device, command buffers, memory, pipelines
   (Metal first, then Vulkan)
2. ****Rendering.RenderGraph**** — Pass declaration, resource management, barrier insertion
3. ****Rendering.CoreRendering**** — Culling, instancing, draw submission, ECS extraction, scene
   rendering pipeline
4. ****Rendering.Lighting**** — Forward+, PBR, shadows, global illumination
5. ****Rendering.AdvancedRendering**** — Ray tracing, advanced materials, character rendering,
   environment, post-processing, AA/upscaling

**Outbound edges:**

| From                        | To                                 |
|-----------------------------|------------------------------------|
| Rendering.GPUAbstraction    | Rendering.RenderGraph              |
| Rendering.GPUAbstraction    | ContentPipeline.AssetProcessing    |
| Rendering.RenderGraph       | Rendering.CoreRendering            |
| Rendering.RenderGraph       | VFX.Particles                      |
| Rendering.RenderGraph       | UI2D.UIRendering                   |
| Rendering.CoreRendering     | Rendering.Lighting                 |
| Rendering.CoreRendering     | GeometryWorld.Terrain              |
| Rendering.CoreRendering     | GeometryWorld.Water                |
| Rendering.Lighting          | Rendering.AdvancedRendering        |
| Rendering.AdvancedRendering | GeometryWorld.ProceduralGeneration |
| Rendering.AdvancedRendering | Animation.ClothHair                |

1. **Rendering.GPUAbstraction** — Render graph submits GPU commands via the abstraction
2. **Rendering.GPUAbstraction** — Shader compilation needs GPU pipeline layout
3. **Rendering.RenderGraph** — Core rendering inserts passes into the render graph
4. **Rendering.RenderGraph** — Particle rendering inserts passes into the render graph
5. **Rendering.RenderGraph** — UI rendering inserts a pass into the render graph
6. **Rendering.CoreRendering** — Lighting is a pass within the core rendering pipeline
7. **Rendering.CoreRendering** — Terrain renders through the core pipeline
8. **Rendering.CoreRendering** — Water renders through the core pipeline
9. **Rendering.Lighting** — Advanced rendering builds on the lighting pipeline
10. **Rendering.AdvancedRendering** — Planet-scale rendering needs advanced LOD and streaming
11. **Rendering.AdvancedRendering** — Hair/cloth rendering (strand rendering, translucency)

**Inbound edges:**

| From | To | Reason |
|------|-----|--------|
| Platform.Windowing | Rendering.GPUAbstraction | Window surface for swapchain |
| CoreRuntime.ECS | Rendering.CoreRendering | ECS extraction for render proxies |
| CoreRuntime.SpatialIndex | Rendering.CoreRendering | Frustum/occlusion culling |
| CoreRuntime.SceneTransforms | Rendering.CoreRendering | World-space transforms |
| ContentPipeline.AssetProcessing | Rendering.GPUAbstraction | Compiled shaders |
| Animation.Skeletal | Rendering.CoreRendering | GPU skinning matrices |
| GeometryWorld.MeshletPipeline | Rendering.CoreRendering | Meshlet GPU-driven rendering |

---

### 4. Content Pipeline (Domain 12)

| Group                               | Features |
|-------------------------------------|----------|
| **ContentPipeline.AssetImport**     | F-12.1.x |
| **ContentPipeline.AssetProcessing** | F-12.2.x |
| **ContentPipeline.AssetDatabase**   | F-12.3.x |
| **ContentPipeline.HotReload**       | F-12.4.x |
| **ContentPipeline.Streaming**       | F-12.5.x |
| **ContentPipeline.DCCPlugins**      | F-12.6.x |

1. ****ContentPipeline.AssetImport**** — Native format ingestion, texture/mesh/audio import,
   validation
2. ****ContentPipeline.AssetProcessing**** — Shader compilation (DXC + Metal Shader Converter),
   texture compression, mesh optimization
3. ****ContentPipeline.AssetDatabase**** — Content-addressable storage, metadata, dependency graph,
   incremental builds
4. ****ContentPipeline.HotReload**** — File watcher, asset/shader/logic graph hot reload
5. ****ContentPipeline.Streaming**** — Platform-native async I/O streaming, GPU direct storage,
   memory pressure
6. ****ContentPipeline.DCCPlugins**** — Native Blender, Maya, Houdini plugins for direct export

**Outbound edges:**

| From                            | To                              |
|---------------------------------|---------------------------------|
| ContentPipeline.AssetImport     | ContentPipeline.AssetProcessing |
| ContentPipeline.AssetProcessing | Rendering.GPUAbstraction        |
| ContentPipeline.AssetProcessing | ContentPipeline.AssetDatabase   |
| ContentPipeline.AssetDatabase   | ContentPipeline.HotReload       |
| ContentPipeline.AssetDatabase   | ContentPipeline.Streaming       |
| ContentPipeline.HotReload       | ToolsEditor.EditorFramework     |
| ContentPipeline.HotReload       | ToolsEditor.LogicGraph          |
| ContentPipeline.Streaming       | GeometryWorld.Terrain           |
| ContentPipeline.Streaming       | GeometryWorld.Foliage           |
| ContentPipeline.Streaming       | Audio.Engine                    |

1. **ContentPipeline.AssetImport** — Imported assets need processing
2. **ContentPipeline.AssetProcessing** — Compiled shaders for pipeline creation
3. **ContentPipeline.AssetProcessing** — Processed assets stored in CAS
4. **ContentPipeline.AssetDatabase** — Hot reload queries the asset database
5. **ContentPipeline.AssetDatabase** — Streaming loads from the asset database
6. **ContentPipeline.HotReload** — Editor needs hot reload for live iteration
7. **ContentPipeline.HotReload** — Logic graph hot reload
8. **ContentPipeline.Streaming** — Terrain LOD streaming
9. **ContentPipeline.Streaming** — Foliage instance streaming
10. **ContentPipeline.Streaming** — Audio streaming playback

**Inbound edges:**

| From                                | To                              |
|-------------------------------------|---------------------------------|
| Platform.Threading                  | ContentPipeline.AssetImport     |
| Platform.OS                         | ContentPipeline.Streaming       |
| CoreRuntime.ReflectionSerialization | ContentPipeline.AssetDatabase   |
| CoreRuntime.MemoryAsyncIO           | ContentPipeline.Streaming       |
| Rendering.GPUAbstraction            | ContentPipeline.AssetProcessing |

1. **Platform.Threading** — Import jobs on thread pool
2. **Platform.OS** — Filesystem abstraction
3. **CoreRuntime.ReflectionSerialization** — Asset metadata serialization
4. **CoreRuntime.MemoryAsyncIO** — Async file I/O
5. **Rendering.GPUAbstraction** — GPU pipeline layout for shader compilation

---

### 5. Physics (Domain 4)

| Group                      | Features         |
|----------------------------|------------------|
| **Physics.RigidBody**      | F-4.1.x, F-4.2.x |
| **Physics.Constraints**    | F-4.3.x          |
| **Physics.SpatialQueries** | F-4.4.x          |
| **Physics.SoftBodyCloth**  | F-4.7.x          |
| **Physics.Destruction**    | F-4.5.x          |
| **Physics.Fluid**          | F-4.6.x          |

1. ****Physics.RigidBody**** — Rigid body dynamics, collision detection, character controller,
   ECS-based
2. ****Physics.Constraints**** — Joints, ragdoll, motor constraints
3. ****Physics.SpatialQueries**** — Ray-cast, shape-cast, overlap queries via shared BVH
4. ****Physics.SoftBodyCloth**** — XPBD cloth simulation, soft body deformation
5. ****Physics.Destruction**** — Fracture, destruction, debris
6. ****Physics.Fluid**** — Fluid simulation, buoyancy

**Outbound edges:**

| From                   | To                               |
|------------------------|----------------------------------|
| Physics.RigidBody      | GameFramework.GameplayPrimitives |
| Physics.RigidBody      | Animation.Procedural             |
| Physics.RigidBody      | Networking.PredictionRollback    |
| Physics.Constraints    | Animation.Skeletal               |
| Physics.Constraints    | Physics.Destruction              |
| Physics.SpatialQueries | AI.Navigation                    |
| Physics.SpatialQueries | GameFramework.GameplayPrimitives |
| Physics.SoftBodyCloth  | Animation.ClothHair              |
| Physics.SoftBodyCloth  | Rendering.AdvancedRendering      |
| Physics.Destruction    | VFX.Destruction                  |
| Physics.Destruction    | GeometryWorld.TerrainVoxel       |
| Physics.Destruction    | AI.Navigation                    |
| Physics.Fluid          | GeometryWorld.Water              |

1. **Physics.RigidBody** — Character controller, movement, collision response
2. **Physics.RigidBody** — Root motion integration with physics
3. **Physics.RigidBody** — Deterministic physics for client prediction
4. **Physics.Constraints** — Ragdoll constraints on bone chains
5. **Physics.Constraints** — Destruction uses breakable constraints
6. **Physics.SpatialQueries** — Obstacle detection for steering
7. **Physics.SpatialQueries** — Ability targeting, line-of-sight
8. **Physics.SoftBodyCloth** — Cloth simulation drives cloth/hair vertex output
9. **Physics.SoftBodyCloth** — Cloth/hair rendering data
10. **Physics.Destruction** — Fracture events trigger destruction VFX
11. **Physics.Destruction** — Terrain voxel destruction
12. **Physics.Destruction** — NavMesh regeneration on destruction
13. **Physics.Fluid** — Buoyancy, fluid surface interaction

**Inbound edges:**

| From | To | Reason |
|------|-----|--------|
| CoreRuntime.ECS | Physics.RigidBody | All physics data as ECS components |
| CoreRuntime.SpatialIndex | Physics.RigidBody | Shared BVH for broadphase |
| CoreRuntime.SceneTransforms | Physics.RigidBody | Transform synchronization |
| Platform.Threading | Physics.RigidBody | Substep parallelization |
| GeometryWorld.Terrain | Physics.RigidBody | Terrain collision geometry |
| Animation.Skeletal | Physics.SoftBodyCloth | Skeleton drives cloth attachment points |

---

### 6. Input (Domain 6)

| Group | Features | Description |
|-------|----------|-------------|
| **Input.DeviceAbstraction** | F-6.1.x | Keyboard, mouse, gamepad, touch device abstraction |
| **Input.ActionsMapping** | F-6.2.x | Action mapping, key rebinding, combos, input contexts |
| **Input.Gestures** | F-6.3.x | Touch gestures, swipe, pinch, drag recognition |
| **Input.VRInput** | F-6.5.x | VR controller tracking, hand tracking, eye tracking |

**Outbound edges:**

| From                    | To                               |
|-------------------------|----------------------------------|
| Input.DeviceAbstraction | Input.ActionsMapping             |
| Input.ActionsMapping    | GameFramework.GameplayPrimitives |
| Input.ActionsMapping    | UI2D.WidgetFramework             |
| Input.ActionsMapping    | ToolsEditor.EditorFramework      |
| Input.Gestures          | UI2D.WidgetFramework             |
| Input.VRInput           | Rendering.AdvancedRendering      |
| Input.VRInput           | GameFramework.GameplayPrimitives |

1. **Input.DeviceAbstraction** — Actions map physical inputs to logical actions
2. **Input.ActionsMapping** — Ability activation, movement, interaction
3. **Input.ActionsMapping** — UI navigation and interaction
4. **Input.ActionsMapping** — Editor input handling
5. **Input.Gestures** — Touch-based UI interaction
6. **Input.VRInput** — Stereo rendering, tracked controller visualization
7. **Input.VRInput** — VR-specific gameplay interactions

**Inbound edges:**

| From | To | Reason |
|------|-----|--------|
| Platform.Windowing | Input.DeviceAbstraction | Input events from the window system |
| CoreRuntime.ECS | Input.DeviceAbstraction | Input state as ECS resources/events |

---

### 7. Audio (Domain 5)

| Group | Features | Description |
|-------|----------|-------------|
| **Audio.Engine** | F-5.1.x | Audio engine, mixer, streaming playback, sound sources |
| **Audio.Spatial** | F-5.2.x | 3D positioning, HRTF, occlusion, reverb zones |
| **Audio.DSP** | F-5.3.x | Parametric filters, effects chains, dynamic mixing |
| **Audio.VoiceSpeech** | F-5.5.x | Voice chat, echo cancellation, lip-sync data, jitter buffer |
| **Audio.MusicSystem** | F-5.4.x | Adaptive music, crossfading, layered tracks |

**Outbound edges:**

| From | To | Reason |
|------|-----|--------|
| Audio.Engine | Audio.Spatial | Spatial audio extends the engine |
| Audio.Engine | Audio.DSP | DSP processing in the mixer chain |
| Audio.Spatial | GameFramework.GameplayPrimitives | Gameplay-driven spatial audio triggers |
| Audio.VoiceSpeech | Animation.ClothHair | Lip-sync data drives facial animation |
| Audio.VoiceSpeech | Networking.Transport | Voice packets over network transport |

**Inbound edges:**

| From | To | Reason |
|------|-----|--------|
| Platform.OS | Audio.Engine | Audio output device |
| Platform.Threading | Audio.Engine | Real-time audio thread |
| CoreRuntime.ECS | Audio.Engine | Sound sources as ECS components |
| CoreRuntime.SpatialIndex | Audio.Spatial | Occlusion ray-casts |
| ContentPipeline.Streaming | Audio.Engine | Streaming audio playback |

---

### 8. Animation (Domain 9)

| Group                      | Features |
|----------------------------|----------|
| **Animation.Skeletal**     | F-9.1.x  |
| **Animation.StateMachine** | F-9.4.x  |
| **Animation.Procedural**   | F-9.3.x  |
| **Animation.ClothHair**    | F-9.5.x  |
| **Animation.FirstPerson**  | F-9.6.x  |

1. ****Animation.Skeletal**** — Skeletal evaluation, blending, GPU skinning, LOD
2. ****Animation.StateMachine**** — Blend trees, state transitions, animation events
3. ****Animation.Procedural**** — IK, ragdoll blend, locomotion matching, root motion
4. ****Animation.ClothHair**** — Cloth skinning integration, strand-based hair, LOD for cloth/hair
5. ****Animation.FirstPerson**** — First-person arm animation, weapon sway, ADS transitions

**Outbound edges:**

| From                   | To                               |
|------------------------|----------------------------------|
| Animation.Skeletal     | Rendering.CoreRendering          |
| Animation.Skeletal     | Physics.Constraints              |
| Animation.StateMachine | GameFramework.GameplayPrimitives |
| Animation.StateMachine | AI.BehaviorDecision              |
| Animation.Procedural   | Physics.RigidBody                |
| Animation.ClothHair    | Rendering.AdvancedRendering      |
| Animation.FirstPerson  | GameFramework.WeaponSystem       |

1. **Animation.Skeletal** — GPU skinning matrices for character rendering
2. **Animation.Skeletal** — Bone chains for ragdoll setup
3. **Animation.StateMachine** — Animation events trigger gameplay logic
4. **Animation.StateMachine** — AI selects animation states
5. **Animation.Procedural** — Root motion integration, IK foot placement on physics surfaces
6. **Animation.ClothHair** — Strand rendering, cloth vertex data
7. **Animation.FirstPerson** — First-person weapon animations

**Inbound edges:**

| From | To | Reason |
|------|-----|--------|
| CoreRuntime.ECS | Animation.Skeletal | Animation state as ECS components |
| CoreRuntime.SceneTransforms | Animation.Skeletal | Bone hierarchy |
| ContentPipeline.AssetImport | Animation.Skeletal | Animation clip import |
| Physics.SoftBodyCloth | Animation.ClothHair | XPBD simulation output drives vertex positions |
| Physics.Constraints | Animation.Procedural | Ragdoll constraint forces |
| Audio.VoiceSpeech | Animation.ClothHair | Lip-sync curves drive facial blend shapes |
| GeometryWorld.Terrain | Animation.Procedural | Terrain surface data for foot IK placement |

---

### 9. AI (Domain 7)

| Group                   | Features                  |
|-------------------------|---------------------------|
| **AI.Navigation**       | F-7.1.x                   |
| **AI.Steering**         | F-7.2.x                   |
| **AI.BehaviorDecision** | F-7.3.x, F-7.4.x, F-7.5.x |
| **AI.Perception**       | F-7.6.x                   |
| **AI.CrowdTactical**    | F-7.7.x, F-7.8.x          |

1. ****AI.Navigation**** — NavMesh generation, pathfinding, dynamic obstacles, streaming tiles
2. ****AI.Steering**** — Steering behaviors, local avoidance, formation movement
3. ****AI.BehaviorDecision**** — Behavior trees, utility AI, GOAP planning
4. ****AI.Perception**** — Sight, hearing, smell senses, perception queries via shared BVH
5. ****AI.CrowdTactical**** — Crowd simulation, tactical combat, LOD for AI

**Outbound edges:**

| From | To | Reason |
|------|-----|--------|
| AI.Navigation | GameFramework.GameplayPrimitives | NPC movement, pathfinding for gameplay |
| AI.Navigation | AI.Steering | Steering follows navigation paths |
| AI.BehaviorDecision | GameFramework.NPCSimulation | NPC behavior drives simulation |
| AI.BehaviorDecision | Animation.StateMachine | AI selects animation states |
| AI.Perception | GameFramework.StealthCover | Stealth depends on perception system |
| AI.Perception | AI.BehaviorDecision | Perception feeds into decision-making |
| AI.CrowdTactical | Networking.MMO | Crowd simulation in MMO contexts |

**Inbound edges:**

| From | To | Reason |
|------|-----|--------|
| CoreRuntime.ECS | AI.Navigation | NavMesh tiles as ECS resources |
| CoreRuntime.SpatialIndex | AI.Perception | Perception queries via shared BVH |
| Physics.SpatialQueries | AI.Navigation | Obstacle detection |
| Physics.Destruction | AI.Navigation | NavMesh regeneration on destruction |
| GeometryWorld.Terrain | AI.Navigation | NavMesh generation on terrain geometry |
| GameFramework.GameplayPrimitives | AI.BehaviorDecision | Gameplay state informs AI decisions |

---

### 10. Networking (Domain 8)

| Group                             | Features |
|-----------------------------------|----------|
| **Networking.Transport**          | F-8.1.x  |
| **Networking.Replication**        | F-8.2.x  |
| **Networking.PredictionRollback** | F-8.4.x  |
| **Networking.Session**            | F-8.5.x  |
| **Networking.MMO**                | F-8.7.x  |
| **Networking.AntiCheat**          | F-8.8.x  |

1. ****Networking.Transport**** — Connection management, reliable/unreliable channels, encryption
2. ****Networking.Replication**** — Component replication, authority, interest management via shared
   BVH
3. ****Networking.PredictionRollback**** — Client-side prediction, server reconciliation, rollback,
   jitter buffer
4. ****Networking.Session**** — Lobby, matchmaking, host migration
5. ****Networking.MMO**** — World sharding, dynamic server mesh, persistent world state
6. ****Networking.AntiCheat**** — Server authority validation, anomaly detection

**Outbound edges:**

| From | To | Reason |
|------|-----|--------|
| Networking.Transport | Networking.Replication | Replication uses transport channels |
| Networking.Transport | Audio.VoiceSpeech | Voice packets over transport |
| Networking.Replication | GameFramework.GameplayPrimitives | Replicated gameplay state |
| Networking.Replication | AI.CrowdTactical | NPC state replication |
| Networking.Session | GameFramework.GameModes | Session management for game modes |
| Networking.MMO | GeometryWorld.ProceduralGeneration | World streaming across server boundaries |
| Networking.MMO | GameFramework.Social | Cross-shard social features |

**Inbound edges:**

| From | To | Reason |
|------|-----|--------|
| CoreRuntime.ECS | Networking.Replication | Component data for replication |
| CoreRuntime.SpatialIndex | Networking.Replication | AOI filtering via shared BVH |
| CoreRuntime.ReflectionSerialization | Networking.Replication | Component serialization |
| Platform.Threading | Networking.Transport | Network I/O thread |
| Physics.RigidBody | Networking.PredictionRollback | Deterministic physics for prediction |
| GameFramework.SaveSystem | Networking.MMO | Persistent world state serialization |

---

### 11. UI & 2D (Domain 10)

| Group                    | Features           |
|--------------------------|--------------------|
| **UI2D.WidgetFramework** | F-10.1.x, F-10.2.x |
| **UI2D.UIRendering**     | F-10.4.x           |
| **UI2D.GameUI**          | F-10.3.x           |
| **UI2D.2DGames**         | F-10.5.x           |
| **UI2D.Accessibility**   | F-10.6.x           |

1. ****UI2D.WidgetFramework**** — Widget tree, layout, text rendering, common widgets
2. ****UI2D.UIRendering**** — Batched quad rendering, SDF text, atlas management
3. ****UI2D.GameUI**** — Health bars, nameplates, minimap, HUD elements
4. ****UI2D.2DGames**** — Sprite rendering, 2D physics, tilemaps, 2D camera, rollback netcode
5. ****UI2D.Accessibility**** — Screen reader, colorblind modes, input remapping

**Outbound edges:**

| From | To | Reason |
|------|-----|--------|
| UI2D.WidgetFramework | ToolsEditor.EditorFramework | Editor UI built on widget framework |
| UI2D.WidgetFramework | GameFramework.GameplayPrimitives | In-game UI for gameplay |
| UI2D.GameUI | GameFramework.GameplayPrimitives | HUD displays gameplay state |
| UI2D.2DGames | Networking.PredictionRollback | 2D rollback netcode |

**Inbound edges:**

| From                          | To                   |
|-------------------------------|----------------------|
| CoreRuntime.ECS               | UI2D.WidgetFramework |
| Rendering.RenderGraph         | UI2D.UIRendering     |
| Rendering.GPUAbstraction      | UI2D.UIRendering     |
| Input.ActionsMapping          | UI2D.WidgetFramework |
| Input.Gestures                | UI2D.WidgetFramework |
| ContentPipeline.HotReload     | UI2D.WidgetFramework |
| Physics.RigidBody             | UI2D.2DGames         |
| Networking.PredictionRollback | UI2D.2DGames         |

1. **CoreRuntime.ECS** — Widget tree as ECS entities
2. **Rendering.RenderGraph** — UI render pass in the render graph
3. **Rendering.GPUAbstraction** — GPU draw calls for UI
4. **Input.ActionsMapping** — UI input handling
5. **Input.Gestures** — Touch gestures for UI
6. **ContentPipeline.HotReload** — UI asset hot reload
7. **Physics.RigidBody** — 2D physics uses the physics system
8. **Networking.PredictionRollback** — Rollback requires deterministic physics (R-X.12.2)

---

### 12. Geometry & World (Domain 3)

| Group                                  | Features         |
|----------------------------------------|------------------|
| **GeometryWorld.MeshletPipeline**      | F-3.1.x          |
| **GeometryWorld.Terrain**              | F-3.2.1–F-3.2.8  |
| **GeometryWorld.TerrainVoxel**         | F-3.2.9–F-3.2.14 |
| **GeometryWorld.Foliage**              | F-3.3.x          |
| **GeometryWorld.Water**                | F-3.4.x          |
| **GeometryWorld.ProceduralGeneration** | F-3.6.x          |

1. ****GeometryWorld.MeshletPipeline**** — Meshlet generation, GPU-driven rendering, cluster culling
2. ****GeometryWorld.Terrain**** — Heightmap terrain, LOD, streaming, collision
3. ****GeometryWorld.TerrainVoxel**** — Voxel terrain, real-time destruction, caves
4. ****GeometryWorld.Foliage**** — Foliage instancing, LOD, wind, density painting
5. ****GeometryWorld.Water**** — Ocean simulation, rivers, buoyancy interaction
6. ****GeometryWorld.ProceduralGeneration**** — Planet-scale generation, universe pipeline, biome
   blending

**Outbound edges:**

| From                               |
|------------------------------------|
| GeometryWorld.MeshletPipeline      |
| GeometryWorld.Terrain              |
| GeometryWorld.Terrain              |
| GeometryWorld.Terrain              |
| GeometryWorld.TerrainVoxel         |
| GeometryWorld.TerrainVoxel         |
| GeometryWorld.Water                |
| GeometryWorld.Foliage              |
| GeometryWorld.ProceduralGeneration |
| GeometryWorld.ProceduralGeneration |

1. **GeometryWorld.MeshletPipeline** — Rendering.CoreRendering
   - **Reason:** Meshlet GPU-driven rendering pipeline
2. **GeometryWorld.Terrain** — Physics.RigidBody
   - **Reason:** Terrain collision geometry
3. **GeometryWorld.Terrain** — AI.Navigation
   - **Reason:** NavMesh generation on terrain
4. **GeometryWorld.Terrain** — Animation.Procedural
   - **Reason:** Surface data for foot IK
5. **GeometryWorld.TerrainVoxel** — Physics.Destruction
   - **Reason:** Voxel destruction integration
6. **GeometryWorld.TerrainVoxel** — AI.Navigation
   - **Reason:** NavMesh regen on voxel changes
7. **GeometryWorld.Water** — Physics.Fluid
   - **Reason:** Buoyancy volume interaction
8. **GeometryWorld.Foliage** — Rendering.CoreRendering
   - **Reason:** Foliage instanced rendering
9. **GeometryWorld.ProceduralGeneration** — Networking.MMO
   - **Reason:** World streaming across server boundaries
10. **GeometryWorld.ProceduralGeneration** — GameFramework.WorldManagement
    - **Reason:** Procedural world integration

**Inbound edges:**

| From | To | Reason |
|------|-----|--------|
| CoreRuntime.ECS | GeometryWorld.MeshletPipeline | Mesh data as ECS components |
| CoreRuntime.SpatialIndex | GeometryWorld.Foliage | Spatial queries for density |
| Rendering.CoreRendering | GeometryWorld.Terrain | Terrain rendering pipeline |
| Rendering.CoreRendering | GeometryWorld.Water | Water rendering pipeline |
| Rendering.AdvancedRendering | GeometryWorld.ProceduralGeneration | Planet-scale rendering |
| ContentPipeline.Streaming | GeometryWorld.Terrain | Terrain LOD streaming |
| ContentPipeline.Streaming | GeometryWorld.Foliage | Foliage instance streaming |
| Physics.Destruction | GeometryWorld.TerrainVoxel | Fracture triggers voxel removal |

---

### 13. VFX (Domain 11)

| Group | Features | Description |
|-------|----------|-------------|
| **VFX.Particles** | F-11.1.x | GPU particle simulation, emitters, forces, collision |
| **VFX.Decals** | F-11.2.x | Projected decals, atlasing, lifecycle management |
| **VFX.ScreenEffects** | F-11.3.x | Fullscreen effects, screen-space distortion |
| **VFX.Weather** | F-11.4.x | Rain, snow, fog, dynamic weather, wind integration |
| **VFX.Destruction** | F-11.5.x | Destruction particles, debris, dust clouds |
| **VFX.EffectGraph** | F-11.6.x | Visual effect composition graph, LOD, performance budget |

**Outbound edges:**

| From | To | Reason |
|------|-----|--------|
| VFX.Weather | Audio.Spatial | Weather affects ambient audio |
| VFX.Weather | AI.Perception | Weather affects perception ranges |
| VFX.EffectGraph | ToolsEditor.LogicGraph | Effect graph shares node runtime with logic graph |

**Inbound edges:**

| From | To | Reason |
|------|-----|--------|
| CoreRuntime.ECS | VFX.Particles | Particles as ECS entities |
| Rendering.RenderGraph | VFX.Particles | Particle render pass |
| Rendering.GPUAbstraction | VFX.Particles | GPU compute for simulation |
| Rendering.CoreRendering | VFX.ScreenEffects | Screen-space post-processing |
| Physics.Destruction | VFX.Destruction | Fracture events trigger VFX |
| Physics.RigidBody | VFX.Particles | Particle-physics collision |
| ContentPipeline.HotReload | VFX.EffectGraph | Effect asset hot reload |
| ToolsEditor.LogicGraph | VFX.EffectGraph | Shared visual graph runtime |

---

### 14. Game Framework (Domain 13)

| Group                                | Features               |
|--------------------------------------|------------------------|
| **GameFramework.GameplayPrimitives** | F-13.1.x               |
| **GameFramework.Camera**             | F-13.25.x              |
| **GameFramework.SaveSystem**         | F-13.3.x               |
| **GameFramework.Abilities**          | F-13.10.x              |
| **GameFramework.WeaponSystem**       | F-13.23.x              |
| **GameFramework.Progression**        | F-13.7.x               |
| **GameFramework.NPCSimulation**      | F-13.16.x              |
| **GameFramework.WorldManagement**    | F-13.2.x               |
| **GameFramework.Social**             | F-13.19.x              |
| **GameFramework.GameModes**          | F-13.22.x              |
| **GameFramework.StealthCover**       | F-13.20.x              |
| **GameFramework.GenreSpecific**      | F-13.11–F-13.21 (misc) |

1. ****GameFramework.GameplayPrimitives**** — Gameplay entity types, modular system registration,
   data-driven configuration
2. ****GameFramework.Camera**** — Third-person, first-person, RTS camera, collision avoidance
3. ****GameFramework.SaveSystem**** — Save/load serialization, checkpoints, migration
4. ****GameFramework.Abilities**** — Data-driven ability composition, cooldowns, targeting
5. ****GameFramework.WeaponSystem**** — Melee, ranged, projectile, hit detection
6. ****GameFramework.Progression**** — XP, leveling, skill trees, loot tables
7. ****GameFramework.NPCSimulation**** — NPC schedules, relationships, factions
8. ****GameFramework.WorldManagement**** — Multi-world, level streaming, instancing
9. ****GameFramework.Social**** — Guilds, chat, friends, trading
10. ****GameFramework.GameModes**** — Game mode lifecycle, scoring, match flow
11. ****GameFramework.StealthCover**** — Stealth detection, cover system, noise propagation
12. ****GameFramework.GenreSpecific**** — Building, racing, stealth, turn-based, survival, fog of
    war, minigames, pets/mounts

**Outbound edges:**

| From | To | Reason |
|------|-----|--------|
| GameFramework.GameplayPrimitives | AI.BehaviorDecision | Gameplay state informs AI |
| GameFramework.SaveSystem | Networking.MMO | Persistent world state |
| GameFramework.WorldManagement | GeometryWorld.ProceduralGeneration | World streaming integration |
| GameFramework.NPCSimulation | AI.BehaviorDecision | NPC schedules drive behavior trees |

**Inbound edges:**

| From | To | Reason |
|------|-----|--------|
| CoreRuntime.ECS | GameFramework.GameplayPrimitives | Gameplay data as ECS components |
| CoreRuntime.SpatialIndex | GameFramework.GameplayPrimitives | Spatial queries for selection, AOE |
| CoreRuntime.ReflectionSerialization | GameFramework.SaveSystem | World state serialization |
| Physics.RigidBody | GameFramework.GameplayPrimitives | Character controller, collision |
| Physics.SpatialQueries | GameFramework.GameplayPrimitives | Ability targeting |
| Input.ActionsMapping | GameFramework.GameplayPrimitives | Player input to gameplay actions |
| Audio.Spatial | GameFramework.GameplayPrimitives | Gameplay-triggered audio |
| Animation.StateMachine | GameFramework.GameplayPrimitives | Animation events for gameplay |
| AI.Navigation | GameFramework.GameplayPrimitives | NPC pathfinding |
| AI.BehaviorDecision | GameFramework.NPCSimulation | AI drives NPC behavior |
| AI.Perception | GameFramework.StealthCover | Perception feeds stealth system |
| Networking.Replication | GameFramework.GameplayPrimitives | Replicated gameplay state |
| Networking.Session | GameFramework.GameModes | Session drives game mode lifecycle |
| UI2D.WidgetFramework | GameFramework.GameplayPrimitives | In-game UI |
| UI2D.GameUI | GameFramework.GameplayPrimitives | HUD elements |

---

### 15. Tools & Editor (Domain 15)

| Group                           | Features       |
|---------------------------------|----------------|
| **ToolsEditor.EditorFramework** | F-15.1.x       |
| **ToolsEditor.LogicGraph**      | F-15.8.x       |
| **ToolsEditor.MaterialEditor**  | F-15.3.x       |
| **ToolsEditor.AnimationEditor** | F-15.2.x       |
| **ToolsEditor.WorldBuilding**   | F-15.4.x       |
| **ToolsEditor.Profiling**       | F-15.5.x       |
| **ToolsEditor.AdvancedTools**   | F-15.6–F-15.16 |

1. ****ToolsEditor.EditorFramework**** — Editor shell, viewport, entity list, property inspector,
   play mode
2. ****ToolsEditor.LogicGraph**** — No-code visual logic graph runtime and editor
3. ****ToolsEditor.MaterialEditor**** — Visual material/shader graph editor
4. ****ToolsEditor.AnimationEditor**** — Timeline, curve editor, state machine editor
5. ****ToolsEditor.WorldBuilding**** — Terrain painting, foliage placement, level tools
6. ****ToolsEditor.Profiling**** — CPU/GPU profiler, stat overlays, memory tracker
7. ****ToolsEditor.AdvancedTools**** — Version control, remote editing, AI assistant, deployment,
   launcher, mod support, localization, CRDT collaboration

**Outbound edges:**

| From                       |
|----------------------------|
| ToolsEditor.LogicGraph     |
| ToolsEditor.LogicGraph     |
| ToolsEditor.LogicGraph     |
| ToolsEditor.MaterialEditor |
| ToolsEditor.WorldBuilding  |
| ToolsEditor.WorldBuilding  |

1. **ToolsEditor.LogicGraph** — GameFramework.GameplayPrimitives
   - **Reason:** Logic graphs define gameplay behavior
2. **ToolsEditor.LogicGraph** — VFX.EffectGraph
   - **Reason:** Shared visual graph runtime
3. **ToolsEditor.LogicGraph** — AI.BehaviorDecision
   - **Reason:** Visual behavior tree editing
4. **ToolsEditor.MaterialEditor** — Rendering.Lighting
   - **Reason:** Material editing affects shading
5. **ToolsEditor.WorldBuilding** — GeometryWorld.Terrain
   - **Reason:** Terrain painting tools
6. **ToolsEditor.WorldBuilding** — GeometryWorld.Foliage
   - **Reason:** Foliage placement tools

**Inbound edges:**

| From                                |
|-------------------------------------|
| CoreRuntime.ECS                     |
| CoreRuntime.ReflectionSerialization |
| UI2D.WidgetFramework                |
| ContentPipeline.HotReload           |
| Input.ActionsMapping                |
| Rendering.CoreRendering             |
| ContentPipeline.HotReload           |

1. **CoreRuntime.ECS** — ToolsEditor.EditorFramework
   - **Reason:** World editing via ECS
2. **CoreRuntime.ReflectionSerialization** — ToolsEditor.EditorFramework
   - **Reason:** Property inspector via reflection
3. **UI2D.WidgetFramework** — ToolsEditor.EditorFramework
   - **Reason:** Editor UI built on widget framework
4. **ContentPipeline.HotReload** — ToolsEditor.EditorFramework
   - **Reason:** Live asset reload in editor
5. **Input.ActionsMapping** — ToolsEditor.EditorFramework
   - **Reason:** Editor input handling
6. **Rendering.CoreRendering** — ToolsEditor.EditorFramework
   - **Reason:** Viewport rendering
7. **ContentPipeline.HotReload** — ToolsEditor.LogicGraph
   - **Reason:** Logic graph hot reload

---

## Topological Build Order

Grouped into concurrency waves. Each wave contains all nodes whose prerequisites are satisfied by
completion of prior waves. The node count per wave represents maximum agent parallelism at that
stage.

### Wave 0 — Root (no prerequisites)

| # | Feature-Group | ~Features |
|---|---------------|-----------|
| 1 | Platform.Windowing | 6 |
| 2 | Platform.Threading | 5 |
| 3 | Platform.OS | 12 |

#### Max parallelism: 3 agents

---

### Wave 1 — Core Runtime (requires Wave 0)

| # | Feature-Group | ~Features |
|---|---------------|-----------|
| 4 | CoreRuntime.ECS | 33 |
| 5 | CoreRuntime.SceneTransforms | 8 |
| 6 | CoreRuntime.ReflectionSerialization | 14 |
| 7 | CoreRuntime.EventsPlugins | 12 |
| 8 | CoreRuntime.MemoryAsyncIO | 10 |
| 9 | CoreRuntime.SpatialIndex | 12 |

#### Max parallelism: 6 agents

Internal ordering: CoreRuntime.ECS should lead. SceneTransforms, EventsPlugins, and SpatialIndex
depend on ECS primitives. ReflectionSerialization and MemoryAsyncIO can proceed in parallel with
ECS.

---

### Wave 2 — Infrastructure Fan-Out (requires Wave 1)

| # | Feature-Group | ~Features | Key Prerequisite |
|---|---------------|-----------|------------------|
| 10 | Rendering.GPUAbstraction | 12 | Platform.Windowing, CoreRuntime.ECS |
| 11 | ContentPipeline.AssetImport | 10 | CoreRuntime.MemoryAsyncIO, Platform.Threading |
| 12 | ContentPipeline.AssetDatabase | 8 | CoreRuntime.ReflectionSerialization |
| 13 | Physics.RigidBody | 20 | CoreRuntime.ECS, CoreRuntime.SpatialIndex |
| 14 | Physics.SpatialQueries | 6 | CoreRuntime.SpatialIndex |
| 15 | Input.DeviceAbstraction | 8 | Platform.Windowing, CoreRuntime.ECS |
| 16 | Audio.Engine | 10 | Platform.OS, Platform.Threading, CoreRuntime.ECS |
| 17 | Animation.Skeletal | 12 | CoreRuntime.ECS, CoreRuntime.SceneTransforms |
| 18 | Networking.Transport | 8 | Platform.Threading, CoreRuntime.ECS |

#### Max parallelism: 9 agents

---

### Wave 3 — Intermediate Systems (requires specific Wave 2 nodes)

| #  | Feature-Group                   | ~Features |
|----|---------------------------------|-----------|
| 19 | Rendering.RenderGraph           | 5         |
| 20 | ContentPipeline.AssetProcessing | 12        |
| 21 | ContentPipeline.Streaming       | 8         |
| 22 | Input.ActionsMapping            | 10        |
| 23 | Audio.Spatial                   | 8         |
| 24 | Animation.StateMachine          | 8         |
| 25 | Physics.Constraints             | 8         |
| 26 | AI.Navigation                   | 10        |
| 27 | Networking.Replication          | 10        |
| 28 | GeometryWorld.MeshletPipeline   | 8         |
| 29 | UI2D.WidgetFramework            | 12        |
| 30 | VFX.Particles                   | 10        |

1. **19** — Rendering.GPUAbstraction
2. **20** — ContentPipeline.AssetImport, Rendering.GPUAbstraction
3. **21** — ContentPipeline.AssetDatabase, CoreRuntime.MemoryAsyncIO
4. **22** — Input.DeviceAbstraction
5. **23** — Audio.Engine, CoreRuntime.SpatialIndex
6. **24** — Animation.Skeletal
7. **25** — Physics.RigidBody
8. **26** — CoreRuntime.SpatialIndex, Physics.SpatialQueries
9. **27** — Networking.Transport, CoreRuntime.ReflectionSerialization, CoreRuntime.SpatialIndex
10. **28** — Rendering.GPUAbstraction, CoreRuntime.ECS
11. **29** — CoreRuntime.ECS, Input.DeviceAbstraction
12. **30** — Rendering.GPUAbstraction, CoreRuntime.ECS

#### Max parallelism: 12 agents

---

### Wave 4 — Integration Layer (requires specific Wave 3 nodes)

| #  | Feature-Group                    | ~Features |
|----|----------------------------------|-----------|
| 31 | Rendering.CoreRendering          | 15        |
| 32 | Rendering.Lighting               | 12        |
| 33 | ContentPipeline.HotReload        | 8         |
| 34 | AI.Steering                      | 8         |
| 35 | AI.Perception                    | 8         |
| 36 | AI.BehaviorDecision              | 15        |
| 37 | Animation.Procedural             | 10        |
| 38 | Audio.DSP                        | 8         |
| 39 | Networking.PredictionRollback    | 10        |
| 40 | Networking.Session               | 8         |
| 41 | GameFramework.GameplayPrimitives | 15        |
| 42 | GameFramework.Camera             | 8         |
| 43 | UI2D.UIRendering                 | 8         |
| 44 | UI2D.GameUI                      | 8         |
| 45 | VFX.Decals                       | 6         |
| 46 | VFX.ScreenEffects                | 6         |
| 47 | GeometryWorld.Terrain            | 12        |
| 48 | GeometryWorld.Foliage            | 8         |
| 49 | Input.Gestures                   | 6         |

1. **31** — Rendering.RenderGraph, CoreRuntime.SpatialIndex, CoreRuntime.SceneTransforms,
   Animation.Skeletal
2. **32** — Rendering.CoreRendering
3. **33** — ContentPipeline.AssetDatabase, ContentPipeline.AssetProcessing
4. **34** — AI.Navigation
5. **35** — CoreRuntime.SpatialIndex, AI.Navigation
6. **36** — AI.Perception
7. **37** — Animation.Skeletal, Physics.RigidBody, Physics.Constraints
8. **38** — Audio.Engine
9. **39** — Networking.Replication, Physics.RigidBody
10. **40** — Networking.Transport
11. **41** — CoreRuntime.ECS, CoreRuntime.SpatialIndex, Physics.RigidBody, Physics.SpatialQueries,
    Input.ActionsMapping
12. **42** — CoreRuntime.SceneTransforms, Physics.SpatialQueries
13. **43** — UI2D.WidgetFramework, Rendering.RenderGraph, Rendering.GPUAbstraction
14. **44** — UI2D.WidgetFramework, UI2D.UIRendering
15. **45** — Rendering.RenderGraph
16. **46** — Rendering.CoreRendering
17. **47** — Rendering.CoreRendering, ContentPipeline.Streaming, Physics.RigidBody
18. **48** — Rendering.CoreRendering, CoreRuntime.SpatialIndex, ContentPipeline.Streaming
19. **49** — Input.DeviceAbstraction

#### Max parallelism: 19 agents

---

### Wave 5 — Advanced Systems (requires specific Wave 4 nodes)

| #  | Feature-Group                 | ~Features |
|----|-------------------------------|-----------|
| 50 | Rendering.AdvancedRendering   | 40        |
| 51 | ToolsEditor.EditorFramework   | 15        |
| 52 | ToolsEditor.LogicGraph        | 14        |
| 53 | ToolsEditor.MaterialEditor    | 8         |
| 54 | ToolsEditor.Profiling         | 8         |
| 55 | GameFramework.SaveSystem      | 10        |
| 56 | GameFramework.Abilities       | 12        |
| 57 | GameFramework.WeaponSystem    | 10        |
| 58 | GameFramework.Progression     | 10        |
| 59 | GameFramework.WorldManagement | 10        |
| 60 | Physics.SoftBodyCloth         | 10        |
| 61 | Physics.Destruction           | 10        |
| 62 | GeometryWorld.Water           | 10        |
| 63 | Audio.MusicSystem             | 8         |
| 64 | Audio.VoiceSpeech             | 10        |
| 65 | AI.CrowdTactical              | 10        |
| 66 | VFX.Weather                   | 8         |
| 67 | VFX.EffectGraph               | 8         |
| 68 | UI2D.2DGames                  | 15        |
| 69 | Networking.AntiCheat          | 6         |

1. **50** — Rendering.Lighting, Rendering.CoreRendering
2. **51** — UI2D.WidgetFramework, CoreRuntime.ReflectionSerialization, ContentPipeline.HotReload,
   Rendering.CoreRendering, Input.ActionsMapping
3. **52** — CoreRuntime.ECS, CoreRuntime.EventsPlugins, ContentPipeline.HotReload
4. **53** — Rendering.Lighting, ContentPipeline.HotReload
5. **54** — CoreRuntime.ECS, Platform.Threading
6. **55** — CoreRuntime.ReflectionSerialization, GameFramework.GameplayPrimitives
7. **56** — GameFramework.GameplayPrimitives, Animation.StateMachine, Physics.SpatialQueries
8. **57** — GameFramework.GameplayPrimitives, Physics.SpatialQueries, Animation.StateMachine
9. **58** — GameFramework.GameplayPrimitives
10. **59** — CoreRuntime.ECS, ContentPipeline.Streaming
11. **60** — Physics.RigidBody, Animation.Skeletal
12. **61** — Physics.RigidBody, Physics.Constraints
13. **62** — Rendering.CoreRendering, Physics.RigidBody
14. **63** — Audio.Engine, Audio.DSP
15. **64** — Audio.Engine, Networking.Transport
16. **65** — AI.BehaviorDecision, AI.Steering
17. **66** — VFX.Particles, Audio.Spatial
18. **67** — VFX.Particles, ToolsEditor.LogicGraph
19. **68** — UI2D.WidgetFramework, Physics.RigidBody, Rendering.CoreRendering
20. **69** — Networking.Replication, GameFramework.GameplayPrimitives

#### Max parallelism: 20 agents

---

### Wave 6 — Capstone Systems (requires specific Wave 5 nodes)

| #  | Feature-Group                      | ~Features |
|----|------------------------------------|-----------|
| 70 | ToolsEditor.AnimationEditor        | 8         |
| 71 | ToolsEditor.WorldBuilding          | 10        |
| 72 | ToolsEditor.AdvancedTools          | 35        |
| 73 | ContentPipeline.DCCPlugins         | 10        |
| 74 | Animation.ClothHair                | 8         |
| 75 | Animation.FirstPerson              | 6         |
| 76 | VFX.Destruction                    | 8         |
| 77 | GameFramework.NPCSimulation        | 10        |
| 78 | GameFramework.Social               | 8         |
| 79 | GameFramework.GameModes            | 8         |
| 80 | GameFramework.StealthCover         | 8         |
| 81 | GameFramework.GenreSpecific        | 40        |
| 82 | GeometryWorld.TerrainVoxel         | 10        |
| 83 | GeometryWorld.ProceduralGeneration | 30        |
| 84 | Physics.Fluid                      | 8         |
| 85 | Networking.MMO                     | 10        |
| 86 | UI2D.Accessibility                 | 8         |
| 87 | Input.VRInput                      | 6         |

1. **70** — ToolsEditor.EditorFramework, Animation.StateMachine
2. **71** — ToolsEditor.EditorFramework, GeometryWorld.Terrain, GeometryWorld.Foliage
3. **72** — ToolsEditor.EditorFramework, Networking.Replication
4. **73** — ContentPipeline.AssetImport, ContentPipeline.AssetProcessing
5. **74** — Physics.SoftBodyCloth, Audio.VoiceSpeech, Rendering.AdvancedRendering
6. **75** — Animation.Skeletal, GameFramework.WeaponSystem
7. **76** — Physics.Destruction, VFX.Particles
8. **77** — AI.BehaviorDecision, GameFramework.GameplayPrimitives
9. **78** — Networking.Session, GameFramework.GameplayPrimitives
10. **79** — Networking.Session, GameFramework.GameplayPrimitives
11. **80** — AI.Perception, GameFramework.GameplayPrimitives
12. **81** — GameFramework.GameplayPrimitives, GameFramework.Abilities, Physics.RigidBody,
    AI.Navigation
13. **82** — GeometryWorld.Terrain, Physics.Destruction
14. **83** — GeometryWorld.Terrain, Rendering.AdvancedRendering, ContentPipeline.Streaming
15. **84** — Physics.RigidBody, GeometryWorld.Water
16. **85** — Networking.Replication, GameFramework.WorldManagement, GameFramework.SaveSystem
17. **86** — UI2D.WidgetFramework, Audio.Engine, Input.ActionsMapping
18. **87** — Input.DeviceAbstraction, Rendering.AdvancedRendering

#### Max parallelism: 18 agents

---

## Critical Path

The longest chain through the DAG determines the minimum calendar time.

```text
Platform.Threading → CoreRuntime.ECS → Rendering.GPUAbstraction → Rendering.RenderGraph
  → Rendering.CoreRendering → Rendering.Lighting → Rendering.AdvancedRendering
    → GeometryWorld.ProceduralGeneration
```

**Critical path length: 8 feature-groups across 7 waves.**

Secondary critical paths:

```text
Platform.Threading → CoreRuntime.ECS → Physics.RigidBody → Physics.Constraints
  → Animation.Procedural → ... → GameFramework.GenreSpecific

Platform.Threading → CoreRuntime.ECS → CoreRuntime.SpatialIndex → AI.Navigation
  → AI.Steering → AI.BehaviorDecision → AI.CrowdTactical
```

---

## Parallelism Profile

| Wave | Nodes | Max Concurrent Agents | Cumulative Feature-Groups Complete |
|------|-------|-----------------------|------------------------------------|
| 0 | 3 | 3 | 3 |
| 1 | 6 | 6 | 9 |
| 2 | 9 | 9 | 18 |
| 3 | 12 | 12 | 30 |
| 4 | 19 | 19 | 49 |
| 5 | 20 | 20 | 69 |
| 6 | 18 | 18 | 87 |

Total feature-groups: **87**

---

## Interoperability Contracts

The following contracts must be defined and locked before Wave 2 begins, as they are consumed by
multiple concurrent development pipelines:

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
| GPU backend trait               | Rendering.GPUAbstraction            |
| Asset handle type               | ContentPipeline.AssetDatabase       |
| Render graph pass API           | Rendering.RenderGraph               |

1. **Component trait + archetype API** — All domains
   - **Scope:** How to define, register, query, and mutate components
2. **Entity handle type** — All domains
   - **Scope:** Generational index format, validity checking
3. **System trait + scheduling API** — All domains
   - **Scope:** How to declare systems, dependencies, and access patterns
4. **Event channel API** — All domains
   - **Scope:** How to send and receive typed events
5. **Spatial index query API** — Physics, Rendering, Networking, AI, Audio, GameFramework
   - **Scope:** Ray-cast, shape-cast, overlap, frustum, k-nearest
6. **Serialization trait** — Networking, ContentPipeline, GameFramework
   - **Scope:** Binary/text encode/decode for components and assets
7. **Async I/O trait** — ContentPipeline, Platform
   - **Scope:** Read/write/cancel operations, completion callbacks
8. **Transform component schema** — Rendering, Physics, Animation, Audio
   - **Scope:** Position, rotation, scale, parent, world matrix
9. **GPU backend trait** — ContentPipeline.AssetProcessing, VFX, UI2D
   - **Scope:** Device, command buffer, pipeline, buffer, texture APIs
10. **Asset handle type** — All domains
    - **Scope:** Typed handle, load state, reference counting
11. **Render graph pass API** — VFX, UI2D, Rendering.*
    - **Scope:** How to declare passes, read/write resources, submit commands
