export module harmonius.gpu.d3d12;

import std;
import harmonius.gpu;

export namespace harmonius::gpu::d3d12
{

  class D3D12CommandBuffer
  {
  };

  class D3D12CommandPool
  {
  public:
    using CommandBufferType = D3D12CommandBuffer;
  };

  class D3D12Device
  {
  public:
    using CommandBufferType = D3D12CommandBuffer;
    using CommandPoolType = D3D12CommandPool;
  };

} // namespace harmonius::gpu::d3d12
