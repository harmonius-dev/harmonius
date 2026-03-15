# Domain Decomposition

Decomposes all 15 engine domains into feature-groups and maps cross-domain dependency edges to maximize parallel
development pipelines. Each feature-group is a node in a directed acyclic graph. Edges represent "must exist before"
relationships and can connect any node to any node regardless of domain or complexity level.

## Constraints

1. **Interoperability:** Any two modules where a requirement dictates interoperation must share contracts (component
   schemas, event types, API traits) before either module builds against those contracts.
2. **All edge types:** Cross-domain dependencies exist in all four directions — foundational-to-foundational,
   advanced-to-advanced, foundational-to-advanced, and advanced-to-foundational.
3. **No-code enforcement:** All user-facing authoring surfaces must be visual (R-X.9.5).
4. **ECS-first:** All simulation data lives as components, all logic as systems.
5. **Shared spatial index:** Single BVH/octree shared across physics, rendering, networking, AI, gameplay.
6. **Platform-native async I/O:** No stdlib file I/O — IOCP on Windows, GCD on macOS, io_uring on Linux.

---

## Feature-Groups by Domain

### 1. Platform (Domain 14)

| Group | Features | Description |
|-------|----------|-------------|
| **Platform.Windowing** | F-14.1.1–F-14.1.6 | Window creation, display modes, DPI, multi-monitor |
| **Platform.Threading** | F-14.3.1–F-14.3.5 | Work-stealing thread pool, task graph, async I/O integration |
| **Platform.OS** | F-14.2.x, F-14.4.x, F-14.5.x | OS integration, crash reporting, platform services, filesystem |

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

**Inbound edges (what these require):**
None. Platform is a root node.

---

### 2. Core Runtime (Domain 1)

| Group | Features | Description |
|-------|----------|-------------|
| **CoreRuntime.ECS** | F-1.1.1–F-1.1.33 | Archetype storage, entities, queries, systems, commands, parallel iteration |
| **CoreRuntime.SceneTransforms** | F-1.2.1–F-1.2.8 | Entity hierarchy, transform propagation, spatial partitioning delegation |
| **CoreRuntime.ReflectionSerialization** | F-1.3.x, F-1.4.x | Type registry, structured descriptors, binary/text serialization |
| **CoreRuntime.EventsPlugins** | F-1.5.x, F-1.6.x | Event channels, plugin system, dependency declaration |
| **CoreRuntime.MemoryAsyncIO** | F-1.7.x, F-1.8.x | Arena allocator, memory budgets, async file I/O abstraction |
| **CoreRuntime.SpatialIndex** | F-1.9.x | Shared BVH/octree — single acceleration structure for all domains |

**Outbound edges:**

| From | To | Reason |
|------|-----|--------|
| CoreRuntime.ECS | *All simulation domains* | Every domain stores data as components and runs logic as systems |
| CoreRuntime.SpatialIndex | Physics.RigidBody | Broadphase collision detection |
| CoreRuntime.SpatialIndex | Rendering.CoreRendering | Frustum/occlusion culling |
| CoreRuntime.SpatialIndex | Networking.Replication | Area-of-interest filtering |
| CoreRuntime.SpatialIndex | AI.Navigation | Perception queries, crowd spatial queries |
| CoreRuntime.SpatialIndex | Audio.Spatial | Occlusion ray-casts for spatial audio |
| CoreRuntime.SpatialIndex | GameFramework.GameplayPrimitives | Spatial queries for selection, AOE |
| CoreRuntime.ReflectionSerialization | Networking.Replication | Component serialization for replication |
| CoreRuntime.ReflectionSerialization | ContentPipeline.AssetDatabase | Asset metadata serialization |
| CoreRuntime.ReflectionSerialization | GameFramework.SaveSystem | World state serialization |
| CoreRuntime.EventsPlugins | *All domains* | Inter-system communication via typed events |
| CoreRuntime.MemoryAsyncIO | ContentPipeline.Streaming | File I/O for asset loading |
| CoreRuntime.SceneTransforms | Rendering.CoreRendering | World-space transforms for draw calls |
| CoreRuntime.SceneTransforms | Physics.RigidBody | Transform sync between physics and scene |
| CoreRuntime.SceneTransforms | Animation.Skeletal | Bone hierarchy as scene entities |

**Inbound edges:**

