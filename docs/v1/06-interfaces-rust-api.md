# Harmonius - Public Safe Rust API

This document is the exhaustive interface specification for the public, 100% safe
Rust API surface of Harmonius. No unsafe code appears here or on any user thread.
All unsafe code is confined to background render/IO threads and C++ backends
accessed via `cxx.rs`.

**Crate layout:**

| Crate | Role |
|---|---|
| `harmonius` | Main entry point; re-exports the full public surface |
| `harmonius-types` | Shared enums, handles, and error types (leaf crate, no deps) |
| `harmonius-graph` | Render graph builder and compiler |
| `harmonius-scene` | Scene graph, transforms, spatial index |
| `harmonius-anim` | Animation clips, state machines, IK |
| `harmonius-ui` | Retained-mode UI, sprites, tilemaps |
| `harmonius-shaders` | Shader graph compiler (Naga front-end) |

---

## 1. Core Types (`harmonius-types`)

### 1.1 `TextureFormat`

Pixel format for textures and render targets. Covers every format required by
Metal 4, Vulkan 1.4, and D3D12.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TextureFormat {
    // 8-bit unorm
    R8Unorm,
    Rg8Unorm,
    Rgba8Unorm,
    Rgba8UnormSrgb,
    Bgra8Unorm,
    Bgra8UnormSrgb,

    // 8-bit snorm / uint / sint
    R8Snorm,
    Rg8Snorm,
    Rgba8Snorm,
    R8Uint,
    R8Sint,
    Rgba8Uint,
    Rgba8Sint,

    // 16-bit float
    R16Float,
    Rg16Float,
    Rgba16Float,

    // 16-bit unorm / uint / sint
    R16Unorm,
    Rg16Unorm,
    Rgba16Unorm,
    R16Uint,
    R16Sint,

    // 32-bit float / uint
    R32Float,
    Rg32Float,
    Rgba32Float,
    R32Uint,
    Rg32Uint,
    Rgba32Uint,

    // Packed formats
    Rgb10a2Unorm,
    Rg11b10Float,
    Rgb9e5Float,

    // Block-compressed
    Bc1RgbaUnorm,
    Bc1RgbaUnormSrgb,
    Bc2RgbaUnorm,
    Bc2RgbaUnormSrgb,
    Bc3RgbaUnorm,
    Bc3RgbaUnormSrgb,
    Bc4RUnorm,
    Bc4RSnorm,
    Bc5RgUnorm,
    Bc5RgSnorm,
    Bc6hRgbUfloat,
    Bc6hRgbSfloat,
    Bc7RgbaUnorm,
    Bc7RgbaUnormSrgb,

    // Depth / stencil
    Depth16Unorm,
    Depth32Float,
    Depth24PlusStencil8,
    Depth32FloatStencil8,
    Stencil8,
}

impl TextureFormat {
    /// Returns `true` if this format has a depth component.
    pub fn has_depth(self) -> bool;

    /// Returns `true` if this format has a stencil component.
    pub fn has_stencil(self) -> bool;

    /// Returns `true` if this format is a block-compressed format.
    pub fn is_compressed(self) -> bool;

    /// Returns the size in bytes of a single texel (0 for compressed formats).
    pub fn texel_byte_size(self) -> u32;

    /// Returns `(block_width, block_height)` — both 1 for uncompressed formats.
    pub fn block_dimensions(self) -> (u32, u32);

    /// Returns the bytes per block (equals texel_byte_size for uncompressed).
    pub fn bytes_per_block(self) -> u32;
}
```

---

### 1.2 `BufferUsage`

Bitflag describing how a GPU buffer will be used. Multiple flags may be combined.

```rust
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BufferUsage: u32 {
        /// Can be used as a source for copy / blit operations.
        const COPY_SRC          = 1 << 0;
        /// Can be written to by copy / blit operations.
        const COPY_DST          = 1 << 1;
        /// Bound as a uniform / constant buffer.
        const UNIFORM           = 1 << 2;
        /// Bound as a storage buffer (read-write in shaders).
        const STORAGE           = 1 << 3;
        /// Holds mesh index data.
        const INDEX             = 1 << 4;
        /// Holds meshlet / vertex attribute data.
        const VERTEX            = 1 << 5;
        /// Used as a GPU indirect dispatch / draw arguments buffer.
        const INDIRECT          = 1 << 6;
        /// Used as a GPU-side acceleration structure scratch or build buffer.
        const ACCELERATION_STRUCTURE = 1 << 7;
        /// Read back to CPU (staging).
        const MAP_READ          = 1 << 8;
        /// Written from CPU (upload).
        const MAP_WRITE         = 1 << 9;
    }
}
```

---

### 1.3 `ShaderStage`

Bitflag identifying which shader stages a resource binding or barrier applies to.

```rust
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ShaderStage: u32 {
        const TASK      = 1 << 0;   // Metal object stage
        const MESH      = 1 << 1;
        const FRAGMENT  = 1 << 2;
        const COMPUTE   = 1 << 3;
        const RAY_GEN   = 1 << 4;
        const CLOSEST_HIT = 1 << 5;
        const ANY_HIT   = 1 << 6;
        const MISS      = 1 << 7;
        const CALLABLE  = 1 << 8;
    }
}
```

---

### 1.4 `Handle<T>`

A typed, generation-counted handle to a GPU resource. Cheap to copy; does not own
the resource. The backend validates the generation on access.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle<T> {
    index: u32,
    generation: u32,
    _phantom: PhantomData<fn() -> T>,
}

impl<T> Handle<T> {
    /// Returns the underlying raw index. Intended for serialization / tooling only.
    pub fn raw_index(self) -> u32;

    /// Returns the generation counter used for stale-handle detection.
    pub fn generation(self) -> u32;

    /// Returns `true` if this is the null (invalid) sentinel.
    pub fn is_null(self) -> bool;

    /// Returns the null sentinel handle.
    pub fn null() -> Self;
}
```

---

### 1.5 `LoadOp`

Action taken on a render target attachment at the beginning of a render pass.

| Variant | Description |
|---|---|
| `Load` | Preserve previous contents |
| `Clear(ClearValue)` | Fill with the given clear value |
| `DontCare` | Discard previous contents (undefined after) |

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum LoadOp {
    Load,
    Clear(ClearValue),
    DontCare,
}
```

---

### 1.6 `StoreOp`

Action taken on a render target attachment at the end of a render pass.

| Variant | Description |
|---|---|
| `Store` | Write results to memory |
| `DontCare` | Discard (transient attachment, stays on-tile) |
| `Resolve` | MSAA resolve to a non-MSAA target |

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreOp {
    Store,
    DontCare,
    Resolve,
}
```

---

### 1.7 `ClearValue`

Union-like type for clearing color or depth/stencil attachments.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ClearValue {
    Color { r: f32, g: f32, b: f32, a: f32 },
    DepthStencil { depth: f32, stencil: u8 },
}

impl ClearValue {
    pub fn black_alpha_one() -> Self;
    pub fn white() -> Self;
    pub fn transparent() -> Self;
    pub fn depth_zero_stencil_zero() -> Self;
    pub fn depth_one_stencil_zero() -> Self;
}
```

---

### 1.8 `CompareFunction`

Comparison function used in depth test, stencil test, and sampler comparison.

| Variant |
|---|
| `Never` |
| `Less` |
| `Equal` |
| `LessEqual` |
| `Greater` |
| `NotEqual` |
| `GreaterEqual` |
| `Always` |

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompareFunction {
    Never,
    Less,
    Equal,
    LessEqual,
    Greater,
    NotEqual,
    GreaterEqual,
    Always,
}
```

---

### 1.9 `BlendState`

