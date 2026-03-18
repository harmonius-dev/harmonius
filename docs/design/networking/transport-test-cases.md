# Networking Transport Layer Test Cases

Companion test cases for [transport.md](transport.md).

## Unit Tests

### TC-8.1.3.1 Sequence Number Wrap

| # | Requirement |
|---|-------------|
| 1 | R-8.1.3     |
| 2 | R-8.1.3     |
| 3 | R-8.1.3     |

1. **#1** — seq_a=65535 (u16::MAX), seq_b=0
   - **Expected:** `is_newer_than(seq_b, seq_a)` returns true
2. **#2** — seq_a=32767, seq_b=32768
   - **Expected:** `is_newer_than(seq_b, seq_a)` returns true
3. **#3** — seq_a=0, seq_b=65535
   - **Expected:** `is_newer_than(seq_b, seq_a)` returns false

### TC-8.1.3.2 SACK Bitfield Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-8.1.3     |
| 2 | R-8.1.3     |

1. **#1** — Send packets 0-49, drop packets 10,20,30,40,45
   - **Expected:** SACK bitfield marks 5 gaps, 45 acked
2. **#2** — Retransmit dropped packets, verify new SACK
   - **Expected:** All 50 marked as acked

### TC-8.1.8.1 RTT Estimator Convergence

| # | Requirement |
|---|-------------|
| 1 | R-8.1.8     |
| 2 | R-8.1.8     |

1. **#1** — Feed 50 RTT samples of 100 ms each
   - **Expected:** SRTT converges to 100 ms +/- 5 ms
2. **#2** — Feed 50 RTT samples of 100 ms, then 50 of 200 ms
   - **Expected:** SRTT converges to 200 ms within 20 samples

### TC-8.1.3.3 Fast Retransmit

| # | Requirement |
|---|-------------|
| 1 | R-8.1.3     |

1. **#1** — Drop packet N, deliver N+1, N+2, N+3
   - **Expected:** Packet N retransmitted after 3rd SACK (not waiting for RTO)

### TC-8.1.3.4 RTO Backoff

| # | Requirement |
|---|-------------|
| 1 | R-8.1.3     |

1. **#1** — Initial RTO=200ms, drop packet repeatedly 5 times
   - **Expected:** RTO values: 200, 400, 800, 1600, 2000 (capped)

### TC-8.1.4.1 Unreliable Sequenced Drop Stale

| # | Requirement |
|---|-------------|
| 1 | R-8.1.4     |

1. **#1** — Deliver packets in order: seq=1, seq=3, seq=2
   - **Expected:** Packet seq=2 dropped (stale), seq=1 and seq=3 delivered

### TC-8.1.4.2 Unreliable Unordered Delivers All

| # | Requirement |
|---|-------------|
| 1 | R-8.1.4     |

1. **#1** — Deliver packets in order: seq=3, seq=1, seq=2
   - **Expected:** All three delivered in receive order (3, 1, 2)

### TC-8.1.4.3 Reliable Unordered

| # | Requirement |
|---|-------------|
| 1 | R-8.1.4     |

1. **#1** — Send 100 messages with 10% simulated loss on reliable unordered channel
   - **Expected:** All 100 arrive; delivery order not enforced

### TC-8.1.6.1 Fragment Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-8.1.6     |

1. **#1** — 32 KiB payload, MTU=1200 bytes
   - **Expected:** Fragmented into ceil(32768/1200) = 28 fragments; reassembly produces original 32
     KiB

### TC-8.1.6.2 Fragment Timeout

| # | Requirement |
|---|-------------|
| 1 | R-8.1.6     |

1. **#1** — Send 27 of 28 fragments, omit last
   - **Expected:** Fragment group expires after configured timeout; resources freed

### TC-8.1.6.3 MTU Fallback

| # | Requirement |
|---|-------------|
| 1 | R-8.1.6     |

1. **#1** — Simulate 3 consecutive failed MTU probes
   - **Expected:** MTU falls back to 1200-byte default

### TC-8.1.7.1 Congestion Slow Start

| # | Requirement |
|---|-------------|
| 1 | R-8.1.7     |

1. **#1** — Start with cwnd=initial, no loss for 5 RTTs
   - **Expected:** cwnd doubles each RTT during slow start

### TC-8.1.7.2 Congestion Loss Recovery

| # | Requirement |
|---|-------------|
| 1 | R-8.1.7     |

1. **#1** — Trigger loss event during steady state
   - **Expected:** cwnd reduces to 70% of pre-loss value, enters Recovery state

