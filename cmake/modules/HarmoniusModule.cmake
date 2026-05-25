# HarmoniusModule.cmake Top-level API for mixed Swift 6.3 / C++26 / Metal
# libraries.
#
# Public API: harmonius_add_module(<TARGET> [SWIFT_SOURCES <file ...>]
# [CXX_SOURCES   <file ...>] [METAL_SOURCES <file ...>] [METAL_OUTPUT_NAME
# <name>]    # default: "default" when METAL_SOURCES set [METAL_INCLUDE_DIRS
# <dir ...>] [INCLUDE_DIRS <dir ...>]      # added to C++ and Swift -Xcc -I
# flags [LIBRARIES <target ...>]      # other harmonius_add_module targets to
# link [FRAMEWORKS <name ...>]       # Apple frameworks, e.g. Metal SwiftUI )
#
# The created static library target has: Swift_MODULE_NAME        = <TARGET>

cmake_minimum_required(VERSION 4.0)

include(HarmoniusMetal)
include(HarmoniusSwiftCXX)

function(harmonius_add_module _target)
  cmake_parse_arguments(
    _M
    ""
    "METAL_OUTPUT_NAME"
    "SWIFT_SOURCES;CXX_SOURCES;METAL_SOURCES;METAL_INCLUDE_DIRS;INCLUDE_DIRS;LIBRARIES;FRAMEWORKS"
    ${ARGN})

  # -------------------------------------------------------------------------
  # Resolve relative paths
  # -------------------------------------------------------------------------
  set(_swift_srcs "")
  foreach(_s IN LISTS _M_SWIFT_SOURCES)
    cmake_path(IS_RELATIVE _s _rel)
    if(_rel)
      list(APPEND _swift_srcs "${CMAKE_CURRENT_SOURCE_DIR}/${_s}")
    else()
      list(APPEND _swift_srcs "${_s}")
    endif()
  endforeach()

  set(_cxx_srcs "")
  foreach(_s IN LISTS _M_CXX_SOURCES)
    cmake_path(IS_RELATIVE _s _rel)
    if(_rel)
      list(APPEND _cxx_srcs "${CMAKE_CURRENT_SOURCE_DIR}/${_s}")
    else()
      list(APPEND _cxx_srcs "${_s}")
    endif()
  endforeach()

  set(_all_srcs ${_swift_srcs} ${_cxx_srcs})
  if(NOT _all_srcs)
    message(
      FATAL_ERROR
        "harmonius_add_module(${_target}): at least one of SWIFT_SOURCES or \
CXX_SOURCES is required")
  endif()

  # -------------------------------------------------------------------------
  # Static library
  # -------------------------------------------------------------------------
  add_library("${_target}" STATIC ${_all_srcs})

  set_target_properties("${_target}" PROPERTIES Swift_MODULE_NAME "${_target}")

  # -------------------------------------------------------------------------
  # Swift / C++ interop flags
  # -------------------------------------------------------------------------
  if(_swift_srcs)
    harmonius_swift_cxx_apply_module("${_target}")
  endif()

  # -------------------------------------------------------------------------
  # Include directories (C++ + Swift -Xcc -I)
  # -------------------------------------------------------------------------
  set(_abs_inc_dirs "")
  foreach(_d IN LISTS _M_INCLUDE_DIRS)
    cmake_path(IS_RELATIVE _d _rel)
    if(_rel)
      set(_d "${CMAKE_CURRENT_SOURCE_DIR}/${_d}")
    endif()
    list(APPEND _abs_inc_dirs "${_d}")
  endforeach()

  if(_abs_inc_dirs)
    # PUBLIC: consumers (e.g. modules that import this one) also get -I so they
    # can find transitively referenced Clang module maps embedded in this
    # module's .swiftmodule file.
    target_include_directories("${_target}" PUBLIC ${_abs_inc_dirs})
    if(_swift_srcs)
      foreach(_d IN LISTS _abs_inc_dirs)
        # PRIVATE: own compilation gets -Xcc -I for C++ header discovery.
        # INTERFACE: propagate to consumers that import this module.
        target_compile_options(
          "${_target}"
          PRIVATE "$<$<COMPILE_LANGUAGE:Swift>:SHELL:-Xcc -I${_d}>")
        target_compile_options(
          "${_target}"
          INTERFACE "$<$<COMPILE_LANGUAGE:Swift>:SHELL:-Xcc -I${_d}>")
      endforeach()
    endif()
  endif()

  # -------------------------------------------------------------------------
  # Library dependencies (other harmonius modules)
  # -------------------------------------------------------------------------
  if(_M_LIBRARIES)
    target_link_libraries("${_target}" PRIVATE ${_M_LIBRARIES})
  endif()

  # -------------------------------------------------------------------------
  # Apple frameworks
  # -------------------------------------------------------------------------
  foreach(_fw IN LISTS _M_FRAMEWORKS)
    target_link_libraries("${_target}" PRIVATE "-framework ${_fw}")
  endforeach()

  # -------------------------------------------------------------------------
  # Metal sources → named metallib custom target
  # -------------------------------------------------------------------------
  if(_M_METAL_SOURCES)
    if(NOT _M_METAL_OUTPUT_NAME)
      set(_M_METAL_OUTPUT_NAME "default")
    endif()

    set(_metal_target "${_target}_metal")
    harmonius_add_metallib(
      "${_metal_target}"
      SOURCES
      ${_M_METAL_SOURCES}
      OUTPUT_NAME
      "${_M_METAL_OUTPUT_NAME}"
      INCLUDE_DIRS
      ${_M_METAL_INCLUDE_DIRS})
  endif()
endfunction()
