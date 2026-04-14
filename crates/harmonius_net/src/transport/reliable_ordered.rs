//! Reliable ordered channel with selective repeat and SACK (test doubles).

/// Runs a selective-repeat simulation: `seq` is message id, second arg is attempt number.
///
/// Returns `(delivered_in_order, retx_count)`.
pub fn simulate_reliable_lossy<F: FnMut(u32, u32) -> bool>(
    count: u32,
    mut loss: F,
) -> (Vec<u32>, u32) {
    let mut delivered: Vec<u32> = Vec::new();
    let mut got: Vec<bool> = vec![false; count as usize];
    let mut retx = 0u32;
    let mut inflight: Vec<(u32, u32)> = Vec::new(); // (seq, attempt)

    for s in 0..count {
        inflight.push((s, 0));
    }

    let mut safety = 0usize;
    while got.iter().any(|g| !*g) && safety < 1_000_000 {
        safety += 1;
        let batch = std::mem::take(&mut inflight);
        for (seq, attempt) in batch {
            if loss(seq, attempt) {
                let next_attempt = attempt + 1;
                if next_attempt > attempt {
                    retx += 1;
                }
                inflight.push((seq, next_attempt));
                continue;
            }
            got[seq as usize] = true;
        }
        // Deliver in order to application when contiguous from 0
        while (delivered.len() as u32) < count && got[delivered.len()] {
            delivered.push(delivered.len() as u32);
        }
    }
    (delivered, retx)
}

/// SACK: ranges of received seq excluding holes; sender retransmits only missing.
pub fn retransmit_missing_only(
    sent_max: u32,
    sack_ok: &[(u32, u32)],
    lost: &[u32],
) -> Vec<u32> {
    let mut need: Vec<u32> = (0..=sent_max).collect();
    for &(lo, hi) in sack_ok {
        need.retain(|&s| s < lo || s > hi);
    }
    for &l in lost {
        if !need.contains(&l) {
            need.push(l);
        }
    }
    need.sort_unstable();
    need.dedup();
    need
}

/// Stateful channel counting retransmissions (used by SACK test).
#[derive(Debug, Default)]
pub struct ReliableOrderedChannel {
    retx: u32,
}

impl ReliableOrderedChannel {
    /// Zero retx counter.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn retx_count(&self) -> u32 {
        self.retx
    }

    /// Marks retransmissions for `seqs` (each call counts one retx per seq).
    pub fn note_retx(&mut self, seqs: &[u32]) {
        self.retx += seqs.len() as u32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.1.3.1 — 10% loss, all payloads arrive in order at the app.
    #[test]
    fn test_reliable_ordered_in_order() {
        let mut rng_state = 0xC0FFEEu32;
        let mut rand = || {
            rng_state = rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
            (rng_state >> 8) & 0xFF
        };
        let (got, _retx) = simulate_reliable_lossy(100, |_seq, attempt| {
            if attempt > 0 {
                return false;
            }
            rand() < 26 // ~10%
        });
        assert_eq!(got, (0..100).collect::<Vec<_>>());
    }

    /// TC-8.1.3.2 — SACK says holes 3,4 missing; only those retransmit.
    #[test]
    fn test_reliable_ordered_sack() {
        let lost = [3u32, 4];
        let sack = [(0u32, 2u32), (5, 9)];
        let need = retransmit_missing_only(9, &sack, &lost);
        assert_eq!(need, vec![3, 4]);
        let mut ch = ReliableOrderedChannel::new();
        ch.note_retx(&need);
        assert_eq!(ch.retx_count(), 2);
    }
}
