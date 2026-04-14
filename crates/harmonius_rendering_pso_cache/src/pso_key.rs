use std::fmt;

use crate::content_hash::ContentHash;
use crate::device::DeviceId;

/// Serialized on-disk size for [`PsoKey`].
pub(crate) const PSO_KEY_WIRE_LEN: usize = 84;

/// Cache key for a pipeline state object (PSO).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PsoKey {
    /// Hash of shader bytecode.
    pub shader_hash: ContentHash,
    /// Hash of root signature / pipeline layout metadata.
    pub signature_hash: ContentHash,
    /// GPU device identity.
    pub device_id: DeviceId,
    /// Driver version token (must change on driver upgrade).
    pub driver_version: u64,
    /// On-disk cache format version.
    pub format_version: u32,
}

impl fmt::Debug for PsoKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PsoKey")
            .field("shader_hash", &self.shader_hash)
            .field("signature_hash", &self.signature_hash)
            .field("device_id", &self.device_id)
            .field("driver_version", &self.driver_version)
            .field("format_version", &self.format_version)
            .finish()
    }
}

/// Writes a key into a fixed-size wire buffer.
pub(crate) fn write_key_wire(key: &PsoKey, out: &mut [u8; PSO_KEY_WIRE_LEN]) {
    out[0..32].copy_from_slice(&key.shader_hash.0);
    out[32..64].copy_from_slice(&key.signature_hash.0);
    out[64..68].copy_from_slice(&key.device_id.vendor.to_le_bytes());
    out[68..72].copy_from_slice(&key.device_id.device.to_le_bytes());
    out[72..80].copy_from_slice(&key.driver_version.to_le_bytes());
    out[80..84].copy_from_slice(&key.format_version.to_le_bytes());
}

/// Parses a key from its wire representation.
pub(crate) fn read_key_wire(buf: &[u8; PSO_KEY_WIRE_LEN]) -> PsoKey {
    let mut shader = [0u8; 32];
    shader.copy_from_slice(&buf[0..32]);
    let mut signature = [0u8; 32];
    signature.copy_from_slice(&buf[32..64]);
    let vendor = u32::from_le_bytes(buf[64..68].try_into().expect("len"));
    let device = u32::from_le_bytes(buf[68..72].try_into().expect("len"));
    let driver_version = u64::from_le_bytes(buf[72..80].try_into().expect("len"));
    let format_version = u32::from_le_bytes(buf[80..84].try_into().expect("len"));
    PsoKey {
        shader_hash: ContentHash(shader),
        signature_hash: ContentHash(signature),
        device_id: DeviceId { vendor, device },
        driver_version,
        format_version,
    }
}