### TC-8.1.7.3 Send Pacing

| # | Requirement |
|---|-------------|
| 1 | R-8.1.7     |

1. **#1** — Send 100 packets in one tick
   - **Expected:** Packets spread across RTT window, not bursted in single batch

### TC-8.1.1.1 Cookie Generation

| # | Requirement |
|---|-------------|
| 1 | R-8.1.1     |
| 2 | R-8.1.1     |

1. **#1** — Generate cookies for addr_A and addr_B
   - **Expected:** Different cookie values (unique per address)
2. **#2** — Cookie generated at t=0, validated at t=expiry+1
   - **Expected:** Rejected (expired)

### TC-8.1.1.2 Cookie Validation

| # | Requirement |
|---|-------------|
| 1 | R-8.1.1     |
| 2 | R-8.1.1     |
| 3 | R-8.1.1     |

1. **#1** — Valid cookie for addr_A, validated from addr_A
   - **Expected:** Accepted
2. **#2** — Expired cookie
   - **Expected:** Rejected
3. **#3** — Forged cookie (random bytes)
   - **Expected:** Rejected

### TC-8.1.1.3 Replay Detection

| # | Requirement |
|---|-------------|
| 1 | R-8.1.1     |

1. **#1** — Send AuthRequest, capture it, replay same bytes
   - **Expected:** Replay rejected by server

### TC-8.1.2.1 Connection State Transitions

| # | Requirement |
|---|-------------|
| 1 | R-8.1.2     |
| 2 | R-8.1.2     |
| 3 | R-8.1.2     |

1. **#1** — Walk: Disconnected -> Connecting -> Connected -> Active
   - **Expected:** All transitions succeed
2. **#2** — Attempt Active -> Connecting (illegal)
   - **Expected:** Transition rejected
3. **#3** — Active -> Migrating -> Active
   - **Expected:** Migration transition succeeds

### TC-8.NFR.7.1 Timing Wheel O(1)

| # | Requirement |
|---|-------------|
| 1 | R-8.NFR.7   |
| 2 | R-8.NFR.7   |

1. **#1** — Schedule 10,000 timeouts, measure tick()
   - **Expected:** tick() time constant regardless of entry count
2. **#2** — Schedule 1,000 vs 10,000 entries
   - **Expected:** tick() time difference < 10%

### TC-8.1.5.1 DTLS Encrypt Decrypt

| # | Requirement |
|---|-------------|
| 1 | R-8.1.5     |

1. **#1** — Encrypt 1200-byte payload, decrypt
   - **Expected:** Decrypted output matches original exactly

### TC-8.1.5.2 DTLS Tamper Detection

| # | Requirement |
|---|-------------|
| 1 | R-8.1.5     |

1. **#1** — Encrypt payload, flip 1 bit in ciphertext, decrypt
   - **Expected:** Auth tag verification fails

### TC-8.1.5.3 Nonce Replay Rejection

| # | Requirement |
|---|-------------|
| 1 | R-8.1.5     |

1. **#1** — Receive packet with nonce=42, replay same nonce=42
   - **Expected:** Second packet rejected (nonce already seen)

### TC-8.1.8.2 Stats EWMA Convergence

| # | Requirement |
|---|-------------|
| 1 | R-8.1.8     |

1. **#1** — Feed stable 5% packet loss for 100 samples
   - **Expected:** EWMA converges to 5% +/- 1%

## Integration Tests

### TC-8.1.1.I1 Handshake 10K Concurrent

| # | Requirement |
|---|-------------|
| 1 | R-8.1.1     |

1. **#1** — 10,000 concurrent handshake attempts
   - **Expected:** All complete within 2 s

### TC-8.1.1.I2 Handshake Replay Rejection

| # | Requirement |
|---|-------------|
| 1 | R-8.1.1     |

1. **#1** — Capture valid handshake, replay to server
   - **Expected:** Server rejects replayed handshake

### TC-8.1.1.I3 Flood 50K Connections

| # | Requirement |
|---|-------------|
| 1 | R-8.1.1     |

1. **#1** — 50,000 connection attempts/sec flood
   - **Expected:** Server remains responsive to legitimate clients

### TC-8.1.3.I1 Reliable 100K Messages Lossy

| # | Requirement |
|---|-------------|
| 1 | R-8.1.3     |

1. **#1** — 100,000 numbered messages, 10% loss, 50 ms latency
   - **Expected:** All arrive in order, zero loss

### TC-8.NFR.12.I1 Reliable 15 Percent Loss

| # | Requirement |
|---|-------------|
| 1 | R-8.NFR.12  |

