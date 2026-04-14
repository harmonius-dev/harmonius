//! Simple alerting heuristics (`R-14.4.3`).

/// Severity for an alert evaluation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AlertSeverity {
    /// No alert.
    None,
    /// Medium severity.
    Medium,
    /// High severity.
    High,
}

/// Returns [`AlertSeverity::High`] when more than `volume_threshold` events occur in `window_ms`.
#[must_use]
pub fn new_cluster_high_volume(
    timestamps_ms: &[u64],
    window_ms: u64,
    volume_threshold: u32,
) -> AlertSeverity {
    if timestamps_ms.len() <= volume_threshold as usize {
        return AlertSeverity::None;
    }
    // Sliding window: if any window contains > threshold events, fire high.
    let mut sorted = timestamps_ms.to_vec();
    sorted.sort_unstable();
    let needed = volume_threshold.saturating_add(1) as usize;
    if sorted.len() < needed {
        return AlertSeverity::None;
    }
    for win_start in 0..=sorted.len().saturating_sub(needed) {
        let start = sorted[win_start];
        let end = sorted[win_start + needed - 1];
        if end.saturating_sub(start) <= window_ms {
            return AlertSeverity::High;
        }
    }
    AlertSeverity::None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alerting_rule_new_cluster_high_volume() {
        let base = 1_700_000_000_000u64;
        let mut ts = Vec::new();
        for i in 0..11 {
            ts.push(base + i * 5_000);
        }
        assert_eq!(
            new_cluster_high_volume(&ts, 60_000, 10),
            AlertSeverity::High
        );
    }
}
