# 8.1 — Transport Layer

## Connection Management

### F-8.1.1 Connection Handshake and Authentication

Establish secure client-server connections using a multi-phase handshake that authenticates player
identity via session tokens before allowing any gameplay traffic. The handshake must resist replay
attacks, man-in-the-middle attacks, and connection flooding to protect MMO servers handling
thousands of concurrent connection attempts.

- **Requirements:** R-8.1.1
- **Dependencies:** None
- **Platform notes:** None

### F-8.1.2 Connection Lifecycle Management

Track connection state (connecting, authenticated, active, migrating, disconnected) with
configurable timeouts, heartbeats, and graceful disconnect sequences. Must support thousands of
simultaneous connections per server process with O(1) per-connection overhead for keepalive and
timeout detection, essential for MMO server density.

- **Requirements:** R-8.1.2
- **Dependencies:** F-8.1.1
- **Platform notes:** None

## Channel Architecture

### F-8.1.3 Reliable Ordered Channel

Provide a TCP-like reliable ordered delivery channel over UDP with selective acknowledgment
(SACK), configurable retransmission timers, and congestion-aware send windows. Used for
critical gameplay events such as inventory changes, quest updates, chat messages, and auction
house transactions where ordering and delivery guarantees are mandatory.

- **Requirements:** R-8.1.3
- **Dependencies:** F-8.1.2
- **Platform notes:** None

### F-8.1.4 Unreliable and Unordered Channels

Provide unreliable unordered, unreliable sequenced, and reliable unordered channel modes to
match the delivery semantics each data type requires. Position updates use unreliable sequenced
(drop stale), voice uses unreliable unordered (lowest latency), and entity spawns use reliable
unordered (deliver but order does not matter).

- **Requirements:** R-8.1.4
- **Dependencies:** F-8.1.2
- **Platform notes:** None

## Security

### F-8.1.5 DTLS Encryption

Encrypt all UDP traffic using DTLS 1.3 to prevent packet sniffing and tampering. Encryption
must be mandatory for all gameplay channels, with hardware-accelerated AES-GCM on platforms
that support it. Key rotation must occur periodically without interrupting active sessions,
and the implementation must handle packet reordering inherent to UDP without breaking the
cipher state.

- **Requirements:** R-8.1.5
- **Dependencies:** F-8.1.1
- **Platform notes:** Use platform TLS libraries where available (Schannel on Windows,
  Security.framework on macOS, OpenSSL/rustls on Linux) for FIPS compliance and hardware
  acceleration.

## Packet Management

### F-8.1.6 Packet Fragmentation, Reassembly, and MTU Discovery

Automatically fragment outbound packets that exceed the path MTU and reassemble them on the
receiving end with timeout-based fragment expiration. Perform path MTU discovery at connection
startup and periodically thereafter to maximize payload per packet and minimize fragmentation.
Critical for large state snapshots during zone transitions or initial world loads where payloads
can reach tens of kilobytes.

- **Requirements:** R-8.1.6
- **Dependencies:** F-8.1.2
- **Platform notes:** PMTUD behavior varies by OS and network; fall back to conservative 1200-byte
  default MTU when discovery is blocked by middleboxes.

### F-8.1.7 Bandwidth Estimation and Congestion Control

Continuously estimate available bandwidth per connection using acknowledgment timing and packet
loss signals, and regulate send rates with a game-oriented congestion controller. The controller
must avoid the bursty behavior of TCP congestion control and instead provide smooth, predictable
throughput suitable for real-time gameplay. Must scale from mobile connections (~500 Kbps) to
high-bandwidth datacenter links between server nodes (~10 Gbps).

- **Requirements:** R-8.1.7
- **Dependencies:** F-8.1.3, F-8.1.4
- **Platform notes:** None
