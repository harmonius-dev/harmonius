//! Whittaker-style biome classification from temperature and precipitation.

/// Coarse biome labels used in acceptance tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BiomeClass {
    /// Hot + wet.
    Tropical,
    /// Cold + dry.
    Tundra,
    /// Fallback bucket.
    Other,
}

/// Classifies `(temperature, precipitation)` into a biome label (normalized `0..1` inputs).
pub fn classify_whittaker(temp: f32, precip: f32) -> BiomeClass {
    if temp > 0.7 && precip > 0.7 {
        BiomeClass::Tropical
    } else if temp < 0.3 && precip < 0.3 {
        BiomeClass::Tundra
    } else {
        BiomeClass::Other
    }
}
