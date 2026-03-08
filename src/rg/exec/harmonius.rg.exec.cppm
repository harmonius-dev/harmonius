export module harmonius.rg.exec;

import std;
import harmonius.gpu;
import harmonius.rg;
import harmonius.rg.compiler;

export namespace harmonius::rg::exec
{

  // ---------------------------------------------------------------------------
  // Transfer pass descriptor
  // ---------------------------------------------------------------------------

  struct TransferPassDesc
  {
    gpu::BufferHandle src_staging;
    ResourceHandle dst_resource;
    std::uint64_t src_offset = 0;
    std::uint64_t dst_offset = 0;
    std::uint64_t size = 0;
    std::uint32_t priority = 0;
  };

  // ---------------------------------------------------------------------------
  // Pass context
  // ---------------------------------------------------------------------------

  class PassContext
  {
  public:
    [[nodiscard]] auto resolve(ResourceHandle handle) const -> gpu::ResourceHandle;
    [[nodiscard]] auto allocate_constants(std::uint64_t size) -> std::pair<gpu::BufferHandle, std::uint64_t>;
    [[nodiscard]] auto frame_index() const -> std::uint32_t;
  };

  // ---------------------------------------------------------------------------
  // Command buffer pool
  // ---------------------------------------------------------------------------

  class CommandBufferPool
  {
  public:
    auto reset_frame() -> void;
    [[nodiscard]] auto allocated_count() const -> std::uint32_t;
  };

  // ---------------------------------------------------------------------------
  // Executor
  // ---------------------------------------------------------------------------

  class Executor
  {
  public:
    auto bind_resource(ResourceHandle rg_handle, gpu::ResourceHandle gpu_handle) -> void;
    auto set_resolution_scale(std::string_view name, float scale) -> void;
    auto set_pass_active(PassHandle pass, bool active) -> void;
    auto invalidate_history(ResourceHandle resource) -> void;
    auto inject_transfer(TransferPassDesc desc) -> void;
    auto set_budget_threshold(std::string_view query_name, float threshold_ms) -> void;
    auto execute(const compiler::ExecutionPlan &plan) -> void;
    [[nodiscard]] auto frame_index() const -> std::uint32_t;

  private:
    struct Impl;
    std::unique_ptr<Impl> impl_;
  };

} // namespace harmonius::rg::exec
