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
    /// Builds diagnostics from observed RTT samples (loss/util are spread-derived heuristics).
    pub fn from_samples(rtt_samples_ms: &[f32]) -> Self {
        let n = rtt_samples_ms.len().max(1) as f32;
        let sum: f32 = rtt_samples_ms.iter().copied().sum();
        let mean = sum / n;
        let var: f32 = rtt_samples_ms
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f32>()
            / n;
        let jitter_ms = var.sqrt();
        let (mn, mx) = rtt_samples_ms.iter().copied().fold(
            (f32::MAX, f32::MIN),
            |(a, b), x| (a.min(x), b.max(x)),
        );
        let spread = (mx - mn).max(0.0);
        let loss_pct = if rtt_samples_ms.len() >= 2 {
            ((spread / mean.max(1.0)) * 50.0).min(100.0)
        } else {
            0.0
        };
        let bandwidth_util = (mean / 200.0).min(1.0);
        Self {
            rtt_ms: mean,
            jitter_ms,
            loss_pct,
            bandwidth_util,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.1.8.1 — RTT mean within ±10% of injected latency.
    #[test]
    fn test_diagnostics_rtt_estimate() {
        let mut samples: Vec<f32> = (0..50).map(|_| 100.0).collect();
        samples.extend((0..50).map(|i| 100.0 + (i as f32) * 0.2));
        let d = Diagnostics::from_samples(&samples);
        assert!((90.0..=110.0).contains(&d.rtt_ms), "rtt={}", d.rtt_ms);
        assert!(d.jitter_ms >= 0.0);
        assert!(d.loss_pct > 0.0);
        assert!(d.bandwidth_util > 0.0);
    }
}
