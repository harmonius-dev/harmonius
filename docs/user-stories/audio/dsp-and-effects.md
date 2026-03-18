# User Stories — 5.3 DSP & Effects

## F-5.3.1

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.3.1.1  | audio designer (P-14)   | F-5.3.1  | R-5.3.1      |
| US-5.3.1.2  | audio designer (P-14)   | F-5.3.1  | R-5.3.1      |
| US-5.3.1.3  | engine developer (P-26) | F-5.3.1  | R-5.3.1      |
| US-5.3.1.4  | engine developer (P-26) | F-5.3.1  | R-5.3.1      |
| US-5.3.1.5  | player (P-23)           | F-5.3.1  | R-5.3.1      |
| US-5.3.1.6  | engine tester (P-27)    | F-5.3.1  | R-5.3.1      |
| US-5.3.1.7  | designer (P-5)          | F-5.3.1  | R-5.3.1      |
| US-5.3.1.8  | audio designer (P-14)   | F-5.3.1  | R-5.3.1      |
| US-5.3.1.9  | engine tester (P-27)    | F-5.3.1  | R-5.3.1      |
| US-5.3.1.10 | player (P-23)           | F-5.3.1  | R-5.3.1      |
| US-5.3.1.11 | engine developer (P-26) | F-5.3.1  | R-5.3.1      |
| US-5.3.1.12 | engine tester (P-27)    | F-5.3.1  | R-5.3.1      |

1. **US-5.3.1.1** — I want low-pass biquad filters for occlusion muffling
   - **Acceptance:** occluded sounds lose high frequencies
2. **US-5.3.1.2** — I want high-pass filters for radio-voice effects
   - **Acceptance:** communication audio sounds tinny
3. **US-5.3.1.3** — I want configurable biquad filters (LP, HP, BP, notch) with cutoff, Q, and gain
   - **Acceptance:** tonal shaping is available
4. **US-5.3.1.4** — I want filter coefficient updates smoothed per-sample
   - **Acceptance:** parameter changes do not produce zipper noise
5. **US-5.3.1.5** — I want sounds to muffle when I go underwater
   - **Acceptance:** the acoustic environment changes believably
6. **US-5.3.1.6** — I want to verify each filter type produces the correct frequency response
   - **Acceptance:** filter accuracy is validated
7. **US-5.3.1.7** — I want to configure filter parameters in the visual editor
   - **Acceptance:** audio effects are tuned visually
8. **US-5.3.1.8** — I want band-pass filters for walkie-talkie effects
   - **Acceptance:** communication channels sound distinct
9. **US-5.3.1.9** — I want to test filters at extreme cutoff and Q values
   - **Acceptance:** no instability or NaN artifacts occur
10. **US-5.3.1.10** — I want in-game phone calls to sound like phone calls
    - **Acceptance:** the audio treatment matches the context
11. **US-5.3.1.11** — I want per-voice filter chains within voice budget
    - **Acceptance:** complex per-source effects are possible
12. **US-5.3.1.12** — I want to verify no zipper noise when sweeping filter parameters
    - **Acceptance:** smooth modulation is confirmed

## F-5.3.2

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.3.2.1  | audio designer (P-14)   | F-5.3.2  | R-5.3.2      |
| US-5.3.2.2  | engine developer (P-26) | F-5.3.2  | R-5.3.2      |
| US-5.3.2.3  | audio designer (P-14)   | F-5.3.2  | R-5.3.2      |
| US-5.3.2.4  | player (P-23)           | F-5.3.2  | R-5.3.2      |
| US-5.3.2.5  | engine tester (P-27)    | F-5.3.2  | R-5.3.2      |
| US-5.3.2.6  | designer (P-5)          | F-5.3.2  | R-5.3.2      |
| US-5.3.2.7  | audio designer (P-14)   | F-5.3.2  | R-5.3.2      |
| US-5.3.2.8  | engine tester (P-27)    | F-5.3.2  | R-5.3.2      |
| US-5.3.2.9  | player (P-23)           | F-5.3.2  | R-5.3.2      |
| US-5.3.2.10 | engine developer (P-26) | F-5.3.2  | R-5.3.2      |
| US-5.3.2.11 | designer (P-5)          | F-5.3.2  | R-5.3.2      |
| US-5.3.2.12 | engine tester (P-27)    | F-5.3.2  | R-5.3.2      |

1. **US-5.3.2.1** — I want a multi-band parametric EQ as a bus insert
   - **Acceptance:** tonal character is shaped per category
2. **US-5.3.2.2** — I want peak, shelf, and pass filter shapes per EQ band
   - **Acceptance:** flexible equalization is possible
