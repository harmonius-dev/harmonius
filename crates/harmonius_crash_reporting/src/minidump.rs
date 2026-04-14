//! Minimal Breakpad-compatible minidump writer (`R-14.4.7`).

use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

/// Stream type for Harmonius metadata (`'HRM0'`).
pub const HARMONIUS_METADATA_STREAM_TYPE: u32 = 0x4852_4d30;

/// Metadata embedded as JSON inside the custom stream (portable test surface).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HarmoniusMetadataStream {
    /// Build identifier string.
    pub build_id: String,
    /// Optional scene label.
    pub scene_name: Option<String>,
    /// Optional player identifier (opaque string for tests).
    pub player_id: Option<String>,
}

/// Writes a tiny minidump containing only header + Harmonius metadata stream.
pub fn write_minidump_with_metadata(path: &Path, meta: &HarmoniusMetadataStream) -> io::Result<()> {
    let payload = serde_json::to_vec(meta)?;
    let stream_dir_rva: u32 = 32;
    let meta_rva: u32 = stream_dir_rva + 12;
    let mut file = File::create(path)?;
    // MINIDUMP_HEADER (32 bytes)
    file.write_all(b"MDMP")?; // signature
    file.write_all(&4280u32.to_le_bytes())?; // version (AARD)
    file.write_all(&1u32.to_le_bytes())?; // number_of_streams
    file.write_all(&stream_dir_rva.to_le_bytes())?; // stream_directory_rva
    file.write_all(&0u32.to_le_bytes())?; // checksum
    file.write_all(&0u32.to_le_bytes())?; // time_date_stamp
    file.write_all(&0u64.to_le_bytes())?; // flags
                                          // MINIDUMP_DIRECTORY
    file.write_all(&HARMONIUS_METADATA_STREAM_TYPE.to_le_bytes())?;
    file.write_all(&(payload.len() as u32).to_le_bytes())?;
    file.write_all(&meta_rva.to_le_bytes())?;
    file.write_all(&payload)?;
    Ok(())
}

/// Returns true if `path` looks like a minidump (signature check).
#[must_use]
pub fn is_valid_minidump_file(path: &Path) -> bool {
    let Ok(bytes) = std::fs::read(path) else {
        return false;
    };
    bytes.len() >= 4 && &bytes[0..4] == b"MDMP"
}

/// Reads Harmonius metadata JSON from a minidump written by this crate.
pub fn read_harmonius_metadata(path: &Path) -> io::Result<HarmoniusMetadataStream> {
    let bytes = std::fs::read(path)?;
    if bytes.len() < 44 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "minidump too small",
        ));
    }
    let count = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
    if count != 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "unexpected stream count",
        ));
    }
    let stream_dir_rva = u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize;
    let dir = &bytes[stream_dir_rva..stream_dir_rva + 12];
    let stream_type = u32::from_le_bytes(dir[0..4].try_into().unwrap());
    if stream_type != HARMONIUS_METADATA_STREAM_TYPE {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "unexpected stream type",
        ));
    }
    let size = u32::from_le_bytes(dir[4..8].try_into().unwrap()) as usize;
    let rva = u32::from_le_bytes(dir[8..12].try_into().unwrap()) as usize;
    let slice = bytes
        .get(rva..rva + size)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "metadata slice OOB"))?;
    serde_json::from_slice(slice).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}
