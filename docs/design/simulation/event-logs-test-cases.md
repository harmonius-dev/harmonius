# Event Logs — Test Cases

Companion to [event-logs.md](event-logs.md).

Test case IDs use `TC-17.1.Z.N` format. Every row links to a specific R-17.1.Z or NFR-EL.N.

## Unit Tests

| ID            | Name                                | Req       |
|---------------|-------------------------------------|-----------|
| TC-17.1.1.1   | `test_push_under_capacity`          | R-17.1.1  |
| TC-17.1.1.2   | `test_push_evicts_oldest_when_full` | R-17.1.1  |
| TC-17.1.1.3   | `test_entries_in_timestamp_order`   | R-17.1.1  |
| TC-17.1.1.4   | `test_clear_resets_count_and_head`  | R-17.1.1  |
| TC-17.1.2.1   | `test_entry_size_within_budget`     | R-17.1.2  |
| TC-17.1.2.2   | `test_entry_size_with_smolstr`      | R-17.1.2  |
| TC-17.1.3.1   | `test_decay_linear_half_life`       | R-17.1.3  |
| TC-17.1.3.2   | `test_decay_exponential_curve`      | R-17.1.3  |
| TC-17.1.3.3   | `test_decay_step_threshold`         | R-17.1.3  |
| TC-17.1.3.4   | `test_decay_clamps_to_zero`         | R-17.1.3  |
| TC-17.1.3.5   | `test_decay_min_accuracy_floor`     | R-17.1.3  |
| TC-17.1.5.1   | `test_propagate_single_hop`         | R-17.1.5  |
| TC-17.1.5.2   | `test_propagate_increments_hops`    | R-17.1.5  |
| TC-17.1.5.3   | `test_propagate_max_hops_stops`     | R-17.1.5  |
| TC-17.1.5.4   | `test_propagate_dedupes_entries`    | R-17.1.5  |
| TC-17.1.5.5   | `test_propagate_filter_tags`        | R-17.1.5  |
| TC-17.1.7.1   | `test_threshold_below_count`        | R-17.1.7  |
| TC-17.1.7.2   | `test_threshold_meets_count`        | R-17.1.7  |
| TC-17.1.7.3   | `test_threshold_window_excludes`    | R-17.1.7  |
| TC-17.1.7.4   | `test_threshold_fires_once_only`    | R-17.1.7  |
| TC-17.1.8.1   | `test_rkyv_roundtrip_empty_log`     | R-17.1.8  |
| TC-17.1.8.2   | `test_rkyv_roundtrip_full_log`      | R-17.1.8  |
| TC-17.1.8.3   | `test_query_filter_event_type`      | R-17.1.1  |
| TC-17.1.8.4   | `test_query_filter_time_range`      | R-17.1.1  |
| TC-17.1.8.5   | `test_query_filter_min_accuracy`    | R-17.1.1  |

1. **TC-17.1.1.1** `test_push_under_capacity` — Push 3 entries into an `EventLog` with
   `capacity = 8`. Assert the count is 3 and the entries appear in insertion order.
   - Input: `EventLog::new(EventLogId(1), 8, linear_decay, 1)`, push three entries with timestamps
     `100`, `101`, `102`
   - Expected: `log.count() == 3`, `log.entries()[0].timestamp == 100`,
     `log.entries()[2].timestamp == 102`, `log.is_full() == false`

2. **TC-17.1.1.2** `test_push_evicts_oldest_when_full` — Push 5 entries into a `capacity = 4` log.
   Assert the oldest entry is evicted and the four most recent remain.
   - Input: capacity 4; push timestamps `10, 20, 30, 40, 50`
   - Expected: `log.count() == 4`, timestamps in entries are `[20, 30, 40, 50]`,
     `log.is_full() == true`

3. **TC-17.1.1.3** `test_entries_in_timestamp_order` — Push entries with monotonically increasing
   ticks across a wrap. Assert iteration yields them in timestamp order, not raw buffer order.
   - Input: capacity 4; push 6 entries with ticks `1..=6`
   - Expected: `entries()` yields ticks `[3, 4, 5, 6]` in that order

4. **TC-17.1.1.4** `test_clear_resets_count_and_head` — Push 3 entries then `clear()`. Assert count
   and head reset and `most_recent()` returns `None`.
   - Input: push 3 entries, call `log.clear()`
   - Expected: `log.count() == 0`, `log.most_recent().is_none()`,
     `log.entries_above_accuracy(0.0).is_empty()`

