# User Stories — 5.2 Spatial Audio

## F-5.2.1

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.2.1.1  | audio designer (P-14)   | F-5.2.1  | R-5.2.1      |
| US-5.2.1.2  | engine developer (P-26) | F-5.2.1  | R-5.2.1      |
| US-5.2.1.3  | engine developer (P-26) | F-5.2.1  | R-5.2.1      |
| US-5.2.1.4  | player (P-23)           | F-5.2.1  | R-5.2.1      |
| US-5.2.1.5  | engine tester (P-27)    | F-5.2.1  | R-5.2.1      |
| US-5.2.1.6  | audio designer (P-14)   | F-5.2.1  | R-5.2.1      |
| US-5.2.1.7  | player (P-23)           | F-5.2.1  | R-5.2.1      |
| US-5.2.1.8  | engine tester (P-27)    | F-5.2.1  | R-5.2.1      |
| US-5.2.1.9  | designer (P-5)          | F-5.2.1  | R-5.2.1      |
| US-5.2.1.10 | player (P-23)           | F-5.2.1  | R-5.2.1      |
| US-5.2.1.11 | engine developer (P-26) | F-5.2.1  | R-5.2.1      |
| US-5.2.1.12 | engine tester (P-27)    | F-5.2.1  | R-5.2.1      |

1. **US-5.2.1.1** — I want sounds positioned in world space with per-listener panning and distance
   gain
   - **Acceptance:** 3D audio placement is automatic
2. **US-5.2.1.2** — I want Doppler pitch shift computed from relative source and listener velocities
   - **Acceptance:** moving objects sound realistic
3. **US-5.2.1.3** — I want source and listener transforms interpolated between game ticks
   - **Acceptance:** audio motion is smooth at audio callback rate
4. **US-5.2.1.4** — I want passing vehicles to pitch shift as they approach and recede
   - **Acceptance:** speed is audible
5. **US-5.2.1.5** — I want to verify no audio artifacts during fast source motion
   - **Acceptance:** interpolation is working correctly
6. **US-5.2.1.6** — I want to configure Doppler scale factor per source
   - **Acceptance:** Doppler intensity is tunable
7. **US-5.2.1.7** — I want sound direction to update as I rotate the camera
   - **Acceptance:** spatial orientation is correct
8. **US-5.2.1.8** — I want to test 3D positioning with the maximum voice count
   - **Acceptance:** spatial audio scales within budget
9. **US-5.2.1.9** — I want to set up 3D sound sources in the visual editor
   - **Acceptance:** spatial audio configuration is visual
10. **US-5.2.1.10** — I want to hear enemies approaching from behind
    - **Acceptance:** spatial audio provides tactical information
11. **US-5.2.1.11** — I want per-voice Doppler calculated efficiently
    - **Acceptance:** it remains lightweight on all platforms
12. **US-5.2.1.12** — I want to verify panning at extreme positions (directly behind, directly
    above)
    - **Acceptance:** edge case directions work correctly

## F-5.2.2

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.2.2.1  | audio designer (P-14)   | F-5.2.2  | R-5.2.2      |
| US-5.2.2.2  | engine developer (P-26) | F-5.2.2  | R-5.2.2      |
| US-5.2.2.3  | audio designer (P-14)   | F-5.2.2  | R-5.2.2      |
| US-5.2.2.4  | player (P-23)           | F-5.2.2  | R-5.2.2      |
| US-5.2.2.5  | audio designer (P-14)   | F-5.2.2  | R-5.2.2      |
| US-5.2.2.6  | engine tester (P-27)    | F-5.2.2  | R-5.2.2      |
| US-5.2.2.7  | designer (P-5)          | F-5.2.2  | R-5.2.2      |
| US-5.2.2.8  | audio designer (P-14)   | F-5.2.2  | R-5.2.2      |
| US-5.2.2.9  | engine tester (P-27)    | F-5.2.2  | R-5.2.2      |
| US-5.2.2.10 | player (P-23)           | F-5.2.2  | R-5.2.2      |
| US-5.2.2.11 | designer (P-5)          | F-5.2.2  | R-5.2.2      |
| US-5.2.2.12 | engine tester (P-27)    | F-5.2.2  | R-5.2.2      |

