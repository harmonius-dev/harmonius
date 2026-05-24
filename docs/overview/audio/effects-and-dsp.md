# Effects and DSP

Real-time signal processing: reverb, delay, modulation, and spatialization.

## What it covers

- Reverb: room modeling via impulse response convolution or algorithmic methods.
- Delay: echo effects with configurable time and feedback.
- Modulation effects: chorus, flanger, phaser, tremolo.
- Distortion: saturation and overdrive for aggressive character.
- Ring modulation and bit crushing for special effects.
- Pitch shifting without time stretching.
- Time stretching without pitch shifting.
- Sidechain compression: ducking one signal based on another.
- Parametric equalization and filters.
- Real-time analysis: FFT spectrum for visualization or game feedback.

## Concepts

### Reverberation

Reverberation models room acoustics: sound bounces off walls, ceiling, floor before reaching the
listener. Convolution reverb uses measured room impulse responses (IRs) applied to the dry signal,
producing authentic acoustics. Algorithmic reverb (Schroeder reverberators) chains delay lines and
feedback networks to approximate reverb without measured IRs. Reverb density ranges from sparse
(cathedral) to dense (small room).

### Delays and Modulation

Simple delay stores recent samples and outputs them after a time interval (e.g., 500 ms). Feedback
recirculates output back through the delay, building repeating echoes. Modulation effects vary a
parameter (delay time, oscillator phase, filter frequency) over time, creating movement: chorus
thickens by detuning; flanger sweeps a narrow notch; phaser shifts allpass filter poles.

### Time and Pitch Manipulation

Pitch shifting resamples audio at a different rate to change frequency without time stretching.
Time stretching uses overlap-add or phase vocoder techniques to change duration without pitch
shift, preserving percussive attack and vocal formants. Both are computationally expensive; the
engine applies them selectively.

### Sidechain and Ducking

Sidechain compression uses one signal (sidechain input, e.g., dialogue) to modulate another signal's
compression (music). When dialogue plays (sidechain active), the music compressor reduces music
volume. This creates the sensation of dialogue pushing through the mix without manual volume
ducking.

### Real-Time Analysis

Fast Fourier Transform (FFT) spectrum analysis runs on incoming or outgoing audio, producing
frequency bins for visualization (equalizer display, audio-reactive effects). Game code can query
the spectrum to trigger particle effects or gameplay events when bass is loud.

## How it fits

- See [engine-and-mixing.md](./engine-and-mixing.md) for bus routing effects chains.
- See [spatial-audio.md](./spatial-audio.md) for convolution reverb using measured room IRs.
- Integrates with [../vfx/particles-and-effects.md](../vfx/particles-and-effects.md) for
  audio-reactive effects.
