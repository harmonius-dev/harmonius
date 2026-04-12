# Networking ↔ Audio Integration Test Cases

All tests are CI-runnable. They use a real `OpusEncoder`/`OpusDecoder`, a real `JitterBuffer`, real
crossbeam-channel MPSC queues, a real `quinn-proto` (sans-IO) endpoint loopback, and a fake
`AudioBackend` that records PCM output instead of driving hardware. No mocking libraries are used.
Per-test setup seeds `Rng` with `0x1234` and uses a fixed 20 ms frame size (960 samples at 48 kHz).

## Integration Tests

### Positive Cases

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-4.3.1.1 | Voice round-trip | Encode+send+recv+decode | PCM matches src | IR-4.3.1 |
| TC-IR-4.3.1.2 | Unreliable delivery | 100 voice packets | >= 95 received | IR-4.3.1 |
| TC-IR-4.3.2.1 | Opus encode 24 kbps | 20 ms PCM frame | Frame < 60 bytes | IR-4.3.2 |
| TC-IR-4.3.2.2 | Opus PLC on loss | 1 packet dropped | PLC fills gap | IR-4.3.2 |
| TC-IR-4.3.3.1 | Jitter reorders | Packets 3,1,2 arrive | Dequeue 1,2,3 | IR-4.3.3 |
| TC-IR-4.3.3.2 | Jitter adapts | Jitter spike 40 ms | Buffer ~60 ms | IR-4.3.3 |
| TC-IR-4.3.4.1 | Pos replicated | Move remote player | Source updates | IR-4.3.4 |
| TC-IR-4.3.4.2 | Spatial pan | Remote left | Audio pans left | IR-4.3.4 |
| TC-IR-4.3.5.1 | VAD gates silence | No voice input | No TX | IR-4.3.5 |
| TC-IR-4.3.5.2 | VAD passes speech | Voice detected | TX occurs | IR-4.3.5 |
| TC-IR-4.3.6.1 | Join party chan | `JoinRequest(Party(1))` | Server acks | IR-4.3.6 |
| TC-IR-4.3.6.2 | Leave channel | `LeaveRequest(Party(1))` | Audio stops | IR-4.3.6 |
| TC-IR-4.3.6.3 | Multi-channel | Proximity + Party | Both mix | IR-4.3.6 |
| TC-IR-4.3.7.1 | Proximity in range | Remote 20 m | Packets rx | IR-4.3.7 |
| TC-IR-4.3.7.2 | Proximity far | Remote 50 m | No TX | IR-4.3.7 |
| TC-IR-4.3.2.3 | AEC removes echo | Speaker ref + echo | Residual < -30 dB | IR-4.3.2 |

### Negative / Failure Mode Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-4.3.1.3 | Auth tag mismatch | Forged `auth_tag` | Dropped, metric++ | IR-4.3.1 |
| TC-IR-4.3.1.4 | Inbound MPSC full | 1025 packets / callback | Excess dropped, PLC | IR-4.3.1 |
| TC-IR-4.3.3.3 | Sustained starvation | No packets for 600 ms | Pause after 500 ms | IR-4.3.3 |
| TC-IR-4.3.3.4 | Starvation recovery | Resume after pause | New sequence plays | IR-4.3.3 |
| TC-IR-4.3.4.3 | Spatial MPSC full | 513 updates / frame | Oldest dropped | IR-4.3.4 |
| TC-IR-4.3.2.4 | Opus decode error | Corrupt `opus_data` | Skip frame, log | IR-4.3.2 |
| TC-IR-4.3.6.4 | Channel RPC timeout | Server silent | Retry exp backoff | IR-4.3.6 |
| TC-IR-4.3.6.5 | Channel join rejected | `NotAuthorized` | UI error shown | IR-4.3.6 |
| TC-IR-4.3.2.5 | Frame pool exhausted | 65 in-flight frames | Oldest recycled | IR-4.3.2 |
| TC-IR-4.3.1.5 | Router hot-path lookup | 256 channels | No HashMap alloc | IR-4.3.1 |