1. **US-5.2.2.1** — I want to apply configurable distance attenuation models per source
   - **Acceptance:** each sound has appropriate rolloff
2. **US-5.2.2.2** — I want inverse, inverse-squared, linear, logarithmic, and custom curve
   attenuation models
   - **Acceptance:** designers have flexibility
3. **US-5.2.2.3** — I want per-source minimum and maximum distance parameters
   - **Acceptance:** rolloff range is controlled per emitter
4. **US-5.2.2.4** — I want sounds to fade naturally with distance
   - **Acceptance:** near sounds are loud and far sounds are quiet
5. **US-5.2.2.5** — I want custom user-defined attenuation curves
   - **Acceptance:** unique rolloff profiles are possible
6. **US-5.2.2.6** — I want to verify attenuation gain at min, max, and mid distances
   - **Acceptance:** each model produces correct rolloff
7. **US-5.2.2.7** — I want to configure attenuation model and distances in the visual editor
   - **Acceptance:** rolloff is visual
8. **US-5.2.2.8** — I want tunable attenuation to balance distant siege weaponry against nearby
   footsteps
   - **Acceptance:** the mix is clear
9. **US-5.2.2.9** — I want to test all attenuation models
   - **Acceptance:** each behaves as specified
10. **US-5.2.2.10** — I want nearby footsteps clearly audible over distant battle sounds
    - **Acceptance:** proximity is communicated
11. **US-5.2.2.11** — I want logarithmic attenuation as a default for realism
    - **Acceptance:** distance perception matches expectations
12. **US-5.2.2.12** — I want to verify attenuation produces identical results across platforms
    - **Acceptance:** audio mix is consistent

## F-5.2.3

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.2.3.1  | engine developer (P-26) | F-5.2.3  | R-5.2.3      |
| US-5.2.3.2  | audio designer (P-14)   | F-5.2.3  | R-5.2.3      |
| US-5.2.3.3  | player (P-23)           | F-5.2.3  | R-5.2.3      |
| US-5.2.3.4  | engine developer (P-26) | F-5.2.3  | R-5.2.3      |
| US-5.2.3.5  | engine developer (P-26) | F-5.2.3  | R-5.2.3      |
| US-5.2.3.6  | engine tester (P-27)    | F-5.2.3  | R-5.2.3      |
| US-5.2.3.7  | designer (P-5)          | F-5.2.3  | R-5.2.3      |
| US-5.2.3.8  | player (P-23)           | F-5.2.3  | R-5.2.3      |
| US-5.2.3.9  | engine tester (P-27)    | F-5.2.3  | R-5.2.3      |
| US-5.2.3.10 | player (P-23)           | F-5.2.3  | R-5.2.3      |
| US-5.2.3.11 | audio designer (P-14)   | F-5.2.3  | R-5.2.3      |
| US-5.2.3.12 | engine tester (P-27)    | F-5.2.3  | R-5.2.3      |

1. **US-5.2.3.1** — I want to render spatialized audio through HRTF for headphone output
   - **Acceptance:** elevation and front-back cues are accurate
2. **US-5.2.3.2** — I want HRTF datasets loadable as swappable SOFA profiles
   - **Acceptance:** different head sizes are accommodated
3. **US-5.2.3.3** — I want sounds above and below me distinguishable on headphones
   - **Acceptance:** elevation perception works
4. **US-5.2.3.4** — I want a fast frequency-domain convolution path for per-voice HRTF
   - **Acceptance:** binaural rendering fits within voice budget
5. **US-5.2.3.5** — I want HRTF processing delegated to Windows Spatial Audio or Apple Spatial Audio
   where available
   - **Acceptance:** platform integration is used
6. **US-5.2.3.6** — I want to verify binaural rendering produces correct left-right and height
   separation
   - **Acceptance:** HRTF is accurate
7. **US-5.2.3.7** — I want HRTF binaural mode selectable in audio settings
   - **Acceptance:** players with headphones can enable it
