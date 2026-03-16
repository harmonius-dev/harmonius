# User Stories — 5.3 DSP & Effects

## F-5.3.1

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.3.1.1 | audio designer (P-14) | I want low-pass biquad filters for occlusion muffling | occluded sounds lose high frequencies | F-5.3.1 | R-5.3.1 |
| US-5.3.1.2 | audio designer (P-14) | I want high-pass filters for radio-voice effects | communication audio sounds tinny | F-5.3.1 | R-5.3.1 |
| US-5.3.1.3 | engine developer (P-26) | I want configurable biquad filters (LP, HP, BP, notch) with cutoff, Q, and gain | tonal shaping is available | F-5.3.1 | R-5.3.1 |
| US-5.3.1.4 | engine developer (P-26) | I want filter coefficient updates smoothed per-sample | parameter changes do not produce zipper noise | F-5.3.1 | R-5.3.1 |
| US-5.3.1.5 | player (P-23) | I want sounds to muffle when I go underwater | the acoustic environment changes believably | F-5.3.1 | R-5.3.1 |
| US-5.3.1.6 | engine tester (P-27) | I want to verify each filter type produces the correct frequency response | filter accuracy is validated | F-5.3.1 | R-5.3.1 |
| US-5.3.1.7 | designer (P-5) | I want to configure filter parameters in the visual editor | audio effects are tuned visually | F-5.3.1 | R-5.3.1 |
| US-5.3.1.8 | audio designer (P-14) | I want band-pass filters for walkie-talkie effects | communication channels sound distinct | F-5.3.1 | R-5.3.1 |
| US-5.3.1.9 | engine tester (P-27) | I want to test filters at extreme cutoff and Q values | no instability or NaN artifacts occur | F-5.3.1 | R-5.3.1 |
| US-5.3.1.10 | player (P-23) | I want in-game phone calls to sound like phone calls | the audio treatment matches the context | F-5.3.1 | R-5.3.1 |
| US-5.3.1.11 | engine developer (P-26) | I want per-voice filter chains within voice budget | complex per-source effects are possible | F-5.3.1 | R-5.3.1 |
| US-5.3.1.12 | engine tester (P-27) | I want to verify no zipper noise when sweeping filter parameters | smooth modulation is confirmed | F-5.3.1 | R-5.3.1 |

## F-5.3.2

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.3.2.1 | audio designer (P-14) | I want a multi-band parametric EQ as a bus insert | tonal character is shaped per category | F-5.3.2 | R-5.3.2 |
| US-5.3.2.2 | engine developer (P-26) | I want peak, shelf, and pass filter shapes per EQ band | flexible equalization is possible | F-5.3.2 | R-5.3.2 |
| US-5.3.2.3 | audio designer (P-14) | I want EQ profiles per reverb zone | underwater sounds distinct from a stone cathedral | F-5.3.2 | R-5.3.2 |
| US-5.3.2.4 | player (P-23) | I want underwater audio to sound muffled and bass-heavy | the underwater environment is immersive | F-5.3.2 | R-5.3.2 |
| US-5.3.2.5 | engine tester (P-27) | I want to confirm mobile caps at 4 EQ bands, Switch 6, desktop 8 | DSP budget scales per tier | F-5.3.2 | R-5.3.2 |
| US-5.3.2.6 | designer (P-5) | I want to configure EQ band parameters in the visual editor | equalization is visual | F-5.3.2 | R-5.3.2 |
| US-5.3.2.7 | audio designer (P-14) | I want EQ profiles applicable per bus | each audio category has its own tonal profile | F-5.3.2 | R-5.3.2 |
| US-5.3.2.8 | engine tester (P-27) | I want to verify EQ frequency response matches configured bands | equalization is accurate | F-5.3.2 | R-5.3.2 |
| US-5.3.2.9 | player (P-23) | I want cathedral environments to have distinct reverberant EQ | acoustic spaces feel unique | F-5.3.2 | R-5.3.2 |
| US-5.3.2.10 | engine developer (P-26) | I want EQ profiles switchable at runtime | zone transitions change EQ smoothly | F-5.3.2 | R-5.3.2 |
| US-5.3.2.11 | designer (P-5) | I want different EQ profiles for indoor, outdoor, and underwater | each environment sounds distinct | F-5.3.2 | R-5.3.2 |
| US-5.3.2.12 | engine tester (P-27) | I want to profile EQ CPU cost scaling with band count | platform limits are validated | F-5.3.2 | R-5.3.2 |

