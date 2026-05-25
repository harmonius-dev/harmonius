# HarmoniusBundle.cmake Low-level .app bundle assembly for the Ninja generator.
# MACOSX_BUNDLE is never used — all layout is driven by custom commands.
#
# Bundle layout (macOS): <TARGET>.app/Contents/MacOS/<EXECUTABLE>
# <TARGET>.app/Contents/Info.plist <TARGET>.app/Contents/Resources/…
#
# Bundle layout (iOS / iOS Simulator): <TARGET>.app/<EXECUTABLE>
# <TARGET>.app/Info.plist <TARGET>.app/…
#
# Public API: harmonius_bundle_init(<TARGET> PLATFORM <macos|ios|ios-simulator>
# [BASE_DIR <dir>]) harmonius_bundle_set_info_plist(<TARGET> <plist-path>)
# harmonius_bundle_set_executable(<TARGET> <exe-cmake-target>)
# harmonius_bundle_add_resources(<TARGET> <file-or-dir ...>)
# harmonius_bundle_add_metallibs(<TARGET> <metallib-target ...>)
# harmonius_bundle_set_provisioning_profile(<TARGET> <path>)
# harmonius_bundle_finalize(<TARGET>) harmonius_bundle_install(<TARGET>
# [DESTINATION <dest>])

cmake_minimum_required(VERSION 4.0)

include(HarmoniusMetal)

# ---------------------------------------------------------------------------
# harmonius_bundle_init
#
# Declare a bundle target. Records the platform and output directory. Must be
# called before any other harmonius_bundle_* function for this target.
#
# The assembled .app lives at: ${BASE_DIR}/${TARGET}.app      (default BASE_DIR
# = CMAKE_BINARY_DIR)
# ---------------------------------------------------------------------------
function(harmonius_bundle_init _target)
  cmake_parse_arguments(_I "" "PLATFORM;BASE_DIR" "" ${ARGN})
  if(NOT _I_PLATFORM)
    message(
      FATAL_ERROR "harmonius_bundle_init(${_target}): PLATFORM is required")
  endif()
  if(NOT _I_BASE_DIR)
    set(_I_BASE_DIR "${CMAKE_BINARY_DIR}")
  endif()

  set(_bundle_dir "${_I_BASE_DIR}/${_target}.app")

  # Create a proxy INTERFACE target to hold bundle metadata as properties. The
  # actual custom target (${_target}_bundle) is created by _finalize.
  add_library("${_target}_bundle_meta" INTERFACE)
  set_target_properties(
    "${_target}_bundle_meta"
    PROPERTIES HARMONIUS_BUNDLE_PLATFORM "${_I_PLATFORM}"
               HARMONIUS_BUNDLE_DIR "${_bundle_dir}"
               HARMONIUS_BUNDLE_PLIST ""
               HARMONIUS_BUNDLE_EXE_TARGET ""
               HARMONIUS_BUNDLE_METALLIBS ""
               HARMONIUS_BUNDLE_RESOURCES ""
               HARMONIUS_BUNDLE_PROVISIONING "")
endfunction()

# ---------------------------------------------------------------------------
# harmonius_bundle_set_info_plist
# ---------------------------------------------------------------------------
function(harmonius_bundle_set_info_plist _target _plist)
  cmake_path(IS_RELATIVE _plist _rel)
  if(_rel)
    set(_plist "${CMAKE_CURRENT_SOURCE_DIR}/${_plist}")
  endif()
  set_target_properties("${_target}_bundle_meta"
                        PROPERTIES HARMONIUS_BUNDLE_PLIST "${_plist}")
endfunction()

# ---------------------------------------------------------------------------
# harmonius_bundle_set_executable
#
# _exe_target is the CMake executable target whose binary goes into MacOS/ (or
# the iOS bundle root). $<TARGET_FILE:_exe_target> is used at build time.
# ---------------------------------------------------------------------------
function(harmonius_bundle_set_executable _target _exe_target)
  set_target_properties("${_target}_bundle_meta"
                        PROPERTIES HARMONIUS_BUNDLE_EXE_TARGET "${_exe_target}")
endfunction()

