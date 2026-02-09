# D3D12 backend (Windows). Uses Windows SDK for D3D12/DXGI.
# DXC and DirectStorage: set DXC_PATH and DSTORAGE_PATH env vars to NuGet package roots.

set(GCRAFT_D3D12_SOURCES
  gcraft_command.cpp
  gcraft_descriptor.cpp
  gcraft_device.cpp
  gcraft_instance.cpp
  gcraft_internal.cpp
  gcraft_pipeline.cpp
  gcraft_resource.cpp
  gcraft_sync.cpp
)

add_library(gcraft_d3d12 STATIC ${GCRAFT_D3D12_SOURCES})
target_include_directories(gcraft_d3d12 PRIVATE
  ${GCRAFT_INCLUDE_DIR}
  ${CMAKE_CURRENT_SOURCE_DIR}
)
target_compile_features(gcraft_d3d12 PRIVATE cxx_std_20)
target_compile_definitions(gcraft_d3d12 PRIVATE UNICODE _UNICODE WIN32_LEAN_AND_MEAN NOMINMAX)
if(CMAKE_CXX_COMPILER_ID STREQUAL "MSVC")
  target_compile_options(gcraft_d3d12 PRIVATE /W4)
else()
  target_compile_options(gcraft_d3d12 PRIVATE -fms-extensions -Wno-ignored-attributes)
endif()
target_link_libraries(gcraft_d3d12 PUBLIC
  d3d12
  dxgi
  dxcompiler
  dstorage
  kernel32
  user32
  ole32
)

# D3D12 test executable
add_executable(gcraft_d3d12_test gcraft_test.cpp)
target_include_directories(gcraft_d3d12_test PRIVATE ${GCRAFT_INCLUDE_DIR} ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_features(gcraft_d3d12_test PRIVATE cxx_std_20)
target_compile_definitions(gcraft_d3d12_test PRIVATE UNICODE _UNICODE WIN32_LEAN_AND_MEAN NOMINMAX)
target_link_libraries(gcraft_d3d12_test PRIVATE gcraft_d3d12)

add_test(NAME test-d3d12 COMMAND gcraft_d3d12_test)