3. **US-5.3.2.3** — I want EQ profiles per reverb zone
   - **Acceptance:** underwater sounds distinct from a stone cathedral
4. **US-5.3.2.4** — I want underwater audio to sound muffled and bass-heavy
   - **Acceptance:** the underwater environment is immersive
5. **US-5.3.2.5** — I want to confirm mobile caps at 4 EQ bands, Switch 6, desktop 8
   - **Acceptance:** DSP budget scales per tier
6. **US-5.3.2.6** — I want to configure EQ band parameters in the visual editor
   - **Acceptance:** equalization is visual
7. **US-5.3.2.7** — I want EQ profiles applicable per bus
   - **Acceptance:** each audio category has its own tonal profile
8. **US-5.3.2.8** — I want to verify EQ frequency response matches configured bands
   - **Acceptance:** equalization is accurate
9. **US-5.3.2.9** — I want cathedral environments to have distinct reverberant EQ
   - **Acceptance:** acoustic spaces feel unique
10. **US-5.3.2.10** — I want EQ profiles switchable at runtime
    - **Acceptance:** zone transitions change EQ smoothly
11. **US-5.3.2.11** — I want different EQ profiles for indoor, outdoor, and underwater
    - **Acceptance:** each environment sounds distinct
12. **US-5.3.2.12** — I want to profile EQ CPU cost scaling with band count
    - **Acceptance:** platform limits are validated

## F-5.3.3

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.3.3.1  | audio designer (P-14)   | F-5.3.3  | R-5.3.3      |
| US-5.3.3.2  | engine developer (P-26) | F-5.3.3  | R-5.3.3      |
| US-5.3.3.3  | player (P-23)           | F-5.3.3  | R-5.3.3      |
| US-5.3.3.4  | engine tester (P-27)    | F-5.3.3  | R-5.3.3      |
| US-5.3.3.5  | audio designer (P-14)   | F-5.3.3  | R-5.3.3      |
| US-5.3.3.6  | engine tester (P-27)    | F-5.3.3  | R-5.3.3      |
| US-5.3.3.7  | designer (P-5)          | F-5.3.3  | R-5.3.3      |
| US-5.3.3.8  | engine tester (P-27)    | F-5.3.3  | R-5.3.3      |
| US-5.3.3.9  | player (P-23)           | F-5.3.3  | R-5.3.3      |
| US-5.3.3.10 | engine developer (P-26) | F-5.3.3  | R-5.3.3      |
| US-5.3.3.11 | audio designer (P-14)   | F-5.3.3  | R-5.3.3      |
| US-5.3.3.12 | engine tester (P-27)    | F-5.3.3  | R-5.3.3      |

1. **US-5.3.3.1** — I want algorithmic reverb (FDN) on buses
   - **Acceptance:** open-world environments have spatial reverb
2. **US-5.3.3.2** — I want a feedback delay network with pre-delay, decay, diffusion, damping, and
   wet/dry
   - **Acceptance:** reverb is tunable
3. **US-5.3.3.3** — I want outdoor and indoor environments to have appropriate reverb
   - **Acceptance:** acoustics match the space
4. **US-5.3.3.4** — I want to confirm algorithmic reverb is default on mobile and Switch
   - **Acceptance:** budget-friendly reverb is used
5. **US-5.3.3.5** — I want reverb parameters configurable per zone
   - **Acceptance:** each space sounds unique
6. **US-5.3.3.6** — I want to confirm FDN delay line count (mobile 4, Switch 8, desktop 16)
   - **Acceptance:** budget scales per tier
7. **US-5.3.3.7** — I want to set reverb parameters in the visual editor
   - **Acceptance:** acoustic tuning is visual
8. **US-5.3.3.8** — I want to verify reverb decay time matches configured values
   - **Acceptance:** behavior is predictable
9. **US-5.3.3.9** — I want canyon environments to have long, diffuse echo
   - **Acceptance:** open spaces sound expansive
10. **US-5.3.3.10** — I want smooth blending between reverb presets
    - **Acceptance:** zone transitions are gradual
11. **US-5.3.3.11** — I want algorithmic reverb for many distinct open-world spaces
    - **Acceptance:** each area has appropriate acoustics
12. **US-5.3.3.12** — I want to verify reduced diffusion on mobile still sounds acceptable
    - **Acceptance:** quality tradeoff is validated

