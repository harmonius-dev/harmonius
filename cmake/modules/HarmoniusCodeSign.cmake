# HarmoniusCodeSign.cmake Ad-hoc or identity-based code signing for an assembled
# .app bundle.
#
# Signing identity is controlled (in priority order) by: 1. The IDENTITY
# argument passed to harmonius_codesign_bundle 2. The CMake cache variable
# HARMONIUS_CODESIGN_IDENTITY 3. Default: "-" (ad-hoc, suitable for local dev
# and CI without certs)
#
# Public API: harmonius_codesign_bundle(<TARGET> [IDENTITY <id>] [ENTITLEMENTS
# <path>])

cmake_minimum_required(VERSION 4.0)

set(HARMONIUS_CODESIGN_IDENTITY
    "-"
    CACHE STRING "Code signing identity for harmonius_codesign_bundle. \
Use '-' for ad-hoc signing or a Developer ID / team name for real signing.")

set(HARMONIUS_CODESIGN_ENTITLEMENTS
    ""
    CACHE FILEPATH "Optional entitlements plist passed to codesign \
--entitlements. Ignored for ad-hoc ('-') signing.")

# ---------------------------------------------------------------------------
# harmonius_codesign_bundle
#
# Create a ${TARGET}_signed ALL custom target that runs codesign on the
# assembled bundle. Depends on ${TARGET}_bundle being finalized first.
# ---------------------------------------------------------------------------
function(harmonius_codesign_bundle _target)
  cmake_parse_arguments(_CS "" "IDENTITY;ENTITLEMENTS" "" ${ARGN})
  if(NOT _CS_IDENTITY)
    set(_CS_IDENTITY "${HARMONIUS_CODESIGN_IDENTITY}")
  endif()
  if(NOT _CS_ENTITLEMENTS)
    set(_CS_ENTITLEMENTS "${HARMONIUS_CODESIGN_ENTITLEMENTS}")
  endif()

  get_target_property(_bdir "${_target}_bundle" HARMONIUS_BUNDLE_OUTPUT_DIR)
  get_target_property(_stamp "${_target}_bundle" HARMONIUS_BUNDLE_STAMP)

  if(NOT _bdir OR NOT _stamp)
    message(
      FATAL_ERROR
        "harmonius_codesign_bundle(${_target}): ${_target}_bundle must be \
finalized (call harmonius_bundle_finalize) before codesigning.")
  endif()

  set(_signed_stamp "${CMAKE_BINARY_DIR}/${_target}.signed.stamp")

  set(_entitlement_args "")
  if(_CS_ENTITLEMENTS AND NOT _CS_IDENTITY STREQUAL "-")
    set(_entitlement_args --entitlements "${_CS_ENTITLEMENTS}")
  endif()

  add_custom_command(
    OUTPUT "${_signed_stamp}"
    COMMAND codesign --force --deep --sign "${_CS_IDENTITY}"
            ${_entitlement_args} "${_bdir}"
    COMMAND "${CMAKE_COMMAND}" -E touch "${_signed_stamp}"
    DEPENDS "${_stamp}"
    COMMENT "Signing ${_target}.app (identity: ${_CS_IDENTITY})"
    VERBATIM)

  add_custom_target("${_target}_signed" ALL DEPENDS "${_signed_stamp}")
  add_dependencies("${_target}_signed" "${_target}_bundle")
  set_target_properties("${_target}_signed" PROPERTIES HARMONIUS_BUNDLE_STAMP
                                                       "${_signed_stamp}")
endfunction()
