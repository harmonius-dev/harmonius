add_library(harmonius-gpu-vulkan STATIC)
target_sources(harmonius-gpu-vulkan PUBLIC FILE_SET CXX_MODULES FILES
                                           harmonius.gpu.vulkan.cppm)
target_link_libraries(
  harmonius-gpu-vulkan PUBLIC harmonius-gpu Vulkan::Vulkan)
