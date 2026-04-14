//! Physical cine lens parameters and derived quantities (`R-2.7.2`).

/// Physical cinema lens parameters in millimeters, f-stops, seconds, and ISO.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CineLens {
    /// Focal length in millimeters.
    pub focal_length_mm: f32,
    /// Aperture as an f-number (e.g. `2.8` for f/2.8).
    pub aperture_fstop: f32,
    /// Sensor width in millimeters (full-frame horizontal = 36.0 when using 35 mm stills width).
    pub sensor_width_mm: f32,
    /// Shutter interval in **seconds** (e.g. `1.0 / 100.0` for 1/100 s).
    pub shutter_speed_s: f32,
    /// ISO sensitivity.
    pub iso: f32,
}

impl CineLens {
    /// Vertical field of view in degrees derived from the sensor width and focal length.
    ///
    /// This matches the reference design snippet:
    /// `2.0 * ((sensor / (2.0 * focal)).atan()).to_degrees()`.
    #[must_use]
    pub fn to_vfov_deg(&self) -> f32 {
        let sensor = self.sensor_width_mm;
        let focal = self.focal_length_mm;
        2.0 * (sensor / (2.0 * focal)).atan().to_degrees()
    }

    /// EV100 exposure value using the design formula:
    /// `log2(N^2 / t * (100 / ISO))`.
    #[must_use]
    pub fn ev100(&self) -> f32 {
        let n = self.aperture_fstop;
        let t = self.shutter_speed_s;
        let iso = self.iso;
        ((n * n) / t * (100.0 / iso)).log2()
    }
}

#[cfg(test)]
mod tests {
    use super::CineLens;

    /// TC-2.7.2.1 — 50 mm lens, 35 mm sensor horizontal reference.
    #[test]
    fn test_cine_lens_vfov_50mm_35mm_sensor() {
        let lens = CineLens {
            focal_length_mm: 50.0,
            aperture_fstop: 2.0,
            sensor_width_mm: 35.0,
            shutter_speed_s: 1.0 / 125.0,
            iso: 100.0,
        };
        let vfov = lens.to_vfov_deg();
        assert!((vfov - 38.58).abs() < 0.05, "expected ~38.58°, got {vfov}");
    }

    /// TC-2.7.2.2 — 24 mm lens, 35 mm sensor.
    #[test]
    fn test_cine_lens_vfov_24mm_35mm_sensor() {
        let lens = CineLens {
            focal_length_mm: 24.0,
            aperture_fstop: 2.0,
            sensor_width_mm: 35.0,
            shutter_speed_s: 1.0 / 125.0,
            iso: 100.0,
        };
        let vfov = lens.to_vfov_deg();
        assert!((vfov - 72.20).abs() < 0.05, "expected ~72.20°, got {vfov}");
    }

    /// TC-2.7.2.3 — f/16, 1/100 s, ISO 100 (golden value from the design formula).
    #[test]
    fn test_cine_lens_ev100_sunny_16() {
        let lens = CineLens {
            focal_length_mm: 50.0,
            aperture_fstop: 16.0,
            sensor_width_mm: 35.0,
            shutter_speed_s: 1.0 / 100.0,
            iso: 100.0,
        };
        let ev = lens.ev100();
        assert!((ev - 14.643856).abs() < 0.1, "expected ~14.64 EV, got {ev}");
    }

    /// TC-2.7.2.4 — f/2.8, 1/60 s, ISO 400.
    #[test]
    fn test_cine_lens_ev100_indoor() {
        let lens = CineLens {
            focal_length_mm: 35.0,
            aperture_fstop: 2.8,
            sensor_width_mm: 36.0,
            shutter_speed_s: 1.0 / 60.0,
            iso: 400.0,
        };
        let ev = lens.ev100();
        assert!((ev - 6.877744).abs() < 0.1, "expected ~6.88 EV, got {ev}");
    }
}
