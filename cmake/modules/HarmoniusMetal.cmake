# HarmoniusMetal.cmake Low-level Metal shader compilation (.metal → .air →
# .metallib) for Ninja builds. Used internally by HarmoniusModule.cmake; may
# also be called directly.
#
# Public API: harmonius_metal_compile_air(OUTPUT <var> SOURCE <file>
# [INCLUDE_DIRS ...]) harmonius_metal_link_metallib(OUTPUT <var> OUTPUT_NAME
# <name> SOURCES ...) harmonius_add_metallib(<TARGET> SOURCES ... [OUTPUT_NAME
# <name>] [INCLUDE_DIRS ...]) harmonius_metallib_link_libraries(<TARGET> <dep>
# ...)

cmake_minimum_required(VERSION 4.0)

# ---------------------------------------------------------------------------
# Internal: derive xcrun SDK name from CMake platform variables.
# ---------------------------------------------------------------------------
function(_harmonius_metal_sdk _out)
  if(CMAKE_SYSTEM_NAME STREQUAL "iOS")
    if(CMAKE_OSX_SYSROOT MATCHES "[Ss]imulator")
      set(${_out}
          "iphonesimulator"
          PARENT_SCOPE)
    else()
      set(${_out}
          "iphoneos"
          PARENT_SCOPE)
    endif()
  else()
    set(${_out}
        "macosx"
        PARENT_SCOPE)
  endif()
endfunction()

# ---------------------------------------------------------------------------
# harmonius_metal_compile_air
#
# Compile one .metal source file to a .air intermediate object. Outputs the .air
# path in OUTPUT.
#
# Usage: harmonius_metal_compile_air( OUTPUT       <var> SOURCE <path-to-.metal>
# [INCLUDE_DIRS <dir ...>] )
# ---------------------------------------------------------------------------
function(harmonius_metal_compile_air)
  cmake_parse_arguments(_A "" "OUTPUT;SOURCE" "INCLUDE_DIRS" ${ARGN})
  if(NOT _A_OUTPUT)
    message(FATAL_ERROR "harmonius_metal_compile_air: OUTPUT is required")
  endif()
  if(NOT _A_SOURCE)
    message(FATAL_ERROR "harmonius_metal_compile_air: SOURCE is required")
  endif()

  cmake_path(IS_RELATIVE _A_SOURCE _is_rel)
  if(_is_rel)
    set(_A_SOURCE "${CMAKE_CURRENT_SOURCE_DIR}/${_A_SOURCE}")
  endif()

  _harmonius_metal_sdk(_sdk)
  cmake_path(GET _A_SOURCE STEM _stem)
  set(_air "${CMAKE_CURRENT_BINARY_DIR}/metal_air/${_stem}.air")

  set(_inc_flags "")
  foreach(_d IN LISTS _A_INCLUDE_DIRS)
    cmake_path(IS_RELATIVE _d _drel)
    if(_drel)
      set(_d "${CMAKE_CURRENT_SOURCE_DIR}/${_d}")
    endif()
    list(APPEND _inc_flags "-I${_d}")
  endforeach()

  add_custom_command(
    OUTPUT "${_air}"
    COMMAND "${CMAKE_COMMAND}" -E make_directory
            "${CMAKE_CURRENT_BINARY_DIR}/metal_air"
    COMMAND xcrun -sdk "${_sdk}" metal -c "${_A_SOURCE}" -o "${_air}"
            ${_inc_flags}
    DEPENDS "${_A_SOURCE}"
    COMMENT "Metal compile ${_stem}.air"
    VERBATIM COMMAND_EXPAND_LISTS)

  set(${_A_OUTPUT}
      "${_air}"
      PARENT_SCOPE)
endfunction()

# ---------------------------------------------------------------------------
# harmonius_metal_link_metallib
#
# Link one or more .air files into a named .metallib. Outputs the .metallib path
# in OUTPUT.
#
# Usage: harmonius_metal_link_metallib( OUTPUT       <var> OUTPUT_NAME  <name> #
# produces <name>.metallib SOURCES      <.air-file ...> )
# ---------------------------------------------------------------------------
function(harmonius_metal_link_metallib)
  cmake_parse_arguments(_L "" "OUTPUT;OUTPUT_NAME" "SOURCES" ${ARGN})
  if(NOT _L_OUTPUT)
    message(FATAL_ERROR "harmonius_metal_link_metallib: OUTPUT is required")
  endif()
  if(NOT _L_OUTPUT_NAME)
    message(
      FATAL_ERROR "harmonius_metal_link_metallib: OUTPUT_NAME is required")
  endif()
  if(NOT _L_SOURCES)
    message(FATAL_ERROR "harmonius_metal_link_metallib: SOURCES is required")
  endif()

  _harmonius_metal_sdk(_sdk)
  set(_lib "${CMAKE_CURRENT_BINARY_DIR}/metallibs/${_L_OUTPUT_NAME}.metallib")

  add_custom_command(
    OUTPUT "${_lib}"
    COMMAND "${CMAKE_COMMAND}" -E make_directory
            "${CMAKE_CURRENT_BINARY_DIR}/metallibs"
    COMMAND xcrun -sdk "${_sdk}" metallib ${_L_SOURCES} -o "${_lib}"
    DEPENDS ${_L_SOURCES}
    COMMENT "Metal link ${_L_OUTPUT_NAME}.metallib"
    VERBATIM COMMAND_EXPAND_LISTS)

  set(${_L_OUTPUT}
      "${_lib}"
      PARENT_SCOPE)
