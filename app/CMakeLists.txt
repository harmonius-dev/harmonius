include(HarmoniusModule)

# ---------------------------------------------------------------------------
# HarmoniusRendering — Metal 4 renderer, geometry helpers, pipeline support.
# Owns Triangle.metal → default.metallib (loaded via makeDefaultLibrary).
# ---------------------------------------------------------------------------
harmonius_add_module(
  HarmoniusRendering
  SWIFT_SOURCES
  HarmoniusRendering/Metal4TriangleRenderer.swift
  HarmoniusRendering/TriangleGeometry.swift
  HarmoniusApp/Metal4PipelineSupport.swift
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
# HarmoniusApp executable — built by CMake, bundled by XcodeGen.
# ---------------------------------------------------------------------------
add_executable(
  HarmoniusApp HarmoniusApp/HarmoniusApp.swift HarmoniusApp/ContentView.swift
               HarmoniusApp/MetalView.swift)
target_link_libraries(HarmoniusApp PRIVATE HarmoniusRendering)
target_link_libraries(HarmoniusApp PRIVATE "-framework SwiftUI"
                                           "-framework MetalKit")
add_dependencies(HarmoniusApp HarmoniusRendering_metal)
set_target_properties(HarmoniusApp PROPERTIES Swift_MODULE_NAME HarmoniusApp
                                              OUTPUT_NAME HarmoniusApp)
harmonius_swift_cxx_apply_module(HarmoniusApp)

# ---------------------------------------------------------------------------
# Tests — built by CMake, bundled by XcodeGen.
# ---------------------------------------------------------------------------
if(NOT CMAKE_SYSTEM_NAME STREQUAL "iOS")
  if(APPLE)
    if(NOT IS_ABSOLUTE "${CMAKE_OSX_SYSROOT}")
      execute_process(
        COMMAND xcrun --sdk "${CMAKE_OSX_SYSROOT}" --show-sdk-path
        OUTPUT_VARIABLE _abs_sdk_path
        OUTPUT_STRIP_TRAILING_WHITESPACE)
    else()
      set(_abs_sdk_path "${CMAKE_OSX_SYSROOT}")
    endif()

    get_filename_component(_sdk_dir "${_abs_sdk_path}" DIRECTORY)
    get_filename_component(_dev_dir "${_sdk_dir}" DIRECTORY)
    set(_dev_frameworks_dir "${_dev_dir}/Library/Frameworks")
    set(_dev_usr_lib_dir "${_dev_dir}/usr/lib")

    execute_process(COMMAND ${CMAKE_Swift_COMPILER} -print-target-info
                    OUTPUT_VARIABLE _swift_target_info)
    string(JSON _runtime_paths GET "${_swift_target_info}" "paths"
           "runtimeLibraryPaths")
    string(JSON _first_runtime_path GET "${_runtime_paths}" 0)
    get_filename_component(_toolchain_usr_lib_swift "${_first_runtime_path}"
                           DIRECTORY)
    get_filename_component(_toolchain_usr_lib "${_toolchain_usr_lib_swift}"
                           DIRECTORY)
    get_filename_component(_toolchain_usr "${_toolchain_usr_lib}" DIRECTORY)
    set(_testing_macros_path
        "${_toolchain_usr}/lib/swift/host/plugins/testing/libTestingMacros.dylib"
    )
  endif()

  target_compile_options(HarmoniusRendering
                         PRIVATE "$<$<COMPILE_LANGUAGE:Swift>:-enable-testing>")
  add_library(
    HarmoniusUnitTests MODULE HarmoniusRendering/TriangleGeometryTests.swift
                              HarmoniusRendering/TestingEntryPoint.swift)
  set_target_properties(
    HarmoniusUnitTests
    PROPERTIES Swift_MODULE_NAME HarmoniusUnitTests
               OUTPUT_NAME HarmoniusUnitTests
               PREFIX ""
               SUFFIX "")
  target_link_libraries(
    HarmoniusUnitTests PRIVATE HarmoniusRendering "-framework Testing"
                               "${_dev_usr_lib_dir}/lib_TestingInterop.dylib")
  harmonius_swift_cxx_apply_module(HarmoniusUnitTests)
  add_dependencies(HarmoniusUnitTests HarmoniusApp)
  target_link_options(HarmoniusUnitTests PRIVATE
                      "LINKER:-bundle_loader,$<TARGET_FILE:HarmoniusApp>")

  if(APPLE)
    target_compile_options(
      HarmoniusUnitTests
      PRIVATE
        "$<$<COMPILE_LANGUAGE:Swift>:SHELL:-F ${_dev_frameworks_dir}>"
        "$<$<COMPILE_LANGUAGE:Swift>:SHELL:-load-plugin-library ${_testing_macros_path}>"
        "$<$<COMPILE_LANGUAGE:Swift>:SHELL:-enable-experimental-feature Extern>"
    )
    target_link_options(HarmoniusUnitTests PRIVATE
                        "SHELL:-F ${_dev_frameworks_dir}")
  endif()

  # UI Tests
  find_package(XCTest REQUIRED)
  add_library(HarmoniusUITests MODULE HarmoniusApp/HarmoniusRenderTests.swift
                                      HarmoniusApp/SnapshotImageTesting.swift)
  set_target_properties(
    HarmoniusUITests
    PROPERTIES Swift_MODULE_NAME HarmoniusUITests
               OUTPUT_NAME HarmoniusUITests
               PREFIX ""
               SUFFIX "")
  harmonius_swift_cxx_apply_module(HarmoniusUITests)
  target_link_libraries(
    HarmoniusUITests PRIVATE ${XCTest_LIBRARIES}
                             "${_dev_usr_lib_dir}/libXCTestSwiftSupport.dylib")
  add_dependencies(HarmoniusUITests HarmoniusApp)
  target_link_options(HarmoniusUITests PRIVATE
                      "LINKER:-bundle_loader,$<TARGET_FILE:HarmoniusApp>")

  if(APPLE)
    target_compile_options(
      HarmoniusUITests
      PRIVATE "$<$<COMPILE_LANGUAGE:Swift>:SHELL:-F ${_dev_frameworks_dir}>"
              "$<$<COMPILE_LANGUAGE:Swift>:SHELL:-I ${_dev_usr_lib_dir}>")
    target_link_options(HarmoniusUITests PRIVATE
                        "SHELL:-F ${_dev_frameworks_dir}")
  endif()
endif()
