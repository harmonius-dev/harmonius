//! Integration test harness for [`harmonius_gpu_runtime`].
//!
//! The deterministic headless `FakeQueryPool` lives in `harmonius_gpu_runtime::fake_gpu` per the
//! profiler ↔ rendering test plan.
#![forbid(unsafe_code)]

/// Headless query pool stand-in used by CI (maps to `harmonius_gpu_runtime::fake_gpu::QueryPool`).
pub type FakeQueryPool = harmonius_gpu_runtime::fake_gpu::QueryPool;
