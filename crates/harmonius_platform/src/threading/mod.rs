//! Work-stealing thread pool, task graphs, job-system helpers, and I/O wiring.
//!
//! This module mirrors the public API described in `docs/design/platform/threading.md`.

mod graph;
mod io;
mod job_system;
mod pool;
mod topology;

#[cfg(test)]
mod acceptance_tests;

pub mod compio;

pub use graph::{TaskGraph, TaskGraphBuilder, TaskGraphError, TaskNodeId, TaskPriority};
pub use io::{BufferPool, BufferSlot, IoError, IoRequest, IoRequestId, IoResult, PlatformIo};
pub use job_system::par_iter;
pub use pool::{JobHandle, JoinHandle, ThreadPool, ThreadPoolConfig, ThreadPriority};
pub use topology::{CoreId, CoreTopology};