Describes per-attachment alpha blending for a render target.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlendFactor {
    Zero,
    One,
    Src,
    OneMinusSrc,
    SrcAlpha,
    OneMinusSrcAlpha,
    Dst,
    OneMinusDst,
    DstAlpha,
    OneMinusDstAlpha,
    SrcAlphaSaturate,
    Constant,
    OneMinusConstant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlendOp {
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlendComponent {
    pub src_factor: BlendFactor,
    pub dst_factor: BlendFactor,
    pub operation: BlendOp,
}

impl BlendComponent {
    pub const OVER: Self;       // Standard "over" compositing
    pub const REPLACE: Self;    // No blending; dst overwritten
    pub const ADD: Self;        // Additive
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlendState {
    pub color: BlendComponent,
    pub alpha: BlendComponent,
}

impl BlendState {
    pub const ALPHA_BLENDING: Self;
    pub const PREMULTIPLIED_ALPHA: Self;
    pub const ADDITIVE: Self;
    pub const REPLACE: Self;
}
```

---

### 1.10 `PrimitiveTopology`

Primitive assembly topology passed to the mesh shader pipeline. (No strip
topologies; mesh shaders output explicit index lists.)

| Variant | Description |
|---|---|
| `TriangleList` | Default; 3 indices per triangle |
| `LineList` | 2 indices per line segment |
| `PointList` | 1 index per point |

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimitiveTopology {
    TriangleList,
    LineList,
    PointList,
}
```

---

### 1.11 Error Types

```rust
/// Top-level error type returned by fallible public API calls.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum HarmoniusError {
    #[error("Device creation failed: {0}")]
    DeviceCreation(String),

    #[error("Capability not supported: {0:?}")]
    CapabilityNotSupported(DeviceCapability),

    #[error("Render graph compilation failed: {0}")]
    GraphCompilation(GraphCompileError),

    #[error("Resource allocation failed: {0}")]
    ResourceAllocation(ResourceError),

    #[error("Out of device memory")]
    OutOfDeviceMemory,

    #[error("Invalid handle: index {index}, generation {generation}")]
    InvalidHandle { index: u32, generation: u32 },

    #[error("Shader compilation failed: {0}")]
    ShaderCompilation(ShaderCompileError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Errors emitted by the render graph compiler.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum GraphCompileError {
    #[error("Cycle detected involving pass '{0}'")]
    CycleDetected(String),

    #[error("Resource '{resource}' has conflicting usages in pass '{pass}'")]
    ConflictingResourceUsage { resource: String, pass: String },

    #[error("Required pass '{0}' was culled by feature gate")]
    RequiredPassCulled(String),

    #[error("Unresolved resource reference '{0}'")]
    UnresolvedResourceRef(String),
}

/// Errors emitted by resource allocation.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ResourceError {
    #[error("Texture dimensions {width}x{height}x{depth} exceed device limits")]
    DimensionsExceedLimit { width: u32, height: u32, depth: u32 },

    #[error("Format {0:?} is not supported for the requested usage")]
    UnsupportedFormatUsage(TextureFormat),

    #[error("Buffer size {0} bytes exceeds device limit")]
    BufferSizeExceedsLimit(u64),
}

/// Errors emitted by the shader graph compiler.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ShaderCompileError {
    #[error("Type mismatch at node '{node}', input '{input}': expected {expected}, got {found}")]
    TypeMismatch { node: String, input: String, expected: String, found: String },

    #[error("Unconnected required input '{input}' on node '{node}'")]
    UnconnectedInput { node: String, input: String },

    #[error("Naga validation failed: {0}")]
    NagaValidation(String),

    #[error("Unsupported node kind '{0}' for target backend")]
    UnsupportedNodeKind(String),
}

pub type Result<T> = std::result::Result<T, HarmoniusError>;
```

---

## 2. Device & Application (`harmonius`)

### 2.1 `BackendType`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackendType {
    /// Metal 4 (macOS / Apple Silicon).
    Metal,
    /// Vulkan 1.4 (Windows, SteamOS, Linux).
    Vulkan,
    /// Direct3D 12 Agility SDK (Windows).
    Direct3D12,
}
```

---

### 2.2 `DeviceCapability`

Individual GPU capability flags. Queried from `DeviceCapabilities`.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DeviceCapability {
    /// Object/task + mesh shader pipeline.
    MeshShaders,
    /// Hardware ray tracing (BLAS/TLAS, inline RT or pipeline).
    RayTracing,
    /// Bindless resource heap (Tier 2+ argument buffers / descriptor indexing).
    Bindless,
    /// Async compute queue distinct from the graphics queue.
    AsyncCompute,
    /// Transfer / copy queue distinct from graphics.
    TransferQueue,
    /// Platform-specific direct IO (MTLIOCommandBuffer / DirectStorage).
    DirectIo,
    /// Sparse texture residency.
    SparseTextures,
    /// Sparse 3D texture residency.
    SparseTextures3D,
    /// 64-bit integer atomics in shaders.
    ShaderInt64Atomics,
    /// Variable-rate shading.
    VariableRateShading,
    /// Conservative rasterisation.
    ConservativeRasterization,
}
```

---

### 2.3 `DeviceCapabilities`

Immutable snapshot of what the selected GPU supports.

```rust
#[derive(Debug, Clone)]
pub struct DeviceCapabilities {
    pub backend: BackendType,
    pub device_name: String,
    pub vendor_id: u32,
    pub device_id: u32,

    // Memory
    pub total_vram_bytes: u64,
    pub shared_memory_bytes: u64,
    pub is_unified_memory: bool,

    // Limits
    pub max_texture_dimension_2d: u32,
    pub max_texture_dimension_3d: u32,
    pub max_bind_groups: u32,
    pub max_push_constant_size_bytes: u32,
    pub max_meshlet_vertices: u32,
    pub max_meshlet_primitives: u32,
    pub max_task_payload_bytes: u32,

    // Capability flags
    pub supported: std::collections::HashSet<DeviceCapability>,
}

impl DeviceCapabilities {
    /// Returns `true` if the given capability is present.
    pub fn supports(&self, cap: DeviceCapability) -> bool;

    /// Returns `true` if every capability in `caps` is present.
    pub fn supports_all(&self, caps: &[DeviceCapability]) -> bool;

    /// Panics unless every capability in `caps` is present (convenience assertion).
    pub fn require(&self, caps: &[DeviceCapability]);
}
```

---

### 2.4 `Device`

An opened GPU device. The entry point for resource allocation and graph building.
Obtained from `HarmoniusApp::open_device`.

```rust
pub struct Device { /* opaque */ }

impl Device {
    /// Returns the immutable capability set for this device.
    pub fn capabilities(&self) -> &DeviceCapabilities;

    /// Returns the backend type.
    pub fn backend(&self) -> BackendType;

    // --- Resource allocation ---

    /// Allocates a GPU buffer.
    pub fn create_buffer(&self, desc: &BufferDesc) -> Result<Handle<Buffer>>;

    /// Allocates a GPU texture.
    pub fn create_texture(&self, desc: &TextureDesc) -> Result<Handle<Texture>>;

    /// Allocates a sampler.
    pub fn create_sampler(&self, desc: &SamplerDesc) -> Result<Handle<Sampler>>;

    /// Creates a render target view over an existing texture handle.
    pub fn create_render_target(
        &self,
        desc: &RenderTargetDesc,
    ) -> Result<Handle<RenderTarget>>;

    /// Destroys a buffer, freeing its GPU memory at the next safe point.
    pub fn destroy_buffer(&self, handle: Handle<Buffer>);

    /// Destroys a texture, freeing its GPU memory at the next safe point.
    pub fn destroy_texture(&self, handle: Handle<Texture>);

    /// Destroys a sampler.
    pub fn destroy_sampler(&self, handle: Handle<Sampler>);

    /// Destroys a render target view.
    pub fn destroy_render_target(&self, handle: Handle<RenderTarget>);

    // --- Bindless ---

    /// Returns the device-global bindless heap.
    pub fn bindless_heap(&self) -> &BindlessHeap;

    // --- Graph building ---

    /// Creates a new render graph builder for this device.
    pub fn new_render_graph(&self) -> RenderGraph;

    // --- Swap chain ---

    /// Creates a swap chain for the given raw window handle.
    pub fn create_swap_chain(
        &self,
        desc: &SwapChainDesc,
    ) -> Result<SwapChain>;
}
```

---

### 2.5 `SwapChainDesc`

```rust
#[derive(Debug, Clone)]
pub struct SwapChainDesc {
    /// Raw window handle (from `raw-window-handle` crate).
    pub window: raw_window_handle::RawWindowHandle,
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    /// Number of back buffers (2 = double-buffering, 3 = triple-buffering).
    pub buffer_count: u32,
    /// Enable vertical sync.
    pub vsync: bool,
    /// HDR10 or scRGB output when supported.
    pub hdr: bool,
}
```

---

### 2.6 `SwapChain`

```rust
pub struct SwapChain { /* opaque */ }

impl SwapChain {
    /// Returns a handle to the current back buffer texture.
    pub fn current_back_buffer(&self) -> Handle<Texture>;

    /// Resizes the swap chain to the new dimensions.
    pub fn resize(&mut self, width: u32, height: u32) -> Result<()>;

    /// Returns the current width in pixels.
    pub fn width(&self) -> u32;

    /// Returns the current height in pixels.
    pub fn height(&self) -> u32;

    /// Returns the pixel format of the back buffers.
    pub fn format(&self) -> TextureFormat;
}
```

---

### 2.7 `HarmoniusApp`

Top-level application object. Entry point for device enumeration and creation.

```rust
pub struct HarmoniusApp { /* opaque */ }

impl HarmoniusApp {
    /// Creates the application context. Should be called once.
    pub fn new() -> Result<Self>;

    /// Enumerates all physical adapters visible to the selected backend.
    pub fn enumerate_adapters(&self, backend: BackendType) -> Vec<AdapterInfo>;

    /// Opens the preferred GPU device. Uses the first discrete GPU if available,
    /// or integrated otherwise.
    pub fn open_device(&self, backend: BackendType) -> Result<Device>;

    /// Opens a specific GPU device by adapter index from `enumerate_adapters`.
    pub fn open_device_by_adapter(
        &self,
        backend: BackendType,
        adapter_index: usize,
    ) -> Result<Device>;
}

/// Lightweight descriptor for an available GPU adapter.
#[derive(Debug, Clone)]
pub struct AdapterInfo {
    pub name: String,
    pub vendor_id: u32,
    pub device_id: u32,
    pub is_discrete: bool,
    pub vram_bytes: u64,
    pub backend: BackendType,
}
```

---

## 3. Render Graph (`harmonius-graph`)

### 3.1 `RenderGraph`

Declarative, frame-invariant description of all render, compute, and transfer
passes. Mutable during the build phase; sealed by calling `compile()`.

```rust
pub struct RenderGraph { /* opaque */ }

impl RenderGraph {
    /// Declares a render (rasterization) pass and returns a builder.
    pub fn add_render_pass(&mut self, desc: RenderPassDesc) -> PassBuilder<RenderPassDesc>;

    /// Declares a compute pass and returns a builder.
    pub fn add_compute_pass(&mut self, desc: ComputePassDesc) -> PassBuilder<ComputePassDesc>;

    /// Declares a transfer (copy/blit/IO) pass and returns a builder.
    pub fn add_transfer_pass(&mut self, desc: TransferPassDesc) -> PassBuilder<TransferPassDesc>;

    /// Declares a buffer resource that the graph owns.
    pub fn create_buffer(&mut self, desc: BufferDesc) -> ResourceRef<Buffer>;

    /// Declares a texture resource that the graph owns.
    pub fn create_texture(&mut self, desc: TextureDesc) -> ResourceRef<Texture>;

    /// Imports an external (device-owned) buffer into the graph.
    pub fn import_buffer(&mut self, handle: Handle<Buffer>) -> ResourceRef<Buffer>;

    /// Imports an external (device-owned) texture into the graph.
    pub fn import_texture(&mut self, handle: Handle<Texture>) -> ResourceRef<Texture>;

    /// Attaches a `FeatureGate` to a pass by name. The pass is culled if the
    /// gate evaluates to `false` during compilation.
    pub fn gate_pass_on_feature(
        &mut self,
        pass_name: &str,
        gate: FeatureGate,
    ) -> Result<()>;

    /// Attaches a `BudgetGate` to a pass. The pass is culled at runtime when
    /// the active `RuntimeBudget` tier is below the gate threshold.
    pub fn gate_pass_on_budget(
        &mut self,
        pass_name: &str,
        gate: BudgetGate,
    ) -> Result<()>;

    /// Compiles the graph into an immutable `ExecutionPlan`.
    /// Performs: feature detection → transitive node culling → topological sort
    /// → resource lifetime analysis → barrier insertion → queue assignment.
    pub fn compile(self, caps: &DeviceCapabilities) -> Result<Arc<ExecutionPlan>>;
}
```

---

### 3.2 `PassBuilder<D>`

Fluent builder returned by `RenderGraph::add_*_pass`. Declares resource edges.

```rust
pub struct PassBuilder<'g, D> { /* opaque */ }

impl<'g, D> PassBuilder<'g, D> {
    /// Declares a read dependency on a buffer.
    pub fn reads_buffer(self, r: ResourceRef<Buffer>, stage: ShaderStage) -> Self;

    /// Declares a read dependency on a texture.
    pub fn reads_texture(self, r: ResourceRef<Texture>, stage: ShaderStage) -> Self;

    /// Declares a write dependency on a buffer (storage write or copy dst).
    pub fn writes_buffer(self, r: ResourceRef<Buffer>, stage: ShaderStage) -> Self;

    /// Declares a write dependency on a texture (storage write or color/depth attachment).
    pub fn writes_texture(self, r: ResourceRef<Texture>, stage: ShaderStage) -> Self;

    /// Declares an explicit ordering dependency on another pass by name,
    /// without a resource edge.
    pub fn depends_on(self, pass_name: &str) -> Self;

    /// Marks this pass as required. A `GraphCompileError::RequiredPassCulled`
    /// error is emitted if the pass would be culled.
    pub fn required(self) -> Self;

    /// Sets the queue preference for this pass.
    pub fn prefer_queue(self, queue: QueueType) -> Self;

    /// Finishes the builder, registering the pass in the graph.
    pub fn finish(self);
}
```

---

### 3.3 `QueueType`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QueueType {
    Graphics,
    AsyncCompute,
    Transfer,
}
```

---

### 3.4 `RenderPassDesc`

Descriptor for a rasterization pass.

```rust
#[derive(Debug, Clone)]
pub struct RenderPassDesc {
    /// Unique name; used in error messages and dependency lookups.
    pub name: String,

    /// Color attachments.
    pub color_attachments: Vec<ColorAttachmentDesc>,

    /// Optional depth/stencil attachment.
    pub depth_stencil_attachment: Option<DepthStencilAttachmentDesc>,

    /// If `true`, a MSAA resolve pass is automatically appended.
    pub resolve_msaa: bool,
}

#[derive(Debug, Clone)]
pub struct ColorAttachmentDesc {
    pub resource: ResourceRef<Texture>,
    pub load_op: LoadOp,
    pub store_op: StoreOp,
    /// Optional MSAA resolve target.
    pub resolve_target: Option<ResourceRef<Texture>>,
}

#[derive(Debug, Clone)]
pub struct DepthStencilAttachmentDesc {
    pub resource: ResourceRef<Texture>,
    pub depth_load_op: LoadOp,
    pub depth_store_op: StoreOp,
    pub stencil_load_op: LoadOp,
    pub stencil_store_op: StoreOp,
    pub read_only: bool,
}
```

---

### 3.5 `ComputePassDesc`

```rust
#[derive(Debug, Clone)]
pub struct ComputePassDesc {
    /// Unique name.
    pub name: String,
    /// If `true`, the compiler may schedule this pass on the async compute queue.
    pub allow_async: bool,
}
```

---

### 3.6 `TransferPassDesc`

```rust
#[derive(Debug, Clone)]
pub struct TransferPassDesc {
    /// Unique name.
    pub name: String,
    /// If `true`, this pass runs on the dedicated transfer/copy queue.
    pub use_dedicated_transfer_queue: bool,
}
```

---

### 3.7 `ResourceRef<T>`

A typed reference to a graph-owned or imported resource. Lightweight, `Copy`.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceRef<T> {
    id: u32,
    _phantom: PhantomData<fn() -> T>,
}

impl<T> ResourceRef<T> {
    /// Returns `true` if this reference was created by `import_*` rather than
    /// `create_*`.
    pub fn is_imported(self) -> bool;
}
```

---

### 3.8 `ExecutionPlan`

Immutable, compiled, thread-safe execution plan produced by `RenderGraph::compile`.
Shared via `Arc` with render threads.

```rust
pub struct ExecutionPlan { /* opaque */ }

impl ExecutionPlan {
    /// Returns the ordered list of pass names that survived culling.
    pub fn active_passes(&self) -> &[String];

    /// Returns `true` if the named pass survived compilation and is active.
    pub fn is_pass_active(&self, name: &str) -> bool;

    /// Returns the set of `DeviceCapability` flags the plan actually requires.
    pub fn required_capabilities(&self) -> &std::collections::HashSet<DeviceCapability>;

    /// Returns per-pass resource aliasing statistics (for profiling / debugging).
    pub fn resource_aliasing_stats(&self) -> Vec<AliasingStat>;
}

#[derive(Debug, Clone)]
pub struct AliasingStat {
    pub resource_name: String,
    pub aliased_with: String,
    pub saved_bytes: u64,
}
```

---

### 3.9 `RuntimeBudget`

Per-frame quality tier supplied to the executor. Budget-gated passes are culled
when the current tier is below their threshold.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BudgetTier {
    /// Absolute minimum; only hard-required passes execute.
    Minimum = 0,
    /// Low quality: basic shadows, SSAO, no RT.
    Low = 1,
    /// Medium quality: 4-cascade CSM, GTAO.
    Medium = 2,
    /// High quality: RT reflections, soft shadows.
    High = 3,
    /// Ultra quality: full RT GI, DDGI, max cascades.
    Ultra = 4,
}

#[derive(Debug, Clone)]
pub struct RuntimeBudget {
    /// Target GPU frame time in microseconds.
    pub target_frame_time_us: u32,
    /// Current quality tier; passes gated above this tier are culled.
    pub tier: BudgetTier,
    /// If `true`, the executor dynamically lowers the tier when frame time is
    /// exceeded.
    pub dynamic_adjustment: bool,
}

impl Default for RuntimeBudget {
    fn default() -> Self;
}
```

---

### 3.10 `FeatureGate`

Declarative predicate evaluated at compile time against `DeviceCapabilities`.

```rust
#[derive(Debug, Clone)]
pub enum FeatureGate {
    /// Pass is always enabled.
    Always,
    /// Pass requires the given capability.
    Requires(DeviceCapability),
    /// Pass requires all listed capabilities.
    RequiresAll(Vec<DeviceCapability>),
    /// Pass requires at least one of the listed capabilities.
    RequiresAny(Vec<DeviceCapability>),
    /// Logical NOT of another gate.
    Not(Box<FeatureGate>),
    /// Logical AND of two gates.
    And(Box<FeatureGate>, Box<FeatureGate>),
    /// Logical OR of two gates.
    Or(Box<FeatureGate>, Box<FeatureGate>),
}

impl FeatureGate {
    /// Evaluates the gate against the given capabilities snapshot.
    pub fn evaluate(&self, caps: &DeviceCapabilities) -> bool;
}
```

---

### 3.11 `BudgetGate`

Declarative predicate evaluated at runtime against the active `BudgetTier`.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BudgetGate {
    /// The minimum tier required for the pass to execute.
    pub minimum_tier: BudgetTier,
}

impl BudgetGate {
    pub fn at_least(tier: BudgetTier) -> Self;

    /// Evaluates against the current budget.
    pub fn evaluate(self, budget: &RuntimeBudget) -> bool;
}
```

---

## 4. Resource Descriptors & Management

### 4.1 `BufferDesc`

```rust
#[derive(Debug, Clone)]
pub struct BufferDesc {
    pub label: Option<String>,
    pub size_bytes: u64,
    pub usage: BufferUsage,
    /// If `true`, CPU-write contents are persistently mapped.
    pub cpu_visible: bool,
}
```

---

### 4.2 `TextureDesc`

```rust
#[derive(Debug, Clone)]
pub struct TextureDesc {
    pub label: Option<String>,
    pub dimension: TextureDimension,
    pub format: TextureFormat,
    pub width: u32,
    pub height: u32,
    /// Depth for 3D textures; array layers for 2D arrays; 1 otherwise.
    pub depth_or_array_layers: u32,
    pub mip_level_count: u32,
    /// MSAA sample count (1 = no MSAA).
    pub sample_count: u32,
    pub usage: TextureUsage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureDimension {
    D1,
    D2,
    D3,
    Cube,
    D2Array,
    CubeArray,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TextureUsage: u32 {
        const COPY_SRC          = 1 << 0;
        const COPY_DST          = 1 << 1;
        const TEXTURE_BINDING   = 1 << 2;   // Sampled in shader
        const STORAGE_BINDING   = 1 << 3;   // Read/write in shader
        const RENDER_ATTACHMENT = 1 << 4;
        const DEPTH_ATTACHMENT  = 1 << 5;
    }
}
```

---

### 4.3 `RenderTargetDesc`

```rust
#[derive(Debug, Clone)]
pub struct RenderTargetDesc {
    pub texture: Handle<Texture>,
    /// Mip level to target.
    pub mip_level: u32,
    /// Array layer or cube face to target (0 for non-array).
    pub array_layer: u32,
}
```

---

### 4.4 `SamplerDesc`

```rust
#[derive(Debug, Clone)]
pub struct SamplerDesc {
    pub label: Option<String>,
    pub address_mode_u: AddressMode,
    pub address_mode_v: AddressMode,
    pub address_mode_w: AddressMode,
    pub mag_filter: FilterMode,
    pub min_filter: FilterMode,
    pub mipmap_filter: FilterMode,
    pub lod_min_clamp: f32,
    pub lod_max_clamp: f32,
    pub compare: Option<CompareFunction>,
    pub anisotropy_clamp: u16,
    pub border_color: Option<SamplerBorderColor>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AddressMode {
    ClampToEdge,
    Repeat,
    MirrorRepeat,
    ClampToBorder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FilterMode {
    Nearest,
    Linear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SamplerBorderColor {
    TransparentBlack,
    OpaqueBlack,
    OpaqueWhite,
}
```

---

### 4.5 `BindlessHeap`

Global descriptor heap. All resource handles are registered here automatically
on creation. Shaders access resources by `u32` index.

```rust
pub struct BindlessHeap { /* opaque */ }

impl BindlessHeap {
    /// Returns the bindless index of a buffer, valid in shaders as a `u32`.
    pub fn buffer_index(&self, handle: Handle<Buffer>) -> Result<u32>;

    /// Returns the bindless index of a texture (sampled view).
    pub fn texture_srv_index(&self, handle: Handle<Texture>) -> Result<u32>;

    /// Returns the bindless index of a texture (storage/UAV view).
    pub fn texture_uav_index(&self, handle: Handle<Texture>) -> Result<u32>;

    /// Returns the bindless index of a sampler.
    pub fn sampler_index(&self, handle: Handle<Sampler>) -> Result<u32>;

    /// Returns the current total number of registered descriptors.
    pub fn descriptor_count(&self) -> u32;

    /// Returns the maximum capacity of the heap.
    pub fn capacity(&self) -> u32;
}
```

---

### 4.6 `ResourceHandle`

Convenience enum for passing any resource type where a unified handle is needed
(e.g., readback, debug tooling).

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceHandle {
    Buffer(Handle<Buffer>),
    Texture(Handle<Texture>),
    Sampler(Handle<Sampler>),
    RenderTarget(Handle<RenderTarget>),
}
```

---

## 5. Scene (`harmonius-scene`)

### 5.1 `Scene`

Top-level scene container. Owns entities and their transforms.

```rust
pub struct Scene { /* opaque */ }

impl Scene {
    pub fn new() -> Self;

    /// Spawns a new entity and returns its handle.
    pub fn spawn(&mut self) -> Entity;

    /// Spawns a child entity parented to `parent`.
    pub fn spawn_child(&mut self, parent: Entity) -> Entity;

    /// Despawns an entity and all its children.
    pub fn despawn(&mut self, entity: Entity);

    /// Returns `true` if the entity handle is valid.
    pub fn is_alive(&self, entity: Entity) -> bool;

    /// Returns the transform for an entity.
    pub fn transform(&self, entity: Entity) -> Option<&Transform>;

    /// Returns a mutable transform for an entity.
    pub fn transform_mut(&mut self, entity: Entity) -> Option<&mut Transform>;

    /// Computes and caches all world-space transforms for the current frame.
    pub fn flush_transforms(&mut self);

    /// Returns the computed world-space transform for an entity.
    /// `None` if `flush_transforms` has not been called after the last mutation.
    pub fn world_transform(&self, entity: Entity) -> Option<glam::Mat4>;

    /// Returns the `SpatialIndex` for broad-phase queries.
    pub fn spatial_index(&self) -> &SpatialIndex;

    /// Returns a mutable `SpatialIndex` to insert/remove entries.
    pub fn spatial_index_mut(&mut self) -> &mut SpatialIndex;
}
```

---

### 5.2 `Entity`

A generation-counted entity identifier. `Copy`, `Eq`, `Hash`.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    index: u32,
    generation: u32,
}

impl Entity {
    pub fn is_null(self) -> bool;
    pub fn null() -> Self;
}
```

---

### 5.3 `Transform`

Local-space transform: translation, rotation (quaternion), non-uniform scale.

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    pub translation: glam::Vec3,
    pub rotation: glam::Quat,
    pub scale: glam::Vec3,
}

