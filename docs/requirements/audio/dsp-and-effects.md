# R-5.3 -- DSP & Effects Requirements

## Filters

| ID      | Derived From                                       |
|---------|----------------------------------------------------|
| R-5.3.1 | [F-5.3.1](../../features/audio/dsp-and-effects.md) |
| R-5.3.2 | [F-5.3.2](../../features/audio/dsp-and-effects.md) |

1. **R-5.3.1** — The engine **SHALL** provide low-pass, high-pass, band-pass, and notch biquad
   filters as enum variants (not trait objects) with configurable cutoff frequency, resonance (Q),
   and gain. Filter coefficient updates **SHALL** be smoothed per-sample to eliminate zipper noise
   during runtime parameter sweeps.
   - **Rationale:** Biquad filters are the fundamental DSP building block for occlusion muffling,
     radio-voice effects, and tonal shaping. Per-sample smoothing prevents audible artifacts during
     dynamic sweeps.
   - **Verification:** Unit test: process white noise through each filter type at a known cutoff and
     verify frequency response matches the biquad transfer function within 0.5 dB. Sweep cutoff from
     200 Hz to 8 kHz over 100 ms and verify no audible clicks or zipper artifacts.
2. **R-5.3.2** — The engine **SHALL** provide a multi-band parametric EQ as a bus insert supporting
   up to 8 bands with peak, shelf, and pass filter shapes per band. Band count **SHALL** scale per
   platform tier: 4 on mobile, 6 on Switch, and 8 on desktop.
   - **Rationale:** Per-bus and per-zone EQ enables distinct tonal character for different
     environments (underwater, cathedral, cave) without requiring separate assets.
   - **Verification:** Unit test: configure an 8-band EQ with known parameters, process a
     flat-spectrum signal, and verify each band's gain matches within 0.5 dB.

## Reverb

| ID      | Derived From                                       |
|---------|----------------------------------------------------|
| R-5.3.3 | [F-5.3.3](../../features/audio/dsp-and-effects.md) |
| R-5.3.4 | [F-5.3.4](../../features/audio/dsp-and-effects.md) |

1. **R-5.3.3** — The engine **SHALL** implement a feedback delay network reverb with configurable
   pre-delay, decay time, diffusion, damping, and wet/dry mix. FDN delay line count **SHALL** scale
   per platform tier: 4 on mobile, 8 on Switch, and 16 on desktop. CPU usage per reverb instance
   **SHALL NOT** exceed 5% of one core.
   - **Rationale:** Algorithmic reverb serves as the default spatial reverb for open-world
     environments where convolution IRs are impractical due to the number of distinct spaces.
   - **Verification:** Unit test: process an impulse with decay time 2.0s and verify the energy
     decay curve matches 2.0s +/- 10%. Benchmark: measure CPU usage per reverb instance and verify
     it does not exceed 5% of one core.
2. **R-5.3.4** — The engine **SHALL** implement partitioned FFT convolution that processes
   user-supplied impulse response assets. IR assets **SHALL** be loaded via the streaming system.
   Convolution output latency **SHALL NOT** exceed one audio buffer period. IR length **SHALL** be
   capped per platform: 0.5s on Switch, 2.0s+ on desktop. Convolution reverb **SHALL NOT** be
   available on mobile.
   - **Rationale:** Convolution reverb reproduces authentic acoustic signatures of real spaces for
     hero locations (throne rooms, boss arenas) where fidelity justifies the higher CPU cost.
   - **Verification:** Unit test: convolve a test impulse with a known IR and verify sample-level
     accuracy within -60 dB of a reference. Verify output latency does not exceed one audio buffer
     period.

## Dynamics

| ID      | Derived From                                       |
|---------|----------------------------------------------------|
| R-5.3.5 | [F-5.3.5](../../features/audio/dsp-and-effects.md) |

1. **R-5.3.5** — The engine **SHALL** provide per-bus compressor inserts with configurable
   threshold, ratio, attack, release, knee, and makeup gain. The engine **SHALL** provide a
   look-ahead limiter on the master bus that prevents output from exceeding 0 dBFS. Both compressor
   and limiter **SHALL** be enum variants (not trait objects).
   - **Rationale:** Dynamics processing prevents digital clipping during scenes with many
     simultaneous sound sources (raid combat, city ambience). The master limiter is a safety net
     against output overdrive.
   - **Verification:** Unit test: feed a signal 12 dB above threshold at ratio 4:1 and verify output
     matches the compression curve within 0.5 dB. Feed a signal 6 dB above 0 dBFS into the master
     limiter and verify output peak never exceeds 0 dBFS.