| From | To | Reason |
|------|-----|--------|
| Platform.Threading | CoreRuntime.ECS | Thread pool for parallel iteration |
| Platform.Threading | CoreRuntime.MemoryAsyncIO | Platform async I/O dispatch |

---

### 3. Rendering (Domain 2)

| Group | Features | Description |
|-------|----------|-------------|
| **Rendering.GPUAbstraction** | F-2.1.1–F-2.1.12 | Backend trait, device, command buffers, memory, pipelines (Metal first, then Vulkan) |
| **Rendering.RenderGraph** | F-2.2.1–F-2.2.5 | Pass declaration, resource management, barrier insertion |
| **Rendering.CoreRendering** | F-2.3.x, F-2.10.x | Culling, instancing, draw submission, ECS extraction, scene rendering pipeline |
| **Rendering.Lighting** | F-2.4.x | Forward+, PBR, shadows, global illumination |
| **Rendering.AdvancedRendering** | F-2.5.x–F-2.9.x | Ray tracing, advanced materials, character rendering, environment, post-processing, AA/upscaling |

**Outbound edges:**

| From | To | Reason |
|------|-----|--------|
| Rendering.GPUAbstraction | Rendering.RenderGraph | Render graph submits GPU commands via the abstraction |
| Rendering.GPUAbstraction | ContentPipeline.AssetProcessing | Shader compilation needs GPU pipeline layout |
| Rendering.RenderGraph | Rendering.CoreRendering | Core rendering inserts passes into the render graph |
| Rendering.RenderGraph | VFX.Particles | Particle rendering inserts passes into the render graph |
| Rendering.RenderGraph | UI2D.UIRendering | UI rendering inserts a pass into the render graph |
| Rendering.CoreRendering | Rendering.Lighting | Lighting is a pass within the core rendering pipeline |
| Rendering.CoreRendering | GeometryWorld.Terrain | Terrain renders through the core pipeline |
| Rendering.CoreRendering | GeometryWorld.Water | Water renders through the core pipeline |
| Rendering.Lighting | Rendering.AdvancedRendering | Advanced rendering builds on the lighting pipeline |
| Rendering.AdvancedRendering | GeometryWorld.ProceduralGeneration | Planet-scale rendering needs advanced LOD and streaming |
| Rendering.AdvancedRendering | Animation.ClothHair | Hair/cloth rendering (strand rendering, translucency) |

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

| Group | Features | Description |
|-------|----------|-------------|
| **ContentPipeline.AssetImport** | F-12.1.x | Native format ingestion, texture/mesh/audio import, validation |
| **ContentPipeline.AssetProcessing** | F-12.2.x | Shader compilation (DXC + Metal Shader Converter), texture compression, mesh optimization |
| **ContentPipeline.AssetDatabase** | F-12.3.x | Content-addressable storage, metadata, dependency graph, incremental builds |
| **ContentPipeline.HotReload** | F-12.4.x | File watcher, asset/shader/logic graph hot reload |
| **ContentPipeline.Streaming** | F-12.5.x | Platform-native async I/O streaming, GPU direct storage, memory pressure |
| **ContentPipeline.DCCPlugins** | F-12.6.x | Native Blender, Maya, Houdini plugins for direct export |

**Outbound edges:**

| From | To | Reason |
|------|-----|--------|
| ContentPipeline.AssetImport | ContentPipeline.AssetProcessing | Imported assets need processing |
| ContentPipeline.AssetProcessing | Rendering.GPUAbstraction | Compiled shaders for pipeline creation |
| ContentPipeline.AssetProcessing | ContentPipeline.AssetDatabase | Processed assets stored in CAS |
| ContentPipeline.AssetDatabase | ContentPipeline.HotReload | Hot reload queries the asset database |
| ContentPipeline.AssetDatabase | ContentPipeline.Streaming | Streaming loads from the asset database |
| ContentPipeline.HotReload | ToolsEditor.EditorFramework | Editor needs hot reload for live iteration |
| ContentPipeline.HotReload | ToolsEditor.LogicGraph | Logic graph hot reload |
| ContentPipeline.Streaming | GeometryWorld.Terrain | Terrain LOD streaming |
| ContentPipeline.Streaming | GeometryWorld.Foliage | Foliage instance streaming |
| ContentPipeline.Streaming | Audio.Engine | Audio streaming playback |

