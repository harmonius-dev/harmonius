# This source file is part of the Swift open source project
#
# Copyright (c) 2023 Apple Inc. and the Swift project authors.
# Licensed under Apache License v2.0 with Runtime Library Exception
#
# See https://swift.org/LICENSE.txt for license information

function(_swift_generate_cxx_header target header)
  if(NOT TARGET ${target})
    message(FATAL_ERROR "Target ${target} not defined.")
  endif()

  if(NOT DEFINED CMAKE_Swift_COMPILER)
    message(WARNING "Swift not enabled. Cannot generate headers for Swift files.")
    return()
  endif()

  cmake_parse_arguments(ARG "" "" "SEARCH_PATHS;MODULE_NAME" ${ARGN})

  if(NOT ARG_MODULE_NAME)
    get_target_property(target_module_name ${target} Swift_MODULE_NAME)
    set(ARG_MODULE_NAME ${target_module_name})
  endif()

  if(ARG_SEARCH_PATHS)
    list(TRANSFORM ARG_SEARCH_PATHS PREPEND "-I")
  endif()

  if(APPLE AND CMAKE_OSX_SYSROOT)
    set(SDK_FLAGS "-sdk" "${CMAKE_OSX_SYSROOT}")
  elseif(WIN32)
    set(SDK_FLAGS "-sdk" "$ENV{SDKROOT}")
  elseif(CMAKE_SYSROOT)
    set(SDK_FLAGS "-sdk" "${CMAKE_SYSROOT}")
  endif()

  cmake_path(APPEND CMAKE_CURRENT_BINARY_DIR include
    OUTPUT_VARIABLE base_path)

  cmake_path(APPEND base_path ${header}
    OUTPUT_VARIABLE header_path)

  cmake_path(APPEND CMAKE_CURRENT_BINARY_DIR "${ARG_MODULE_NAME}.emit-module.d"
    OUTPUT_VARIABLE depfile_path)

  get_target_property(_AllSources ${target} SOURCES)
  set(_SwiftSources ${_AllSources})
  add_custom_command(
    OUTPUT ${header_path}
    DEPENDS ${_SwiftSources}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    COMMAND
      ${CMAKE_Swift_COMPILER} -typecheck
      ${ARG_SEARCH_PATHS}
      ${_SwiftSources}
      ${SDK_FLAGS}
      -module-name "${ARG_MODULE_NAME}"
      -cxx-interoperability-mode=default
      -emit-clang-header-path ${header_path}
      -emit-dependencies
      DEPFILE "${depfile_path}"
    WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
    COMMENT "Generating '${header_path}'"
    COMMAND_EXPAND_LISTS)

  target_include_directories(${target} PUBLIC ${base_path})
  target_sources(${target} PRIVATE ${header_path})
endfunction()
