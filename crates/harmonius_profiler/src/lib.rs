//! Harmonius profiler: lock-free per-thread event capture and frame aggregation.
//!
//! Implements [`ProfileRingBuffer`], [`FrameCollector`], and companion types from
//! `docs/design/tools/profiler.md`.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]

mod cpu_timeline;
mod ecs_tracker;
mod frame_collector;
mod gpu_vendor;
mod leak;
mod memory;
mod network;
mod overlays;
mod remote;
mod ring_buffer;
mod types;

pub use cpu_timeline::{CpuProfileViewMode, CpuTimeline, TimelineFilter};
pub use ecs_tracker::{EcsSystemTracker, SystemTiming};
pub use frame_collector::{CpuScopeRecorder, FrameCapture, FrameCollector, QueryPool};
pub use gpu_vendor::{VendorCounterData, VendorCounters};
pub use leak::LeakDetector;
pub use memory::{
    Allocation, LeakGroup, LeakReport, MemAllocTracker, MemorySnapshot, StackCapture,
};
pub use network::{
    ChannelBandwidth, DecodedPacket, NetBandwidthTracker, NetDirection, PacketField,
    PacketFilter, PacketInspector,
};
pub use overlays::{OverlayConfig, OverlayPosition, StatOverlay, StatOverlays};
pub use remote::{
    BinaryProtocol, CaptureGranularity, DecodeError, RemoteClient, RemoteError, RemoteServer,
};
pub use ring_buffer::{CpuEvent, FrameArena, ProfileRingBuffer};
pub use types::{FrameStats, GpuPassTiming};