**Inbound edges:**

| From | To | Reason |
|------|-----|--------|
| Platform.Threading | ContentPipeline.AssetImport | Import jobs on thread pool |
| Platform.OS | ContentPipeline.Streaming | Filesystem abstraction |
| CoreRuntime.ReflectionSerialization | ContentPipeline.AssetDatabase | Asset metadata serialization |
| CoreRuntime.MemoryAsyncIO | ContentPipeline.Streaming | Async file I/O |
| Rendering.GPUAbstraction | ContentPipeline.AssetProcessing | GPU pipeline layout for shader compilation |

---

### 5. Physics (Domain 4)

| Group | Features | Description |
|-------|----------|-------------|
| **Physics.RigidBody** | F-4.1.x, F-4.2.x | Rigid body dynamics, collision detection, character controller, ECS-based |
| **Physics.Constraints** | F-4.3.x | Joints, ragdoll, motor constraints |
| **Physics.SpatialQueries** | F-4.4.x | Ray-cast, shape-cast, overlap queries via shared BVH |
| **Physics.SoftBodyCloth** | F-4.7.x | XPBD cloth simulation, soft body deformation |
| **Physics.Destruction** | F-4.5.x | Fracture, destruction, debris |
| **Physics.Fluid** | F-4.6.x | Fluid simulation, buoyancy |

**Outbound edges:**

| From | To | Reason |
|------|-----|--------|
| Physics.RigidBody | GameFramework.GameplayPrimitives | Character controller, movement, collision response |
| Physics.RigidBody | Animation.Procedural | Root motion integration with physics |
| Physics.RigidBody | Networking.PredictionRollback | Deterministic physics for client prediction |
| Physics.Constraints | Animation.Skeletal | Ragdoll constraints on bone chains |
| Physics.Constraints | Physics.Destruction | Destruction uses breakable constraints |
| Physics.SpatialQueries | AI.Navigation | Obstacle detection for steering |
| Physics.SpatialQueries | GameFramework.GameplayPrimitives | Ability targeting, line-of-sight |
| Physics.SoftBodyCloth | Animation.ClothHair | Cloth simulation drives cloth/hair vertex output |
| Physics.SoftBodyCloth | Rendering.AdvancedRendering | Cloth/hair rendering data |
| Physics.Destruction | VFX.Destruction | Fracture events trigger destruction VFX |
| Physics.Destruction | GeometryWorld.TerrainVoxel | Terrain voxel destruction |
| Physics.Destruction | AI.Navigation | NavMesh regeneration on destruction |
| Physics.Fluid | GeometryWorld.Water | Buoyancy, fluid surface interaction |

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

| From | To | Reason |
|------|-----|--------|
| Input.DeviceAbstraction | Input.ActionsMapping | Actions map physical inputs to logical actions |
| Input.ActionsMapping | GameFramework.GameplayPrimitives | Ability activation, movement, interaction |
| Input.ActionsMapping | UI2D.WidgetFramework | UI navigation and interaction |
| Input.ActionsMapping | ToolsEditor.EditorFramework | Editor input handling |
| Input.Gestures | UI2D.WidgetFramework | Touch-based UI interaction |
| Input.VRInput | Rendering.AdvancedRendering | Stereo rendering, tracked controller visualization |
| Input.VRInput | GameFramework.GameplayPrimitives | VR-specific gameplay interactions |

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

| Group | Features | Description |
|-------|----------|-------------|
| **Animation.Skeletal** | F-9.1.x | Skeletal evaluation, blending, GPU skinning, LOD |
| **Animation.StateMachine** | F-9.4.x | Blend trees, state transitions, animation events |
| **Animation.Procedural** | F-9.3.x | IK, ragdoll blend, locomotion matching, root motion |
| **Animation.ClothHair** | F-9.5.x | Cloth skinning integration, strand-based hair, LOD for cloth/hair |
| **Animation.FirstPerson** | F-9.6.x | First-person arm animation, weapon sway, ADS transitions |

**Outbound edges:**

