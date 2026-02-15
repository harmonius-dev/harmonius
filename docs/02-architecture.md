# Harmonius - Architecture

## Thread Model

```mermaid
graph LR
    subgraph "User Thread (Safe)"
        UT[Application Logic]
        RGB[Render Graph Builder]
        ASM[Animation State Machine]
        SGU[Scene Graph Updates]
    end

    subgraph "Render Thread Pool (Unsafe)"
        RT1[Render Thread 1<br/>Camera A]
        RT2[Render Thread 2<br/>Camera B]
        RTn[Render Thread N<br/>Camera N]
    end

    subgraph "IO Thread Pool (Unsafe)"
        IO1[IO Worker 1]
        IO2[IO Worker 2]
        IOn[IO Worker N]
    end

    subgraph "Async Compute (Unsafe)"
        AC[Compute Dispatcher]
    end

    UT --> RGB
    UT --> ASM
    UT --> SGU
    RGB -->|"Execution Plan (Arc)"| RT1
    RGB -->|"Execution Plan (Arc)"| RT2
    RGB -->|"Execution Plan (Arc)"| RTn
    SGU -->|"Frame Data (ring buffer)"| RT1
    IO1 -.->|"fence sync"| RT1
    IO2 -.->|"fence sync"| RT2
    AC -.->|"fence sync"| RT1
```

### Thread Responsibilities

| Thread | Safety | Responsibilities |
|---|---|---|
| **User Thread** | 100% Safe Rust | Graph building, scene updates, animation state, input handling |
| **Render Thread(s)** | Unsafe (C++) | Command encoding, GPU submission, swap chain present, per-camera |
| **IO Thread(s)** | Unsafe (C++) | Disk reads, GPU uploads via transfer queue, streaming management |
| **Async Compute** | Unsafe (C++) | Multi-frame compute jobs, mesh generation, readback |

### Synchronization Primitives

| Mechanism | Purpose | Platform Mapping |
|---|---|---|
| **GPU Fence** | Cross-queue sync (render↔transfer↔compute) | MTLFence / VkSemaphore / ID3D12Fence |
| **GPU Event** | Fine-grained intra-queue sync | MTLEvent / VkEvent / ID3D12Fence |
| **Ring Buffer** | User thread → render thread frame data | Lock-free SPSC per render thread |
| **Arc\<ExecutionPlan\>** | Shared immutable execution plan | Rust Arc, read-only after compilation |
| **Resource Registry** | Shared resource handles across threads | Concurrent hash map with generation counters |

---

## Render Graph Lifecycle

```mermaid
stateDiagram-v2
    [*] --> Build: Application startup
    Build --> Compile: graph.compile()
    Compile --> Execute: Each frame
    Execute --> Execute: Next frame
    Execute --> Compile: Graph invalidated
    Compile --> Build: Graph restructured

    state Build {
        [*] --> DeclareNodes
        DeclareNodes --> DeclareEdges
        DeclareEdges --> DeclareResources
        DeclareResources --> [*]
    }

    state Compile {
        [*] --> FeatureDetection
        FeatureDetection --> NodeCulling
        NodeCulling --> DependencyResolution
        DependencyResolution --> ResourceScheduling
        ResourceScheduling --> BarrierInsertion
        BarrierInsertion --> [*]
    }

    state Execute {
        [*] --> FrameDataUpdate
        FrameDataUpdate --> RuntimeCulling
        RuntimeCulling --> CommandEncoding
        CommandEncoding --> QueueSubmit
        QueueSubmit --> Present
        Present --> [*]
    }
```

### Phase 1: Build (Declarative, Frame-Invariant)

The user constructs a render graph using the safe Rust API. The graph is a DAG
of **passes** connected by **resource edges**. This graph does not change
frame-to-frame.

| Concept | Description |
|---|---|
| **Pass Node** | A render, compute, or transfer operation |
| **Resource Node** | A buffer, texture, or acceleration structure |
| **Edge** | Data dependency: pass reads/writes a resource |
| **Feature Gate** | Conditional node activation based on capability flags |
| **Budget Gate** | Conditional node activation based on performance budget |

```mermaid
graph TD
    subgraph "Example Render Graph"
        CSM[Cascaded Shadow Maps] --> FWD[Forward+ Lighting]
        GBuf[G-Buffer Pass] --> DEF[Deferred Lighting]
        DEF --> COMP[Composite]
        FWD --> COMP
        HZB[HZB Build] --> CULL[GPU Culling]
        CULL --> GBuf
        CULL --> FWD
        CULL --> CSM
        RT[Ray Trace Reflections] --> COMP
        VOL[Volumetrics] --> COMP
        SKY[Procedural Sky] --> COMP
        COMP --> PP[Post-Process]
        PP --> UI[UI Overlay]
        UI --> PRESENT[Present]

        RT -.->|"feature gate: RT capable"| RT
        VOL -.->|"budget gate: high quality"| VOL
    end
```

### Phase 2: Compile (Optimization)

