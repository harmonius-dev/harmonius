/// @file harmonius.gpu_runtime.state.cppm
/// @brief GPU state tracking — tracked command buffers, resource state cache,
///        barrier optimization (batching, deduplication, elision).
///
/// Wraps gpu::CommandBuffer to filter redundant state-setting calls and
/// optimize barrier emission.
///
/// Requirements: GR-2.1–GR-2.7

export module harmonius.gpu_runtime.state;

import harmonius.gpu;

export namespace harmonius::gpu_runtime::state {

// Placeholder — see docs/design/gpu-runtime.md for full API design.

} // namespace harmonius::gpu_runtime::state
