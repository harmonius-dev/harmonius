# Networking ↔ Audio Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-4.3.1.1 | Voice packet round-trip | Encode + send + recv + decode | Audio matches source | IR-4.3.1 |
| TC-IR-4.3.1.2 | Unreliable channel delivery | Send 100 voice packets | >= 95 received | IR-4.3.1 |
| TC-IR-4.3.2.1 | Opus encode at 24 kbps | 20 ms PCM frame | Opus frame < 60 bytes | IR-4.3.2 |
| TC-IR-4.3.2.2 | Opus PLC on lost packet | 1 packet dropped | PLC generates fill audio | IR-4.3.2 |
| TC-IR-4.3.3.1 | Jitter buffer reorders | Packets 3,1,2 arrive | Dequeue order 1,2,3 | IR-4.3.3 |
| TC-IR-4.3.3.2 | Jitter buffer adapts depth | Jitter spikes to 40 ms | Buffer grows to ~60 ms | IR-4.3.3 |
| TC-IR-4.3.4.1 | Spatial position replicated | Move remote player | AudioSource position updates | IR-4.3.4 |
| TC-IR-4.3.4.2 | Spatial audio pans correctly | Remote left of listener | Audio pans left | IR-4.3.4 |
| TC-IR-4.3.5.1 | VAD gates silence | No voice input | No packets transmitted | IR-4.3.5 |
| TC-IR-4.3.5.2 | VAD passes speech | Voice detected | Packets transmitted | IR-4.3.5 |
| TC-IR-4.3.6.1 | Join party voice channel | RPC JoinChannel(Party(1)) | Server ACKs, audio flows | IR-4.3.6 |
| TC-IR-4.3.6.2 | Leave voice channel | RPC LeaveChannel(Party(1)) | Audio stops from channel | IR-4.3.6 |
| TC-IR-4.3.7.1 | Proximity voice in range | Remote within 20 m | Voice packets received | IR-4.3.7 |
| TC-IR-4.3.7.2 | Proximity voice out of range | Remote beyond 50 m | No voice packets sent | IR-4.3.7 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-4.3.1.B1 | End-to-end voice latency | < 150 ms | IR-4.3.1 |
| TC-IR-4.3.2.B1 | Opus encode 20 ms frame | < 0.5 ms | IR-4.3.2 |
| TC-IR-4.3.3.B1 | Jitter buffer dequeue | < 10 us | IR-4.3.3 |
| TC-IR-4.3.4.B1 | 32 simultaneous voice streams | < 0.5 ms audio thread | IR-4.3.4 |
