# DSP Effects, Adaptive Music, and Voice Chat Test Cases

Companion test cases for [dsp-music.md](dsp-music.md).

## Unit Tests

### TC-5.3.1.1 Biquad Low-Pass Response

| # | Requirement |
|---|-------------|
| 1 | R-5.3.1     |
| 2 | R-5.3.1     |

1. **#1** — White noise through LP at 1 kHz cutoff
   - **Expected:** -3 dB at 1 kHz, -12 dB/oct rolloff within 0.5 dB
2. **#2** — White noise through LP at 200 Hz cutoff
   - **Expected:** -3 dB at 200 Hz within 0.5 dB

### TC-5.3.1.2 Biquad High-Pass Response

| # | Requirement |
|---|-------------|
| 1 | R-5.3.1     |

1. **#1** — White noise through HP at 1 kHz cutoff
   - **Expected:** -3 dB at 1 kHz, +12 dB/oct rise within 0.5 dB

### TC-5.3.1.3 Biquad Band-Pass Response

| # | Requirement |
|---|-------------|
| 1 | R-5.3.1     |

1. **#1** — White noise through BP at 1 kHz, Q=2
   - **Expected:** 0 dB at center, -6 dB at bandwidth edges within 0.5 dB

### TC-5.3.1.4 Biquad Notch Response

| # | Requirement |
|---|-------------|
| 1 | R-5.3.1     |

1. **#1** — White noise through notch at 1 kHz
   - **Expected:** Center frequency attenuated by >= 30 dB

### TC-5.3.1.5 Biquad No Zipper

| # | Requirement |
|---|-------------|
| 1 | R-5.3.1     |

1. **#1** — Sweep cutoff 200 Hz to 8 kHz over 100 ms
   - **Expected:** Zero clicks detected via zero-crossing analysis

### TC-5.3.1.6 Biquad Stability

| # | Requirement |
|---|-------------|
| 1 | R-5.3.1     |
| 2 | R-5.3.1     |

1. **#1** — Cutoff=20 Hz, Q=100
   - **Expected:** No NaN or Inf in output
2. **#2** — Cutoff=Nyquist (24 kHz at 48 kHz SR), Q=0.1
   - **Expected:** No NaN or Inf in output

### TC-5.3.2.1 EQ 8-Band Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-5.3.2     |

1. **#1** — 8-band EQ, each band +6 dB, flat input spectrum
   - **Expected:** Each band gain within 0.5 dB of +6 dB

### TC-5.3.2.2 EQ Band Count Per Tier

| # | Requirement |
|---|-------------|
| 1 | R-5.3.2     |
| 2 | R-5.3.2     |
| 3 | R-5.3.2     |

1. **#1** — Mobile config
   - **Expected:** Max bands = 4
2. **#2** — Switch config
   - **Expected:** Max bands = 6
3. **#3** — Desktop config
   - **Expected:** Max bands = 8

### TC-5.3.3.1 FDN Decay Time

| # | Requirement |
|---|-------------|
| 1 | R-5.3.3     |

1. **#1** — Impulse input, target RT60=2.0s
   - **Expected:** Measured RT60 within 10% of 2.0s (1.8-2.2s)

### TC-5.3.3.2 FDN Delay Count Per Tier

| # | Requirement |
|---|-------------|
| 1 | R-5.3.3     |
| 2 | R-5.3.3     |
| 3 | R-5.3.3     |

1. **#1** — Mobile config
   - **Expected:** delay_lines = 4
2. **#2** — Switch config
   - **Expected:** delay_lines = 8
3. **#3** — Desktop config
   - **Expected:** delay_lines = 16

### TC-5.3.4.1 Convolution Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-5.3.4     |

1. **#1** — Known IR convolved with impulse
   - **Expected:** Output within -60 dB of analytical reference

### TC-5.3.4.2 Convolution Latency

| # | Requirement |
|---|-------------|
| 1 | R-5.3.4     |

1. **#1** — Convolution reverb, measure output latency
   - **Expected:** Latency <= 1 buffer period (512 samples at 48 kHz = 10.67 ms)

### TC-5.3.4.3 Convolution Unavailable Mobile

| # | Requirement |
|---|-------------|
| 1 | R-5.3.4     |

