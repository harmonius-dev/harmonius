//! Normalised LMS acoustic echo cancellation (software fallback path).

/// Single-channel NLMS filter for echo removal on the mic signal.
#[derive(Debug)]
pub struct AcousticEchoCanceller {
    /// Tail length in samples (determines filter order).
    pub tail_length_samples: u32,
    weights: Vec<f32>,
    x_hist: Vec<f32>,
    /// NLMS step size.
    mu: f32,
    /// Small regulariser for stability.
    eps: f32,
}

impl AcousticEchoCanceller {
    /// Allocates filter state for the given tail length.
    #[must_use]
    pub fn new(tail_length_samples: u32) -> Self {
        let n = tail_length_samples.max(1) as usize;
        Self {
            tail_length_samples,
            weights: vec![0.0f32; n],
            x_hist: vec![0.0f32; n],
            mu: 0.05,
            eps: 1e-6,
        }
    }

    /// Cancels echo in `mic` using the speaker `reference` signal.
    pub fn process(&mut self, mic: &mut [f32], reference: &[f32]) {
        let n = self.weights.len();
        for i in 0..mic.len().min(reference.len()) {
            if n > 1 {
                self.x_hist.copy_within(0..n - 1, 1);
            }
            self.x_hist[0] = reference[i];
            let y_hat: f32 = self
                .weights
                .iter()
                .zip(self.x_hist.iter())
                .map(|(w, x)| w * x)
                .sum();
            let e = mic[i] - y_hat;
            mic[i] = e;
            let norm: f32 = self.x_hist.iter().map(|x| x * x).sum::<f32>() + self.eps;
            let step = (self.mu / norm) * e;
            for (w, x) in self.weights.iter_mut().zip(self.x_hist.iter()) {
                *w += step * x;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn db_energy_ratio(clean: &[f32], reference: &[f32]) -> f32 {
        let p_sig: f32 = clean.iter().map(|x| x * x).sum();
        let p_ref: f32 = reference.iter().map(|x| x * x).sum();
        10.0 * (p_sig / (p_ref + 1e-12)).log10()
    }

    /// TC-IR-4.3.2.3 — residual energy well below reference after NLMS processing.
    #[test]
    fn tc_ir_4_3_2_3_aec_residual() {
        let len = 960usize;
        let reference: Vec<f32> = (0..len).map(|i| (i as f32 * 0.01).sin()).collect();
        let mut mic = reference.clone();
        let mut aec = AcousticEchoCanceller::new(64);
        aec.process(&mut mic, reference.as_slice());
        let db = db_energy_ratio(&mic, &reference);
        assert!(db < -6.0, "residual ratio {db} dB");
    }
}
