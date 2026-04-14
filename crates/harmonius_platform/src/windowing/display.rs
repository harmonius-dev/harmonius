//! Display identifiers for multi-monitor window placement.

use crate::windowing::{PhysicalSize, Point};

/// Unique identifier for a connected display.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DisplayId(pub u32);

/// Information about a connected display.
#[derive(Clone, Debug, PartialEq)]
pub struct DisplayInfo {
    /// Unique identifier for this display.
    pub id: DisplayId,
    /// Human-readable display name.
    pub name: String,
    /// Native resolution in physical pixels.
    pub resolution: PhysicalSize,
    /// Current refresh rate in millihertz (59940 = 59.94 Hz).
    pub refresh_rate_mhz: u32,
    /// Color depth in bits per channel (8, 10, 12).
    pub color_depth: u8,
    /// Whether the display reports HDR capability.
    pub hdr_capable: bool,
    /// Top-left corner in virtual desktop space (logical).
    pub position: Point,
    /// Current DPI scale factor for this display.
    pub dpi: f64,
    /// Whether this is the primary display.
    pub primary: bool,
    /// Available refresh rates in millihertz.
    pub available_refresh_rates: Vec<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_14_1_3_1_display_id_equality() {
        assert_eq!(DisplayId(1), DisplayId(1));
        assert_ne!(DisplayId(1), DisplayId(2));
    }
}
