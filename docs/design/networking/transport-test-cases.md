# Networking Transport Layer Test Cases

Companion test cases for [transport.md](transport.md).

## Unit Tests

### TC-8.1.3.1 Sequence Number Wrap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | seq_a=65535 (u16::MAX), seq_b=0 | `is_newer_than(seq_b, seq_a)` returns true | R-8.1.3 |
| 2 | seq_a=32767, seq_b=32768 | `is_newer_than(seq_b, seq_a)` returns true | R-8.1.3 |
| 3 | seq_a=0, seq_b=65535 | `is_newer_than(seq_b, seq_a)` returns false | R-8.1.3 |

### TC-8.1.3.2 SACK Bitfield Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Send packets 0-49, drop packets 10,20,30,40,45 | SACK bitfield marks 5 gaps, 45 acked | R-8.1.3 |
| 2 | Retransmit dropped packets, verify new SACK | All 50 marked as acked | R-8.1.3 |

### TC-8.1.8.1 RTT Estimator Convergence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Feed 50 RTT samples of 100 ms each | SRTT converges to 100 ms +/- 5 ms | R-8.1.8 |
| 2 | Feed 50 RTT samples of 100 ms, then 50 of 200 ms | SRTT converges to 200 ms within 20 samples | R-8.1.8 |

### TC-8.1.3.3 Fast Retransmit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Drop packet N, deliver N+1, N+2, N+3 | Packet N retransmitted after 3rd SACK (not waiting for RTO) | R-8.1.3 |

### TC-8.1.3.4 RTO Backoff

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Initial RTO=200ms, drop packet repeatedly 5 times | RTO values: 200, 400, 800, 1600, 2000 (capped) | R-8.1.3 |

### TC-8.1.4.1 Unreliable Sequenced Drop Stale

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Deliver packets in order: seq=1, seq=3, seq=2 | Packet seq=2 dropped (stale), seq=1 and seq=3 delivered | R-8.1.4 |

### TC-8.1.4.2 Unreliable Unordered Delivers All

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Deliver packets in order: seq=3, seq=1, seq=2 | All three delivered in receive order (3, 1, 2) | R-8.1.4 |

### TC-8.1.4.3 Reliable Unordered

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Send 100 messages with 10% simulated loss on reliable unordered channel | All 100 arrive; delivery order not enforced | R-8.1.4 |

### TC-8.1.6.1 Fragment Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 32 KiB payload, MTU=1200 bytes | Fragmented into ceil(32768/1200) = 28 fragments; reassembly produces original 32 KiB | R-8.1.6 |

### TC-8.1.6.2 Fragment Timeout

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Send 27 of 28 fragments, omit last | Fragment group expires after configured timeout; resources freed | R-8.1.6 |

### TC-8.1.6.3 MTU Fallback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Simulate 3 consecutive failed MTU probes | MTU falls back to 1200-byte default | R-8.1.6 |

### TC-8.1.7.1 Congestion Slow Start

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Start with cwnd=initial, no loss for 5 RTTs | cwnd doubles each RTT during slow start | R-8.1.7 |

### TC-8.1.7.2 Congestion Loss Recovery

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger loss event during steady state | cwnd reduces to 70% of pre-loss value, enters Recovery state | R-8.1.7 |

### TC-8.1.7.3 Send Pacing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Send 100 packets in one tick | Packets spread across RTT window, not bursted in single batch | R-8.1.7 |

### TC-8.1.1.1 Cookie Generation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Generate cookies for addr_A and addr_B | Different cookie values (unique per address) | R-8.1.1 |
| 2 | Cookie generated at t=0, validated at t=expiry+1 | Rejected (expired) | R-8.1.1 |

### TC-8.1.1.2 Cookie Validation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Valid cookie for addr_A, validated from addr_A | Accepted | R-8.1.1 |
| 2 | Expired cookie | Rejected | R-8.1.1 |
| 3 | Forged cookie (random bytes) | Rejected | R-8.1.1 |

### TC-8.1.1.3 Replay Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Send AuthRequest, capture it, replay same bytes | Replay rejected by server | R-8.1.1 |

