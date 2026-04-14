use std::fmt;

/// PCI-style identifiers for a GPU.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceId {
    /// PCI vendor id.
    pub vendor: u32,
    /// PCI device id.
    pub device: u32,
}

/// Stable fingerprint used for per-device cache directories.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DeviceFingerprint {
    /// Vendor id from [`DeviceId::vendor`].
    pub vendor_id: u32,
    /// Device id from [`DeviceId::device`].
    pub device_id: u32,
    /// Driver version token (opaque u64 in this stub; OS layers map semver).
    pub driver_version: u64,
    /// Graphics API version token.
    pub api_version: u32,
}

impl DeviceFingerprint {
    /// Builds a filesystem-safe directory name for the fingerprint.
    #[must_use]
    pub fn directory_name(&self) -> String {
        let (maj, min, pat) = split_version(self.driver_version);
        format!(
            "vendor_{:04x}_dev_{:04x}_api_{:08x}_drv_{}_{}_{}",
            self.vendor_id, self.device_id, self.api_version, maj, min, pat
        )
    }
}

fn split_version(packed: u64) -> (u32, u32, u32) {
    let maj = ((packed >> 40) & 0xFF_FFFF) as u32;
    let min = ((packed >> 20) & 0xFF_FFFF) as u32;
    let pat = (packed & 0xFF_FFFF) as u32;
    (maj, min, pat)
}

/// Minimal GPU descriptor used when opening a [`crate::PsoCache`](super::PsoCache).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpuDevice {
    /// Device ids.
    pub ids: DeviceId,
    /// Driver version token.
    pub driver_version: u64,
    /// API version token.
    pub api_version: u32,
}

impl GpuDevice {
    /// Derives a fingerprint for cache partitioning.
    #[must_use]
    pub fn fingerprint(&self) -> DeviceFingerprint {
        DeviceFingerprint {
            vendor_id: self.ids.vendor,
            device_id: self.ids.device,
            driver_version: self.driver_version,
            api_version: self.api_version,
        }
    }
}

impl fmt::Display for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04x}:{:04x}", self.vendor, self.device)
    }
}
