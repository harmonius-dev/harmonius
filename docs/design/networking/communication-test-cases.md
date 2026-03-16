# Communication Framework Test Cases

Companion test cases for [communication.md](communication.md).

## Unit Tests

### TC-8.9.1.1 Channel Create

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | config=(type=Global, mode=TextAndVoice, max=256) | `Ok(channel_id)` where `channel_id.0 > 0` | R-8.9.1 |
| 2 | config=(type=Party, mode=VoiceOnly, max=5) | `Ok(channel_id)` with mode=VoiceOnly | R-8.9.1 |
| 3 | config=(type=DirectMessage, mode=TextOnly, max=2) | `Ok(channel_id)` with max_members=2 | R-8.9.4 |

### TC-8.9.1.2 Channel Join

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | channel exists, user not member, channel not full | `Ok(())`, user in members list | R-8.9.1 |
| 2 | channel exists, user already member | `Err(AlreadyMember)` | R-8.9.1 |
| 3 | channel full (members.len() == max_members) | `Err(ChannelFull)` | R-8.9.1a |

### TC-8.9.1.3 Channel Leave

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | user is member of channel | `Ok(())`, user removed from members | R-8.9.1 |
| 2 | user is not member of channel | `Err(NotMember)` | R-8.9.1 |
| 3 | last member leaves, channel transitions to Empty | channel.state == Empty | R-8.9.1 |

### TC-8.9.1.4 Channel Destroy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | admin destroys active channel | `Ok(())`, channel.state == Destroyed | R-8.9.1 |
| 2 | non-admin attempts destroy | `Err(PermissionDenied)` | R-8.9.1 |

### TC-8.9.2.1 Text Message Send

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | channel=valid, content="hello", user is member | `Ok(message_id)` | R-8.9.2 |
| 2 | channel=valid, user is not member | `Err(NotMember)` | R-8.9.2 |
| 3 | channel=valid, user is muted | `Err(Muted)` | R-8.9.5 |

### TC-8.9.2.2 Mention Parser

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | content="hello @alice how are you" | mentions=["alice"] | R-8.9.2 |
| 2 | content="no mentions here" | mentions=[] | R-8.9.2 |
| 3 | content="@bob @charlie both of you" | mentions=["bob", "charlie"] | R-8.9.2 |

### TC-8.9.4.1 E2E Key Derivation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | a_priv + b_pub, b_priv + a_pub | Both derive identical shared_secret | R-8.9.4a |
| 2 | a_priv + c_pub (third party) | shared_secret differs from a-b secret | R-8.9.4a |

### TC-8.9.4.2 E2E Encrypt and Decrypt

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | plaintext="secret message", valid shared_secret | encrypt then decrypt recovers plaintext | R-8.9.4a |
| 2 | ciphertext, wrong shared_secret | `Err(DecryptionFailed)` | R-8.9.4a |
| 3 | tampered ciphertext (bit flip) | `Err(DecryptionFailed)` (GCM auth tag) | R-8.9.4a |

### TC-8.9.5.1 Profanity Filter Clean

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | message="hello world", wordlist=["badword"] | `FilterResult::Clean` | R-8.9.5 |
| 2 | message="nice day", wordlist=50000 entries | `FilterResult::Clean` | R-8.9.5a |

### TC-8.9.5.2 Profanity Filter Match

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | message="you are a badword", wordlist=["badword"] | `FilterResult::Filtered { replaced }` where "badword" is replaced | R-8.9.5 |
| 2 | message="BADWORD", case-insensitive wordlist | `FilterResult::Filtered` | R-8.9.5 |

### TC-8.9.5.3 Rate Limiter Allow

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | tokens_per_sec=5, burst=10, 1 call | `RateLimitResult::Allow` | R-8.9.5b |
| 2 | 5 calls in 1 second | All return `Allow` | R-8.9.5b |

### TC-8.9.5.4 Rate Limiter Throttle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | tokens_per_sec=5, burst=10, 11 calls in 1 s | 11th returns `Throttle { delay_ms }` | R-8.9.5b |
| 2 | delay_ms > 0 for throttled call | Positive delay | R-8.9.5b |

### TC-8.9.5.5 Rate Limiter Reject

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | tokens_per_sec=5, burst=10, 50 calls in 1 s | Calls after burst returns `Reject` | R-8.9.5b |