## F-5.3.4

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.3.4.1  | audio designer (P-14)   | F-5.3.4  | R-5.3.4      |
| US-5.3.4.2  | engine developer (P-26) | F-5.3.4  | R-5.3.4      |
| US-5.3.4.3  | player (P-23)           | F-5.3.4  | R-5.3.4      |
| US-5.3.4.4  | engine developer (P-26) | F-5.3.4  | R-5.3.4      |
| US-5.3.4.5  | engine tester (P-27)    | F-5.3.4  | R-5.3.4      |
| US-5.3.4.6  | audio designer (P-14)   | F-5.3.4  | R-5.3.4      |
| US-5.3.4.7  | engine tester (P-27)    | F-5.3.4  | R-5.3.4      |
| US-5.3.4.8  | designer (P-5)          | F-5.3.4  | R-5.3.4      |
| US-5.3.4.9  | engine tester (P-27)    | F-5.3.4  | R-5.3.4      |
| US-5.3.4.10 | player (P-23)           | F-5.3.4  | R-5.3.4      |
| US-5.3.4.11 | engine developer (P-26) | F-5.3.4  | R-5.3.4      |
| US-5.3.4.12 | engine tester (P-27)    | F-5.3.4  | R-5.3.4      |

1. **US-5.3.4.1** — I want convolution reverb using supplied IRs
   - **Acceptance:** hero locations have realistic acoustics
2. **US-5.3.4.2** — I want partitioned FFT convolution within one buffer latency
   - **Acceptance:** convolution reverb runs in real time
3. **US-5.3.4.3** — I want throne rooms to have realistic reverb
   - **Acceptance:** hero locations sound special
4. **US-5.3.4.4** — I want IR assets loaded via streaming
   - **Acceptance:** large responses do not block startup
5. **US-5.3.4.5** — I want to confirm convolution reverb is unavailable on mobile
   - **Acceptance:** CPU budget is met
6. **US-5.3.4.6** — I want IR assets assigned to specific zones
   - **Acceptance:** boss arenas have unique acoustics
7. **US-5.3.4.7** — I want to confirm Switch caps IR at 0.5s
   - **Acceptance:** FFT count stays within budget
8. **US-5.3.4.8** — I want to select convolution or algorithmic per zone
   - **Acceptance:** hero locations get premium reverb
9. **US-5.3.4.9** — I want to test convolution with various IR lengths
   - **Acceptance:** all sizes work correctly
10. **US-5.3.4.10** — I want boss arenas to have unique, dramatic reverb
    - **Acceptance:** encounters sound epic
11. **US-5.3.4.11** — I want desktop to support 2s+ IR lengths
    - **Acceptance:** large spaces can be captured
12. **US-5.3.4.12** — I want to verify convolution latency stays within one audio buffer
    - **Acceptance:** real-time performance is met

## F-5.3.5

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.3.5.1  | audio designer (P-14)   | F-5.3.5  | R-5.3.5      |
| US-5.3.5.2  | engine developer (P-26) | F-5.3.5  | R-5.3.5      |
| US-5.3.5.3  | player (P-23)           | F-5.3.5  | R-5.3.5      |
| US-5.3.5.4  | audio designer (P-14)   | F-5.3.5  | R-5.3.5      |
| US-5.3.5.5  | engine tester (P-27)    | F-5.3.5  | R-5.3.5      |
| US-5.3.5.6  | engine tester (P-27)    | F-5.3.5  | R-5.3.5      |
| US-5.3.5.7  | designer (P-5)          | F-5.3.5  | R-5.3.5      |
| US-5.3.5.8  | engine tester (P-27)    | F-5.3.5  | R-5.3.5      |
| US-5.3.5.9  | player (P-23)           | F-5.3.5  | R-5.3.5      |
| US-5.3.5.10 | audio designer (P-14)   | F-5.3.5  | R-5.3.5      |
| US-5.3.5.11 | engine developer (P-26) | F-5.3.5  | R-5.3.5      |
| US-5.3.5.12 | engine tester (P-27)    | F-5.3.5  | R-5.3.5      |

1. **US-5.3.5.1** — I want per-bus compressor inserts
   - **Acceptance:** dynamic range is controlled per category
2. **US-5.3.5.2** — I want a look-ahead limiter on the master bus
   - **Acceptance:** digital clipping is prevented
3. **US-5.3.5.3** — I want audio to never distort
   - **Acceptance:** output quality is consistent
4. **US-5.3.5.4** — I want threshold, ratio, attack, release, knee, and makeup gain configurable
   - **Acceptance:** dynamics are tunable
5. **US-5.3.5.5** — I want to verify the master limiter prevents clipping under maximum load
   - **Acceptance:** output never exceeds 0dBFS
