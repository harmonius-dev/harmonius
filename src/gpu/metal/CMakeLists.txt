add_library(harmonius-gpu-metal STATIC)
target_sources(harmonius-gpu-metal
    PUBLIC FILE_SET CXX_MODULES FILES
        harmonius.gpu.metal.cppm
)
target_link_libraries(harmonius-gpu-metal
    PUBLIC harmonius-gpu metal-cpp
)
