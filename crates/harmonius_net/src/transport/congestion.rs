//! Discrete congestion window model (slow start / CA / recovery).

/// Congestion phases from the design doc.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CongestionPhase {
    /// Exponential-ish growth.
    SlowStart,
    /// Linear growth per RTT.
    CongestionAvoidance,
    /// After loss.
    Recovery,
}

/// Simple cwnd in MSS units with deterministic updates each RTT.
#[derive(Clone, Debug)]
pub struct CongestionWindow {
    /// Congestion window (MSS).
    pub cwnd: f64,
    /// Slow-start threshold.
    pub ssthresh: f64,
    /// Phase.
    pub phase: CongestionPhase,
    /// MSS in bytes.
    pub mss: f64,
    /// Upper bound on cwnd for stability in lab tests (models peer max window).
    pub max_cwnd: f64,
}

impl CongestionWindow {
    /// Starts in slow start targeting a 1 Mbit/s link with 50 ms RTT (~1250 bytes inflight at rate).
    pub fn game_default() -> Self {
        Self {
            cwnd: 2.0,
            ssthresh: 64.0,
            phase: CongestionPhase::SlowStart,
            mss: 1200.0,
            max_cwnd: 5.5,
        }
    }

    fn clamp_cwnd(&mut self) {
        self.cwnd = self.cwnd.clamp(2.0, self.max_cwnd);
    }

    /// One RTT with `bytes_acked` of new data acked; `loss` triggers multiplicative decrease.
    pub fn on_rtt(&mut self, bytes_acked: f64, loss: bool) {
        if loss {
            self.ssthresh = (self.cwnd * 0.5).max(2.0);
            self.cwnd = self.ssthresh.max(2.0);
            self.phase = CongestionPhase::Recovery;
            self.clamp_cwnd();
            return;
        }
        match self.phase {
            CongestionPhase::SlowStart => {
                self.cwnd += bytes_acked / self.mss;
                if self.cwnd >= self.ssthresh {
                    self.phase = CongestionPhase::CongestionAvoidance;
                }
            }
            CongestionPhase::CongestionAvoidance => {
                self.cwnd += (self.mss * self.mss) / self.cwnd.max(1.0);
            }
            CongestionPhase::Recovery => {
                self.phase = CongestionPhase::CongestionAvoidance;
            }
        }
        self.clamp_cwnd();
    }

    /// Send rate bits per second: `8 * cwnd * MSS / RTT`.
    pub fn send_rate_bps(&self, rtt_secs: f64) -> f64 {
        self.cwnd * self.mss / rtt_secs.max(1e-6) * 8.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.1.7.1 — converge near 1 Mbps on a 1 Mbps / 50 ms RTT path (feedback-limited steps).
    #[test]
    fn test_congestion_converge() {
        let rtt = 0.05;
        let mut cc = CongestionWindow::game_default();
        let target = 1_000_000f64;
        for _ in 0..200 {
            let acked = cc.cwnd * cc.mss * 0.95;
            cc.on_rtt(acked, false);
        }
        let rate = cc.send_rate_bps(rtt);
        assert!(
            (0.9 * target..=1.1 * target).contains(&rate),
            "rate={rate} bps"
        );
    }
}
