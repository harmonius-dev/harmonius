cmake_minimum_required(VERSION 3.30)

# Enable import std; support (experimental in CMake 4.x)
set(CMAKE_EXPERIMENTAL_CXX_IMPORT_STD "d0edc3af-4c50-42ea-a356-e2862fe7a444")

project(
  harmonius
  VERSION 0.1.0
  LANGUAGES CXX
)

set(CMAKE_CXX_STANDARD 26)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_MODULE_STD ON)

# On macOS with homebrew LLVM, point CMake at the libc++ modules.json
if(APPLE AND CMAKE_CXX_COMPILER MATCHES "llvm")
  set(CMAKE_CXX_STDLIB_MODULES_JSON
      "/opt/homebrew/opt/llvm/lib/c++/libc++.modules.json"
      CACHE FILEPATH "Path to libc++.modules.json for import std support"
  )
endif()

# Backend selection — one backend per binary
set(HARMONIUS_GPU_BACKEND
    ""
    CACHE STRING "GPU backend: metal, d3d12, vulkan"
)

# Auto-detect if not specified
if(NOT HARMONIUS_GPU_BACKEND)
  if(APPLE)
    set(HARMONIUS_GPU_BACKEND "metal")
  elseif(WIN32)
    set(HARMONIUS_GPU_BACKEND "d3d12")
  elseif(UNIX)
    set(HARMONIUS_GPU_BACKEND "vulkan")
  endif()
endif()

message(STATUS "Harmonius GPU backend: ${HARMONIUS_GPU_BACKEND}")

# Platform-specific dependencies
if(HARMONIUS_GPU_BACKEND STREQUAL "metal")
  find_library(METAL_FRAMEWORK Metal REQUIRED)
  find_library(QUARTZCORE_FRAMEWORK QuartzCore REQUIRED)
  find_library(FOUNDATION_FRAMEWORK Foundation REQUIRED)
  find_library(IOKIT_FRAMEWORK IOKit REQUIRED)
  find_library(IOSURFACE_FRAMEWORK IOSurface REQUIRED)

  add_library(metal-cpp INTERFACE)
  target_include_directories(metal-cpp INTERFACE vendor/metal-cpp)
  target_link_libraries(
    metal-cpp INTERFACE ${METAL_FRAMEWORK} ${QUARTZCORE_FRAMEWORK} ${FOUNDATION_FRAMEWORK} ${IOKIT_FRAMEWORK}
                        ${IOSURFACE_FRAMEWORK}
  )
elseif(HARMONIUS_GPU_BACKEND STREQUAL "vulkan")
  find_package(Vulkan REQUIRED)
elseif(HARMONIUS_GPU_BACKEND STREQUAL "d3d12")
  find_package(directx-headers CONFIG REQUIRED)
endif()
