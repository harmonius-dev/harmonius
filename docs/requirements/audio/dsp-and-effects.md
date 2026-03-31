# R-5.3 -- DSP & Effects Requirements

## Filters

1. **R-5.3.1** — The engine **SHALL** provide low-pass, high-pass, band-pass, and notch biquad
   filters as enum variants with configurable cutoff frequency, resonance (Q), and gain. Coefficient
   updates **SHALL** be smoothed per-sample to eliminate zipper noise during parameter sweeps.
   - **Rationale:** Biquad filters are the building block for occlusion muffling, radio-voice
     effects, and tonal shaping. Per-sample smoothing prevents audible artifacts.
   - **Verification:** Unit test: process white noise through each filter at a known cutoff and
     verify frequency response within 0.5 dB. Sweep cutoff from 200 Hz to 8 kHz over 100 ms and
     verify no zipper artifacts.

2. **R-5.3.2** — The engine **SHALL** provide a multi-band parametric EQ as a bus insert supporting
   up to 8 bands with peak, shelf, and pass filter shapes. Band count **SHALL** scale per tier: 4 on
   mobile, 6 on Switch, 8 on desktop.
   - **Rationale:** Per-bus and per-zone EQ enables distinct tonal character for different
     environments without requiring separate assets.
   - **Verification:** Unit test: configure an 8-band EQ, process a flat-spectrum signal, and verify
     each band's gain within 0.5 dB.

## Reverb

3. **R-5.3.3** — The engine **SHALL** implement a feedback delay network reverb with configurable
   pre-delay, decay time, diffusion, damping, and wet/dry mix. FDN delay line count **SHALL** scale
   per tier: 4 on mobile, 8 on Switch, 16 on desktop. CPU per reverb instance **SHALL NOT** exceed
   5% of one core.
   - **Rationale:** Algorithmic reverb serves as the default for open-world environments where
     convolution IRs are impractical.
   - **Verification:** Unit test: process an impulse with decay 2.0 s and verify energy decay
     matches +/- 10%. Benchmark: verify CPU per instance below 5%.

4. **R-5.3.4** — The engine **SHALL** implement partitioned FFT convolution processing user-supplied
   impulse responses. IR assets **SHALL** load via the streaming system. Output latency
   **SHALL NOT** exceed one audio buffer period. IR length **SHALL** be capped per platform: 0.5 s
   on Switch, 2.0 s+ on desktop. Convolution **SHALL NOT** be available on mobile.
   - **Rationale:** Convolution reproduces authentic acoustic signatures for hero locations where
     fidelity justifies higher CPU cost.
   - **Verification:** Unit test: convolve a test impulse with a known IR and verify accuracy within
     -60 dB of reference. Verify output latency does not exceed one buffer period.

## Dynamics

5. **R-5.3.5** — The engine **SHALL** provide per-bus compressor inserts with threshold, ratio,
   attack, release, knee, and makeup gain. A look-ahead limiter on the master bus **SHALL** prevent
   output exceeding 0 dBFS. Both **SHALL** be enum variants.
   - **Rationale:** Dynamics processing prevents clipping during scenes with many simultaneous
     sources. The master limiter is a safety net against output overdrive.
   - **Verification:** Unit test: feed a signal 12 dB above threshold at ratio 4:1 and verify output
     within 0.5 dB. Feed 6 dB above 0 dBFS into the limiter and verify peak never exceeds 0 dBFS.

## Modulation and Time Effects

6. **R-5.3.6** — The engine **SHALL** provide delay (configurable time and feedback), chorus
   (multi-tap modulated delay), and flanger (short modulated delay with feedback) as enum variants.
   - **Rationale:** Time-based effects support creative sound design and gameplay-driven treatments
     such as canyon echoes and magical phasing.
   - **Verification:** Unit test: configure a 500 ms delay with 50% feedback, process an impulse,
     and verify echoes at 500 ms intervals with 6 dB decay within 0.5 dB.

7. **R-5.3.7** — The engine **SHALL** shift pitch independently of playback speed within +/- 12
   semitones. Desktop **SHALL** use phase-vocoder. Mobile **SHALL** use OLA. Output duration
   **SHALL** equal input within 1 ms. Artifacts **SHALL** stay below -40 dB.
   - **Rationale:** Pitch shifting enables real-time voice modulation, spell design, and slow-motion
     audio without altering source duration.
   - **Verification:** Unit test: shift 440 Hz up 12 semitones and verify output is 880 Hz +/- 1%
     with artifacts below -40 dB. Verify duration matches input within 1 ms.

## Extensibility

8. **R-5.3.8** — The engine **SHALL** allow runtime registration of custom DSP nodes insertable at
   any mixer bus graph point. Each node **SHALL** implement a stateless process callback receiving a
   buffer and parameter block. Total chain length **SHALL** be capped per tier: 8-12 on mobile,
   16-24 on Switch, 32+ on desktop.
   - **Rationale:** Extensibility enables third-party effects, game-specific processing, and rapid
     prototyping without modifying engine internals.
   - **Verification:** Unit test: register a custom 6 dB gain node, insert it, and verify output
     increases by 6 dB +/- 0.1 dB. Stress test: fill chains to platform limits and verify no
     underruns.

9. **R-5.3.8a** — All built-in DSP types **SHALL** be variants of a single `DspEffect` enum. The
   audio thread **SHALL NOT** perform dynamic dispatch for built-in effect processing.
   - **Rationale:** The audio thread runs under a strict 0.5 ms budget. Enum dispatch eliminates
     vtable indirection, producing optimal machine code for the hot DSP loop.
   - **Verification:** Inspect generated assembly for no indirect calls. Benchmark: 4-insert chain
     per-sample cost under 1 microsecond at 48 kHz.

## Non-Functional Requirements

10. **R-5.3.NF1** — The engine **SHALL** process a 4-insert DSP chain (filter, EQ, compressor,
    modulation) on a single voice within 1 microsecond per sample at 48 kHz, enabling full chains on
    at least 64 simultaneous voices.
    - **Rationale:** DSP is the second-largest consumer of audio thread time. Per-voice cost must be
      bounded to scale with the target voice count.
    - **Verification:** Benchmark: 4-insert chain on 64 voices over 10,000 buffers. Assert p99
      per-sample cost below 1 microsecond.
