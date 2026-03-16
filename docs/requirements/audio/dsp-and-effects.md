# R-5.3 — DSP & Effects Requirements

## Filters

### R-5.3.1 Biquad Parametric Filters

The engine **SHALL** provide low-pass, high-pass,
band-pass, and notch biquad filters as enum variants
(not trait objects) with configurable cutoff frequency,
resonance (Q), and gain. Filter coefficient updates
**SHALL** be smoothed per-sample to eliminate zipper
noise during runtime parameter sweeps.

- **Derived from:**
  [F-5.3.1](../../features/audio/dsp-and-effects.md)
- **Rationale:** Biquad filters are the fundamental DSP
  building block for occlusion muffling, radio-voice
  effects, and tonal shaping. Per-sample smoothing
  prevents audible artifacts during dynamic sweeps.
- **Verification:** Unit test: process white noise
  through each filter type at a known cutoff and verify
  frequency response matches the biquad transfer
  function within 0.5 dB. Sweep cutoff from 200 Hz to
  8 kHz over 100 ms and verify no audible clicks or
  zipper artifacts.

### R-5.3.2 Parametric Equalizer

The engine **SHALL** provide a multi-band parametric EQ
as a bus insert supporting up to 8 bands with peak,
shelf, and pass filter shapes per band. Band count
**SHALL** scale per platform tier: 4 on mobile, 6 on
Switch, and 8 on desktop.

- **Derived from:**
  [F-5.3.2](../../features/audio/dsp-and-effects.md)
- **Rationale:** Per-bus and per-zone EQ enables distinct
  tonal character for different environments (underwater,
  cathedral, cave) without requiring separate assets.
- **Verification:** Unit test: configure an 8-band EQ
  with known parameters, process a flat-spectrum signal,
  and verify each band's gain matches within 0.5 dB.

## Reverb

### R-5.3.3 Algorithmic Reverb (FDN)

The engine **SHALL** implement a feedback delay network
reverb with configurable pre-delay, decay time,
diffusion, damping, and wet/dry mix. FDN delay line
count **SHALL** scale per platform tier: 4 on mobile,
8 on Switch, and 16 on desktop. CPU usage per reverb
instance **SHALL NOT** exceed 5% of one core.

- **Derived from:**
  [F-5.3.3](../../features/audio/dsp-and-effects.md)
- **Rationale:** Algorithmic reverb serves as the default
  spatial reverb for open-world environments where
  convolution IRs are impractical due to the number of
  distinct spaces.
- **Verification:** Unit test: process an impulse with
  decay time 2.0s and verify the energy decay curve
  matches 2.0s +/- 10%. Benchmark: measure CPU usage
  per reverb instance and verify it does not exceed 5%
  of one core.

### R-5.3.4 Convolution Reverb

The engine **SHALL** implement partitioned FFT
convolution that processes user-supplied impulse
response assets. IR assets **SHALL** be loaded via the
streaming system. Convolution output latency **SHALL
NOT** exceed one audio buffer period. IR length **SHALL**
be capped per platform: 0.5s on Switch, 2.0s+ on
desktop. Convolution reverb **SHALL NOT** be available
on mobile.

- **Derived from:**
  [F-5.3.4](../../features/audio/dsp-and-effects.md)
- **Rationale:** Convolution reverb reproduces authentic
  acoustic signatures of real spaces for hero locations
  (throne rooms, boss arenas) where fidelity justifies
  the higher CPU cost.
- **Verification:** Unit test: convolve a test impulse
  with a known IR and verify sample-level accuracy
  within -60 dB of a reference. Verify output latency
  does not exceed one audio buffer period.

## Dynamics

### R-5.3.5 Compressor and Limiter

The engine **SHALL** provide per-bus compressor inserts
with configurable threshold, ratio, attack, release,
knee, and makeup gain. The engine **SHALL** provide a
look-ahead limiter on the master bus that prevents
output from exceeding 0 dBFS. Both compressor and
limiter **SHALL** be enum variants (not trait objects).

- **Derived from:**
  [F-5.3.5](../../features/audio/dsp-and-effects.md)
- **Rationale:** Dynamics processing prevents digital
  clipping during scenes with many simultaneous sound
  sources (raid combat, city ambience). The master
  limiter is a safety net against output overdrive.
