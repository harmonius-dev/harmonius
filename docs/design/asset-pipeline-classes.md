# Asset Pipeline Class and Sequence Diagrams

Class diagrams for each module and sequence diagrams showing inter-module interactions.
Companion to [asset-pipeline.md](asset-pipeline.md).

---

## Contents

- [Module Class Diagrams](#module-class-diagrams)
  - [1. Asset Types and Cooking](#1-asset-types-and-cooking)
  - [2. Mesh Cooking](#2-mesh-cooking)
  - [3. Texture Cooking](#3-texture-cooking)
  - [4. Bundle Format](#4-bundle-format)
  - [5. Streaming](#5-streaming)
  - [6. IO Backend](#6-io-backend)
  - [7. Resource Registry](#7-resource-registry)
  - [8. Bindless Descriptor Heap](#8-bindless-descriptor-heap)
- [Cross-Module Relationships](#cross-module-relationships)
- [Sequence Diagrams](#sequence-diagrams)
  - [Full Asset Lifecycle](#full-asset-lifecycle)
  - [Streaming Fault Resolution](#streaming-fault-resolution)
  - [Descriptor Heap Allocation Flow](#descriptor-heap-allocation-flow)

---

## Module Class Diagrams

### 1. Asset Types and Cooking

`harmonius::asset` — Core identity types, raw/cooked asset structs, and the cooker
orchestrator.

```mermaid
classDiagram
    class AssetId {
        <<enum class : uint64_t>>
        invalid = 0
    }
    class AssetType {
        <<enum class : uint8_t>>
        mesh
        texture
        material
        terrain_tile
        shader
        acceleration_structure
    }
    class RawAsset {
        +AssetId id
        +AssetType type
        +filesystem_path source_path
        +vector~uint8_t~ raw_data
    }
    class CookedAsset {
        +AssetId id
        +AssetType type
        +gpu_Backend target_backend
        +vector~uint8_t~ data
        +uint64_t content_hash
        +CookedMetadata metadata
    }
    class CookedMetadata {
        +uint32_t meshlet_count
        +uint32_t vertex_count
        +uint32_t triangle_count
        +rg_Format format
        +uint32_t width
        +uint32_t height
        +uint32_t mip_levels
        +uint32_t array_layers
        +uint64_t gpu_size_bytes
    }
    class AssetCooker {
        +AssetCooker(gpu_Backend target)
        +Cook(RawAsset) expected~CookedAsset, CookError~
        +CookBatch(span~RawAsset~) vector~expected~
        +RegisterProcessor(AssetType, function) void
    }

    RawAsset --> AssetId
    RawAsset --> AssetType
    CookedAsset --> AssetId
    CookedAsset --> AssetType
    CookedAsset *-- CookedMetadata
    AssetCooker --> RawAsset : consumes
    AssetCooker --> CookedAsset : produces
```

### 2. Mesh Cooking

`harmonius::asset` — Meshlet generation and LOD chain construction via meshoptimizer.

```mermaid
classDiagram
    class MeshletData {
        +vector~Meshlet~ meshlets
        +vector~uint8_t~ vertex_data
        +vector~uint8_t~ triangle_data
        +uint32_t vertex_stride
    }
    class Meshlet {
        +uint32_t vertex_offset
        +uint32_t triangle_offset
        +uint8_t vertex_count
        +uint8_t triangle_count
        +float bounding_sphere[4]
        +float normal_cone[4]
    }
    class MeshletBuilder {
        +Build(span~float~, span~uint32_t~, uint32_t) MeshletData
        +BuildLodChain(span~float~, span~uint32_t~, uint32_t, uint32_t) vector~MeshletData~
    }

    MeshletData *-- Meshlet
    MeshletBuilder --> MeshletData : produces
```

### 3. Texture Cooking

`harmonius::asset` — Block compression (BC7/BC5/BC1) and mip-map generation.

```mermaid
classDiagram
    class TextureCookOptions {
        +rg_Format target_format
        +bool generate_mips
        +bool srgb
        +uint32_t max_dimension
    }
    class TextureCompressor {
        +Compress(span~uint8_t~, uint32_t, uint32_t, TextureCookOptions) CookedAsset
    }

    TextureCompressor --> TextureCookOptions : reads
    TextureCompressor --> CookedAsset : produces
```

### 4. Bundle Format

`harmonius::asset` — Manifest describing all bundles and chunk layout with 64KB
alignment.

```mermaid
classDiagram
    class BundleManifest {
        +vector~BundleEntry~ bundles
        +vector~AssetEntry~ assets
        +Find(AssetId) AssetEntry*
    }
    class BundleEntry {
        +string bundle_path
        +uint64_t total_size
        +vector~AssetId~ asset_ids
    }
    class AssetEntry {
        +AssetId id
        +AssetType type
        +uint64_t gpu_size
        +uint32_t bundle_index
        +uint32_t chunk_index
        +uint64_t chunk_offset
        +uint64_t chunk_size
        +uint16_t priority_bias
        +CookedMetadata metadata
    }
    class ChunkLayout {
        +vector~Region~ regions
        +uint64_t total_size
        +uint64_t alignment$
    }
    class Region {
        +uint64_t offset
        +uint64_t size
        +AssetType type
    }

    BundleManifest *-- BundleEntry
    BundleManifest *-- AssetEntry
    AssetEntry --> AssetId
    AssetEntry --> AssetType
    AssetEntry *-- CookedMetadata
    BundleEntry --> AssetId
    ChunkLayout *-- Region
    Region --> AssetType
```

### 5. Streaming

`harmonius::asset` — Priority-based streaming scheduler that bridges IO and the render
graph via transfer pass injection.

```mermaid
classDiagram
    class StreamPriority {
        <<enum class : uint8_t>>
        critical = 0
        high = 1
        normal = 2
        low = 3
    }
    class StreamRequest {
        +AssetId asset_id
        +uint32_t chunk_index
        +StreamPriority priority
        +float camera_distance
    }
    class StreamingScheduler {
        +StreamingScheduler(BundleManifest, PoolAllocator, RingAllocator)
        +Request(span~StreamRequest~) void
        +ProcessPending() vector~TransferPassDesc~
        +SetEvictionPolicy(function) void
        +IsResident(AssetId) bool
        +ResidencyRatio() float
        +IoBackend() IOBackend
        -IOBackend io_backend_
    }

    StreamRequest --> AssetId
    StreamRequest --> StreamPriority
    StreamingScheduler --> StreamRequest : processes
    StreamingScheduler --> BundleManifest : reads
    StreamingScheduler --> PoolAllocator : allocates from
    StreamingScheduler --> RingAllocator : stages with
    StreamingScheduler *-- IOBackend
```

### 6. IO Backend

`harmonius::asset` — Platform-native async IO with concept-based static dispatch.
No C++ standard library file IO is used. One backend is compiled per binary, selected
at build time via CMake and target platform. No virtual methods, no vtables, no
dynamic dispatch.

```mermaid
classDiagram
    class IOCompletion {
        +AssetId asset_id
        +gpu_ResourceHandle staging_buffer
        +uint64_t buffer_offset
        +uint64_t size
        +bool success
    }
    class IOBackendConcept {
        <<concept>>
        +SubmitRead(string_view, uint64_t, uint64_t, ResourceHandle, uint64_t) void
        +PollCompletions() vector~IOCompletion~
    }
    class DispatchIOBackend {
        +SubmitRead(string_view, uint64_t, uint64_t, ResourceHandle, uint64_t) void
        +PollCompletions() vector~IOCompletion~
    }
    class DirectStorageBackend {
        +SubmitRead(string_view, uint64_t, uint64_t, ResourceHandle, uint64_t) void
        +PollCompletions() vector~IOCompletion~
    }
    class IocpBackend {
        +SubmitRead(string_view, uint64_t, uint64_t, ResourceHandle, uint64_t) void
        +PollCompletions() vector~IOCompletion~
    }
    class IoUringBackend {
        +SubmitRead(string_view, uint64_t, uint64_t, ResourceHandle, uint64_t) void
        +PollCompletions() vector~IOCompletion~
    }
    class IOBackend {
        <<compile-time alias>>
    }

    IOBackendConcept ..> DispatchIOBackend : satisfied by
    IOBackendConcept ..> DirectStorageBackend : satisfied by
    IOBackendConcept ..> IocpBackend : satisfied by
    IOBackendConcept ..> IoUringBackend : satisfied by
    DispatchIOBackend --> IOCompletion : produces
    DirectStorageBackend --> IOCompletion : produces
    IocpBackend --> IOCompletion : produces
    IoUringBackend --> IOCompletion : produces
    IOBackend ..> DispatchIOBackend : Metal (macOS)
    IOBackend ..> DirectStorageBackend : D3D12 (Windows)
    IOBackend ..> IocpBackend : Vulkan (Windows)
    IOBackend ..> IoUringBackend : Vulkan (Linux)
```

### 7. Resource Registry

`harmonius::asset` — Maps asset IDs to live GPU resource handles and bindless descriptor
indices. Generational handles detect stale references.

```mermaid
classDiagram
    class AssetHandle {
        +uint32_t index
        +uint32_t generation
        +IsValid() bool
    }
    class ResourceRegistry {
        +ResourceRegistry(gpu_Device, BindlessDescriptorHeap)
        +RegisterAsset(AssetId, gpu_ResourceHandle, CookedMetadata) AssetHandle
        +Unregister(AssetHandle) void
        +Resolve(AssetHandle) expected~gpu_ResourceHandle, AssetError~
        +ResolveDescriptorIndex(AssetHandle) expected~uint32_t, AssetError~
        +FindByRgResource(rg_ResourceHandle) AssetHandle
        +RegisteredCount() uint32_t
        +TotalGpuBytes() uint64_t
        -vector~Entry~ entries_
        -unordered_map id_to_index_
        -queue~uint32_t~ free_list_
        -BindlessDescriptorHeap heap_
    }
    class Entry {
        +AssetId id
        +gpu_ResourceHandle gpu_handle
        +uint32_t descriptor_index
        +uint32_t generation
        +CookedMetadata metadata
    }

    ResourceRegistry *-- Entry
    ResourceRegistry --> AssetHandle : produces
    ResourceRegistry --> BindlessDescriptorHeap : delegates to
    ResourceRegistry --> AssetId : maps from
    Entry --> AssetId
    Entry --> CookedMetadata
```

### 8. Bindless Descriptor Heap

`harmonius::asset` — Single global descriptor heap (1M slots) managing all GPU
resources. Shaders address resources by `uint32_t` index via push constants.

```mermaid
classDiagram
    class BindlessDescriptorHeap {
        +BindlessDescriptorHeap(gpu_Device, uint32_t max_descriptors)
        +Allocate(gpu_ResourceHandle) expected~uint32_t, HeapError~
        +Free(uint32_t index) void
        +Update(uint32_t index, gpu_ResourceHandle) void
        +GpuHandle() gpu_ResourceHandle
        +ImportToGraph(GraphBuilder) rg_ResourceHandle
        +AllocatedCount() uint32_t
        +Capacity() uint32_t
        -gpu_Device device_
        -gpu_ResourceHandle heap_handle_
        -queue~uint32_t~ free_list_
        -uint32_t next_index_
        -uint32_t max_descriptors_
    }

    BindlessDescriptorHeap --> gpu_Device : uses
    BindlessDescriptorHeap --> GraphBuilder : imports into
```

---

## Cross-Module Relationships

How the eight modules depend on each other at the class level, including integration
points with the render graph resource system and execution engine.

```mermaid
classDiagram
    class AssetCooker {
        <<cooking>>
    }
    class MeshletBuilder {
        <<cooking>>
    }
    class TextureCompressor {
        <<cooking>>
    }
    class BundleManifest {
        <<bundle>>
    }
    class ChunkLayout {
        <<bundle>>
    }
    class StreamingScheduler {
        <<streaming>>
    }
    class IOBackend {
        <<io>>
    }
    class ResourceRegistry {
        <<registry>>
    }
    class BindlessDescriptorHeap {
        <<registry>>
    }
    class PoolAllocator {
        <<rg resource>>
    }
    class RingAllocator {
        <<rg resource>>
    }
    class Executor {
        <<rg exec>>
    }
    class Device {
        <<gpu>>
    }

    AssetCooker --> MeshletBuilder : delegates mesh cooking
    AssetCooker --> TextureCompressor : delegates texture cooking
    AssetCooker --> CookedAsset : produces
    CookedAsset --> BundleManifest : packed into
    BundleManifest --> ChunkLayout : describes chunks
    StreamingScheduler --> BundleManifest : reads manifest
    StreamingScheduler --> IOBackend : submits reads
    StreamingScheduler --> PoolAllocator : allocates device memory
    StreamingScheduler --> RingAllocator : allocates staging
    StreamingScheduler --> Executor : injects transfers
    Executor --> ResourceRegistry : triggers registration
    ResourceRegistry --> BindlessDescriptorHeap : allocates descriptors
    BindlessDescriptorHeap --> Device : manages GPU heap
    ResourceRegistry --> Device : resolves GPU handles
```

### Asset Pipeline to Render Graph Type Mapping

How asset pipeline types translate into render graph and GPU backend types at module
boundaries.

| Asset Pipeline Type      | Render Graph / GPU Type                  | Translation Point                |
| ------------------------ | ---------------------------------------- | -------------------------------- |
| `AssetId` (uint64_t)    | `rg::ResourceHandle`                     | `ResourceRegistry::FindByRgResource()` |
| `AssetHandle` (gen+idx) | `gpu::ResourceHandle`                    | `ResourceRegistry::Resolve()`    |
| `AssetHandle` (gen+idx) | `uint32_t` descriptor index              | `ResourceRegistry::ResolveDescriptorIndex()` |
| `CookedMetadata`        | `gpu::TextureDesc` / `gpu::BufferDesc`   | Resource allocation at registration |
| `StreamRequest`         | `exec::TransferPassDesc`                 | `StreamingScheduler::ProcessPending()` |
| `IOCompletion`          | `gpu::ResourceHandle` (staging Buffer)   | IO backend returns staging refs  |
| `ChunkLayout::alignment`| `gpu::AllocationInfo` alignment          | 64KB alignment matches GPU DMA   |

---

## Sequence Diagrams

### Full Asset Lifecycle

Cook, bundle, stream, register, and render across build time and runtime.

```mermaid
sequenceDiagram
    participant DCC as DCC Tool
    participant Cook as AssetCooker
    participant MB as MeshletBuilder
    participant TC as TextureCompressor
    participant Pack as BundlePacker
    participant Sched as StreamingScheduler
    participant IO as IOBackend
    participant Reg as ResourceRegistry
    participant Heap as BindlessDescriptorHeap
    participant Exec as Executor
    participant RG as Render Graph

    Note over DCC,RG: Phase 1 - Offline Cook (build time)
    DCC->>Cook: RawAsset (glTF, PNG, EXR)
    Cook->>MB: mesh raw data
    MB->>MB: optimize, meshletize, LOD chain
    MB-->>Cook: MeshletData
    Cook->>TC: texture raw data
    TC->>TC: block Compress (BC7/BC5), mip-gen
    TC-->>Cook: CookedAsset
    Cook->>Pack: CookedAsset batch
    Pack->>Pack: align 64KB, combine chunks
    Pack-->>Cook: BundleManifest + .hab files

    Note over DCC,RG: Phase 2 - Runtime Streaming
    Sched->>Sched: evaluate camera, visibility
    Sched->>IO: SubmitRead(chunk)
    IO->>IO: platform-native async read
    IO-->>Sched: IOCompletion (staging buffer)
    Sched->>Exec: InjectTransfer(staging, device)
    Exec->>RG: transfer pass in graph
    RG->>RG: DMA upload (transfer queue)
    RG-->>Exec: completion fence

    Note over DCC,RG: Phase 3 - Registration
    Exec->>Reg: register_asset(id, gpu_handle, metadata)
    Reg->>Heap: Allocate(gpu_handle)
    Heap-->>Reg: descriptor_index
    Reg-->>Exec: AssetHandle

    Note over DCC,RG: Phase 4 - Rendering
    RG->>Reg: ResolveDescriptorIndex(handle)
    Reg-->>RG: uint32_t index
    RG->>RG: push_constants.idx = index
    RG->>RG: shader reads ResourceDescriptorHeap[index]
```

### Streaming Fault Resolution

How a residency fault flows through the system across two frames, from detection
through IO, upload, registration, and rendering with the new asset.

```mermaid
sequenceDiagram
    participant Renderer
    participant Sched as StreamingScheduler
    participant IO as IOBackend
    participant Pool as PoolAllocator
    participant Ring as RingAllocator
    participant Exec as Executor
    participant TX as Transfer Queue
    participant GFX as Graphics Queue
    participant Reg as ResourceRegistry
    participant Heap as BindlessDescriptorHeap

    Note over Renderer,Heap: Frame N - fault detected
    GFX->>Renderer: page fault (asset not resident)
    Renderer->>Sched: Request([asset_id, critical])
    Sched->>Sched: sort by priority, camera_distance
    Sched->>IO: SubmitRead(bundle_path, offset, size, staging, buf_offset)
    IO->>IO: platform-native async read
    IO-->>Sched: IOCompletion (success, staging_buffer)

    Note over Renderer,Heap: Frame N+1 - upload and register
    Sched->>Pool: Allocate() device-local memory
    Pool-->>Sched: gpu::ResourceHandle
    Sched->>Exec: InjectTransfer(staging to device)
    Exec->>TX: encode copy command
    TX->>TX: copy staging to device-local
    TX-->>Exec: completion fence signal

    Sched->>Reg: register_asset(id, gpu_handle, metadata)
    Reg->>Heap: Allocate(gpu_handle)
    Heap-->>Reg: descriptor_index
    Reg-->>Sched: AssetHandle

    Exec->>GFX: bind updated resources
    GFX->>GFX: render pass reads new asset
    Renderer->>Sched: IsResident(asset_id)
    Sched-->>Renderer: true
```

### Descriptor Heap Allocation Flow

Detailed flow showing how a descriptor slot is allocated from the free list or by
advancing the next index, how shaders consume the index, and how eviction returns
slots to the free list.

```mermaid
sequenceDiagram
    participant Sched as StreamingScheduler
    participant Reg as ResourceRegistry
    participant Heap as BindlessDescriptorHeap
    participant Device as gpu::Device
    participant RG as Render Graph
    participant Shader

    Note over Sched,Shader: New asset arrives from streaming
    Sched->>Reg: register_asset(asset_id, gpu_handle, metadata)
    Reg->>Reg: find free slot in entries_ (or grow)
    Reg->>Reg: id_to_index_[asset_id] = slot
    Reg->>Heap: Allocate(gpu_handle)

    alt Free list not empty
        Heap->>Heap: index = free_list_.front()
        Heap->>Heap: free_list_.pop()
    else Free list empty
        Heap->>Heap: index = next_index_++
    end

    Heap->>Device: write descriptor at index
    Heap-->>Reg: expected(index)
    Reg->>Reg: entry.descriptor_index = index
    Reg-->>Sched: AssetHandle(slot, generation)

    Note over Sched,Shader: Render graph uses descriptor
    RG->>Reg: ResolveDescriptorIndex(handle)
    Reg->>Reg: validate generation
    Reg-->>RG: expected(42)
    RG->>Shader: push_constants.texture_idx = 42
    Shader->>Shader: ResourceDescriptorHeap[42]

    Note over Sched,Shader: Asset eviction
    Sched->>Reg: Unregister(handle)
    Reg->>Heap: Free(descriptor_index)
    Heap->>Heap: free_list_.push(descriptor_index)
    Reg->>Reg: increment generation
    Reg->>Reg: free_list_.push(slot)
```
