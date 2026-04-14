//! Build identifiers and symbol-server index keys (`R-14.4.2`).

use url::Url;

/// Native build identifier used for symbol indexing.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum BuildId {
    /// Windows PDB GUID (no separators) plus age decimal suffix.
    WindowsPdbGuidAge {
        /// PDB file name including extension (for example `game.pdb`).
        pdb_name: String,
        /// 32 lowercase hex nibbles without `{}` separators.
        guid: String,
        /// PDB age field.
        age: u32,
    },
    /// macOS LC_UUID value as 32 lowercase hex nibbles.
    MacLcUuid {
        /// Mach-O image name used for layout (for example `game`).
        binary_name: String,
        /// 32 lowercase hex nibbles.
        uuid: String,
    },
    /// Linux `.note.gnu.build-id` contents as lowercase hex.
    LinuxGnuBuildId {
        /// ELF name used for layout (for example `game`).
        binary_name: String,
        /// Variable-length lowercase hex string.
        build_id_hex: String,
    },
}

impl BuildId {
    /// Symbol-server index key for this build id.
    #[must_use]
    pub fn index_key(&self) -> String {
        match self {
            BuildId::WindowsPdbGuidAge {
                pdb_name,
                guid,
                age,
            } => format!("{pdb_name}/{guid}{age}/{pdb_name}"),
            BuildId::MacLcUuid {
                binary_name,
                uuid,
            } => format!("{binary_name}.dSYM/Contents/Resources/DWARF/{uuid}"),
            BuildId::LinuxGnuBuildId {
                binary_name,
                build_id_hex,
            } => format!("{binary_name}/{build_id_hex}"),
        }
    }
}

/// One symbol bundle entry uploaded to the symbol server.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SymbolBundleEntry {
    /// Index key returned by [`BuildId::index_key`] or a symfile-specific key.
    pub index_key: String,
    /// Raw symfile bytes.
    pub bytes: Vec<u8>,
}

/// Iterator adapter for [`upload_symbols`](crate::symbols::upload_symbols).
pub struct SymbolBundle {
    entries: Vec<SymbolBundleEntry>,
}

impl SymbolBundle {
    /// Creates a bundle from owned entries.
    #[must_use]
    pub fn new(entries: Vec<SymbolBundleEntry>) -> Self {
        Self { entries }
    }
}

impl IntoIterator for SymbolBundle {
    type IntoIter = std::vec::IntoIter<SymbolBundleEntry>;
    type Item = SymbolBundleEntry;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

/// Uploads all entries in `bundle` using `client`.
///
/// This is the design-level helper; `client` is typically a QUIC-capable transport in production.
pub fn upload_symbols(
    bundle: SymbolBundle,
    client: &mut dyn SymbolPut,
) -> Result<(), SymbolUploadError> {
    for symfile in bundle {
        client.put(&symfile.index_key, &symfile.bytes)?;
    }
    Ok(())
}

/// Minimal PUT contract for symbol uploads.
pub trait SymbolPut {
    /// Stores `bytes` at `index_key`.
    fn put(&mut self, index_key: &str, bytes: &[u8]) -> Result<(), SymbolUploadError>;
}

/// Errors surfaced while uploading symbol bundles.
#[derive(Debug, thiserror::Error)]
pub enum SymbolUploadError {
    /// Transport / IO failure.
    #[error("symbol upload failed: {0}")]
    Io(String),
}

/// Client stub used by [`upload_symbols`].
pub struct SymbolServerClient {
    server: Url,
}

impl SymbolServerClient {
    /// Creates a client targeting `server`.
    #[must_use]
    pub fn new(server: Url) -> Self {
        Self { server }
    }

    /// Returns the configured server URL.
    #[must_use]
    pub fn server(&self) -> &Url {
        &self.server
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdb_guid_age_index_key() {
        let id = BuildId::WindowsPdbGuidAge {
            pdb_name: "game.pdb".to_owned(),
            guid: "1234567890abcdef1234567890abcdef".to_owned(),
            age: 2,
        };
        assert_eq!(
            id.index_key(),
            "game.pdb/1234567890abcdef1234567890abcdef2/game.pdb"
        );
    }

    #[test]
    fn test_mac_lc_uuid_index_key() {
        let id = BuildId::MacLcUuid {
            binary_name: "game".to_owned(),
            uuid: "abcd".repeat(8),
        };
        let expected = format!(
            "game.dSYM/Contents/Resources/DWARF/{}",
            "abcd".repeat(8)
        );
        assert_eq!(id.index_key(), expected);
    }

    #[test]
    fn test_upload_symbols_smoke() {
        use crate::symbol_server::InMemorySymbolServer;

        let mut server = InMemorySymbolServer::new();
        let bundle = SymbolBundle::new(vec![SymbolBundleEntry {
            index_key: "k".to_owned(),
            bytes: vec![9, 8, 7],
        }]);
        upload_symbols(bundle, &mut server).unwrap();
        assert_eq!(server.get("k"), Some(vec![9, 8, 7]));
    }

    #[test]
    fn test_linux_build_id_index_key() {
        let id = BuildId::LinuxGnuBuildId {
            binary_name: "game".to_owned(),
            build_id_hex: "deadbeef".to_owned(),
        };
        assert_eq!(id.index_key(), "game/deadbeef");
    }
}
