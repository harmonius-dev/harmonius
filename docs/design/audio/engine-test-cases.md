# Audio Engine Test Cases

Companion test cases for [engine.md](engine.md).

## Unit Tests

### TC-5.1.4.1 Voice Allocate and Release

| # | Requirement |
|---|-------------|
| 1 | R-5.1.4     |
| 2 | R-5.1.4     |

1. **#1** — Allocate max_real_voices=128, release all
   - **Expected:** Voice pool empty, real_voice_count=0
2. **#2** — Allocate 1 voice, verify ID
   - **Expected:** VoiceId returned, real_voice_count=1

### TC-5.1.4.2 Voice Priority Stealing

| # | Requirement |
|---|-------------|
| 1 | R-5.1.4     |
| 2 | R-5.1.4     |

1. **#1** — Fill pool with VoicePriority::Low, allocate VoicePriority::Critical
   - **Expected:** Lowest-score Low voice virtualized, Critical voice allocated
2. **#2** — Fill pool with VoicePriority::Critical, allocate VoicePriority::Low
   - **Expected:** Allocation returns None (no stealable voice)

### TC-5.1.4.3 Voice Virtualize Restore

| # | Requirement |
|---|-------------|
| 1 | R-5.1.4     |
| 2 | R-5.1.4     |

1. **#1** — Voice at playback_offset=48000, virtualize
   - **Expected:** VoiceState::Virtual, playback_offset retained at 48000
2. **#2** — Restore virtualized voice
   - **Expected:** VoiceState::Playing, playback resumes from 48000

### TC-5.1.3.1 Mixer Gain Inheritance

| # | Requirement |
|---|-------------|
| 1 | R-5.1.3     |
| 2 | R-5.1.3     |

1. **#1** — Master gain=0.5, SFX bus gain=1.0
   - **Expected:** SFX effective_gain=0.5
2. **#2** — Master gain=0.5, SFX gain=0.5
   - **Expected:** SFX effective_gain=0.25

### TC-5.1.3.2 Mixer Mute Propagation

| # | Requirement |
|---|-------------|
| 1 | R-5.1.3     |
| 2 | R-5.1.3     |

1. **#1** — Mute "Gameplay Submix" bus
   - **Expected:** All descendant bus outputs = zero samples
2. **#2** — Unmute "Gameplay Submix" bus
   - **Expected:** Descendant outputs restore to previous levels

### TC-5.1.3.3 Mixer Solo

| # | Requirement |
|---|-------------|
| 1 | R-5.1.3     |
| 2 | R-5.1.3     |

1. **#1** — Solo SFX bus
   - **Expected:** Only SFX and its children produce output
2. **#2** — Unsolo SFX bus
   - **Expected:** All buses produce output again

### TC-5.1.3.4 Mixer DAG Topological Order

| # | Requirement |
|---|-------------|
| 1 | R-5.1.3     |

1. **#1** — Build DAG with 8 buses in tree structure
   - **Expected:** Mix processes leaf buses before parent buses

### TC-5.1.3.5 Bus Runtime Rewire

| # | Requirement |
|---|-------------|
| 1 | R-5.1.3     |

1. **#1** — Reparent SFX bus from Gameplay to Master
   - **Expected:** Gain inheritance updates to reflect new parent

### TC-5.1.6.1 Command Queue SPSC

| # | Requirement |
|---|-------------|
| 1 | R-5.1.6     |
| 2 | R-5.1.6     |

1. **#1** — Push 10000 commands, drain all
   - **Expected:** 10000 commands received in push order
2. **#2** — Push 0 commands, drain
   - **Expected:** Iterator yields 0 items

### TC-5.1.6.2 Sample Accurate Scheduling

| # | Requirement |
|---|-------------|
| 1 | R-5.1.6     |

1. **#1** — Schedule two sounds at SampleOffset(256)
   - **Expected:** Both start at sample 256, phase alignment = 0 samples

### TC-5.1.6.3 Scheduling Jitter

| # | Requirement |
|---|-------------|
| 1 | R-5.1.6     |

1. **#1** — 1000 scheduled commands, measure jitter
   - **Expected:** Zero-sample deviation from target offsets

### TC-5.1.7.1 Codec PCM Decode

| # | Requirement |
|---|-------------|
| 1 | R-5.1.7     |

