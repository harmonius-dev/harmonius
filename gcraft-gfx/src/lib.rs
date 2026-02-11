//! Vulkan render graph and graphics utilities for gcraft.
//!
//! This crate provides a declarative render graph builder ([`RenderGraph`]),
//! compilation with automatic barrier insertion ([`CompiledGraph`]), device
//! and queue abstraction ([`DeviceContext`]), frame synchronization
//! ([`FrameSync`]), and transient resource pooling ([`ResourcePool`]).

pub mod api;
pub mod backend;

pub use api::*;
