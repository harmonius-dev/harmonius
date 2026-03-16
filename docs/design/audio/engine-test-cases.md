# Audio Engine Test Cases

Companion test cases for [engine.md](engine.md).

## Unit Tests

### TC-5.1.4.1 Voice Allocate and Release

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Allocate max_real_voices=128, release all | Voice pool empty, real_voice_count=0 | R-5.1.4 |
| 2 | Allocate 1 voice, verify ID | VoiceId returned, real_voice_count=1 | R-5.1.4 |

### TC-5.1.4.2 Voice Priority Stealing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fill pool with VoicePriority::Low, allocate VoicePriority::Critical | Lowest-score Low voice virtualized, Critical voice allocated | R-5.1.4 |
| 2 | Fill pool with VoicePriority::Critical, allocate VoicePriority::Low | Allocation returns None (no stealable voice) | R-5.1.4 |

### TC-5.1.4.3 Voice Virtualize Restore

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Voice at playback_offset=48000, virtualize | VoiceState::Virtual, playback_offset retained at 48000 | R-5.1.4 |
| 2 | Restore virtualized voice | VoiceState::Playing, playback resumes from 48000 | R-5.1.4 |

### TC-5.1.3.1 Mixer Gain Inheritance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Master gain=0.5, SFX bus gain=1.0 | SFX effective_gain=0.5 | R-5.1.3 |
| 2 | Master gain=0.5, SFX gain=0.5 | SFX effective_gain=0.25 | R-5.1.3 |

### TC-5.1.3.2 Mixer Mute Propagation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mute "Gameplay Submix" bus | All descendant bus outputs = zero samples | R-5.1.3 |
| 2 | Unmute "Gameplay Submix" bus | Descendant outputs restore to previous levels | R-5.1.3 |

### TC-5.1.3.3 Mixer Solo

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Solo SFX bus | Only SFX and its children produce output | R-5.1.3 |
| 2 | Unsolo SFX bus | All buses produce output again | R-5.1.3 |

### TC-5.1.3.4 Mixer DAG Topological Order

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Build DAG with 8 buses in tree structure | Mix processes leaf buses before parent buses | R-5.1.3 |

### TC-5.1.3.5 Bus Runtime Rewire

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Reparent SFX bus from Gameplay to Master | Gain inheritance updates to reflect new parent | R-5.1.3 |

### TC-5.1.6.1 Command Queue SPSC

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Push 10000 commands, drain all | 10000 commands received in push order | R-5.1.6 |
| 2 | Push 0 commands, drain | Iterator yields 0 items | R-5.1.6 |

### TC-5.1.6.2 Sample Accurate Scheduling

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Schedule two sounds at SampleOffset(256) | Both start at sample 256, phase alignment = 0 samples | R-5.1.6 |

### TC-5.1.6.3 Scheduling Jitter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000 scheduled commands, measure jitter | Zero-sample deviation from target offsets | R-5.1.6 |

### TC-5.1.7.1 Codec PCM Decode

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Decode PCM WAV file | Output matches reference waveform (bitwise) | R-5.1.7 |

### TC-5.1.7.2 Codec Vorbis Decode

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Decode Vorbis file | Output within -60 dB of reference | R-5.1.7 |

### TC-5.1.7.3 Codec Opus Decode

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Decode Opus file | Output within -60 dB of reference | R-5.1.7 |

### TC-5.1.7.4 Codec FLAC Decode

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Decode FLAC file | Output matches reference (lossless, bitwise) | R-5.1.7 |

### TC-5.1.7.5 Codec Registry Plugin

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register custom codec, create decoder | Decoder decodes correctly via factory | R-5.1.7 |

### TC-5.1.7.6 Metadata Extraction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | PCM WAV 48000 Hz stereo | sample_rate=48000, channel_count=2 | R-5.1.7 |
| 2 | Vorbis 44100 Hz mono | sample_rate=44100, channel_count=1 | R-5.1.7 |

### TC-5.2.2.1 Attenuation Inverse

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Inverse model, distance=min_distance | Gain = 1.0 | R-5.2.2 |
| 2 | Inverse model, distance=mid_distance | Gain within 0.1% of 1/distance | R-5.2.2 |
| 3 | Inverse model, distance=max_distance | Gain = 0.0 | R-5.2.2 |

### TC-5.2.2.2 Attenuation Inverse-Squared

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | InverseSquared model, distance=mid_distance | Gain within 0.1% of 1/distance^2 (normalized) | R-5.2.2 |

### TC-5.2.2.3 Attenuation Linear

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Linear model, distance=halfway between min and max | Gain = 0.5 within 0.1% | R-5.2.2 |

### TC-5.2.2.4 Attenuation Logarithmic

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Logarithmic model, distance=mid_distance | Gain within 0.1% of logarithmic curve | R-5.2.2 |

### TC-5.2.2.5 Attenuation Custom Curve

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Custom 3-point curve, distance between points 1 and 2 | Gain = linear interpolation of control points within 0.1% | R-5.2.2 |

### TC-5.2.1.1 Doppler Pitch Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Source moving at 34.3 m/s toward listener (Mach 0.1) | Doppler ratio within 1% of 1.111 | R-5.2.1 |
| 2 | Source moving at 34.3 m/s away from listener | Doppler ratio within 1% of 0.909 | R-5.2.1 |