5. **TC-17.1.2.1** `test_entry_size_within_budget` — Static assert that
   `size_of::<DecayingEntry<CombatEvent>>() <= 256` for a representative `T = CombatEvent` with
   damage, type, and target fields.
   - Input: `CombatEvent { damage: f32, kind: u8, target: Entity }`
   - Expected: `const _: () = assert!(size_of::<DecayingEntry<CombatEvent>>() <= 256);` compiles

6. **TC-17.1.2.2** `test_entry_size_with_smolstr` — Same assertion for `T = DialogueLine` containing
   a `SmolStr` payload (inline up to 23 bytes). Assert size still under 256 B.
   - Input: `DialogueLine { speaker: Entity, line: SmolStr }`
   - Expected: `size_of::<DecayingEntry<DialogueLine>>() <= 256` evaluates to true

7. **TC-17.1.3.1** `test_decay_linear_half_life` — Linear decay with `rate = 0.1`. Push entry at
   tick 0 with accuracy 1.0; advance to tick 5; assert accuracy is `0.5 ± 1e-6`.
   - Input: `DecayCurve { rate: 0.1, min_accuracy: 0.0, curve_type: Linear }`, push at tick 0,
     `decay_tick(5)`
   - Expected: `entry.accuracy ≈ 0.5`, error within `1e-6`

8. **TC-17.1.3.2** `test_decay_exponential_curve` — Exponential decay with `rate = 0.5`. After 2
   ticks, accuracy must equal `(1.0 - 0.5)^2 = 0.25`.
   - Input: `DecayCurve { rate: 0.5, min_accuracy: 0.0, curve_type: Exponential }`, push at tick 0,
     `decay_tick(2)`
   - Expected: `entry.accuracy ≈ 0.25`, within `1e-6`

9. **TC-17.1.3.3** `test_decay_step_threshold` — Step curve with threshold tick of 10. Before 10,
   accuracy is 1.0; at 10, drops to `min_accuracy`.
   - Input: `DecayCurve { rate: 10.0, min_accuracy: 0.2, curve_type: Step }`, push at tick 0,
     `decay_tick(9)` then `decay_tick(10)`
   - Expected: after `decay_tick(9)` accuracy is 1.0; after `decay_tick(10)` accuracy is 0.2

10. **TC-17.1.3.4** `test_decay_clamps_to_zero` — Linear decay over a long horizon. Assert accuracy
    is clamped to `min_accuracy` and never goes negative.
    - Input: `rate = 0.1`, `min_accuracy = 0.0`, push at tick 0, `decay_tick(1000)`
    - Expected: `entry.accuracy == 0.0`, not `-99.0`

11. **TC-17.1.3.5** `test_decay_min_accuracy_floor` — Decay with `min_accuracy = 0.3`. After many
    ticks, accuracy floors at 0.3 and stays there.
    - Input: `rate = 0.5`, `min_accuracy = 0.3`, exponential, push tick 0, `decay_tick(100)`
    - Expected: `entry.accuracy == 0.3`

12. **TC-17.1.5.1** `test_propagate_single_hop` — Source log has one entry at accuracy 1.0;
    `accuracy_multiplier = 0.5`. Propagate to empty target. Assert target receives entry at accuracy
    0.5.
    - Input: source with one entry; rule with `range = 10.0`, `accuracy_multiplier = 0.5`,
      `delay_ticks = 0`, `max_hops = 5`, `filter_tags = TagSet::EMPTY`
    - Expected: `target.entries()[0].accuracy ≈ 0.5`, `target.entries()[0].hop_count == 1`

13. **TC-17.1.5.2** `test_propagate_increments_hops` — Propagate A→B→C. Assert `hop_count` is 1 in B
    and 2 in C.
    - Input: chain of three logs, propagate twice
    - Expected: `B.entries()[0].hop_count == 1`, `C.entries()[0].hop_count == 2`

14. **TC-17.1.5.3** `test_propagate_max_hops_stops` — Rule with `max_hops = 2`. Propagate four times
    in a chain. Assert no entry past hop 2 exists.
    - Input: `max_hops: 2`, four sequential propagations
    - Expected: hop_count maxes at 2; later targets remain empty

15. **TC-17.1.5.4** `test_propagate_dedupes_entries` — Propagate the same source entry twice into
    the same target. Assert target contains exactly one copy.
    - Input: call `propagate_entries(source, target, rule, tick)` twice with no source change
    - Expected: `target.count() == 1`

16. **TC-17.1.5.5** `test_propagate_filter_tags` — Source log contains entries tagged "hostile" and
    "friendly". Propagation rule's `filter_tags` includes only "hostile". Assert target receives
    only the "hostile" entry.
    - Input: two entries with distinct tags, `filter_tags = TagSet::from(["hostile"])`
    - Expected: `target.count() == 1`, surviving entry has the "hostile" tag

