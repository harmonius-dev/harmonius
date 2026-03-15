# User Stories — 5.2 Spatial Audio

## US-5.2.1.1 Position Sounds in World Space
**As an** audio designer (P-14), **I want** sounds positioned in world space with per-listener
panning and distance gain, **so that** 3D audio placement is automatic.

## US-5.2.1.2 Apply Doppler Pitch Shift
**As an** engine developer (P-26), **I want** Doppler pitch shift computed from relative source
and listener velocities, **so that** moving objects sound realistic.

## US-5.2.1.3 Interpolate Transforms Between Ticks
**As an** engine developer (P-26), **I want** source and listener transforms interpolated
between game ticks, **so that** audio motion is smooth at audio callback rate.

## US-5.2.1.4 Hear Passing Vehicles with Doppler
**As a** player (P-23), **I want** passing vehicles to pitch shift as they approach and recede,
**so that** speed is audible.

## US-5.2.1.5 Verify Artifact-Free Motion Audio
**As an** engine tester (P-27), **I want to** verify no audio artifacts during fast source
motion, **so that** interpolation is working correctly.

## US-5.2.1.6 Configure Doppler Factor Per Source
**As an** audio designer (P-14), **I want to** configure Doppler scale factor per source, **so
that** Doppler intensity is tunable.

## US-5.2.1.7 Hear Sound Direction Change as I Turn
**As a** player (P-23), **I want** sound direction to update as I rotate the camera, **so
that** spatial orientation is correct.

## US-5.2.1.8 Test 3D Positioning with Many Sources
**As an** engine tester (P-27), **I want to** test 3D positioning with the maximum voice count,
**so that** spatial audio scales within budget.

## US-5.2.1.9 Set Up 3D Sounds in Visual Editor
**As a** designer (P-5), **I want to** set up 3D sound sources in the visual editor, **so
that** spatial audio configuration is visual.

## US-5.2.1.10 Hear Enemies Approach from Behind
**As a** player (P-23), **I want to** hear enemies approaching from behind, **so that** spatial
audio provides tactical information.

## US-5.2.1.11 Implement Per-Voice Doppler Calculation
**As an** engine developer (P-26), **I want** per-voice Doppler calculated efficiently, **so
that** it remains lightweight on all platforms.

## US-5.2.1.12 Verify Panning at Edge Cases
**As an** engine tester (P-27), **I want to** verify panning at extreme positions (directly
behind, directly above), **so that** edge case directions work correctly.

## US-5.2.2.1 Apply Distance Attenuation Per Source
**As an** audio designer (P-14), **I want to** apply configurable distance attenuation models
per source, **so that** each sound has appropriate rolloff.

## US-5.2.2.2 Support Multiple Attenuation Models
**As an** engine developer (P-26), **I want** inverse, inverse-squared, linear, logarithmic,
and custom curve attenuation models, **so that** designers have flexibility.

## US-5.2.2.3 Set Min and Max Distance Per Source
**As an** audio designer (P-14), **I want** per-source minimum and maximum distance parameters,
**so that** rolloff range is controlled per emitter.

## US-5.2.2.4 Hear Distant Sounds Fade Naturally
**As a** player (P-23), **I want** sounds to fade naturally with distance, **so that** near
sounds are loud and far sounds are quiet.

## US-5.2.2.5 Define Custom Attenuation Curves
**As an** audio designer (P-14), **I want** custom user-defined attenuation curves, **so that**
unique rolloff profiles are possible.

## US-5.2.2.6 Verify Attenuation at Key Distances
**As an** engine tester (P-27), **I want to** verify attenuation gain at min, max, and mid
distances, **so that** each model produces correct rolloff.

## US-5.2.2.7 Configure Attenuation in Visual Editor
**As a** designer (P-5), **I want to** configure attenuation model and distances in the visual
editor, **so that** rolloff is visual.

## US-5.2.2.8 Balance Distant and Near Sounds
**As an** audio designer (P-14), **I want** tunable attenuation to balance distant siege
weaponry against nearby footsteps, **so that** the mix is clear.