impl Transform {
    pub const IDENTITY: Self;

    pub fn from_translation(t: glam::Vec3) -> Self;
    pub fn from_rotation(r: glam::Quat) -> Self;
    pub fn from_scale(s: glam::Vec3) -> Self;
    pub fn from_translation_rotation(t: glam::Vec3, r: glam::Quat) -> Self;
    pub fn from_matrix(mat: glam::Mat4) -> Self;

    /// Converts to a column-major 4×4 matrix.
    pub fn to_matrix(&self) -> glam::Mat4;

    /// Returns a new transform that is the composition of `self` (parent)
    /// with `child` (local).
    pub fn mul_transform(&self, child: &Transform) -> Transform;

    /// Returns the forward direction (-Z in local space transformed to world).
    pub fn forward(&self) -> glam::Vec3;

    /// Returns the up direction (+Y in local space).
    pub fn up(&self) -> glam::Vec3;

    /// Returns the right direction (+X in local space).
    pub fn right(&self) -> glam::Vec3;
}

impl Default for Transform {
    fn default() -> Self; // returns IDENTITY
}
```

---

### 5.4 `SpatialIndex`

BVH-backed broad-phase spatial query structure. Updated from the user thread.

```rust
pub struct SpatialIndex { /* opaque */ }

impl SpatialIndex {
    pub fn new() -> Self;

