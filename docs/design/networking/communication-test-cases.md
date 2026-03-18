# Communication Framework Test Cases

Companion test cases for [communication.md](communication.md).

## Unit Tests

### TC-8.9.1.1 Channel Create

| # | Requirement |
|---|-------------|
| 1 | R-8.9.1     |
| 2 | R-8.9.1     |
| 3 | R-8.9.4     |

1. **#1** — config=(type=Global, mode=TextAndVoice, max=256)
   - **Expected:** `Ok(channel_id)` where `channel_id.0 > 0`
2. **#2** — config=(type=Party, mode=VoiceOnly, max=5)
   - **Expected:** `Ok(channel_id)` with mode=VoiceOnly
3. **#3** — config=(type=DirectMessage, mode=TextOnly, max=2)
   - **Expected:** `Ok(channel_id)` with max_members=2

### TC-8.9.1.2 Channel Join

| # | Requirement |
|---|-------------|
| 1 | R-8.9.1     |
| 2 | R-8.9.1     |
| 3 | R-8.9.1a    |

1. **#1** — channel exists, user not member, channel not full
   - **Expected:** `Ok(())`, user in members list
2. **#2** — channel exists, user already member
   - **Expected:** `Err(AlreadyMember)`
3. **#3** — channel full (members.len() == max_members)
   - **Expected:** `Err(ChannelFull)`

### TC-8.9.1.3 Channel Leave

| # | Requirement |
|---|-------------|
| 1 | R-8.9.1     |
| 2 | R-8.9.1     |
| 3 | R-8.9.1     |

1. **#1** — user is member of channel
   - **Expected:** `Ok(())`, user removed from members
2. **#2** — user is not member of channel
   - **Expected:** `Err(NotMember)`
3. **#3** — last member leaves, channel transitions to Empty
   - **Expected:** channel.state == Empty

### TC-8.9.1.4 Channel Destroy

| # | Requirement |
|---|-------------|
| 1 | R-8.9.1     |
| 2 | R-8.9.1     |

1. **#1** — admin destroys active channel
   - **Expected:** `Ok(())`, channel.state == Destroyed
2. **#2** — non-admin attempts destroy
   - **Expected:** `Err(PermissionDenied)`

### TC-8.9.2.1 Text Message Send

| # | Requirement |
|---|-------------|
| 1 | R-8.9.2     |
| 2 | R-8.9.2     |
| 3 | R-8.9.5     |

1. **#1** — channel=valid, content="hello", user is member
   - **Expected:** `Ok(message_id)`
2. **#2** — channel=valid, user is not member
   - **Expected:** `Err(NotMember)`
3. **#3** — channel=valid, user is muted
   - **Expected:** `Err(Muted)`

### TC-8.9.2.2 Mention Parser

| # | Requirement |
|---|-------------|
| 1 | R-8.9.2     |
| 2 | R-8.9.2     |
| 3 | R-8.9.2     |

1. **#1** — content="hello @alice how are you"
   - **Expected:** mentions=["alice"]
2. **#2** — content="no mentions here"
   - **Expected:** mentions=[]
3. **#3** — content="@bob @charlie both of you"
   - **Expected:** mentions=["bob", "charlie"]

### TC-8.9.4.1 E2E Key Derivation

| # | Requirement |
|---|-------------|
| 1 | R-8.9.4a    |
| 2 | R-8.9.4a    |

1. **#1** — a_priv + b_pub, b_priv + a_pub
   - **Expected:** Both derive identical shared_secret
2. **#2** — a_priv + c_pub (third party)
   - **Expected:** shared_secret differs from a-b secret

### TC-8.9.4.2 E2E Encrypt and Decrypt

| # | Requirement |
|---|-------------|
| 1 | R-8.9.4a    |
| 2 | R-8.9.4a    |
| 3 | R-8.9.4a    |

