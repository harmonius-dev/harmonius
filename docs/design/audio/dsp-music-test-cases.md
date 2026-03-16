# DSP Effects, Adaptive Music, and Voice Chat Test Cases

Companion test cases for [dsp-music.md](dsp-music.md).

## Unit Tests

### TC-5.3.1.1 Biquad Low-Pass Response

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | White noise through LP at 1 kHz cutoff | -3 dB at 1 kHz, -12 dB/oct rolloff within 0.5 dB | R-5.3.1 |
| 2 | White noise through LP at 200 Hz cutoff | -3 dB at 200 Hz within 0.5 dB | R-5.3.1 |

### TC-5.3.1.2 Biquad High-Pass Response

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | White noise through HP at 1 kHz cutoff | -3 dB at 1 kHz, +12 dB/oct rise within 0.5 dB | R-5.3.1 |

### TC-5.3.1.3 Biquad Band-Pass Response

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | White noise through BP at 1 kHz, Q=2 | 0 dB at center, -6 dB at bandwidth edges within 0.5 dB | R-5.3.1 |

### TC-5.3.1.4 Biquad Notch Response

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | White noise through notch at 1 kHz | Center frequency attenuated by >= 30 dB | R-5.3.1 |

### TC-5.3.1.5 Biquad No Zipper

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sweep cutoff 200 Hz to 8 kHz over 100 ms | Zero clicks detected via zero-crossing analysis | R-5.3.1 |

### TC-5.3.1.6 Biquad Stability

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cutoff=20 Hz, Q=100 | No NaN or Inf in output | R-5.3.1 |
| 2 | Cutoff=Nyquist (24 kHz at 48 kHz SR), Q=0.1 | No NaN or Inf in output | R-5.3.1 |

### TC-5.3.2.1 EQ 8-Band Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 8-band EQ, each band +6 dB, flat input spectrum | Each band gain within 0.5 dB of +6 dB | R-5.3.2 |

### TC-5.3.2.2 EQ Band Count Per Tier

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile config | Max bands = 4 | R-5.3.2 |
| 2 | Switch config | Max bands = 6 | R-5.3.2 |
| 3 | Desktop config | Max bands = 8 | R-5.3.2 |

### TC-5.3.3.1 FDN Decay Time

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Impulse input, target RT60=2.0s | Measured RT60 within 10% of 2.0s (1.8-2.2s) | R-5.3.3 |

### TC-5.3.3.2 FDN Delay Count Per Tier

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile config | delay_lines = 4 | R-5.3.3 |
| 2 | Switch config | delay_lines = 8 | R-5.3.3 |
| 3 | Desktop config | delay_lines = 16 | R-5.3.3 |

### TC-5.3.4.1 Convolution Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Known IR convolved with impulse | Output within -60 dB of analytical reference | R-5.3.4 |

### TC-5.3.4.2 Convolution Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Convolution reverb, measure output latency | Latency <= 1 buffer period (512 samples at 48 kHz = 10.67 ms) | R-5.3.4 |

### TC-5.3.4.3 Convolution Unavailable Mobile

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile config, request convolution reverb | Unavailable (returns error or fallback to FDN) | R-5.3.4 |

### TC-5.3.5.1 Compressor Curve

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Signal 12 dB above threshold, ratio=4:1 | Output within 0.5 dB of expected (3 dB above threshold) | R-5.3.5 |

### TC-5.3.5.2 Limiter Clipping

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Signal 6 dB above 0 dBFS | Output peak <= 0 dBFS | R-5.3.5 |
| 2 | Signal 12 dB above 0 dBFS | Output peak <= 0 dBFS | R-5.3.5 |

### TC-5.3.6.1 Delay Echo Timing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 500 ms delay, 50% feedback | Echoes at 500 ms intervals, 6 dB decay per echo within 0.5 dB | R-5.3.6 |

### TC-5.3.6.2 Chorus Modulation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 440 Hz sine through chorus | Measurable pitch modulation sidebands in spectrum | R-5.3.6 |

### TC-5.3.7.1 Pitch Shift Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 440 Hz sine shifted +12 semitones | Output = 880 Hz +/- 1%, artifacts < -40 dB | R-5.3.7 |
| 2 | 440 Hz sine shifted -12 semitones | Output = 220 Hz +/- 1% | R-5.3.7 |

### TC-5.3.7.2 Pitch Duration Preserved

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1.0s input, pitch shift applied | Output duration = 1.0s within 1 ms | R-5.3.7 |

### TC-5.3.8.1 Custom Node Gain

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register custom +6 dB gain node, process signal | Output = input + 6 dB within 0.1 dB | R-5.3.8 |

### TC-5.3.8.2 Custom Node Removal

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Remove custom node at runtime | Bus output reverts to pre-insert signal | R-5.3.8 |

### TC-5.4.4.1 Beat Clock Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 120 BPM, 4/4 time, 10 seconds | Exactly 20 beat events and 5 bar events | R-5.4.4 |

### TC-5.4.4.2 Beat Clock Tempo Change

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Change from 120 BPM to 140 BPM mid-playback | Beat intervals after change within 1 ms of 140 BPM spacing | R-5.4.4 |

