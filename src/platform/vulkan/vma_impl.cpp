// vma_impl.cpp - VulkanMemoryAllocator implementation
// This file provides the single translation unit for VMA

#define VMA_IMPLEMENTATION
#define VMA_STATIC_VULKAN_FUNCTIONS 0
#define VMA_DYNAMIC_VULKAN_FUNCTIONS 1

#include <volk.h>
#include <vk_mem_alloc.h>
