//! Harmonius profiler: lock-free per-thread event capture and frame aggregation.
//!
//! Implements [`ProfileRingBuffer`] and related types from `docs/design/tools/profiler.md`.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]

mod ring_buffer;

pub use ring_buffer::{CpuEvent, FrameArena, ProfileRingBuffer};
