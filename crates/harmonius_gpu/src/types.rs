//! GPU resource types.

/// Pixel format for swapchain images.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    /// `VK_FORMAT_B8G8R8A8_SRGB`
    B8g8r8a8Srgb,
}
