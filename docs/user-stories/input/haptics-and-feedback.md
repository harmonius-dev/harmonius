# User Stories -- 6.4 Haptics & Feedback

## US-6.4.1 Feel Combat Impacts Through the Controller
**As a** player, **I want** dual-motor rumble with data-driven keyframe patterns that
provide distinct tactile feedback for combat hits, ability impacts, and mount locomotion,
**so that** gameplay feels physically impactful through the controller.

## US-6.4.2 Feel Bowstring Tension in the Triggers
**As a** player, **I want** DualSense adaptive trigger resistance and vibration (bowstring
draw, fishing reel, gear shifting), **so that** physical interactions feel tactile on
supported controllers and degrade gracefully to rumble on others.

## US-6.4.3 Feel Surface Textures Through HD Haptics
**As a** player, **I want** high-fidelity haptic waveforms that reproduce surface textures
(footsteps on stone vs. grass, rain on a shield, lockpicking tumbler feedback), **so that**
fine environmental details are communicated through touch on controllers with HD haptic
actuators.

## US-6.4.4 Get Haptic Feedback Matching Every Sound
**As an** audio designer, **I want** haptic waveforms auto-generated from audio signals by
extracting low-frequency bands and amplitude envelopes, **so that** every explosion and
spell impact has matching tactile presence without hand-authoring haptic assets for each
sound.

## US-6.4.5 Author Rich Haptic Profiles as Single Assets
**As an** input designer, **I want** data-driven haptic profiles combining rumble, adaptive
triggers, and HD haptics with parameter binding and configurable fallback chains, **so that**
a single profile asset works on every controller class with appropriate degradation.
