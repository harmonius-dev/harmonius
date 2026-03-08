add_library(harmonius-gpu-d3d12 STATIC)
target_sources(harmonius-gpu-d3d12
    PUBLIC FILE_SET CXX_MODULES FILES
        harmonius.gpu.d3d12.cppm
)
target_link_libraries(harmonius-gpu-d3d12
    PUBLIC harmonius-gpu Microsoft::DirectX-Headers unofficial::d3d12-memory-allocator
)