1. **TC-IR-4.3.1.3** -- Client builds a `VoicePacket` with an incorrect `auth_tag`. Server HMAC
   validation fails, the packet is dropped, and `voice_auth_failures` increments by 1. No PCM is
   emitted and no panic occurs.
2. **TC-IR-4.3.1.4** -- Main thread enqueues 1025 voice packets into the 1024-entry inbound MPSC
   before the audio thread drains. The 1025th `try_send` returns `Err`; the drop counts as packet
   loss and the receiver emits PLC for that sequence.
3. **TC-IR-4.3.1.5** -- Construct a `VoiceChannelRouter` with 64 `Party` and 64 `Raid` channels.
   Call `lookup` 1000 times. Assert via `dhat` that no `HashMap` allocation occurs; only `Vec`
   binary searches execute.
4. **TC-IR-4.3.2.3** -- Feed the `AcousticEchoCanceller` a speaker reference signal and a mic input
   equal to the reference convolved with a short impulse response. After processing, residual energy
   in the mic buffer is < -30 dB relative to the reference signal.
5. **TC-IR-4.3.2.4** -- Corrupt the `opus_data` bytes. `OpusDecoder::decode` returns an error which
   is logged; the frame is skipped and playback continues with PLC.
6. **TC-IR-4.3.2.5** -- Acquire 64 frame handles from `OpusFramePool` without releasing. The 65th
   `acquire` returns `None`; the caller forcibly releases the oldest handle and increments
   `frame_pool_overrun`.
7. **TC-IR-4.3.3.3** -- Push no packets to the `JitterBuffer` for 600 ms. `starvation_ms` grows past
   500 ms; after the threshold the voice is paused and `voice_starvation` increments.
8. **TC-IR-4.3.3.4** -- Following TC-IR-4.3.3.3, push a new packet with sequence 42. The buffer
   clears `starvation_ms`, resumes playback at sequence 42, and no panic occurs.
9. **TC-IR-4.3.4.3** -- Send 513 `VoiceSpatialCommand::UpdatePosition` messages in one frame. The
   513th message overwrites the oldest pending update (oldest-drop backpressure). The audio thread
   sees 512 updates and no panic occurs.
10. **TC-IR-4.3.6.3** -- Client joins both `VoiceChannelId::Proximity` and
    `VoiceChannelId::Party(1)` concurrently. Voice packets from two remote speakers (one per
    channel) are decoded simultaneously on the audio thread and mixed onto the `BusId::Voice` bus
    without starvation or dropped frames.
11. **TC-IR-4.3.6.4** -- Server does not respond to a `JoinRequest`. Client retries after 100 ms,
    200 ms, 400 ms exponential backoff until `JoinResponse` arrives or the session ends.
12. **TC-IR-4.3.6.5** -- Server responds `JoinResponse { result: NotAuthorized }`. The UI layer
    surfaces an error banner; no audio flows from the channel.

## Benchmarks

Reference hardware: Apple M2 Pro (10-core, 3.5 GHz), 32 GB LPDDR5. Targets are measured on this
reference CPU; CI asserts each target as a hard gate.

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-4.3.1.B1 | End-to-end voice latency | < 150 ms | IR-4.3.1 |
| TC-IR-4.3.2.B1 | Opus encode 20 ms frame | < 0.5 ms | IR-4.3.2 |
| TC-IR-4.3.2.B2 | AEC NLMS process 20 ms frame | < 0.3 ms | IR-4.3.2 |
| TC-IR-4.3.3.B1 | Jitter buffer dequeue | < 10 us | IR-4.3.3 |
| TC-IR-4.3.4.B1 | 32 simultaneous voice streams | < 1.0 ms audio cb | IR-4.3.4 |
| TC-IR-4.3.1.B2 | MPSC inbound send p99 | < 5 us | IR-4.3.1 |
| TC-IR-4.3.1.B3 | Router lookup 256 channels | < 1 us | IR-4.3.1 |

Benchmarks run via `cargo bench` under the `networking_audio_integration` harness. Each benchmark
records wall-clock and p99 latency, then asserts its target as a hard CI gate (not advisory).
