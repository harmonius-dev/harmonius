//! Deterministic fake shaping used by integration tests.

use crate::types::{AssetId, FontChain};

/// One shaped glyph with its chosen font.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShapedGlyph {
    /// Source character.
    pub ch: char,
    /// Font selected for this glyph.
    pub font: AssetId,
}

/// Ordered shaped glyphs for a line.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlyphRun {
    /// Shaped glyphs in visual order.
    pub glyphs: Vec<ShapedGlyph>,
}

/// Shapes `text` using `chain` and a static `(char, font)` support map.
///
/// Returns `(run, fm3_hits)` where `fm3_hits` counts codepoints with no supported font in the
/// chain (FM-3).
#[must_use]
pub fn shape_line(text: &str, chain: &FontChain, support: &[(char, AssetId)]) -> (GlyphRun, u32) {
    let mut fm3 = 0_u32;
    let mut glyphs = Vec::new();
    'chars: for ch in text.chars() {
        for font in &chain.fonts {
            if support.iter().any(|(c, fid)| *c == ch && *fid == *font) {
                glyphs.push(ShapedGlyph { ch, font: *font });
                continue 'chars;
            }
        }
        fm3 += 1;
        glyphs.push(ShapedGlyph {
            ch,
            font: AssetId(0),
        });
    }
    (GlyphRun { glyphs }, fm3)
}
