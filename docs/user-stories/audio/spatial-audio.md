# User Stories — 5.2 Spatial Audio

## F-5.2.1

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.2.1.1 | audio designer (P-14) | I want sounds positioned in world space with per-listener panning and distance gain | 3D audio placement is automatic | F-5.2.1 | R-5.2.1 |
| US-5.2.1.2 | engine developer (P-26) | I want Doppler pitch shift computed from relative source and listener velocities | moving objects sound realistic | F-5.2.1 | R-5.2.1 |
| US-5.2.1.3 | engine developer (P-26) | I want source and listener transforms interpolated between game ticks | audio motion is smooth at audio callback rate | F-5.2.1 | R-5.2.1 |
| US-5.2.1.4 | player (P-23) | I want passing vehicles to pitch shift as they approach and recede | speed is audible | F-5.2.1 | R-5.2.1 |
| US-5.2.1.5 | engine tester (P-27) | I want to verify no audio artifacts during fast source motion | interpolation is working correctly | F-5.2.1 | R-5.2.1 |
| US-5.2.1.6 | audio designer (P-14) | I want to configure Doppler scale factor per source | Doppler intensity is tunable | F-5.2.1 | R-5.2.1 |
| US-5.2.1.7 | player (P-23) | I want sound direction to update as I rotate the camera | spatial orientation is correct | F-5.2.1 | R-5.2.1 |
| US-5.2.1.8 | engine tester (P-27) | I want to test 3D positioning with the maximum voice count | spatial audio scales within budget | F-5.2.1 | R-5.2.1 |
| US-5.2.1.9 | designer (P-5) | I want to set up 3D sound sources in the visual editor | spatial audio configuration is visual | F-5.2.1 | R-5.2.1 |
| US-5.2.1.10 | player (P-23) | I want to hear enemies approaching from behind | spatial audio provides tactical information | F-5.2.1 | R-5.2.1 |
| US-5.2.1.11 | engine developer (P-26) | I want per-voice Doppler calculated efficiently | it remains lightweight on all platforms | F-5.2.1 | R-5.2.1 |
| US-5.2.1.12 | engine tester (P-27) | I want to verify panning at extreme positions (directly behind, directly above) | edge case directions work correctly | F-5.2.1 | R-5.2.1 |

## F-5.2.2

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.2.2.1 | audio designer (P-14) | I want to apply configurable distance attenuation models per source | each sound has appropriate rolloff | F-5.2.2 | R-5.2.2 |
| US-5.2.2.2 | engine developer (P-26) | I want inverse, inverse-squared, linear, logarithmic, and custom curve attenuation models | designers have flexibility | F-5.2.2 | R-5.2.2 |
| US-5.2.2.3 | audio designer (P-14) | I want per-source minimum and maximum distance parameters | rolloff range is controlled per emitter | F-5.2.2 | R-5.2.2 |
| US-5.2.2.4 | player (P-23) | I want sounds to fade naturally with distance | near sounds are loud and far sounds are quiet | F-5.2.2 | R-5.2.2 |
| US-5.2.2.5 | audio designer (P-14) | I want custom user-defined attenuation curves | unique rolloff profiles are possible | F-5.2.2 | R-5.2.2 |
| US-5.2.2.6 | engine tester (P-27) | I want to verify attenuation gain at min, max, and mid distances | each model produces correct rolloff | F-5.2.2 | R-5.2.2 |
| US-5.2.2.7 | designer (P-5) | I want to configure attenuation model and distances in the visual editor | rolloff is visual | F-5.2.2 | R-5.2.2 |
| US-5.2.2.8 | audio designer (P-14) | I want tunable attenuation to balance distant siege weaponry against nearby footsteps | the mix is clear | F-5.2.2 | R-5.2.2 |
| US-5.2.2.9 | engine tester (P-27) | I want to test all attenuation models | each behaves as specified | F-5.2.2 | R-5.2.2 |
| US-5.2.2.10 | player (P-23) | I want nearby footsteps clearly audible over distant battle sounds | proximity is communicated | F-5.2.2 | R-5.2.2 |
| US-5.2.2.11 | designer (P-5) | I want logarithmic attenuation as a default for realism | distance perception matches expectations | F-5.2.2 | R-5.2.2 |
| US-5.2.2.12 | engine tester (P-27) | I want to verify attenuation produces identical results across platforms | audio mix is consistent | F-5.2.2 | R-5.2.2 |

