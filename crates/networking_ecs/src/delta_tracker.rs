//! Dense-array XOR delta compression and per-client baselines.

use crate::ids::{ConnectionId, Entity, SequenceTick};
use crate::wire::DeltaPayload;
use crate::wire::DeltaRun;

/// Errors surfaced by [`DeltaTracker`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeltaError {
    /// Baseline length disagrees with live chunk bytes (requires full resync).
    LayoutMismatch,
}

#[derive(Debug)]
struct BaselineState {
    connection: ConnectionId,
    tick: SequenceTick,
    state: Vec<u8>,
    /// Live bytes last encoded for this client until an ACK applies them to `state`.
    staged_live: Vec<u8>,
}

/// Tracks per-client dense baselines and computes XOR/RLE deltas.
#[derive(Debug, Default)]
pub struct DeltaTracker {
    baselines: Vec<BaselineState>,
}

impl DeltaTracker {
    /// Creates an empty tracker.
    #[must_use]
    pub fn new() -> Self {
        Self {
            baselines: Vec::new(),
        }
    }

    /// Computes a delta between the client's baseline and `live` bytes for `entity`.
    ///
    /// `entity` is accepted for API symmetry with the integration design; the dense `live` slice
    /// already reflects the entity's chunk layout for this stub.
    pub fn compute_delta(
        &mut self,
        client: ConnectionId,
        entity: Entity,
        tick: SequenceTick,
        live: &[u8],
    ) -> Result<DeltaPayload, DeltaError> {
        let _ = entity;
        let idx = match self.baseline_index(client) {
            Some(i) => i,
            None => {
                self.baselines.push(BaselineState {
                    connection: client,
                    tick: SequenceTick(0),
                    state: vec![0; live.len()],
                    staged_live: Vec::new(),
                });
                self.sort_baselines();
                self.baseline_index(client).expect("inserted client")
            }
        };

        let baseline = &mut self.baselines[idx];
        if baseline.state.len() != live.len() {
            return Err(DeltaError::LayoutMismatch);
        }

        let runs = compute_xor_rle(&baseline.state, live);
        baseline.staged_live.clear();
        baseline.staged_live.extend_from_slice(live);

        Ok(DeltaPayload { tick, runs })
    }

    /// Applies an ACK: moves staged live bytes into the dense baseline at `tick`.
    pub fn advance_baseline(&mut self, client: ConnectionId, tick: SequenceTick) {
        let Some(idx) = self.baseline_index(client) else {
            return;
        };
        let baseline = &mut self.baselines[idx];
        if baseline.staged_live.is_empty() {
            return;
        }
        if baseline.state.len() == baseline.staged_live.len() {
            baseline.state.copy_from_slice(&baseline.staged_live);
        }
        baseline.tick = tick;
    }

    /// Returns the stored baseline tick for `client`, if present.
    #[must_use]
    pub fn baseline_tick(&self, client: ConnectionId) -> Option<SequenceTick> {
        self.baseline_index(client)
            .map(|idx| self.baselines[idx].tick)
    }

    fn baseline_index(&self, client: ConnectionId) -> Option<usize> {
        self.baselines
            .binary_search_by_key(&client, |b| b.connection)
            .ok()
    }

    fn sort_baselines(&mut self) {
        self.baselines.sort_by_key(|b| b.connection);
    }
}

fn compute_xor_rle(baseline: &[u8], live: &[u8]) -> Vec<DeltaRun> {
    debug_assert_eq!(baseline.len(), live.len());
    let mut runs = Vec::new();
    let mut i = 0usize;
    while i < baseline.len() {
        if baseline[i] == live[i] {
            i += 1;
            continue;
        }
        let start = i;
        while i < baseline.len() && baseline[i] != live[i] {
            i += 1;
        }
        runs.push(DeltaRun {
            offset: start as u32,
            bytes: live[start..i].to_vec(),
        });
    }
    runs
}

#[cfg(test)]
mod tests {
    use super::*;
    use rkyv::Deserialize as _;

    #[test]
    fn tc_ir_4_4_2_1_single_field_run() {
        let mut tracker = DeltaTracker::new();
        let client = ConnectionId(1);
        let entity = Entity(42);
        let mut live = vec![0u8; 20];
        live[7] = 9;
        let delta = tracker
            .compute_delta(client, entity, SequenceTick(1), &live)
            .expect("delta");
        assert_eq!(delta.runs.len(), 1);
        assert_eq!(delta.runs[0].offset, 7);
        assert_eq!(delta.runs[0].bytes, vec![9]);
    }

    #[test]
    fn tc_ir_4_4_2_2_baseline_advances_on_ack() {
        let mut tracker = DeltaTracker::new();
        let client = ConnectionId(1);
        let entity = Entity(1);
        let live = vec![1u8, 2, 3];
        tracker
            .compute_delta(client, entity, SequenceTick(1), &live)
            .expect("delta");
        tracker.advance_baseline(client, SequenceTick(100));
        assert_eq!(tracker.baseline_tick(client), Some(SequenceTick(100)));
        let delta = tracker
            .compute_delta(client, entity, SequenceTick(101), &live)
            .expect("delta");
        assert!(delta.runs.is_empty());
    }

    #[test]
    fn tc_ir_4_4_2_3_slot_seven_offset() {
        let mut tracker = DeltaTracker::new();
        let client = ConnectionId(2);
        let entity = Entity(7);
        let slot_width = 4usize;
        let slots = 8usize;
        let len = slot_width * slots;
        let mut live = vec![0u8; len];
        let slot = 7usize;
        let byte_start = slot * slot_width;
        for b in &mut live[byte_start..byte_start + slot_width] {
            *b = 0xAB;
        }
        let delta = tracker
            .compute_delta(client, entity, SequenceTick(1), &live)
            .expect("delta");
        assert_eq!(delta.runs.len(), 1);
        assert_eq!(delta.runs[0].offset as usize, byte_start);
        assert_eq!(delta.runs[0].bytes.len(), slot_width);
    }

    #[test]
    fn tc_ir_4_4_2_4_rkyv_round_trip() {
        let original = DeltaPayload {
            tick: SequenceTick(9),
            runs: vec![
                DeltaRun {
                    offset: 0,
                    bytes: vec![1, 2, 3],
                },
                DeltaRun {
                    offset: 10,
                    bytes: vec![4, 5],
                },
            ],
        };
        let bytes = rkyv::to_bytes::<_, 256>(&original).expect("serialize");
        let archived = rkyv::check_archived_root::<DeltaPayload>(&bytes).expect("validate");
        let deserialized: DeltaPayload = archived
            .deserialize(&mut rkyv::Infallible)
            .expect("deserialize");
        assert_eq!(deserialized, original);
    }

    #[test]
    fn tc_ir_4_4_2_n1_ack_unknown_client_is_ignored() {
        let mut tracker = DeltaTracker::new();
        tracker.advance_baseline(ConnectionId(99), SequenceTick(1));
        assert_eq!(tracker.baseline_tick(ConnectionId(99)), None);
    }
}
