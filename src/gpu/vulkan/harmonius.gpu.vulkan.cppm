export module harmonius.gpu.vulkan;

import std;
import harmonius.gpu;

export namespace harmonius::gpu::vulkan
{

  class VulkanCommandBuffer
  {
  };

  class VulkanCommandPool
  {
  public:
    using CommandBufferType = VulkanCommandBuffer;
  };

  class VulkanDevice
  {
  public:
    using CommandBufferType = VulkanCommandBuffer;
    using CommandPoolType = VulkanCommandPool;
  };

} // namespace harmonius::gpu::vulkan
