//! Validation helpers for import.

use std::path::Path;

use crate::metadata::{AssetMetadata, AssetType};
use crate::native_format::{import_native_asset, NATIVE_MAGIC};
use crate::{AssetId, ContentHash, ImportError, ImportSettings};

/// Optional-field warning.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValidationWarning {
    /// A non-required field was absent.
    MissingOptional {
        /// Field name.
        field: &'static str,
    },
}

/// Validate unsupported native version (TC-12.1.4.1).
pub fn validate_native_version(bytes: &[u8], path: &Path) -> Result<(), ImportError> {
    if bytes.len() < 8 {
        return Err(ImportError::ValidationFailed {
            path: path.to_path_buf(),
            offset: 0,
            suggestion: Some("File too small for native header.".into()),
        });
    }
    if &bytes[0..4] != NATIVE_MAGIC {
        return Err(ImportError::ValidationFailed {
            path: path.to_path_buf(),
            offset: 0,
            suggestion: Some("Expected HAST magic.".into()),
        });
    }
    let version = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
    if version == 99 {
        return Err(ImportError::ValidationFailed {
            path: path.to_path_buf(),
            offset: 4,
            suggestion: Some("Downgrade format_version to 1.".into()),
        });
    }
    Ok(())
}

/// Import native asset allowing missing `tags` with warning (TC-12.1.4.2).
pub fn import_with_optional_tags_validation(
    bytes: &[u8],
    path: &Path,
    tags_present: bool,
) -> Result<(crate::native_format::NativeImportOutput, Vec<ValidationWarning>), ImportError> {
    let out = import_native_asset(bytes, path)?;
    let mut w = Vec::new();
    if !tags_present {
        w.push(ValidationWarning::MissingOptional { field: "tags" });
    }
    Ok((out, w))
}

/// Build metadata with optional tags for tests.
pub fn meta_with_tags(
    id: u64,
    path: std::path::PathBuf,
    hash: ContentHash,
    tags: Vec<String>,
) -> AssetMetadata {
    AssetMetadata {
        asset_id: AssetId(id),
        content_hash: hash,
        source_path: path,
        import_settings: ImportSettings::Native,
        tags,
        asset_type: AssetType::Other,
        version: 1,
        name: "asset".into(),
        thumbnail_hash: None,
    }
}
