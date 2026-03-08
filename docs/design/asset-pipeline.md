# Asset Pipeline Design

How raw assets are transformed into GPU-optimized resources, packed into streamable bundles,
and addressed by the render graph. Companion to [render-graph-design.md](render-graph-design.md)
and [shader-pipeline.md](shader-pipeline.md).

**Requirements:** R-2.11.4 (glTF import), R-2.12.1–R-2.12.10 (streaming/IO), R-3.4 (resource
budgets), F-6.1.5 (asset import), F-6.2.1–F-6.2.9 (IO/streaming features).

---

## Contents

- [Asset Transformation Pipeline](#asset-transformation-pipeline)
- [Asset Bundle Format](#asset-bundle-format)
- [Buffer and Texture Combination](#buffer-and-texture-combination)
- [Streaming Chunk Architecture](#streaming-chunk-architecture)
- [Resource Registry and Addressing](#resource-registry-and-addressing)
- [Bindless Descriptor Heap](#bindless-descriptor-heap)
- [Integration with Render Graph](#integration-with-render-graph)

---

## Asset Transformation Pipeline

Raw assets (glTF, images, shader graphs) pass through an offline cook pipeline that produces
GPU-optimized, platform-specific bundles. No raw asset parsing occurs at runtime.

```mermaid
flowchart TD
    subgraph raw["Raw Assets"]
        GLTF["glTF 2.0<br/>meshes, materials, textures"]
        IMG["Images<br/>PNG, EXR, HDR"]
        SG["Shader Graphs<br/>binary DAGs"]
        HMAP["Heightmaps<br/>terrain tiles"]
    end

    subgraph cook["Offline Cook Pipeline"]
        MI["Mesh Import<br/>extract primitives"]
        ML["Meshletize<br/>128V/128T clusters"]
        TC["Texture Compress<br/>BC7, BC5, BC1"]
        MIP["Mip Generation<br/>Kaiser filter"]
        SC["Shader Compile<br/>DXC to DXIL/SPIR-V/Metal IR"]
        MAT["Material Convert<br/>PBR to shader graph"]
        TERR["Terrain Tile<br/>page table generation"]
        PACK["Bundle Packer<br/>combine + align"]
    end

    subgraph output["Cooked Output"]
        Bundle["Asset Bundles<br/>(.hab files)"]
        Manifest["Bundle Manifest<br/>(.habm)"]
        PSO_Cache["PSO Cache<br/>(.hpso)"]
    end

    GLTF --> MI --> ML --> PACK
    IMG --> TC --> MIP --> PACK
    SG --> SC --> PACK
    SG --> MAT --> PACK
    HMAP --> TERR --> PACK
    PACK --> Bundle
    PACK --> Manifest
    SC --> PSO_Cache
```

### Cook Pipeline API

```cpp
namespace harmonius::asset {

// A single imported asset before cooking
struct RawAsset {
    AssetId              id;
    AssetType            type;    // mesh, texture, material, terrain, shader
    std::filesystem::path source_path;
    std::vector<uint8_t> raw_data;
};

enum class AssetType : uint8_t {
    mesh,
    texture,
    material,
    terrain_tile,
    shader,
    acceleration_structure,
};

// Cooked asset — GPU-ready, platform-specific
struct CookedAsset {
    AssetId              id;
    AssetType            type;
    gpu::Backend         target_backend;
    std::vector<uint8_t> data;           // GPU-ready payload
    uint64_t             content_hash;
    CookedMetadata       metadata;       // format, dimensions, etc.
};

struct CookedMetadata {
    // Mesh metadata
    uint32_t meshlet_count   = 0;
    uint32_t vertex_count    = 0;
    uint32_t triangle_count  = 0;

    // Texture metadata
    rg::Format format        = rg::Format::undefined;
    uint32_t   width         = 0;
    uint32_t   height        = 0;
    uint32_t   mip_levels    = 0;
    uint32_t   array_layers  = 0;

    // Size for allocation
    uint64_t   gpu_size_bytes = 0;
};

class AssetCooker {
public:
    explicit AssetCooker(gpu::Backend target);

    // Cook a single asset
    [[nodiscard]]
    std::expected<CookedAsset, CookError> cook(const RawAsset& raw);

    // Cook batch (parallel)
    [[nodiscard]]
    std::vector<std::expected<CookedAsset, CookError>> cook_batch(
        std::span<const RawAsset> assets
    );

    // Register custom asset processor
    void register_processor(AssetType type,
                            std::move_only_function<CookedAsset(const RawAsset&)> fn);
};

} // namespace harmonius::asset
```

### Mesh Cooking: Meshletization

Meshes are decomposed into meshlet clusters of at most 128 vertices and 128 triangles
(R-3.4.1, F-3.1.1). The output is a tightly packed buffer of meshlet descriptors, vertex
data, and triangle index data.

```cpp
namespace harmonius::asset {

struct MeshletData {
    // Per-meshlet descriptor
    struct Meshlet {
        uint32_t vertex_offset;
        uint32_t triangle_offset;
        uint8_t  vertex_count;     // max 128
        uint8_t  triangle_count;   // max 128
        float    bounding_sphere[4]; // xyz center + radius
        float    normal_cone[4];     // xyz axis + cutoff (backface culling)
    };

    std::vector<Meshlet>  meshlets;
    std::vector<uint8_t>  vertex_data;    // interleaved position/normal/uv/tangent
    std::vector<uint8_t>  triangle_data;  // micro-indices within meshlet
    uint32_t              vertex_stride;
};

class MeshletBuilder {
public:
    [[nodiscard]]
    MeshletData build(std::span<const float> positions,
                      std::span<const uint32_t> indices,
                      uint32_t vertex_stride);
};

} // namespace harmonius::asset
```

### Texture Cooking: Compression + Mips

```cpp
namespace harmonius::asset {

struct TextureCookOptions {
    rg::Format target_format = rg::Format::bc7_unorm; // default block-compressed
    bool       generate_mips = true;
    bool       srgb          = false;
    uint32_t   max_dimension = 4096;
};

class TextureCompressor {
public:
    [[nodiscard]]
    CookedAsset compress(std::span<const uint8_t> rgba_data,
                         uint32_t width, uint32_t height,
                         const TextureCookOptions& options);
};

} // namespace harmonius::asset
```

---

## Asset Bundle Format

Cooked assets are packed into **asset bundles** (`.hab` files) optimized for sequential
reading and GPU upload. Each bundle is self-contained and can be streamed independently.

### Bundle Layout

```mermaid
flowchart TD
    subgraph hab["Asset Bundle (.hab)"]
        Header["Header<br/>magic, version, asset count,<br/>TOC offset, total size"]
        TOC["Table of Contents<br/>per-asset: id, type, offset,<br/>size, hash, chunk_id"]
        subgraph chunks["Data Chunks (aligned to 64KB)"]
            C0["Chunk 0<br/>meshlet buffers<br/>(vertex + index + descriptors)"]
            C1["Chunk 1<br/>textures mip 0<br/>(highest detail)"]
            C2["Chunk 2<br/>textures mip 1-N<br/>(lower mips)"]
            C3["Chunk 3<br/>material constants<br/>+ shader references"]
        end
    end

    Header --> TOC --> C0 --> C1 --> C2 --> C3
```

### Bundle Manifest

A separate manifest file (`.habm`) describes all bundles and their contents for the streaming
scheduler to make loading decisions without reading bundle headers:

```cpp
namespace harmonius::asset {

struct BundleManifest {
    struct BundleEntry {
        std::string          bundle_path;
        uint64_t             total_size;
        std::vector<AssetId> asset_ids;
    };

    struct AssetEntry {
        AssetId        id;
        AssetType      type;
        uint64_t       gpu_size;       // how much VRAM it needs
        uint32_t       bundle_index;   // which bundle contains it
        uint32_t       chunk_index;    // which chunk within the bundle
        uint64_t       chunk_offset;   // offset within chunk
        uint64_t       chunk_size;     // size within chunk
        uint16_t       priority_bias;  // static priority hint
        CookedMetadata metadata;
    };

    std::vector<BundleEntry> bundles;
    std::vector<AssetEntry>  assets;

    // Lookup by asset ID
    [[nodiscard]]
    const AssetEntry* find(AssetId id) const;
};

} // namespace harmonius::asset
```

---

## Buffer and Texture Combination

To optimize load times and reduce IO operations, multiple small assets are combined into
larger buffers and texture atlases within each chunk.

### Buffer Merging

Small meshlet buffers from multiple meshes are merged into a single large buffer per chunk.
Each mesh records its byte offset and size within the merged buffer. At runtime, the merged
buffer is uploaded as a single transfer operation and meshes index into it via base offset.

```mermaid
flowchart LR
    subgraph meshes["Individual Meshes"]
        M0["Mesh A<br/>12KB vertices<br/>4KB indices"]
        M1["Mesh B<br/>8KB vertices<br/>2KB indices"]
        M2["Mesh C<br/>20KB vertices<br/>8KB indices"]
    end

    subgraph merged["Merged Chunk Buffer"]
        VB["Vertex Region<br/>|A 12KB|B 8KB|C 20KB|<br/>40KB contiguous"]
        IB["Index Region<br/>|A 4KB|B 2KB|C 8KB|<br/>14KB contiguous"]
    end

    subgraph offsets["Offset Table"]
        OT["A: vtx=0, idx=0<br/>B: vtx=12K, idx=4K<br/>C: vtx=20K, idx=6K"]
    end

    meshes --> merged
    merged --> offsets
```

### Texture Atlasing

Small textures (UI icons, material detail maps) are packed into atlas pages. Each texture
records its UV rectangle within the atlas. Large textures (>512x512) remain standalone.

### Alignment Rules

All chunk data is aligned to 64KB boundaries to enable:
- **Direct Storage / Metal IO:** GPU-direct DMA requires sector alignment
- **Sparse texture pages:** tile granularity is typically 64KB
- **Memory-mapped IO:** page-aligned reads avoid copy overhead

```cpp
namespace harmonius::asset {

struct ChunkLayout {
    static constexpr uint64_t alignment = 65536; // 64KB

    struct Region {
        uint64_t offset;   // aligned to 64KB
        uint64_t size;
        AssetType type;
    };

    std::vector<Region> regions;
    uint64_t            total_size; // sum of all regions, 64KB-aligned
};

} // namespace harmonius::asset
```

---

## Streaming Chunk Architecture

Asset bundles are divided into streaming chunks — the smallest unit of IO. Each chunk can be
loaded independently, enabling progressive loading and priority-based streaming.

### Chunk Hierarchy

```mermaid
flowchart TD
    subgraph scene["Scene"]
        Zone0["Zone A<br/>visible"]
        Zone1["Zone B<br/>nearby"]
        Zone2["Zone C<br/>distant"]
    end

    subgraph bundles["Bundles per Zone"]
        B0["Bundle A.hab<br/>12 chunks"]
        B1["Bundle B.hab<br/>8 chunks"]
    end

    subgraph chunks["Chunks (64KB aligned)"]
        C_crit["Critical Chunks<br/>LOD 0 meshlets<br/>mip 0 textures"]
        C_high["High Priority<br/>LOD 1 meshlets<br/>mip 1-2 textures"]
        C_norm["Normal Priority<br/>LOD 2+ meshlets<br/>mip 3+ textures"]
        C_low["Low Priority<br/>distant LODs<br/>lowest mips"]
    end

    Zone0 --> B0
    Zone1 --> B1
    B0 --> C_crit
    B0 --> C_high
    B0 --> C_norm
    B1 --> C_norm
    B1 --> C_low
```

### Streaming Scheduler

The streaming scheduler decides which chunks to load based on camera position, visibility,
and priority. It interfaces with the render graph via transfer pass injection (RG-14.7).

```cpp
namespace harmonius::asset {

enum class StreamPriority : uint8_t {
    critical = 0,  // visible this frame, blocks rendering
    high     = 1,  // will be visible next few frames
    normal   = 2,  // predictive prefetch
    low      = 3,  // speculative background load
};

struct StreamRequest {
    AssetId        asset_id;
    uint32_t       chunk_index;
    StreamPriority priority;
    float          camera_distance;  // for priority refinement
};

class StreamingScheduler {
public:
    explicit StreamingScheduler(
        const BundleManifest& manifest,
        resource::PoolAllocator& pool,
        resource::RingAllocator& staging
    );

    // Submit streaming requests for this frame
    void request(std::span<const StreamRequest> requests);

    // Process requests: read from disk, upload to GPU
    // Returns transfer pass descriptors to inject into the render graph
    [[nodiscard]]
    std::vector<exec::TransferPassDesc> process_pending();

    // Handle eviction when pools are full
    void set_eviction_policy(
        std::move_only_function<std::vector<AssetId>(uint32_t needed)> policy
    );

    // Query residency state
    [[nodiscard]] bool is_resident(AssetId id) const;
    [[nodiscard]] float residency_ratio() const; // fraction of requested assets loaded

    // Platform-native IO backend
    void set_io_backend(std::unique_ptr<IOBackend> backend);
};

} // namespace harmonius::asset
```

### Platform-Native IO

Each platform uses its native high-performance IO path (R-1.2.4):

```cpp
namespace harmonius::asset {

class IOBackend {
public:
    virtual ~IOBackend() = default;

    // Submit async read from file to staging buffer
    virtual void submit_read(
        std::string_view path,
        uint64_t file_offset,
        uint64_t size,
        gpu::ResourceHandle staging_buffer,
        uint64_t buffer_offset
    ) = 0;

    // Poll for completed reads
    [[nodiscard]]
    virtual std::vector<IOCompletion> poll_completions() = 0;
};

struct IOCompletion {
    AssetId             asset_id;
    gpu::ResourceHandle staging_buffer;
    uint64_t            buffer_offset;
    uint64_t            size;
    bool                success;
};

// Platform implementations
class MetalIOBackend : public IOBackend {
    // Uses MTLIOCommandBuffer for GPU-direct file reads
};

class DirectStorageBackend : public IOBackend {
    // Uses DirectStorage for GPU decompression pipeline
};

class IoUringBackend : public IOBackend {
    // Uses io_uring for async file reads on Linux
};

} // namespace harmonius::asset
```

### GPU Decompression (R-2.12.9, F-6.2.8)

Asset chunks are stored compressed on disk. A compute pass decompresses them in GPU memory
after the transfer queue uploads the compressed data:

```mermaid
sequenceDiagram
    participant Disk
    participant Staging as Staging Buffer
    participant TX as Transfer Queue
    participant CS as Compute Queue
    participant Device as Device-Local Memory

    Disk->>Staging: io_uring / DirectStorage read
    TX->>Device: copy compressed data
    TX-->>CS: fence signal
    CS->>CS: decompress compute pass
    CS-->>Device: decompressed data ready
    CS->>CS: signal completion fence
```

---

## Resource Registry and Addressing

The resource registry maps asset IDs to live GPU resource handles and bindless descriptor
indices. It is the bridge between the asset system and the render graph.

### Asset Identity

```cpp
namespace harmonius::asset {

// Strongly-typed asset identifier — stable across sessions
// Computed as a hash of the asset source path and cook parameters
enum class AssetId : uint64_t { invalid = 0 };

// Generational handle — detects stale references (R-5.1.4)
struct AssetHandle {
    uint32_t index;
    uint32_t generation; // incremented on reuse

    [[nodiscard]] bool is_valid() const;
};

} // namespace harmonius::asset
```

### Resource Registry

```cpp
namespace harmonius::asset {

class ResourceRegistry {
public:
    explicit ResourceRegistry(gpu::Device& device,
                              BindlessDescriptorHeap& heap);

    // Register a cooked asset's GPU resource and get a handle
    [[nodiscard]]
    AssetHandle register_asset(AssetId id,
                               gpu::ResourceHandle gpu_resource,
                               const CookedMetadata& metadata);

    // Unregister (on eviction or destruction)
    void unregister(AssetHandle handle);

    // Resolve asset handle to GPU resource
    [[nodiscard]]
    std::expected<gpu::ResourceHandle, AssetError> resolve(AssetHandle handle) const;

    // Resolve to bindless descriptor index (for shader access)
    [[nodiscard]]
    std::expected<uint32_t, AssetError> resolve_descriptor_index(AssetHandle handle) const;

    // Resolve render graph ResourceHandle to asset
    [[nodiscard]]
    AssetHandle find_by_rg_resource(rg::ResourceHandle rg_handle) const;

    // Bulk query
    [[nodiscard]] uint32_t registered_count() const;
    [[nodiscard]] uint64_t total_gpu_bytes() const;

private:
    struct Entry {
        AssetId             id;
        gpu::ResourceHandle gpu_handle;
        uint32_t            descriptor_index;
        uint32_t            generation;
        CookedMetadata      metadata;
    };

    std::vector<Entry>                        entries_;
    std::unordered_map<AssetId, uint32_t>     id_to_index_;
    std::queue<uint32_t>                      free_list_;
    BindlessDescriptorHeap&                   heap_;
};

} // namespace harmonius::asset
```

---

## Bindless Descriptor Heap

A single global descriptor heap manages all GPU resources (R-2.12.10, F-6.2.9). Shaders
address resources by `uint32_t` index. The heap is imported into the render graph as a
persistent resource (RG-2.15).

```mermaid
flowchart TD
    subgraph heap["Global Bindless Descriptor Heap (1M slots)"]
        S0["Slot 0: scene_buffer"]
        S1["Slot 1: transform_buffer"]
        S2["Slot 2: light_buffer"]
        S3["Slot 3: shadow_atlas"]
        SN["Slot 4-N: material textures,<br/>streaming tiles,<br/>pool resources"]
        SF["Slot N+1: free list"]
    end

    subgraph shader["Shader Access"]
        CB["FrameConstants.texture_idx = 42"]
        RDH["ResourceDescriptorHeap[42]"]
    end

    subgraph rg_import["Render Graph Import"]
        Import["declare_imported(<br/>  bindless_heap,<br/>  heap_handle<br/>)"]
    end

    heap --> RDH
    CB --> RDH
    heap --> Import
```

### Heap Manager

```cpp
namespace harmonius::asset {

class BindlessDescriptorHeap {
public:
    explicit BindlessDescriptorHeap(gpu::Device& device,
                                    uint32_t max_descriptors = 1'000'000);

    // Allocate a descriptor slot and write the resource into it
    [[nodiscard]]
    std::expected<uint32_t, HeapError> allocate(gpu::ResourceHandle resource);

    // Free a descriptor slot back to the free list
    void free(uint32_t index);

    // Update an existing slot with a new resource (e.g., after streaming)
    void update(uint32_t index, gpu::ResourceHandle new_resource);

    // GPU handle for binding to the root signature
    [[nodiscard]] gpu::ResourceHandle gpu_handle() const;

    // Import into render graph
    [[nodiscard]] rg::ResourceHandle import_to_graph(rg::builder::GraphBuilder& builder) const;

    // Statistics
    [[nodiscard]] uint32_t allocated_count() const;
    [[nodiscard]] uint32_t capacity() const;

private:
    gpu::Device&         device_;
    gpu::ResourceHandle  heap_handle_;
    std::queue<uint32_t> free_list_;
    uint32_t             next_index_ = 0;
    uint32_t             max_descriptors_;
};

} // namespace harmonius::asset
```

---

## Integration with Render Graph

### How Assets Become Render Graph Resources

```mermaid
sequenceDiagram
    participant Cook as AssetCooker
    participant Bundle as BundlePacker
    participant Sched as StreamingScheduler
    participant IO as IOBackend
    participant Reg as ResourceRegistry
    participant Heap as BindlessHeap
    participant RG as Render Graph

    Note over Cook,RG: Offline (build time)
    Cook->>Cook: meshletize, compress, mip-gen
    Cook->>Bundle: pack into .hab chunks

    Note over Cook,RG: Runtime (loading)
    Sched->>IO: submit_read(chunk)
    IO-->>Sched: IOCompletion
    Sched->>RG: inject_transfer(staging to device)
    RG->>RG: transfer queue upload
    RG-->>Sched: completion fence

    Note over Cook,RG: Runtime (registration)
    Sched->>Reg: register_asset(id, gpu_handle, metadata)
    Reg->>Heap: allocate(gpu_handle)
    Heap-->>Reg: descriptor_index=42
    Reg-->>Sched: AssetHandle

    Note over Cook,RG: Runtime (rendering)
    RG->>RG: pass callback executes
    RG->>Reg: resolve_descriptor_index(handle)
    Reg-->>RG: 42
    RG->>RG: push_constants.texture_idx = 42
    RG->>RG: shader reads ResourceDescriptorHeap[42]
```

### Addressing Summary

| Layer             | Handle Type              | Scope              | Lifetime            |
| ----------------- | ------------------------ | ------------------ | ------------------- |
| Asset identity    | `AssetId` (uint64_t)     | Global, stable     | Permanent           |
| Asset instance    | `AssetHandle` (gen+idx)  | Per-session        | Until evicted       |
| GPU resource      | `gpu::ResourceHandle`    | Per-device         | Until destroyed     |
| Descriptor index  | `uint32_t`               | Per-heap           | Until freed         |
| Render graph slot | `rg::ResourceHandle`     | Per-graph          | Until graph rebuilt  |
| Shader access     | `uint32_t` push constant | Per-draw           | Single draw call    |

### Render Graph Resource Binding Flow

The render graph never sees `AssetId` or `AssetHandle` directly. It operates on
`rg::ResourceHandle` slots that are bound to `gpu::ResourceHandle` values each frame.
The asset system provides the mapping:

```
AssetId  ──[ResourceRegistry]──>  gpu::ResourceHandle
                                        │
                                        ▼
rg::ResourceHandle  ──[Executor::bind_resource()]──>  gpu::ResourceHandle
                                                            │
                                                            ▼
PassContext::resolve()  ──>  gpu::ResourceHandle  ──[BindlessHeap]──>  uint32_t index
                                                                          │
                                                                          ▼
                                                              Shader: ResourceDescriptorHeap[index]
```

This indirection allows the streaming system to swap GPU resources (e.g., upgrading a
texture's mip level) without the render graph knowing. The binding is updated at frame
boundaries via `Executor::bind_resource()`, and the graph continues executing with the
new resource.
