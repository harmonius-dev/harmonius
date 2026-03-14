# 5.3 — DSP & Effects

## Filter Effects

### F-5.3.1 Parametric Filters

Provide low-pass, high-pass, band-pass, and notch biquad filters with configurable cutoff
frequency, resonance (Q), and gain. Filters are the building blocks for occlusion muffling,
radio-voice effects, and tonal shaping across the mixer bus hierarchy. Coefficient updates
are smoothed per-sample to avoid zipper noise when parameters change at runtime.

- **Requirements:** R-5.3.1
- **Dependencies:** F-5.1.3
- **Platform notes:** None

### F-5.3.2 Parametric Equalizer

Offer a multi-band parametric EQ (up to 8 bands) as a bus insert, supporting peak, shelf,
and pass filter shapes per band. EQ profiles can be authored per reverb zone or per bus to
shape the tonal character of environments, ensuring that underwater ambience sounds distinct
from a stone cathedral without requiring separate assets.

- **Requirements:** R-5.3.2
- **Dependencies:** F-5.3.1
- **Platform notes:** None

## Reverb

### F-5.3.3 Algorithmic Reverb

Implement a low-cost algorithmic reverb (feedback delay network) with controls for
pre-delay, decay time, diffusion, damping, and wet/dry mix. Algorithmic reverb serves as
the default spatial reverb for open-world environments where convolution impulse responses
are impractical due to the sheer number of distinct spaces in an MMO world.

- **Requirements:** R-5.3.3
- **Dependencies:** F-5.3.1
- **Platform notes:** None

### F-5.3.4 Convolution Reverb

Convolve audio with user-supplied impulse responses (IRs) to reproduce the acoustic
signature of real or designed spaces. IR assets are loaded via the streaming system and
processed using partitioned FFT convolution to keep latency within one audio buffer. Used
selectively for hero locations (throne rooms, boss arenas) where acoustic fidelity justifies
the higher CPU cost.

- **Requirements:** R-5.3.4
- **Dependencies:** F-5.1.5, F-5.3.1
- **Platform notes:** None

## Dynamics

### F-5.3.5 Compressor, Limiter, and Dynamics Processing

Provide per-bus compressor and limiter inserts with configurable threshold, ratio, attack,
release, knee, and makeup gain. A look-ahead limiter on the master bus prevents digital
clipping during MMO scenarios where many simultaneous sound sources (raid combat, city
ambience) would otherwise overdrive the output.

- **Requirements:** R-5.3.5
- **Dependencies:** F-5.1.3
- **Platform notes:** None

## Modulation and Time Effects

### F-5.3.6 Delay, Chorus, and Flanger

Implement delay-line-based time effects — simple delay with feedback, chorus (multi-tap
modulated delay), and flanger (short modulated delay with feedback). These effects support
creative sound design on dedicated send buses and power gameplay-driven audio treatments
such as echo in canyons or phasing on magical abilities.

- **Requirements:** R-5.3.6
- **Dependencies:** F-5.1.3
- **Platform notes:** None

### F-5.3.7 Pitch Shifting

Shift the pitch of a signal independently of playback speed using a phase-vocoder or
time-domain overlap-add algorithm. Pitch shifting enables real-time voice modulation
(e.g., demon voice, chipmunk effect), spell-cast sound design, and slow-motion audio
without altering the duration of the source material.

- **Requirements:** R-5.3.7
- **Dependencies:** F-5.1.3
- **Platform notes:** None

## Extensibility

### F-5.3.8 Custom DSP Node Chains

Allow users and plugins to register custom DSP processing nodes that can be inserted into
any point in the mixer bus graph. Each node implements a stateless process callback
receiving an audio buffer and parameter block, enabling third-party effects, game-specific
audio processing, and rapid prototyping without modifying engine internals.

- **Requirements:** R-5.3.8
- **Dependencies:** F-5.1.3
- **Platform notes:** None
