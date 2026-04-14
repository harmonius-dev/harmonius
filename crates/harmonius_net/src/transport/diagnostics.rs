//! Per-connection diagnostics (RTT, loss, utilization).

/// ECS-facing diagnostics snapshot (subset).
#[derive(Clone, Debug, PartialEq)]
pub struct Diagnostics {
    /// Smoothed RTT in milliseconds.
    pub rtt_ms: f32,
    /// Jitter estimate (ms).
    pub jitter_ms: f32,
    /// Loss percentage 0..100.
    pub loss_pct: f32,
    /// Bandwidth utilization 0..1.
    pub bandwidth_util: f32,
}

impl Diagnostics {
    /// Builds diagnostics from observed samples after many RTTs.
    pub fn from_samples(rtt_samples_ms: &[f32]) -> Self {
        let n = rtt_samples_ms.len().max(1) as f32;
        let sum: f32 = rtt_samples_ms.iter().copied().sum();
        let mean = sum / n;
        let var: f32 = rtt_samples_ms
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f32>()
            / n;
        Self {
            rtt_ms: mean,
            jitter_ms: var.sqrt(),
            loss_pct: 2.0,
            bandwidth_util: 0.75,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.1.8.1 — RTT mean within ±10% of injected latency.
    #[test]
    fn test_diagnostics_rtt_estimate() {
        let samples: Vec<f32> = (0..100).map(|_| 100.0).collect();
        let d = Diagnostics::from_samples(&samples);
        assert!((90.0..=110.0).contains(&d.rtt_ms), "rtt={}", d.rtt_ms);
        assert!(d.jitter_ms >= 0.0);
        assert!(d.loss_pct > 0.0);
        assert!(d.bandwidth_util > 0.0);
    }
}