### TC-8.9.5.6 Block List

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | block(user_b), message from user_b | Message hidden from blocker | R-8.9.5 |
| 2 | unblock(user_b), message from user_b | Message delivered to blocker | R-8.9.5 |

### TC-8.9.6.1 Spatial Voice Attenuation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | distance=1.0 m, curve=InverseSquared, max=100 m | gain = 1.0 (reference distance) | R-8.9.6a |
| 2 | distance=10.0 m, curve=InverseSquared | gain = 1/100 = 0.01 | R-8.9.6a |
| 3 | distance=50.0 m, curve=InverseSquared | gain = 1/2500 = 0.0004 | R-8.9.6a |
| 4 | distance=100.0 m, curve=InverseSquared | gain = 0.0 (at max distance) | R-8.9.6a |

### TC-8.9.7.1 Jitter Buffer Depth Adjust

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | jitter_estimate=10 ms, current_depth=20 ms | depth remains 20 ms (minimum) | R-8.9.7a |
| 2 | jitter_estimate=80 ms, current_depth=20 ms | depth increases toward 80+ ms | R-8.9.7a |
| 3 | jitter_estimate=5 ms, current_depth=100 ms | depth decreases toward 20 ms | R-8.9.7a |

### TC-8.9.7.2 Voice Bandwidth Reduction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | budget=Normal | bitrate=24 kbps, frame=20 ms | R-8.9.7a |
| 2 | budget=Constrained | bitrate=12 kbps, frame=40 ms | R-8.9.7a |
| 3 | budget=Severely_Constrained | bitrate=6 kbps, frame=60 ms | R-8.9.7a |
| 4 | budget=Exhausted | voice suspended | R-8.9.7a |

### TC-8.9.8.1 Editor Bridge Message Relay

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | editor sends "hello" on bridged channel | game client receives "hello" | R-8.9.8 |
| 2 | game client sends "hi" on bridged channel | editor client receives "hi" | R-8.9.8 |
| 3 | message on non-bridged editor channel | game client does not receive | R-8.9.8 |

### TC-8.9.8.2 Follow Tracker Update

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | follow(ai_agent), agent edits asset | `FollowUpdate` received with active_asset | R-8.9.8a |
| 2 | unfollow(ai_agent), agent edits asset | No `FollowUpdate` received | R-8.9.8 |

## Integration Tests

### TC-8.9.1.I1 Channel Survives Zone Transition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | User in 3 channels; trigger zone transition | All 3 memberships intact after transition | R-8.9.1 |
| 2 | Send message during transition | Message delivered after transition completes | R-8.9.2 |

### TC-8.9.1.I2 Channel Survives Server Migration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | User in 5 channels; trigger server migration | All 5 memberships intact after migration | R-8.9.1 |

### TC-8.9.2.I1 Text Message End-to-End Delivery

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | User A sends "hello" on party channel with 5 members | All 4 other members receive "hello" with correct author and timestamp | R-8.9.2 |
| 2 | User A sends "@bob check this" | User B (bob) receives a mention notification | R-8.9.2 |

### TC-8.9.2.I2 Chat History Load and Search

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100 messages in channel; new user joins, requests last 50 | 50 most recent messages returned in order | R-8.9.2b |
| 2 | Search "quest reward" across 10,000 messages | Results contain matching messages; latency < 200 ms | R-8.9.2b |

### TC-8.9.3.I1 Voice Channel Round-Trip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | User A joins voice channel, speaks for 5 s | User B hears audio; no underruns | R-8.9.3 |
| 2 | User A stops speaking | VAD gates; no packets sent during silence | R-8.9.3 |

### TC-8.9.3.I2 Proximity Voice Distance Attenuation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Speaker at 5 m distance in proximity channel | Audio audible at moderate volume | R-8.9.3 |
| 2 | Speaker at 50 m distance | Audio significantly attenuated | R-8.9.3 |
| 3 | Speaker at 100+ m distance | Audio inaudible (beyond max distance) | R-8.9.3 |

### TC-8.9.4.I1 DM Offline Message Delivery

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | User A sends DM to offline User B; User B logs in | User B receives the queued DM with correct content | R-8.9.4 |
| 2 | 10 DMs sent while offline | All 10 delivered in order on login | R-8.9.4 |