| From | To | Reason |
|------|-----|--------|
| Animation.Skeletal | Rendering.CoreRendering | GPU skinning matrices for character rendering |
| Animation.Skeletal | Physics.Constraints | Bone chains for ragdoll setup |
| Animation.StateMachine | GameFramework.GameplayPrimitives | Animation events trigger gameplay logic |
| Animation.StateMachine | AI.BehaviorDecision | AI selects animation states |
| Animation.Procedural | Physics.RigidBody | Root motion integration, IK foot placement on physics surfaces |
| Animation.ClothHair | Rendering.AdvancedRendering | Strand rendering, cloth vertex data |
| Animation.FirstPerson | GameFramework.WeaponSystem | First-person weapon animations |

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

| Group | Features | Description |
|-------|----------|-------------|
| **AI.Navigation** | F-7.1.x | NavMesh generation, pathfinding, dynamic obstacles, streaming tiles |
| **AI.Steering** | F-7.2.x | Steering behaviors, local avoidance, formation movement |
| **AI.BehaviorDecision** | F-7.3.x, F-7.4.x, F-7.5.x | Behavior trees, utility AI, GOAP planning |
| **AI.Perception** | F-7.6.x | Sight, hearing, smell senses, perception queries via shared BVH |
| **AI.CrowdTactical** | F-7.7.x, F-7.8.x | Crowd simulation, tactical combat, LOD for AI |

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

| Group | Features | Description |
|-------|----------|-------------|
| **Networking.Transport** | F-8.1.x | Connection management, reliable/unreliable channels, encryption |
| **Networking.Replication** | F-8.2.x | Component replication, authority, interest management via shared BVH |
| **Networking.PredictionRollback** | F-8.4.x | Client-side prediction, server reconciliation, rollback, jitter buffer |
| **Networking.Session** | F-8.5.x | Lobby, matchmaking, host migration |
| **Networking.MMO** | F-8.7.x | World sharding, dynamic server mesh, persistent world state |
| **Networking.AntiCheat** | F-8.8.x | Server authority validation, anomaly detection |

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

| Group | Features | Description |
|-------|----------|-------------|
| **UI2D.WidgetFramework** | F-10.1.x, F-10.2.x | Widget tree, layout, text rendering, common widgets |
| **UI2D.UIRendering** | F-10.4.x | Batched quad rendering, SDF text, atlas management |
| **UI2D.GameUI** | F-10.3.x | Health bars, nameplates, minimap, HUD elements |
| **UI2D.2DGames** | F-10.5.x | Sprite rendering, 2D physics, tilemaps, 2D camera, rollback netcode |
| **UI2D.Accessibility** | F-10.6.x | Screen reader, colorblind modes, input remapping |

**Outbound edges:**

| From | To | Reason |
|------|-----|--------|
| UI2D.WidgetFramework | ToolsEditor.EditorFramework | Editor UI built on widget framework |
| UI2D.WidgetFramework | GameFramework.GameplayPrimitives | In-game UI for gameplay |
| UI2D.GameUI | GameFramework.GameplayPrimitives | HUD displays gameplay state |
| UI2D.2DGames | Networking.PredictionRollback | 2D rollback netcode |

**Inbound edges:**

| From | To | Reason |
|------|-----|--------|
| CoreRuntime.ECS | UI2D.WidgetFramework | Widget tree as ECS entities |
| Rendering.RenderGraph | UI2D.UIRendering | UI render pass in the render graph |
| Rendering.GPUAbstraction | UI2D.UIRendering | GPU draw calls for UI |
| Input.ActionsMapping | UI2D.WidgetFramework | UI input handling |
| Input.Gestures | UI2D.WidgetFramework | Touch gestures for UI |
| ContentPipeline.HotReload | UI2D.WidgetFramework | UI asset hot reload |
| Physics.RigidBody | UI2D.2DGames | 2D physics uses the physics system |
| Networking.PredictionRollback | UI2D.2DGames | Rollback requires deterministic physics (R-X.12.2) |

---

### 12. Geometry & World (Domain 3)

| Group | Features | Description |
|-------|----------|-------------|
| **GeometryWorld.MeshletPipeline** | F-3.1.x | Meshlet generation, GPU-driven rendering, cluster culling |
| **GeometryWorld.Terrain** | F-3.2.1–F-3.2.8 | Heightmap terrain, LOD, streaming, collision |
| **GeometryWorld.TerrainVoxel** | F-3.2.9–F-3.2.14 | Voxel terrain, real-time destruction, caves |
| **GeometryWorld.Foliage** | F-3.3.x | Foliage instancing, LOD, wind, density painting |
| **GeometryWorld.Water** | F-3.4.x | Ocean simulation, rivers, buoyancy interaction |
| **GeometryWorld.ProceduralGeneration** | F-3.6.x | Planet-scale generation, universe pipeline, biome blending |