1. **#1** — plaintext="secret message", valid shared_secret
   - **Expected:** encrypt then decrypt recovers plaintext
2. **#2** — ciphertext, wrong shared_secret
   - **Expected:** `Err(DecryptionFailed)`
3. **#3** — tampered ciphertext (bit flip)
   - **Expected:** `Err(DecryptionFailed)` (GCM auth tag)

### TC-8.9.5.1 Profanity Filter Clean

| # | Requirement |
|---|-------------|
| 1 | R-8.9.5     |
| 2 | R-8.9.5a    |

1. **#1** — message="hello world", wordlist=["badword"]
   - **Expected:** `FilterResult::Clean`
2. **#2** — message="nice day", wordlist=50000 entries
   - **Expected:** `FilterResult::Clean`

### TC-8.9.5.2 Profanity Filter Match

| # | Requirement |
|---|-------------|
| 1 | R-8.9.5     |
| 2 | R-8.9.5     |

1. **#1** — message="you are a badword", wordlist=["badword"]
   - **Expected:** `FilterResult::Filtered { replaced }` where "badword" is replaced
2. **#2** — message="BADWORD", case-insensitive wordlist
   - **Expected:** `FilterResult::Filtered`

### TC-8.9.5.3 Rate Limiter Allow

| # | Requirement |
|---|-------------|
| 1 | R-8.9.5b    |
| 2 | R-8.9.5b    |

1. **#1** — tokens_per_sec=5, burst=10, 1 call
   - **Expected:** `RateLimitResult::Allow`
2. **#2** — 5 calls in 1 second
   - **Expected:** All return `Allow`

### TC-8.9.5.4 Rate Limiter Throttle

| # | Requirement |
|---|-------------|
| 1 | R-8.9.5b    |
| 2 | R-8.9.5b    |

1. **#1** — tokens_per_sec=5, burst=10, 11 calls in 1 s
   - **Expected:** 11th returns `Throttle { delay_ms }`
2. **#2** — delay_ms > 0 for throttled call
   - **Expected:** Positive delay

### TC-8.9.5.5 Rate Limiter Reject

| # | Requirement |
|---|-------------|
| 1 | R-8.9.5b    |

1. **#1** — tokens_per_sec=5, burst=10, 50 calls in 1 s
   - **Expected:** Calls after burst returns `Reject`

### TC-8.9.5.6 Block List

| # | Requirement |
|---|-------------|
| 1 | R-8.9.5     |
| 2 | R-8.9.5     |

1. **#1** — block(user_b), message from user_b
   - **Expected:** Message hidden from blocker
2. **#2** — unblock(user_b), message from user_b
   - **Expected:** Message delivered to blocker

### TC-8.9.6.1 Spatial Voice Attenuation

| # | Requirement |
|---|-------------|
| 1 | R-8.9.6a    |
| 2 | R-8.9.6a    |
| 3 | R-8.9.6a    |
| 4 | R-8.9.6a    |

1. **#1** — distance=1.0 m, curve=InverseSquared, max=100 m
   - **Expected:** gain = 1.0 (reference distance)
2. **#2** — distance=10.0 m, curve=InverseSquared
   - **Expected:** gain = 1/100 = 0.01
3. **#3** — distance=50.0 m, curve=InverseSquared
   - **Expected:** gain = 1/2500 = 0.0004
4. **#4** — distance=100.0 m, curve=InverseSquared
   - **Expected:** gain = 0.0 (at max distance)

### TC-8.9.7.1 Jitter Buffer Depth Adjust

| # | Requirement |
|---|-------------|
| 1 | R-8.9.7a    |
| 2 | R-8.9.7a    |
| 3 | R-8.9.7a    |

1. **#1** — jitter_estimate=10 ms, current_depth=20 ms
   - **Expected:** depth remains 20 ms (minimum)
2. **#2** — jitter_estimate=80 ms, current_depth=20 ms
   - **Expected:** depth increases toward 80+ ms
