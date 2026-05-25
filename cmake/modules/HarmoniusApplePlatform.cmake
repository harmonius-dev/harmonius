# HarmoniusApplePlatform.cmake
#
# Configure Apple embedded-OS cross-compilation before project(). CMake's Swift
# compiler probe uses the iPhoneOS sysroot but defaults to a macOS triple unless
# CMAKE_Swift_COMPILER_TARGET is set explicitly.
#
# Include this from the root CMakeLists.txt before project().

if(CMAKE_SYSTEM_NAME STREQUAL "iOS")
  if(NOT CMAKE_OSX_ARCHITECTURES)
    set(CMAKE_OSX_ARCHITECTURES
        arm64
        CACHE STRING "Architectures for iOS builds")
  endif()

  if(NOT CMAKE_OSX_DEPLOYMENT_TARGET)
    set(CMAKE_OSX_DEPLOYMENT_TARGET
        "26.0"
        CACHE STRING "Minimum iOS deployment target")
  endif()

  if(NOT CMAKE_Swift_COMPILER_TARGET)
    if(CMAKE_OSX_SYSROOT MATCHES "simulator")
      set(_harmonius_swift_triple
          "arm64-apple-ios${CMAKE_OSX_DEPLOYMENT_TARGET}-simulator")
    else()
      set(_harmonius_swift_triple
          "arm64-apple-ios${CMAKE_OSX_DEPLOYMENT_TARGET}")
    endif()
    set(CMAKE_Swift_COMPILER_TARGET
        "${_harmonius_swift_triple}"
        CACHE STRING "Swift target triple for iOS builds")
  endif()

  if(NOT CMAKE_CXX_COMPILER_TARGET)
    set(CMAKE_CXX_COMPILER_TARGET
        "${CMAKE_Swift_COMPILER_TARGET}"
        CACHE STRING "C++ target triple for iOS builds")
  endif()

  if(NOT CMAKE_C_COMPILER_TARGET)
    set(CMAKE_C_COMPILER_TARGET
        "${CMAKE_Swift_COMPILER_TARGET}"
        CACHE STRING "C target triple for iOS builds")
  endif()
endif()
