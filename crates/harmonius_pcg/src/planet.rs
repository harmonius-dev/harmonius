//! Planet classification helpers (frost line heuristic).

/// Planet categories used in acceptance tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanetKind {
    /// Rocky / terrestrial.
    Rocky,
    /// Gas giant.
    GasGiant,
}

/// Classifies a planet given orbital distance vs frost line distance (same units).
pub fn classify_by_frost_line(orbital_distance_au: f64, frost_line_au: f64) -> PlanetKind {
    if orbital_distance_au < frost_line_au {
        PlanetKind::Rocky
    } else {
        PlanetKind::GasGiant
    }
}