1. **#1** — Decode PCM WAV file
   - **Expected:** Output matches reference waveform (bitwise)

### TC-5.1.7.2 Codec Vorbis Decode

| # | Requirement |
|---|-------------|
| 1 | R-5.1.7     |

1. **#1** — Decode Vorbis file
   - **Expected:** Output within -60 dB of reference

### TC-5.1.7.3 Codec Opus Decode

| # | Requirement |
|---|-------------|
| 1 | R-5.1.7     |

1. **#1** — Decode Opus file
   - **Expected:** Output within -60 dB of reference

### TC-5.1.7.4 Codec FLAC Decode

| # | Requirement |
|---|-------------|
| 1 | R-5.1.7     |

1. **#1** — Decode FLAC file
   - **Expected:** Output matches reference (lossless, bitwise)

### TC-5.1.7.5 Codec Registry Plugin

| # | Requirement |
|---|-------------|
| 1 | R-5.1.7     |

1. **#1** — Register custom codec, create decoder
   - **Expected:** Decoder decodes correctly via factory

### TC-5.1.7.6 Metadata Extraction

| # | Requirement |
|---|-------------|
| 1 | R-5.1.7     |
| 2 | R-5.1.7     |

1. **#1** — PCM WAV 48000 Hz stereo
   - **Expected:** sample_rate=48000, channel_count=2
2. **#2** — Vorbis 44100 Hz mono
   - **Expected:** sample_rate=44100, channel_count=1

### TC-5.2.2.1 Attenuation Inverse

| # | Requirement |
|---|-------------|
| 1 | R-5.2.2     |
| 2 | R-5.2.2     |
| 3 | R-5.2.2     |

1. **#1** — Inverse model, distance=min_distance
   - **Expected:** Gain = 1.0
2. **#2** — Inverse model, distance=mid_distance
   - **Expected:** Gain within 0.1% of 1/distance
3. **#3** — Inverse model, distance=max_distance
   - **Expected:** Gain = 0.0

### TC-5.2.2.2 Attenuation Inverse-Squared

| # | Requirement |
|---|-------------|
| 1 | R-5.2.2     |

1. **#1** — InverseSquared model, distance=mid_distance
   - **Expected:** Gain within 0.1% of 1/distance^2 (normalized)

### TC-5.2.2.3 Attenuation Linear

| # | Requirement |
|---|-------------|
| 1 | R-5.2.2     |

1. **#1** — Linear model, distance=halfway between min and max
   - **Expected:** Gain = 0.5 within 0.1%

### TC-5.2.2.4 Attenuation Logarithmic

| # | Requirement |
|---|-------------|
| 1 | R-5.2.2     |

1. **#1** — Logarithmic model, distance=mid_distance
   - **Expected:** Gain within 0.1% of logarithmic curve

### TC-5.2.2.5 Attenuation Custom Curve

| # | Requirement |
|---|-------------|
| 1 | R-5.2.2     |

1. **#1** — Custom 3-point curve, distance between points 1 and 2
   - **Expected:** Gain = linear interpolation of control points within 0.1%

### TC-5.2.1.1 Doppler Pitch Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-5.2.1     |
| 2 | R-5.2.1     |

1. **#1** — Source moving at 34.3 m/s toward listener (Mach 0.1)
   - **Expected:** Doppler ratio within 1% of 1.111
2. **#2** — Source moving at 34.3 m/s away from listener
   - **Expected:** Doppler ratio within 1% of 0.909

### TC-5.2.1.2 Panning Edge Cases

| # | Requirement |
|---|-------------|
| 1 | R-5.2.1     |
| 2 | R-5.2.1     |
| 3 | R-5.2.1     |
| 4 | R-5.2.1     |

1. **#1** — Source at 0 degrees (front)
   - **Expected:** pan = 0.0
2. **#2** — Source at 90 degrees (right)
   - **Expected:** pan = 1.0
3. **#3** — Source at 180 degrees (behind)
   - **Expected:** pan = 0.0
4. **#4** — Source at 270 degrees (left)
   - **Expected:** pan = -1.0

### TC-5.2.5.1 Occlusion Single Ray

| # | Requirement |
|---|-------------|
| 1 | R-5.2.5     |
| 2 | R-5.2.5     |

1. **#1** — Ray through wall, material loss=10 dB
   - **Expected:** Attenuation within 1 dB of 10 dB
