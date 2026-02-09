cmake_minimum_required(VERSION 3.21)
project(
  gcraft
  VERSION 0.1.0
  LANGUAGES CXX
  DESCRIPTION "Cross-platform graphics abstraction")

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Backend options
set(GCRAFT_BACKEND_D3D12
    OFF
    CACHE BOOL "Build D3D12 backend (Windows)")
set(GCRAFT_BACKEND_VULKAN
    OFF
    CACHE BOOL "Build Vulkan backend")

# Auto-enable backends by platform
if(UNIX OR APPLE)
  # Vulkan on macOS via KosmicKrisp from LunarG SDK
  set(GCRAFT_BACKEND_VULKAN
      ON
      CACHE BOOL "Build Vulkan backend" FORCE)
endif()
if(WIN32)
  set(GCRAFT_BACKEND_D3D12
      ON
      CACHE BOOL "Build D3D12 backend" FORCE)
endif()

enable_testing()
add_subdirectory(src)