8. **US-5.2.3.8** — I want to choose an HRTF profile matching my head size
   - **Acceptance:** binaural audio is personalized
9. **US-5.2.3.9** — I want to profile per-voice HRTF CPU cost
   - **Acceptance:** binaural rendering stays within budget
10. **US-5.2.3.10** — I want front and behind sounds distinguishable on headphones
    - **Acceptance:** I can orient by ear
11. **US-5.2.3.11** — I want multiple SOFA files loadable
    - **Acceptance:** HRTF variety is available for personalization
12. **US-5.2.3.12** — I want to verify engine-native HRTF works when platform spatial audio is
    unavailable
    - **Acceptance:** fallback is seamless

## F-5.2.4

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.2.4.1  | engine developer (P-26) | F-5.2.4  | R-5.2.4      |
| US-5.2.4.2  | engine developer (P-26) | F-5.2.4  | R-5.2.4      |
| US-5.2.4.3  | player (P-23)           | F-5.2.4  | R-5.2.4      |
| US-5.2.4.4  | engine developer (P-26) | F-5.2.4  | R-5.2.4      |
| US-5.2.4.5  | engine tester (P-27)    | F-5.2.4  | R-5.2.4      |
| US-5.2.4.6  | audio designer (P-14)   | F-5.2.4  | R-5.2.4      |
| US-5.2.4.7  | designer (P-5)          | F-5.2.4  | R-5.2.4      |
| US-5.2.4.8  | engine tester (P-27)    | F-5.2.4  | R-5.2.4      |
| US-5.2.4.9  | player (P-23)           | F-5.2.4  | R-5.2.4      |
| US-5.2.4.10 | audio designer (P-14)   | F-5.2.4  | R-5.2.4      |
| US-5.2.4.11 | engine developer (P-26) | F-5.2.4  | R-5.2.4      |
| US-5.2.4.12 | engine tester (P-27)    | F-5.2.4  | R-5.2.4      |

1. **US-5.2.4.1** — I want to encode sound sources into first- or third-order Ambisonics
   - **Acceptance:** mixing is speaker-agnostic
2. **US-5.2.4.2** — I want Ambisonics decoded to stereo, 5.1, 7.1, or binaural output
   - **Acceptance:** any speaker configuration is supported
3. **US-5.2.4.3** — I want immersive surround sound on my speaker setup
   - **Acceptance:** audio fills the room
4. **US-5.2.4.4** — I want the Ambisonics field to rotate with listener orientation
   - **Acceptance:** soundfield tracking is seamless
5. **US-5.2.4.5** — I want to confirm mobile uses first-order only and desktop up to third-order
   - **Acceptance:** CPU budget is respected
6. **US-5.2.4.6** — I want reverb returns mixed in Ambisonics
   - **Acceptance:** spatial reverb preserves directionality
7. **US-5.2.4.7** — I want output format selectable in settings
   - **Acceptance:** Ambisonics decodes to the player's setup
8. **US-5.2.4.8** — I want to test Ambisonics decoding to stereo, 5.1, 7.1, and binaural
   - **Acceptance:** all output paths work correctly
9. **US-5.2.4.9** — I want sounds to pan smoothly across speaker channels
   - **Acceptance:** source movement is artifact-free
10. **US-5.2.4.10** — I want ambience beds encoded in Ambisonics
    - **Acceptance:** immersive environments use full surround
11. **US-5.2.4.11** — I want Ambisonics rotation to support VR head tracking
    - **Acceptance:** VR spatial audio is immersive
12. **US-5.2.4.12** — I want to profile CPU cost per Ambisonics order
    - **Acceptance:** platform tier limits are validated