endfunction()

# ---------------------------------------------------------------------------
# harmonius_add_metallib
#
# Create a named custom CMake target that builds a .metallib from Metal sources.
#
# Properties set on the target: HARMONIUS_METALLIB_FILE  — absolute path to the
# produced .metallib HARMONIUS_METALLIB_NAME  — output name (without .metallib)
# HARMONIUS_METALLIB_DEPS  — other harmonius_add_metallib targets depended on
#
# Usage: harmonius_add_metallib(<TARGET> SOURCES      <.metal-file ...>
# [OUTPUT_NAME <name>]           # default: "${TARGET}" [INCLUDE_DIRS <dir ...>]
# )
# ---------------------------------------------------------------------------
function(harmonius_add_metallib _target)
  cmake_parse_arguments(_M "" "OUTPUT_NAME" "SOURCES;INCLUDE_DIRS" ${ARGN})
  if(NOT _M_SOURCES)
    message(FATAL_ERROR "harmonius_add_metallib: SOURCES is required")
  endif()
  if(NOT _M_OUTPUT_NAME)
    set(_M_OUTPUT_NAME "${_target}")
  endif()

  set(_air_files "")
  foreach(_src IN LISTS _M_SOURCES)
    harmonius_metal_compile_air(OUTPUT _air SOURCE "${_src}" INCLUDE_DIRS
                                ${_M_INCLUDE_DIRS})
    list(APPEND _air_files "${_air}")
  endforeach()

  harmonius_metal_link_metallib(OUTPUT _lib_file OUTPUT_NAME
                                "${_M_OUTPUT_NAME}" SOURCES ${_air_files})

  add_custom_target("${_target}" DEPENDS "${_lib_file}")

  set_target_properties(
    "${_target}" PROPERTIES HARMONIUS_METALLIB_FILE "${_lib_file}"
                            HARMONIUS_METALLIB_NAME "${_M_OUTPUT_NAME}")
endfunction()

# ---------------------------------------------------------------------------
# harmonius_metallib_link_libraries
#
# Declare that TARGET depends on one or more other harmonius_add_metallib
# targets. Adds CMake build-order dependencies and records the deps in the
# HARMONIUS_METALLIB_DEPS property for transitive bundle assembly.
#
# Usage: harmonius_metallib_link_libraries(<TARGET> <dep> ...)
# ---------------------------------------------------------------------------
function(harmonius_metallib_link_libraries _target)
  foreach(_dep IN LISTS ARGN)
    add_dependencies("${_target}" "${_dep}")
  endforeach()
  set_property(
    TARGET "${_target}"
    APPEND
    PROPERTY HARMONIUS_METALLIB_DEPS "${ARGN}")
endfunction()

# ---------------------------------------------------------------------------
# Internal: recursively collect all HARMONIUS_METALLIB_FILE values from a list
# of harmonius_add_metallib targets (following HARMONIUS_METALLIB_DEPS).
#
# _harmonius_collect_metallib_files(<out-var> <target> ...)
# ---------------------------------------------------------------------------
function(_harmonius_collect_metallib_files _result_var)
  set(_files "")
  foreach(_tgt IN LISTS ARGN)
    if(NOT TARGET "${_tgt}")
      continue()
    endif()
    get_target_property(_f "${_tgt}" HARMONIUS_METALLIB_FILE)
    if(_f)
      list(APPEND _files "${_f}")
    endif()
    get_target_property(_deps "${_tgt}" HARMONIUS_METALLIB_DEPS)
    if(_deps)
      _harmonius_collect_metallib_files(_sub ${_deps})
      list(APPEND _files ${_sub})
    endif()
  endforeach()
  list(REMOVE_DUPLICATES _files)
  set(${_result_var}
      "${_files}"
      PARENT_SCOPE)
endfunction()
