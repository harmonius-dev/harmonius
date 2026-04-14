//! Small synchronous read helper retained for compatibility with historical design wording.

use std::path::Path;

/// Reads an entire file (used by tests that refer to the legacy `compio` naming).
///
/// Production code should prefer [`super::PlatformIo`], which keeps blocking work on the main
/// thread and delivers completions through channels.
pub fn read_file(path: &Path) -> std::io::Result<Vec<u8>> {
    std::fs::read(path)
}
