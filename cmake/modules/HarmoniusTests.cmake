# HarmoniusTests.cmake Swift Testing and SnapshotTesting test helpers.

cmake_minimum_required(VERSION 4.0)

include(CTest)
include(FetchContent)
include(HarmoniusSwiftCXX)

function(_harmonius_xcode_developer_framework_dir _result)
  execute_process(
    COMMAND xcode-select -p
    OUTPUT_VARIABLE _developer_dir
    OUTPUT_STRIP_TRAILING_WHITESPACE
    COMMAND_ERROR_IS_FATAL ANY)
  set(${_result}
      "${_developer_dir}/Platforms/MacOSX.platform/Developer/Library/Frameworks"
      PARENT_SCOPE)
endfunction()

function(_harmonius_xcode_developer_library_dir _result)
  execute_process(
    COMMAND xcode-select -p
    OUTPUT_VARIABLE _developer_dir
    OUTPUT_STRIP_TRAILING_WHITESPACE
    COMMAND_ERROR_IS_FATAL ANY)
  set(${_result}
      "${_developer_dir}/Platforms/MacOSX.platform/Developer/usr/lib"
      PARENT_SCOPE)
endfunction()

function(_harmonius_testing_macros_library _result)
  execute_process(
    COMMAND xcrun --toolchain XcodeDefault.xctoolchain --find swiftc
    OUTPUT_VARIABLE _swiftc
    OUTPUT_STRIP_TRAILING_WHITESPACE
    COMMAND_ERROR_IS_FATAL ANY)
  cmake_path(GET _swiftc PARENT_PATH _bin_dir)
  cmake_path(GET _bin_dir PARENT_PATH _usr_dir)
  set(_macros "${_usr_dir}/lib/swift/host/plugins/testing/libTestingMacros.dylib")
  set(${_result}
      "${_macros}"
      PARENT_SCOPE)
endfunction()

function(harmonius_add_snapshot_testing)
  if(TARGET SnapshotTesting)
    return()
  endif()

  FetchContent_Declare(
    swift_snapshot_testing
    GIT_REPOSITORY https://github.com/pointfreeco/swift-snapshot-testing.git
    GIT_TAG 1.19.2)
  FetchContent_MakeAvailable(swift_snapshot_testing)

  set(_snapshot_root "${swift_snapshot_testing_SOURCE_DIR}/Sources/SnapshotTesting")
  set(
    _snapshot_sources
    "${_snapshot_root}/AssertSnapshot.swift"
    "${_snapshot_root}/Async.swift"
    "${_snapshot_root}/Common/Internal.swift"
    "${_snapshot_root}/Common/String+SpecialCharacters.swift"
    "${_snapshot_root}/Common/XCTAttachment.swift"
    "${_snapshot_root}/Diff.swift"
    "${_snapshot_root}/Diffing.swift"
    "${_snapshot_root}/Internal/RecordIssue.swift"
    "${_snapshot_root}/SnapshotTestingConfiguration.swift"
    "${_snapshot_root}/Snapshotting.swift"
    "${_snapshot_root}/Snapshotting/NSImage.swift"
    "${_snapshot_root}/Snapshotting/UIImage.swift"
    "${_snapshot_root}/SnapshotsTestTrait.swift")

  add_library(SnapshotTesting STATIC ${_snapshot_sources})
  set_target_properties(
    SnapshotTesting
    PROPERTIES Swift_LANGUAGE_VERSION 5 Swift_MODULE_NAME SnapshotTesting)

  _harmonius_xcode_developer_framework_dir(_framework_dir)
  _harmonius_xcode_developer_library_dir(_library_dir)
  target_compile_options(
    SnapshotTesting
    PRIVATE
      "$<$<COMPILE_LANGUAGE:Swift>:SHELL:-F ${_framework_dir}>"
      "$<$<COMPILE_LANGUAGE:Swift>:SHELL:-I ${_library_dir}>")
  target_link_libraries(
    SnapshotTesting
    PRIVATE
      "-framework AppKit"
      "-framework Accelerate"
      "-framework CoreGraphics"
      "-framework CoreImage"
      "-framework MetalPerformanceShaders"
      "-framework Testing"
      "-framework XCTest")
  target_link_options(
    SnapshotTesting
    PRIVATE
      "SHELL:-F ${_framework_dir}"
      "SHELL:-L ${_library_dir}"
      "SHELL:-Xlinker -rpath -Xlinker ${_framework_dir}"
      "SHELL:-Xlinker -rpath -Xlinker ${_library_dir}")
