//! Unreliable channel semantics (unordered and sequenced).

/// Unreliable unordered: no retransmissions; delivery count tracks received unique seqs.
#[derive(Debug, Default)]
pub struct UnreliableUnorderedChannel {
    pub(crate) seen: std::collections::HashSet<u32>,
    pub(crate) retx: u32,
}

impl UnreliableUnorderedChannel {
    /// New channel.
    pub fn new() -> Self {
        Self::default()
    }

    /// Receiver path: `dropped` simulates network loss before arrival.
    pub fn on_receive(&mut self, seq: u32, dropped: bool) {
        if dropped {
            return;
        }
        self.seen.insert(seq);
    }

    /// Sender path: unreliable never schedules retx.
    pub fn on_send_attempt(&mut self) {
        self.retx += 0;
    }

    /// Retransmission counter (always zero for this channel).
    pub fn retx_count(&self) -> u32 {
        self.retx
    }

    /// Delivered message count.
    pub fn delivered_count(&self) -> usize {
        self.seen.len()
    }
}

/// Unreliable sequenced: stale (older than latest) arrivals are dropped.
#[derive(Debug, Default)]
pub struct UnreliableSequencedRx {
    latest: Option<u32>,
    pub(crate) drops: u32,
    pub(crate) delivered: Vec<u32>,
}

impl UnreliableSequencedRx {
    /// Empty receiver.
    pub fn new() -> Self {
        Self::default()
    }

    /// Ingest one message with sequence `seq` (arrival order arbitrary).
    pub fn receive(&mut self, seq: u32) {
        match self.latest {
            None => {
                self.latest = Some(seq);
                self.delivered.push(seq);
            }
            Some(cur) => {
                if seq == cur {
                    return;
                }
                if seq > cur {
                    self.latest = Some(seq);
                    self.delivered.push(seq);
                } else {
                    self.drops += 1;
                }
            }
        }
    }

    /// Observed stale drops.
    pub fn drop_count(&self) -> u32 {
        self.drops
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.1.4.1 — loss does not trigger retransmissions; ~90% arrive with 10% independent loss.
    #[test]
    fn test_unreliable_unordered_no_retx() {
        let mut ch = UnreliableUnorderedChannel::new();
        let mut rng = 0xACE0u32;
        let mut rand = || {
            rng = rng.wrapping_mul(1103515245).wrapping_add(12345);
            (rng >> 16) & 0x7FFF
        };
        for i in 0..1000 {
            let drop = (rand() % 100) < 10;
            ch.on_receive(i, drop);
        }
        assert_eq!(ch.retx_count(), 0);
        let d = ch.delivered_count();
        assert!((800..=980).contains(&d), "delivered={d}");
    }

    /// TC-8.1.4.2 — reorder 1,3,2 delivers 1,3 and drops stale 2.
    #[test]
    fn test_unreliable_sequenced_drop_old() {
        let mut rx = UnreliableSequencedRx::new();
        rx.receive(1);
        rx.receive(3);
        rx.receive(2);
        assert_eq!(rx.delivered, vec![1, 3]);
        assert_eq!(rx.drop_count(), 1);
    }
}