**Outbound edges:**

| From | To | Reason |
|------|-----|--------|
| GeometryWorld.MeshletPipeline | Rendering.CoreRendering | Meshlet GPU-driven rendering pipeline |
| GeometryWorld.Terrain | Physics.RigidBody | Terrain collision geometry |
| GeometryWorld.Terrain | AI.Navigation | NavMesh generation on terrain |
| GeometryWorld.Terrain | Animation.Procedural | Surface data for foot IK |
| GeometryWorld.TerrainVoxel | Physics.Destruction | Voxel destruction integration |
| GeometryWorld.TerrainVoxel | AI.Navigation | NavMesh regen on voxel changes |
| GeometryWorld.Water | Physics.Fluid | Buoyancy volume interaction |
| GeometryWorld.Foliage | Rendering.CoreRendering | Foliage instanced rendering |
| GeometryWorld.ProceduralGeneration | Networking.MMO | World streaming across server boundaries |
| GeometryWorld.ProceduralGeneration | GameFramework.WorldManagement | Procedural world integration |

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

| Group | Features | Description |
|-------|----------|-------------|
| **GameFramework.GameplayPrimitives** | F-13.1.x | Gameplay entity types, modular system registration, data-driven configuration |
| **GameFramework.Camera** | F-13.25.x | Third-person, first-person, RTS camera, collision avoidance |
| **GameFramework.SaveSystem** | F-13.3.x | Save/load serialization, checkpoints, migration |
| **GameFramework.Abilities** | F-13.10.x | Data-driven ability composition, cooldowns, targeting |
| **GameFramework.WeaponSystem** | F-13.23.x | Melee, ranged, projectile, hit detection |
| **GameFramework.Progression** | F-13.7.x | XP, leveling, skill trees, loot tables |
| **GameFramework.NPCSimulation** | F-13.16.x | NPC schedules, relationships, factions |
| **GameFramework.WorldManagement** | F-13.2.x | Multi-world, level streaming, instancing |
| **GameFramework.Social** | F-13.19.x | Guilds, chat, friends, trading |
| **GameFramework.GameModes** | F-13.22.x | Game mode lifecycle, scoring, match flow |
| **GameFramework.StealthCover** | F-13.20.x | Stealth detection, cover system, noise propagation |
| **GameFramework.GenreSpecific** | F-13.11–F-13.21 (misc) | Building, racing, stealth, turn-based, survival, fog of war, minigames, pets/mounts |

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

| Group | Features | Description |
|-------|----------|-------------|
| **ToolsEditor.EditorFramework** | F-15.1.x | Editor shell, viewport, entity list, property inspector, play mode |
| **ToolsEditor.LogicGraph** | F-15.8.x | No-code visual logic graph runtime and editor |
| **ToolsEditor.MaterialEditor** | F-15.3.x | Visual material/shader graph editor |
| **ToolsEditor.AnimationEditor** | F-15.2.x | Timeline, curve editor, state machine editor |
| **ToolsEditor.WorldBuilding** | F-15.4.x | Terrain painting, foliage placement, level tools |
| **ToolsEditor.Profiling** | F-15.5.x | CPU/GPU profiler, stat overlays, memory tracker |
| **ToolsEditor.AdvancedTools** | F-15.6–F-15.16 | Version control, remote editing, AI assistant, deployment, launcher, mod support, localization, CRDT collaboration |

**Outbound edges:**

| From | To | Reason |
|------|-----|--------|
| ToolsEditor.LogicGraph | GameFramework.GameplayPrimitives | Logic graphs define gameplay behavior |
| ToolsEditor.LogicGraph | VFX.EffectGraph | Shared visual graph runtime |
| ToolsEditor.LogicGraph | AI.BehaviorDecision | Visual behavior tree editing |
| ToolsEditor.MaterialEditor | Rendering.Lighting | Material editing affects shading |
| ToolsEditor.WorldBuilding | GeometryWorld.Terrain | Terrain painting tools |
| ToolsEditor.WorldBuilding | GeometryWorld.Foliage | Foliage placement tools |