2. **#2** — Ray with no obstruction
   - **Expected:** Attenuation = 0 dB

### TC-5.2.5.2 Occlusion Shared BVH

| # | Requirement |
|---|-------------|
| 1 | R-5.2.5     |

1. **#1** — Occlusion query for active voice
   - **Expected:** Query uses shared SpatialIndex (not separate structure)

### TC-5.2.3.1 HRTF Load SOFA

| # | Requirement |
|---|-------------|
| 1 | R-5.2.3     |
| 2 | R-5.2.3     |

1. **#1** — Load valid SOFA file
   - **Expected:** Azimuth/elevation lookup returns valid HrtfIndex
2. **#2** — Load invalid SOFA data
   - **Expected:** Returns HrtfError::InvalidFormat

### TC-5.2.3.2 HRTF Swap Runtime

| # | Requirement |
|---|-------------|
| 1 | R-5.2.3     |

1. **#1** — Swap HRTF dataset during playback
   - **Expected:** New profile active within one buffer period

### TC-5.2.4.1 Ambisonics Encode Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-5.2.4     |

1. **#1** — Mono source at azimuth=0, elevation=0
   - **Expected:** W/X/Y/Z coefficients within 0.1% of analytical values

### TC-5.2.4.2 Ambisonics Rotation

| # | Requirement |
|---|-------------|
| 1 | R-5.2.4     |

1. **#1** — Rotate Ambisonics field 90 degrees
   - **Expected:** Coefficients shift correctly (X and Y swap with sign)

### TC-5.2.4.3 Ambisonics Decode Stereo

| # | Requirement |
|---|-------------|
| 1 | R-5.2.4     |

1. **#1** — First-order Ambisonics decoded to stereo
   - **Expected:** Left/right output matches analytical decode

### TC-5.2.4.4 Ambisonics Decode Surround

| # | Requirement |
|---|-------------|
| 1 | R-5.2.4     |
| 2 | R-5.2.4     |

1. **#1** — Decode to 5.1 format
   - **Expected:** 6 channels with correct speaker mapping
2. **#2** — Decode to 7.1 format
   - **Expected:** 8 channels with correct speaker mapping

### TC-5.2.7.1 Reverb Zone Blending

| # | Requirement |
|---|-------------|
| 1 | R-5.2.7     |

1. **#1** — Move listener between zones (decay 1.0s and 3.0s)
   - **Expected:** Smooth crossfade of decay parameters

### TC-5.2.7.2 Reverb Nested Priority

| # | Requirement |
|---|-------------|
| 1 | R-5.2.7     |

1. **#1** — Inner zone priority=10, outer zone priority=5
   - **Expected:** Inner zone parameters override at listener position

### TC-5.1.5.1 Stream Ring Buffer

| # | Requirement |
|---|-------------|
| 1 | R-5.1.5     |

1. **#1** — Write 10000 samples, read 10000 samples
   - **Expected:** All samples match, no data loss or overrun

### TC-5.1.1.1 Component Size

| # | Requirement |
|---|-------------|
| 1 | R-5.1.1     |

1. **#1** — Measure size_of::<AudioSource>()
   - **Expected:** Size <= 128 bytes

### TC-5.1.2.1 Listener Default Camera

| # | Requirement |
|---|-------------|
| 1 | R-5.1.2     |

1. **#1** — Remove all AudioListener components
   - **Expected:** Active camera entity used as fallback listener

## Integration Tests

### TC-5.1.NF4.I1 End-to-End Latency

| # | Requirement |
|---|-------------|
| 1 | R-5.1.NF4   |

1. **#1** — Trigger play command, measure to first non-zero sample
   - **Expected:** Latency < 20 ms

### TC-5.1.5.I1 Streaming Platform IO

| # | Requirement |
|---|-------------|
| 1 | R-5.1.5     |
| 2 | R-5.1.5     |
| 3 | R-5.1.5     |

1. **#1** — Windows, stream audio file
   - **Expected:** Uses IOCP (not stdlib file I/O)
2. **#2** — macOS, stream audio file
   - **Expected:** Uses GCD Dispatch IO
3. **#3** — Linux, stream audio file
   - **Expected:** Uses io_uring

### TC-5.1.5.I2 Streaming Memory Cap

