//! Core runtime: memory allocators, generational containers, and synchronous I/O facades.
#![deny(clippy::all)]
// Large protocol enums mirror `docs/design/core-runtime/io.md`; keep docs there until APIs
// stabilize.
#![allow(missing_docs)]

pub mod memory;
pub mod platform_io;
pub mod primitives;

#[cfg(test)]
mod plan_memory_async_io_tests;