### TC-5.4.1.1 Stem Sync

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 4-stem music cue, 60 seconds | Cross-correlation peak offset = 0 samples between all stem pairs | R-5.4.1 |

### TC-5.4.1.2 Stem Fade Smooth

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fade stem gain 1.0 to 0.0 over 500 ms | Zero clicks in output (zero-crossing analysis) | R-5.4.1 |

### TC-5.4.2.1 Bar-Quantized Transition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger segment transition mid-bar | Transition executes at next bar boundary within +/- 1 sample | R-5.4.2 |

### TC-5.4.5.1 Stinger Cooldown

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger stinger, trigger again within cooldown | Second stinger suppressed (only first plays) | R-5.4.5 |

### TC-5.4.5.2 Stinger Priority

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Low-priority stinger playing, trigger high-priority | High-priority stinger ducks low-priority | R-5.4.5 |

### TC-5.4.6.1 Playlist Non-Repeat

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Shuffle playlist, 20 selections | No consecutive repeated track | R-5.4.6 |

### TC-5.4.6.2 Playlist Weighted

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | One track weighted 10x vs others, 1000 selections | Weighted track selected proportionally within 10% | R-5.4.6 |

### TC-5.4.7.1 Intensity Clamping

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set intensity = -0.5 | Clamped to 0.0 | R-5.4.7 |
| 2 | Set intensity = 1.5 | Clamped to 1.0 | R-5.4.7 |
| 3 | Set intensity = 0.7 | Value = 0.7 (unchanged) | R-5.4.7 |

### TC-5.5.1.1 Opus Encode Decode

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10 s speech at 24 kbps, round-trip | PESQ score > 3.0 | R-5.5.1 |

### TC-5.5.2.1 Jitter Buffer Adaptive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 50 ms jitter, 5% packet loss, 60 s | Jitter buffer depth < 80 ms | R-5.5.2 |

### TC-5.5.2.2 PLC No Silence Gap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Drop 1 packet during speech | PLC fills gap, no silence > 1 Opus frame (20 ms) | R-5.5.2 |

### TC-5.5.3.1 VAD Gates Silence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10 s silence then 5 s speech | 0 packets during silence, speech onset < 20 ms | R-5.5.3 |

### TC-5.5.3.2 Noise Suppression SNR

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Speech + keyboard noise at 0 dB SNR | Output SNR improvement >= 20 dB | R-5.5.3 |

### TC-5.5.9.1 AEC ERLE

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Reference playback through speakers, mic capture | ERLE > 30 dB | R-5.5.9 |

### TC-5.5.9.2 AEC Reconvergence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Change room acoustics mid-session | AEC re-convergence < 2 s | R-5.5.9 |

### TC-5.5.5.1 Viseme Timing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Known phoneme content processed | Viseme timestamps within 30 ms of reference | R-5.5.5 |

### TC-5.5.6.1 Dialogue Priority

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Bark playing, trigger critical dialogue line | Bark interrupted, critical line plays | R-5.5.6 |

### TC-5.5.6.2 Dialogue No Overlap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Queue 5 barks sequentially | All play sequentially with no overlap | R-5.5.6 |

## Integration Tests

### TC-5.3.NF1.I1 Full DSP Chain 64 Voices

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 4-insert DSP chain on 64 voices, 10K callbacks | p99 latency < 1 us/sample | R-5.3.NF1 |

### TC-5.4.NF1.I1 Music Zone Transition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger bar-quantized transition at random point | Switch at next bar within +/- 1 sample, no clicks | R-5.4.NF1 |

### TC-5.5.NF2.I1 Voice 32 Streams

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Decode and mix 32 Opus streams at 24 kbps for 60 s | Zero audio underruns | R-5.5.NF2 |

### TC-5.5.NF1.I1 Voice End-to-End Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Loopback capture-to-output, 50 ms RTT | Total latency < 150 ms | R-5.5.NF1 |

### TC-5.4.7.I1 Intensity Drives All Systems

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Ramp intensity 0.0 to 1.0 over 10 s | Stems, segment selection, stinger probability all respond | R-5.4.7 |

### TC-5.5.1.I1 Platform Mic Capture

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Windows, mic capture | WASAPI capture active | R-5.5.1 |
| 2 | macOS, mic capture | CoreAudio capture active | R-5.5.1 |
| 3 | Linux, mic capture | PipeWire/ALSA capture active | R-5.5.1 |

### TC-5.5.8.I1 Channel Isolation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create proximity, party, raid channels | Each has independent volume and routing | R-5.5.8 |

### TC-5.5.6.I1 Subtitle Sync

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Play VO with subtitles | Subtitle events within 16 ms of VO playback | R-5.5.6 |

### TC-5.5.8.I2 Proximity Attenuation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Proximity voice channel, source at 50 m | Distance attenuation applied via shared spatial index | R-5.5.8 |

### TC-5.5.4.I1 TTS Platform APIs

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Windows, TTS request | Uses SAPI | R-5.5.4 |
| 2 | macOS, TTS request | Uses AVSpeechSynthesizer | R-5.5.4 |
| 3 | Linux, TTS request | Uses Speech Dispatcher | R-5.5.4 |

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
