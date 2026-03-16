# User Stories — 5.3 DSP & Effects

## US-5.3.1.1 Apply Low-Pass Filter for Occlusion

**As an** audio designer (P-14), **I want** low-pass biquad filters for occlusion muffling,
**so that** occluded sounds lose high frequencies.

## US-5.3.1.2 Apply High-Pass Filter for Radio Voice

**As an** audio designer (P-14), **I want** high-pass filters for radio-voice effects, **so that**
communication audio sounds tinny.

## US-5.3.1.3 Implement Biquad Filter DSP Node

**As an** engine developer (P-26), **I want** configurable biquad filters (LP, HP, BP, notch) with
cutoff, Q, and gain, **so that** tonal shaping is available.

## US-5.3.1.4 Smooth Coefficient Updates Per Sample

**As an** engine developer (P-26), **I want** filter coefficient updates smoothed per-sample,
**so that** parameter changes do not produce zipper noise.

## US-5.3.1.5 Hear Underwater Muffling Effect

**As a** player (P-23), **I want** sounds to muffle when I go underwater, **so that** the acoustic
environment changes believably.

## US-5.3.1.6 Verify Filter Frequency Response

**As an** engine tester (P-27), **I want to** verify each filter type produces the correct frequency
response, **so that** filter accuracy is validated.

## US-5.3.1.7 Configure Filters in Visual Editor

**As a** designer (P-5), **I want to** configure filter parameters in the visual editor, **so that**
audio effects are tuned visually.

## US-5.3.1.8 Apply Band-Pass for Communication Effects

**As an** audio designer (P-14), **I want** band-pass filters for walkie-talkie effects, **so that**
communication channels sound distinct.

## US-5.3.1.9 Test Filter Stability at Extreme Settings

**As an** engine tester (P-27), **I want to** test filters at extreme cutoff and Q values,
**so that** no instability or NaN artifacts occur.

## US-5.3.1.10 Hear Phone Call Effect on Voice Chat

**As a** player (P-23), **I want** in-game phone calls to sound like phone calls, **so that** the
audio treatment matches the context.

## US-5.3.1.11 Chain Multiple Filters Per Voice

**As an** engine developer (P-26), **I want** per-voice filter chains within voice budget,
**so that** complex per-source effects are possible.

## US-5.3.1.12 Verify Zipper-Free Parameter Changes

**As an** engine tester (P-27), **I want to** verify no zipper noise when sweeping filter
parameters, **so that** smooth modulation is confirmed.

## US-5.3.2.1 Insert Multi-Band EQ on Bus

**As an** audio designer (P-14), **I want** a multi-band parametric EQ as a bus insert, **so that**
tonal character is shaped per category.

## US-5.3.2.2 Support Peak, Shelf, and Pass Shapes

**As an** engine developer (P-26), **I want** peak, shelf, and pass filter shapes per EQ band,
**so that** flexible equalization is possible.

## US-5.3.2.3 Author EQ Profiles Per Reverb Zone

**As an** audio designer (P-14), **I want** EQ profiles per reverb zone, **so that** underwater
sounds distinct from a stone cathedral.

## US-5.3.2.4 Hear Underwater Audio Treatment

**As a** player (P-23), **I want** underwater audio to sound muffled and bass-heavy, **so that** the
underwater environment is immersive.

## US-5.3.2.5 Verify Band Count Per Platform

**As an** engine tester (P-27), **I want to** confirm mobile caps at 4 EQ bands, Switch 6, desktop
8, **so that** DSP budget scales per tier.

## US-5.3.2.6 Configure EQ in Visual Editor

**As a** designer (P-5), **I want to** configure EQ band parameters in the visual editor,
**so that** equalization is visual.

## US-5.3.2.7 Apply EQ Per Bus

**As an** audio designer (P-14), **I want** EQ profiles applicable per bus, **so that** each audio
category has its own tonal profile.

## US-5.3.2.8 Test EQ Frequency Response Accuracy

**As an** engine tester (P-27), **I want to** verify EQ frequency response matches configured bands,
**so that** equalization is accurate.

## US-5.3.2.9 Hear Distinct Cathedral Acoustics

**As a** player (P-23), **I want** cathedral environments to have distinct reverberant EQ,
**so that** acoustic spaces feel unique.

## US-5.3.2.10 Switch EQ Profiles at Runtime

**As an** engine developer (P-26), **I want** EQ profiles switchable at runtime, **so that** zone
transitions change EQ smoothly.

## US-5.3.2.11 Configure EQ Per Environment