**Inbound edges:**

| From | To | Reason |
|------|-----|--------|
| CoreRuntime.ECS | ToolsEditor.EditorFramework | World editing via ECS |
| CoreRuntime.ReflectionSerialization | ToolsEditor.EditorFramework | Property inspector via reflection |
| UI2D.WidgetFramework | ToolsEditor.EditorFramework | Editor UI built on widget framework |
| ContentPipeline.HotReload | ToolsEditor.EditorFramework | Live asset reload in editor |
| Input.ActionsMapping | ToolsEditor.EditorFramework | Editor input handling |
| Rendering.CoreRendering | ToolsEditor.EditorFramework | Viewport rendering |
| ContentPipeline.HotReload | ToolsEditor.LogicGraph | Logic graph hot reload |

---

## Topological Build Order

Grouped into concurrency waves. Each wave contains all nodes whose prerequisites are satisfied by completion of
prior waves. The node count per wave represents maximum agent parallelism at that stage.

### Wave 0 — Root (no prerequisites)

| # | Feature-Group | ~Features |
|---|---------------|-----------|
| 1 | Platform.Windowing | 6 |
| 2 | Platform.Threading | 5 |
| 3 | Platform.OS | 12 |

**Max parallelism: 3 agents**

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

**Max parallelism: 6 agents**

Internal ordering: CoreRuntime.ECS should lead. SceneTransforms, EventsPlugins, and SpatialIndex depend on ECS
primitives. ReflectionSerialization and MemoryAsyncIO can proceed in parallel with ECS.

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

**Max parallelism: 9 agents**

---

### Wave 3 — Intermediate Systems (requires specific Wave 2 nodes)

| # | Feature-Group | ~Features | Key Prerequisite |
|---|---------------|-----------|------------------|
| 19 | Rendering.RenderGraph | 5 | Rendering.GPUAbstraction |
| 20 | ContentPipeline.AssetProcessing | 12 | ContentPipeline.AssetImport, Rendering.GPUAbstraction |
| 21 | ContentPipeline.Streaming | 8 | ContentPipeline.AssetDatabase, CoreRuntime.MemoryAsyncIO |
| 22 | Input.ActionsMapping | 10 | Input.DeviceAbstraction |
| 23 | Audio.Spatial | 8 | Audio.Engine, CoreRuntime.SpatialIndex |
| 24 | Animation.StateMachine | 8 | Animation.Skeletal |
| 25 | Physics.Constraints | 8 | Physics.RigidBody |
| 26 | AI.Navigation | 10 | CoreRuntime.SpatialIndex, Physics.SpatialQueries |
| 27 | Networking.Replication | 10 | Networking.Transport, CoreRuntime.ReflectionSerialization, CoreRuntime.SpatialIndex |
| 28 | GeometryWorld.MeshletPipeline | 8 | Rendering.GPUAbstraction, CoreRuntime.ECS |
| 29 | UI2D.WidgetFramework | 12 | CoreRuntime.ECS, Input.DeviceAbstraction |
| 30 | VFX.Particles | 10 | Rendering.GPUAbstraction, CoreRuntime.ECS |

**Max parallelism: 12 agents**

---

### Wave 4 — Integration Layer (requires specific Wave 3 nodes)