17. **TC-17.1.7.1** `test_threshold_below_count` — Trigger requires `count = 3` matches in
    `window_ticks = 60`. Push 2 matching entries. Assert no action fires.
    - Input: trigger with `predicate = P`, `count = 3`, `window_ticks = 60`, action
      `FireEvent("alert".into())`; push 2 matching entries within window
    - Expected: `check_thresholds(...).is_empty()`

18. **TC-17.1.7.2** `test_threshold_meets_count` — Push 3 matching entries within window. Assert
    `check_thresholds` returns the configured action exactly once.
    - Input: same trigger, push 3 matching entries at ticks 100, 110, 120; current_tick = 130
    - Expected: result has length 1; element matches `ThresholdAction::FireEvent("alert")`

19. **TC-17.1.7.3** `test_threshold_window_excludes` — Entries scattered outside the window do not
    count. Push 5 entries spaced 100 ticks apart with `window_ticks = 60`. Assert no fire.
    - Input: entries at ticks `0, 100, 200, 300, 400`; current_tick = 400; window 60
    - Expected: only the entry at tick 400 falls in window; result is empty

20. **TC-17.1.7.4** `test_threshold_fires_once_only` — After firing, the same trigger must not
    re-fire on the next tick unless new entries arrive that re-cross the threshold.
    - Input: trigger fires at tick 130; call `check_thresholds` again at tick 131 with no new
      entries
    - Expected: second call returns the same set; system-level dedup (covered in integration test)
      suppresses repeat dispatch

21. **TC-17.1.8.1** `test_rkyv_roundtrip_empty_log` — Serialize an empty `EventLog<CombatEvent>` via
    rkyv; deserialize; assert byte-identical structure and zero entries.
    - Input: `EventLog::new(EventLogId(7), 32, decay, 1)`
    - Expected: `archived.count == 0`, `archived.capacity == 32`, byte length matches expected

22. **TC-17.1.8.2** `test_rkyv_roundtrip_full_log` — Push 16 entries with varied timestamps and
    accuracies. Serialize, deserialize via zero-copy mmap. Assert all fields equal originals.
    - Input: 16 entries with random tick/accuracy/source values
    - Expected: each `archived.entries[i]` matches original byte-for-byte

23. **TC-17.1.8.3** `test_query_filter_event_type` — Log has entries with three different event type
    IDs. Query filtered by one type returns only those.
    - Input: 9 entries (3 of each type), `EventLogQuery { event_type: Some(2), .. }`
    - Expected: result length == 3; all results have `event_type == 2`

24. **TC-17.1.8.4** `test_query_filter_time_range` — `entries_in_window(50, 100)` over entries at
    ticks `[10, 60, 80, 110]`. Assert two entries returned (60 and 80).
    - Input: log with 4 entries; window `[50, 100]`
    - Expected: result length == 2, ticks `[60, 80]`, inclusive boundary obeyed

25. **TC-17.1.8.5** `test_query_filter_min_accuracy` — `entries_above_accuracy(0.5)` over entries
    with accuracies `[0.9, 0.6, 0.4, 0.1]`. Assert two entries returned.
    - Input: 4 entries with given accuracies
    - Expected: result length == 2; both returned entries have accuracy > 0.5

## Integration Tests

| ID            | Name                                  | Req       |
|---------------|---------------------------------------|-----------|
| TC-17.1.I.1   | `test_decay_pass_1000_logs`           | R-17.1.4  |
| TC-17.1.I.2   | `test_propagation_to_50_neighbors`    | R-17.1.6  |
| TC-17.1.I.3   | `test_log_entry_added_event_emitted`  | R-17.1.1  |
| TC-17.1.I.4   | `test_log_entry_decayed_event`        | R-17.1.3  |
| TC-17.1.I.5   | `test_threshold_fires_ecs_event`      | R-17.1.7  |
| TC-17.1.I.6   | `test_save_load_roundtrip`            | R-17.1.8  |
| TC-17.1.I.7   | `test_propagation_uses_shared_bvh`    | R-17.1.5  |
| TC-17.1.I.8   | `test_codegen_predicate_hot_reload`   | R-17.1.7  |

1. **TC-17.1.I.1** `test_decay_pass_1000_logs` — Spawn 1,000 entities each with an `EventLog`
   holding 100 entries. Run one `SimulationDecay` stage tick. Assert wall time under 2 ms and every
   entry's accuracy is reduced by the expected linear amount.
   - Input: 1,000 logs × 100 entries, linear `rate = 0.01`, current_tick advanced by 1
   - Expected: total stage time < 2 ms; each `entry.accuracy == old - 0.01`

