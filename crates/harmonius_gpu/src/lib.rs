//! Vulkan-only GPU abstraction (bootstrap).

#![deny(clippy::all)]

pub mod backend;
pub mod command;
pub mod device;
pub mod shader;
pub mod swapchain;
pub mod types;

pub use backend::vulkan::{VulkanError, VulkanRenderer};
pub use device::GpuDevice;
pub use shader::ShaderModule;
pub use types::Format;
