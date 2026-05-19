//! Vulkan backend (sole GPU path in bootstrap).

mod context;
mod renderer;
mod surface;
mod swapchain;

pub use context::VulkanContext;
pub use renderer::VulkanRenderer;
pub use surface::create_vulkan_surface;
pub use swapchain::VulkanSwapchain;

use thiserror::Error;

/// Vulkan backend error.
#[derive(Clone, Debug, Error)]
pub enum VulkanError {
    #[error("vulkan loader: {0}")]
    Loader(String),
    #[error("vulkan: {0}")]
    Api(String),
    #[error("no suitable GPU found")]
    NoDevice,
    #[error("platform surface error: {0}")]
    Surface(String),
}