## US-5.2.2.9 Test All Attenuation Models
**As an** engine tester (P-27), **I want to** test all attenuation models, **so that** each
behaves as specified.

## US-5.2.2.10 Hear Nearby Footsteps Over Distant Battle
**As a** player (P-23), **I want** nearby footsteps clearly audible over distant battle sounds,
**so that** proximity is communicated.

## US-5.2.2.11 Use Logarithmic Rolloff for Realism
**As a** designer (P-5), **I want** logarithmic attenuation as a default for realism, **so
that** distance perception matches expectations.

## US-5.2.2.12 Verify Consistent Attenuation Across Platforms
**As an** engine tester (P-27), **I want to** verify attenuation produces identical results
across platforms, **so that** audio mix is consistent.

## US-5.2.3.1 Render Binaural Audio via HRTF
**As an** engine developer (P-26), **I want to** render spatialized audio through HRTF for
headphone output, **so that** elevation and front-back cues are accurate.

## US-5.2.3.2 Load Swappable HRTF Profiles
**As an** audio designer (P-14), **I want** HRTF datasets loadable as swappable SOFA profiles,
**so that** different head sizes are accommodated.

## US-5.2.3.3 Hear Sounds with Height Cues on Headphones
**As a** player (P-23), **I want** sounds above and below me distinguishable on headphones,
**so that** elevation perception works.

## US-5.2.3.4 Implement Frequency-Domain Convolution
**As an** engine developer (P-26), **I want** a fast frequency-domain convolution path for
per-voice HRTF, **so that** binaural rendering fits within voice budget.

## US-5.2.3.5 Delegate to OS Spatial Audio
**As an** engine developer (P-26), **I want** HRTF processing delegated to Windows Spatial
Audio or Apple Spatial Audio where available, **so that** platform integration is used.

## US-5.2.3.6 Verify Binaural Accuracy
**As an** engine tester (P-27), **I want to** verify binaural rendering produces correct
left-right and height separation, **so that** HRTF is accurate.

## US-5.2.3.7 Enable HRTF in Audio Settings
**As a** designer (P-5), **I want** HRTF binaural mode selectable in audio settings, **so
that** players with headphones can enable it.

## US-5.2.3.8 Choose HRTF Profile
**As a** player (P-23), **I want to** choose an HRTF profile matching my head size, **so
that** binaural audio is personalized.

## US-5.2.3.9 Test HRTF Per-Voice CPU Cost
**As an** engine tester (P-27), **I want to** profile per-voice HRTF CPU cost, **so that**
binaural rendering stays within budget.

## US-5.2.3.10 Distinguish Front from Behind
**As a** player (P-23), **I want** front and behind sounds distinguishable on headphones,
**so that** I can orient by ear.

## US-5.2.3.11 Support Multiple SOFA Files
**As an** audio designer (P-14), **I want** multiple SOFA files loadable, **so that** HRTF
variety is available for personalization.

## US-5.2.3.12 Verify Platform Delegation Fallback
**As an** engine tester (P-27), **I want to** verify engine-native HRTF works when platform
spatial audio is unavailable, **so that** fallback is seamless.

## US-5.2.4.1 Encode Sources into Ambisonics
**As an** engine developer (P-26), **I want to** encode sound sources into first- or
third-order Ambisonics, **so that** mixing is speaker-agnostic.

## US-5.2.4.2 Decode Ambisonics to Output Format
**As an** engine developer (P-26), **I want** Ambisonics decoded to stereo, 5.1, 7.1, or
binaural output, **so that** any speaker configuration is supported.

## US-5.2.4.3 Hear Immersive Surround Sound
**As a** player (P-23), **I want** immersive surround sound on my speaker setup, **so that**
audio fills the room.

## US-5.2.4.4 Rotate Ambisonics with Listener
**As an** engine developer (P-26), **I want** the Ambisonics field to rotate with listener
orientation, **so that** soundfield tracking is seamless.

## US-5.2.4.5 Verify Order Scaling Per Platform
**As an** engine tester (P-27), **I want to** confirm mobile uses first-order only and desktop
up to third-order, **so that** CPU budget is respected.

