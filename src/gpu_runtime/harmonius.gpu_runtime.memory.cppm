/// @file harmonius.gpu_runtime.memory.cppm
/// @brief GPU memory management — heap sub-allocation (TLSF), ring buffers,
///        pool allocation, defragmentation, and budget tracking.
///
/// Replaces VMA and D3D12MA as the sole memory management layer. Built entirely
/// on top of harmonius::gpu device methods (create_heap, create_placed_texture, etc.).
///
/// Requirements: GR-1.1–GR-1.11

export module harmonius.gpu_runtime.memory;

import harmonius.gpu;

export namespace harmonius::gpu_runtime::memory {

// Placeholder — see docs/design/gpu-runtime.md for full API design.

} // namespace harmonius::gpu_runtime::memory
