add_library(harmonius-gpu STATIC)
target_sources(harmonius-gpu
    PUBLIC FILE_SET CXX_MODULES FILES
        harmonius.gpu.cppm
        harmonius.gpu.concepts.cppm
)

if(HARMONIUS_GPU_BACKEND STREQUAL "metal")
    add_subdirectory(metal)
elseif(HARMONIUS_GPU_BACKEND STREQUAL "vulkan")
    add_subdirectory(vulkan)
elseif(HARMONIUS_GPU_BACKEND STREQUAL "d3d12")
    add_subdirectory(d3d12)
endif()
