# R-5.3 — DSP & Effects Requirements

## Filter Effects

### R-5.3.1 Parametric Filters

The engine **SHALL** provide low-pass, high-pass, band-pass, and notch biquad filters with
configurable cutoff frequency, resonance (Q), and gain, smoothing coefficient updates
per-sample to prevent zipper noise during runtime parameter changes.

- **Derived from:** [F-5.3.1](../../features/audio/dsp-and-effects.md)
- **Rationale:** Biquad filters are the building blocks for occlusion muffling, radio-voice
  effects, and tonal shaping; zipper-free updates are essential for smooth transitions.
- **Verification:** Unit test: for each filter type, process a white noise buffer at a known
  cutoff frequency. Verify the frequency response matches the expected biquad transfer
  function within 0.5 dB tolerance at 10 test frequencies. Sweep the cutoff from 200 Hz to
  8 kHz over 100 ms and verify no audible clicks or zipper artifacts in the output waveform.

### R-5.3.2 Parametric Equalizer

The engine **SHALL** provide a multi-band parametric EQ with up to 8 bands as a bus insert,
supporting peak, shelf, and pass filter shapes per band, with EQ profiles assignable per
reverb zone or per bus.

- **Derived from:** [F-5.3.2](../../features/audio/dsp-and-effects.md)
- **Rationale:** Per-zone and per-bus EQ enables tonal shaping of environments without
  requiring separate asset variants for each acoustic space.
- **Verification:** Unit test: configure an 8-band EQ with known gain, frequency, and Q per
  band. Process a flat-spectrum test signal and measure the output spectrum. Verify each
  band's gain matches the configured value within 0.5 dB at its center frequency. Assign
  different EQ profiles to two reverb zones and verify the correct profile is applied in
  each zone.

## Reverb

### R-5.3.3 Algorithmic Reverb

The engine **SHALL** implement a feedback delay network reverb with controls for pre-delay,
decay time, diffusion, damping, and wet/dry mix, consuming no more than 5% of a single CPU
core per active instance at 48 kHz sample rate.

- **Derived from:** [F-5.3.3](../../features/audio/dsp-and-effects.md)
- **Rationale:** Algorithmic reverb is the default spatial reverb for open-world environments
  where convolution impulse responses are impractical due to the number of distinct spaces.
- **Verification:** Unit test: process a known impulse through the reverb with decay time set
  to 2.0 s. Verify the energy decay curve (EDT) matches 2.0 s +/- 10%. Benchmark: measure
  CPU usage per instance at 48 kHz and verify it does not exceed 5% of one core. Verify
  all five parameters produce audible and measurable changes to the output.

### R-5.3.4 Convolution Reverb

The engine **SHALL** convolve audio with user-supplied impulse responses using partitioned FFT
convolution, keeping processing latency within one audio buffer period, and loading IR assets
via the streaming system.

- **Derived from:** [F-5.3.4](../../features/audio/dsp-and-effects.md)
- **Rationale:** Convolution reverb reproduces the acoustic signature of real or designed
  spaces with high fidelity for hero locations where quality justifies cost.
- **Verification:** Unit test: convolve a test impulse with a known IR and compare the output
  against offline reference convolution; verify sample-level accuracy within -60 dB error.
  Measure output latency and verify it does not exceed one audio buffer period. Load a 2 s
  IR via streaming and verify playback begins without blocking the audio thread.

## Dynamics

### R-5.3.5 Compressor, Limiter, and Dynamics Processing

The engine **SHALL** provide per-bus compressor and limiter inserts with configurable
threshold, ratio, attack, release, knee, and makeup gain, and **SHALL** include a look-ahead
limiter on the master bus that prevents digital clipping (output never exceeds 0 dBFS).

- **Derived from:** [F-5.3.5](../../features/audio/dsp-and-effects.md)
- **Rationale:** Dynamics processing prevents output clipping during high-density audio
  scenarios with many simultaneous sound sources.
- **Verification:** Unit test: feed a signal 12 dB above threshold into the compressor with
  ratio 4:1. Verify output level matches expected compression curve within 0.5 dB. Feed a
  signal 6 dB above 0 dBFS into the master bus limiter and verify the output peak never
  exceeds 0 dBFS. Verify attack and release timing match configured values within 10%.

## Modulation and Time Effects

### R-5.3.6 Delay, Chorus, and Flanger

The engine **SHALL** implement delay-line-based time effects including simple delay with
feedback, chorus (multi-tap modulated delay), and flanger (short modulated delay with
feedback), each insertable on any mixer bus.

- **Derived from:** [F-5.3.6](../../features/audio/dsp-and-effects.md)
- **Rationale:** Time-based effects support creative sound design and gameplay-driven audio
  treatments such as echo in canyons or phasing on magical abilities.
- **Verification:** Unit test: configure a delay with 500 ms delay time and 50% feedback.
  Process a single impulse and verify echoes appear at 500 ms intervals with 6 dB decay per
  tap within 0.5 dB tolerance. Verify chorus produces measurable pitch modulation in the
  output spectrum. Verify flanger produces comb-filter notches at expected frequencies.

### R-5.3.7 Pitch Shifting

The engine **SHALL** shift the pitch of a signal independently of playback speed, preserving
the original duration, with a supported range of at least +/- 12 semitones and artifact
level below -40 dB relative to the signal.

- **Derived from:** [F-5.3.7](../../features/audio/dsp-and-effects.md)
- **Rationale:** Pitch shifting enables real-time voice modulation, spell-cast sound design,
  and slow-motion audio without altering source duration.
- **Verification:** Unit test: pitch-shift a 440 Hz sine tone up 12 semitones and verify the
  output fundamental is 880 Hz +/- 1%. Verify the output duration equals the input duration
  within 1 ms. Measure artifact energy below -40 dB relative to the fundamental. Test at
  -12, -6, +6, and +12 semitone shifts.

## Extensibility

### R-5.3.8 Custom DSP Node Chains

The engine **SHALL** allow registration of custom DSP processing nodes insertable at any
point in the mixer bus graph, where each node implements a stateless process callback
receiving an audio buffer and parameter block.

- **Derived from:** [F-5.3.8](../../features/audio/dsp-and-effects.md)
- **Rationale:** Extensible DSP chains enable third-party effects, game-specific audio
  processing, and rapid prototyping without modifying engine internals.
- **Verification:** Integration test: register a custom DSP node that applies 6 dB gain.
  Insert it into a bus chain and verify the output level increases by 6 dB +/- 0.1 dB.
  Register a second custom node, insert both in series, and verify combined effect. Remove
  a node at runtime and verify the bus output reverts. Verify the node callback receives
  correct buffer size and parameter values.

---

## Non-Functional Requirements

### R-5.3.NF1 DSP Chain Per-Voice Budget

The engine **SHALL** process a 4-insert DSP chain (filter, EQ, compressor, and one
modulation effect) on a single voice within 1 microsecond per audio buffer sample at 48 kHz,
enabling full DSP chains on at least 64 simultaneous voices within the audio thread budget
(R-5.1.NF1).

- **Derived from:** [R-5.1.NF1](audio-engine.md),
  [F-5.3.8](../../features/audio/dsp-and-effects.md)
- **Rationale:** DSP processing is the second-largest consumer of audio thread time after
  spatialization. Per-voice cost must be bounded to scale with the target voice count.
- **Verification:** Benchmark: process a 4-insert chain on 64 voices over 10,000 buffer
  callbacks. Assert p99 per-sample cost is below 1 microsecond on minimum-spec hardware.