1. **#1** — 15% loss, 200 ms RTT, send 10,000 messages
   - **Expected:** 100% delivery, retransmit latency within 1.5x RTT

### TC-8.1.2.I1 10K Connections Keepalive

| # | Requirement |
|---|-------------|
| 1 | R-8.1.2     |

1. **#1** — Establish 10,000 connections, measure keepalive cost
   - **Expected:** CPU cost below 0.1 ms per tick

### TC-8.1.6.I1 64K Payload Fragmentation

| # | Requirement |
|---|-------------|
| 1 | R-8.1.6     |

1. **#1** — 64 KiB payload over 1400-byte MTU link
   - **Expected:** Correct reassembly, payload matches original

### TC-8.1.6.I2 PMTUD Blocked Fallback

| # | Requirement |
|---|-------------|
| 1 | R-8.1.6     |

1. **#1** — Block all PMTUD probes
   - **Expected:** Fallback to 1200-byte MTU, communication continues

### TC-8.1.7.I1 Congestion 1Mbps Link

| # | Requirement |
|---|-------------|
| 1 | R-8.1.7     |

1. **#1** — 1 Mbps link, 2% loss
   - **Expected:** Convergence to within 10% of capacity in 5 s; no sawtooth > 20% of average

### TC-8.1.7.I2 Congestion 10Gbps Loopback

| # | Requirement |
|---|-------------|
| 1 | R-8.1.7     |

1. **#1** — 10 Gbps loopback test
   - **Expected:** Controller scales to full link capacity

### TC-8.1.5.I1 DTLS Key Rotation

| # | Requirement |
|---|-------------|
| 1 | R-8.1.5     |

1. **#1** — Trigger key rotation during active traffic (1000 packets in flight)
   - **Expected:** Zero packet loss during rotation

### TC-8.NFR.11.I1 DTLS HW AES Throughput

| # | Requirement |
|---|-------------|
| 1 | R-8.NFR.11  |

1. **#1** — Benchmark AES-GCM encrypt/decrypt with hardware acceleration
   - **Expected:** >= 1 Gbps throughput

### TC-8.1.8.I1 Diagnostics Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-8.1.8     |

1. **#1** — Inject 100 ms latency and 5% loss for 30 s
   - **Expected:** RTT within 10% of 100 ms; loss converges to 5% +/- 1% within 10 s

### TC-8.1.2.I2 Mobile Profile

| # | Requirement |
|---|-------------|
| 1 | R-8.1.2     |

1. **#1** — Enable mobile profile
   - **Expected:** Heartbeat=5 s, timeout=60 s, initial_rate=500 Kbps

### TC-8.1.2.I3 Graceful Disconnect

| # | Requirement |
|---|-------------|
| 1 | R-8.1.2     |

1. **#1** — Initiate graceful disconnect
   - **Expected:** Disconnect ack received, all resources released on both sides

### TC-8.1.2.I4 Migration Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-8.1.2     |

1. **#1** — Trigger migration: Active -> Migrating -> Active
   - **Expected:** Zero message loss during transition

## Benchmarks

### TC-8.1.1.B1 Handshake Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single handshake on LAN | Latency | < 10 ms | R-8.1.1 |

### TC-8.NFR.7.B1 Per-Connection Keepalive Overhead

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Keepalive processing per connection per tick | CPU time | < 10 ns | R-8.NFR.7 |

### TC-8.1.3.B1 Packet Encode Decode

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Encode + decode single packet | Latency | < 100 ns per packet | R-8.1.3 |

### TC-8.NFR.11.B1 DTLS Encrypt Decrypt Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | AES-GCM encrypt + decrypt 1200-byte packet | Latency | < 200 ns per packet | R-8.NFR.11 |

### TC-8.1.3.B2 Reliable Channel Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Sustained reliable channel transfer | Throughput | >= 100 Mbps | R-8.1.3 |

### TC-8.1.7.B1 Congestion Controller Convergence

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Converge from initial cwnd to link capacity | Time to 90% capacity | < 5 s | R-8.1.7 |

### TC-8.1.6.B1 Fragment Reassembly

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Reassemble 64 KiB from fragments | Total time | < 50 us | R-8.1.6 |

### TC-8.1.B1 Net Systems Frame Time

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | All net systems at 10,000 connections | Total frame time | < 0.5 ms | R-8.1.8 |

### TC-8.NFR.7.B2 Timing Wheel Tick

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Timing wheel with 10,000 entries | tick() time | < 1 us | R-8.NFR.7 |