3. **#3** — jitter_estimate=5 ms, current_depth=100 ms
   - **Expected:** depth decreases toward 20 ms

### TC-8.9.7.2 Voice Bandwidth Reduction

| # | Requirement |
|---|-------------|
| 1 | R-8.9.7a    |
| 2 | R-8.9.7a    |
| 3 | R-8.9.7a    |
| 4 | R-8.9.7a    |

1. **#1** — budget=Normal
   - **Expected:** bitrate=24 kbps, frame=20 ms
2. **#2** — budget=Constrained
   - **Expected:** bitrate=12 kbps, frame=40 ms
3. **#3** — budget=Severely_Constrained
   - **Expected:** bitrate=6 kbps, frame=60 ms
4. **#4** — budget=Exhausted
   - **Expected:** voice suspended

### TC-8.9.8.1 Editor Bridge Message Relay

| # | Requirement |
|---|-------------|
| 1 | R-8.9.8     |
| 2 | R-8.9.8     |
| 3 | R-8.9.8     |

1. **#1** — editor sends "hello" on bridged channel
   - **Expected:** game client receives "hello"
2. **#2** — game client sends "hi" on bridged channel
   - **Expected:** editor client receives "hi"
3. **#3** — message on non-bridged editor channel
   - **Expected:** game client does not receive

### TC-8.9.8.2 Follow Tracker Update

| # | Requirement |
|---|-------------|
| 1 | R-8.9.8a    |
| 2 | R-8.9.8     |

1. **#1** — follow(ai_agent), agent edits asset
   - **Expected:** `FollowUpdate` received with active_asset
2. **#2** — unfollow(ai_agent), agent edits asset
   - **Expected:** No `FollowUpdate` received

## Integration Tests

### TC-8.9.1.I1 Channel Survives Zone Transition

| # | Requirement |
|---|-------------|
| 1 | R-8.9.1     |
| 2 | R-8.9.2     |

1. **#1** — User in 3 channels; trigger zone transition
   - **Expected:** All 3 memberships intact after transition
2. **#2** — Send message during transition
   - **Expected:** Message delivered after transition completes

### TC-8.9.1.I2 Channel Survives Server Migration

| # | Requirement |
|---|-------------|
| 1 | R-8.9.1     |

1. **#1** — User in 5 channels; trigger server migration
   - **Expected:** All 5 memberships intact after migration

### TC-8.9.2.I1 Text Message End-to-End Delivery

| # | Requirement |
|---|-------------|
| 1 | R-8.9.2     |
| 2 | R-8.9.2     |

1. **#1** — User A sends "hello" on party channel with 5 members
   - **Expected:** All 4 other members receive "hello" with correct author and timestamp
2. **#2** — User A sends "@bob check this"
   - **Expected:** User B (bob) receives a mention notification

### TC-8.9.2.I2 Chat History Load and Search

| # | Requirement |
|---|-------------|
| 1 | R-8.9.2b    |
| 2 | R-8.9.2b    |

1. **#1** — 100 messages in channel; new user joins, requests last 50
   - **Expected:** 50 most recent messages returned in order
2. **#2** — Search "quest reward" across 10,000 messages
   - **Expected:** Results contain matching messages; latency < 200 ms

### TC-8.9.3.I1 Voice Channel Round-Trip

| # | Requirement |
|---|-------------|
| 1 | R-8.9.3     |
| 2 | R-8.9.3     |

1. **#1** — User A joins voice channel, speaks for 5 s
   - **Expected:** User B hears audio; no underruns
2. **#2** — User A stops speaking
   - **Expected:** VAD gates; no packets sent during silence

### TC-8.9.3.I2 Proximity Voice Distance Attenuation

| # | Requirement |
|---|-------------|
| 1 | R-8.9.3     |
| 2 | R-8.9.3     |
| 3 | R-8.9.3     |

