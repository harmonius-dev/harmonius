set(GCRAFT_INCLUDE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/include" CACHE INTERNAL "gcraft public include dir")

if(GCRAFT_BACKEND_D3D12)
  add_subdirectory(d3d12)
endif()
if(GCRAFT_BACKEND_VULKAN)
  add_subdirectory(vulkan)
endif()
