//! Dump rotation helpers (`R-14.4.8`).

use std::fs;
use std::io;
use std::path::Path;
use std::time::SystemTime;

/// Removes oldest `.mdmp` files in `dir` until at most `cap` remain.
pub fn rotate_dumps(dir: &Path, cap: u32) -> io::Result<()> {
    let mut entries: Vec<_> = fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("mdmp"))
        .collect();

    entries.sort_by_key(|e| {
        e.metadata()
            .ok()
            .and_then(|m| m.modified().ok())
            .unwrap_or(SystemTime::UNIX_EPOCH)
    });

    while entries.len() > cap as usize {
        let victim = entries.remove(0);
        fs::remove_file(victim.path())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::thread;
    use std::time::Duration;

    use tempfile::tempdir;

    fn write_mdmp(dir: &Path, name: &str) {
        let path = dir.join(name);
        let mut f = File::create(&path).unwrap();
        f.write_all(b"MDMP").unwrap();
    }

    #[test]
    fn test_rotation_caps_count() {
        let dir = tempdir().unwrap();
        for i in 0..15 {
            write_mdmp(dir.path(), &format!("{i}.mdmp"));
            // Ensure distinct mtimes across platforms with coarse timestamps.
            thread::sleep(Duration::from_millis(10));
        }
        rotate_dumps(dir.path(), 10).unwrap();
        let remaining = fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("mdmp"))
            .count();
        assert_eq!(remaining, 10);
    }

    #[test]
    fn test_rotation_removes_oldest_first() {
        let dir = tempdir().unwrap();
        write_mdmp(dir.path(), "old.mdmp");
        thread::sleep(Duration::from_millis(40));
        write_mdmp(dir.path(), "new.mdmp");
        rotate_dumps(dir.path(), 1).unwrap();
        let names: Vec<_> = fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        assert!(names.iter().any(|n| n == "new.mdmp"));
        assert!(!names.iter().any(|n| n == "old.mdmp"));
    }
}