**As a** designer (P-5), **I want** different EQ profiles for indoor, outdoor, and underwater,
**so that** each environment sounds distinct.

## US-5.3.2.12 Profile EQ CPU Cost Per Band Count

**As an** engine tester (P-27), **I want to** profile EQ CPU cost scaling with band count,
**so that** platform limits are validated.

## US-5.3.3.1 Apply Algorithmic Reverb on Bus

**As an** audio designer (P-14), **I want** algorithmic reverb (FDN) on buses, **so that**
open-world environments have spatial reverb.

## US-5.3.3.2 Implement Feedback Delay Network

**As an** engine developer (P-26), **I want** a feedback delay network with pre-delay, decay,
diffusion, damping, and wet/dry, **so that** reverb is tunable.

## US-5.3.3.3 Hear Reverb in Open World

**As a** player (P-23), **I want** outdoor and indoor environments to have appropriate reverb,
**so that** acoustics match the space.

## US-5.3.3.4 Verify Default on Mobile and Switch

**As an** engine tester (P-27), **I want to** confirm algorithmic reverb is default on mobile and
Switch, **so that** budget-friendly reverb is used.

## US-5.3.3.5 Configure Reverb Per Zone

**As an** audio designer (P-14), **I want** reverb parameters configurable per zone, **so that**
each space sounds unique.

## US-5.3.3.6 Verify FDN Delay Count Per Platform

**As an** engine tester (P-27), **I want to** confirm FDN delay line count (mobile 4, Switch 8,
desktop 16), **so that** budget scales per tier.

## US-5.3.3.7 Set Reverb in Visual Editor

**As a** designer (P-5), **I want to** set reverb parameters in the visual editor, **so that**
acoustic tuning is visual.

## US-5.3.3.8 Test Reverb Decay Accuracy

**As an** engine tester (P-27), **I want to** verify reverb decay time matches configured values,
**so that** behavior is predictable.

## US-5.3.3.9 Hear Canyon Echo

**As a** player (P-23), **I want** canyon environments to have long, diffuse echo, **so that** open
spaces sound expansive.

## US-5.3.3.10 Blend Reverb Presets

**As an** engine developer (P-26), **I want** smooth blending between reverb presets, **so that**
zone transitions are gradual.

## US-5.3.3.11 Use Reverb for Many Distinct Spaces

**As an** audio designer (P-14), **I want** algorithmic reverb for many distinct open-world spaces,
**so that** each area has appropriate acoustics.

## US-5.3.3.12 Verify Diffusion Quality on Mobile

**As an** engine tester (P-27), **I want to** verify reduced diffusion on mobile still sounds
acceptable, **so that** quality tradeoff is validated.

## US-5.3.4.1 Convolve with Impulse Responses

**As an** audio designer (P-14), **I want** convolution reverb using supplied IRs, **so that** hero
locations have realistic acoustics.

## US-5.3.4.2 Implement Partitioned FFT Convolution

**As an** engine developer (P-26), **I want** partitioned FFT convolution within one buffer latency,
**so that** convolution reverb runs in real time.

## US-5.3.4.3 Hear Realistic Throne Room Acoustics

**As a** player (P-23), **I want** throne rooms to have realistic reverb, **so that** hero locations
sound special.

## US-5.3.4.4 Load IR via Streaming System

**As an** engine developer (P-26), **I want** IR assets loaded via streaming, **so that** large
responses do not block startup.

## US-5.3.4.5 Verify Unavailable on Mobile

**As an** engine tester (P-27), **I want to** confirm convolution reverb is unavailable on mobile,
**so that** CPU budget is met.

## US-5.3.4.6 Author IR Presets for Key Locations

**As an** audio designer (P-14), **I want** IR assets assigned to specific zones, **so that** boss
arenas have unique acoustics.

## US-5.3.4.7 Cap IR Length on Switch

**As an** engine tester (P-27), **I want to** confirm Switch caps IR at 0.5s, **so that** FFT count
stays within budget.

## US-5.3.4.8 Select Reverb Type in Editor

**As a** designer (P-5), **I want to** select convolution or algorithmic per zone, **so that** hero
locations get premium reverb.

## US-5.3.4.9 Test Various IR Lengths

**As an** engine tester (P-27), **I want to** test convolution with various IR lengths, **so that**
all sizes work correctly.

## US-5.3.4.10 Hear Boss Arena Acoustics

**As a** player (P-23), **I want** boss arenas to have unique, dramatic reverb, **so that**
encounters sound epic.