## Modulation and Time Effects

| ID      | Derived From                                       |
|---------|----------------------------------------------------|
| R-5.3.6 | [F-5.3.6](../../features/audio/dsp-and-effects.md) |
| R-5.3.7 | [F-5.3.7](../../features/audio/dsp-and-effects.md) |

1. **R-5.3.6** — The engine **SHALL** provide delay-line-based time effects: simple delay with
   configurable delay time and feedback, chorus (multi-tap modulated delay), and flanger (short
   modulated delay with feedback). All three **SHALL** be enum variants of the effect type.
   - **Rationale:** Time-based effects support creative sound design and gameplay-driven audio
     treatments such as canyon echoes and magical ability phasing.
   - **Verification:** Unit test: configure a 500ms delay with 50% feedback, process an impulse, and
     verify echoes at 500ms intervals with 6 dB decay within 0.5 dB. Verify chorus produces
     measurable pitch modulation in the output spectrum.
2. **R-5.3.7** — The engine **SHALL** shift pitch independently of playback speed within +/- 12
   semitones. Desktop **SHALL** use a phase-vocoder algorithm. Mobile **SHALL** use time-domain
   overlap-add (OLA) for lower CPU cost. Pitch-shifted output duration **SHALL** equal input
   duration within 1 ms. Artifacts **SHALL** remain below -40 dB.
   - **Rationale:** Pitch shifting enables real-time voice modulation (demon voice, chipmunk),
     spell-cast design, and slow-motion audio without altering source duration.
   - **Verification:** Unit test: shift a 440 Hz tone up 12 semitones and verify output is 880 Hz
     +/- 1% with artifacts below -40 dB. Verify output duration equals input duration within 1 ms.

## Extensibility

| ID       | Derived From                                       |
|----------|----------------------------------------------------|
| R-5.3.8  | [F-5.3.8](../../features/audio/dsp-and-effects.md) |
| R-5.3.8a | [F-5.3.8](../../features/audio/dsp-and-effects.md) |

1. **R-5.3.8** — The engine **SHALL** allow runtime registration of custom DSP processing nodes
   insertable at any point in the mixer bus graph. Each node **SHALL** implement a stateless process
   callback receiving an audio buffer and parameter block. Total DSP chain length **SHALL** be
   capped per tier: 8-12 nodes on mobile, 16-24 on Switch, and 32+ on desktop.
   - **Rationale:** Extensibility enables third-party effects, game-specific audio processing, and
     rapid prototyping without modifying engine internals.
   - **Verification:** Unit test: register a custom 6 dB gain node, insert it into a bus, and verify
     output increases by 6 dB +/- 0.1 dB. Remove the node at runtime and verify output reverts
     immediately. Stress test: fill chains to platform limits and verify no audio underruns.
2. **R-5.3.8a** — All built-in DSP effect types (filter, EQ, algorithmic reverb, convolution reverb,
   compressor, limiter, delay, chorus, flanger, pitch shifter) **SHALL** be variants of a single
   `DspEffect` enum. The audio thread **SHALL NOT** perform dynamic dispatch (no `dyn` trait
   objects) for built-in effect processing.
   - **Rationale:** The audio thread runs under a strict 0.5 ms budget per buffer callback. Enum
     dispatch eliminates vtable indirection, enabling the compiler to inline effect processing and
     produce optimal machine code for the hot DSP loop.
   - **Verification:** Inspect generated assembly for the effect processing loop and verify no
     indirect calls through vtables. Benchmark: process a 4-insert chain using enum dispatch and
     verify per-sample cost remains under 1 microsecond at 48 kHz.

## Non-Functional Requirements

| ID        | Derived From       |
|-----------|--------------------|
| R-5.3.NF1 | R-5.1.NF1, F-5.3.8 |

1. **R-5.3.NF1** — The engine **SHALL** process a 4-insert DSP chain (filter, EQ, compressor, and
   one modulation effect) on a single voice within 1 microsecond per audio buffer sample at 48 kHz,
   enabling full DSP chains on at least 64 simultaneous voices within the audio thread budget.
   - **Rationale:** DSP processing is the second-largest consumer of audio thread time after
     spatialization. Per-voice cost must be bounded to scale with the target voice count.
   - **Verification:** Benchmark: process a 4-insert chain on 64 voices over 10,000 buffer
     callbacks. Assert p99 per-sample cost is below 1 microsecond on minimum-spec hardware.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/audio/dsp-and-effects.md](../../user-stories/audio/dsp-and-effects.md).
