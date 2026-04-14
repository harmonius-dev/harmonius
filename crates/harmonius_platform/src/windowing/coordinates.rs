//! Logical and physical coordinate types for DPI-aware window geometry.

/// A size in logical (DPI-independent) coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LogicalSize {
    /// Width in logical units.
    pub width: f64,
    /// Height in logical units.
    pub height: f64,
}

/// A size in physical (pixel) coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PhysicalSize {
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
}

/// A point in logical (DPI-independent) coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    /// X coordinate in logical units.
    pub x: f64,
    /// Y coordinate in logical units.
    pub y: f64,
}

/// A rectangle in logical coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    /// Origin X in logical units.
    pub x: f64,
    /// Origin Y in logical units.
    pub y: f64,
    /// Width in logical units.
    pub width: f64,
    /// Height in logical units.
    pub height: f64,
}

impl LogicalSize {
    /// Convert to physical size at the given scale factor.
    #[must_use]
    pub fn to_physical(self, scale_factor: f64) -> PhysicalSize {
        PhysicalSize {
            width: (self.width * scale_factor).round() as u32,
            height: (self.height * scale_factor).round() as u32,
        }
    }
}

impl PhysicalSize {
    /// Convert to logical size at the given scale factor.
    #[must_use]
    pub fn to_logical(self, scale_factor: f64) -> LogicalSize {
        LogicalSize {
            width: self.width as f64 / scale_factor,
            height: self.height as f64 / scale_factor,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_14_1_4_1_logical_to_physical_100() {
        let logical = LogicalSize {
            width: 1280.0,
            height: 720.0,
        };
        assert_eq!(
            logical.to_physical(1.0),
            PhysicalSize {
                width: 1280,
                height: 720,
            }
        );
    }

    #[test]
    fn tc_14_1_4_2_logical_to_physical_125() {
        let logical = LogicalSize {
            width: 1280.0,
            height: 720.0,
        };
        assert_eq!(
            logical.to_physical(1.25),
            PhysicalSize {
                width: 1600,
                height: 900,
            }
        );
    }

    #[test]
    fn tc_14_1_4_3_logical_to_physical_150() {
        let logical = LogicalSize {
            width: 1280.0,
            height: 720.0,
        };
        assert_eq!(
            logical.to_physical(1.5),
            PhysicalSize {
                width: 1920,
                height: 1080,
            }
        );
    }

    #[test]
    fn tc_14_1_4_4_logical_to_physical_200() {
        let logical = LogicalSize {
            width: 1280.0,
            height: 720.0,
        };
        assert_eq!(
            logical.to_physical(2.0),
            PhysicalSize {
                width: 2560,
                height: 1440,
            }
        );
    }
}
