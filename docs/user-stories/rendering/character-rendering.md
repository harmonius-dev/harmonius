# User Stories -- 2.8 Character Rendering

## US-2.8.1.1 Render Strand-Based Hair on Hero Characters

**As a** character artist (P-9), **I want** to render hundreds of thousands of individual hair
strands with Marschner anisotropic BSDF shading and deep opacity self-shadowing, **so that** hero
characters have photorealistic hair in cinematic close-ups.

## US-2.8.1.2 Profile Strand Rendering Cost at 100K vs 500K Strands

**As an** engine developer (P-26), **I want** to benchmark strand-based hair rendering at 100K
strands (desktop) and 500K+ strands (high-end) and measure GPU time per frame, **so that** I can set
per-platform strand budgets that fit within the character rendering allocation.

## US-2.8.1.3 Verify Strand Hair Is Disabled on Mobile and Switch

**As an** engine tester (P-27), **I want** to confirm that strand-based hair is automatically
disabled on mobile and Switch and that card-based hair (F-2.8.2) activates as the fallback,
**so that** characters render correctly on lower-tier platforms.

## US-2.8.2.1 Author Card-Based Hair for NPCs and Crowd Characters

**As a** character artist (P-9), **I want** to arrange textured polygon strips in layers with alpha
blending to approximate hair volume, **so that** mid-distance NPCs have convincing hair at a
fraction of the strand rendering cost.

## US-2.8.2.2 Validate Card Layer Count Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile uses 2-3 card layers with
alpha-test only and desktop uses full card layers with alpha blending, **so that** card-based hair
respects per-platform overdraw budgets.

## US-2.8.3.1 See Smooth Hair LOD Transitions at Varying Distances

**As a** player (P-23), **I want** hair to transition smoothly from full strands (close) to cards
(mid) to mesh proxy (far) with cross-fade dithering, **so that** hair quality changes are
imperceptible during gameplay.

## US-2.8.3.2 Test Hair LOD Threshold Distances Per Platform

**As an** engine tester (P-27), **I want** to walk a camera from near to far and verify that LOD
transitions fire at correct screen-size thresholds per platform, **so that** strand-to-card and
card-to-mesh transitions occur at the specified distances.

## US-2.8.4.1 Author Realistic Eyes With Cornea Refraction and Iris Parallax

**As a** character artist (P-9), **I want** a layered eye shading model with transparent cornea,
iris parallax depth, sclera SSS, and limbal ring darkening, **so that** character eyes look lifelike
in dialogue close-ups.

## US-2.8.4.2 Validate Eye Shading Fallback on Mobile

**As an** engine tester (P-27), **I want** to verify that mobile uses a single-layer flat iris with
limbal ring (no cornea refraction or parallax), **so that** eye rendering remains functional on
mobile without shader complexity overflow.

## US-2.8.5.1 Create Cloth Materials With Fiber Scattering

**As a** character artist (P-9), **I want** a cloth shading model with a fuzz layer that replaces
standard specular with fabric-appropriate fiber scattering, **so that** robes, scarves, and armor
padding look distinctly cloth rather than plastic.

## US-2.8.5.2 Test Cloth Shading With Subsurface Transmission on High-End

**As an** engine tester (P-27), **I want** to verify that high-end enables subsurface transmission
for thin fabrics and mobile uses simplified wrap lighting, **so that** cloth quality scales across
all platforms.

## US-2.8.6.1 Preview Skin Subsurface Scattering Profiles in the Editor

**As a** character artist (P-9), **I want** to adjust per-material scatter radius, scatter color,
and transmission distance for skin SSS and see results in real time, **so that** I can iterate on
skin appearance without export-and-test cycles.

## US-2.8.6.2 Compare Burley Diffusion Model Against Preintegrated LUT

**As an** engine tester (P-27), **I want** to screenshot-compare full Burley diffusion SSS on
desktop against the preintegrated LUT fallback on mobile, **so that** skin rendering on mobile
remains visually acceptable relative to the reference.

## US-2.8.7.1 Render Sub-Pixel Hair Strands With Compute Rasterization

**As an** engine developer (P-26), **I want** a compute shader software rasterizer for sub-pixel
hair strands that bypasses hardware rasterization, **so that** thin strands render with correct
anti-aliasing and transparency ordering via froxel-based line registration.

## US-2.8.7.2 Validate Compute Hair Rasterizer Activates Only on Desktop+

**As an** engine tester (P-27), **I want** to confirm that compute software rasterized hair is
enabled alongside strand rendering on desktop and high-end, and disabled on mobile and Switch,
**so that** the feature only runs where strand rendering is active.

## US-2.8.8.1 See Peach Fuzz on Character Faces in Close-Up Views

**As a** player (P-23), **I want** visible vellus hair rendered as a screen-space fuzz layer on
character faces, **so that** skin looks natural and organic in cinematic close-ups rather than
perfectly smooth.

## US-2.8.8.2 Verify Peach Fuzz Activates Only Within Screen-Size Threshold

**As an** engine tester (P-27), **I want** to dolly the camera toward a character face and verify
that peach fuzz enables only when the face exceeds the configurable screen-size threshold,
**so that** GPU cost is spent only when the effect is visible.

## US-2.8.9.1 Author Diverse Skin Tones With Melanin and Blood Maps

**As a** character artist (P-9), **I want** to parameterize skin appearance using melanin
concentration and blood distribution maps instead of explicit diffuse color, **so that** any skin
tone renders with correct subsurface scattering from biological pigment parameters.

## US-2.8.9.2 Validate Biometric Skin Baked Fallback on Mobile

**As an** engine tester (P-27), **I want** to verify that melanin and blood maps are baked to
diffuse color at import on mobile with no runtime pigment evaluation, **so that** the biometric skin
model degrades without GPU overhead on low-tier platforms.