| # | Feature-Group | ~Features | Key Prerequisite |
|---|---------------|-----------|------------------|
| 31 | Rendering.CoreRendering | 15 | Rendering.RenderGraph, CoreRuntime.SpatialIndex, CoreRuntime.SceneTransforms, Animation.Skeletal |
| 32 | Rendering.Lighting | 12 | Rendering.CoreRendering |
| 33 | ContentPipeline.HotReload | 8 | ContentPipeline.AssetDatabase, ContentPipeline.AssetProcessing |
| 34 | AI.Steering | 8 | AI.Navigation |
| 35 | AI.Perception | 8 | CoreRuntime.SpatialIndex, AI.Navigation |
| 36 | AI.BehaviorDecision | 15 | AI.Perception |
| 37 | Animation.Procedural | 10 | Animation.Skeletal, Physics.RigidBody, Physics.Constraints |
| 38 | Audio.DSP | 8 | Audio.Engine |
| 39 | Networking.PredictionRollback | 10 | Networking.Replication, Physics.RigidBody |
| 40 | Networking.Session | 8 | Networking.Transport |
| 41 | GameFramework.GameplayPrimitives | 15 | CoreRuntime.ECS, CoreRuntime.SpatialIndex, Physics.RigidBody, Physics.SpatialQueries, Input.ActionsMapping |
| 42 | GameFramework.Camera | 8 | CoreRuntime.SceneTransforms, Physics.SpatialQueries |
| 43 | UI2D.UIRendering | 8 | UI2D.WidgetFramework, Rendering.RenderGraph, Rendering.GPUAbstraction |
| 44 | UI2D.GameUI | 8 | UI2D.WidgetFramework, UI2D.UIRendering |
| 45 | VFX.Decals | 6 | Rendering.RenderGraph |
| 46 | VFX.ScreenEffects | 6 | Rendering.CoreRendering |
| 47 | GeometryWorld.Terrain | 12 | Rendering.CoreRendering, ContentPipeline.Streaming, Physics.RigidBody |
| 48 | GeometryWorld.Foliage | 8 | Rendering.CoreRendering, CoreRuntime.SpatialIndex, ContentPipeline.Streaming |
| 49 | Input.Gestures | 6 | Input.DeviceAbstraction |

**Max parallelism: 19 agents**

---

### Wave 5 — Advanced Systems (requires specific Wave 4 nodes)

| # | Feature-Group | ~Features | Key Prerequisite |
|---|---------------|-----------|------------------|
| 50 | Rendering.AdvancedRendering | 40 | Rendering.Lighting, Rendering.CoreRendering |
| 51 | ToolsEditor.EditorFramework | 15 | UI2D.WidgetFramework, CoreRuntime.ReflectionSerialization, ContentPipeline.HotReload, Rendering.CoreRendering, Input.ActionsMapping |
| 52 | ToolsEditor.LogicGraph | 14 | CoreRuntime.ECS, CoreRuntime.EventsPlugins, ContentPipeline.HotReload |
| 53 | ToolsEditor.MaterialEditor | 8 | Rendering.Lighting, ContentPipeline.HotReload |
| 54 | ToolsEditor.Profiling | 8 | CoreRuntime.ECS, Platform.Threading |
| 55 | GameFramework.SaveSystem | 10 | CoreRuntime.ReflectionSerialization, GameFramework.GameplayPrimitives |
| 56 | GameFramework.Abilities | 12 | GameFramework.GameplayPrimitives, Animation.StateMachine, Physics.SpatialQueries |
| 57 | GameFramework.WeaponSystem | 10 | GameFramework.GameplayPrimitives, Physics.SpatialQueries, Animation.StateMachine |
| 58 | GameFramework.Progression | 10 | GameFramework.GameplayPrimitives |
| 59 | GameFramework.WorldManagement | 10 | CoreRuntime.ECS, ContentPipeline.Streaming |
| 60 | Physics.SoftBodyCloth | 10 | Physics.RigidBody, Animation.Skeletal |
| 61 | Physics.Destruction | 10 | Physics.RigidBody, Physics.Constraints |
| 62 | GeometryWorld.Water | 10 | Rendering.CoreRendering, Physics.RigidBody |
| 63 | Audio.MusicSystem | 8 | Audio.Engine, Audio.DSP |
| 64 | Audio.VoiceSpeech | 10 | Audio.Engine, Networking.Transport |
| 65 | AI.CrowdTactical | 10 | AI.BehaviorDecision, AI.Steering |
| 66 | VFX.Weather | 8 | VFX.Particles, Audio.Spatial |
| 67 | VFX.EffectGraph | 8 | VFX.Particles, ToolsEditor.LogicGraph |
| 68 | UI2D.2DGames | 15 | UI2D.WidgetFramework, Physics.RigidBody, Rendering.CoreRendering |
| 69 | Networking.AntiCheat | 6 | Networking.Replication, GameFramework.GameplayPrimitives |

**Max parallelism: 20 agents**

---

### Wave 6 — Capstone Systems (requires specific Wave 5 nodes)

