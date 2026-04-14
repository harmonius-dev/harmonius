//! Small identifier and handle types shared across the crate.

/// Opaque asset identifier placeholder until the asset pipeline lands.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AssetId(pub u64);

/// Atlas binding for resolved glyphs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GlyphAtlasId(pub u32);

/// Locale-aware virtual keycode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Keycode(pub u32);

/// Touch region identifier for authored touch bindings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TouchRegionId(pub u32);

/// Identifier for an authored combo graph (`InputSource::ComboTree`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComboTreeId(pub u32);
