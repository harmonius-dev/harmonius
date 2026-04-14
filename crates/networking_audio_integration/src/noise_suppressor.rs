//! In-place noise suppression placeholder (spectral model deferred).

/// Attenuates stationary noise before Opus encoding (placeholder implementation).
#[derive(Clone, Debug)]
pub struct NoiseSuppressor {
    /// Suppression level in decibels (negative values attenuate more).
    pub suppression_db: f32,
}

impl NoiseSuppressor {
    /// Creates a suppressor with the requested nominal attenuation.
    #[must_use]
    pub fn new(suppression_db: f32) -> Self {
        Self { suppression_db }
    }

    /// Applies the suppression model in-place (currently a gentle scalar gate).
    pub fn process(&mut self, pcm: &mut [f32]) {
        let factor = 10f32.powf(self.suppression_db / 20.0).clamp(0.0, 1.0);
        for s in pcm.iter_mut() {
            *s *= factor;
        }
    }
}
