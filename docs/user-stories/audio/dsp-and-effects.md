# User Stories -- 5.3 DSP & Effects

## US-5.3.1 Shape Sound with Filters and EQ
**As an** audio designer, **I want** parametric filters and multi-band EQ inserts on any
mixer bus, **so that** I can create radio-voice effects, underwater muffling, and
environment-specific tonal shaping without separate asset variants.

## US-5.3.2 Choose Between Algorithmic and Convolution Reverb
**As an** audio designer, **I want** both a lightweight algorithmic reverb for open-world
environments and a high-fidelity convolution reverb for hero locations, **so that** I can
balance acoustic quality against CPU cost per scene.

## US-5.3.3 Prevent Audio Clipping in Dense Scenes
**As a** player, **I want** a look-ahead limiter on the master bus to prevent distortion
when many sounds play simultaneously during raids, **so that** audio remains clear and
undistorted regardless of scene density.

## US-5.3.4 Apply Creative Time-Based Effects
**As an** audio designer, **I want** delay, chorus, flanger, and pitch shifting effects on
send buses, **so that** I can create echo in canyons, phasing on magical abilities, and
real-time voice modulation for character effects.

## US-5.3.5 Extend the Audio Pipeline with Custom DSP
**As an** audio designer, **I want** to register custom DSP processing nodes insertable at
any point in the mixer bus graph, **so that** third-party effects and game-specific audio
processing can be added without modifying the engine.