1. **#1** — Mobile config, request convolution reverb
   - **Expected:** Unavailable (returns error or fallback to FDN)

### TC-5.3.5.1 Compressor Curve

| # | Requirement |
|---|-------------|
| 1 | R-5.3.5     |

1. **#1** — Signal 12 dB above threshold, ratio=4:1
   - **Expected:** Output within 0.5 dB of expected (3 dB above threshold)

### TC-5.3.5.2 Limiter Clipping

| # | Requirement |
|---|-------------|
| 1 | R-5.3.5     |
| 2 | R-5.3.5     |

1. **#1** — Signal 6 dB above 0 dBFS
   - **Expected:** Output peak <= 0 dBFS
2. **#2** — Signal 12 dB above 0 dBFS
   - **Expected:** Output peak <= 0 dBFS

### TC-5.3.6.1 Delay Echo Timing

| # | Requirement |
|---|-------------|
| 1 | R-5.3.6     |

1. **#1** — 500 ms delay, 50% feedback
   - **Expected:** Echoes at 500 ms intervals, 6 dB decay per echo within 0.5 dB

### TC-5.3.6.2 Chorus Modulation

| # | Requirement |
|---|-------------|
| 1 | R-5.3.6     |

1. **#1** — 440 Hz sine through chorus
   - **Expected:** Measurable pitch modulation sidebands in spectrum

### TC-5.3.7.1 Pitch Shift Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-5.3.7     |
| 2 | R-5.3.7     |

1. **#1** — 440 Hz sine shifted +12 semitones
   - **Expected:** Output = 880 Hz +/- 1%, artifacts < -40 dB
2. **#2** — 440 Hz sine shifted -12 semitones
   - **Expected:** Output = 220 Hz +/- 1%

### TC-5.3.7.2 Pitch Duration Preserved

| # | Requirement |
|---|-------------|
| 1 | R-5.3.7     |

1. **#1** — 1.0s input, pitch shift applied
   - **Expected:** Output duration = 1.0s within 1 ms

### TC-5.3.8.1 Custom Node Gain

| # | Requirement |
|---|-------------|
| 1 | R-5.3.8     |

1. **#1** — Register custom +6 dB gain node, process signal
   - **Expected:** Output = input + 6 dB within 0.1 dB

### TC-5.3.8.2 Custom Node Removal

| # | Requirement |
|---|-------------|
| 1 | R-5.3.8     |

1. **#1** — Remove custom node at runtime
   - **Expected:** Bus output reverts to pre-insert signal

### TC-5.4.4.1 Beat Clock Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-5.4.4     |

1. **#1** — 120 BPM, 4/4 time, 10 seconds
   - **Expected:** Exactly 20 beat events and 5 bar events

### TC-5.4.4.2 Beat Clock Tempo Change

| # | Requirement |
|---|-------------|
| 1 | R-5.4.4     |

1. **#1** — Change from 120 BPM to 140 BPM mid-playback
   - **Expected:** Beat intervals after change within 1 ms of 140 BPM spacing

### TC-5.4.1.1 Stem Sync

| # | Requirement |
|---|-------------|
| 1 | R-5.4.1     |

1. **#1** — 4-stem music cue, 60 seconds
   - **Expected:** Cross-correlation peak offset = 0 samples between all stem pairs

### TC-5.4.1.2 Stem Fade Smooth

| # | Requirement |
|---|-------------|
| 1 | R-5.4.1     |

1. **#1** — Fade stem gain 1.0 to 0.0 over 500 ms
   - **Expected:** Zero clicks in output (zero-crossing analysis)

### TC-5.4.2.1 Bar-Quantized Transition

| # | Requirement |
|---|-------------|
| 1 | R-5.4.2     |

1. **#1** — Trigger segment transition mid-bar
   - **Expected:** Transition executes at next bar boundary within +/- 1 sample

### TC-5.4.5.1 Stinger Cooldown

| # | Requirement |
|---|-------------|
| 1 | R-5.4.5     |

1. **#1** — Trigger stinger, trigger again within cooldown
   - **Expected:** Second stinger suppressed (only first plays)

### TC-5.4.5.2 Stinger Priority

| # | Requirement |
|---|-------------|
| 1 | R-5.4.5     |