    /// Inserts or updates an entity's axis-aligned bounding box.
    pub fn insert(&mut self, entity: Entity, aabb: Aabb);

    /// Removes an entity from the index.
    pub fn remove(&mut self, entity: Entity);

    /// Returns all entities whose AABBs intersect the given frustum.
    pub fn query_frustum(&self, frustum: &Frustum) -> Vec<Entity>;

    /// Returns all entities within the given sphere.
    pub fn query_sphere(&self, center: glam::Vec3, radius: f32) -> Vec<Entity>;

    /// Returns all entities whose AABBs intersect the given AABB.
    pub fn query_aabb(&self, aabb: &Aabb) -> Vec<Entity>;

    /// Returns the nearest N entities to `point`, sorted by distance.
    pub fn nearest_n(&self, point: glam::Vec3, n: usize) -> Vec<Entity>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub min: glam::Vec3,
    pub max: glam::Vec3,
}

impl Aabb {
    pub fn from_center_half_extents(center: glam::Vec3, half: glam::Vec3) -> Self;
    pub fn center(&self) -> glam::Vec3;
    pub fn half_extents(&self) -> glam::Vec3;
    pub fn contains_point(&self, p: glam::Vec3) -> bool;
    pub fn intersects(&self, other: &Aabb) -> bool;
    pub fn union(&self, other: &Aabb) -> Aabb;
    pub fn expand(&self, amount: f32) -> Aabb;
}

/// View frustum defined by 6 planes (outward normals).
#[derive(Debug, Clone, Copy)]
pub struct Frustum {
    pub planes: [glam::Vec4; 6],
}

impl Frustum {
    /// Extracts frustum planes from a clip-space matrix (view × projection).
    pub fn from_view_projection(vp: glam::Mat4) -> Self;

    /// Returns `true` if the AABB intersects or is inside the frustum.
    pub fn intersects_aabb(&self, aabb: &Aabb) -> bool;

    /// Returns `true` if the sphere intersects or is inside the frustum.
    pub fn intersects_sphere(&self, center: glam::Vec3, radius: f32) -> bool;
}
```

---

### 5.5 `DistanceSorter<T>`

Generic CPU-side sorter for back-to-front or front-to-back ordering. Used for
transparent objects, particles, and isometric 2.5D tiles.

```rust
pub struct DistanceSorter<T> {
    items: Vec<(f32, T)>,
}

impl<T: Clone> DistanceSorter<T> {
    pub fn new() -> Self;

    /// Clears all entries without freeing memory.
    pub fn clear(&mut self);

    /// Pushes an item with its camera-space distance (positive = in front).
    pub fn push(&mut self, distance: f32, item: T);

