# HarmoniusIpa.cmake Package a signed .app into a .ipa for iOS distribution /
# TestFlight. Only relevant when CMAKE_SYSTEM_NAME == "iOS".
#
# The .ipa is placed at: <CMAKE_INSTALL_PREFIX>/${TARGET}.ipa
#
# Public API: harmonius_package_ipa(<TARGET>)

cmake_minimum_required(VERSION 4.0)

# ---------------------------------------------------------------------------
# harmonius_package_ipa
#
# Creates an ${TARGET}_ipa ALL custom target that packages the signed .app into
# a Payload/<TARGET>.app/... zip archive named ${TARGET}.ipa.
#
# Depends on ${TARGET}_signed (or ${TARGET}_bundle if signing was skipped).
# ---------------------------------------------------------------------------
function(harmonius_package_ipa _target)
  if(NOT CMAKE_SYSTEM_NAME STREQUAL "iOS")
    message(STATUS "harmonius_package_ipa: skipping on non-iOS platform \
(CMAKE_SYSTEM_NAME=${CMAKE_SYSTEM_NAME})")
    return()
  endif()

  # Prefer the signed stamp; fall back to the assembly stamp.
  if(TARGET "${_target}_signed")
    get_target_property(_dep_stamp "${_target}_signed" HARMONIUS_BUNDLE_STAMP)
    set(_dep_target "${_target}_signed")
  else()
    get_target_property(_dep_stamp "${_target}_bundle" HARMONIUS_BUNDLE_STAMP)
    set(_dep_target "${_target}_bundle")
  endif()

  get_target_property(_bdir "${_target}_bundle" HARMONIUS_BUNDLE_OUTPUT_DIR)

  set(_ipa_work_dir "${CMAKE_BINARY_DIR}/${_target}_ipa_work")
  set(_ipa_file "${CMAKE_BINARY_DIR}/${_target}.ipa")
  set(_ipa_stamp "${CMAKE_BINARY_DIR}/${_target}.ipa.stamp")

  add_custom_command(
    OUTPUT "${_ipa_stamp}"
    COMMAND "${CMAKE_COMMAND}" -E rm -rf "${_ipa_work_dir}"
    COMMAND "${CMAKE_COMMAND}" -E make_directory "${_ipa_work_dir}/Payload"
    COMMAND "${CMAKE_COMMAND}" -E copy_directory "${_bdir}"
            "${_ipa_work_dir}/Payload/${_target}.app"
    COMMAND "${CMAKE_COMMAND}" -E chdir "${_ipa_work_dir}" zip -r "${_ipa_file}"
            "Payload"
    COMMAND "${CMAKE_COMMAND}" -E touch "${_ipa_stamp}"
    DEPENDS "${_dep_stamp}"
    COMMENT "Packaging ${_target}.ipa"
    VERBATIM)

  add_custom_target("${_target}_ipa" ALL DEPENDS "${_ipa_stamp}")
  add_dependencies("${_target}_ipa" "${_dep_target}")

  install(FILES "${_ipa_file}" DESTINATION ".")
endfunction()
