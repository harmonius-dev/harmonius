# User Stories -- 2.8 Character Rendering

## Stories

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-2.8.1.1 | character artist (P-9) | I want to render hundreds of thousands of individual hair strands with Marschner anisotropic BSDF shading and deep opacity self-shadowing | hero characters have photorealistic hair in cinematic close-ups |  |  |
| US-2.8.1.2 | engine developer (P-26) | I want to benchmark strand-based hair rendering at 100K strands (desktop) and 500K+ strands (high-end) and measure GPU time per frame | I can set per-platform strand budgets that fit within the character rendering allocation |  |  |
| US-2.8.1.3 | engine tester (P-27) | I want to confirm that strand-based hair is automatically disabled on mobile and Switch and that card-based hair (F-2.8.2) activates as the fallback | characters render correctly on lower-tier platforms |  |  |
| US-2.8.2.1 | character artist (P-9) | I want to arrange textured polygon strips in layers with alpha blending to approximate hair volume | mid-distance NPCs have convincing hair at a fraction of the strand rendering cost |  |  |
| US-2.8.2.2 | engine tester (P-27) | I want to verify that mobile uses 2-3 card layers with alpha-test only and desktop uses full card layers with alpha blending | card-based hair respects per-platform overdraw budgets |  |  |
| US-2.8.3.1 | player (P-23) | I want hair to transition smoothly from full strands (close) to cards (mid) to mesh proxy (far) with cross-fade dithering | hair quality changes are imperceptible during gameplay |  |  |
| US-2.8.3.2 | engine tester (P-27) | I want to walk a camera from near to far and verify that LOD transitions fire at correct screen-size thresholds per platform | strand-to-card and card-to-mesh transitions occur at the specified distances |  |  |
| US-2.8.4.1 | character artist (P-9) | I want a layered eye shading model with transparent cornea, iris parallax depth, sclera SSS, and limbal ring darkening | character eyes look lifelike in dialogue close-ups |  |  |
| US-2.8.4.2 | engine tester (P-27) | I want to verify that mobile uses a single-layer flat iris with limbal ring (no cornea refraction or parallax) | eye rendering remains functional on mobile without shader complexity overflow |  |  |
| US-2.8.5.1 | character artist (P-9) | I want a cloth shading model with a fuzz layer that replaces standard specular with fabric-appropriate fiber scattering | robes, scarves, and armor padding look distinctly cloth rather than plastic |  |  |
| US-2.8.5.2 | engine tester (P-27) | I want to verify that high-end enables subsurface transmission for thin fabrics and mobile uses simplified wrap lighting | cloth quality scales across all platforms |  |  |
| US-2.8.6.1 | character artist (P-9) | I want to adjust per-material scatter radius, scatter color, and transmission distance for skin SSS and see results in real time | I can iterate on skin appearance without export-and-test cycles |  |  |
| US-2.8.6.2 | engine tester (P-27) | I want to screenshot-compare full Burley diffusion SSS on desktop against the preintegrated LUT fallback on mobile | skin rendering on mobile remains visually acceptable relative to the reference |  |  |
| US-2.8.7.1 | engine developer (P-26) | I want a compute shader software rasterizer for sub-pixel hair strands that bypasses hardware rasterization | thin strands render with correct anti-aliasing and transparency ordering via froxel-based line registration |  |  |
| US-2.8.7.2 | engine tester (P-27) | I want to confirm that compute software rasterized hair is enabled alongside strand rendering on desktop and high-end, and disabled on mobile and Switch | the feature only runs where strand rendering is active |  |  |
| US-2.8.8.1 | player (P-23) | I want visible vellus hair rendered as a screen-space fuzz layer on character faces | skin looks natural and organic in cinematic close-ups rather than perfectly smooth |  |  |
| US-2.8.8.2 | engine tester (P-27) | I want to dolly the camera toward a character face and verify that peach fuzz enables only when the face exceeds the configurable screen-size threshold | GPU cost is spent only when the effect is visible |  |  |
| US-2.8.9.1 | character artist (P-9) | I want to parameterize skin appearance using melanin concentration and blood distribution maps instead of explicit diffuse color | any skin tone renders with correct subsurface scattering from biological pigment parameters |  |  |
| US-2.8.9.2 | engine tester (P-27) | I want to verify that melanin and blood maps are baked to diffuse color at import on mobile with no runtime pigment evaluation | the biometric skin model degrades without GPU overhead on low-tier platforms |  |  |
