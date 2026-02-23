# Harmonius - Tracked Assumptions

## Architecture Assumptions

| ID | Assumption | Rationale | Impact if Wrong |
|---|---|---|---|
| ARCH-1 | GPU-driven rendering only; no CPU-driven draw calls | Modern GPU architecture favors indirect dispatch | Entire culling pipeline would need CPU fallback |
| ARCH-2 | Mesh shaders are a hard requirement, no vertex pipeline fallback | Simplifies pipeline, reduces code surface | Cannot run on pre-mesh-shader hardware at all |
| ARCH-3 | `VK_EXT_mesh_shader` is NOT core in Vulkan 1.4 | Must be explicitly queried and enabled | Fatal error if assumed to be core and not queried |
| ARCH-4 | Reverse-Z depth is mandatory on all platforms | Maximizes depth precision at distance | HZB min-reduction direction, depth comparison ops change |
| ARCH-5 | Single-writer invariant for shared resources during render phase | Eliminates locks during command encoding | Multi-writer scenarios need explicit sync |
| ARCH-6 | User thread is 100% safe Rust; unsafe only on background threads | Safety boundary is thread-based, not module-based | Refactor if any user-thread code needs unsafe |
| ARCH-7 | No legacy pipelines: geometry shaders, tessellation, traditional vertex | Lean modern codebase | Cannot support hardware without mesh shaders |
| ARCH-8 | No compatibility layers: MoltenVK, DXVK, KosmicKrisp | Direct native API access for performance | Extra backend work per platform |

## Platform Assumptions

| ID | Assumption | Rationale | Impact if Wrong |
|---|---|---|---|
| PLAT-1 | Metal 4 minimum (Apple Silicon M1+) | Mesh shaders require Metal 3+; Metal 4 for latest features | Cannot run on Intel Macs |
| PLAT-2 | Apple Silicon has unified memory | Staging buffer "copies" are a no-op | Need staging on non-unified memory Macs (Intel, unlikely) |
| PLAT-3 | `MTLIOCommandBuffer` available on macOS 13+ | Required for direct IO path | Need fallback IO on older macOS |
| PLAT-4 | Sparse textures available on macOS 14+ | Required for streaming tiles/voxels | Need non-sparse fallback or minimum macOS 14 |
| PLAT-5 | DirectStorage only available on Windows | Linux/SteamOS uses io_uring fallback | io_uring path must be equivalent in capability |
| PLAT-6 | io_uring available on Linux 5.1+ for SteamOS | SteamOS kernel version is modern | Fall back to async pread if io_uring unavailable |
| PLAT-7 | D3D12 Agility SDK provides latest features without OS updates | Shader Model 6.6, Enhanced Barriers | Minimum Windows version requirement increases |
| PLAT-8 | Vulkan 1.4 drivers available on SteamOS hardware | AMD RDNA2+ has Vulkan 1.4 support | May need to target Vulkan 1.3 as minimum |

## Animation Assumptions

| ID | Assumption | Rationale | Impact if Wrong |
|---|---|---|---|
| ANIM-1 | Max 256 bones per skeleton | Covers all production characters; fits structured buffer | Need larger palette buffer allocation |
| ANIM-2 | Max 4 simultaneous blend clips per instance | Sufficient for crossfade + additive layer | Increase GPU blend descriptor size |
| ANIM-3 | Morph target evaluation precedes skinning | Standard glTF/USD pipeline ordering | Reorder compute dispatch graph |
| ANIM-4 | State machine evaluation is CPU-only (Safe Rust) | Branchy graph traversal unsuited for GPU | Add GPU state machine compute pass |
| ANIM-5 | Animation curve outputs <= 32 slots per pass | Stays within push-constant budget on all platforms | Use UBO fallback for larger payloads |
| ANIM-6 | Ragdoll readback is opt-in (not default) | Avoids GPU stalls in common case | Change default if game logic commonly needs it |
| ANIM-7 | IK max chain length 8 bones | Covers arm, leg, spine IK chains | Increase compute shader loop bound |
| ANIM-8 | f16 delta precision sufficient for morph targets | Sub-millimeter deformations; validated by industry | Use f32 deltas if quality issues appear |

## Rendering Assumptions

| ID | Assumption | Rationale | Impact if Wrong |
|---|---|---|---|
| REND-1 | Forward+ and Deferred are mutually configurable | Graph builder selects one; execution plan specializes | Need both active simultaneously (unlikely) |
| REND-2 | Bindless descriptor heap model on all platforms | All target hardware supports it | Need per-draw descriptor binding fallback |
| REND-3 | 1 ray/pixel with temporal denoising is production quality | Industry standard (UE5, others) | Need multi-spp modes for quality settings |
| REND-4 | DDGI probe grid sufficient for diffuse GI | Works for most indoor/outdoor scenes | Need voxel GI alternative for dense scenes |
| REND-5 | Half-resolution RT dispatch with bilateral upscaling | Standard cost/quality tradeoff | Need full-res option for quality settings |
| REND-6 | BLAS compaction reduces memory 40-70% | Well-documented for triangle meshes | Budget VRAM without compaction savings |
| REND-7 | TLAS rebuild each frame is cheap (~0.1-0.5ms for 10K instances) | Instance count stays bounded | Need incremental TLAS updates |

## Streaming Assumptions

| ID | Assumption | Rationale | Impact if Wrong |
|---|---|---|---|
| STRM-1 | Fixed-capacity GPU resource pools (no dynamic reallocation) | Prevents fragmentation | Pools may be too small or waste VRAM |
| STRM-2 | LRU eviction with hysteresis prevents thrashing | Standard streaming policy | Need more sophisticated eviction policy |
| STRM-3 | Predictive prefetch using camera velocity | Works for player-controlled cameras | Need path-based prefetch for cutscenes |
| STRM-4 | Sparse textures align to hardware tile granularity | Required for efficient sparse binding | Granularity mismatch wastes memory |
| STRM-5 | 16^3 voxel pages preferred over 32^3 | Finer granularity, faster extraction | Need larger pages for coherence |

## Tooling Assumptions

| ID | Assumption | Rationale | Impact if Wrong |
|---|---|---|---|
| TOOL-1 | Naga is sole shader IR; no hand-authored HLSL/MSL | Portability guarantee | Need manual shader escape hatch |
| TOOL-2 | Shader graph files are serializable MessagePack | Compact, deterministic, diffable | Change format if tooling requires it |
| TOOL-3 | Blender GLTF 2.0 is the primary import format | Industry standard, well-supported | Add FBX/USD importers |
| TOOL-4 | Material → shader graph auto-generation from GLTF | Covers standard PBR materials | Custom materials need manual graph authoring |
| TOOL-5 | Max 32-64 SSS profiles in bindless array | Sufficient for most scenes | Increase buffer size |

## UI/2D Assumptions

| ID | Assumption | Rationale | Impact if Wrong |
|---|---|---|---|
| UI-1 | Vector UI uses vello-style compute pipeline | Best GPU utilization for path rendering | Mesh shader curve expansion alternative |
| UI-2 | Physics integration is CPU-write/GPU-read only | No GPU physics engine in scope | Interface changes for GPU physics |
| UI-3 | Isometric sort runs CPU-side (generic distance sort) | O(N log N) sufficient for typical tile counts | Need GPU radix sort for >100K tiles |
| UI-4 | UI passes do not write depth buffer | Pure alpha compositing | Changes for diegetic in-world UI |
| UI-5 | Bitmap UI glyph rasterization via C++ FreeType | Industry standard, high quality | Swap for Rust-native rasterizer |
