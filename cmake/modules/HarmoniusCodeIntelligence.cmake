# HarmoniusCodeIntelligence.cmake Editor metadata helpers.

cmake_minimum_required(VERSION 4.0)

function(_harmonius_json_escape _result _value)
  string(REPLACE "\\" "\\\\" _escaped "${_value}")
  string(REPLACE "\"" "\\\"" _escaped "${_escaped}")
  set(${_result}
      "${_escaped}"
      PARENT_SCOPE)
endfunction()

function(harmonius_generate_clangd_database)
  cmake_parse_arguments(
    _C
    ""
    "OUTPUT_DIR"
    "SOURCES;INCLUDE_DIRS"
    ${ARGN})
  if(NOT _C_OUTPUT_DIR)
    message(FATAL_ERROR "harmonius_generate_clangd_database: OUTPUT_DIR is required")
  endif()

  file(MAKE_DIRECTORY "${_C_OUTPUT_DIR}")

  set(_command "${CMAKE_CXX_COMPILER} -x c++ -std=c++26")
  foreach(_dir IN LISTS _C_INCLUDE_DIRS)
    cmake_path(IS_RELATIVE _dir _rel)
    if(_rel)
      set(_dir "${CMAKE_SOURCE_DIR}/${_dir}")
    endif()
    string(APPEND _command " -I${_dir}")
  endforeach()

  set(_json "[\n")
  set(_first TRUE)
  foreach(_file IN LISTS _C_SOURCES)
    cmake_path(IS_RELATIVE _file _rel)
    if(_rel)
      set(_file "${CMAKE_SOURCE_DIR}/${_file}")
    endif()

    if(_first)
      set(_first FALSE)
    else()
      string(APPEND _json ",\n")
    endif()

    _harmonius_json_escape(_command_json "${_command} ${_file}")
    _harmonius_json_escape(_directory_json "${CMAKE_SOURCE_DIR}")
    _harmonius_json_escape(_file_json "${_file}")
    string(APPEND _json "  {\n")
    string(APPEND _json "    \"command\": \"${_command_json}\",\n")
    string(APPEND _json "    \"directory\": \"${_directory_json}\",\n")
    string(APPEND _json "    \"file\": \"${_file_json}\"\n")
    string(APPEND _json "  }")
  endforeach()
  string(APPEND _json "\n]\n")

  file(WRITE "${_C_OUTPUT_DIR}/compile_commands.json" "${_json}")
endfunction()