## F-5.2.3

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.2.3.1 | engine developer (P-26) | I want to render spatialized audio through HRTF for headphone output | elevation and front-back cues are accurate | F-5.2.3 | R-5.2.3 |
| US-5.2.3.2 | audio designer (P-14) | I want HRTF datasets loadable as swappable SOFA profiles | different head sizes are accommodated | F-5.2.3 | R-5.2.3 |
| US-5.2.3.3 | player (P-23) | I want sounds above and below me distinguishable on headphones | elevation perception works | F-5.2.3 | R-5.2.3 |
| US-5.2.3.4 | engine developer (P-26) | I want a fast frequency-domain convolution path for per-voice HRTF | binaural rendering fits within voice budget | F-5.2.3 | R-5.2.3 |
| US-5.2.3.5 | engine developer (P-26) | I want HRTF processing delegated to Windows Spatial Audio or Apple Spatial Audio where available | platform integration is used | F-5.2.3 | R-5.2.3 |
| US-5.2.3.6 | engine tester (P-27) | I want to verify binaural rendering produces correct left-right and height separation | HRTF is accurate | F-5.2.3 | R-5.2.3 |
| US-5.2.3.7 | designer (P-5) | I want HRTF binaural mode selectable in audio settings | players with headphones can enable it | F-5.2.3 | R-5.2.3 |
| US-5.2.3.8 | player (P-23) | I want to choose an HRTF profile matching my head size | binaural audio is personalized | F-5.2.3 | R-5.2.3 |
| US-5.2.3.9 | engine tester (P-27) | I want to profile per-voice HRTF CPU cost | binaural rendering stays within budget | F-5.2.3 | R-5.2.3 |
| US-5.2.3.10 | player (P-23) | I want front and behind sounds distinguishable on headphones | I can orient by ear | F-5.2.3 | R-5.2.3 |
| US-5.2.3.11 | audio designer (P-14) | I want multiple SOFA files loadable | HRTF variety is available for personalization | F-5.2.3 | R-5.2.3 |
| US-5.2.3.12 | engine tester (P-27) | I want to verify engine-native HRTF works when platform spatial audio is unavailable | fallback is seamless | F-5.2.3 | R-5.2.3 |

## F-5.2.4

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.2.4.1 | engine developer (P-26) | I want to encode sound sources into first- or third-order Ambisonics | mixing is speaker-agnostic | F-5.2.4 | R-5.2.4 |
| US-5.2.4.2 | engine developer (P-26) | I want Ambisonics decoded to stereo, 5.1, 7.1, or binaural output | any speaker configuration is supported | F-5.2.4 | R-5.2.4 |
| US-5.2.4.3 | player (P-23) | I want immersive surround sound on my speaker setup | audio fills the room | F-5.2.4 | R-5.2.4 |
| US-5.2.4.4 | engine developer (P-26) | I want the Ambisonics field to rotate with listener orientation | soundfield tracking is seamless | F-5.2.4 | R-5.2.4 |
| US-5.2.4.5 | engine tester (P-27) | I want to confirm mobile uses first-order only and desktop up to third-order | CPU budget is respected | F-5.2.4 | R-5.2.4 |
| US-5.2.4.6 | audio designer (P-14) | I want reverb returns mixed in Ambisonics | spatial reverb preserves directionality | F-5.2.4 | R-5.2.4 |
| US-5.2.4.7 | designer (P-5) | I want output format selectable in settings | Ambisonics decodes to the player's setup | F-5.2.4 | R-5.2.4 |
| US-5.2.4.8 | engine tester (P-27) | I want to test Ambisonics decoding to stereo, 5.1, 7.1, and binaural | all output paths work correctly | F-5.2.4 | R-5.2.4 |
| US-5.2.4.9 | player (P-23) | I want sounds to pan smoothly across speaker channels | source movement is artifact-free | F-5.2.4 | R-5.2.4 |
| US-5.2.4.10 | audio designer (P-14) | I want ambience beds encoded in Ambisonics | immersive environments use full surround | F-5.2.4 | R-5.2.4 |
| US-5.2.4.11 | engine developer (P-26) | I want Ambisonics rotation to support VR head tracking | VR spatial audio is immersive | F-5.2.4 | R-5.2.4 |
| US-5.2.4.12 | engine tester (P-27) | I want to profile CPU cost per Ambisonics order | platform tier limits are validated | F-5.2.4 | R-5.2.4 |

## F-5.2.5

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.2.5.1 | engine developer (P-26) | I want sounds with occluded direct paths attenuated and low-pass filtered | walls muffle sound | F-5.2.5 | R-5.2.5 |
| US-5.2.5.2 | engine developer (P-26) | I want occlusion to ray-cast against the shared BVH | physics, rendering, and audio share the same structure | F-5.2.5 | R-5.2.5 |
| US-5.2.5.3 | audio designer (P-14) | I want material-dependent transmission loss (wood vs stone) | different materials muffle differently | F-5.2.5 | R-5.2.5 |
| US-5.2.5.4 | player (P-23) | I want sounds behind walls to be muffled | occlusion enhances spatial awareness | F-5.2.5 | R-5.2.5 |
| US-5.2.5.5 | engine tester (P-27) | I want to confirm mobile uses 1 ray, Switch 2, desktop 4 | budget scales per tier | F-5.2.5 | R-5.2.5 |
| US-5.2.5.6 | audio designer (P-14) | I want to configure per-material transmission loss coefficients | surface types affect sound differently | F-5.2.5 | R-5.2.5 |
| US-5.2.5.7 | designer (P-5) | I want occlusion quality adjustable in settings | players can tune performance vs quality | F-5.2.5 | R-5.2.5 |
| US-5.2.5.8 | engine tester (P-27) | I want to test occlusion through glass, wood, stone, and metal | material coefficients produce distinct muffling | F-5.2.5 | R-5.2.5 |
| US-5.2.5.9 | player (P-23) | I want closing a door to muffle sounds from the other side | occlusion responds to world state changes | F-5.2.5 | R-5.2.5 |
| US-5.2.5.10 | engine developer (P-26) | I want desktop to use multi-ray material-weighted transmission | occlusion accuracy is maximized | F-5.2.5 | R-5.2.5 |
| US-5.2.5.11 | audio designer (P-14) | I want to configure occlusion ray update rate | CPU cost is controlled | F-5.2.5 | R-5.2.5 |
| US-5.2.5.12 | engine tester (P-27) | I want to verify mobile uses binary occlusion | mobile budget is met | F-5.2.5 | R-5.2.5 |

