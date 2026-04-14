//! Texture encoding profile selection (IR-5.2.4).

use crate::contracts::{GpuTextureFormat, TargetPlatform};

/// Selects GPU texture format for imported PNG RGBA8 data on a target platform.
#[must_use]
pub const fn gpu_texture_format_for_png_rgba8(platform: TargetPlatform) -> GpuTextureFormat {
    match platform {
        TargetPlatform::WindowsD3D12
        | TargetPlatform::LinuxVulkan
        | TargetPlatform::AndroidVulkan => GpuTextureFormat::Bc7Unorm,
        TargetPlatform::MacOsMetal | TargetPlatform::IosMetal => GpuTextureFormat::Astc4x4Unorm,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_5_2_4_u1_windows_png_rgba8_is_bc7() {
        assert_eq!(
            gpu_texture_format_for_png_rgba8(TargetPlatform::WindowsD3D12),
            GpuTextureFormat::Bc7Unorm
        );
    }

    #[test]
    fn tc_ir_5_2_4_u2_ios_png_rgba8_is_astc() {
        assert_eq!(
            gpu_texture_format_for_png_rgba8(TargetPlatform::IosMetal),
            GpuTextureFormat::Astc4x4Unorm
        );
    }

    #[test]
    fn tc_ir_5_2_4_u3_macos_png_rgba8_is_astc() {
        assert_eq!(
            gpu_texture_format_for_png_rgba8(TargetPlatform::MacOsMetal),
            GpuTextureFormat::Astc4x4Unorm
        );
    }
}
