include(HarmoniusModule)
include(HarmoniusApp)
include(HarmoniusCodeSign)
include(HarmoniusBundle)
include(HarmoniusIpa)

# ---------------------------------------------------------------------------
# HarmoniusRendering — Metal 4 renderer, geometry helpers, pipeline support.
# Owns Triangle.metal → default.metallib (loaded via makeDefaultLibrary).
# ---------------------------------------------------------------------------
harmonius_add_module(
  HarmoniusRendering
  SWIFT_SOURCES
  HarmoniusRendering/Metal4TriangleRenderer.swift
  HarmoniusRendering/TriangleGeometry.swift
  NativeMetal/Metal4PipelineSupport.swift
  METAL_SOURCES
  Shaders/Triangle.metal
  METAL_INCLUDE_DIRS
  Shaders
  INCLUDE_DIRS
  Shaders
  FRAMEWORKS
  Metal
  MetalKit
  QuartzCore)

# ---------------------------------------------------------------------------
# HarmoniusAppLib — SwiftUI entry point, views, and Metal view bridge. Provides
# HarmoniusApp which is started via HarmoniusApp.main() in the generated stub
# created by harmonius_add_app_bundle.
# ---------------------------------------------------------------------------
harmonius_add_module(
  HarmoniusAppLib
  SWIFT_SOURCES
  HarmoniusAppLib/HarmoniusApp.swift
  HarmoniusAppLib/ContentView.swift
  HarmoniusAppLib/MetalView.swift
  LIBRARIES
  HarmoniusRendering
  FRAMEWORKS
  SwiftUI
  MetalKit)

# ---------------------------------------------------------------------------
# App bundle — no source code; links modules and assembles Harmonius.app.
# ---------------------------------------------------------------------------
if(CMAKE_SYSTEM_NAME STREQUAL "iOS")
  set(_harmonius_platform ios)
  set(_harmonius_plist "${CMAKE_CURRENT_SOURCE_DIR}/Info-iOS.plist")
else()
  set(_harmonius_platform macos)
  set(_harmonius_plist "${CMAKE_CURRENT_SOURCE_DIR}/Info.plist")
endif()

harmonius_add_app_bundle(
  Harmonius
  PLATFORM
  ${_harmonius_platform}
  LIBRARIES
  HarmoniusAppLib
  INFO_PLIST
  "${_harmonius_plist}"
  ENTRY_MODULE
  HarmoniusAppLib
  ENTRY_POINT
  HarmoniusApp)

if(CMAKE_SYSTEM_NAME STREQUAL "iOS" AND HARMONIUS_PROVISIONING_PROFILE_FILE)
  harmonius_bundle_set_provisioning_profile(Harmonius
    "${HARMONIUS_PROVISIONING_PROFILE_FILE}")
endif()

harmonius_bundle_finalize(Harmonius)
harmonius_codesign_bundle(Harmonius)
harmonius_bundle_install(Harmonius DESTINATION .)

# iOS: also produce an .ipa
harmonius_package_ipa(Harmonius)