## F-5.2.5

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.2.5.1  | engine developer (P-26) | F-5.2.5  | R-5.2.5      |
| US-5.2.5.2  | engine developer (P-26) | F-5.2.5  | R-5.2.5      |
| US-5.2.5.3  | audio designer (P-14)   | F-5.2.5  | R-5.2.5      |
| US-5.2.5.4  | player (P-23)           | F-5.2.5  | R-5.2.5      |
| US-5.2.5.5  | engine tester (P-27)    | F-5.2.5  | R-5.2.5      |
| US-5.2.5.6  | audio designer (P-14)   | F-5.2.5  | R-5.2.5      |
| US-5.2.5.7  | designer (P-5)          | F-5.2.5  | R-5.2.5      |
| US-5.2.5.8  | engine tester (P-27)    | F-5.2.5  | R-5.2.5      |
| US-5.2.5.9  | player (P-23)           | F-5.2.5  | R-5.2.5      |
| US-5.2.5.10 | engine developer (P-26) | F-5.2.5  | R-5.2.5      |
| US-5.2.5.11 | audio designer (P-14)   | F-5.2.5  | R-5.2.5      |
| US-5.2.5.12 | engine tester (P-27)    | F-5.2.5  | R-5.2.5      |

1. **US-5.2.5.1** — I want sounds with occluded direct paths attenuated and low-pass filtered
   - **Acceptance:** walls muffle sound
2. **US-5.2.5.2** — I want occlusion to ray-cast against the shared BVH
   - **Acceptance:** physics, rendering, and audio share the same structure
3. **US-5.2.5.3** — I want material-dependent transmission loss (wood vs stone)
   - **Acceptance:** different materials muffle differently
4. **US-5.2.5.4** — I want sounds behind walls to be muffled
   - **Acceptance:** occlusion enhances spatial awareness
5. **US-5.2.5.5** — I want to confirm mobile uses 1 ray, Switch 2, desktop 4
   - **Acceptance:** budget scales per tier
6. **US-5.2.5.6** — I want to configure per-material transmission loss coefficients
   - **Acceptance:** surface types affect sound differently
7. **US-5.2.5.7** — I want occlusion quality adjustable in settings
   - **Acceptance:** players can tune performance vs quality
8. **US-5.2.5.8** — I want to test occlusion through glass, wood, stone, and metal
   - **Acceptance:** material coefficients produce distinct muffling
9. **US-5.2.5.9** — I want closing a door to muffle sounds from the other side
   - **Acceptance:** occlusion responds to world state changes
10. **US-5.2.5.10** — I want desktop to use multi-ray material-weighted transmission
    - **Acceptance:** occlusion accuracy is maximized
11. **US-5.2.5.11** — I want to configure occlusion ray update rate
    - **Acceptance:** CPU cost is controlled
12. **US-5.2.5.12** — I want to verify mobile uses binary occlusion
    - **Acceptance:** mobile budget is met

## F-5.2.6

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.2.6.1  | engine developer (P-26) | F-5.2.6  | R-5.2.6      |
| US-5.2.6.2  | engine developer (P-26) | F-5.2.6  | R-5.2.6      |
| US-5.2.6.3  | player (P-23)           | F-5.2.6  | R-5.2.6      |
| US-5.2.6.4  | engine developer (P-26) | F-5.2.6  | R-5.2.6      |
| US-5.2.6.5  | engine developer (P-26) | F-5.2.6  | R-5.2.6      |
| US-5.2.6.6  | engine tester (P-27)    | F-5.2.6  | R-5.2.6      |
| US-5.2.6.7  | audio designer (P-14)   | F-5.2.6  | R-5.2.6      |
| US-5.2.6.8  | engine tester (P-27)    | F-5.2.6  | R-5.2.6      |
| US-5.2.6.9  | player (P-23)           | F-5.2.6  | R-5.2.6      |
| US-5.2.6.10 | engine developer (P-26) | F-5.2.6  | R-5.2.6      |
| US-5.2.6.11 | designer (P-5)          | F-5.2.6  | R-5.2.6      |
| US-5.2.6.12 | engine tester (P-27)    | F-5.2.6  | R-5.2.6      |

1. **US-5.2.6.1** — I want indirect sound paths through portals (doorways, windows, tunnels)
   - **Acceptance:** sound travels around corners
2. **US-5.2.6.2** — I want a hybrid ray-and-portal propagation solver
   - **Acceptance:** both portal and reflective paths are computed
3. **US-5.2.6.3** — I want combat sounds to echo through dungeon corridors
   - **Acceptance:** sound propagation enhances immersion