## F-5.3.3

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.3.3.1 | audio designer (P-14) | I want algorithmic reverb (FDN) on buses | open-world environments have spatial reverb | F-5.3.3 | R-5.3.3 |
| US-5.3.3.2 | engine developer (P-26) | I want a feedback delay network with pre-delay, decay, diffusion, damping, and wet/dry | reverb is tunable | F-5.3.3 | R-5.3.3 |
| US-5.3.3.3 | player (P-23) | I want outdoor and indoor environments to have appropriate reverb | acoustics match the space | F-5.3.3 | R-5.3.3 |
| US-5.3.3.4 | engine tester (P-27) | I want to confirm algorithmic reverb is default on mobile and Switch | budget-friendly reverb is used | F-5.3.3 | R-5.3.3 |
| US-5.3.3.5 | audio designer (P-14) | I want reverb parameters configurable per zone | each space sounds unique | F-5.3.3 | R-5.3.3 |
| US-5.3.3.6 | engine tester (P-27) | I want to confirm FDN delay line count (mobile 4, Switch 8, desktop 16) | budget scales per tier | F-5.3.3 | R-5.3.3 |
| US-5.3.3.7 | designer (P-5) | I want to set reverb parameters in the visual editor | acoustic tuning is visual | F-5.3.3 | R-5.3.3 |
| US-5.3.3.8 | engine tester (P-27) | I want to verify reverb decay time matches configured values | behavior is predictable | F-5.3.3 | R-5.3.3 |
| US-5.3.3.9 | player (P-23) | I want canyon environments to have long, diffuse echo | open spaces sound expansive | F-5.3.3 | R-5.3.3 |
| US-5.3.3.10 | engine developer (P-26) | I want smooth blending between reverb presets | zone transitions are gradual | F-5.3.3 | R-5.3.3 |
| US-5.3.3.11 | audio designer (P-14) | I want algorithmic reverb for many distinct open-world spaces | each area has appropriate acoustics | F-5.3.3 | R-5.3.3 |
| US-5.3.3.12 | engine tester (P-27) | I want to verify reduced diffusion on mobile still sounds acceptable | quality tradeoff is validated | F-5.3.3 | R-5.3.3 |

## F-5.3.4

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.3.4.1 | audio designer (P-14) | I want convolution reverb using supplied IRs | hero locations have realistic acoustics | F-5.3.4 | R-5.3.4 |
| US-5.3.4.2 | engine developer (P-26) | I want partitioned FFT convolution within one buffer latency | convolution reverb runs in real time | F-5.3.4 | R-5.3.4 |
| US-5.3.4.3 | player (P-23) | I want throne rooms to have realistic reverb | hero locations sound special | F-5.3.4 | R-5.3.4 |
| US-5.3.4.4 | engine developer (P-26) | I want IR assets loaded via streaming | large responses do not block startup | F-5.3.4 | R-5.3.4 |
| US-5.3.4.5 | engine tester (P-27) | I want to confirm convolution reverb is unavailable on mobile | CPU budget is met | F-5.3.4 | R-5.3.4 |
| US-5.3.4.6 | audio designer (P-14) | I want IR assets assigned to specific zones | boss arenas have unique acoustics | F-5.3.4 | R-5.3.4 |
| US-5.3.4.7 | engine tester (P-27) | I want to confirm Switch caps IR at 0.5s | FFT count stays within budget | F-5.3.4 | R-5.3.4 |
| US-5.3.4.8 | designer (P-5) | I want to select convolution or algorithmic per zone | hero locations get premium reverb | F-5.3.4 | R-5.3.4 |
| US-5.3.4.9 | engine tester (P-27) | I want to test convolution with various IR lengths | all sizes work correctly | F-5.3.4 | R-5.3.4 |
| US-5.3.4.10 | player (P-23) | I want boss arenas to have unique, dramatic reverb | encounters sound epic | F-5.3.4 | R-5.3.4 |
| US-5.3.4.11 | engine developer (P-26) | I want desktop to support 2s+ IR lengths | large spaces can be captured | F-5.3.4 | R-5.3.4 |
| US-5.3.4.12 | engine tester (P-27) | I want to verify convolution latency stays within one audio buffer | real-time performance is met | F-5.3.4 | R-5.3.4 |