    /// Sorts entries back-to-front (largest distance first).
    pub fn sort_back_to_front(&mut self);

    /// Sorts entries front-to-back (smallest distance first).
    pub fn sort_front_to_back(&mut self);

    /// Returns a slice of `(distance, item)` pairs after sorting.
    pub fn sorted_items(&self) -> &[(f32, T)];

    /// Returns an iterator over only the items (without distances) after sorting.
    pub fn items(&self) -> impl Iterator<Item = &T>;
}
```

---

## 6. Animation (`harmonius-anim`)

### 6.1 `Skeleton`

CPU-side skeleton hierarchy. Parent indices define the joint tree.

```rust
#[derive(Debug, Clone)]
pub struct Skeleton {
    /// Human-readable joint names (parallel to joint arrays).
    pub joint_names: Vec<String>,
    /// Parent index for each joint; `u16::MAX` denotes a root joint.
    pub parent_indices: Vec<u16>,
    /// Bind-pose inverse world-space transform for each joint.
    pub inverse_bind_matrices: Vec<glam::Mat4>,
}

impl Skeleton {
    pub fn new(
        joint_names: Vec<String>,
        parent_indices: Vec<u16>,
        inverse_bind_matrices: Vec<glam::Mat4>,
    ) -> Result<Self>;

    /// Returns the number of joints in the skeleton.
    pub fn joint_count(&self) -> usize;

    /// Returns the index of a joint by name, if present.
    pub fn joint_index(&self, name: &str) -> Option<usize>;

    /// Returns `true` if the given index is a root joint.
    pub fn is_root(&self, index: usize) -> bool;

    /// Returns an iterator over the child indices of the given joint.
    pub fn children_of(&self, index: usize) -> impl Iterator<Item = usize> + '_;
}
```

---

### 6.2 `AnimationClip`

A baked animation clip. Contains sampled joint tracks and metadata.

```rust
#[derive(Debug, Clone)]
pub struct AnimationClip {
    pub name: String,
    /// Total duration in seconds.
    pub duration_secs: f32,
    pub looping: bool,
    /// Per-joint tracks. Track index maps to joint index in the paired Skeleton.
    pub joint_tracks: Vec<JointTrack>,
}

#[derive(Debug, Clone)]
pub struct JointTrack {
    pub joint_index: u16,
    pub translations: AnimCurve<glam::Vec3>,
    pub rotations: AnimCurve<glam::Quat>,
    pub scales: AnimCurve<glam::Vec3>,
}

impl AnimationClip {
    /// Samples local-space joint transforms at `time_secs`.
    /// Returns one transform per joint; joints without tracks return the
    /// bind-pose identity transform.
    pub fn sample(&self, time_secs: f32) -> Vec<Transform>;
}
```

---

### 6.3 `AnimCurve<T>`

Hermite-interpolated animation curve parameterized over a value type `T`.

```rust
#[derive(Debug, Clone)]
pub struct AnimCurve<T> {
    pub keyframes: Vec<CurveKeyframe<T>>,
    pub interpolation: CurveInterpolation,
}

#[derive(Debug, Clone)]
pub struct CurveKeyframe<T> {
    pub time: f32,
    pub value: T,
    pub tangent_in: T,
    pub tangent_out: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CurveInterpolation {
    Step,
    Linear,
    CubicSpline,
}

impl<T: Lerp + Clone> AnimCurve<T> {
    /// Evaluates the curve at `time`, clamping to the first/last keyframe.
    pub fn evaluate(&self, time: f32) -> T;

    /// Evaluates the curve with wrapping for looped animations.
    pub fn evaluate_looped(&self, time: f32, duration: f32) -> T;
}

/// Trait bound for types that can be linearly interpolated.
pub trait Lerp: Sized {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self;
}
```

---

### 6.4 `AnimStateMachine`

Declarative, CPU-evaluated state machine. Transitions are triggered by
boolean/float conditions on named parameters.

```rust
#[derive(Debug, Clone)]
pub struct AnimStateMachine {
    pub states: Vec<AnimState>,
    pub transitions: Vec<AnimTransition>,
    pub initial_state: String,
    pub parameters: std::collections::HashMap<String, AnimParameter>,
}

impl AnimStateMachine {
    pub fn new(initial_state: impl Into<String>) -> Self;

    /// Adds a state to the machine.
    pub fn add_state(&mut self, state: AnimState);

    /// Adds a transition between two states.
    pub fn add_transition(&mut self, transition: AnimTransition);

    /// Registers a named boolean parameter.
    pub fn add_bool_param(&mut self, name: impl Into<String>, default: bool);

    /// Registers a named float parameter.
    pub fn add_float_param(&mut self, name: impl Into<String>, default: f32);

    /// Registers a named trigger parameter (automatically resets after use).
    pub fn add_trigger_param(&mut self, name: impl Into<String>);

    /// Sets a boolean parameter by name.
    pub fn set_bool(&mut self, name: &str, value: bool) -> Result<()>;

    /// Sets a float parameter by name.
    pub fn set_float(&mut self, name: &str, value: f32) -> Result<()>;

    /// Fires a trigger parameter by name.
    pub fn fire_trigger(&mut self, name: &str) -> Result<()>;

    /// Advances the state machine by `delta_secs`, evaluating transitions.
    /// Returns the current `BlendDescriptor` to be uploaded to the GPU.
    pub fn update(&mut self, delta_secs: f32) -> BlendDescriptor;

    /// Returns the name of the currently active state.
    pub fn current_state(&self) -> &str;
}

/// Current parameter value.
#[derive(Debug, Clone)]
pub enum AnimParameter {
    Bool(bool),
    Float(f32),
    Trigger(bool),
}
```

---

### 6.5 `AnimState`

A single node in the state machine, referencing one or more clips.

```rust
#[derive(Debug, Clone)]
pub struct AnimState {
    pub name: String,
    pub clip_name: String,
    /// Playback speed multiplier.
    pub speed: f32,
    /// If set, the state machine transitions to this state when the clip ends.
    pub on_exit_transition: Option<String>,
}
```

---

### 6.6 `AnimTransition`

Describes when and how the state machine moves from one state to another.

```rust
#[derive(Debug, Clone)]
pub struct AnimTransition {
    pub from_state: String,
    pub to_state: String,
    pub condition: TransitionCondition,
    /// Crossfade blend duration in seconds.
    pub blend_duration_secs: f32,
    /// If `true`, the transition may interrupt itself.
    pub can_interrupt: bool,
}

#[derive(Debug, Clone)]
pub enum TransitionCondition {
    BoolTrue(String),
    BoolFalse(String),
    FloatGreater { param: String, threshold: f32 },
    FloatLess    { param: String, threshold: f32 },
    TriggerFired(String),
    /// Transition fires when the clip reaches or exceeds this normalised time [0,1].
    AtNormalizedTime(f32),
    /// Transition fires immediately (used for force-overrides).
    Immediate,
}
```

---

### 6.7 `BlendDescriptor`

CPU-computed blend weights uploaded to the GPU each frame.

```rust
#[derive(Debug, Clone, Copy)]
pub struct BlendEntry {
    pub clip_index: u32,
    pub weight: f32,
    pub normalized_time: f32,
}

/// Up to 4 simultaneous blended clips per instance (assumption ANIM-2).
#[derive(Debug, Clone, Copy, Default)]
pub struct BlendDescriptor {
    pub entries: [BlendEntry; 4],
    pub entry_count: u32,
}
```

---

### 6.8 `MorphTarget`

CPU-side morph target weight container. Weights are uploaded to the GPU.

```rust
#[derive(Debug, Clone)]
pub struct MorphTarget {
    pub name: String,
    /// Current blend weight in [0.0, 1.0].
    pub weight: f32,
    /// Index into the GPU morph delta buffer for this target.
    pub buffer_index: u32,
}
```

---

### 6.9 `IKChain`

Declarative IK chain definition. The GPU compute pass solves it each frame.

```rust
#[derive(Debug, Clone)]
pub struct IKChain {
    /// Joint indices in the skeleton, root-to-effector order.
    /// Maximum 8 joints (assumption ANIM-7).
    pub joint_indices: Vec<u16>,
    /// World-space goal position for the end effector.
    pub effector_goal: glam::Vec3,
    /// Optional pole vector controlling elbow / knee direction.
    pub pole_vector: Option<glam::Vec3>,
    /// Number of FABRIK or CCD iterations.
    pub solver_iterations: u32,
    /// Positional tolerance in meters to consider the goal reached.
    pub tolerance: f32,
}
```

---

## 7. UI (`harmonius-ui`)

### 7.1 `UiCanvas`

Root retained UI scene. Owns the element tree and layout state.

```rust
pub struct UiCanvas { /* opaque */ }

impl UiCanvas {
    pub fn new(width: f32, height: f32) -> Self;

    /// Sets the canvas logical pixel dimensions (before DPI scaling).
    pub fn set_size(&mut self, width: f32, height: f32);

    /// Sets the DPI scale factor.
    pub fn set_scale_factor(&mut self, scale: f32);

    /// Adds a root element to the canvas.
    pub fn add_element(&mut self, element: UiElement) -> UiElementId;

    /// Removes an element and all its children.
    pub fn remove_element(&mut self, id: UiElementId);

    /// Returns a mutable reference to the element, or `None` if not found.
    pub fn element_mut(&mut self, id: UiElementId) -> Option<&mut UiElement>;

