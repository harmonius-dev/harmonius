//! Depth-of-field circle-of-confusion helpers (`R-2.7.6`).

/// Bokeh kernel shape for depth-of-field passes (design `BokehShape`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum BokehShape {
    /// Isotropic circular kernel.
    #[default]
    Circle,
    /// Six-sided aperture highlight.
    Hexagon,
    /// Eight-sided aperture highlight.
    Octagon,
    /// Custom kernel supplied by the render graph.
    Custom,
}

/// Physical depth-of-field parameters carried by a render camera.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DofSettings {
    /// Enables depth-of-field evaluation for this camera.
    pub enabled: bool,
    /// Focus distance in meters along the view ray.
    pub focus_distance_m: f32,
    /// Focal length in millimeters.
    pub focal_length_mm: f32,
    /// Aperture as an f-number.
    pub aperture_fstop: f32,
    /// Highlight convolution shape for the DoF pass.
    pub bokeh_shape: BokehShape,
}

/// Computes the circle-of-confusion diameter in **meters** for a thin lens model.
///
/// This follows the Harmonius design note (PBRT thin lens, Section 6.2.3) with all distances
/// expressed in meters and the aperture diameter derived from focal length and f-stop.
#[must_use]
pub fn circle_of_confusion_m(
    depth_m: f32,
    focus_distance_m: f32,
    focal_length_mm: f32,
    aperture_fstop: f32,
) -> f32 {
    const EPS: f32 = 1.0e-4;
    let f_m = focal_length_mm / 1000.0;
    let aperture_diameter_m = (focal_length_mm / aperture_fstop) / 1000.0;
    let p = focus_distance_m;
    let d = depth_m;
    if (d - p).abs() < EPS {
        return 0.0;
    }
    let denom = (d * (p - f_m)).abs().max(EPS);
    (aperture_diameter_m * f_m * (d - p).abs()) / denom
}

/// Validates that DoF settings are finite and usable by a render pass.
/// Reasons a depth-of-field pass may be skipped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DofPassError {
    /// DoF is disabled; callers should skip the pass.
    Disabled,
    /// One or more parameters were not finite (`NaN` / `inf`).
    NonFinite,
    /// Focus distance, focal length, or f-stop was non-positive.
    InvalidRange,
}

impl DofSettings {
    /// Returns `Ok(())` when a DoF pass may run without violating basic invariants.
    pub fn validate_for_pass(self) -> Result<(), DofPassError> {
        if !self.enabled {
            return Err(DofPassError::Disabled);
        }
        if !self.focus_distance_m.is_finite()
            || !self.focal_length_mm.is_finite()
            || !self.aperture_fstop.is_finite()
        {
            return Err(DofPassError::NonFinite);
        }
        if self.focus_distance_m <= 0.0 || self.focal_length_mm <= 0.0 || self.aperture_fstop <= 0.0
        {
            return Err(DofPassError::InvalidRange);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::circle_of_confusion_m;

    /// TC-2.7.6.1 — CoC at the plane of focus is zero.
    #[test]
    fn test_dof_coc_at_focus() {
        let coc = circle_of_confusion_m(3.0, 3.0, 50.0, 2.0);
        assert!(coc.abs() < 1.0e-3);
    }

    /// TC-2.7.6.2 — behind focus, CoC grows as depth increases away from focus.
    #[test]
    fn test_dof_coc_behind_focus() {
        let p = 2.0_f32;
        let f = 50.0_f32;
        let n = 2.0_f32;
        let d0 = 2.5_f32;
        let d1 = 4.0_f32;
        let c0 = circle_of_confusion_m(d0, p, f, n);
        let c1 = circle_of_confusion_m(d1, p, f, n);
        assert!(c1 > c0);
    }

    /// TC-2.7.6.3 — in front of focus, CoC grows as depth decreases away from focus.
    #[test]
    fn test_dof_coc_in_front_of_focus() {
        let p = 3.0_f32;
        let f = 35.0_f32;
        let n = 1.8_f32;
        let d0 = 2.8_f32;
        let d1 = 1.5_f32;
        let c0 = circle_of_confusion_m(d0, p, f, n);
        let c1 = circle_of_confusion_m(d1, p, f, n);
        assert!(c1 > c0);
    }
}
