//! Debug symbol discovery (`R-14.4.2`).

use crate::filesystem::CanonicalPath;

use super::CrashError;

/// Recognized symbol container kinds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SymbolFormat {
    /// Windows PDB sidecar.
    Pdb,
    /// Apple dSYM bundle.
    Dsym,
    /// DWARF in executable.
    Dwarf,
}

/// Extracts build identifiers from shipping binaries.
pub struct SymbolUploader;

impl SymbolUploader {
    /// Guesses symbol format from the file name and extension.
    pub fn extract_build_id(path: &CanonicalPath) -> Result<SymbolFormat, CrashError> {
        let s = path.as_str();
        if s.ends_with(".pdb") || s.ends_with(".exe") {
            return Ok(SymbolFormat::Pdb);
        }
        if s.contains(".dSYM") || s.ends_with(".dylib") {
            return Ok(SymbolFormat::Dsym);
        }
        Ok(SymbolFormat::Dwarf)
    }
}