endfunction()

function(harmonius_add_swift_testing_executable _target)
  cmake_parse_arguments(
    _T
    ""
    ""
    "SOURCES;LIBRARIES;INCLUDE_DIRS;FRAMEWORKS;ENVIRONMENT"
    ${ARGN})
  if(NOT _T_SOURCES)
    message(FATAL_ERROR "harmonius_add_swift_testing_executable: SOURCES is required")
  endif()

  set(_runner "${CMAKE_CURRENT_BINARY_DIR}/${_target}Runner.swift")
  file(
    WRITE
    "${_runner}"
    "import Darwin\n"
    "import Testing\n"
    "\n"
    "@main struct ${_target}Runner {\n"
    "  static func main() async {\n"
    "    exit(await Testing.__swiftPMEntryPoint())\n"
    "  }\n"
    "}\n")

  add_executable("${_target}" ${_T_SOURCES} "${_runner}")
  set_target_properties("${_target}" PROPERTIES Swift_MODULE_NAME "${_target}")
  harmonius_swift_cxx_apply_module("${_target}")

  _harmonius_xcode_developer_framework_dir(_framework_dir)
  _harmonius_xcode_developer_library_dir(_library_dir)
  _harmonius_testing_macros_library(_macros_library)

  target_compile_options(
    "${_target}"
    PRIVATE
      "$<$<COMPILE_LANGUAGE:Swift>:SHELL:-parse-as-library>"
      "$<$<COMPILE_LANGUAGE:Swift>:SHELL:-F ${_framework_dir}>"
      "$<$<COMPILE_LANGUAGE:Swift>:SHELL:-I ${_library_dir}>"
      "$<$<COMPILE_LANGUAGE:Swift>:SHELL:-load-plugin-library ${_macros_library}>")
  target_link_options(
    "${_target}"
    PRIVATE
      "SHELL:-F ${_framework_dir}"
      "SHELL:-L ${_library_dir}"
      "SHELL:-Xlinker -rpath -Xlinker ${_framework_dir}"
      "SHELL:-Xlinker -rpath -Xlinker ${_library_dir}")
  target_link_libraries("${_target}" PRIVATE "-framework Testing")

  foreach(_dir IN LISTS _T_INCLUDE_DIRS)
    cmake_path(IS_RELATIVE _dir _rel)
    if(_rel)
      set(_dir "${CMAKE_CURRENT_SOURCE_DIR}/${_dir}")
    endif()
    target_include_directories("${_target}" PRIVATE "${_dir}")
    target_compile_options(
      "${_target}"
      PRIVATE "$<$<COMPILE_LANGUAGE:Swift>:SHELL:-Xcc -I${_dir}>")
  endforeach()

  foreach(_framework IN LISTS _T_FRAMEWORKS)
    target_link_libraries("${_target}" PRIVATE "-framework ${_framework}")
  endforeach()

  if(_T_LIBRARIES)
    target_link_libraries("${_target}" PRIVATE ${_T_LIBRARIES})
  endif()

  add_test(NAME "${_target}" COMMAND "$<TARGET_FILE:${_target}>")
  if(_T_ENVIRONMENT)
    set_tests_properties("${_target}" PROPERTIES ENVIRONMENT "${_T_ENVIRONMENT}")
  endif()
endfunction()
