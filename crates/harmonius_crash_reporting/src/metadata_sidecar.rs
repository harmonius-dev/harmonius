//! JSON sidecars next to dump files (`R-14.4.8`).

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::Serialize;

/// Metadata mirrored into a `.json` sidecar next to a `.mdmp`.
#[derive(Clone, Debug, Serialize)]
pub struct DumpSidecarMetadata {
    /// Build identifier string (opaque, platform-specific formatting).
    pub build_id: String,
    /// Optional scene label.
    pub scene_name: Option<String>,
}

/// Writes `<stem>.json` beside `<stem>.mdmp`.
pub fn write_sidecar_for_dump(dump_path: &Path, meta: &DumpSidecarMetadata) -> io::Result<PathBuf> {
    let stem = dump_path
        .file_stem()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "dump path has no stem"))?;
    let sidecar = dump_path.with_file_name(format!("{}.json", stem.to_string_lossy()));
    let json = serde_json::to_vec_pretty(meta)?;
    fs::write(&sidecar, json)?;
    Ok(sidecar)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    use tempfile::tempdir;

    #[test]
    fn test_sidecar_metadata_written() {
        let dir = tempdir().unwrap();
        let dump = dir.path().join("1700000000-deadbeef.mdmp");
        File::create(&dump).unwrap().write_all(b"MDMP").unwrap();
        let sidecar = write_sidecar_for_dump(
            &dump,
            &DumpSidecarMetadata {
                build_id: "deadbeef".to_owned(),
                scene_name: Some("level_1".to_owned()),
            },
        )
        .unwrap();
        let raw = fs::read_to_string(&sidecar).unwrap();
        assert!(raw.contains("deadbeef"));
        assert!(raw.contains("level_1"));
    }
}