The compiler transforms the declarative graph into an **Execution Plan** — an
optimized, flattened schedule of operations.

| Step | Operation | Details |
|---|---|---|
| 1 | **Feature Detection** | Query GPU capabilities, disable unsupported pass nodes |
| 2 | **Node Culling** | Remove disabled nodes and their exclusive dependencies (transitive) |
| 3 | **Dependency Resolution** | Topological sort of remaining nodes, detect parallelism |
| 4 | **Resource Scheduling** | Compute resource lifetimes, plan aliasing/reuse |
| 5 | **Barrier Insertion** | Insert minimal memory/execution barriers between passes |
| 6 | **Queue Assignment** | Assign passes to render/compute/transfer queues |

#### Transitive Culling

```mermaid
graph LR
    A[Pass A] --> B[Pass B]
    B --> C[Pass C<br/>DISABLED]
    C --> D[Pass D]
    B --> E[Pass E]

    style C fill:#f66,color:#fff
    style D fill:#f66,color:#fff
```

When Pass C is disabled, Pass D is culled if C was its only dependency. Pass B
remains because Pass E still depends on it.

### Phase 3: Execute (Per-Frame)

The executor runs the compiled plan each frame, handling dynamic state.

| Step | Operation |
|---|---|
| 1 | **Frame Data Update** | Push per-frame data (transforms, lights, camera) via ring buffer |
| 2 | **Runtime Culling** | Disable budget-gated passes if frame budget exceeded |
| 3 | **Distance Sorting** | CPU-side generic distance sort for transparency, particles, etc. |
| 4 | **Command Encoding** | Parallel command encoding on render threads |
| 5 | **Queue Submit** | Submit command buffers with appropriate fences |
| 6 | **Present** | Swap chain present, pacing to display refresh |

---

## Resource Management

### Resource Types

| Type | Description | Bindless | Streaming |
|---|---|---|---|
| **Buffer** | Vertex, index, uniform, storage, indirect | Yes | No |
| **Texture2D** | Color, normal, material maps | Yes | Tiled |
| **Texture3D** | Voxel data, volume fog LUTs | Yes | Sliced |
| **TextureCube** | Environment maps, reflection probes | Yes | Per-face |
| **AccelerationStructure** | BLAS/TLAS for ray tracing | N/A | Incremental |
| **RenderTarget** | Framebuffers, G-buffer layers | No | No |

### Resource Lifecycle

```mermaid
stateDiagram-v2
    [*] --> Requested: User declares resource
    Requested --> Loading: IO worker picks up
    Loading --> Resident: Upload complete
    Resident --> InUse: Referenced by active pass
    InUse --> Resident: Pass complete
    Resident --> Evicting: Memory pressure
    Evicting --> Requested: Re-requested
    Evicting --> [*]: No longer needed

    Loading --> Loading: Streaming (progressive)
```

### Bindless Resource Model

All resources are accessed via bindless descriptor indices. No per-draw
descriptor set binding.

```mermaid
graph LR
    subgraph "Bindless Heap"
        H[Global Descriptor Heap]
    end

    subgraph "Per-Draw Data"
        DD[Draw Data Buffer<br/>resource_indices: u32 array]
    end

    subgraph "Shader"
        S["texture = heap[draw.albedo_idx]"]
    end

    DD --> S
    H --> S
```

| Platform | Mechanism |
|---|---|
| Metal | Argument Buffers (Tier 2) + Heap |
| Vulkan | VK_EXT_descriptor_indexing (UPDATE_AFTER_BIND) |
| D3D12 | Shader Model 6.6 bindless, CBV_SRV_UAV heap |

---

## IO & Streaming Architecture

```mermaid
graph TB
    subgraph "IO Manager"
        REQ[Request Queue] --> PRI[Priority Scheduler]
        PRI --> W1[IO Worker 1]
        PRI --> W2[IO Worker 2]
        PRI --> Wn[IO Worker N]
    end

    subgraph "Platform IO"
        W1 --> MTLIO[MTLIOCommandBuffer]
        W1 --> DS[DirectStorage]
        W1 --> IOURING[io_uring]
    end

    subgraph "Transfer Queues"
        MTLIO --> TQ1[Transfer Queue 1]
        DS --> TQ1
        IOURING --> TQ1
        TQ1 --> FENCE[GPU Fence]
    end

    subgraph "Render Queue"
        FENCE --> RQ[Render Queue]
    end

    subgraph "Streaming Types"
        TILE[Terrain Tiles<br/>2D heightmap chunks]
        VOX[Voxel Slices<br/>3D texture layers]
        MESH[Meshlet Groups<br/>LOD streaming]
    end

    TILE --> REQ
    VOX --> REQ
    MESH --> REQ
```

### Streaming Priority Model

| Priority | Source | Example |
|---|---|---|
| **Critical** | Visible, no LOD available | Player's immediate surroundings |
| **High** | Visible, low LOD loaded | Terrain tile upgrading from LOD 2→0 |
| **Normal** | Pre-fetch for predicted movement | Camera direction extrapolation |
| **Low** | Background loading | Distant environment pre-caching |

