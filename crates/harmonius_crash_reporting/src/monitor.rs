//! Monitor-side fault notification decoding (`R-14.4.7`).

use std::io::{self, Read};
use std::path::Path;

use crate::minidump::{write_minidump_with_metadata, HarmoniusMetadataStream};

/// Arguments passed to the monitor binary.
///
/// The full monitor implementation wires these fields into platform attach + dump writers.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub struct MonitorArgs {
    /// Game process id that faulted.
    pub game_pid: u32,
    /// Dump output directory.
    pub dump_dir: std::path::PathBuf,
    /// Shared state file path (memory map).
    pub shared_state_path: std::path::PathBuf,
}

/// Compact fault notification record sent over the monitor pipe.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FaultNotification {
    /// Faulting pid.
    pub pid: u32,
    /// Opaque fault code (platform-defined).
    pub code: u32,
}

impl FaultNotification {
    /// Encodes into 8 bytes (little-endian).
    #[must_use]
    pub fn encode(self) -> [u8; 8] {
        let mut out = [0u8; 8];
        out[0..4].copy_from_slice(&self.pid.to_le_bytes());
        out[4..8].copy_from_slice(&self.code.to_le_bytes());
        out
    }

    /// Decodes from 8 bytes (little-endian).
    pub fn decode(bytes: &[u8; 8]) -> Self {
        Self {
            pid: u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            code: u32::from_le_bytes(bytes[4..8].try_into().unwrap()),
        }
    }
}

/// Reads one fault notification from `reader` (blocking up to EOF).
pub fn read_fault_notification<R: Read>(mut reader: R) -> io::Result<FaultNotification> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    Ok(FaultNotification::decode(&buf))
}

/// Reads a [`FaultNotification`] then writes a minidump containing `meta`.
pub fn monitor_write_minidump_from_stdin(
    mut stdin: impl Read,
    dump_path: &Path,
    meta: &HarmoniusMetadataStream,
) -> io::Result<FaultNotification> {
    let note = read_fault_notification(&mut stdin)?;
    write_minidump_with_metadata(dump_path, meta)?;
    Ok(note)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    use tempfile::tempdir;

    use crate::minidump::{is_valid_minidump_file, read_harmonius_metadata};

    #[test]
    fn test_monitor_reads_notification() {
        let note = FaultNotification {
            pid: 4242,
            code: 11,
        };
        let bytes = note.encode();
        let mut cur = Cursor::new(bytes);
        let read_back = read_fault_notification(&mut cur).unwrap();
        assert_eq!(read_back, note);
    }

    #[test]
    fn test_monitor_writes_minidump_file() {
        let dir = tempdir().unwrap();
        let dump = dir.path().join("crash.mdmp");
        let note = FaultNotification { pid: 9, code: 3 };
        let mut stdin = Cursor::new(note.encode());
        monitor_write_minidump_from_stdin(
            &mut stdin,
            &dump,
            &HarmoniusMetadataStream {
                build_id: "deadbeef".to_owned(),
                scene_name: None,
                player_id: None,
            },
        )
        .unwrap();
        assert!(is_valid_minidump_file(&dump));
    }

    #[test]
    fn test_monitor_attaches_metadata_stream() {
        let dir = tempdir().unwrap();
        let dump = dir.path().join("crash.mdmp");
        let note = FaultNotification { pid: 1, code: 1 };
        let mut stdin = Cursor::new(note.encode());
        monitor_write_minidump_from_stdin(
            &mut stdin,
            &dump,
            &HarmoniusMetadataStream {
                build_id: "build-a".to_owned(),
                scene_name: Some("scene".to_owned()),
                player_id: Some("player".to_owned()),
            },
        )
        .unwrap();
        let meta = read_harmonius_metadata(&dump).unwrap();
        assert_eq!(meta.build_id, "build-a");
        assert_eq!(meta.scene_name.as_deref(), Some("scene"));
        assert_eq!(meta.player_id.as_deref(), Some("player"));
    }
}