### TC-8.1.2.1 Connection State Transitions

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Walk: Disconnected -> Connecting -> Connected -> Active | All transitions succeed | R-8.1.2 |
| 2 | Attempt Active -> Connecting (illegal) | Transition rejected | R-8.1.2 |
| 3 | Active -> Migrating -> Active | Migration transition succeeds | R-8.1.2 |

### TC-8.NFR.7.1 Timing Wheel O(1)

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Schedule 10,000 timeouts, measure tick() | tick() time constant regardless of entry count | R-8.NFR.7 |
| 2 | Schedule 1,000 vs 10,000 entries | tick() time difference < 10% | R-8.NFR.7 |

### TC-8.1.5.1 DTLS Encrypt Decrypt

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Encrypt 1200-byte payload, decrypt | Decrypted output matches original exactly | R-8.1.5 |

### TC-8.1.5.2 DTLS Tamper Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Encrypt payload, flip 1 bit in ciphertext, decrypt | Auth tag verification fails | R-8.1.5 |

### TC-8.1.5.3 Nonce Replay Rejection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Receive packet with nonce=42, replay same nonce=42 | Second packet rejected (nonce already seen) | R-8.1.5 |

### TC-8.1.8.2 Stats EWMA Convergence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Feed stable 5% packet loss for 100 samples | EWMA converges to 5% +/- 1% | R-8.1.8 |

## Integration Tests

### TC-8.1.1.I1 Handshake 10K Concurrent

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10,000 concurrent handshake attempts | All complete within 2 s | R-8.1.1 |

### TC-8.1.1.I2 Handshake Replay Rejection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Capture valid handshake, replay to server | Server rejects replayed handshake | R-8.1.1 |

### TC-8.1.1.I3 Flood 50K Connections

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 50,000 connection attempts/sec flood | Server remains responsive to legitimate clients | R-8.1.1 |

### TC-8.1.3.I1 Reliable 100K Messages Lossy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100,000 numbered messages, 10% loss, 50 ms latency | All arrive in order, zero loss | R-8.1.3 |

### TC-8.NFR.12.I1 Reliable 15 Percent Loss

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 15% loss, 200 ms RTT, send 10,000 messages | 100% delivery, retransmit latency within 1.5x RTT | R-8.NFR.12 |

### TC-8.1.2.I1 10K Connections Keepalive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Establish 10,000 connections, measure keepalive cost | CPU cost below 0.1 ms per tick | R-8.1.2 |

### TC-8.1.6.I1 64K Payload Fragmentation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 64 KiB payload over 1400-byte MTU link | Correct reassembly, payload matches original | R-8.1.6 |

### TC-8.1.6.I2 PMTUD Blocked Fallback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Block all PMTUD probes | Fallback to 1200-byte MTU, communication continues | R-8.1.6 |

### TC-8.1.7.I1 Congestion 1Mbps Link

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1 Mbps link, 2% loss | Convergence to within 10% of capacity in 5 s; no sawtooth > 20% of average | R-8.1.7 |

### TC-8.1.7.I2 Congestion 10Gbps Loopback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10 Gbps loopback test | Controller scales to full link capacity | R-8.1.7 |

### TC-8.1.5.I1 DTLS Key Rotation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger key rotation during active traffic (1000 packets in flight) | Zero packet loss during rotation | R-8.1.5 |

### TC-8.NFR.11.I1 DTLS HW AES Throughput

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Benchmark AES-GCM encrypt/decrypt with hardware acceleration | >= 1 Gbps throughput | R-8.NFR.11 |

### TC-8.1.8.I1 Diagnostics Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Inject 100 ms latency and 5% loss for 30 s | RTT within 10% of 100 ms; loss converges to 5% +/- 1% within 10 s | R-8.1.8 |

### TC-8.1.2.I2 Mobile Profile

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enable mobile profile | Heartbeat=5 s, timeout=60 s, initial_rate=500 Kbps | R-8.1.2 |

### TC-8.1.2.I3 Graceful Disconnect

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Initiate graceful disconnect | Disconnect ack received, all resources released on both sides | R-8.1.2 |

### TC-8.1.2.I4 Migration Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger migration: Active -> Migrating -> Active | Zero message loss during transition | R-8.1.2 |

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
