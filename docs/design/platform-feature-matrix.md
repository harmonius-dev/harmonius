# Platform Feature Matrix

Consolidated cross-platform reference for the Harmonius GPU graphics framework. This document
collects all platform-specific capabilities, constraints, and emulation strategies from the
backend design documents into a single matrix.

**Source documents:**

- [gpu-backend-interface.md](gpu-backend-interface.md) -- abstract interface and capability summary
- [gpu-backend-d3d12.md](gpu-backend-d3d12.md) -- Direct3D 12 backend
- [gpu-backend-vulkan.md](gpu-backend-vulkan.md) -- Vulkan backend
- [gpu-backend-metal.md](gpu-backend-metal.md) -- Metal backend
- [gpu-runtime.md](gpu-runtime.md) -- GPU runtime feature emulation
- [shader-pipeline.md](shader-pipeline.md) -- shader compilation pipeline
- [asset-pipeline.md](asset-pipeline.md) -- IO backends

---

## Contents

- [Platform Overview](#platform-overview)
- [Device Capabilities Matrix](#device-capabilities-matrix)
- [Feature Emulation Matrix](#feature-emulation-matrix)
- [Shader Compilation Matrix](#shader-compilation-matrix)
- [IO Backend Matrix](#io-backend-matrix)
- [Synchronization Primitives](#synchronization-primitives)
- [Resource Binding Model](#resource-binding-model)
- [Pipeline State Model](#pipeline-state-model)
- [Cross-Backend Compatibility Constraints](#cross-backend-compatibility-constraints)
- [Minimum Hardware Reference](#minimum-hardware-reference)

---

## Platform Overview

| Platform | GPU API | Min GPU | IO Path | Status |
|----------|---------|---------|---------|--------|
| macOS (Apple Silicon M1+) | Metal 4 | Apple7+ (M1+) | dispatch_io (GCD) | Initial dev target |
| Windows | Direct3D 12 (Agility SDK 1.619+) | SM 6.9, D3D12 Ultimate | DirectStorage | Supported |
| Windows | Vulkan 1.4 | VK_EXT_mesh_shader required | IOCP | Supported |
| Linux / SteamOS | Vulkan 1.4 | VK_EXT_mesh_shader required | io_uring | Supported |

The backend is selected at build time via the `HARMONIUS_GPU_BACKEND` CMake option. Only one
backend is compiled into a given binary. All calls resolve through C++20 concepts to direct
method calls on the concrete backend type, with no vtable overhead.

---

## Device Capabilities Matrix

Every capability is queried at device initialization. Hard-gated capabilities cause device
creation to fail with `DeviceError::kFeatureNotSupported` if absent. Soft-gated capabilities
control render graph pass culling at runtime.

| Capability | D3D12 | Vulkan | Metal | Gate Type |
|-----------|-------|--------|-------|-----------|
| `mesh_shaders` | Required (SM 6.5+) | Required (VK_EXT_mesh_shader) | Required (Apple8+) | Hard |
| `bindless_resources` | Required (Tier 3) | Required (VK_EXT_descriptor_indexing) | Required (Tier 2) | Hard |
| `timeline_fences` | Required (ID3D12Fence) | Required (core 1.2) | Required (MTLSharedEvent) | Hard |
| `async_compute_queue` | Required (dedicated Compute engine) | Required (dedicated Compute queue family) | Required (universal queues) | Hard |
| `transfer_queue` | Required (dedicated copy engine) | Required (dedicated transfer queue family) | Required (universal queues) | Hard |
| `ray_tracing` | Soft-gated (DXR 1.2) | Soft-gated (VK_KHR_ray_tracing_pipeline) | Always false (no RT pipeline) | Soft |
| `ray_tracing_inline` | Soft-gated (SM 6.5 RayQuery) | Soft-gated (VK_KHR_ray_query) | Soft-gated (Apple7+ intersector) | Soft |
| `opacity_micromaps` | Soft-gated (DXR 1.2) | Soft-gated (VK_EXT_opacity_micromap) | Always false | Soft |
| `sparse_textures` | Soft-gated (Tiled Resources Tier 1+) | Soft-gated (sparseBinding) | Soft-gated (Apple7+) | Soft |
| `int64_atomics` | Soft-gated (SM 6.6) | Soft-gated (shaderBufferInt64Atomics) | Soft-gated (Apple7+) | Soft |
| `variable_rate_shading` | Soft-gated (VRS Tier 2) | Soft-gated (VK_KHR_fragment_shading_rate) | Always false | Soft |
| `work_graphs` | Soft-gated (Work Graphs Tier 1) | Always false | Always false | Soft |
| `split_barriers` | Always true (enhanced barriers) | Always true (events) | Always false | N/A |
| `shader_function_linking` | Soft (ID3D12FunctionLinkingGraph) | Soft (VK_EXT_graphics_pipeline_library) | Soft (MTLStitchedLibrary) | Soft |

---

## Feature Emulation Matrix

The GPU runtime (`harmonius::gpu_runtime::compat`) emulates features when native support is
missing. Emulation is transparent to the render graph.

| Feature | D3D12 | Vulkan | Metal | Emulation Strategy |
|---------|-------|--------|-------|--------------------|
| Split barriers | Native (enhanced barriers) | Native (events) | Emulated: deferred Begin, immediate combined at End | GR-4.2 |
| Queue ownership transfers | Native | Native | Elided (unified memory) | GR-4.5 |
| RT pipeline dispatch | Native (DispatchRays) | Native (vkCmdTraceRaysKHR) | Emulated: compute dispatch with inline ray queries | GR-4.6 |
| Shader binding table | Native | Native | Emulated: flat material buffer indexed by instanceID * geomCount + geomIdx | GR-4.7 |
| Work graphs | Native (DispatchGraph) | Emulated: CPU-side command recording per pass | Emulated: CPU-side command recording per pass | GR-3.1 |
| Variable rate shading | Native (VRS image) | Native (fragment shading rate image) | Not available (pass culled) | Capability gate |
| Indirect mesh count buffer | Native (ExecuteIndirect) | Native (vkCmdDrawMeshTasksIndirectCountEXT) | Emulated: compute pre-pass clamps count into argument buffer | Shim |
| Texture layouts | Explicit transitions | Explicit transitions | Ignored (Metal manages internally) | Shim |

---

## Shader Compilation Matrix

All shaders are authored in HLSL. DXC compiles to platform-native bytecode. Metal uses an
additional conversion step from DXIL to Metal IR.

| Backend | Source | Compiler | Target IR | Profile |
|---------|--------|----------|-----------|---------|
| Direct3D 12 | HLSL | DXC | DXIL | Shader Model 6.9 |
| Vulkan 1.4 | HLSL | DXC -spirv | SPIR-V | -fspv-target-env=vulkan1.3 |
| Metal 4 | HLSL | DXC then metal-shaderconverter | Metal IR | via DXIL intermediate |

**Fragment linking targets:**

| Backend | Fragment Compilation Target | Library Format | Linking Mechanism |
|---------|---------------------------|----------------|-------------------|
| D3D12 | `dxc -T lib_6_9` | DXIL library | ID3D12FunctionLinkingGraph |
| Vulkan | `dxc -T lib_6_9 -spirv` | SPIR-V module with exports | VK_EXT_graphics_pipeline_library + LTO |
| Metal | metal-shaderconverter | Metal IR library | MTLStitchedLibrary |

---

## IO Backend Matrix

Each platform uses its native high-performance async IO path for asset streaming (R-1.2.4).
C++ standard library file IO is not used. The backend is selected at build time alongside the
GPU backend and target platform.

| Platform | Backend Class | IO Mechanism | GPU-Direct | Decompression |
|----------|--------------|-------------|------------|---------------|
| macOS | DispatchIOBackend | dispatch_io (GCD) | No (staging buffer) | CPU-side (zstd) |
| Windows (D3D12) | DirectStorageBackend | DirectStorage API | Yes | GPU-side (DirectStorage) |
| Windows (Vulkan) | IocpBackend | I/O completion ports | No (staging buffer) | CPU-side (zstd) |
| Linux/SteamOS | IoUringBackend | io_uring | No (staging buffer) | CPU-side (zstd) |

---

## Synchronization Primitives

| Primitive | D3D12 | Vulkan | Metal |
|-----------|-------|--------|-------|
| Timeline fence | ID3D12Fence | VkSemaphore (timeline) | MTLSharedEvent |
| Pipeline Barrier | D3D12_BARRIER (enhanced) | vkCmdPipelineBarrier2 | MTLBarrier |
| Split Barrier | D3D12_BARRIER (SPLIT sync) | VkEvent + vkCmdSetEvent2/vkCmdWaitEvents2 | Emulated (deferred + immediate) |
| Queue ownership | D3D12_BARRIER (cross-queue) | VkBufferMemoryBarrier2 (queue family) | Elided (UMA) |

---

## Resource Binding Model

| Aspect | D3D12 | Vulkan | Metal |
|--------|-------|--------|-------|
| Descriptor model | SM 6.6 bindless heap | Descriptor indexing + UPDATE_AFTER_BIND | Argument Buffers Tier 2 |
| Push constants | Root constants (max 32 bytes for cross-platform) | Push constants | Set bytes on encoder |
| Descriptor heap Size | 1M+ CBV/SRV/UAV | Limited by maxDescriptorSetSize | Argument Buffer Size |

---

## Pipeline State Model

| Aspect | D3D12 | Vulkan | Metal |
|--------|-------|--------|-------|
| PSO creation | Stream-based (D3D12_PIPELINE_STATE_STREAM_DESC) | VkGraphicsPipelineCreateInfo | MTLRenderPipelineDescriptor |
| Pipeline cache | ID3D12PipelineLibrary | VkPipelineCache + binary archives | MTLBinaryArchive |
| Dynamic rendering | Render Passes optional | VK_KHR_dynamic_rendering | Always dynamic (Metal 3+) |
| Mesh shader PSO | Mesh + amplification stages | VK_EXT_mesh_shader pipeline | Object + mesh function |

---

## Cross-Backend Compatibility Constraints

These constraints define the lowest common denominator for portable rendering across all three
backends.

- **Push constants** capped at 32 bytes (D3D12 root constant limit for practical portability).
  Larger per-frame data goes through the frame constants CBV.
- **Mesh shader output:** 128 vertices / 128 primitives per threadgroup (cross-platform safe
  maximum; individual backends support up to 256).
- **Queue topology:** 3 queues minimum (graphics, async compute, transfer). All backends
  provide this; Metal uses universal queues.
- **Texture format:** common subset across all 3 APIs. BC7/BC5/BC1 block-compressed formats
  are universally supported.
- **Descriptor heap:** 1M slot minimum (D3D12 CBV/SRV/UAV heap). Vulkan and Metal scale
  similarly.
- **Memory model:** Metal uses unified memory (UMA). D3D12 and Vulkan assume discrete GPU
  with separate device-local and host-visible memory. Queue ownership transfers are elided
  on UMA.
- **Image layouts:** D3D12 and Vulkan require explicit texture layout transitions. Metal
  manages layouts internally; layout fields in barrier descriptors are ignored.
- **Ray tracing model:** D3D12 and Vulkan provide dedicated RT pipeline stages. Metal uses
  inline ray queries in compute shaders only; full RT pipeline shaders (raygen/hit/miss)
  are not available on Metal.

---

## Minimum Hardware Reference

| Platform | Minimum GPU | GPU Family | Year |
|----------|------------|------------|------|
| macOS | Apple M1 | Apple7 | 2020 |
| Windows (D3D12) | NVIDIA RTX 2000 / AMD RX 6000 | D3D12 Ultimate | 2020 |
| Windows (Vulkan) | Same + VK_EXT_mesh_shader support | Vulkan 1.4 | 2020 |
| Linux | Same as Windows Vulkan | Vulkan 1.4 | 2020 |