## US-5.3.4.11 Support Long IR on Desktop

**As an** engine developer (P-26), **I want** desktop to support 2s+ IR lengths, **so that** large
spaces can be captured.

## US-5.3.4.12 Verify Latency Within Buffer

**As an** engine tester (P-27), **I want to** verify convolution latency stays within one audio
buffer, **so that** real-time performance is met.

## US-5.3.5.1 Apply Compressor on Bus

**As an** audio designer (P-14), **I want** per-bus compressor inserts, **so that** dynamic range is
controlled per category.

## US-5.3.5.2 Apply Limiter on Master Bus

**As an** engine developer (P-26), **I want** a look-ahead limiter on the master bus, **so that**
digital clipping is prevented.

## US-5.3.5.3 Hear Clean Audio Without Distortion

**As a** player (P-23), **I want** audio to never distort, **so that** output quality is consistent.

## US-5.3.5.4 Configure Compressor Parameters

**As an** audio designer (P-14), **I want** threshold, ratio, attack, release, knee, and makeup gain
configurable, **so that** dynamics are tunable.

## US-5.3.5.5 Verify Limiter Prevents Clipping

**As an** engine tester (P-27), **I want to** verify the master limiter prevents clipping under
maximum load, **so that** output never exceeds 0dBFS.

## US-5.3.5.6 Limit Compressors on Mobile

**As an** engine tester (P-27), **I want to** confirm mobile limits compressors to master + SFX bus
only, **so that** DSP budget is controlled.

## US-5.3.5.7 Set Dynamics in Visual Editor

**As a** designer (P-5), **I want to** configure dynamics in the visual editor, **so that**
processing is visual.

## US-5.3.5.8 Test Under Raid Combat Load

**As an** engine tester (P-27), **I want to** test dynamics during simulated raid combat,
**so that** the mix stays clean.

## US-5.3.5.9 Hear Balanced Combat Audio

**As a** player (P-23), **I want** balanced audio during intense combat, **so that** loud scenes do
not overwhelm.

## US-5.3.5.10 Apply Sidechain for Ducking

**As an** audio designer (P-14), **I want** sidechain compression for dialogue ducking, **so that**
speech is always clear.

## US-5.3.5.11 Implement Look-Ahead Limiter

**As an** engine developer (P-26), **I want** look-ahead limiting on master bus, **so that**
transient peaks are caught.

## US-5.3.5.12 Verify Dynamics on All Platforms

**As an** engine tester (P-27), **I want to** verify dynamics processing works on all platforms,
**so that** clipping prevention is universal.

## US-5.3.6.1 Apply Delay on Send Bus

**As an** audio designer (P-14), **I want** delay effects with feedback on send buses, **so that**
echo effects are routed cleanly.

## US-5.3.6.2 Implement Chorus and Flanger

**As an** engine developer (P-26), **I want** chorus and flanger effects, **so that** modulation
effects are available.

## US-5.3.6.3 Hear Canyon Echo

**As a** player (P-23), **I want** canyon environments to produce echo, **so that** open spaces
sound expansive.

## US-5.3.6.4 Configure Delay Parameters

**As an** audio designer (P-14), **I want** delay time, feedback, and wet/dry configurable,
**so that** echo is tunable.

## US-5.3.6.5 Verify Effect Count Per Platform

**As an** engine tester (P-27), **I want to** confirm mobile supports 1-2 time effects, Switch 3-4,
desktop 8+, **so that** budget scales.

## US-5.3.6.6 Apply Phasing on Magic

**As an** audio designer (P-14), **I want** flanger on magical ability sounds, **so that** spells
have otherworldly audio.

## US-5.3.6.7 Configure Effects in Visual Editor

**As a** designer (P-5), **I want to** configure time effects in the visual editor, **so that**
effects are visual.

## US-5.3.6.8 Test Delay Feedback Stability

**As an** engine tester (P-27), **I want to** test high-feedback delay for stability, **so that**
effects do not oscillate.

## US-5.3.6.9 Hear Magical Sound Design

**As a** player (P-23), **I want** spells to have distinctive modulation, **so that** abilities
sound unique.

## US-5.3.6.10 Sync Delay to Beat Clock

**As an** engine developer (P-26), **I want** delay time optionally synced to beat clock,
**so that** rhythmic delays match the score.

## US-5.3.6.11 Apply Chorus to Ambient

**As an** audio designer (P-14), **I want** chorus on ambient buses, **so that** natural
environments have richness.

