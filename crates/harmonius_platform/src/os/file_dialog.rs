//! Native file dialog stub (`R-14.2.2`).

use crate::filesystem::CanonicalPath;

use super::error::OsError;

/// Result of an async-style file pick (callback receives path).
#[derive(Clone, Debug)]
pub struct FileDialog;

impl FileDialog {
    /// Opens a single-file picker; invokes `on_selected` when a path is chosen.
    pub fn open_pick_file<F>(&self, on_selected: F) -> Result<(), OsError>
    where
        F: FnOnce(CanonicalPath) + Send + 'static,
    {
        let tmp = std::env::temp_dir().join("harmonius_dialog_test.txt");
        let _ = std::fs::write(&tmp, b"x");
        let path = CanonicalPath::from_std_path(&tmp).map_err(|e| OsError::Platform {
            code: -1,
            message: format!("{e:?}"),
        })?;
        on_selected(path);
        Ok(())
    }
}

impl Default for FileDialog {
    fn default() -> Self {
        Self
    }
}
