# Networking Non-Functional User Stories

Non-functional stories for the Networking domain (X = 8). Latency, throughput, capacity, and
operational targets that drive matching requirements in
[`requirements/networking/transport-layer.md`](../../requirements/networking/transport-layer.md),
[`requirements/networking/state-replication.md`](../../requirements/networking/state-replication.md),
and [`requirements/networking/mmo-infrastructure.md`](../../requirements/networking/mmo-infrastructure.md).

Folder rules: stories use the `As a {persona} (P-N), I want {action}, so that {benefit}` form
with no acceptance criteria, no feature links, and no requirement links inline. Persona
definitions live in [`personas.md`](../personas.md).

## Transport Latency

| ID           | Persona              |
|--------------|----------------------|
| US-8.NFR.1.1 | gamer (P-23)         |
| US-8.NFR.1.2 | engine dev (P-26)    |
| US-8.NFR.1.3 | engine tester (P-27) |

1. **US-8.NFR.1.1** — As a gamer (P-23), I want my actions to register on the server within
   one server tick, so that the game feels responsive.
2. **US-8.NFR.1.2** — As an engine dev (P-26), I want QUIC handshake completion under 200 ms
   on a 100 ms RTT path, so that join latency is short.
3. **US-8.NFR.1.3** — As an engine tester (P-27), I want to verify p99 RPC RTT under 250 ms on
   a 100 ms baseline, so that latency regressions are caught.

## Replication Throughput

| ID           | Persona              |
|--------------|----------------------|
| US-8.NFR.2.1 | server admin (P-22)  |
| US-8.NFR.2.2 | engine dev (P-26)    |
| US-8.NFR.2.3 | engine tester (P-27) |

1. **US-8.NFR.2.1** — As a server admin (P-22), I want a single zone server to push state for
   500 active entities at 30 Hz, so that crowded battlegrounds remain playable.
2. **US-8.NFR.2.2** — As an engine dev (P-26), I want delta compression to keep average per-
   client uplink under 100 KB/s on a typical battle, so that broadband isn't required.
3. **US-8.NFR.2.3** — As an engine tester (P-27), I want to verify 500-entity scenes consume
   under 100 KB/s per client at 30 Hz, so that bandwidth budgets are regression-tested.

## Area-of-Interest Scaling

| ID           | Persona              |
|--------------|----------------------|
| US-8.NFR.3.1 | server admin (P-22)  |
| US-8.NFR.3.2 | engine dev (P-26)    |
| US-8.NFR.3.3 | engine tester (P-27) |

1. **US-8.NFR.3.1** — As a server admin (P-22), I want AOI updates per tick capped to 100
   entities per client, so that worst-case crowding cannot saturate uplinks.
2. **US-8.NFR.3.2** — As an engine dev (P-26), I want AOI lookups to be O(log N) over the
   spatial index, so that worst-case query cost stays bounded.
3. **US-8.NFR.3.3** — As an engine tester (P-27), I want to verify p95 AOI lookup latency
   under 50 µs at 50 000 entities, so that AOI scaling is regression-tested.

## Server-Mesh Handoff

| ID           | Persona              |
|--------------|----------------------|
| US-8.NFR.4.1 | gamer (P-23)         |
| US-8.NFR.4.2 | engine dev (P-26)    |
| US-8.NFR.4.3 | engine tester (P-27) |

1. **US-8.NFR.4.1** — As a gamer (P-23), I want zone transitions with no loading screen and
   no perceived hitch, so that the world feels continuous.
2. **US-8.NFR.4.2** — As an engine dev (P-26), I want the handoff of authoritative state to
   complete in under 100 ms, so that the client never observes a discontinuity.
3. **US-8.NFR.4.3** — As an engine tester (P-27), I want to verify mid-combat handoffs under
   100 ms preserve buffs, cooldowns, and pending RPCs, so that handoff is regression-tested.

## Auto-Scaling

| ID           | Persona              |
|--------------|----------------------|
| US-8.NFR.5.1 | server admin (P-22)  |
| US-8.NFR.5.2 | engine dev (P-26)    |
| US-8.NFR.5.3 | engine tester (P-27) |

1. **US-8.NFR.5.1** — As a server admin (P-22), I want a 5× population surge handled by new
   server processes within 30 seconds, so that demand spikes don't cause queues.
2. **US-8.NFR.5.2** — As an engine dev (P-26), I want graceful drain before pod termination,
   so that scale-down never disconnects players abruptly.
3. **US-8.NFR.5.3** — As an engine tester (P-27), I want to verify auto-scale-up within 30 s
   and drain within 60 s on simulated population swings, so that auto-scaling is regression
   tested.

## Replay Storage

| ID           | Persona              |
|--------------|----------------------|
| US-8.NFR.6.1 | server admin (P-22)  |
| US-8.NFR.6.2 | engine dev (P-26)    |
| US-8.NFR.6.3 | engine tester (P-27) |

1. **US-8.NFR.6.1** — As a server admin (P-22), I want replay files compressed to ≤ 5 MB per
   minute of gameplay, so that long-term storage stays affordable.
2. **US-8.NFR.6.2** — As an engine dev (P-26), I want replay seeking to land any frame within
   100 ms, so that the replay viewer is responsive.
3. **US-8.NFR.6.3** — As an engine tester (P-27), I want to verify a 60-minute replay
   compresses under 300 MB and seeks any frame in p95 under 100 ms, so that storage and seek
   targets are regression-tested.

## Anti-Cheat Overhead

| ID           | Persona              |
|--------------|----------------------|
| US-8.NFR.7.1 | server admin (P-22)  |
| US-8.NFR.7.2 | engine dev (P-26)    |
| US-8.NFR.7.3 | engine tester (P-27) |

1. **US-8.NFR.7.1** — As a server admin (P-22), I want anti-cheat validators to add no more
   than 5% to server CPU time, so that anti-cheat doesn't shrink server capacity.
2. **US-8.NFR.7.2** — As an engine dev (P-26), I want movement and damage validators to run
   in under 10 µs per validation, so that hot-path overhead stays bounded.
3. **US-8.NFR.7.3** — As an engine tester (P-27), I want to verify validator overhead stays
   under 5% server CPU at peak load, so that anti-cheat overhead is regression-tested.

## Persistence Throughput

| ID           | Persona              |
|--------------|----------------------|
| US-8.NFR.8.1 | server admin (P-22)  |
| US-8.NFR.8.2 | engine dev (P-26)    |
| US-8.NFR.8.3 | engine tester (P-27) |

1. **US-8.NFR.8.1** — As a server admin (P-22), I want sustained 10 000 transactions per
   second to TiKV, so that persistence keeps up with peak gameplay.
2. **US-8.NFR.8.2** — As an engine dev (P-26), I want database access to never block the
   simulation tick, so that backend latency cannot degrade gameplay.
3. **US-8.NFR.8.3** — As an engine tester (P-27), I want to verify a 10 000-write soak with
   p99 simulation tick under 16 ms, so that persistence isolation is regression-tested.
