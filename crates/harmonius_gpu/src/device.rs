//! `GpuDevice` facade over the Vulkan backend.

use harmonius_platform::SurfaceHandle;

use crate::backend::vulkan::{VulkanError, VulkanRenderer};

/// GPU device facade (Vulkan-only bootstrap).
pub struct GpuDevice {
    renderer: VulkanRenderer,
}

impl GpuDevice {
    /// Create a GPU device and swapchain for `surface` with SPIR-V shaders.
    pub fn new(
        surface: SurfaceHandle,
        vert_spirv: &[u32],
        frag_spirv: &[u32],
    ) -> Result<Self, VulkanError> {
        let renderer = VulkanRenderer::new(surface, vert_spirv, frag_spirv)?;
        Ok(Self { renderer })
    }

    /// Mutable access to the triangle renderer.
    pub fn renderer_mut(&mut self) -> &mut VulkanRenderer {
        &mut self.renderer
    }

    /// Draw one frame of the triangle demo.
    pub fn draw_triangle_frame(&mut self) -> Result<(), VulkanError> {
        self.renderer.draw_frame()
    }

    /// Recreate swapchain after surface resize.
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), VulkanError> {
        self.renderer.resize(width, height)
    }
}
