//! Integration test harness for [`harmonius_gpu_runtime`].
//!
//! The headless `FakeGpuBackend` lives in `harmonius_gpu_runtime::fake_gpu` per the profiler ↔
//! rendering test plan.
#![forbid(unsafe_code)]

/// Type alias matching the integration test-case naming (`FakeGpuBackend`).
pub type FakeGpuBackend = harmonius_gpu_runtime::fake_gpu::QueryPool;