## F-5.3.5

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.3.5.1 | audio designer (P-14) | I want per-bus compressor inserts | dynamic range is controlled per category | F-5.3.5 | R-5.3.5 |
| US-5.3.5.2 | engine developer (P-26) | I want a look-ahead limiter on the master bus | digital clipping is prevented | F-5.3.5 | R-5.3.5 |
| US-5.3.5.3 | player (P-23) | I want audio to never distort | output quality is consistent | F-5.3.5 | R-5.3.5 |
| US-5.3.5.4 | audio designer (P-14) | I want threshold, ratio, attack, release, knee, and makeup gain configurable | dynamics are tunable | F-5.3.5 | R-5.3.5 |
| US-5.3.5.5 | engine tester (P-27) | I want to verify the master limiter prevents clipping under maximum load | output never exceeds 0dBFS | F-5.3.5 | R-5.3.5 |
| US-5.3.5.6 | engine tester (P-27) | I want to confirm mobile limits compressors to master + SFX bus only | DSP budget is controlled | F-5.3.5 | R-5.3.5 |
| US-5.3.5.7 | designer (P-5) | I want to configure dynamics in the visual editor | processing is visual | F-5.3.5 | R-5.3.5 |
| US-5.3.5.8 | engine tester (P-27) | I want to test dynamics during simulated raid combat | the mix stays clean | F-5.3.5 | R-5.3.5 |
| US-5.3.5.9 | player (P-23) | I want balanced audio during intense combat | loud scenes do not overwhelm | F-5.3.5 | R-5.3.5 |
| US-5.3.5.10 | audio designer (P-14) | I want sidechain compression for dialogue ducking | speech is always clear | F-5.3.5 | R-5.3.5 |
| US-5.3.5.11 | engine developer (P-26) | I want look-ahead limiting on master bus | transient peaks are caught | F-5.3.5 | R-5.3.5 |
| US-5.3.5.12 | engine tester (P-27) | I want to verify dynamics processing works on all platforms | clipping prevention is universal | F-5.3.5 | R-5.3.5 |

## F-5.3.6

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.3.6.1 | audio designer (P-14) | I want delay effects with feedback on send buses | echo effects are routed cleanly | F-5.3.6 | R-5.3.6 |
| US-5.3.6.2 | engine developer (P-26) | I want chorus and flanger effects | modulation effects are available | F-5.3.6 | R-5.3.6 |
| US-5.3.6.3 | player (P-23) | I want canyon environments to produce echo | open spaces sound expansive | F-5.3.6 | R-5.3.6 |
| US-5.3.6.4 | audio designer (P-14) | I want delay time, feedback, and wet/dry configurable | echo is tunable | F-5.3.6 | R-5.3.6 |
| US-5.3.6.5 | engine tester (P-27) | I want to confirm mobile supports 1-2 time effects, Switch 3-4, desktop 8+ | budget scales | F-5.3.6 | R-5.3.6 |
| US-5.3.6.6 | audio designer (P-14) | I want flanger on magical ability sounds | spells have otherworldly audio | F-5.3.6 | R-5.3.6 |
| US-5.3.6.7 | designer (P-5) | I want to configure time effects in the visual editor | effects are visual | F-5.3.6 | R-5.3.6 |
| US-5.3.6.8 | engine tester (P-27) | I want to test high-feedback delay for stability | effects do not oscillate | F-5.3.6 | R-5.3.6 |
| US-5.3.6.9 | player (P-23) | I want spells to have distinctive modulation | abilities sound unique | F-5.3.6 | R-5.3.6 |
| US-5.3.6.10 | engine developer (P-26) | I want delay time optionally synced to beat clock | rhythmic delays match the score | F-5.3.6 | R-5.3.6 |
| US-5.3.6.11 | audio designer (P-14) | I want chorus on ambient buses | natural environments have richness | F-5.3.6 | R-5.3.6 |
| US-5.3.6.12 | engine tester (P-27) | I want to verify chorus and flanger produce no aliasing | modulation quality is clean | F-5.3.6 | R-5.3.6 |

