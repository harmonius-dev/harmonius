use std::io;
use std::path::{Path, PathBuf};

use harmonius_crash_reporting::{CrashConfig, CrashHandler, UserId};
use tempfile::tempdir;
use url::Url;

fn monitor_bin() -> PathBuf {
    PathBuf::from(
        std::env::var("CARGO_BIN_EXE_harmonius_crashmon")
            .expect("CARGO_BIN_EXE_harmonius_crashmon"),
    )
}

fn test_config(dir: &Path) -> CrashConfig {
    CrashConfig {
        dump_dir: dir.join("dumps"),
        rotation_cap: 10,
        monitor_binary: monitor_bin(),
        upload_url: Url::parse("https://example.invalid/").unwrap(),
        anonymize: false,
        shared_state_path: dir.join("shared.bin"),
    }
}

fn read_shared_breadcrumb(path: &Path) -> io::Result<u32> {
    let bytes = std::fs::read(path)?;
    Ok(u32::from_le_bytes(bytes[4..8].try_into().unwrap()))
}

fn read_shared_scene(path: &Path) -> io::Result<String> {
    let bytes = std::fs::read(path)?;
    let len = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
    let end = 12usize.saturating_add(len).min(268);
    let slice = &bytes[12..end];
    Ok(String::from_utf8_lossy(slice).into_owned())
}

fn read_shared_player(path: &Path) -> io::Result<Option<UserId>> {
    let bytes = std::fs::read(path)?;
    let present = u32::from_le_bytes(bytes[268..272].try_into().unwrap());
    if present == 0 {
        return Ok(None);
    }
    let raw = u128::from_le_bytes(bytes[272..288].try_into().unwrap());
    Ok(Some(UserId(raw)))
}

#[test]
fn test_stub_install_spawns_monitor() {
    let dir = tempdir().unwrap();
    let h = CrashHandler::install(test_config(dir.path())).unwrap();
    assert!(h.monitor_pid() > 0);
    h.uninstall().unwrap();
}

#[test]
fn test_breadcrumb_handle_write() {
    let dir = tempdir().unwrap();
    let cfg = test_config(dir.path());
    let path = cfg.shared_state_path.clone();
    let mut h = CrashHandler::install(cfg).unwrap();
    h.breadcrumb_buffer().store(0x1122_3344);
    h.uninstall().unwrap();
    assert_eq!(read_shared_breadcrumb(&path).unwrap(), 0x1122_3344);
}

#[test]
fn test_update_scene_metadata() {
    let dir = tempdir().unwrap();
    let cfg = test_config(dir.path());
    let path = cfg.shared_state_path.clone();
    let mut h = CrashHandler::install(cfg).unwrap();
    h.update_scene("level_1").unwrap();
    h.uninstall().unwrap();
    assert_eq!(read_shared_scene(&path).unwrap(), "level_1");
}

#[test]
fn test_update_player_metadata() {
    let dir = tempdir().unwrap();
    let cfg = test_config(dir.path());
    let path = cfg.shared_state_path.clone();
    let mut h = CrashHandler::install(cfg).unwrap();
    h.update_player(UserId(99)).unwrap();
    h.uninstall().unwrap();
    assert_eq!(read_shared_player(&path).unwrap(), Some(UserId(99)));
}
