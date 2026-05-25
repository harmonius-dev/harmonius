cmake_minimum_required(VERSION 4.0)

if(POLICY CMP0157)
  cmake_policy(SET CMP0157 NEW)
endif()

project(Harmonius LANGUAGES CXX Swift)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake/modules")

include(InitializeSwift)
include(AddSwift)

set(CMAKE_CXX_STANDARD 26)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_Swift_LANGUAGE_VERSION 6)
set(CMAKE_Swift_COMPILATION_MODE incremental)

# Global Swift ↔ C++ interop flags applied to all Swift compilation units.
add_compile_options(
  "$<$<COMPILE_LANGUAGE:Swift>:SHELL:-cxx-interoperability-mode=default>"
  "$<$<COMPILE_LANGUAGE:Swift>:SHELL:-Xcc -std=c++20>")

add_subdirectory(app)