| # | Requirement |
|---|-------------|
| 1 | R-5.1.5     |

1. **#1** — Stream 5-minute file, measure peak memory
   - **Expected:** Peak memory < 256 KiB per stream

### TC-5.1.5.I3 Prefetch Latency

| # | Requirement |
|---|-------------|
| 1 | R-5.1.5     |

1. **#1** — Prefetch 500 ms ahead, measure startup latency
   - **Expected:** Startup latency < 10 ms

### TC-5.1.5.I4 Streaming Under Contention

| # | Requirement |
|---|-------------|
| 1 | R-5.1.5     |

1. **#1** — Stream audio during heavy asset loading, 60 s
   - **Expected:** Zero audio underruns

### TC-5.1.NF2.I1 Full Mix No Underrun

| # | Requirement |
|---|-------------|
| 1 | R-5.1.NF2   |

1. **#1** — 256 voices with spatialization + 2-insert DSP, 60 s
   - **Expected:** Zero underruns

### TC-5.2.6.I1 Propagation Async

| # | Requirement |
|---|-------------|
| 1 | R-5.2.6     |

1. **#1** — Propagation solver running
   - **Expected:** Solver runs on worker thread, audio thread not blocked

### TC-5.2.6.I2 Portal Propagation

| # | Requirement |
|---|-------------|
| 1 | R-5.2.6     |
| 2 | R-5.2.6     |

1. **#1** — Source in room A, listener in room B via open portal
   - **Expected:** Delayed attenuated indirect path present
2. **#2** — Close portal between rooms
   - **Expected:** Indirect path removed

### TC-5.2.5.I1 Occlusion Per Platform Rays

| # | Requirement |
|---|-------------|
| 1 | R-5.2.5     |
| 2 | R-5.2.5     |
| 3 | R-5.2.5     |

1. **#1** — Mobile config
   - **Expected:** occlusion_rays = 1
2. **#2** — Switch config
   - **Expected:** occlusion_rays = 2
3. **#3** — Desktop config
   - **Expected:** occlusion_rays = 4

### TC-5.1.6.I1 Cross Thread Command

| # | Requirement |
|---|-------------|
| 1 | R-5.1.6     |

1. **#1** — Push command from game thread
   - **Expected:** Command arrives on audio thread within one buffer latency

### TC-5.1.2.I1 Multi Listener Split Screen

| # | Requirement |
|---|-------------|
| 1 | R-5.1.2     |

1. **#1** — Two listeners with different positions
   - **Expected:** Each produces independent spatial audio output

### TC-5.2.2.I1 Attenuation Cross Platform

| # | Requirement |
|---|-------------|
| 1 | R-5.2.2     |

1. **#1** — Same source/listener on all platforms
   - **Expected:** Identical attenuation gain values

## Benchmarks

### TC-5.1.NF1.B1 Audio Callback

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 256 voices, full DSP chain | p99 callback time | < 0.5 ms | R-5.1.NF1 |

### TC-5.2.NF1.B1 Per Voice Spatialization

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | HRTF spatialization per voice | p99 time | < 2 us | R-5.2.NF1 |

### TC-5.2.NF2.B1 Propagation Solver

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 20 portals, 50 surfaces, 64 sources | p99 update time | < 4 ms | R-5.2.NF2 |

### TC-5.1.6.B1 Command Queue Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Sustained command push/drain | Throughput | > 100K cmds/s | R-5.1.6 |

### TC-5.1.4.B1 Voice Allocation Release

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single allocate + release cycle | CPU time | < 1 us | R-5.1.4 |

### TC-5.1.5.B1 Stream Ring Buffer Read

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Read 512 samples from ring buffer | CPU time | < 0.5 us | R-5.1.5 |

### TC-5.1.3.B1 Mixer Graph Traversal

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 8 buses, 128 voices | Mix time | < 0.1 ms | R-5.1.3 |

### TC-5.2.3.B1 HRTF Convolution

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 512-sample block HRTF convolution | CPU time | < 50 us | R-5.2.3 |

### TC-5.2.4.B1 Ambisonics Encode Decode

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | First-order encode + decode per voice | CPU time | < 10 us | R-5.2.4 |

### TC-5.1.NF3.B1 Audio System Memory

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Max config (256 voices, full DSP, spatial) | Resident memory | < 64 MiB | R-5.1.NF3 |