## US-5.2.4.6 Mix Reverb Returns in Ambisonics
**As an** audio designer (P-14), **I want** reverb returns mixed in Ambisonics, **so that**
spatial reverb preserves directionality.

## US-5.2.4.7 Select Output Format in Settings
**As a** designer (P-5), **I want** output format selectable in settings, **so that**
Ambisonics decodes to the player's setup.

## US-5.2.4.8 Test All Output Formats
**As an** engine tester (P-27), **I want to** test Ambisonics decoding to stereo, 5.1, 7.1,
and binaural, **so that** all output paths work correctly.

## US-5.2.4.9 Hear Smooth Panning Across Channels
**As a** player (P-23), **I want** sounds to pan smoothly across speaker channels, **so that**
source movement is artifact-free.

## US-5.2.4.10 Encode Ambience Beds in Ambisonics
**As an** audio designer (P-14), **I want** ambience beds encoded in Ambisonics, **so that**
immersive environments use full surround.

## US-5.2.4.11 Support Ambisonic Rotation for VR
**As an** engine developer (P-26), **I want** Ambisonics rotation to support VR head tracking,
**so that** VR spatial audio is immersive.

## US-5.2.4.12 Profile CPU Cost Per Ambisonics Order
**As an** engine tester (P-27), **I want to** profile CPU cost per Ambisonics order, **so
that** platform tier limits are validated.

## US-5.2.5.1 Attenuate Occluded Sounds
**As an** engine developer (P-26), **I want** sounds with occluded direct paths attenuated and
low-pass filtered, **so that** walls muffle sound.

## US-5.2.5.2 Query Shared BVH for Occlusion
**As an** engine developer (P-26), **I want** occlusion to ray-cast against the shared BVH,
**so that** physics, rendering, and audio share the same structure.

## US-5.2.5.3 Apply Material Transmission Loss
**As an** audio designer (P-14), **I want** material-dependent transmission loss (wood vs
stone), **so that** different materials muffle differently.

## US-5.2.5.4 Hear Muffled Sounds Through Walls
**As a** player (P-23), **I want** sounds behind walls to be muffled, **so that** occlusion
enhances spatial awareness.

## US-5.2.5.5 Verify Occlusion Ray Count Per Platform
**As an** engine tester (P-27), **I want to** confirm mobile uses 1 ray, Switch 2, desktop 4,
**so that** budget scales per tier.

## US-5.2.5.6 Configure Transmission Coefficients
**As an** audio designer (P-14), **I want to** configure per-material transmission loss
coefficients, **so that** surface types affect sound differently.

## US-5.2.5.7 Set Occlusion Quality in Settings
**As a** designer (P-5), **I want** occlusion quality adjustable in settings, **so that**
players can tune performance vs quality.

## US-5.2.5.8 Test Occlusion with Various Materials
**As an** engine tester (P-27), **I want to** test occlusion through glass, wood, stone, and
metal, **so that** material coefficients produce distinct muffling.

## US-5.2.5.9 Hear Door Block Sound
**As a** player (P-23), **I want** closing a door to muffle sounds from the other side,
**so that** occlusion responds to world state changes.

## US-5.2.5.10 Use Multi-Ray Weighted Transmission on Desktop
**As an** engine developer (P-26), **I want** desktop to use multi-ray material-weighted
transmission, **so that** occlusion accuracy is maximized.

## US-5.2.5.11 Configure Occlusion Update Rate
**As an** audio designer (P-14), **I want to** configure occlusion ray update rate, **so that**
CPU cost is controlled.

## US-5.2.5.12 Verify Binary Occlusion on Mobile
**As an** engine tester (P-27), **I want to** verify mobile uses binary occlusion, **so that**
mobile budget is met.

## US-5.2.6.1 Model Sound Through Portals
**As an** engine developer (P-26), **I want** indirect sound paths through portals (doorways,
windows, tunnels), **so that** sound travels around corners.

## US-5.2.6.2 Implement Hybrid Ray-Portal Solver
**As an** engine developer (P-26), **I want** a hybrid ray-and-portal propagation solver,
**so that** both portal and reflective paths are computed.