# ---------------------------------------------------------------------------
# harmonius_bundle_add_resources
#
# Append files/directories to be copied into Contents/Resources (macOS) or the
# bundle root (iOS). Paths may be absolute or relative to
# CMAKE_CURRENT_SOURCE_DIR.
# ---------------------------------------------------------------------------
function(harmonius_bundle_add_resources _target)
  set(_res "")
  foreach(_r IN LISTS ARGN)
    cmake_path(IS_RELATIVE _r _rel)
    if(_rel)
      set(_r "${CMAKE_CURRENT_SOURCE_DIR}/${_r}")
    endif()
    list(APPEND _res "${_r}")
  endforeach()
  if(_res)
    set_property(
      TARGET "${_target}_bundle_meta"
      APPEND
      PROPERTY HARMONIUS_BUNDLE_RESOURCES "${_res}")
  endif()
endfunction()

# ---------------------------------------------------------------------------
# harmonius_bundle_add_metallibs
#
# Append harmonius_add_metallib (or harmonius_add_module _metal) targets whose
# .metallib files should land in the bundle Resources (macOS) or bundle root
# (iOS). Transitive HARMONIUS_METALLIB_DEPS are collected automatically.
# ---------------------------------------------------------------------------
function(harmonius_bundle_add_metallibs _target)
  if(ARGN)
    set_property(
      TARGET "${_target}_bundle_meta"
      APPEND
      PROPERTY HARMONIUS_BUNDLE_METALLIBS "${ARGN}")
  endif()
endfunction()

# ---------------------------------------------------------------------------
# harmonius_bundle_set_provisioning_profile
#
# iOS only: copy <path> to embedded.mobileprovision during finalize. macOS
# bundles ignore this setting.
# ---------------------------------------------------------------------------
function(harmonius_bundle_set_provisioning_profile _target _profile)
  cmake_path(IS_RELATIVE _profile _rel)
  if(_rel)
    set(_profile "${CMAKE_CURRENT_SOURCE_DIR}/${_profile}")
  endif()
  set_target_properties("${_target}_bundle_meta"
                        PROPERTIES HARMONIUS_BUNDLE_PROVISIONING "${_profile}")
endfunction()