### TC-5.2.1.2 Panning Edge Cases

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Source at 0 degrees (front) | pan = 0.0 | R-5.2.1 |
| 2 | Source at 90 degrees (right) | pan = 1.0 | R-5.2.1 |
| 3 | Source at 180 degrees (behind) | pan = 0.0 | R-5.2.1 |
| 4 | Source at 270 degrees (left) | pan = -1.0 | R-5.2.1 |

### TC-5.2.5.1 Occlusion Single Ray

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Ray through wall, material loss=10 dB | Attenuation within 1 dB of 10 dB | R-5.2.5 |
| 2 | Ray with no obstruction | Attenuation = 0 dB | R-5.2.5 |

### TC-5.2.5.2 Occlusion Shared BVH

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Occlusion query for active voice | Query uses shared SpatialIndex (not separate structure) | R-5.2.5 |

### TC-5.2.3.1 HRTF Load SOFA

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load valid SOFA file | Azimuth/elevation lookup returns valid HrtfIndex | R-5.2.3 |
| 2 | Load invalid SOFA data | Returns HrtfError::InvalidFormat | R-5.2.3 |

### TC-5.2.3.2 HRTF Swap Runtime

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Swap HRTF dataset during playback | New profile active within one buffer period | R-5.2.3 |

### TC-5.2.4.1 Ambisonics Encode Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mono source at azimuth=0, elevation=0 | W/X/Y/Z coefficients within 0.1% of analytical values | R-5.2.4 |

### TC-5.2.4.2 Ambisonics Rotation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Rotate Ambisonics field 90 degrees | Coefficients shift correctly (X and Y swap with sign) | R-5.2.4 |

### TC-5.2.4.3 Ambisonics Decode Stereo

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | First-order Ambisonics decoded to stereo | Left/right output matches analytical decode | R-5.2.4 |

### TC-5.2.4.4 Ambisonics Decode Surround

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Decode to 5.1 format | 6 channels with correct speaker mapping | R-5.2.4 |
| 2 | Decode to 7.1 format | 8 channels with correct speaker mapping | R-5.2.4 |

### TC-5.2.7.1 Reverb Zone Blending

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Move listener between zones (decay 1.0s and 3.0s) | Smooth crossfade of decay parameters | R-5.2.7 |

### TC-5.2.7.2 Reverb Nested Priority

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Inner zone priority=10, outer zone priority=5 | Inner zone parameters override at listener position | R-5.2.7 |

### TC-5.1.5.1 Stream Ring Buffer

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write 10000 samples, read 10000 samples | All samples match, no data loss or overrun | R-5.1.5 |

### TC-5.1.1.1 Component Size

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Measure size_of::<AudioSource>() | Size <= 128 bytes | R-5.1.1 |

### TC-5.1.2.1 Listener Default Camera

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Remove all AudioListener components | Active camera entity used as fallback listener | R-5.1.2 |

## Integration Tests

### TC-5.1.NF4.I1 End-to-End Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger play command, measure to first non-zero sample | Latency < 20 ms | R-5.1.NF4 |

### TC-5.1.5.I1 Streaming Platform IO

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Windows, stream audio file | Uses IOCP (not stdlib file I/O) | R-5.1.5 |
| 2 | macOS, stream audio file | Uses GCD Dispatch IO | R-5.1.5 |
| 3 | Linux, stream audio file | Uses io_uring | R-5.1.5 |

### TC-5.1.5.I2 Streaming Memory Cap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stream 5-minute file, measure peak memory | Peak memory < 256 KiB per stream | R-5.1.5 |

### TC-5.1.5.I3 Prefetch Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Prefetch 500 ms ahead, measure startup latency | Startup latency < 10 ms | R-5.1.5 |

### TC-5.1.5.I4 Streaming Under Contention

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stream audio during heavy asset loading, 60 s | Zero audio underruns | R-5.1.5 |

### TC-5.1.NF2.I1 Full Mix No Underrun

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 256 voices with spatialization + 2-insert DSP, 60 s | Zero underruns | R-5.1.NF2 |

### TC-5.2.6.I1 Propagation Async

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Propagation solver running | Solver runs on worker thread, audio thread not blocked | R-5.2.6 |

### TC-5.2.6.I2 Portal Propagation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Source in room A, listener in room B via open portal | Delayed attenuated indirect path present | R-5.2.6 |
| 2 | Close portal between rooms | Indirect path removed | R-5.2.6 |

### TC-5.2.5.I1 Occlusion Per Platform Rays

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile config | occlusion_rays = 1 | R-5.2.5 |
| 2 | Switch config | occlusion_rays = 2 | R-5.2.5 |
| 3 | Desktop config | occlusion_rays = 4 | R-5.2.5 |

### TC-5.1.6.I1 Cross Thread Command

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Push command from game thread | Command arrives on audio thread within one buffer latency | R-5.1.6 |

### TC-5.1.2.I1 Multi Listener Split Screen

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two listeners with different positions | Each produces independent spatial audio output | R-5.1.2 |

### TC-5.2.2.I1 Attenuation Cross Platform

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same source/listener on all platforms | Identical attenuation gain values | R-5.2.2 |

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
