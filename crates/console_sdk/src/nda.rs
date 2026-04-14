//! Source scanners for forbidden proprietary markers (public CI checks).

/// Class of forbidden marker matched in a source scan.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForbiddenMarkerKind {
    /// Marker associated with proprietary PlayStation SDK headers (tokenized public probe).
    PlayStationSdkHeaderToken,
    /// Marker associated with proprietary Xbox GDK headers (tokenized public probe).
    XboxGdkHeaderToken,
}

/// Returns stable probe substrings used by CI/tests (not real SDK header names).
#[must_use]
pub fn forbidden_substrings() -> &'static [&'static str] {
    &[
        "__HARMONIUS_PUBLIC_PROBE_PS_SDK_HEADER__",
        "__HARMONIUS_PUBLIC_PROBE_MS_GDK_HEADER__",
    ]
}

/// Scans `source` for any forbidden console SDK markers.
#[must_use]
pub fn scan_source_for_console_nda_markers(source: &str) -> Vec<ForbiddenMarkerKind> {
    let mut hits = Vec::new();
    if source.contains(forbidden_substrings()[0]) {
        hits.push(ForbiddenMarkerKind::PlayStationSdkHeaderToken);
    }
    if source.contains(forbidden_substrings()[1]) {
        hits.push(ForbiddenMarkerKind::XboxGdkHeaderToken);
    }
    hits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nda_symbol_scan_rejects_ps5_header() {
        let src = format!("fn demo() {{ let _ = \"{}\"; }}", forbidden_substrings()[0]);
        assert!(scan_source_for_console_nda_markers(&src)
            .contains(&ForbiddenMarkerKind::PlayStationSdkHeaderToken));
    }

    #[test]
    fn test_nda_symbol_scan_rejects_gdk_header() {
        let src = format!("fn demo() {{ let _ = \"{}\"; }}", forbidden_substrings()[1]);
        assert!(scan_source_for_console_nda_markers(&src)
            .contains(&ForbiddenMarkerKind::XboxGdkHeaderToken));
    }
}