1. **#1** — Speaker at 5 m distance in proximity channel
   - **Expected:** Audio audible at moderate volume
2. **#2** — Speaker at 50 m distance
   - **Expected:** Audio significantly attenuated
3. **#3** — Speaker at 100+ m distance
   - **Expected:** Audio inaudible (beyond max distance)

### TC-8.9.4.I1 DM Offline Message Delivery

| # | Requirement |
|---|-------------|
| 1 | R-8.9.4     |
| 2 | R-8.9.4     |

1. **#1** — User A sends DM to offline User B; User B logs in
   - **Expected:** User B receives the queued DM with correct content
2. **#2** — 10 DMs sent while offline
   - **Expected:** All 10 delivered in order on login

### TC-8.9.4.I2 DM Encryption Verification

| # | Requirement |
|---|-------------|
| 1 | R-8.9.4a    |
| 2 | R-8.9.4a    |

1. **#1** — User A sends DM "secret"; capture server-side payload
   - **Expected:** Payload contains only ciphertext; no "secret" in plaintext
2. **#2** — User B receives and decrypts
   - **Expected:** Decrypted content == "secret"

### TC-8.9.5.I1 Moderation Mute Enforcement

| # | Requirement |
|---|-------------|
| 1 | R-8.9.5     |
| 2 | R-8.9.5     |

1. **#1** — Admin mutes User A (text) in channel; User A sends text
   - **Expected:** Message rejected with `Err(Muted)`
2. **#2** — Admin mutes User A (voice); User A transmits audio
   - **Expected:** Voice packets dropped server-side

### TC-8.9.5.I2 Report with Evidence

| # | Requirement |
|---|-------------|
| 1 | R-8.9.5     |
| 2 | R-8.9.5     |

1. **#1** — User A reports User B in channel with 3 recent messages
   - **Expected:** Report stored with 3 message evidence entries, timestamps, and channel ID
2. **#2** — Admin queries reports
   - **Expected:** Report appears with full evidence

### TC-8.9.6.I1 VR Spatial Voice Positioning

| # | Requirement |
|---|-------------|
| 1 | R-8.9.6     |
| 2 | R-8.9.6     |

1. **#1** — Speaker avatar at (10, 1.7, 5), listener at (10, 1.7, 0)
   - **Expected:** Audio source positioned 5 m ahead; HRTF renders center
2. **#2** — Speaker avatar at (15, 1.7, 0), listener facing +Z
   - **Expected:** Audio source positioned to the right; HRTF renders right ear dominant

### TC-8.9.6.I2 VR Virtual Keyboard

| # | Requirement |
|---|-------------|
| 1 | R-8.9.6     |

1. **#1** — User types "hello" on VR virtual keyboard, presses enter
   - **Expected:** Text message "hello" sent on active channel

### TC-8.9.8.I1 Editor-Game Cross-Context Delivery

| # | Requirement |
|---|-------------|
| 1 | R-8.9.8     |
| 2 | R-8.9.8     |

1. **#1** — Editor user sends "fix this" on bridged channel
   - **Expected:** Game client receives "fix this"
2. **#2** — Game client sends "@designer check level 3"
   - **Expected:** Editor user receives mention notification

### TC-8.9.8.I2 Follow AI Agent Live

| # | Requirement |
|---|-------------|
| 1 | R-8.9.8a    |

1. **#1** — Designer follows AI agent; agent modifies an entity
   - **Expected:** Designer receives FollowUpdate with active_asset and edit summary within 200 ms

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

| # | Metric                              | Target        | Requirement |
|---|-------------------------------------|---------------|-------------|
| 1 | All delivered within latency target | 100% delivery | R-8.9.NF2   |
| 2 | Total memory                        | < 16 MiB      | R-8.9.NF1   |

1. **1** — 10,000 msg/s across 100 channels, 50 members each
2. **2** — Memory usage with 64 active channels