---

## Async Compute Integration

```mermaid
sequenceDiagram
    participant IO as IO Thread
    participant CQ as Compute Queue
    participant RQ as Render Queue

    IO->>CQ: Signal fence (upload complete)
    CQ->>CQ: Process geometry (mesh shader output)
    CQ->>CQ: Generate terrain chunks
    CQ->>RQ: Signal fence (compute complete)
    RQ->>RQ: Render using generated assets
    CQ->>IO: Readback request (persist generated data)
    IO->>IO: Write to disk
```

### Multi-Frame Compute Jobs

| Job Type | Duration | Readback | Example |
|---|---|---|---|
| **Terrain Generation** | 1-4 frames | Yes | Heightmap → meshlets |
| **Voxel Meshing** | 1-2 frames | Optional | SDF → mesh surface |
| **GI Update** | N frames (amortized) | No | Irradiance probe update |
| **Acceleration Structure Build** | 1 frame | No | BLAS/TLAS rebuild |

---

## Command Encoding Model

```mermaid
graph TB
    subgraph "Frame N"
        EP[Execution Plan] --> ENC1[Encoder: Shadow Passes]
        EP --> ENC2[Encoder: G-Buffer]
        EP --> ENC3[Encoder: Forward+]
        EP --> ENC4[Encoder: Post-Process]

        ENC1 --> CB1[Command Buffer 1]
        ENC2 --> CB2[Command Buffer 2]
        ENC3 --> CB3[Command Buffer 3]
        ENC4 --> CB4[Command Buffer 4]

        CB1 --> SUBMIT[Ordered Submit]
        CB2 --> SUBMIT
        CB3 --> SUBMIT
        CB4 --> SUBMIT
    end
```

### Per-Platform Command Model

| Concept | Metal | Vulkan | D3D12 |
|---|---|---|---|
| Command Buffer | MTLCommandBuffer | VkCommandBuffer | ID3D12CommandList |
| Render Pass | MTLRenderCommandEncoder | VkRenderPass / Dynamic Rendering | Render Pass (Agility SDK) |
| Compute Pass | MTLComputeCommandEncoder | vkCmdDispatch | ID3D12CommandList::Dispatch |
| Indirect Draw | drawMeshThreadgroups(indirectBuffer:) | vkCmdDrawMeshTasksIndirectEXT | ExecuteIndirect |
| Barrier | MTLFence + memoryBarrier | vkCmdPipelineBarrier2 | ResourceBarrier |

---

## Multi-Camera / Split-Screen

```mermaid
graph TB
    subgraph "Shared Resources"
        SR[Shadow Maps<br/>Environment Maps<br/>GI Probes]
    end

    subgraph "Render Thread 1"
        RT1[Camera A Encoding]
        RT1 --> CB_A[Command Buffer A]
    end

    subgraph "Render Thread 2"
        RT2[Camera B Encoding]
        RT2 --> CB_B[Command Buffer B]
    end

    SR --> RT1
    SR --> RT2

    CB_A --> Q1[Queue 1]
    CB_B --> Q2[Queue 2]

    Q1 --> FENCE_SHARED[Shared Resource Fence]
    Q2 --> FENCE_SHARED
```

### Resource Sharing Rules

| Resource | Shared | Sync Required |
|---|---|---|
| Shadow maps | Yes (if same sun/lights) | Read-only after generation |
| GI probes | Yes | Read-only after update |
| Meshlet buffers | Yes | Immutable |
| Per-camera uniforms | No | Per-thread ring buffer |
| Render targets | No | Per-camera exclusive |
| Acceleration structures | Yes | Read-only after build |

---

## Data Flow Summary

```mermaid
graph LR
    subgraph "CPU (Safe Rust)"
        SCENE[Scene Data] --> SORT[Distance Sort<br/>Generic]
        SORT --> UPLOAD[Ring Buffer Upload]
    end

    subgraph "GPU"
        UPLOAD --> UBO[Per-Frame Uniforms]
        UBO --> CULL_GPU[GPU Culling<br/>Frustum+Occlusion+Backface]
        CULL_GPU --> INDIRECT[Indirect Draw Buffer]
        INDIRECT --> MESH_SHADE[Mesh Shader Pipeline]
        MESH_SHADE --> RASTER[Rasterization]
        RASTER --> SHADE[Fragment Shading]
        SHADE --> COMPOSITE[Compositing]
    end
```

### CPU-Side Distance Sorting

Generic distance-based sorting runs on the CPU before upload. Use cases:

| Use Case | Sort Key | Direction |
|---|---|---|
| Transparent objects | Camera distance | Back-to-front |
| Particles | Camera distance | Back-to-front |
| Opaque objects | Camera distance | Front-to-back (optional, for early-z) |
| LOD selection | Camera distance | N/A (bucket selection) |
| Streaming priority | Camera distance | Nearest first |