## US-5.3.6.12 Verify No Aliasing in Modulation

**As an** engine tester (P-27), **I want to** verify chorus and flanger produce no aliasing,
**so that** modulation quality is clean.

## US-5.3.7.1 Shift Pitch Without Changing Duration

**As an** engine developer (P-26), **I want** pitch shifting independent of playback speed,
**so that** real-time voice modulation works.

## US-5.3.7.2 Apply Demon Voice Effect

**As an** audio designer (P-14), **I want** pitch-down for demon voices, **so that** monster speech
sounds menacing.

## US-5.3.7.3 Hear Slow-Motion Audio

**As a** player (P-23), **I want** audio to pitch-shift during slow-motion, **so that** time
dilation feels immersive.

## US-5.3.7.4 Use OLA on Mobile

**As an** engine developer (P-26), **I want** time-domain OLA on mobile, **so that** CPU cost is
minimal.

## US-5.3.7.5 Use Phase-Vocoder on Desktop

**As an** engine developer (P-26), **I want** phase-vocoder on desktop, **so that** quality is
maximized.

## US-5.3.7.6 Verify Pitch Accuracy

**As an** engine tester (P-27), **I want to** verify pitch shift produces correct frequency offset,
**so that** accuracy is validated.

## US-5.3.7.7 Configure Pitch Shift in Editor

**As a** designer (P-5), **I want to** configure pitch shift in the visual editor, **so that** voice
effects are tunable.

## US-5.3.7.8 Apply Chipmunk Effect

**As an** audio designer (P-14), **I want** pitch-up for comedic voices, **so that** humorous
characters sound appropriate.

## US-5.3.7.9 Test Quality Under Load

**As an** engine tester (P-27), **I want to** test pitch shifting on multiple voices, **so that**
quality holds under load.

## US-5.3.7.10 Hear Spell Pitch Modulation

**As a** player (P-23), **I want** spell sounds with pitch modulation, **so that** magic feels
dynamic.

## US-5.3.7.11 Modulate Pitch Dynamically

**As an** audio designer (P-14), **I want** pitch modulated by gameplay parameters, **so that**
sounds respond to game state.

## US-5.3.7.12 Compare OLA vs Phase-Vocoder

**As an** engine tester (P-27), **I want to** compare OLA and phase-vocoder quality, **so that**
method choice is validated.

## US-5.3.8.1 Register Custom DSP Nodes

**As an** engine developer (P-26), **I want** custom DSP nodes registrable via plugin API,
**so that** third-party effects extend the engine.

## US-5.3.8.2 Insert Nodes at Any Bus Point

**As an** audio designer (P-14), **I want** custom nodes insertable anywhere in the mixer,
**so that** custom effects are flexible.

## US-5.3.8.3 Implement Stateless Process Callback

**As an** engine developer (P-26), **I want** each node to implement a stateless process callback,
**so that** processing is safe and predictable.

## US-5.3.8.4 Hear Polished Custom Effects

**As a** player (P-23), **I want** game-specific audio effects to sound polished, **so that** custom
processing enhances the experience.

## US-5.3.8.5 Verify Chain Length Per Platform

**As an** engine tester (P-27), **I want to** confirm total DSP chain caps (mobile 8-12, Switch
16-24, desktop 32+), **so that** budget is controlled.

## US-5.3.8.6 Prototype Effects Rapidly

**As an** audio designer (P-14), **I want** custom DSP for rapid prototyping, **so that**
experimental effects are testable quickly.

## US-5.3.8.7 Configure Nodes in Visual Editor

**As a** designer (P-5), **I want to** configure custom node parameters visually, **so that** plugin
effects are tuned easily.

## US-5.3.8.8 Test Node Stability

**As an** engine tester (P-27), **I want to** test custom nodes with various signals, **so that**
plugin stability is validated.

## US-5.3.8.9 Add Game-Specific Processing

**As an** engine developer (P-26), **I want** game-specific DSP without engine modifications,
**so that** custom audio is modular.

## US-5.3.8.10 Experience Unique Audio Character

**As a** player (P-23), **I want** each game to have unique audio character, **so that** audio
design stands out.

## US-5.3.8.11 Pass Parameter Blocks to Callbacks

**As an** engine developer (P-26), **I want** parameter blocks passed to callbacks, **so that**
custom nodes are configurable at runtime.

## US-5.3.8.12 Profile Custom Node CPU Cost

**As an** engine tester (P-27), **I want to** profile custom node CPU cost, **so that** plugin
overhead is measured.