6. **US-5.3.5.6** — I want to confirm mobile limits compressors to master + SFX bus only
   - **Acceptance:** DSP budget is controlled
7. **US-5.3.5.7** — I want to configure dynamics in the visual editor
   - **Acceptance:** processing is visual
8. **US-5.3.5.8** — I want to test dynamics during simulated raid combat
   - **Acceptance:** the mix stays clean
9. **US-5.3.5.9** — I want balanced audio during intense combat
   - **Acceptance:** loud scenes do not overwhelm
10. **US-5.3.5.10** — I want sidechain compression for dialogue ducking
    - **Acceptance:** speech is always clear
11. **US-5.3.5.11** — I want look-ahead limiting on master bus
    - **Acceptance:** transient peaks are caught
12. **US-5.3.5.12** — I want to verify dynamics processing works on all platforms
    - **Acceptance:** clipping prevention is universal

## F-5.3.6

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.3.6.1  | audio designer (P-14)   | F-5.3.6  | R-5.3.6      |
| US-5.3.6.2  | engine developer (P-26) | F-5.3.6  | R-5.3.6      |
| US-5.3.6.3  | player (P-23)           | F-5.3.6  | R-5.3.6      |
| US-5.3.6.4  | audio designer (P-14)   | F-5.3.6  | R-5.3.6      |
| US-5.3.6.5  | engine tester (P-27)    | F-5.3.6  | R-5.3.6      |
| US-5.3.6.6  | audio designer (P-14)   | F-5.3.6  | R-5.3.6      |
| US-5.3.6.7  | designer (P-5)          | F-5.3.6  | R-5.3.6      |
| US-5.3.6.8  | engine tester (P-27)    | F-5.3.6  | R-5.3.6      |
| US-5.3.6.9  | player (P-23)           | F-5.3.6  | R-5.3.6      |
| US-5.3.6.10 | engine developer (P-26) | F-5.3.6  | R-5.3.6      |
| US-5.3.6.11 | audio designer (P-14)   | F-5.3.6  | R-5.3.6      |
| US-5.3.6.12 | engine tester (P-27)    | F-5.3.6  | R-5.3.6      |

1. **US-5.3.6.1** — I want delay effects with feedback on send buses
   - **Acceptance:** echo effects are routed cleanly
2. **US-5.3.6.2** — I want chorus and flanger effects
   - **Acceptance:** modulation effects are available
3. **US-5.3.6.3** — I want canyon environments to produce echo
   - **Acceptance:** open spaces sound expansive
4. **US-5.3.6.4** — I want delay time, feedback, and wet/dry configurable
   - **Acceptance:** echo is tunable
5. **US-5.3.6.5** — I want to confirm mobile supports 1-2 time effects, Switch 3-4, desktop 8+
   - **Acceptance:** budget scales
6. **US-5.3.6.6** — I want flanger on magical ability sounds
   - **Acceptance:** spells have otherworldly audio
7. **US-5.3.6.7** — I want to configure time effects in the visual editor
   - **Acceptance:** effects are visual
8. **US-5.3.6.8** — I want to test high-feedback delay for stability
   - **Acceptance:** effects do not oscillate
9. **US-5.3.6.9** — I want spells to have distinctive modulation
   - **Acceptance:** abilities sound unique
10. **US-5.3.6.10** — I want delay time optionally synced to beat clock
    - **Acceptance:** rhythmic delays match the score
11. **US-5.3.6.11** — I want chorus on ambient buses
    - **Acceptance:** natural environments have richness
12. **US-5.3.6.12** — I want to verify chorus and flanger produce no aliasing
    - **Acceptance:** modulation quality is clean

## F-5.3.7

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.3.7.1  | engine developer (P-26) | F-5.3.7  | R-5.3.7      |
| US-5.3.7.2  | audio designer (P-14)   | F-5.3.7  | R-5.3.7      |
| US-5.3.7.3  | player (P-23)           | F-5.3.7  | R-5.3.7      |
| US-5.3.7.4  | engine developer (P-26) | F-5.3.7  | R-5.3.7      |
| US-5.3.7.5  | engine developer (P-26) | F-5.3.7  | R-5.3.7      |
| US-5.3.7.6  | engine tester (P-27)    | F-5.3.7  | R-5.3.7      |
| US-5.3.7.7  | designer (P-5)          | F-5.3.7  | R-5.3.7      |
| US-5.3.7.8  | audio designer (P-14)   | F-5.3.7  | R-5.3.7      |
| US-5.3.7.9  | engine tester (P-27)    | F-5.3.7  | R-5.3.7      |
| US-5.3.7.10 | player (P-23)           | F-5.3.7  | R-5.3.7      |
| US-5.3.7.11 | audio designer (P-14)   | F-5.3.7  | R-5.3.7      |
| US-5.3.7.12 | engine tester (P-27)    | F-5.3.7  | R-5.3.7      |

