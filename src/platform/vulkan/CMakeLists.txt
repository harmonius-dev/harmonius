# Vulkan backend. Uses LunarG Vulkan SDK 1.4.341.0 with KosmicKrisp on macOS.

set(GCRAFT_VULKAN_SOURCES
    gcraft_command.cpp
    gcraft_descriptor.cpp
    gcraft_device.cpp
    gcraft_instance.cpp
    gcraft_pipeline.cpp
    gcraft_resource.cpp
    gcraft_surface.cpp
    gcraft_sync.cpp
    vma_impl.cpp)

add_library(gcraft_vulkan STATIC ${GCRAFT_VULKAN_SOURCES})
target_include_directories(gcraft_vulkan PRIVATE ${GCRAFT_INCLUDE_DIR}
                                                 ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_features(gcraft_vulkan PRIVATE cxx_std_20)
target_compile_options(gcraft_vulkan PRIVATE -fno-exceptions)

find_package(volk CONFIG REQUIRED)
find_package(VulkanMemoryAllocator CONFIG REQUIRED)

# On macOS, use KosmicKrisp from LunarG SDK
if(APPLE)
  if(NOT DEFINED ENV{VULKAN_SDK})
    message(
      FATAL_ERROR
        "VULKAN_SDK environment variable not set. Please install LunarG Vulkan SDK 1.4.341.0 or greater"
    )
  endif()

  set(VULKAN_SDK_PATH $ENV{VULKAN_SDK})
  target_include_directories(gcraft_vulkan PRIVATE ${VULKAN_SDK_PATH}/include)

  # Link against KosmicKrisp (Vulkan loader from LunarG SDK)
  find_library(
    VULKAN_LIBRARY vulkan
    PATHS ${VULKAN_SDK_PATH}/lib REQUIRED
    NO_DEFAULT_PATH)
  target_link_libraries(
    gcraft_vulkan PUBLIC ${VULKAN_LIBRARY} volk::volk volk::volk_headers
                         GPUOpen::VulkanMemoryAllocator)
else()
  # On other platforms, use standard Vulkan
  find_package(Vulkan REQUIRED)
  target_include_directories(gcraft_vulkan PRIVATE ${Vulkan_INCLUDE_DIRS})
  target_link_libraries(
    gcraft_vulkan PUBLIC ${Vulkan_LIBRARIES} volk::volk volk::volk_headers
                         GPUOpen::VulkanMemoryAllocator)
endif()

# Vulkan test executable
set(GCRAFT_VULKAN_TEST_SOURCES
    gcraft_command.cpp
    gcraft_descriptor.cpp
    gcraft_device.cpp
    gcraft_instance.cpp
    gcraft_pipeline.cpp
    gcraft_resource.cpp
    gcraft_surface.cpp
    gcraft_sync.cpp
    vma_impl.cpp
    test_vulkan.cpp)
add_executable(test_vulkan ${GCRAFT_VULKAN_TEST_SOURCES})
target_include_directories(test_vulkan PRIVATE ${GCRAFT_INCLUDE_DIR}
                                               ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_features(test_vulkan PRIVATE cxx_std_20)
target_compile_options(test_vulkan PRIVATE -fno-exceptions)

if(APPLE)
  target_include_directories(test_vulkan PRIVATE ${VULKAN_SDK_PATH}/include)
  target_link_libraries(
    test_vulkan PRIVATE ${VULKAN_LIBRARY} volk::volk volk::volk_headers
                        GPUOpen::VulkanMemoryAllocator)
else()
  target_include_directories(test_vulkan PRIVATE ${Vulkan_INCLUDE_DIRS})
  target_link_libraries(
    test_vulkan PRIVATE ${Vulkan_LIBRARIES} volk::volk volk::volk_headers
                        GPUOpen::VulkanMemoryAllocator)
endif()

add_test(NAME test-vulkan COMMAND test_vulkan)