# ---------------------------------------------------------------------------
# harmonius_bundle_finalize
#
# Generate the custom command that assembles the .app directory and create the
# ${TARGET}_bundle ALL custom target that drives the assembly. Call this after
# all harmonius_bundle_* setup functions.
# ---------------------------------------------------------------------------
function(harmonius_bundle_finalize _target)
  # Retrieve metadata
  get_target_property(_platform "${_target}_bundle_meta"
                      HARMONIUS_BUNDLE_PLATFORM)
  get_target_property(_bdir "${_target}_bundle_meta" HARMONIUS_BUNDLE_DIR)
  get_target_property(_plist "${_target}_bundle_meta" HARMONIUS_BUNDLE_PLIST)
  get_target_property(_exe_tgt "${_target}_bundle_meta"
                      HARMONIUS_BUNDLE_EXE_TARGET)
  get_target_property(_ml_tgts "${_target}_bundle_meta"
                      HARMONIUS_BUNDLE_METALLIBS)
  get_target_property(_resources "${_target}_bundle_meta"
                      HARMONIUS_BUNDLE_RESOURCES)
  get_target_property(_provisioning "${_target}_bundle_meta"
                      HARMONIUS_BUNDLE_PROVISIONING)

  if(NOT _exe_tgt)
    message(
      FATAL_ERROR "harmonius_bundle_finalize(${_target}): executable not set. \
Call harmonius_bundle_set_executable first.")
  endif()
  if(NOT _plist)
    message(
      FATAL_ERROR "harmonius_bundle_finalize(${_target}): Info.plist not set. \
Call harmonius_bundle_set_info_plist first.")
  endif()

  # Platform-specific paths
  if(_platform STREQUAL "macos")
    set(_exe_dest "${_bdir}/Contents/MacOS/${_target}")
    set(_plist_dest "${_bdir}/Contents/Info.plist")
    set(_res_dest "${_bdir}/Contents/Resources")
    set(_exe_dir "${_bdir}/Contents/MacOS")
  else()
    # iOS / iOS simulator — flat bundle
    set(_exe_dest "${_bdir}/${_target}")
    set(_plist_dest "${_bdir}/Info.plist")
    set(_res_dest "${_bdir}")
    set(_exe_dir "${_bdir}")
  endif()

  # Collect metallib files and their source targets (for DEPENDS)
  set(_ml_files "")
  set(_ml_deps "")
  if(_ml_tgts)
    foreach(_ml_tgt IN LISTS _ml_tgts)
      _harmonius_collect_metallib_files(_ml_tgt_files "${_ml_tgt}")
      list(APPEND _ml_files ${_ml_tgt_files})
      list(APPEND _ml_deps "${_ml_tgt}")
    endforeach()
    list(REMOVE_DUPLICATES _ml_files)
  endif()

  set(_stamp "${CMAKE_BINARY_DIR}/${_target}.bundle.stamp")

  # Build command list
  set(_cmds
      COMMAND
      "${CMAKE_COMMAND}"
      -E
      rm
      -rf
      "${_bdir}"
      COMMAND
      "${CMAKE_COMMAND}"
      -E
      make_directory
      "${_exe_dir}"
      COMMAND
      "${CMAKE_COMMAND}"
      -E
      make_directory
      "${_res_dest}"
      COMMAND
      "${CMAKE_COMMAND}"
      -E
      copy
      "${_plist}"
      "${_plist_dest}"
      COMMAND
      "${CMAKE_COMMAND}"
      -E
      copy
      "$<TARGET_FILE:${_exe_tgt}>"
      "${_exe_dest}"
      COMMAND
      chmod
      755
      "${_exe_dest}")

  foreach(_ml IN LISTS _ml_files)
    cmake_path(GET _ml FILENAME _ml_name)
    list(
      APPEND
      _cmds
      COMMAND
      "${CMAKE_COMMAND}"
      -E
      copy
      "${_ml}"
      "${_res_dest}/${_ml_name}")
  endforeach()

  if(_resources)
    foreach(_r IN LISTS _resources)
      list(
        APPEND
        _cmds
        COMMAND
        "${CMAKE_COMMAND}"
        -E
        copy_if_different
        "${_r}"
        "${_res_dest}/")
    endforeach()
  endif()

  if(_provisioning AND NOT _platform STREQUAL "macos")
    list(
      APPEND
      _cmds
      COMMAND
      "${CMAKE_COMMAND}"
      -E
      copy
      "${_provisioning}"
      "${_bdir}/embedded.mobileprovision")
  endif()

  list(
    APPEND
    _cmds
    COMMAND
    "${CMAKE_COMMAND}"
    -E
    touch
    "${_stamp}")

  add_custom_command(
    OUTPUT "${_stamp}" ${_cmds}
    DEPENDS "${_exe_tgt}" ${_ml_deps}
    COMMENT "Assembling ${_target}.app"
    VERBATIM COMMAND_EXPAND_LISTS)

  add_custom_target("${_target}_bundle" ALL DEPENDS "${_stamp}")

  set_target_properties(
    "${_target}_bundle" PROPERTIES HARMONIUS_BUNDLE_OUTPUT_DIR "${_bdir}"
                                   HARMONIUS_BUNDLE_STAMP "${_stamp}")
endfunction()

# ---------------------------------------------------------------------------
# harmonius_bundle_install
#
# Register a CMake install rule that copies the assembled .app to DESTINATION.
# Run cmake --build before cmake --install to ensure the bundle is up to date.
# ---------------------------------------------------------------------------
function(harmonius_bundle_install _target)
  cmake_parse_arguments(_BI "" "DESTINATION" "" ${ARGN})
  if(NOT _BI_DESTINATION)
    set(_BI_DESTINATION ".")
  endif()

  get_target_property(_bdir "${_target}_bundle" HARMONIUS_BUNDLE_OUTPUT_DIR)
  if(NOT _bdir)
    message(
      FATAL_ERROR
        "harmonius_bundle_install(${_target}): ${_target}_bundle not yet finalized"
    )
  endif()

  install(
    DIRECTORY "${_bdir}"
    DESTINATION "${_BI_DESTINATION}"
    USE_SOURCE_PERMISSIONS)
endfunction()
