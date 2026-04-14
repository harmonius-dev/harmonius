//! Shared value types for localization resolution and UI shaping.

/// Stable string table key.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LocalizedStringId(pub u32);

/// BCP-47-ish locale tag stored in a fixed-width buffer.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LocaleId(pub [u8; 16]);

impl LocaleId {
    /// Builds a locale id from a short ASCII tag (<= 16 bytes), right-padded with NUL.
    #[must_use]
    pub fn from_ascii(tag: &str) -> Self {
        let mut buf = [0_u8; 16];
        let b = tag.as_bytes();
        let n = b.len().min(16);
        buf[..n].copy_from_slice(&b[..n]);
        Self(buf)
    }
}

/// Resolved paragraph direction for layout.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextDirection {
    /// Left-to-right.
    Ltr,
    /// Right-to-left.
    Rtl,
    /// Resolved using the first strong directional character.
    Auto,
}

/// ISO 15924 script tag (4 ASCII letters).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScriptTag(pub [u8; 4]);

impl ScriptTag {
    /// Latin script.
    pub const LATN: Self = Self(*b"Latn");
    /// Arabic script.
    pub const ARAB: Self = Self(*b"Arab");
    /// Han script.
    pub const HANI: Self = Self(*b"Hani");
    /// Hebrew script.
    pub const HEBR: Self = Self(*b"Hebr");
}

/// Opaque font asset handle used by the fake shaper.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AssetId(pub u32);

/// Ordered font fallback list (short, bounded).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FontChain {
    /// Font ids in priority order.
    pub fonts: Vec<AssetId>,
}

impl FontChain {
    /// Creates a chain from explicit ids.
    #[must_use]
    pub fn new(fonts: Vec<AssetId>) -> Self {
        Self { fonts }
    }

    /// Sentinel empty chain (FM-3 / `.notdef` path in tests).
    #[must_use]
    pub fn empty() -> Self {
        Self { fonts: Vec::new() }
    }
}

/// Selects a deterministic fake font chain for a script tag.
#[derive(Clone, Copy, Debug, Default)]
pub struct FontChainResolver;

impl FontChainResolver {
    /// Returns the fake chain for `script` per integration test expectations.
    #[must_use]
    pub fn resolve(self, script: ScriptTag) -> FontChain {
        if script == ScriptTag::HANI {
            FontChain::new(vec![AssetId(10), AssetId(1)])
        } else if script == ScriptTag::ARAB {
            FontChain::new(vec![AssetId(20), AssetId(1)])
        } else {
            FontChain::new(vec![AssetId(1)])
        }
    }
}

/// Message template metadata stored in a locale table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MessageTemplate {
    /// ICU-ish pattern text.
    pub pattern: String,
    /// Stored text direction for the template.
    pub direction: TextDirection,
    /// Primary script classification for font selection.
    pub script: ScriptTag,
}

/// Fully resolved text ready for shaping and layout.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedText {
    /// Display string after formatting.
    pub display: String,
    /// Layout direction after `Auto` resolution.
    pub direction: TextDirection,
    /// Script tag carried through from the template.
    pub script: ScriptTag,
    /// Selected font chain for shaping.
    pub fonts: FontChain,
}

/// Format argument values supported by the tiny formatter.
#[derive(Clone, Debug, PartialEq)]
pub enum ArgValue {
    /// Signed integer.
    Int(i64),
    /// Floating point (unused in current tests).
    Float(f64),
    /// Inline string.
    Str(String),
    /// Nested localized id (unused in current tests).
    LocalizedId(LocalizedStringId),
    /// Plural branch selector.
    Plural(i64),
    /// Gender branch selector.
    Gender(Gender),
}

/// Gender selector for ICU-ish gender blocks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gender {
    /// Masculine grammatical gender.
    Male,
    /// Feminine grammatical gender.
    Female,
    /// Other / unknown.
    Other,
}

/// Diagnostic counters for fallback modes FM-*.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FallbackCounters {
    /// FM-1: used fallback locale entry.
    pub fm1: u32,
    /// FM-2: missing in active and fallback.
    pub fm2: u32,
    /// FM-3: unsupported glyph in chain.
    pub fm3: u32,
    /// FM-4: IME commit after focus lost.
    pub fm4: u32,
    /// FM-6: missing interpolation key.
    pub fm6: u32,
    /// FM-7: locale change deferred across a layout frame.
    pub fm7: u32,
}
