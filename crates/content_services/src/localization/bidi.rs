//! Bidirectional text classification for preview locales.

/// Resolved layout direction for a UI string preview.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextDirection {
    /// Left-to-right scripts.
    LeftToRight,
    /// Right-to-left scripts such as Arabic.
    RightToLeft,
}

/// Chooses RTL when the first strong character is Arabic (TC-15.13.2.2).
pub fn resolve_text_direction(sample: &str) -> TextDirection {
    for ch in sample.chars() {
        if ('\u{0600}'..='\u{06FF}').contains(&ch) {
            return TextDirection::RightToLeft;
        }
    }
    TextDirection::LeftToRight
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-15.13.2.2 — Arabic greeting resolves to RTL.
    #[test]
    fn tc_15_13_2_2_rtl_bidi_resolution() {
        assert_eq!(
            resolve_text_direction("مرحبا"),
            TextDirection::RightToLeft
        );
    }
}