## F-5.3.7

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.3.7.1 | engine developer (P-26) | I want pitch shifting independent of playback speed | real-time voice modulation works | F-5.3.7 | R-5.3.7 |
| US-5.3.7.2 | audio designer (P-14) | I want pitch-down for demon voices | monster speech sounds menacing | F-5.3.7 | R-5.3.7 |
| US-5.3.7.3 | player (P-23) | I want audio to pitch-shift during slow-motion | time dilation feels immersive | F-5.3.7 | R-5.3.7 |
| US-5.3.7.4 | engine developer (P-26) | I want time-domain OLA on mobile | CPU cost is minimal | F-5.3.7 | R-5.3.7 |
| US-5.3.7.5 | engine developer (P-26) | I want phase-vocoder on desktop | quality is maximized | F-5.3.7 | R-5.3.7 |
| US-5.3.7.6 | engine tester (P-27) | I want to verify pitch shift produces correct frequency offset | accuracy is validated | F-5.3.7 | R-5.3.7 |
| US-5.3.7.7 | designer (P-5) | I want to configure pitch shift in the visual editor | voice effects are tunable | F-5.3.7 | R-5.3.7 |
| US-5.3.7.8 | audio designer (P-14) | I want pitch-up for comedic voices | humorous characters sound appropriate | F-5.3.7 | R-5.3.7 |
| US-5.3.7.9 | engine tester (P-27) | I want to test pitch shifting on multiple voices | quality holds under load | F-5.3.7 | R-5.3.7 |
| US-5.3.7.10 | player (P-23) | I want spell sounds with pitch modulation | magic feels dynamic | F-5.3.7 | R-5.3.7 |
| US-5.3.7.11 | audio designer (P-14) | I want pitch modulated by gameplay parameters | sounds respond to game state | F-5.3.7 | R-5.3.7 |
| US-5.3.7.12 | engine tester (P-27) | I want to compare OLA and phase-vocoder quality | method choice is validated | F-5.3.7 | R-5.3.7 |

## F-5.3.8

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.3.8.1 | engine developer (P-26) | I want custom DSP nodes registrable via plugin API | third-party effects extend the engine | F-5.3.8 | R-5.3.8 |
| US-5.3.8.2 | audio designer (P-14) | I want custom nodes insertable anywhere in the mixer | custom effects are flexible | F-5.3.8 | R-5.3.8 |
| US-5.3.8.3 | engine developer (P-26) | I want each node to implement a stateless process callback | processing is safe and predictable | F-5.3.8 | R-5.3.8 |
| US-5.3.8.4 | player (P-23) | I want game-specific audio effects to sound polished | custom processing enhances the experience | F-5.3.8 | R-5.3.8 |
| US-5.3.8.5 | engine tester (P-27) | I want to confirm total DSP chain caps (mobile 8-12, Switch 16-24, desktop 32+) | budget is controlled | F-5.3.8 | R-5.3.8 |
| US-5.3.8.6 | audio designer (P-14) | I want custom DSP for rapid prototyping | experimental effects are testable quickly | F-5.3.8 | R-5.3.8 |
| US-5.3.8.7 | designer (P-5) | I want to configure custom node parameters visually | plugin effects are tuned easily | F-5.3.8 | R-5.3.8 |
| US-5.3.8.8 | engine tester (P-27) | I want to test custom nodes with various signals | plugin stability is validated | F-5.3.8 | R-5.3.8 |
| US-5.3.8.9 | engine developer (P-26) | I want game-specific DSP without engine modifications | custom audio is modular | F-5.3.8 | R-5.3.8 |
| US-5.3.8.10 | player (P-23) | I want each game to have unique audio character | audio design stands out | F-5.3.8 | R-5.3.8 |
| US-5.3.8.11 | engine developer (P-26) | I want parameter blocks passed to callbacks | custom nodes are configurable at runtime | F-5.3.8 | R-5.3.8 |
| US-5.3.8.12 | engine tester (P-27) | I want to profile custom node CPU cost | plugin overhead is measured | F-5.3.8 | R-5.3.8 |