- **Verification:** Unit test: feed a signal 12 dB above
  threshold at ratio 4:1 and verify output matches the
  compression curve within 0.5 dB. Feed a signal 6 dB
  above 0 dBFS into the master limiter and verify output
  peak never exceeds 0 dBFS.

## Modulation and Time Effects

### R-5.3.6 Delay, Chorus, and Flanger

The engine **SHALL** provide delay-line-based time
effects: simple delay with configurable delay time and
feedback, chorus (multi-tap modulated delay), and
flanger (short modulated delay with feedback). All
three **SHALL** be enum variants of the effect type.

- **Derived from:**
  [F-5.3.6](../../features/audio/dsp-and-effects.md)
- **Rationale:** Time-based effects support creative
  sound design and gameplay-driven audio treatments such
  as canyon echoes and magical ability phasing.
- **Verification:** Unit test: configure a 500ms delay
  with 50% feedback, process an impulse, and verify
  echoes at 500ms intervals with 6 dB decay within
  0.5 dB. Verify chorus produces measurable pitch
  modulation in the output spectrum.

### R-5.3.7 Pitch Shifting

The engine **SHALL** shift pitch independently of
playback speed within +/- 12 semitones. Desktop
**SHALL** use a phase-vocoder algorithm. Mobile **SHALL**
use time-domain overlap-add (OLA) for lower CPU cost.
Pitch-shifted output duration **SHALL** equal input
duration within 1 ms. Artifacts **SHALL** remain below
-40 dB.

- **Derived from:**
  [F-5.3.7](../../features/audio/dsp-and-effects.md)
- **Rationale:** Pitch shifting enables real-time voice
  modulation (demon voice, chipmunk), spell-cast design,
  and slow-motion audio without altering source duration.
- **Verification:** Unit test: shift a 440 Hz tone up 12
  semitones and verify output is 880 Hz +/- 1% with
  artifacts below -40 dB. Verify output duration equals
  input duration within 1 ms.

## Extensibility

### R-5.3.8 Custom DSP Node Registry

The engine **SHALL** allow runtime registration of
custom DSP processing nodes insertable at any point in
the mixer bus graph. Each node **SHALL** implement a
stateless process callback receiving an audio buffer and
parameter block. Total DSP chain length **SHALL** be
capped per tier: 8-12 nodes on mobile, 16-24 on Switch,
and 32+ on desktop.

- **Derived from:**
  [F-5.3.8](../../features/audio/dsp-and-effects.md)
- **Rationale:** Extensibility enables third-party
  effects, game-specific audio processing, and rapid
  prototyping without modifying engine internals.
- **Verification:** Unit test: register a custom 6 dB
  gain node, insert it into a bus, and verify output
  increases by 6 dB +/- 0.1 dB. Remove the node at
  runtime and verify output reverts immediately. Stress
  test: fill chains to platform limits and verify no
  audio underruns.

### R-5.3.8a Enum Dispatch for Built-In Effects

All built-in DSP effect types (filter, EQ, algorithmic
reverb, convolution reverb, compressor, limiter, delay,
chorus, flanger, pitch shifter) **SHALL** be variants of
a single `DspEffect` enum. The audio thread **SHALL
NOT** perform dynamic dispatch (no `dyn` trait objects)
for built-in effect processing.

- **Derived from:**
  [F-5.3.8](../../features/audio/dsp-and-effects.md)
- **Rationale:** The audio thread runs under a strict
  0.5 ms budget per buffer callback. Enum dispatch
  eliminates vtable indirection, enabling the compiler
  to inline effect processing and produce optimal
  machine code for the hot DSP loop.
- **Verification:** Inspect generated assembly for the
  effect processing loop and verify no indirect calls
  through vtables. Benchmark: process a 4-insert chain
  using enum dispatch and verify per-sample cost remains
  under 1 microsecond at 48 kHz.

---

## User Stories

## F-5.3.1 Parametric Filters

## US-5.3.1.1 Apply Biquad Filters to Audio

**As an** audio designer (P-14), **I want to** apply low-pass, high-pass, band-pass, and notch
biquad filters with configurable cutoff, resonance, and gain, **so that** tonal shaping and
occlusion muffling are possible.

