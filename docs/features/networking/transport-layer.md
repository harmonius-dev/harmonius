# 8.1 — Transport Layer

## Connection Management

| ID      | Feature                                 | Requirements |
|---------|-----------------------------------------|--------------|
| F-8.1.1 | Connection Handshake and Authentication | R-8.1.1      |
| F-8.1.2 | Connection Lifecycle Management         | R-8.1.2      |

1. **F-8.1.1** — Establish secure client-server connections using a multi-phase handshake that
   authenticates player identity via session tokens before allowing any gameplay traffic. The
   handshake must resist replay attacks, man-in-the-middle attacks, and connection flooding to
   protect MMO servers handling thousands of concurrent connection attempts.
   - **Platform:** Mobile clients maintain a single server connection. Handshake must tolerate
     high-latency mobile networks and Wi-Fi/cellular handoffs.
2. **F-8.1.2** — Track connection state (connecting, authenticated, active, migrating, disconnected)
   with configurable timeouts, heartbeats, and graceful disconnect sequences. Must support thousands
   of simultaneous connections per server process with O(1) per-connection overhead for keepalive
   and timeout detection, essential for MMO server density.
   - **Deps:** F-8.1.1
   - **Platform:** Mobile uses longer heartbeat intervals (5 s vs. 1 s) and more lenient timeout
     windows to tolerate cellular network variability.

## Channel Architecture

| ID      | Feature                           | Requirements |
|---------|-----------------------------------|--------------|
| F-8.1.3 | Reliable Ordered Channel          | R-8.1.3      |
| F-8.1.4 | Unreliable and Unordered Channels | R-8.1.4      |

1. **F-8.1.3** — Provide a TCP-like reliable ordered delivery channel over UDP with selective
   acknowledgment (SACK), configurable retransmission timers, and congestion-aware send windows.
   Used for critical gameplay events such as inventory changes, quest updates, chat messages, and
   auction house transactions where ordering and delivery guarantees are mandatory.
   - **Deps:** F-8.1.2
2. **F-8.1.4** — Provide unreliable unordered, unreliable sequenced, and reliable unordered channel
   modes to match the delivery semantics each data type requires. Position updates use unreliable
   sequenced (drop stale), voice uses unreliable unordered (lowest latency), and entity spawns use
   reliable unordered (deliver but order does not matter).
   - **Deps:** F-8.1.2

## Security

| ID      | Feature         | Requirements |
|---------|-----------------|--------------|
| F-8.1.5 | DTLS Encryption | R-8.1.5      |

1. **F-8.1.5** — Encrypt all UDP traffic using DTLS 1.3 to prevent packet sniffing and tampering.
   Encryption must be mandatory for all gameplay channels, with hardware-accelerated AES-GCM on
   platforms that support it. Key rotation must occur periodically without interrupting active
   sessions, and the implementation must handle packet reordering inherent to UDP without breaking
   the cipher state.
   - **Deps:** F-8.1.1
   - **Platform:** Use platform TLS libraries where available (Schannel on Windows,
     Security.framework on macOS, OpenSSL/rustls on Linux) for FIPS compliance and hardware
     acceleration.

## Packet Management

| ID      | Feature                                             | Requirements |
|---------|-----------------------------------------------------|--------------|
| F-8.1.6 | Packet Fragmentation, Reassembly, and MTU Discovery | R-8.1.6      |
| F-8.1.7 | Bandwidth Estimation and Congestion Control         | R-8.1.7      |

1. **F-8.1.6** — Automatically fragment outbound packets that exceed the path MTU and reassemble
   them on the receiving end with timeout-based fragment expiration. Perform path MTU discovery at
   connection startup and periodically thereafter to maximize payload per packet and minimize
   fragmentation. Critical for large state snapshots during zone transitions or initial world loads
   where payloads can reach tens of kilobytes.
   - **Deps:** F-8.1.2
   - **Platform:** PMTUD behavior varies by OS and network; fall back to conservative 1200-byte
     default MTU when discovery is blocked by middleboxes.
2. **F-8.1.7** — Continuously estimate available bandwidth per connection using acknowledgment
   timing and packet loss signals, and regulate send rates with a game-oriented congestion
   controller. The controller must avoid the bursty behavior of TCP congestion control and instead
   provide smooth, predictable throughput suitable for real-time gameplay. Must scale from mobile
   connections (~500 Kbps) to high-bandwidth datacenter links between server nodes (~10 Gbps).
   - **Deps:** F-8.1.3, F-8.1.4
   - **Platform:** Mobile defaults to a conservative initial send rate (~500 Kbps) and uses more
     aggressive back-off on loss to respect metered data plans.

## Diagnostics

| ID      | Feature                                    | Requirements |
|---------|--------------------------------------------|--------------|
| F-8.1.8 | Network Diagnostics and Quality Indicators | R-8.1.8      |

1. **F-8.1.8** — Real-time network quality metrics exposed to both the engine and the player-facing
   UI. Tracks round-trip time (RTT), packet loss percentage, jitter, bandwidth utilization, and
   connection stability score. A configurable HUD widget (F-10.3.1) displays ping, connection
   quality bars, and packet loss warnings. The diagnostic system logs network events (spike,
   timeout, reconnect) for post-session analysis. Developers access raw metrics through ECS resource
   components for custom quality-of-service logic such as adaptive tick rates or graceful
   degradation under poor conditions.
   - **Deps:** F-8.1.1, F-8.1.4 (Bandwidth Estimation)
   - **Platform:** Mobile HUD shows simplified connection indicator (icon only) to save screen
     space. Cellular vs. Wi-Fi network type is reported on mobile.
