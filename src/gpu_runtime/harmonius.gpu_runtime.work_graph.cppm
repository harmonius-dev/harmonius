/// @file harmonius.gpu_runtime.work_graph.cppm
/// @brief Work graph runtime — transparent execution via native GPU work graphs
///        (D3D12) or CPU-side emulation (all backends).
///
/// When DeviceCapabilities::work_graphs is true, translates the render graph
/// execution plan into a GPU work graph program for GPU self-scheduling.
/// When false, performs traditional CPU-side command buffer recording per pass.
///
/// Requirements: GR-3.1–GR-3.7

export module harmonius.gpu_runtime.work_graph;

import harmonius.gpu;

export namespace harmonius::gpu_runtime::work_graph {

// Placeholder — see docs/design/gpu-runtime.md for full API design.

} // namespace harmonius::gpu_runtime::work_graph
