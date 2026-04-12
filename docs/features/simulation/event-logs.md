# 17.1 — Event Logs

## Bounded Log Primitive

| ID       | Feature                             |
|----------|-------------------------------------|
| F-17.1.1 | Generic Bounded EventLog Primitive |
| F-17.1.2 | Per-Entry Memory Budget            |

1. **F-17.1.1** — `EventLog<T>` is a generic bounded ring buffer of typed entries with timestamps.
   Oldest entries are evicted when capacity is reached. One primitive replaces NPC memory, gossip
   systems, threat tables, and combat logs.
   - **Deps:** F-1.1.1 (ECS Storage), F-13.1.2 (Game Clock)
2. **F-17.1.2** — Per-entry memory is bounded to 256 bytes across all platforms to keep aggregate
   log memory within mobile budgets when thousands of NPCs each hold an event log.
   - **Deps:** F-17.1.1

## Decay

| ID       | Feature                     |
|----------|-----------------------------|
| F-17.1.3 | Entry Accuracy Decay Curves|
| F-17.1.4 | Fast Decay Pass            |

1. **F-17.1.3** — Entries decay accuracy over time via configurable decay curves (linear,
   exponential, step). Accuracy starts at 1.0 and decays toward 0.0. NPC memory fades with realistic
   uncertainty rather than fixed timeouts.
   - **Deps:** F-17.1.1
2. **F-17.1.4** — The decay pass runs across 1,000 active event logs in under 2 ms per tick on
   desktop hardware, fitting within per-tick simulation budgets.
   - **Deps:** F-17.1.1, F-17.1.3

## Propagation

| ID       | Feature                        |
|----------|--------------------------------|
| F-17.1.5 | Propagation with Accuracy Loss|
| F-17.1.6 | Neighbor Propagation Budget   |

1. **F-17.1.5** — Propagation rules spread entries between entities with configurable per-hop
   accuracy loss. Gossip scenarios benefit: rumors lose fidelity as they pass between NPCs.
   - **Deps:** F-17.1.1, F-1.9.1 (Shared Spatial Index)
2. **F-17.1.6** — Propagation from one source to 50 neighbors completes within 0.5 ms on desktop
   hardware, supporting crowd-scale rumor spread.
   - **Deps:** F-17.1.5

## Threshold Triggers

| ID       | Feature                       |
|----------|-------------------------------|
| F-17.1.7 | Threshold-Based Event Triggers|

1. **F-17.1.7** — Threshold triggers fire events when an entity's log matches a configurable
   condition (count, time window, event type). Example: "Alert after 3 hostile events in 60
   seconds".
   - **Deps:** F-17.1.1, F-1.5.1 (Event Channels)

## Persistence

| ID       | Feature                 |
|----------|-------------------------|
| F-17.1.8 | Save/Load Round Trip   |

1. **F-17.1.8** — Event logs serialize to the save system via rkyv zero-copy binary with
   bit-identical round-trip. NPC memory persists deterministically across save/load.
   - **Deps:** F-17.1.1, F-1.4.1
