//! Swapchain trait and Vulkan implementation re-export.

pub use crate::backend::vulkan::VulkanSwapchain;

/// Swapchain presentation interface.
pub trait Swapchain {
    /// Acquire the next backbuffer image index.
    fn acquire_backbuffer(&mut self) -> Result<u32, String>;
    /// Present the acquired image.
    fn present(&mut self) -> Result<(), String>;
    /// Recreate swapchain images after a resize.
    fn on_resize(&mut self, width: u32, height: u32) -> Result<(), String>;
}

impl Swapchain for VulkanSwapchain {
    fn acquire_backbuffer(&mut self) -> Result<u32, String> {
        self.acquire_image().map_err(|e| e.to_string())
    }

    fn present(&mut self) -> Result<(), String> {
        Err("present requires a Vulkan queue; use VulkanRenderer::draw_frame".into())
    }

    fn on_resize(&mut self, _width: u32, _height: u32) -> Result<(), String> {
        Err("resize is handled by VulkanRenderer::resize".into())
    }
}
