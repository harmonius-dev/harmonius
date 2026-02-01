# Metal backend using Metal-cpp (Apple C++ Metal API).
# Set METAL_CPP_PATH to the directory containing Foundation/, Metal/, QuartzCore/
# from Apple's metal-cpp zip (https://developer.apple.com/metal/cpp/).
if(DEFINED ENV{METAL_CPP_PATH})
  set(METAL_CPP_DIR "$ENV{METAL_CPP_PATH}" CACHE PATH "Metal-cpp headers (Apple zip)")
else()
  include(FetchContent)
  FetchContent_Declare(metal-cpp
    GIT_REPOSITORY https://github.com/jevinskie/metal-cpp-mirror
    GIT_TAG main
    GIT_SHALLOW TRUE
  )
  FetchContent_MakeAvailable(metal-cpp)
  set(METAL_CPP_DIR "${metal-cpp_SOURCE_DIR}" CACHE PATH "Metal-cpp headers")
endif()

set(GCRAFT_METAL_SOURCES
  gcraft_metal_impl.cpp
  gcraft_internal.cpp
  gcraft_instance.cpp
  gcraft_device.cpp
  gcraft_command.cpp
  gcraft_descriptor.cpp
  gcraft_pipeline.cpp
  gcraft_resource.cpp
  gcraft_sync.cpp
)

add_library(gcraft_metal STATIC ${GCRAFT_METAL_SOURCES})
target_include_directories(gcraft_metal PRIVATE
  ${GCRAFT_INCLUDE_DIR}
  ${CMAKE_CURRENT_SOURCE_DIR}
  ${METAL_CPP_DIR}
)
target_compile_features(gcraft_metal PRIVATE cxx_std_20)
target_link_libraries(gcraft_metal PUBLIC
  "-framework Metal"
  "-framework Foundation"
  "-framework QuartzCore"
)
if(CMAKE_SYSTEM_NAME STREQUAL "Darwin")
  target_link_libraries(gcraft_metal PUBLIC "-framework AppKit")
endif()

# Metal integration test executable
add_executable(test_metal_integration gcraft_test.cpp)
target_include_directories(test_metal_integration PRIVATE
  ${GCRAFT_INCLUDE_DIR}
  ${CMAKE_CURRENT_SOURCE_DIR}
  ${METAL_CPP_DIR}
)
target_compile_features(test_metal_integration PRIVATE cxx_std_20)
target_link_libraries(test_metal_integration PRIVATE
  gcraft_metal
  "-framework Metal"
  "-framework Foundation"
  "-framework QuartzCore"
)
if(CMAKE_SYSTEM_NAME STREQUAL "Darwin")
  target_link_libraries(test_metal_integration PRIVATE "-framework AppKit")
endif()

add_test(NAME test-metal COMMAND test_metal_integration)