| # | Feature-Group | ~Features | Key Prerequisite |
|---|---------------|-----------|------------------|
| 70 | ToolsEditor.AnimationEditor | 8 | ToolsEditor.EditorFramework, Animation.StateMachine |
| 71 | ToolsEditor.WorldBuilding | 10 | ToolsEditor.EditorFramework, GeometryWorld.Terrain, GeometryWorld.Foliage |
| 72 | ToolsEditor.AdvancedTools | 35 | ToolsEditor.EditorFramework, Networking.Replication |
| 73 | ContentPipeline.DCCPlugins | 10 | ContentPipeline.AssetImport, ContentPipeline.AssetProcessing |
| 74 | Animation.ClothHair | 8 | Physics.SoftBodyCloth, Audio.VoiceSpeech, Rendering.AdvancedRendering |
| 75 | Animation.FirstPerson | 6 | Animation.Skeletal, GameFramework.WeaponSystem |
| 76 | VFX.Destruction | 8 | Physics.Destruction, VFX.Particles |
| 77 | GameFramework.NPCSimulation | 10 | AI.BehaviorDecision, GameFramework.GameplayPrimitives |
| 78 | GameFramework.Social | 8 | Networking.Session, GameFramework.GameplayPrimitives |
| 79 | GameFramework.GameModes | 8 | Networking.Session, GameFramework.GameplayPrimitives |
| 80 | GameFramework.StealthCover | 8 | AI.Perception, GameFramework.GameplayPrimitives |
| 81 | GameFramework.GenreSpecific | 40 | GameFramework.GameplayPrimitives, GameFramework.Abilities, Physics.RigidBody, AI.Navigation |
| 82 | GeometryWorld.TerrainVoxel | 10 | GeometryWorld.Terrain, Physics.Destruction |
| 83 | GeometryWorld.ProceduralGeneration | 30 | GeometryWorld.Terrain, Rendering.AdvancedRendering, ContentPipeline.Streaming |
| 84 | Physics.Fluid | 8 | Physics.RigidBody, GeometryWorld.Water |
| 85 | Networking.MMO | 10 | Networking.Replication, GameFramework.WorldManagement, GameFramework.SaveSystem |
| 86 | UI2D.Accessibility | 8 | UI2D.WidgetFramework, Audio.Engine, Input.ActionsMapping |
| 87 | Input.VRInput | 6 | Input.DeviceAbstraction, Rendering.AdvancedRendering |

**Max parallelism: 18 agents**

---

## Critical Path

The longest chain through the DAG determines the minimum calendar time.

```
Platform.Threading → CoreRuntime.ECS → Rendering.GPUAbstraction → Rendering.RenderGraph
  → Rendering.CoreRendering → Rendering.Lighting → Rendering.AdvancedRendering
    → GeometryWorld.ProceduralGeneration
```

**Critical path length: 8 feature-groups across 7 waves.**

Secondary critical paths:

```
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

The following contracts must be defined and locked before Wave 2 begins, as they are consumed by multiple concurrent
development pipelines:

| Contract | Defined By | Consumed By | Scope |
|----------|-----------|-------------|-------|
| Component trait + archetype API | CoreRuntime.ECS | All domains | How to define, register, query, and mutate components |
| Entity handle type | CoreRuntime.ECS | All domains | Generational index format, validity checking |
| System trait + scheduling API | CoreRuntime.ECS | All domains | How to declare systems, dependencies, and access patterns |
| Event channel API | CoreRuntime.EventsPlugins | All domains | How to send and receive typed events |
| Spatial index query API | CoreRuntime.SpatialIndex | Physics, Rendering, Networking, AI, Audio, GameFramework | Ray-cast, shape-cast, overlap, frustum, k-nearest |
| Serialization trait | CoreRuntime.ReflectionSerialization | Networking, ContentPipeline, GameFramework | Binary/text encode/decode for components and assets |
| Async I/O trait | CoreRuntime.MemoryAsyncIO | ContentPipeline, Platform | Read/write/cancel operations, completion callbacks |
| Transform component schema | CoreRuntime.SceneTransforms | Rendering, Physics, Animation, Audio | Position, rotation, scale, parent, world matrix |
| GPU backend trait | Rendering.GPUAbstraction | ContentPipeline.AssetProcessing, VFX, UI2D | Device, command buffer, pipeline, buffer, texture APIs |
| Asset handle type | ContentPipeline.AssetDatabase | All domains | Typed handle, load state, reference counting |
| Render graph pass API | Rendering.RenderGraph | VFX, UI2D, Rendering.* | How to declare passes, read/write resources, submit commands |