1. **#1** — Low-priority stinger playing, trigger high-priority
   - **Expected:** High-priority stinger ducks low-priority

### TC-5.4.6.1 Playlist Non-Repeat

| # | Requirement |
|---|-------------|
| 1 | R-5.4.6     |

1. **#1** — Shuffle playlist, 20 selections
   - **Expected:** No consecutive repeated track

### TC-5.4.6.2 Playlist Weighted

| # | Requirement |
|---|-------------|
| 1 | R-5.4.6     |

1. **#1** — One track weighted 10x vs others, 1000 selections
   - **Expected:** Weighted track selected proportionally within 10%

### TC-5.4.7.1 Intensity Clamping

| # | Requirement |
|---|-------------|
| 1 | R-5.4.7     |
| 2 | R-5.4.7     |
| 3 | R-5.4.7     |

1. **#1** — Set intensity = -0.5
   - **Expected:** Clamped to 0.0
2. **#2** — Set intensity = 1.5
   - **Expected:** Clamped to 1.0
3. **#3** — Set intensity = 0.7
   - **Expected:** Value = 0.7 (unchanged)

### TC-5.5.1.1 Opus Encode Decode

| # | Requirement |
|---|-------------|
| 1 | R-5.5.1     |

1. **#1** — 10 s speech at 24 kbps, round-trip
   - **Expected:** PESQ score > 3.0

### TC-5.5.2.1 Jitter Buffer Adaptive

| # | Requirement |
|---|-------------|
| 1 | R-5.5.2     |

1. **#1** — 50 ms jitter, 5% packet loss, 60 s
   - **Expected:** Jitter buffer depth < 80 ms

### TC-5.5.2.2 PLC No Silence Gap

| # | Requirement |
|---|-------------|
| 1 | R-5.5.2     |

1. **#1** — Drop 1 packet during speech
   - **Expected:** PLC fills gap, no silence > 1 Opus frame (20 ms)

### TC-5.5.3.1 VAD Gates Silence

| # | Requirement |
|---|-------------|
| 1 | R-5.5.3     |

1. **#1** — 10 s silence then 5 s speech
   - **Expected:** 0 packets during silence, speech onset < 20 ms

### TC-5.5.3.2 Noise Suppression SNR

| # | Requirement |
|---|-------------|
| 1 | R-5.5.3     |

1. **#1** — Speech + keyboard noise at 0 dB SNR
   - **Expected:** Output SNR improvement >= 20 dB

### TC-5.5.9.1 AEC ERLE

| # | Requirement |
|---|-------------|
| 1 | R-5.5.9     |

1. **#1** — Reference playback through speakers, mic capture
   - **Expected:** ERLE > 30 dB

### TC-5.5.9.2 AEC Reconvergence

| # | Requirement |
|---|-------------|
| 1 | R-5.5.9     |

1. **#1** — Change room acoustics mid-session
   - **Expected:** AEC re-convergence < 2 s

### TC-5.5.5.1 Viseme Timing

| # | Requirement |
|---|-------------|
| 1 | R-5.5.5     |

1. **#1** — Known phoneme content processed
   - **Expected:** Viseme timestamps within 30 ms of reference

### TC-5.5.6.1 Dialogue Priority

| # | Requirement |
|---|-------------|
| 1 | R-5.5.6     |

1. **#1** — Bark playing, trigger critical dialogue line
   - **Expected:** Bark interrupted, critical line plays

### TC-5.5.6.2 Dialogue No Overlap

| # | Requirement |
|---|-------------|
| 1 | R-5.5.6     |

1. **#1** — Queue 5 barks sequentially
   - **Expected:** All play sequentially with no overlap

## Integration Tests

### TC-5.3.NF1.I1 Full DSP Chain 64 Voices

| # | Requirement |
|---|-------------|
| 1 | R-5.3.NF1   |

1. **#1** — 4-insert DSP chain on 64 voices, 10K callbacks
   - **Expected:** p99 latency < 1 us/sample

### TC-5.4.NF1.I1 Music Zone Transition

| # | Requirement |
|---|-------------|
| 1 | R-5.4.NF1   |

