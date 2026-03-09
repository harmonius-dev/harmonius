/// @file harmonius.gpu_runtime.cppm
/// @brief GPU runtime shared services — memory, state tracking, work graphs, compat.
///
/// Sits between the GPU backend interface (harmonius::gpu) and higher-level consumers
/// (render graph, asset pipeline). Provides memory management, GPU state tracking,
/// transparent work graph execution, and cross-backend feature emulation.

export module harmonius.gpu_runtime;

export import harmonius.gpu_runtime.memory;
export import harmonius.gpu_runtime.state;
export import harmonius.gpu_runtime.work_graph;
export import harmonius.gpu_runtime.compat;
