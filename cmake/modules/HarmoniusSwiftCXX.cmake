# HarmoniusSwiftCXX.cmake Swift 6.3 ↔ C++26 interoperability helpers. Extends
# the upstream AddSwift.cmake with project-level wrappers. Used internally by
# HarmoniusModule.cmake.
#
# Public API: harmonius_swift_cxx_apply_module(<TARGET>)
# harmonius_swift_cxx_generate_header(<TARGET> <HEADER> [SEARCH_PATHS ...])
# harmonius_swift_cxx_link_cpp_library(<TARGET> <CPP_TARGET>)

cmake_minimum_required(VERSION 4.0)

include(AddSwift)

# ---------------------------------------------------------------------------
# harmonius_swift_cxx_apply_module
#
# Apply Swift 6.3 / C++26 interop compile flags to TARGET. Also enables
# per-module index store for IDE integration.
#
# This is called automatically by harmonius_add_module for every module that has
# Swift sources.
# ---------------------------------------------------------------------------
function(harmonius_swift_cxx_apply_module _target)
  target_compile_options(
    "${_target}"
    PRIVATE
      "$<$<COMPILE_LANGUAGE:Swift>:SHELL:-cxx-interoperability-mode=default>"
      "$<$<COMPILE_LANGUAGE:Swift>:SHELL:-Xcc -std=c++26>"
      "$<$<COMPILE_LANGUAGE:Swift>:SHELL:\
-index-store-path ${CMAKE_BINARY_DIR}/index-store/${_target}>")
endfunction()

# ---------------------------------------------------------------------------
# harmonius_swift_cxx_generate_header
#
# Emit a Swift → C++ bridging header for TARGET using swiftc -typecheck. HEADER
# is the output filename (relative to CMAKE_CURRENT_BINARY_DIR/include).
#
# Wraps _swift_generate_cxx_header from AddSwift.cmake.
#
# Usage: harmonius_swift_cxx_generate_header(<TARGET> <header-filename>
# [SEARCH_PATHS <dir ...>] [MODULE_NAME <name>] )
# ---------------------------------------------------------------------------
function(harmonius_swift_cxx_generate_header _target _header)
  cmake_parse_arguments(_H "" "MODULE_NAME" "SEARCH_PATHS" ${ARGN})
  set(_extra "")
  if(_H_SEARCH_PATHS)
    list(APPEND _extra SEARCH_PATHS ${_H_SEARCH_PATHS})
  endif()
  if(_H_MODULE_NAME)
    list(APPEND _extra MODULE_NAME ${_H_MODULE_NAME})
  endif()
  _swift_generate_cxx_header("${_target}" "${_header}" ${_extra})
endfunction()

# ---------------------------------------------------------------------------
# harmonius_swift_cxx_link_cpp_library
#
# Link a C++ static library into a Swift module and expose its include paths to
# Swift via -Xcc -I flags so that C++ headers are importable.
#
# Usage: harmonius_swift_cxx_link_cpp_library(<SWIFT_TARGET> <CPP_TARGET>)
# ---------------------------------------------------------------------------
function(harmonius_swift_cxx_link_cpp_library _swift_target _cpp_target)
  target_link_libraries("${_swift_target}" PRIVATE "${_cpp_target}")

  get_target_property(_inc_dirs "${_cpp_target}" INTERFACE_INCLUDE_DIRECTORIES)
  if(_inc_dirs)
    foreach(_dir IN LISTS _inc_dirs)
      target_compile_options(
        "${_swift_target}"
        PRIVATE "$<$<COMPILE_LANGUAGE:Swift>:SHELL:-Xcc -I${_dir}>")
    endforeach()
  endif()
endfunction()
