//! Helpers for mixed text + companion layouts (F-1.4.8 / F-1.4.10).

/// Where a logical field is persisted in mixed-format saves.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FieldStorage {
    /// Inline in the text section.
    InlineText,
    /// Stored in the binary companion, optionally LZ4-compressed.
    Companion {
        /// When true, companion bytes are LZ4-compressed.
        compress_lz4: bool,
    },
}

/// Renders a minimal mixed-format manifest line consumed by tooling tests.
///
/// Human-readable fields stay in `inline_text`; companion-backed fields are
/// listed by name on an `@companion` directive.
pub fn render_mixed_manifest(inline_text: &str, companion_field_names: &[&str]) -> String {
    let names = companion_field_names.join(",");
    format!("{inline_text}\n@companion {names}\n")
}

/// Parses [`render_mixed_manifest`] output into inline body and companion names.
pub fn parse_mixed_manifest(manifest: &str) -> (String, Vec<String>) {
    let mut inline = String::new();
    let mut companion = Vec::new();
    for line in manifest.lines() {
        if let Some(rest) = line.strip_prefix("@companion ") {
            if !rest.is_empty() {
                companion.extend(rest.split(',').map(|s| s.trim().to_string()));
            }
        } else if !line.is_empty() {
            if !inline.is_empty() {
                inline.push('\n');
            }
            inline.push_str(line);
        }
    }
    (inline, companion)
}
