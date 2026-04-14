//! Platform profile → texture compression (IR-5.1.2).

use crate::types::{PlatformProfile, TargetPlatform};

/// Texture compression selected for a cook.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TextureCompression {
    Bc7,
    Astc,
    Etc2,
}

/// Returns the texture compression for `profile` (honours Windows → BC7, iOS → ASTC).
pub fn select_texture_format(profile: &PlatformProfile) -> TextureCompression {
    if let Some(t) = profile.texture_override {
        return match t {
            TargetPlatform::Windows | TargetPlatform::Linux => TextureCompression::Bc7,
            TargetPlatform::IOS | TargetPlatform::Android => TextureCompression::Astc,
            TargetPlatform::MacOS => TextureCompression::Astc,
            TargetPlatform::ConsoleA | TargetPlatform::ConsoleB => TextureCompression::Bc7,
        };
    }
    match profile.target {
        TargetPlatform::Windows | TargetPlatform::Linux => TextureCompression::Bc7,
        TargetPlatform::IOS | TargetPlatform::Android | TargetPlatform::MacOS => {
            TextureCompression::Astc
        }
        TargetPlatform::ConsoleA | TargetPlatform::ConsoleB => TextureCompression::Bc7,
    }
}
