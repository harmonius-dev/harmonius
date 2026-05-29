cmake_minimum_required(VERSION 4.0)

if(POLICY CMP0157)
  cmake_policy(SET CMP0157 NEW)
endif()

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake/modules")

project(Harmonius LANGUAGES CXX Swift)

include(InitializeSwift)
include(AddSwift)
include(CTest)
include(HarmoniusCodeIntelligence)

set(CMAKE_CXX_STANDARD 26)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_Swift_LANGUAGE_VERSION 6)
set(CMAKE_Swift_COMPILATION_MODE incremental)

# Global Swift ↔ C++ interop flags applied to all Swift compilation units.
add_compile_options(
  "$<$<COMPILE_LANGUAGE:Swift>:SHELL:-cxx-interoperability-mode=default>"
  "$<$<COMPILE_LANGUAGE:Swift>:SHELL:-Xcc -std=c++26>"
  "$<$<COMPILE_LANGUAGE:Swift>:SHELL:-Xcc -fexceptions>")

if(CMAKE_OSX_SYSROOT)
  add_compile_options(
    "$<$<COMPILE_LANGUAGE:Swift>:SHELL:-sdk ${CMAKE_OSX_SYSROOT}>")
endif()

include(FetchContent)
FetchContent_Declare(
  hlslpp
  GIT_REPOSITORY https://github.com/redorav/hlslpp.git
  GIT_TAG "3.9")
FetchContent_MakeAvailable(hlslpp)
set(HARMONIUS_HLSLPP_INCLUDE_DIR "${hlslpp_SOURCE_DIR}/include")

harmonius_generate_clangd_database(
  OUTPUT_DIR
  "${CMAKE_BINARY_DIR}/clangd"
  SOURCES
  app/Shaders/ShaderTypes.h
  app/Shaders/SwiftCompat.h
  INCLUDE_DIRS
  app/Shaders
  "${HARMONIUS_HLSLPP_INCLUDE_DIR}")

add_subdirectory(app)
