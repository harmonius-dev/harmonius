# HarmoniusSlang.cmake Low-level Slang shader compilation (.slang → .metallib)
# for Ninja builds. Used internally by HarmoniusModule.cmake; may also be called
# directly.
#
# slangc is invoked once per slanglib target with `-target metallib`; it shells
# out to xcrun metal / metallib internally and emits an Apple-loadable
# .metallib. Replaces the previous .metal → .air → .metallib pipeline.
#
# Public API: harmonius_add_slanglib(<TARGET> SOURCES      <.slang-file ...>
# [OUTPUT_NAME <name>]           # default: "${TARGET}" [INCLUDE_DIRS <dir
# ...>]) harmonius_slanglib_link_libraries(<TARGET> <dep> ...)

cmake_minimum_required(VERSION 4.0)

find_package(slang CONFIG REQUIRED)

# Resolve the slangc compiler binary. vcpkg installs it under
# tools/shader-slang/ which is not on PATH; the HINTS path is derived from the
# directory holding slangConfig.cmake (lib/cmake/slang/).
if(NOT HARMONIUS_SLANGC)
  find_program(
    HARMONIUS_SLANGC
    NAMES slangc
    HINTS "${slang_DIR}/../../../tools/shader-slang" REQUIRED)
  message(STATUS "HARMONIUS_SLANGC: ${HARMONIUS_SLANGC}")
endif()

# ---------------------------------------------------------------------------
# harmonius_add_slanglib
#
# Create a named custom CMake target that compiles one or more .slang sources
# into a single .metallib via slangc -target metallib.
#
# Properties set on the target: HARMONIUS_SLANGLIB_FILE  — absolute path to the
# produced .metallib HARMONIUS_SLANGLIB_NAME  — output name (without .metallib)
# HARMONIUS_SLANGLIB_DEPS  — other harmonius_add_slanglib targets depended on
#
# Usage: harmonius_add_slanglib(<TARGET> SOURCES      <.slang-file ...>
# [OUTPUT_NAME <name>]           # default: "${TARGET}" [INCLUDE_DIRS <dir
# ...>])
# ---------------------------------------------------------------------------
function(harmonius_add_slanglib _target)
  cmake_parse_arguments(_S "" "OUTPUT_NAME" "SOURCES;INCLUDE_DIRS" ${ARGN})
  if(NOT _S_SOURCES)
    message(FATAL_ERROR "harmonius_add_slanglib: SOURCES is required")
  endif()
  if(NOT _S_OUTPUT_NAME)
    set(_S_OUTPUT_NAME "${_target}")
  endif()

  set(_abs_srcs "")
  foreach(_src IN LISTS _S_SOURCES)
    cmake_path(IS_RELATIVE _src _rel)
    if(_rel)
      set(_src "${CMAKE_CURRENT_SOURCE_DIR}/${_src}")
    endif()
    list(APPEND _abs_srcs "${_src}")
  endforeach()

  set(_inc_flags "")
  foreach(_d IN LISTS _S_INCLUDE_DIRS)
    cmake_path(IS_RELATIVE _d _drel)
    if(_drel)
      set(_d "${CMAKE_CURRENT_SOURCE_DIR}/${_d}")
    endif()
    list(APPEND _inc_flags "-I${_d}")
  endforeach()

  set(_lib "${CMAKE_CURRENT_BINARY_DIR}/metallibs/${_S_OUTPUT_NAME}.metallib")

  add_custom_command(
    OUTPUT "${_lib}"
    COMMAND "${CMAKE_COMMAND}" -E make_directory
            "${CMAKE_CURRENT_BINARY_DIR}/metallibs"
    COMMAND "${HARMONIUS_SLANGC}" -target metallib -o "${_lib}" ${_inc_flags}
            ${_abs_srcs}
    DEPENDS ${_abs_srcs}
    COMMENT "Slang compile ${_S_OUTPUT_NAME}.metallib"
    VERBATIM COMMAND_EXPAND_LISTS)

  add_custom_target("${_target}" DEPENDS "${_lib}")

  set_target_properties(
    "${_target}" PROPERTIES HARMONIUS_SLANGLIB_FILE "${_lib}"
                            HARMONIUS_SLANGLIB_NAME "${_S_OUTPUT_NAME}")
endfunction()

# ---------------------------------------------------------------------------
# harmonius_slanglib_link_libraries
#
# Declare that TARGET depends on one or more other harmonius_add_slanglib
# targets. Adds CMake build-order dependencies and records the deps in the
# HARMONIUS_SLANGLIB_DEPS property for transitive bundle assembly.
#
# Usage: harmonius_slanglib_link_libraries(<TARGET> <dep> ...)
# ---------------------------------------------------------------------------
function(harmonius_slanglib_link_libraries _target)
  foreach(_dep IN LISTS ARGN)
    add_dependencies("${_target}" "${_dep}")
  endforeach()
  set_property(
    TARGET "${_target}"
    APPEND
    PROPERTY HARMONIUS_SLANGLIB_DEPS "${ARGN}")
endfunction()

# ---------------------------------------------------------------------------
# Internal: recursively collect all HARMONIUS_SLANGLIB_FILE values from a list
# of harmonius_add_slanglib targets (following HARMONIUS_SLANGLIB_DEPS).
#
# _harmonius_collect_slanglib_files(<out-var> <target> ...)
# ---------------------------------------------------------------------------
function(_harmonius_collect_slanglib_files _result_var)
  set(_files "")
  foreach(_tgt IN LISTS ARGN)
    if(NOT TARGET "${_tgt}")
      continue()
    endif()
    get_target_property(_f "${_tgt}" HARMONIUS_SLANGLIB_FILE)
    if(_f)
      list(APPEND _files "${_f}")
    endif()
    get_target_property(_deps "${_tgt}" HARMONIUS_SLANGLIB_DEPS)
    if(_deps)
      _harmonius_collect_slanglib_files(_sub ${_deps})
      list(APPEND _files ${_sub})
    endif()
  endforeach()
  list(REMOVE_DUPLICATES _files)
  set(${_result_var}
      "${_files}"
      PARENT_SCOPE)
endfunction()