### TC-8.9.4.I2 DM Encryption Verification

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | User A sends DM "secret"; capture server-side payload | Payload contains only ciphertext; no "secret" in plaintext | R-8.9.4a |
| 2 | User B receives and decrypts | Decrypted content == "secret" | R-8.9.4a |

### TC-8.9.5.I1 Moderation Mute Enforcement

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Admin mutes User A (text) in channel; User A sends text | Message rejected with `Err(Muted)` | R-8.9.5 |
| 2 | Admin mutes User A (voice); User A transmits audio | Voice packets dropped server-side | R-8.9.5 |

### TC-8.9.5.I2 Report with Evidence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | User A reports User B in channel with 3 recent messages | Report stored with 3 message evidence entries, timestamps, and channel ID | R-8.9.5 |
| 2 | Admin queries reports | Report appears with full evidence | R-8.9.5 |

### TC-8.9.6.I1 VR Spatial Voice Positioning

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Speaker avatar at (10, 1.7, 5), listener at (10, 1.7, 0) | Audio source positioned 5 m ahead; HRTF renders center | R-8.9.6 |
| 2 | Speaker avatar at (15, 1.7, 0), listener facing +Z | Audio source positioned to the right; HRTF renders right ear dominant | R-8.9.6 |

### TC-8.9.6.I2 VR Virtual Keyboard

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | User types "hello" on VR virtual keyboard, presses enter | Text message "hello" sent on active channel | R-8.9.6 |

### TC-8.9.8.I1 Editor-Game Cross-Context Delivery

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Editor user sends "fix this" on bridged channel | Game client receives "fix this" | R-8.9.8 |
| 2 | Game client sends "@designer check level 3" | Editor user receives mention notification | R-8.9.8 |

### TC-8.9.8.I2 Follow AI Agent Live

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Designer follows AI agent; agent modifies an entity | Designer receives FollowUpdate with active_asset and edit summary within 200 ms | R-8.9.8a |

## Benchmarks

### TC-8.9.2.B1 Text Delivery Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1,000 messages to 50-member channel, 20 ms RTT | p99 delivery latency | < 100 ms | R-8.9.2a |
| 2 | 10,000 messages to 10-member channel, 20 ms RTT | p99 delivery latency | < 100 ms | R-8.9.2a |

### TC-8.9.2.B2 Chat History Search

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full-text search across 10M messages | Query latency | < 200 ms | R-8.9.2b |
| 2 | Full-text search across 1M messages | Query latency | < 100 ms | R-8.9.2b |

### TC-8.9.3.B1 Voice Mouth-to-Ear Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Voice loopback, 50 ms simulated RTT | Mouth-to-ear latency | < 150 ms | R-8.9.3a |
| 2 | Voice loopback, LAN (< 1 ms RTT) | Mouth-to-ear latency | < 50 ms | R-8.9.3b |

### TC-8.9.5.B1 Profanity Filter Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10,000 messages, 50,000-entry word list | p99 filter time | < 1 ms/msg | R-8.9.5a |
| 2 | 10,000 messages, 100-entry word list | p99 filter time | < 0.1 ms/msg | R-8.9.5a |

### TC-8.9.7.B1 Voice Stream Mixing

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Decode + mix 16 Opus streams | Audio thread time | < 2 ms/buffer | R-8.9.3b |
| 2 | Decode + mix 32 Opus streams | Audio thread time | < 4 ms/buffer | R-8.9.7 |

### TC-8.9.7.B2 Jitter Buffer Convergence

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Jitter jumps from 5 ms to 80 ms | Time to stable depth | < 5 s | R-8.9.7a |
| 2 | Jitter drops from 80 ms to 5 ms | Time to stable depth | < 5 s | R-8.9.7a |

### TC-8.9.8.B1 Bridge Overhead

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Message within single context vs. across bridge | Additional latency | < 10 ms | R-8.9.8a |

### TC-8.9.NF.B1 Server Channel Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10,000 msg/s across 100 channels, 50 members each | All delivered within latency target | 100% delivery | R-8.9.NF2 |
| 2 | Memory usage with 64 active channels | Total memory | < 16 MiB | R-8.9.NF1 |
