//! HDR color space configuration for window-backed swapchains.

/// Color space for HDR output (R-14.1.6).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSpace {
    /// Standard sRGB (SDR).
    Srgb,
    /// scRGB (linear, FP16). Used on Windows for HDR.
    ScRgb,
    /// BT.2020 with PQ transfer function (HDR10).
    Bt2020Pq,
    /// Extended linear sRGB. Used on macOS (EDR).
    ExtendedLinearSrgb,
}

/// HDR output configuration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HdrConfig {
    /// Target color space for the swapchain.
    pub color_space: ColorSpace,
    /// Peak luminance in nits reported to the compositor.
    pub peak_luminance_nits: f32,
    /// Whether HDR output is enabled.
    pub enabled: bool,
}

impl HdrConfig {
    /// Create a disabled (SDR) configuration.
    #[must_use]
    pub const fn disabled() -> Self {
        Self {
            color_space: ColorSpace::Srgb,
            peak_luminance_nits: 80.0,
            enabled: false,
        }
    }

    /// Create a platform-appropriate HDR configuration.
    #[cfg(target_os = "windows")]
    #[must_use]
    pub fn platform_default(peak_nits: f32) -> Self {
        Self {
            color_space: ColorSpace::ScRgb,
            peak_luminance_nits: peak_nits,
            enabled: true,
        }
    }

    /// Create a platform-appropriate HDR configuration.
    #[cfg(target_os = "macos")]
    #[must_use]
    pub fn platform_default(peak_nits: f32) -> Self {
        Self {
            color_space: ColorSpace::ExtendedLinearSrgb,
            peak_luminance_nits: peak_nits,
            enabled: true,
        }
    }

    /// Create a platform-appropriate HDR configuration.
    #[cfg(target_os = "linux")]
    #[must_use]
    pub fn platform_default(peak_nits: f32) -> Self {
        Self {
            color_space: ColorSpace::Bt2020Pq,
            peak_luminance_nits: peak_nits,
            enabled: true,
        }
    }

    /// Create a platform-appropriate HDR configuration for targets without a dedicated path yet.
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    #[must_use]
    pub fn platform_default(peak_nits: f32) -> Self {
        Self {
            color_space: ColorSpace::Srgb,
            peak_luminance_nits: peak_nits,
            enabled: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_14_1_6_1_hdr_config_disabled() {
        let cfg = HdrConfig::disabled();
        assert!(!cfg.enabled);
        assert_eq!(cfg.color_space, ColorSpace::Srgb);
        assert_eq!(cfg.peak_luminance_nits, 80.0);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn tc_14_1_6_2_hdr_config_platform_default_windows() {
        let cfg = HdrConfig::platform_default(1000.0);
        assert_eq!(cfg.color_space, ColorSpace::ScRgb);
        assert_eq!(cfg.peak_luminance_nits, 1000.0);
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn tc_14_1_6_3_hdr_config_platform_default_macos() {
        let cfg = HdrConfig::platform_default(1000.0);
        assert_eq!(cfg.color_space, ColorSpace::ExtendedLinearSrgb);
        assert_eq!(cfg.peak_luminance_nits, 1000.0);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn tc_14_1_6_4_hdr_config_platform_default_linux() {
        let cfg = HdrConfig::platform_default(1000.0);
        assert_eq!(cfg.color_space, ColorSpace::Bt2020Pq);
        assert_eq!(cfg.peak_luminance_nits, 1000.0);
    }
}
