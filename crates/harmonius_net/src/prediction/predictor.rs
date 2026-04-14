//! Deterministic replay after server correction.

/// Simple 1D predicted state with inputs as position deltas.
#[derive(Clone, Debug, PartialEq)]
pub struct PredictedState {
    /// Position.
    pub pos: f32,
}

/// Server tells client authoritative position at `ack_tick`.
#[derive(Clone, Debug, PartialEq)]
pub struct Correction {
    /// Authoritative position.
    pub pos: f32,
    /// Server ack tick.
    pub ack_tick: u32,
}

/// Replays `inputs` (delta per tick) after applying `correction.pos` for ticks `> correction.ack_tick`.
pub fn replay_unacked(correction: &Correction, inputs: &[(u32, f32)]) -> PredictedState {
    let mut pos = correction.pos;
    for &(t, dpos) in inputs {
        if t > correction.ack_tick {
            pos += dpos;
        }
    }
    PredictedState { pos }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.4.1.1 — replay unacked inputs after correction.
    #[test]
    fn test_prediction_replay_inputs() {
        let inputs: Vec<(u32, f32)> = (100..=110).map(|t| (t, 0.1)).collect();
        let corr = Correction {
            pos: 10.5,
            ack_tick: 105,
        };
        let out = replay_unacked(&corr, &inputs);
        assert!((out.pos - 11.0).abs() < 0.02, "pos={}", out.pos);
    }
}
