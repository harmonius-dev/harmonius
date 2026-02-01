cmake_minimum_required(VERSION 3.21)
project(gcraft
  VERSION 0.0.0
  LANGUAGES CXX
  DESCRIPTION "Cross-platform graphics abstraction"
)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Ninja generator on all platforms (invoke as: cmake -G Ninja -B build)
if(NOT CMAKE_GENERATOR STREQUAL "Ninja" AND NOT CMAKE_GENERATOR STREQUAL "Ninja Multi-Config")
  message(STATUS "Recommended: use Ninja generator: cmake -G Ninja -B build")
endif()

# Backend options
set(GCRAFT_BACKEND_METAL OFF CACHE BOOL "Build Metal backend (macOS/iOS)")
set(GCRAFT_BACKEND_D3D12 OFF CACHE BOOL "Build D3D12 backend (Windows)")
set(GCRAFT_BACKEND_VULKAN OFF CACHE BOOL "Build Vulkan backend")

# Auto-enable backends by platform
if(APPLE)
  set(GCRAFT_BACKEND_METAL ON CACHE BOOL "Build Metal backend" FORCE)
endif()
if(WIN32)
  set(GCRAFT_BACKEND_D3D12 ON CACHE BOOL "Build D3D12 backend" FORCE)
endif()
if(UNIX AND NOT APPLE)
  set(GCRAFT_BACKEND_VULKAN ON CACHE BOOL "Build Vulkan backend" FORCE)
endif()
if(APPLE)
  # Vulkan on macOS via MoltenVK optional
  if(DEFINED ENV{VULKAN_SDK})
    set(GCRAFT_BACKEND_VULKAN ON CACHE BOOL "Build Vulkan backend" FORCE)
  endif()
endif()

enable_testing()
add_subdirectory(src)
