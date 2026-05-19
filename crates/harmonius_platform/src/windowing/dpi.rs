//! Logical and physical coordinate types.

/// A size in logical (DPI-independent) coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LogicalSize {
    pub width: f64,
    pub height: f64,
}

/// A size in physical (pixel) coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PhysicalSize {
    pub width: u32,
    pub height: u32,
}

impl LogicalSize {
    /// Convert to physical size at the given scale factor.
    #[must_use]
    pub fn to_physical(&self, scale_factor: f64) -> PhysicalSize {
        PhysicalSize {
            width: DpiScaler::logical_to_physical(self.width, scale_factor),
            height: DpiScaler::logical_to_physical(self.height, scale_factor),
        }
    }
}

impl PhysicalSize {
    /// Convert to logical size at the given scale factor.
    #[must_use]
    pub fn to_logical(&self, scale_factor: f64) -> LogicalSize {
        LogicalSize {
            width: DpiScaler::physical_to_logical(self.width, scale_factor),
            height: DpiScaler::physical_to_logical(self.height, scale_factor),
        }
    }
}

/// DPI conversion helpers.
pub struct DpiScaler;

impl DpiScaler {
    /// Convert logical units to physical pixels at `scale_factor`.
    #[must_use]
    pub fn logical_to_physical(logical: f64, scale_factor: f64) -> u32 {
        (logical * scale_factor).round() as u32
    }

    /// Convert physical pixels to logical units at `scale_factor`.
    #[must_use]
    pub fn physical_to_logical(physical: u32, scale_factor: f64) -> f64 {
        f64::from(physical) / scale_factor
    }
}

#[cfg(test)]
mod tests {
    use super::LogicalSize;

    #[test]
    fn test_logical_to_physical_150() {
        let logical = LogicalSize {
            width: 1280.0,
            height: 720.0,
        };
        let physical = logical.to_physical(1.5);
        assert_eq!(physical.width, 1920);
        assert_eq!(physical.height, 1080);
    }
}