## US-5.3.1.2 Automate Filter Parameters at Runtime

**As an** audio designer (P-14), **I want to** sweep filter cutoff and resonance at runtime without
zipper noise, **so that** dynamic sound design transitions are smooth.

## US-5.3.1.3 Set Up Filters in Editor

**As a** designer (P-5), **I want to** add and configure biquad filters on mixer buses in the visual
editor, **so that** filtering is authored without code.

## US-5.3.1.4 Verify Filter Frequency Response

**As an** engine tester (P-27), **I want to** process white noise through each filter type at a
known cutoff and verify frequency response matches the biquad transfer function within 0.5 dB, **so
that** filter accuracy is confirmed.

## US-5.3.1.5 Verify Zipper-Free Parameter Sweeps

**As an** engine tester (P-27), **I want to** sweep cutoff from 200 Hz to 8 kHz over 100 ms and
verify no audible clicks or zipper artifacts, **so that** per-sample smoothing works.

## US-5.3.1.6 Implement Biquad Filter DSP Node

**As an** engine developer (P-26), **I want to** implement biquad filters with per-sample
coefficient smoothing for LP, HP, BP, and notch types, **so that** filters are the basic DSP
building block.

## US-5.3.1.7 Hear Muffled Underwater Audio

**As a** player (P-23), **I want** audio to sound muffled when my character is underwater, **so
that** the acoustic environment changes with context.

---

## F-5.3.2 Parametric Equalizer

## US-5.3.2.1 Configure Multi-Band EQ

**As an** audio designer (P-14), **I want to** configure up to 8 EQ bands per bus with peak, shelf,
and pass shapes, **so that** each bus has precise tonal control.

## US-5.3.2.2 Assign EQ Profiles to Reverb Zones

**As an** audio designer (P-14), **I want to** assign different EQ profiles to reverb zones, **so
that** each acoustic space has a distinct tonal character.

## US-5.3.2.3 Set Up EQ in Editor

**As a** designer (P-5), **I want to** configure EQ bands and profiles in the visual editor, **so
that** equalization is authored without code.

## US-5.3.2.4 Verify EQ Band Accuracy

**As an** engine tester (P-27), **I want to** configure an 8-band EQ with known parameters, process
a flat-spectrum signal, and verify each band's gain matches within 0.5 dB, **so that** EQ precision
is confirmed.

## US-5.3.2.5 Implement Parametric EQ Node

**As an** engine developer (P-26), **I want to** implement a multi-band parametric EQ as a bus
insert, **so that** tonal shaping is available per-bus and per-zone.

## US-5.3.2.6 Hear Tonal Differences Between Environments

**As a** player (P-23), **I want** underground caves to sound different from open fields in tone,
**so that** environments are acoustically distinct.

---

## F-5.3.3 Algorithmic Reverb

## US-5.3.3.1 Configure Algorithmic Reverb Parameters

**As an** audio designer (P-14), **I want to** set pre-delay, decay time, diffusion, damping, and
wet/dry mix on the algorithmic reverb, **so that** room character is tunable.

## US-5.3.3.2 Use Algorithmic Reverb as Default

**As a** designer (P-5), **I want** algorithmic reverb to be the default reverb type in the project,
**so that** open-world environments have reverb without impulse responses.

## US-5.3.3.3 Verify Decay Time Accuracy

**As an** engine tester (P-27), **I want to** process an impulse with decay time 2.0s and verify the
energy decay curve matches 2.0s +/- 10%, **so that** reverb timing is accurate.

## US-5.3.3.4 Benchmark Algorithmic Reverb Cost

**As an** engine tester (P-27), **I want to** measure CPU usage per reverb instance and verify it
does not exceed 5% of one core, **so that** reverb cost is bounded.

## US-5.3.3.5 Implement FDN Reverb

**As an** engine developer (P-26), **I want to** implement a feedback delay network reverb with
configurable delay line counts per platform, **so that** algorithmic reverb is available on all
tiers.

## US-5.3.3.6 Hear Room Echo and Reverberation

**As a** player (P-23), **I want** sounds in enclosed spaces to echo and reverberate, **so that**
rooms feel acoustically realistic.

---

## F-5.3.4 Convolution Reverb

