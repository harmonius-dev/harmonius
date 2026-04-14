//! Headless-friendly GPU profiler data structures used by the render pipeline and profiler UI.
#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod draw_list;
mod drop_queue;
pub mod frame_collector;
pub mod gpu_profiler;
pub mod timebase;

pub use draw_list::{DrawCommand, DrawList, DrawListStats, RenderPhase};
pub use drop_queue::{GpuTimestampQueue, GpuTimestampSender};
pub use frame_collector::FrameCollector;
pub mod fake_gpu {
    //! Deterministic headless backend surface used by CI integration tests.

    pub use crate::gpu_profiler::{
        CommandBuffer, GpuProfilerState, GpuScope, ProfilingQueries, QueryPool,
    };
}

pub use gpu_profiler::{
    assemble_gpu_frame_stats, CommandBuffer, GpuAllocatorStats, GpuFrameCapture, GpuFrameStats,
    GpuPassTiming, GpuProfilerState, GpuScope, ProfilingQueries, QueryPool, ResolvedTimestamps,
};
pub use timebase::gpu_ticks_to_ms;
