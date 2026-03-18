# R-8.1 — Transport Layer Requirements

## Connection Management

| ID      | Derived From                                            |
|---------|---------------------------------------------------------|
| R-8.1.1 | [F-8.1.1](../../features/networking/transport-layer.md) |
| R-8.1.2 | [F-8.1.2](../../features/networking/transport-layer.md) |

1. **R-8.1.1** — Connection Handshake and Authentication: The engine **SHALL** establish
   client-server connections via a multi-phase handshake that authenticates player identity using
   session tokens, resisting replay attacks, man-in-the-middle attacks, and connection flooding.
   - **Rationale:** MMO servers handling thousands of concurrent connection attempts must verify
     player identity and reject malicious connections before any gameplay traffic flows.
   - **Verification:** Integration test: perform 10,000 concurrent handshakes and verify all
     authenticate within 2 seconds. Replay a captured handshake token and verify the server rejects
     it. Inject a modified token mid-handshake and verify rejection. Flood 50,000 connection
     attempts per second and verify the server remains responsive to legitimate clients.
2. **R-8.1.2** — Connection Lifecycle Management: The engine **SHALL** track each connection through
   states (connecting, authenticated, active, migrating, disconnected) with configurable timeouts,
   heartbeat intervals, and graceful disconnect sequences, supporting at least 10,000 simultaneous
   connections per server process with O(1) per-connection keepalive and timeout detection overhead.
   - **Rationale:** MMO server density depends on per-connection overhead remaining constant
     regardless of player count.
   - **Verification:** Load test: establish 10,000 connections and verify heartbeat and timeout
     detection CPU cost grows linearly (O(1) per connection). Unit test: verify each state
     transition fires the correct lifecycle event. Simulate network loss and verify the connection
     transitions to disconnected after the configured timeout.

## Channel Architecture

| ID      | Derived From                                            |
|---------|---------------------------------------------------------|
| R-8.1.3 | [F-8.1.3](../../features/networking/transport-layer.md) |
| R-8.1.4 | [F-8.1.4](../../features/networking/transport-layer.md) |

1. **R-8.1.3** — Reliable Ordered Channel: The engine **SHALL** provide a reliable ordered delivery
   channel over UDP with selective acknowledgment (SACK), configurable retransmission timers, and
   congestion-aware send windows, guaranteeing in-order delivery of all transmitted messages.
   - **Rationale:** Critical gameplay events such as inventory changes and quest updates require
     guaranteed ordered delivery without head-of-line blocking from TCP.
   - **Verification:** Integration test: send 100,000 numbered messages over a lossy link (10%
     packet loss, 50 ms latency). Verify all messages arrive in order with zero loss. Benchmark
     retransmission timer accuracy within 5 ms of configured values.
2. **R-8.1.4** — Unreliable and Unordered Channels: The engine **SHALL** provide unreliable
   unordered, unreliable sequenced, and reliable unordered channel modes, each enforcing its
   declared delivery semantics per message.
   - **Rationale:** Different data types require different delivery guarantees; position updates
     drop stale packets, voice needs lowest latency, and entity spawns need delivery without
     ordering.
   - **Verification:** Unit test per mode: (1) unreliable unordered — send 1,000 messages, verify no
     reordering correction and no retransmission. (2) unreliable sequenced — send with simulated
     reordering, verify stale messages are dropped. (3) reliable unordered — send with 10% loss,
     verify all messages arrive but order is not enforced.

## Security

| ID      | Derived From                                            |
|---------|---------------------------------------------------------|
| R-8.1.5 | [F-8.1.5](../../features/networking/transport-layer.md) |

1. **R-8.1.5** — DTLS Encryption: The engine **SHALL** encrypt all UDP traffic using DTLS 1.3 with
   mandatory AES-GCM encryption, support hardware-accelerated AES-GCM where available, and perform
   periodic key rotation without interrupting active sessions.
   - **Rationale:** Unencrypted UDP packets expose player data and enable tampering; key rotation
     limits the impact window of a compromised key.
   - **Verification:** Integration test: capture encrypted traffic and verify no plaintext payload
     is extractable. Trigger a key rotation during an active session and verify zero packet loss
     during the transition. Benchmark encryption throughput and verify hardware-accelerated AES-GCM
     achieves at least 1 Gbps on supported platforms.

## Packet Management

| ID      | Derived From                                            |
|---------|---------------------------------------------------------|
| R-8.1.6 | [F-8.1.6](../../features/networking/transport-layer.md) |
| R-8.1.7 | [F-8.1.7](../../features/networking/transport-layer.md) |

1. **R-8.1.6** — Packet Fragmentation, Reassembly, and MTU Discovery: The engine **SHALL**
   automatically fragment packets exceeding the path MTU, reassemble fragments on the receiver with
   timeout-based expiration, and perform path MTU discovery at connection startup and periodically
   thereafter, falling back to a 1,200-byte default MTU when discovery is blocked.
   - **Rationale:** Large state snapshots during zone transitions can reach tens of kilobytes and
     must be transmitted without relying on IP-layer fragmentation.
   - **Verification:** Integration test: send a 64 KiB payload over a link with 1,400-byte MTU and
     verify correct reassembly. Simulate a middlebox blocking PMTUD and verify fallback to
     1,200-byte MTU. Verify fragment expiration fires after the configured timeout and frees
     resources.
2. **R-8.1.7** — Bandwidth Estimation and Congestion Control: The engine **SHALL** continuously
   estimate per-connection available bandwidth using acknowledgment timing and packet loss signals,
   and regulate send rates with a congestion controller that provides smooth throughput from 500
   Kbps to 10 Gbps without bursty behavior.
   - **Rationale:** Game traffic requires steady, predictable throughput rather than the bursty
     behavior of TCP congestion control.
   - **Verification:** Integration test: simulate a 1 Mbps link with 2% loss and verify the
     controller converges to within 10% of available bandwidth within 5 seconds. Verify send rate
     does not exhibit sawtooth oscillations exceeding 20% of the average rate. Benchmark on a 10
     Gbps loopback and verify the controller scales to full link capacity.

## Diagnostics

| ID      | Derived From                                            |
|---------|---------------------------------------------------------|
| R-8.1.8 | [F-8.1.8](../../features/networking/transport-layer.md) |

1. **R-8.1.8** — Network Diagnostics and Quality Indicators: The engine **SHALL** expose real-time
   network quality metrics (RTT, packet loss percentage, jitter, bandwidth utilization, connection
   stability score) as ECS resource components, and log network events (spike, timeout, reconnect)
   for post-session analysis.
   - **Rationale:** Developers and players need network quality visibility for adaptive
     quality-of-service logic and troubleshooting connectivity issues.
   - **Verification:** Integration test: introduce 100 ms latency and 5% packet loss on a test link.
     Verify RTT measurement is within 10% of actual. Verify packet loss percentage converges to 5%
     +/-1% within 10 seconds. Verify jitter metric is non-zero under variable delay. Verify spike
     and timeout events appear in the diagnostic log.