## US-5.3.4.1 Load Impulse Response Assets

**As an** audio designer (P-14), **I want to** load user-supplied impulse response (IR) assets for
hero locations, **so that** specific spaces reproduce real acoustic signatures.

## US-5.3.4.2 Assign Convolution Reverb to Hero Rooms

**As a** designer (P-5), **I want to** assign convolution reverb to specific reverb zones (throne
rooms, boss arenas) in the editor, **so that** premium spaces sound authentic.

## US-5.3.4.3 Verify Convolution Accuracy

**As an** engine tester (P-27), **I want to** convolve a test impulse with a known IR and verify
sample-level accuracy within -60 dB of a reference, **so that** convolution is precise.

## US-5.3.4.4 Verify Convolution Latency

**As an** engine tester (P-27), **I want to** verify convolution output latency does not exceed one
audio buffer period, **so that** latency is acceptable.

## US-5.3.4.5 Implement Partitioned FFT Convolution

**As an** engine developer (P-26), **I want to** implement partitioned FFT convolution that streams
IR assets via the streaming system, **so that** convolution reverb is available for hero locations.

## US-5.3.4.6 Hear Authentic Room Acoustics in Special Locations

**As a** player (P-23), **I want** throne rooms and cathedrals to sound exactly like real large
spaces, **so that** key locations have premium acoustic quality.

---

## F-5.3.5 Compressor, Limiter, and Dynamics Processing

## US-5.3.5.1 Configure Bus Compressors

**As an** audio designer (P-14), **I want to** add compressor inserts with threshold, ratio, attack,
release, knee, and makeup gain to any bus, **so that** dynamics are controlled per category.

## US-5.3.5.2 Enable Master Bus Limiter

**As an** audio designer (P-14), **I want** a look-ahead limiter on the master bus that prevents
digital clipping, **so that** output never exceeds 0 dBFS.

## US-5.3.5.3 Configure Dynamics in Editor

**As a** designer (P-5), **I want to** set compressor and limiter parameters in the visual editor,
**so that** dynamics processing is authored without code.

## US-5.3.5.4 Verify Compression Curve Accuracy

**As an** engine tester (P-27), **I want to** feed a signal 12 dB above threshold at ratio 4:1 and
verify output matches the compression curve within 0.5 dB, **so that** compression is accurate.

## US-5.3.5.5 Verify Limiter Prevents Clipping

**As an** engine tester (P-27), **I want to** feed a signal 6 dB above 0 dBFS into the master
limiter and verify output peak never exceeds 0 dBFS, **so that** clipping prevention works.

## US-5.3.5.6 Implement Compressor and Limiter Nodes

**As an** engine developer (P-26), **I want to** implement per-bus compressor and look-ahead limiter
DSP nodes, **so that** dynamics processing is available.

## US-5.3.5.7 Hear Clean Audio Without Distortion

**As a** player (P-23), **I want** audio to never distort even during chaotic scenes with many
simultaneous sounds, **so that** sound quality is maintained.

---

## F-5.3.6 Delay, Chorus, and Flanger

## US-5.3.6.1 Configure Delay Effects

**As an** audio designer (P-14), **I want to** add delay effects with configurable delay time and
feedback, **so that** echo and repeat effects are available.

## US-5.3.6.2 Configure Chorus and Flanger Effects

**As an** audio designer (P-14), **I want to** add chorus and flanger effects to send buses, **so
that** modulated delay effects are available for creative sound design.

## US-5.3.6.3 Set Up Time Effects in Editor

**As a** designer (P-5), **I want to** configure delay, chorus, and flanger effects on buses in the
visual editor, **so that** time effects are data-driven.

## US-5.3.6.4 Verify Delay Echo Timing

**As an** engine tester (P-27), **I want to** configure a 500ms delay with 50% feedback, process an
impulse, and verify echoes at 500ms intervals with 6 dB decay within 0.5 dB, **so that** delay
timing and feedback are accurate.

## US-5.3.6.5 Verify Chorus Pitch Modulation

**As an** engine tester (P-27), **I want to** verify chorus produces measurable pitch modulation in
the output spectrum, **so that** the effect is functioning.

## US-5.3.6.6 Implement Delay-Line Time Effects

