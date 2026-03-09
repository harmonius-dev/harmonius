add_library(harmonius-gpu-runtime STATIC)
target_sources(
  harmonius-gpu-runtime
  PUBLIC
    FILE_SET CXX_MODULES FILES
      harmonius.gpu_runtime.cppm
      harmonius.gpu_runtime.memory.cppm
      harmonius.gpu_runtime.state.cppm
      harmonius.gpu_runtime.work_graph.cppm
      harmonius.gpu_runtime.compat.cppm)
target_link_libraries(harmonius-gpu-runtime PUBLIC harmonius-gpu)