## F-5.2.6

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.2.6.1 | engine developer (P-26) | I want indirect sound paths through portals (doorways, windows, tunnels) | sound travels around corners | F-5.2.6 | R-5.2.6 |
| US-5.2.6.2 | engine developer (P-26) | I want a hybrid ray-and-portal propagation solver | both portal and reflective paths are computed | F-5.2.6 | R-5.2.6 |
| US-5.2.6.3 | player (P-23) | I want combat sounds to echo through dungeon corridors | sound propagation enhances immersion | F-5.2.6 | R-5.2.6 |
| US-5.2.6.4 | engine developer (P-26) | I want the propagation solver running asynchronously at reduced rate | CPU impact is amortized | F-5.2.6 | R-5.2.6 |
| US-5.2.6.5 | engine developer (P-26) | I want propagation results fed as delay, gain, and filter parameters into per-voice taps | reflections are audible | F-5.2.6 | R-5.2.6 |
| US-5.2.6.6 | engine tester (P-27) | I want to confirm mobile uses portal-only propagation | budget is controlled | F-5.2.6 | R-5.2.6 |
| US-5.2.6.7 | audio designer (P-14) | I want to configure the portal graph in the visual editor | propagation paths are visual | F-5.2.6 | R-5.2.6 |
| US-5.2.6.8 | engine tester (P-27) | I want to test multi-bounce ray + portal propagation on desktop | complex paths are validated | F-5.2.6 | R-5.2.6 |
| US-5.2.6.9 | player (P-23) | I want outdoor sounds to come through open windows | indirect paths feel natural | F-5.2.6 | R-5.2.6 |
| US-5.2.6.10 | engine developer (P-26) | I want desktop propagation updated every 1-2 frames | response is fast | F-5.2.6 | R-5.2.6 |
| US-5.2.6.11 | designer (P-5) | I want propagation complexity configurable per platform | quality scales with capability | F-5.2.6 | R-5.2.6 |
| US-5.2.6.12 | engine tester (P-27) | I want to verify mobile updates every 4-8 frames and desktop every 1-2 | rates match spec | F-5.2.6 | R-5.2.6 |

## F-5.2.7

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.2.7.1 | audio designer (P-14) | I want to define reverb zones as axis-aligned or convex volumes | enclosed spaces have distinct reverb | F-5.2.7 | R-5.2.7 |
| US-5.2.7.2 | engine developer (P-26) | I want reverb zones to blend smoothly at boundaries | transitions are inaudible | F-5.2.7 | R-5.2.7 |
| US-5.2.7.3 | player (P-23) | I want large rooms to have cathedral-like reverb | acoustic environments feel distinct | F-5.2.7 | R-5.2.7 |
| US-5.2.7.4 | engine developer (P-26) | I want nested zones combined via priority ordering | specific zones override general ones | F-5.2.7 | R-5.2.7 |
| US-5.2.7.5 | audio designer (P-14) | I want early reflections from room geometry | spatial cues are reinforced | F-5.2.7 | R-5.2.7 |
| US-5.2.7.6 | engine tester (P-27) | I want to confirm mobile supports 1-2 concurrent zones | budget is respected | F-5.2.7 | R-5.2.7 |
| US-5.2.7.7 | designer (P-5) | I want to configure reverb zones in the visual editor | acoustic design is visual | F-5.2.7 | R-5.2.7 |
| US-5.2.7.8 | engine tester (P-27) | I want to test reverb zone blending at boundaries | transitions are smooth | F-5.2.7 | R-5.2.7 |
| US-5.2.7.9 | player (P-23) | I want echo to increase when entering a cave | acoustic changes are noticeable | F-5.2.7 | R-5.2.7 |
| US-5.2.7.10 | engine tester (P-27) | I want to confirm early reflections disabled on mobile | algorithmic reverb only is used | F-5.2.7 | R-5.2.7 |
| US-5.2.7.11 | audio designer (P-14) | I want to author early reflection patterns per zone | specific rooms have tailored acoustics | F-5.2.7 | R-5.2.7 |
| US-5.2.7.12 | engine developer (P-26) | I want desktop to support 6+ concurrent reverb zones | complex environments work | F-5.2.7 | R-5.2.7 |
