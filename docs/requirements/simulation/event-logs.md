# R-17.1 — Event Logs Requirements

## Bounded Log Primitive

1. **R-17.1.1** — The engine **SHALL** provide a generic bounded event log primitive `EventLog<T>`
   with configurable capacity, timestamp-ordered entries, and automatic eviction of oldest entries
   when capacity is reached.
   - **Rationale:** One generic primitive replaces NPC memory, gossip systems, threat tables, and
     combat logs — all of which are bounded timestamp-ordered entry collections.
   - **Verification:** Unit test: build an EventLog with capacity 4; push 5 entries; assert oldest
     entry evicted and remaining 4 entries in timestamp order.
2. **R-17.1.2** — The engine **SHALL** limit per-entry memory to at most 256 bytes on all platforms
   to keep aggregate log memory bounded on mobile targets.
   - **Rationale:** Thousands of NPCs each holding a log must not exceed mobile memory budgets; 256
     B per entry is the constraint derived from mobile targets.
   - **Verification:** Compile-time assertion: `assert_eq!(size_of::<LogEntry<T>>() <= 256)` for all
     instantiated T types.

## Decay

1. **R-17.1.3** — The engine **SHALL** decay log entry accuracy over time via configurable decay
   curves (linear, exponential, step) where accuracy starts at 1.0 and decays toward 0.0.
   - **Rationale:** NPC memory should fade realistically; fixed timeouts do not capture the gradual
     uncertainty of old information.
   - **Verification:** Unit test: push an entry; advance half of configured decay time; assert
     accuracy matches curve-expected value within tolerance.
2. **R-17.1.4** — The engine **SHALL** run the decay pass across 1,000 active event logs within 2 ms
   per tick on desktop hardware.
   - **Rationale:** Decay must run continuously for many entities; aggregate cost must remain within
     per-tick simulation budget.
   - **Verification:** Benchmark: spawn 1,000 EventLog components each with 100 entries; run one
     decay tick; assert total time under 2 ms.

## Propagation

1. **R-17.1.5** — The engine **SHALL** propagate log entries between entities according to
   configurable propagation rules with per-hop accuracy loss, where an entry copied from A to B
   arrives with reduced accuracy.
   - **Rationale:** Gossip systems require entries to spread with fidelity loss; a direct copy would
     let rumors remain as accurate as firsthand observation.
   - **Verification:** Unit test: push entry on entity A at accuracy 1.0; propagate to B with 0.5
     loss; assert B's entry has accuracy 0.5.
2. **R-17.1.6** — The engine **SHALL** complete propagation from one source entity to 50 nearby
   neighbors within 0.5 ms on desktop hardware.
   - **Rationale:** Proximity-based rumor spread runs on many NPCs; worst-case propagation must stay
     within per-tick budget for crowds.
   - **Verification:** Benchmark: place 51 entities in a cluster; push an entry on one; trigger
     propagation; assert total time under 0.5 ms.

## Threshold Triggers

1. **R-17.1.7** — The engine **SHALL** fire threshold-triggered events when an entity's event log
   matches a configurable condition (count, time window, event type) so that repeated events
   escalate entity behavior.
   - **Rationale:** "Alert after 3 hostile events in 60 seconds" is a core NPC behavior pattern;
     threshold evaluation must be a first-class log capability.
   - **Verification:** Unit test: configure threshold "3 hostile in 60s"; push 2 hostile entries;
     assert no trigger. Push third within window; assert trigger fires once.

## Persistence

1. **R-17.1.8** — The engine **SHALL** serialize event logs to the save system via rkyv zero-copy
   binary format and round-trip loads with bit-identical state restoration.
   - **Rationale:** NPC memory and reputation must persist across save/load; determinism requires
     bit-identical restoration.
   - **Verification:** Round-trip test: populate a log; save; load; assert loaded entries equal
     original byte-for-byte.
