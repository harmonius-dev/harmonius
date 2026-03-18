# User Stories -- 2.8 Character Rendering

## Stories

| ID         | Persona                 | Features | Requirements |
|------------|-------------------------|----------|--------------|
| US-2.8.1.1 | character artist (P-9)  |          |              |
| US-2.8.1.2 | engine developer (P-26) |          |              |
| US-2.8.1.3 | engine tester (P-27)    |          |              |
| US-2.8.2.1 | character artist (P-9)  |          |              |
| US-2.8.2.2 | engine tester (P-27)    |          |              |
| US-2.8.3.1 | player (P-23)           |          |              |
| US-2.8.3.2 | engine tester (P-27)    |          |              |
| US-2.8.4.1 | character artist (P-9)  |          |              |
| US-2.8.4.2 | engine tester (P-27)    |          |              |
| US-2.8.5.1 | character artist (P-9)  |          |              |
| US-2.8.5.2 | engine tester (P-27)    |          |              |
| US-2.8.6.1 | character artist (P-9)  |          |              |
| US-2.8.6.2 | engine tester (P-27)    |          |              |
| US-2.8.7.1 | engine developer (P-26) |          |              |
| US-2.8.7.2 | engine tester (P-27)    |          |              |
| US-2.8.8.1 | player (P-23)           |          |              |
| US-2.8.8.2 | engine tester (P-27)    |          |              |
| US-2.8.9.1 | character artist (P-9)  |          |              |
| US-2.8.9.2 | engine tester (P-27)    |          |              |

1. **US-2.8.1.1** — I want to render hundreds of thousands of individual hair strands with Marschner
   anisotropic BSDF shading and deep opacity self-shadowing
   - **Acceptance:** hero characters have photorealistic hair in cinematic close-ups
2. **US-2.8.1.2** — I want to benchmark strand-based hair rendering at 100K strands (desktop) and
   500K+ strands (high-end) and measure GPU time per frame
   - **Acceptance:** I can set per-platform strand budgets that fit within the character rendering
     allocation
3. **US-2.8.1.3** — I want to confirm that strand-based hair is automatically disabled on mobile and
   Switch and that card-based hair (F-2.8.2) activates as the fallback
   - **Acceptance:** characters render correctly on lower-tier platforms
4. **US-2.8.2.1** — I want to arrange textured polygon strips in layers with alpha blending to
   approximate hair volume
   - **Acceptance:** mid-distance NPCs have convincing hair at a fraction of the strand rendering
     cost
5. **US-2.8.2.2** — I want to verify that mobile uses 2-3 card layers with alpha-test only and
   desktop uses full card layers with alpha blending
   - **Acceptance:** card-based hair respects per-platform overdraw budgets
6. **US-2.8.3.1** — I want hair to transition smoothly from full strands (close) to cards (mid) to
   mesh proxy (far) with cross-fade dithering
   - **Acceptance:** hair quality changes are imperceptible during gameplay
7. **US-2.8.3.2** — I want to walk a camera from near to far and verify that LOD transitions fire at
   correct screen-size thresholds per platform
   - **Acceptance:** strand-to-card and card-to-mesh transitions occur at the specified distances
8. **US-2.8.4.1** — I want a layered eye shading model with transparent cornea, iris parallax depth,
   sclera SSS, and limbal ring darkening
   - **Acceptance:** character eyes look lifelike in dialogue close-ups
9. **US-2.8.4.2** — I want to verify that mobile uses a single-layer flat iris with limbal ring (no
   cornea refraction or parallax)
   - **Acceptance:** eye rendering remains functional on mobile without shader complexity overflow
10. **US-2.8.5.1** — I want a cloth shading model with a fuzz layer that replaces standard specular
    with fabric-appropriate fiber scattering
    - **Acceptance:** robes, scarves, and armor padding look distinctly cloth rather than plastic
11. **US-2.8.5.2** — I want to verify that high-end enables subsurface transmission for thin fabrics
    and mobile uses simplified wrap lighting
    - **Acceptance:** cloth quality scales across all platforms
12. **US-2.8.6.1** — I want to adjust per-material scatter radius, scatter color, and transmission
    distance for skin SSS and see results in real time
    - **Acceptance:** I can iterate on skin appearance without export-and-test cycles
13. **US-2.8.6.2** — I want to screenshot-compare full Burley diffusion SSS on desktop against the
    preintegrated LUT fallback on mobile
    - **Acceptance:** skin rendering on mobile remains visually acceptable relative to the reference
14. **US-2.8.7.1** — I want a compute shader software rasterizer for sub-pixel hair strands that
    bypasses hardware rasterization
    - **Acceptance:** thin strands render with correct anti-aliasing and transparency ordering via
      froxel-based line registration
15. **US-2.8.7.2** — I want to confirm that compute software rasterized hair is enabled alongside
    strand rendering on desktop and high-end, and disabled on mobile and Switch
    - **Acceptance:** the feature only runs where strand rendering is active
16. **US-2.8.8.1** — I want visible vellus hair rendered as a screen-space fuzz layer on character
    faces
    - **Acceptance:** skin looks natural and organic in cinematic close-ups rather than perfectly
      smooth
17. **US-2.8.8.2** — I want to dolly the camera toward a character face and verify that peach fuzz
    enables only when the face exceeds the configurable screen-size threshold
    - **Acceptance:** GPU cost is spent only when the effect is visible
18. **US-2.8.9.1** — I want to parameterize skin appearance using melanin concentration and blood
    distribution maps instead of explicit diffuse color
    - **Acceptance:** any skin tone renders with correct subsurface scattering from biological
      pigment parameters
19. **US-2.8.9.2** — I want to verify that melanin and blood maps are baked to diffuse color at
    import on mobile with no runtime pigment evaluation
    - **Acceptance:** the biometric skin model degrades without GPU overhead on low-tier platforms