## US-5.2.6.3 Hear Combat Echo Through Corridors
**As a** player (P-23), **I want** combat sounds to echo through dungeon corridors, **so
that** sound propagation enhances immersion.

## US-5.2.6.4 Run Solver Asynchronously
**As an** engine developer (P-26), **I want** the propagation solver running asynchronously at
reduced rate, **so that** CPU impact is amortized.

## US-5.2.6.5 Feed Results into Per-Voice Taps
**As an** engine developer (P-26), **I want** propagation results fed as delay, gain, and
filter parameters into per-voice taps, **so that** reflections are audible.

## US-5.2.6.6 Verify Portal-Only on Mobile
**As an** engine tester (P-27), **I want to** confirm mobile uses portal-only propagation,
**so that** budget is controlled.

## US-5.2.6.7 Configure Portal Graph in Editor
**As an** audio designer (P-14), **I want to** configure the portal graph in the visual
editor, **so that** propagation paths are visual.

## US-5.2.6.8 Test Multi-Bounce on Desktop
**As an** engine tester (P-27), **I want to** test multi-bounce ray + portal propagation on
desktop, **so that** complex paths are validated.

## US-5.2.6.9 Hear Sound Through Open Window
**As a** player (P-23), **I want** outdoor sounds to come through open windows, **so that**
indirect paths feel natural.

## US-5.2.6.10 Update Every 1-2 Frames on Desktop
**As an** engine developer (P-26), **I want** desktop propagation updated every 1-2 frames,
**so that** response is fast.

## US-5.2.6.11 Set Complexity Per Platform
**As a** designer (P-5), **I want** propagation complexity configurable per platform, **so
that** quality scales with capability.

## US-5.2.6.12 Verify Update Rate Per Platform
**As an** engine tester (P-27), **I want to** verify mobile updates every 4-8 frames and
desktop every 1-2, **so that** rates match spec.

## US-5.2.7.1 Define Reverb Zones
**As an** audio designer (P-14), **I want to** define reverb zones as axis-aligned or convex
volumes, **so that** enclosed spaces have distinct reverb.

## US-5.2.7.2 Blend Zones at Boundaries
**As an** engine developer (P-26), **I want** reverb zones to blend smoothly at boundaries,
**so that** transitions are inaudible.

## US-5.2.7.3 Hear Cathedral Reverb
**As a** player (P-23), **I want** large rooms to have cathedral-like reverb, **so that**
acoustic environments feel distinct.

## US-5.2.7.4 Combine Nested Zones via Priority
**As an** engine developer (P-26), **I want** nested zones combined via priority ordering,
**so that** specific zones override general ones.

## US-5.2.7.5 Derive Early Reflections from Geometry
**As an** audio designer (P-14), **I want** early reflections from room geometry, **so that**
spatial cues are reinforced.

## US-5.2.7.6 Verify Zone Count Per Platform
**As an** engine tester (P-27), **I want to** confirm mobile supports 1-2 concurrent zones,
**so that** budget is respected.

## US-5.2.7.7 Configure Zones in Visual Editor
**As a** designer (P-5), **I want to** configure reverb zones in the visual editor, **so
that** acoustic design is visual.

## US-5.2.7.8 Test Zone Blending
**As an** engine tester (P-27), **I want to** test reverb zone blending at boundaries, **so
that** transitions are smooth.

## US-5.2.7.9 Hear Cave Echo
**As a** player (P-23), **I want** echo to increase when entering a cave, **so that** acoustic
changes are noticeable.

## US-5.2.7.10 Disable Early Reflections on Mobile
**As an** engine tester (P-27), **I want to** confirm early reflections disabled on mobile,
**so that** algorithmic reverb only is used.

## US-5.2.7.11 Author Reflection Patterns
**As an** audio designer (P-14), **I want to** author early reflection patterns per zone, **so
that** specific rooms have tailored acoustics.

## US-5.2.7.12 Support 6+ Zones on Desktop
**As an** engine developer (P-26), **I want** desktop to support 6+ concurrent reverb zones,
**so that** complex environments work.
