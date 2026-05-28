include(HarmoniusModule)

# ---------------------------------------------------------------------------
# HarmoniusRendering — Metal 4 renderer, geometry helpers, pipeline support.
# Owns Triangle.slang → default.metallib (compiled via slangc, loaded via
# makeDefaultLibrary).
# ---------------------------------------------------------------------------
harmonius_add_module(
  HarmoniusRendering
  SWIFT_SOURCES
  HarmoniusRendering/Metal4TriangleRenderer.swift
  HarmoniusRendering/TriangleGeometry.swift
  HarmoniusApp/Metal4PipelineSupport.swift
  SLANG_SOURCES
  Shaders/Triangle.slang
  SLANG_INCLUDE_DIRS
  Shaders
  INCLUDE_DIRS
  Shaders
  FRAMEWORKS
  CoreGraphics
  Metal
  MetalKit
  QuartzCore)

# ---------------------------------------------------------------------------
# HarmoniusApp executable — built by CMake, bundled by XcodeGen.
# ---------------------------------------------------------------------------
add_executable(
  HarmoniusApp HarmoniusApp/HarmoniusApp.swift HarmoniusApp/ContentView.swift
               HarmoniusApp/HarmoniusLaunchOptions.swift HarmoniusApp/MetalView.swift)
target_link_libraries(HarmoniusApp PRIVATE HarmoniusRendering)
target_link_libraries(HarmoniusApp PRIVATE "-framework SwiftUI"
                                           "-framework CoreGraphics"
                                           "-framework MetalKit")
add_dependencies(HarmoniusApp HarmoniusRendering_slang)
set_target_properties(HarmoniusApp PROPERTIES Swift_MODULE_NAME HarmoniusApp
                                              OUTPUT_NAME HarmoniusApp)
harmonius_swift_cxx_apply_module(HarmoniusApp)

# ---------------------------------------------------------------------------
# Tests — compiled by Xcode; CMake only builds HarmoniusRendering for imports.
# ---------------------------------------------------------------------------
if(NOT CMAKE_SYSTEM_NAME STREQUAL "iOS")
  target_compile_options(HarmoniusRendering
                         PRIVATE "$<$<COMPILE_LANGUAGE:Swift>:-enable-testing>")
endif()