    /// Returns an immutable reference to the element.
    pub fn element(&self, id: UiElementId) -> Option<&UiElement>;

    /// Runs the layout pass, computing bounding boxes for all elements.
    pub fn layout(&mut self);

    /// Returns the element at the given logical pixel position, if any.
    pub fn hit_test(&self, x: f32, y: f32) -> Option<UiElementId>;

    /// Returns a reference to the associated `TextLayout` engine.
    pub fn text_layout(&self) -> &TextLayout;
}

/// A stable identifier for a UI element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UiElementId(u32);
```

---

### 7.2 `UiElement`

A single node in the UI element tree.

```rust
#[derive(Debug, Clone)]
pub struct UiElement {
    pub kind: UiElementKind,
    pub layout: UiLayout,
    pub children: Vec<UiElement>,
    pub visible: bool,
    pub clip_children: bool,
}

#[derive(Debug, Clone)]
pub enum UiElementKind {
    /// Invisible container.
    Container,
    /// Solid or gradient filled rectangle.
    Rect { fill: UiFill, border_radius: f32 },
    /// Bitmap sprite from an atlas.
    Sprite(SpriteInstance),
    /// Laid-out text block.
    Text {
        text: String,
        font_size: f32,
        color: glam::Vec4,
        font_id: FontId,
        wrap: TextWrap,
    },
    /// Vector path (compute-rendered via vello-style pipeline).
    Path {
        commands: Vec<PathCommand>,
        fill: Option<UiFill>,
        stroke: Option<UiStroke>,
    },
}

#[derive(Debug, Clone)]
pub enum UiFill {
    Solid(glam::Vec4),
    LinearGradient { from: glam::Vec2, to: glam::Vec2, stops: Vec<GradientStop> },
    RadialGradient { center: glam::Vec2, radius: f32, stops: Vec<GradientStop> },
}

#[derive(Debug, Clone)]
pub struct UiStroke {
    pub color: glam::Vec4,
    pub width: f32,
    pub line_cap: LineCap,
    pub line_join: LineJoin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineCap { Butt, Round, Square }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineJoin { Miter, Round, Bevel }

#[derive(Debug, Clone)]
pub struct GradientStop {
    pub offset: f32,
    pub color: glam::Vec4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextWrap {
    None,
    Word,
    Character,
}

#[derive(Debug, Clone)]
pub enum PathCommand {
    MoveTo(glam::Vec2),
    LineTo(glam::Vec2),
    CubicBezierTo { ctrl1: glam::Vec2, ctrl2: glam::Vec2, to: glam::Vec2 },
    QuadBezierTo  { ctrl: glam::Vec2, to: glam::Vec2 },
    Close,
}

/// Layout parameters for a UI element (Flexbox-like).
#[derive(Debug, Clone)]
pub struct UiLayout {
    pub position: LayoutPosition,
    pub left: LayoutValue,
    pub top: LayoutValue,
    pub width: LayoutValue,
    pub height: LayoutValue,
    pub margin: EdgeInsets,
    pub padding: EdgeInsets,
    pub flex_direction: FlexDirection,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub align_items: Alignment,
    pub justify_content: Alignment,
    pub z_index: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutValue {
    Auto,
    Pixels(f32),
    Percent(f32),
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct EdgeInsets {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl EdgeInsets {
    pub fn all(v: f32) -> Self;
    pub fn horizontal(h: f32) -> Self;
    pub fn vertical(v: f32) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutPosition { Static, Relative, Absolute }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexDirection { Row, Column, RowReverse, ColumnReverse }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Start, Center, End, Stretch, SpaceBetween, SpaceAround,
}
```

---

### 7.3 `TextLayout`

Text shaping and glyph atlas management.

```rust
pub struct TextLayout { /* opaque */ }

impl TextLayout {
    /// Registers a font from raw bytes and returns its `FontId`.
    pub fn load_font(&mut self, font_data: Vec<u8>) -> Result<FontId>;

    /// Returns the bounding box of `text` when rendered at `font_size` logical
    /// pixels in the given font with the given max-width wrap constraint.
    pub fn measure(
        &self,
        text: &str,
        font_id: FontId,
        font_size: f32,
        max_width: Option<f32>,
    ) -> glam::Vec2;
}

/// Opaque font identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontId(u32);
```

---

### 7.4 `SpriteAtlas`

Packed bitmap atlas for UI and 2D sprites.

```rust
#[derive(Debug, Clone)]
pub struct SpriteAtlas {
    pub texture: Handle<Texture>,
    /// Each entry maps a sprite name to its UV rect in [0,1]² space.
    pub sprites: std::collections::HashMap<String, UvRect>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UvRect {
    pub u_min: f32,
    pub v_min: f32,
    pub u_max: f32,
    pub v_max: f32,
}

impl SpriteAtlas {
    /// Returns the UV rect for a named sprite, or `None` if not found.
    pub fn sprite(&self, name: &str) -> Option<UvRect>;
}
```

---

### 7.5 `SpriteInstance`

Per-instance data for a 2D sprite draw.

```rust
#[derive(Debug, Clone, Copy)]
pub struct SpriteInstance {
    pub position: glam::Vec2,
    pub size: glam::Vec2,
    pub rotation_rad: f32,
    pub uv_rect: UvRect,
    pub atlas_index: u32,
    /// Tint/modulate color (RGBA, linear).
    pub tint: glam::Vec4,
    /// Z-order for sorting; higher values draw on top.
    pub z_order: i32,
    /// Pixel-aligned clip rectangle; `None` means no clipping.
    pub clip_rect: Option<[f32; 4]>,
}
```

---

### 7.6 `TilemapDesc`

Descriptor for a 2D tilemap layer.

```rust
#[derive(Debug, Clone)]
pub struct TilemapDesc {
    /// Width of each tile in logical pixels.
    pub tile_width: f32,
    /// Height of each tile in logical pixels.
    pub tile_height: f32,
    /// Number of tile columns.
    pub columns: u32,
    /// Number of tile rows.
    pub rows: u32,
    /// Number of layers (for parallax / depth stacking).
    pub layer_count: u32,
    /// Atlas holding all tile sprites.
    pub atlas: Handle<Texture>,
    /// Tile index data (row-major, one u16 per cell per layer; u16::MAX = empty).
    pub tile_data: Vec<Vec<u16>>,
}
```

---

## 8. Cameras

Camera data is passed as a CPU-side descriptor. The graph executor uploads it
to per-frame uniform buffers.

### 8.1 `PerspectiveCamera`

```rust
#[derive(Debug, Clone, Copy)]
pub struct PerspectiveCamera {
    /// Vertical field of view in radians.
    pub fov_y_rad: f32,
    pub aspect_ratio: f32,
    /// Near plane distance (reverse-Z: near maps to 1.0 in NDC).
    pub near: f32,
    /// Far plane distance (reverse-Z: far maps to 0.0 in NDC).
    /// `None` for an infinite far plane.
    pub far: Option<f32>,
}

impl PerspectiveCamera {
    /// Returns the projection matrix (reverse-Z, infinite far if `far` is None).
    pub fn projection_matrix(self) -> glam::Mat4;
}
```

---

### 8.2 `OrthographicCamera`

```rust
#[derive(Debug, Clone, Copy)]
pub struct OrthographicCamera {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
}

impl OrthographicCamera {
    /// Returns the orthographic projection matrix.
    pub fn projection_matrix(self) -> glam::Mat4;
}
```

---

### 8.3 `CameraData`

Complete per-frame camera data bundle uploaded to the GPU.

```rust
#[derive(Debug, Clone, Copy)]
pub struct CameraData {
    pub view: glam::Mat4,
    pub projection: glam::Mat4,
    pub view_projection: glam::Mat4,
    pub inverse_view: glam::Mat4,
    pub inverse_projection: glam::Mat4,
    /// View-projection from the previous frame (for motion vectors).
    pub prev_view_projection: glam::Mat4,
    pub world_position: glam::Vec3,
    pub near_plane: f32,
    /// `f32::INFINITY` for infinite projection.
    pub far_plane: f32,
    pub viewport_size: glam::Vec2,
    pub jitter_offset: glam::Vec2,
    pub projection_kind: ProjectionKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionKind {
    Perspective,
    Orthographic,
}
```

---

## 9. Lighting

### 9.1 `LightDesc`

Unified light descriptor covering all three light types.

```rust
#[derive(Debug, Clone)]
pub enum LightDesc {
    Directional(DirectionalLightDesc),
    Point(PointLightDesc),
    Spot(SpotLightDesc),
}

#[derive(Debug, Clone, Copy)]
pub struct DirectionalLightDesc {
    /// World-space direction pointing *toward* the light.
    pub direction: glam::Vec3,
    /// Linear HDR color (pre-multiplied by intensity).
    pub color: glam::Vec3,
    pub illuminance_lux: f32,
    pub cast_shadows: bool,
    pub shadow_config: Option<CsmConfig>,
}

#[derive(Debug, Clone, Copy)]
pub struct PointLightDesc {
    pub position: glam::Vec3,
    pub color: glam::Vec3,
    pub intensity_lumens: f32,
    /// Physical attenuation cutoff radius.
    pub range: f32,
    pub cast_shadows: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct SpotLightDesc {
    pub position: glam::Vec3,
    pub direction: glam::Vec3,
    pub color: glam::Vec3,
    pub intensity_lumens: f32,
    pub range: f32,
    /// Inner (full intensity) cone half-angle in radians.
    pub inner_angle_rad: f32,
    /// Outer (zero intensity) cone half-angle in radians.
    pub outer_angle_rad: f32,
    pub cast_shadows: bool,
}
```

---

### 9.2 `ShadowConfig`

Generic per-light shadow settings.

```rust
#[derive(Debug, Clone, Copy)]
pub struct ShadowConfig {
    pub shadow_map_resolution: u32,
    pub depth_bias: f32,
    pub normal_bias: f32,
    pub soft_shadow: SoftShadowConfig,
}
```

---

### 9.3 `CsmConfig`

Cascaded Shadow Map configuration for directional lights.

```rust
#[derive(Debug, Clone, Copy)]
pub struct CsmConfig {
    /// Number of shadow cascades (1–4).
    pub cascade_count: u32,
    /// Log/linear split lambda in [0.0, 1.0].
    pub lambda: f32,
    pub shadow_map_resolution: u32,
    pub depth_bias: f32,
    pub normal_bias: f32,
    /// Reduce temporal shimmering by rounding cascade extents to texel size.
    pub stabilize: bool,
    pub soft_shadow: SoftShadowConfig,
}
```

---

### 9.4 `SoftShadowConfig`

```rust
#[derive(Debug, Clone, Copy)]
pub enum SoftShadowConfig {
    /// Percentage Closer Filtering.
    Pcf { kernel_radius: u32 },
    /// Percentage Closer Soft Shadows.
    Pcss { light_size_world: f32 },
    /// Ray traced soft shadows.
    RayTraced {
        sample_count: u32,
        /// Requires `DeviceCapability::RayTracing`.
        budget_gate: BudgetGate,
    },
}
```

---

### 9.5 `AoConfig`

Ambient Occlusion configuration.

```rust
#[derive(Debug, Clone, Copy)]
pub enum AoConfig {
    /// Screen-Space Ambient Occlusion.
    Ssao {
        radius_world: f32,
        sample_count: u32,
        half_resolution: bool,
    },
    /// Ground Truth Ambient Occlusion (with bent normals).
    Gtao {
        radius_world: f32,
        slice_count: u32,
        steps_per_slice: u32,
        half_resolution: bool,
    },
    /// Ray traced ambient occlusion.
    RayTraced {
        radius_world: f32,
        sample_count: u32,
        half_resolution: bool,
        budget_gate: BudgetGate,
    },
}
```

---

## 10. Materials

### 10.1 `PbrMaterial`

Standard PBR material using Cook-Torrance BRDF. All texture slots are bindless
indices resolved by `BindlessHeap`.

```rust
#[derive(Debug, Clone)]
pub struct PbrMaterial {
    /// RGBA8 albedo + alpha.
    pub albedo_texture: Option<Handle<Texture>>,
    pub albedo_factor: glam::Vec4,
    /// R = metallic, G = roughness.
    pub metallic_roughness_texture: Option<Handle<Texture>>,
    pub metallic_factor: f32,
    pub roughness_factor: f32,
    /// Occlusion in R channel.
    pub occlusion_texture: Option<Handle<Texture>>,
    pub occlusion_strength: f32,
    pub normal_texture: Option<Handle<Texture>>,
    pub normal_scale: f32,
    pub emissive_texture: Option<Handle<Texture>>,
    pub emissive_factor: glam::Vec3,
    pub alpha_mode: AlphaMode,
    pub double_sided: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlphaMode {
    Opaque,
    Mask { cutoff: f32 },
    Blend,
}

/// Opaque handle to a registered material.
pub type MaterialHandle = Handle<PbrMaterial>;
```

---

### 10.2 `BsdfMaterial`

Extended material adding subsurface scattering, clearcoat, anisotropy, and sheen
on top of the standard PBR base.

```rust
#[derive(Debug, Clone)]
pub struct BsdfMaterial {
    pub base: PbrMaterial,

    // Subsurface scattering
    pub subsurface: Option<SssConfig>,

    // Clearcoat
    pub clearcoat_factor: f32,
    pub clearcoat_roughness_factor: f32,
    pub clearcoat_normal_texture: Option<Handle<Texture>>,

    // Anisotropy
    pub anisotropy_strength: f32,
    /// Angle in radians from the tangent direction.
    pub anisotropy_rotation: f32,
    pub anisotropy_texture: Option<Handle<Texture>>,

    // Sheen (cloth / velvet)
    pub sheen_color_factor: glam::Vec3,
    pub sheen_roughness_factor: f32,
    pub sheen_color_texture: Option<Handle<Texture>>,
}

#[derive(Debug, Clone, Copy)]
pub struct SssConfig {
    pub profile: SssProfile,
    /// Uniform scale on scatter radius.
    pub thickness_scale: f32,
}
```

---

### 10.3 `SssProfile`

Subsurface scattering profile. Configured once, referenced by index from shaders.

```rust
#[derive(Debug, Clone, Copy)]
pub struct SssProfile {
    /// Wavelength-dependent scatter radius in world meters: [R, G, B].
    pub scatter_radius: [f32; 3],
    /// Extinction coefficients: [R, G, B].
    pub extinction: [f32; 3],
    /// Subsurface albedo: [R, G, B].
    pub albedo: [f32; 3],
    /// Index of refraction.
    pub ior: f32,
}
```

---

## 11. Geometry

### 11.1 `MeshDesc`

High-level mesh input descriptor. The backend meshletizes the data on upload.

```rust
#[derive(Debug, Clone)]
pub struct MeshDesc {
    pub label: Option<String>,
    /// Interleaved vertex attribute data.
    pub vertex_data: Vec<u8>,
    pub vertex_layout: VertexLayout,
    /// 32-bit triangle indices.
    pub indices: Vec<u32>,
    pub meshlet_config: MeshletConfig,
    /// Axis-aligned bounding box of the mesh in local space.
    pub aabb: Aabb,
}

#[derive(Debug, Clone)]
pub struct VertexLayout {
    /// Byte stride between vertices.
    pub stride: u32,
    pub attributes: Vec<VertexAttribute>,
}

#[derive(Debug, Clone, Copy)]
pub struct VertexAttribute {
    pub kind: VertexAttributeKind,
    pub offset: u32,
    pub format: VertexFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VertexAttributeKind {
    Position,
    Normal,
    Tangent,
    Uv0,
    Uv1,
    Color,
    BoneIndices,
    BoneWeights,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VertexFormat {
    Float32x2,
    Float32x3,
    Float32x4,
    Float16x4,
    Unorm8x4,
    Uint8x4,
    Sint8x4,
}
```

---

### 11.2 `MeshletConfig`

Tuning parameters passed to the meshletizer.

```rust
#[derive(Debug, Clone, Copy)]
pub struct MeshletConfig {
    /// Maximum number of vertices per meshlet (hardware limit: 64–256).
    pub max_vertices: u32,
    /// Maximum number of triangles per meshlet (hardware limit: 64–512).
    pub max_primitives: u32,
    /// Cone culling weight in [0.0, 1.0]; higher values bias toward smaller
    /// normal cones.
    pub cone_weight: f32,
}

impl Default for MeshletConfig {
    fn default() -> Self; // max_vertices=128, max_primitives=128, cone_weight=0.5
}
```

---

### 11.3 `ProceduralGenJob`

Parameters for a GPU-side procedural geometry generation dispatch.

```rust
#[derive(Debug, Clone, Copy)]
pub struct ProceduralGenJob {
    pub seed: u64,
    /// World-space AABB the job should populate.
    pub world_aabb: Aabb,
    /// Ring buffer slot the output meshlets are written to.
    pub output_ring_slot: u32,
    pub lod_level: u32,
    /// Async compute dispatch dimensions.
    pub dispatch: [u32; 3],
}
```

---

### 11.4 `SplineDesc`

A catmull-rom / cubic bezier spline path.

```rust
#[derive(Debug, Clone)]
pub struct SplineDesc {
    pub control_points: Vec<SplineControlPoint>,
    pub closed: bool,
    /// Number of subdivisions per segment for GPU evaluation.
    pub subdivisions_per_segment: u32,
    /// Extrude the path into a tube of this radius (0.0 = line only).
    pub extrusion_radius: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct SplineControlPoint {
    pub position: glam::Vec3,
    pub tangent_weight: f32,
    pub up_vector: glam::Vec3,
}
```

---

### 11.5 `ChunkStreamingConfig`

Parameters for infinite-universe chunk-based streaming.

```rust
#[derive(Debug, Clone, Copy)]
pub struct ChunkStreamingConfig {
    /// World-space size of a single chunk (cubic).
    pub chunk_size_world: f32,
    /// Number of LOD levels (1 = no LOD).
    pub lod_levels: u32,
    /// GPU pool capacity in number of resident chunks.
    pub max_resident_chunks: u32,
    /// Distance from camera to begin streaming in chunks.
    pub stream_in_distance: f32,
    /// Distance from camera to begin streaming out chunks.
    pub stream_out_distance: f32,
}
```

---

### 11.6 `VoxelWorldConfig`

Parameters for planetary-scale sparse voxel world rendering.

```rust
#[derive(Debug, Clone, Copy)]
pub struct VoxelWorldConfig {
    /// World-space size of one voxel at the finest LOD.
    pub voxel_size: f32,
    /// Depth of the sparse voxel octree.
    pub octree_depth: u32,
    /// Voxel page size (cube root; typical: 16).
    pub page_size: u32,
    /// Maximum number of resident voxel pages.
    pub max_resident_pages: u32,
}
```

---

### 11.7 `TerrainConfig`

Heightmap terrain descriptor for detailed open-world rendering.

```rust
#[derive(Debug, Clone)]
pub struct TerrainConfig {
    /// World-space width and depth of the terrain.
    pub extent_world: glam::Vec2,
    /// Minimum and maximum height values.
    pub height_range: std::ops::RangeInclusive<f32>,
    /// Resolution of the heightmap texture (power of two recommended).
    pub heightmap_resolution: u32,
    pub tile_config: ChunkStreamingConfig,
    /// Number of virtual texture layers (albedo, normal, etc.).
    pub virtual_texture_layers: u32,
    /// Physical page size for virtual texture pages.
    pub virtual_texture_page_size: u32,
}
```

---

### 11.8 `OceanDesc`

FFT ocean simulation descriptor.

```rust
#[derive(Debug, Clone, Copy)]
pub struct OceanDesc {
    /// FFT resolution (power of two; typical: 256, 512).
    pub fft_resolution: u32,
    /// Simulation patch size in world meters.
    pub patch_size: f32,
    pub wind_direction: glam::Vec2,
    pub wind_speed: f32,
    /// Horizontal displacement scale (choppiness).
    pub choppiness: f32,
    /// Foam generation threshold.
    pub foam_threshold: f32,
    /// Number of rendered LOD tiles.
    pub lod_levels: u32,
    pub enable_underwater: bool,
}
```

---

## 12. Volumetrics & Environment

### 12.1 `FroxelVolumeDesc`

Parameters for the froxel (frustum-aligned voxel) volumetric pass.

```rust
#[derive(Debug, Clone, Copy)]
pub struct FroxelVolumeDesc {
    /// Screen-space tile size in pixels.
    pub tile_size_px: u32,
    /// Number of depth slices.
    pub depth_slices: u32,
    pub near: f32,
    pub far: f32,
    /// Depth distribution warping: 0.0 = linear, 1.0 = log.
    pub distribution_lambda: f32,
}
```

---

### 12.2 `AtmosphereDesc`

Bruneton/Hillaire atmosphere model parameters.

```rust
#[derive(Debug, Clone, Copy)]
pub struct AtmosphereDesc {
    pub planet_radius_km: f32,
    pub atmosphere_height_km: f32,
    /// Rayleigh scattering coefficient at sea level (RGB, km⁻¹).
    pub rayleigh_scattering: glam::Vec3,
    /// Rayleigh scale height in km.
    pub rayleigh_scale_height_km: f32,
    /// Mie scattering coefficient (km⁻¹).
    pub mie_scattering: f32,
    /// Mie absorption coefficient (km⁻¹).
    pub mie_absorption: f32,
    /// Mie scale height in km.
    pub mie_scale_height_km: f32,
    /// Mie asymmetry (Henyey-Greenstein g parameter).
    pub mie_asymmetry: f32,
    pub sun_angular_radius_rad: f32,
    pub sun_irradiance: glam::Vec3,
    /// Ozone absorption coefficients (RGB, km⁻¹).
    pub ozone_absorption: glam::Vec3,
}
```

---

### 12.3 `CloudLayerDesc`

Parameters for a volumetric cloud layer.

```rust
#[derive(Debug, Clone, Copy)]
pub struct CloudLayerDesc {
    pub altitude_min_km: f32,
    pub altitude_max_km: f32,
    pub coverage: f32,
    pub wind_direction: glam::Vec2,
    pub wind_speed: f32,
    /// Base cloud density scale.
    pub density: f32,
    /// Number of ray march steps.
    pub march_steps: u32,
    /// Temporal reprojection blend weight.
    pub temporal_blend: f32,
}
```

---

### 12.4 `FogDesc`

Analytical or volumetric fog descriptor.

```rust
#[derive(Debug, Clone, Copy)]
pub enum FogDesc {
    /// Disabled.
    None,
    /// Exponential fog.
    Exponential {
        density: f32,
        height_falloff: f32,
        color: glam::Vec3,
    },
    /// Exponential squared fog.
    ExponentialSquared {
        density: f32,
        height_falloff: f32,
        color: glam::Vec3,
    },
    /// Full volumetric froxel integration (requires `FroxelVolumeDesc`).
    Volumetric {
        scattering: glam::Vec3,
        absorption: glam::Vec3,
        phase_asymmetry: f32,
    },
}
```

---

### 12.5 `GodRayDesc`

God-ray / light shaft configuration.

```rust
#[derive(Debug, Clone, Copy)]
pub enum GodRayDesc {
    /// Screen-space radial blur approximation.
    ScreenSpace {
        sample_count: u32,
        decay: f32,
        weight: f32,
        exposure: f32,
    },
    /// Full volumetric integration through the froxel volume.
    Volumetric,
}
```

---

## 13. Ray Tracing

### 13.1 `AccelerationStructureConfig`

Parameters for BLAS/TLAS construction.

```rust
#[derive(Debug, Clone, Copy)]
pub struct AccelerationStructureConfig {
    /// Allow compaction after initial build to reduce memory usage.
    pub allow_compaction: bool,
    /// Allow fast refit for deforming meshes (vs. full rebuild).
    pub allow_fast_refit: bool,
    /// Prefer fast build over fast trace quality.
    pub prefer_fast_build: bool,
    /// Maximum number of TLAS instances.
    pub max_tlas_instances: u32,
}

impl Default for AccelerationStructureConfig {
    fn default() -> Self;
    // allow_compaction: true, allow_fast_refit: false,
    // prefer_fast_build: false, max_tlas_instances: 65536
}
```

---

### 13.2 `RTReflectionConfig`

Configuration for the hybrid SSR + ray traced reflection pass.

```rust
#[derive(Debug, Clone, Copy)]
pub struct RTReflectionConfig {
    /// Enable screen-space reflection as the primary ray source.
    pub enable_ssr: bool,
    /// Maximum SSR ray-march steps.
    pub ssr_max_steps: u32,
    /// Surface roughness threshold above which RT rays are cast.
    pub rt_roughness_threshold: f32,
    /// Ray traced reflection sample count per pixel.
    pub rt_sample_count: u32,
    pub half_resolution: bool,
    pub denoiser: ReflectionDenoiser,
    pub budget_gate: BudgetGate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReflectionDenoiser {
    None,
    Spatial,
    Temporal,
    SpatialTemporal,
}
```

---

### 13.3 `RTIndirectConfig`

Ray traced indirect lighting (diffuse GI complement to DDGI).

```rust
#[derive(Debug, Clone, Copy)]
pub struct RTIndirectConfig {
    pub samples_per_pixel: u32,
    pub max_bounces: u32,
    pub half_resolution: bool,
    pub denoiser_iterations: u32,
    pub budget_gate: BudgetGate,
}
```

---

### 13.4 `DDGIVolumeConfig`

Dynamic Diffuse Global Illumination probe volume configuration.

```rust
#[derive(Debug, Clone, Copy)]
pub struct DDGIVolumeConfig {
    /// World-space origin of the probe grid.
    pub origin: glam::Vec3,
    /// World-space spacing between adjacent probes.
    pub probe_spacing: glam::Vec3,
    /// Number of probes along each axis.
    pub probe_counts: [u32; 3],
    /// Number of rays traced per probe per frame.
    pub rays_per_probe: u32,
    /// Temporal hysteresis blend weight for irradiance update.
    pub irradiance_hysteresis: f32,
    /// Temporal hysteresis blend weight for visibility update.
    pub visibility_hysteresis: f32,
    /// Probe irradiance texture resolution (power of two).
    pub irradiance_probe_resolution: u32,
    /// Probe visibility (depth) texture resolution.
    pub visibility_probe_resolution: u32,
    pub budget_gate: BudgetGate,
}

impl Default for DDGIVolumeConfig {
    fn default() -> Self;
    // rays_per_probe: 128, irradiance_hysteresis: 0.97,
    // visibility_hysteresis: 0.99, irradiance_probe_resolution: 8,
    // visibility_probe_resolution: 16
}
```

---

## Appendix: Marker Types

These zero-sized marker types are used as the generic parameter `T` in
`Handle<T>`, `ResourceRef<T>`, and similar type-parameterized containers.
They carry no data and are never instantiated directly.

```rust
/// Marker for GPU buffer resources.
pub struct Buffer;

/// Marker for GPU texture resources.
pub struct Texture;

/// Marker for GPU sampler objects.
pub struct Sampler;

/// Marker for render target views.
pub struct RenderTarget;
```

---

## Appendix: Notable Design Constraints

| Constraint | Value | Source |
|---|---|---|
| Max bones per skeleton | 256 | ANIM-1 |
| Max blend clips per instance | 4 | ANIM-2 |
| Max IK chain length | 8 joints | ANIM-7 |
| Morph target delta precision | f16 | ANIM-8 |
| Depth convention | Reverse-Z (near=1, far=0) | ARCH-4 |
| Mesh shaders | Hard requirement, no fallback | ARCH-2 |
| CPU-driven draw calls | Prohibited | ARCH-1 |
| Legacy pipelines (geo shaders, tessellation) | Excluded | ARCH-7 |
| Compatibility layers (MoltenVK, DXVK) | Excluded | ARCH-8 |
| User-thread unsafe code | Prohibited | ARCH-6 |
| Shader IR | Naga only | TOOL-1 |
| Shader graph file format | MessagePack (.hsg) | TOOL-2 |