2. **TC-17.1.I.2** `test_propagation_to_50_neighbors` — Cluster of 51 entities within propagation
   range. Push one entry on the central entity. Run propagation phase. Assert all 50 neighbors
   receive the entry within 0.5 ms.
   - Input: 51 entities placed in 5 m radius; range = 5 m; one source entry
   - Expected: each neighbor's log contains exactly 1 entry with `hop_count == 1`; phase time < 0.5
     ms

3. **TC-17.1.I.3** `test_log_entry_added_event_emitted` — Push an entry through `EventLog::push` in
   `PostUpdate`. Assert exactly one `LogEntryAdded` ECS event fires for that frame with the correct
   entity, entry id, and timestamp.
   - Input: one entity, one push at tick 200
   - Expected: `EventReader<LogEntryAdded>` yields one event `{ entity, entry_id, timestamp: 200 }`

4. **TC-17.1.I.4** `test_log_entry_decayed_event` — Run decay until an entry crosses `min_accuracy`.
   Assert exactly one `LogEntryDecayed` event is emitted with the entry's final accuracy.
   - Input: one entry at accuracy 1.0; linear `rate = 0.5`; advance 3 ticks
   - Expected: at the tick that crosses min_accuracy, one `LogEntryDecayed` event fires; entry is
     pruned next tick

5. **TC-17.1.I.5** `test_threshold_fires_ecs_event` — Configure trigger "3 hostile entries within 60
   ticks → FireEvent('alert')". Push 3 hostile entries. Assert one `ThresholdFired` event in the
   next stage with `fired_action = FireEvent("alert")` and `matched_count = 3`.
   - Input: trigger as described; push 3 hostile entries at ticks 100, 130, 150; current_tick 155
   - Expected: one `ThresholdFired` event reads back; subsequent ticks without new entries do not
     re-emit (system-level dedup)

6. **TC-17.1.I.6** `test_save_load_roundtrip` — Populate a world with 100 entities each with
   randomized logs. Save via the engine save system; reload from the file. Assert every loaded log
   is byte-identical to its source.
   - Input: 100 entities, varied capacities, decay curves, hop counts
   - Expected: `loaded_log == saved_log` for every entity; bit-identical entry list, head, count

7. **TC-17.1.I.7** `test_propagation_uses_shared_bvh` — Place entities at varied positions; run
   propagation. Assert spatial queries hit the shared BVH (F-1.9.1) and return only entities inside
   `range`.
   - Input: 200 entities scattered in a 100 m × 100 m area; rule range = 5 m
   - Expected: `LogPropagated` events fire only for pairs whose distance is ≤ 5 m; no false
     positives across the 100 × 100 grid

8. **TC-17.1.I.8** `test_codegen_predicate_hot_reload` — Modify a `PredicateId`-bound visual filter
   graph in the editor and rebuild the middleman .dylib. Assert the running game picks up the new
   function pointer without restart and threshold evaluation reflects the new logic.
   - Input: hot-reload the middleman .dylib while a trigger is active
   - Expected: subsequent `check_thresholds` calls dispatch to the new function pointer; no
     stale-pointer crash

## Benchmarks

| ID            | Benchmark                            | Target     | Req         |
|---------------|--------------------------------------|------------|-------------|
| TC-17.1.B.1   | Decay pass 1,000 logs × 100 entries  | < 2 ms     | R-17.1.4    |
| TC-17.1.B.2   | Decay pass 1 log × 1,000 entries     | < 0.1 ms   | NFR-EL.1    |
| TC-17.1.B.3   | Propagation to 50 neighbors          | < 0.5 ms   | NFR-EL.2    |
| TC-17.1.B.4   | rkyv save/load (10k entries)         | < 5 ms     | R-17.1.8    |
| TC-17.1.B.5   | Threshold scan (1k entries window)   | < 50 µs    | R-17.1.7    |

1. **TC-17.1.B.1** — Spawn 1,000 `EventLog<CombatEvent>` components with 100 entries each. Measure
   wall time of one `decay_tick` pass over the entire query. Measured with `criterion`.

2. **TC-17.1.B.2** — Single log with 1,000 entries. Measure one `decay_tick` call. Tests the
   inner-loop hot path independent of ECS iteration overhead.

3. **TC-17.1.B.3** — 51 entities clustered inside propagation range. Single source push and one
   `propagate_entries` call. Wall time end to end including spatial query.

4. **TC-17.1.B.4** — Serialize an `EventLog` with 10k entries to rkyv bytes; mmap-load back. Wall
   time from `serialize` call through first `entries()` access on the loaded log.

5. **TC-17.1.B.5** — `check_thresholds` over a log with 1,000 entries against 4 distinct triggers,
   each with a `window_ticks` of 60. Wall time for the full scan.
