include(HarmoniusModule)
include(HarmoniusTests)

# ---------------------------------------------------------------------------
# HarmoniusRendering — Metal 4 renderer, geometry helpers, pipeline support.
# Owns Triangle.slang → default.metallib (compiled via slangc, loaded via
# makeDefaultLibrary).
# ---------------------------------------------------------------------------
harmonius_add_module(
  HarmoniusRendering
  SWIFT_SOURCES
  HarmoniusRendering/Metal4TriangleRenderer.swift
  HarmoniusRendering/MetalTriangleRenderer.swift
  HarmoniusRendering/TriangleGeometry.swift
  HarmoniusApp/Metal4PipelineSupport.swift
  SLANG_SOURCES
  Shaders/Triangle.slang
  SHADER_TYPE_SOURCES
  Shaders/ShaderTypes.h
  Shaders/SwiftCompat.h
  SLANG_INCLUDE_DIRS
  Shaders
  INCLUDE_DIRS
  Shaders
  ${HARMONIUS_HLSLPP_INCLUDE_DIR}
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
# Tests — compiled and run by CMake/CTest through Swift Testing.
# ---------------------------------------------------------------------------
if(NOT CMAKE_SYSTEM_NAME STREQUAL "iOS")
  target_compile_options(HarmoniusRendering
                         PRIVATE "$<$<COMPILE_LANGUAGE:Swift>:-enable-testing>")

  harmonius_add_snapshot_testing()

  harmonius_add_swift_testing_executable(
    HarmoniusUnitTests
    SOURCES
    HarmoniusRendering/TriangleGeometryTests.swift
    LIBRARIES
    HarmoniusRendering)

  harmonius_add_swift_testing_executable(
    HarmoniusRenderTests
    SOURCES
    HarmoniusApp/HarmoniusRenderTests.swift
    INCLUDE_DIRS
    Shaders
    ${HARMONIUS_HLSLPP_INCLUDE_DIR}
    LIBRARIES
    HarmoniusRendering
    SnapshotTesting
    FRAMEWORKS
    AppKit
    CoreGraphics
    Metal
    ENVIRONMENT
    "HARMONIUS_METALLIB_PATH=${CMAKE_CURRENT_BINARY_DIR}/metallibs/default.metallib")
  add_dependencies(HarmoniusRenderTests HarmoniusRendering_slang)
endif()
