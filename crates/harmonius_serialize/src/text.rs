//! Deterministic git-friendly text codec (RF-3; RON replaced).

use std::collections::BTreeMap;

use crate::error::{DeserializeError, SerializeError};
use crate::migration::{MigrationRegistry, MigrationValue, SchemaVersion};

/// Incremental writer for the Harmonius text serialization format.
#[derive(Debug, Default)]
pub struct TextWriter {
    lines: Vec<String>,
}

impl TextWriter {
    /// Starts an empty document.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records schema version and logical type name (first lines, deterministic order).
    pub fn begin(&mut self, type_name: &str, version: SchemaVersion) {
        self.lines.push(format!(
            "@schema {}.{}.{}",
            version.major, version.minor, version.patch
        ));
        self.lines.push(format!("@type {type_name}"));
    }

    /// Appends a scalar field `key=value` with plain escaping for newlines.
    pub fn field(&mut self, key: &str, value: &str) -> Result<(), SerializeError> {
        if key.contains('\n') || key.contains('=') {
            return Err(SerializeError::BinaryPayload(format!(
                "invalid text field key: {key:?}"
            )));
        }
        let escaped = value.replace('\n', "\\n");
        self.lines.push(format!("{key}={escaped}"));
        Ok(())
    }

    /// Final string, one component block per line group (stable order preserved).
    pub fn finish(self) -> String {
        self.lines.join("\n")
    }
}

/// Pull parser for [`TextWriter`] output.
#[derive(Debug)]
pub struct TextReader<'a> {
    text: &'a str,
}

impl<'a> TextReader<'a> {
    /// Wraps a full text document.
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    /// Consumes the document and returns `(type_name, schema_version, fields)`.
    pub fn into_map(
        self,
    ) -> Result<(String, SchemaVersion, BTreeMap<String, String>), DeserializeError> {
        let mut type_name: Option<String> = None;
        let mut schema: Option<SchemaVersion> = None;
        let mut fields = BTreeMap::new();
        for (idx, line) in self.text.lines().enumerate() {
            let line_no = idx + 1;
            let line = line.trim_end();
            if line.is_empty() {
                continue;
            }
            if let Some(rest) = line.strip_prefix("@schema ") {
                let mut p = rest.split('.');
                let major: u16 = p.next().and_then(|s| s.parse().ok()).ok_or_else(|| {
                    DeserializeError::TextParse {
                        line: line_no,
                        message: "bad @schema".into(),
                    }
                })?;
                let minor: u16 = p.next().and_then(|s| s.parse().ok()).ok_or_else(|| {
                    DeserializeError::TextParse {
                        line: line_no,
                        message: "bad @schema minor".into(),
                    }
                })?;
                let patch: u16 = p.next().and_then(|s| s.parse().ok()).ok_or_else(|| {
                    DeserializeError::TextParse {
                        line: line_no,
                        message: "bad @schema patch".into(),
                    }
                })?;
                schema = Some(SchemaVersion::new(major, minor, patch));
            } else if let Some(rest) = line.strip_prefix("@type ") {
                type_name = Some(rest.trim().to_string());
            } else if let Some((k, v)) = line.split_once('=') {
                let v = v.replace("\\n", "\n");
                fields.insert(k.to_string(), v);
            } else {
                return Err(DeserializeError::TextParse {
                    line: line_no,
                    message: format!("unexpected line: {line}"),
                });
            }
        }
        let t = type_name.ok_or_else(|| DeserializeError::TextParse {
            line: 0,
            message: "missing @type".into(),
        })?;
        let s = schema.ok_or_else(|| DeserializeError::TextParse {
            line: 0,
            message: "missing @schema".into(),
        })?;
        Ok((t, s, fields))
    }
}

/// Types that can serialize themselves into [`TextWriter`] output.
pub trait TextSerialize {
    /// Writes this value using `out`.
    fn serialize_text(&self, out: &mut TextWriter) -> Result<(), SerializeError>;
}

/// Types that can be parsed from [`TextReader`] input.
pub trait TextDeserialize: Sized {
    /// Reads from `src` after optional migration of embedded version.
    fn deserialize_text(
        src: TextReader<'_>,
        migration: &MigrationRegistry,
    ) -> Result<Self, DeserializeError>;
}

/// Serializes a value that implements [`TextSerialize`].
pub fn serialize_text<T: TextSerialize>(value: &T) -> Result<String, SerializeError> {
    let mut w = TextWriter::new();
    value.serialize_text(&mut w)?;
    Ok(w.finish())
}

/// Deserializes a value that implements [`TextDeserialize`].
pub fn deserialize_text<T: TextDeserialize>(
    text: &str,
    migration: &MigrationRegistry,
) -> Result<T, DeserializeError> {
    T::deserialize_text(TextReader::new(text), migration)
}

/// Builds a [`MigrationValue`] map from sorted text fields (excluding `@` lines).
pub fn fields_to_migration_map(
    fields: BTreeMap<String, String>,
) -> BTreeMap<String, MigrationValue> {
    fields
        .into_iter()
        .map(|(k, v)| (k, MigrationValue::String(v)))
        .collect()
}