1. **#1** — Trigger bar-quantized transition at random point
   - **Expected:** Switch at next bar within +/- 1 sample, no clicks

### TC-5.5.NF2.I1 Voice 32 Streams

| # | Requirement |
|---|-------------|
| 1 | R-5.5.NF2   |

1. **#1** — Decode and mix 32 Opus streams at 24 kbps for 60 s
   - **Expected:** Zero audio underruns

### TC-5.5.NF1.I1 Voice End-to-End Latency

| # | Requirement |
|---|-------------|
| 1 | R-5.5.NF1   |

1. **#1** — Loopback capture-to-output, 50 ms RTT
   - **Expected:** Total latency < 150 ms

### TC-5.4.7.I1 Intensity Drives All Systems

| # | Requirement |
|---|-------------|
| 1 | R-5.4.7     |

1. **#1** — Ramp intensity 0.0 to 1.0 over 10 s
   - **Expected:** Stems, segment selection, stinger probability all respond

### TC-5.5.1.I1 Platform Mic Capture

| # | Requirement |
|---|-------------|
| 1 | R-5.5.1     |
| 2 | R-5.5.1     |
| 3 | R-5.5.1     |

1. **#1** — Windows, mic capture
   - **Expected:** WASAPI capture active
2. **#2** — macOS, mic capture
   - **Expected:** CoreAudio capture active
3. **#3** — Linux, mic capture
   - **Expected:** PipeWire/ALSA capture active

### TC-5.5.8.I1 Channel Isolation

| # | Requirement |
|---|-------------|
| 1 | R-5.5.8     |

1. **#1** — Create proximity, party, raid channels
   - **Expected:** Each has independent volume and routing

### TC-5.5.6.I1 Subtitle Sync

| # | Requirement |
|---|-------------|
| 1 | R-5.5.6     |

1. **#1** — Play VO with subtitles
   - **Expected:** Subtitle events within 16 ms of VO playback

### TC-5.5.8.I2 Proximity Attenuation

| # | Requirement |
|---|-------------|
| 1 | R-5.5.8     |

1. **#1** — Proximity voice channel, source at 50 m
   - **Expected:** Distance attenuation applied via shared spatial index

### TC-5.5.4.I1 TTS Platform APIs

| # | Requirement |
|---|-------------|
| 1 | R-5.5.4     |
| 2 | R-5.5.4     |
| 3 | R-5.5.4     |

1. **#1** — Windows, TTS request
   - **Expected:** Uses SAPI
2. **#2** — macOS, TTS request
   - **Expected:** Uses AVSpeechSynthesizer
3. **#3** — Linux, TTS request
   - **Expected:** Uses Speech Dispatcher

## Benchmarks

### TC-5.3.NF1.B1 DSP Chain Per Voice

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 4-insert chain, 48 kHz | Per-sample cost | < 1 us | R-5.3.NF1 |

### TC-5.3.3.B1 FDN Reverb Per Instance

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single FDN reverb instance | CPU usage | < 5% of one core | US-5.3.3.4 |

### TC-5.3.4.B1 Convolution Reverb Per Instance

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single convolution reverb instance | CPU usage | < 10% of one core | F-5.3.4 |

### TC-5.4.4.B1 Beat Clock Tick Per Buffer

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Beat clock update per audio buffer | CPU time | < 1 us | US-5.4.4.10 |

### TC-5.4.NF1.B1 Music Transition Resolution

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Music segment transition processing | CPU time | < 10 us | R-5.4.NF1 |

### TC-5.5.1.B1 Opus Encode Per Frame

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single Opus frame encode at 24 kbps | CPU time | < 500 us | F-5.5.1 |

### TC-5.5.1.B2 Opus Decode Per Frame

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single Opus frame decode | CPU time | < 200 us | F-5.5.1 |

### TC-5.5.9.B1 AEC Processing Per Buffer

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | AEC processing for one audio buffer | CPU time | < 1 ms | R-5.5.9 |

### TC-5.5.5.B1 Viseme Generation Per Frame

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Viseme generation for one audio frame | CPU time | < 100 us | F-5.5.5 |

### TC-5.5.3.B1 Noise Suppression Per Frame

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Noise suppression for one audio frame | CPU time | < 500 us | F-5.5.3 |