**As an** engine developer (P-26), **I want to** implement delay, chorus, and flanger DSP nodes
based on delay lines, **so that** time-based effects are available.

## US-5.3.6.7 Hear Echoes in Canyons

**As a** player (P-23), **I want** sounds in canyons and large spaces to echo realistically, **so
that** environments have distinctive acoustic character.

---

## F-5.3.7 Pitch Shifting

## US-5.3.7.1 Apply Pitch Shifting to Voices

**As an** audio designer (P-14), **I want to** shift pitch independently of playback speed within
+/- 12 semitones, **so that** voice modulation and slow-motion effects are possible.

## US-5.3.7.2 Configure Pitch Shift in Editor

**As a** designer (P-5), **I want to** set pitch shift amounts on sounds in the editor, **so that**
pitch effects are configured without code.

## US-5.3.7.3 Verify Pitch Shift Accuracy

**As an** engine tester (P-27), **I want to** shift a 440 Hz tone up 12 semitones and verify output
is 880 Hz +/- 1% with artifacts below -40 dB, **so that** shift accuracy is confirmed.

## US-5.3.7.4 Verify Duration Preservation

**As an** engine tester (P-27), **I want to** verify pitch-shifted output duration equals input
duration within 1 ms, **so that** time-stretching is not introduced.

## US-5.3.7.5 Implement Pitch Shifting DSP Node

**As an** engine developer (P-26), **I want to** implement pitch shifting using phase-vocoder
(desktop) and time-domain OLA (mobile), **so that** pitch shifting is available on all platforms.

## US-5.3.7.6 Hear Demon Voice and Chipmunk Effects

**As a** player (P-23), **I want** voice modulation effects (demon voice, chipmunk) to sound clean,
**so that** character audio effects are immersive.

---

## F-5.3.8 Custom DSP Node Chains

## US-5.3.8.1 Register Custom DSP Nodes

**As an** audio designer (P-14), **I want to** register custom DSP processing nodes and insert them
at any point in the mixer bus graph, **so that** game-specific audio effects are possible.

## US-5.3.8.2 Configure DSP Chains in Editor

**As a** designer (P-5), **I want to** build DSP effect chains by inserting nodes into buses in the
visual editor, **so that** effect routing is authored visually.

## US-5.3.8.3 Verify Custom Node Processing

**As an** engine tester (P-27), **I want to** register a custom 6 dB gain node, insert it into a
bus, and verify output increases by 6 dB +/- 0.1 dB, **so that** custom node processing is correct.

## US-5.3.8.4 Verify Runtime Node Removal

**As an** engine tester (P-27), **I want to** remove a custom DSP node at runtime and verify the bus
output reverts immediately, **so that** runtime chain modification works.

## US-5.3.8.5 Stress Test DSP Chain Capacity

**As an** engine tester (P-27), **I want to** fill DSP chains to platform limits (8-12 mobile, 32+
desktop) and verify no audio underruns, **so that** chain capacity meets requirements.

## US-5.3.8.6 Implement Custom DSP Node Registry

**As an** engine developer (P-26), **I want to** implement the custom DSP node registry with
stateless process callbacks receiving audio buffers and parameter blocks, **so that** the effect
pipeline is extensible.

## US-5.3.8.7 Hear Game-Specific Audio Effects

**As a** player (P-23), **I want** unique game-specific audio effects (magical distortion,
environmental processing) to enhance immersion, **so that** the soundscape is distinctive.

---

## Non-Functional Requirements

### R-5.3.NF1 DSP Chain Per-Voice Budget

The engine **SHALL** process a 4-insert DSP chain (filter, EQ, compressor, and one modulation
effect) on a single voice within 1 microsecond per audio buffer sample at 48 kHz, enabling full DSP
chains on at least 64 simultaneous voices within the audio thread budget.

- **Derived from:** R-5.1.NF1, F-5.3.8
- **Rationale:** DSP processing is the second-largest consumer of audio thread time after
  spatialization. Per-voice cost must be bounded to scale with the target voice count.
- **Verification:** Benchmark: process a 4-insert chain on 64 voices over 10,000 buffer callbacks.
  Assert p99 per-sample cost is below 1 microsecond on minimum-spec hardware.