4. **US-5.2.6.4** — I want the propagation solver running asynchronously at reduced rate
   - **Acceptance:** CPU impact is amortized
5. **US-5.2.6.5** — I want propagation results fed as delay, gain, and filter parameters into
   per-voice taps
   - **Acceptance:** reflections are audible
6. **US-5.2.6.6** — I want to confirm mobile uses portal-only propagation
   - **Acceptance:** budget is controlled
7. **US-5.2.6.7** — I want to configure the portal graph in the visual editor
   - **Acceptance:** propagation paths are visual
8. **US-5.2.6.8** — I want to test multi-bounce ray + portal propagation on desktop
   - **Acceptance:** complex paths are validated
9. **US-5.2.6.9** — I want outdoor sounds to come through open windows
   - **Acceptance:** indirect paths feel natural
10. **US-5.2.6.10** — I want desktop propagation updated every 1-2 frames
    - **Acceptance:** response is fast
11. **US-5.2.6.11** — I want propagation complexity configurable per platform
    - **Acceptance:** quality scales with capability
12. **US-5.2.6.12** — I want to verify mobile updates every 4-8 frames and desktop every 1-2
    - **Acceptance:** rates match spec

## F-5.2.7

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.2.7.1  | audio designer (P-14)   | F-5.2.7  | R-5.2.7      |
| US-5.2.7.2  | engine developer (P-26) | F-5.2.7  | R-5.2.7      |
| US-5.2.7.3  | player (P-23)           | F-5.2.7  | R-5.2.7      |
| US-5.2.7.4  | engine developer (P-26) | F-5.2.7  | R-5.2.7      |
| US-5.2.7.5  | audio designer (P-14)   | F-5.2.7  | R-5.2.7      |
| US-5.2.7.6  | engine tester (P-27)    | F-5.2.7  | R-5.2.7      |
| US-5.2.7.7  | designer (P-5)          | F-5.2.7  | R-5.2.7      |
| US-5.2.7.8  | engine tester (P-27)    | F-5.2.7  | R-5.2.7      |
| US-5.2.7.9  | player (P-23)           | F-5.2.7  | R-5.2.7      |
| US-5.2.7.10 | engine tester (P-27)    | F-5.2.7  | R-5.2.7      |
| US-5.2.7.11 | audio designer (P-14)   | F-5.2.7  | R-5.2.7      |
| US-5.2.7.12 | engine developer (P-26) | F-5.2.7  | R-5.2.7      |

1. **US-5.2.7.1** — I want to define reverb zones as axis-aligned or convex volumes
   - **Acceptance:** enclosed spaces have distinct reverb
2. **US-5.2.7.2** — I want reverb zones to blend smoothly at boundaries
   - **Acceptance:** transitions are inaudible
3. **US-5.2.7.3** — I want large rooms to have cathedral-like reverb
   - **Acceptance:** acoustic environments feel distinct
4. **US-5.2.7.4** — I want nested zones combined via priority ordering
   - **Acceptance:** specific zones override general ones
5. **US-5.2.7.5** — I want early reflections from room geometry
   - **Acceptance:** spatial cues are reinforced
6. **US-5.2.7.6** — I want to confirm mobile supports 1-2 concurrent zones
   - **Acceptance:** budget is respected
7. **US-5.2.7.7** — I want to configure reverb zones in the visual editor
   - **Acceptance:** acoustic design is visual
8. **US-5.2.7.8** — I want to test reverb zone blending at boundaries
   - **Acceptance:** transitions are smooth
9. **US-5.2.7.9** — I want echo to increase when entering a cave
   - **Acceptance:** acoustic changes are noticeable
10. **US-5.2.7.10** — I want to confirm early reflections disabled on mobile
    - **Acceptance:** algorithmic reverb only is used
11. **US-5.2.7.11** — I want to author early reflection patterns per zone
    - **Acceptance:** specific rooms have tailored acoustics
12. **US-5.2.7.12** — I want desktop to support 6+ concurrent reverb zones
    - **Acceptance:** complex environments work