1. **US-5.3.7.1** — I want pitch shifting independent of playback speed
   - **Acceptance:** real-time voice modulation works
2. **US-5.3.7.2** — I want pitch-down for demon voices
   - **Acceptance:** monster speech sounds menacing
3. **US-5.3.7.3** — I want audio to pitch-shift during slow-motion
   - **Acceptance:** time dilation feels immersive
4. **US-5.3.7.4** — I want time-domain OLA on mobile
   - **Acceptance:** CPU cost is minimal
5. **US-5.3.7.5** — I want phase-vocoder on desktop
   - **Acceptance:** quality is maximized
6. **US-5.3.7.6** — I want to verify pitch shift produces correct frequency offset
   - **Acceptance:** accuracy is validated
7. **US-5.3.7.7** — I want to configure pitch shift in the visual editor
   - **Acceptance:** voice effects are tunable
8. **US-5.3.7.8** — I want pitch-up for comedic voices
   - **Acceptance:** humorous characters sound appropriate
9. **US-5.3.7.9** — I want to test pitch shifting on multiple voices
   - **Acceptance:** quality holds under load
10. **US-5.3.7.10** — I want spell sounds with pitch modulation
    - **Acceptance:** magic feels dynamic
11. **US-5.3.7.11** — I want pitch modulated by gameplay parameters
    - **Acceptance:** sounds respond to game state
12. **US-5.3.7.12** — I want to compare OLA and phase-vocoder quality
    - **Acceptance:** method choice is validated

## F-5.3.8

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.3.8.1  | engine developer (P-26) | F-5.3.8  | R-5.3.8      |
| US-5.3.8.2  | audio designer (P-14)   | F-5.3.8  | R-5.3.8      |
| US-5.3.8.3  | engine developer (P-26) | F-5.3.8  | R-5.3.8      |
| US-5.3.8.4  | player (P-23)           | F-5.3.8  | R-5.3.8      |
| US-5.3.8.5  | engine tester (P-27)    | F-5.3.8  | R-5.3.8      |
| US-5.3.8.6  | audio designer (P-14)   | F-5.3.8  | R-5.3.8      |
| US-5.3.8.7  | designer (P-5)          | F-5.3.8  | R-5.3.8      |
| US-5.3.8.8  | engine tester (P-27)    | F-5.3.8  | R-5.3.8      |
| US-5.3.8.9  | engine developer (P-26) | F-5.3.8  | R-5.3.8      |
| US-5.3.8.10 | player (P-23)           | F-5.3.8  | R-5.3.8      |
| US-5.3.8.11 | engine developer (P-26) | F-5.3.8  | R-5.3.8      |
| US-5.3.8.12 | engine tester (P-27)    | F-5.3.8  | R-5.3.8      |

1. **US-5.3.8.1** — I want custom DSP nodes registrable via plugin API
   - **Acceptance:** third-party effects extend the engine
2. **US-5.3.8.2** — I want custom nodes insertable anywhere in the mixer
   - **Acceptance:** custom effects are flexible
3. **US-5.3.8.3** — I want each node to implement a stateless process callback
   - **Acceptance:** processing is safe and predictable
4. **US-5.3.8.4** — I want game-specific audio effects to sound polished
   - **Acceptance:** custom processing enhances the experience
5. **US-5.3.8.5** — I want to confirm total DSP chain caps (mobile 8-12, Switch 16-24, desktop 32+)
   - **Acceptance:** budget is controlled
6. **US-5.3.8.6** — I want custom DSP for rapid prototyping
   - **Acceptance:** experimental effects are testable quickly
7. **US-5.3.8.7** — I want to configure custom node parameters visually
   - **Acceptance:** plugin effects are tuned easily
8. **US-5.3.8.8** — I want to test custom nodes with various signals
   - **Acceptance:** plugin stability is validated
9. **US-5.3.8.9** — I want game-specific DSP without engine modifications
   - **Acceptance:** custom audio is modular
10. **US-5.3.8.10** — I want each game to have unique audio character
    - **Acceptance:** audio design stands out
11. **US-5.3.8.11** — I want parameter blocks passed to callbacks
    - **Acceptance:** custom nodes are configurable at runtime
12. **US-5.3.8.12** — I want to profile custom node CPU cost
    - **Acceptance:** plugin overhead is measured
