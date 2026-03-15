# 2.8 — Character Rendering

### F-2.8.1 Strand-Based Hair Rendering

Individual hair strand rendering using curve geometry rasterized via the mesh shader pipeline. Each
strand is shaded with the Marschner anisotropic hair BSDF providing R (reflection), TT
(transmission), and TRT (double internal reflection) lobes. Supports hundreds of thousands of strands
with proper self-shadowing via deep opacity maps.

- **Requirements:** R-2.8.1
- **Dependencies:** None
- **Platform notes:** Mobile: disabled; uses card-based hair (F-2.8.2). Switch:
  disabled; card-based fallback. Desktop: enabled for hero characters; max 100k
  strands. High-end: full strand rendering; 500k+ strands with deep opacity maps.

### F-2.8.2 Card-Based Hair Rendering

Hair approximation using textured polygon strips (cards) with alpha-tested or alpha-blended
rendering. Cards are arranged in layers to build up volume and directionality. Lower cost than strand
rendering, suitable for mid-distance characters and performance-constrained scenarios.

- **Requirements:** R-2.8.2
- **Dependencies:** None
- **Platform notes:** Mobile: primary hair method; reduced card layers (2-3); alpha-test
  only (no blending). Switch: 4-6 card layers; alpha blending for hero characters.
  Desktop: full card layers with alpha blending. High-end: used for distant LOD only;
  strands preferred up close.

### F-2.8.3 Hair LOD System

Multi-tier hair level-of-detail: full strand rendering at close range, card-based representation at
mid range, and a simplified mesh proxy (hair helmet) at far distance. LOD transitions are
screen-size-driven with cross-fade dithering.

- **Requirements:** R-2.8.3
- **Dependencies:** F-2.8.1, F-2.8.2
- **Platform notes:** Mobile: card LOD and mesh proxy only (no strand tier). Switch:
  same as mobile. Desktop: all 3 tiers; strand threshold at closer distance. High-end:
  all 3 tiers with extended strand rendering distance.

### F-2.8.4 Eye Rendering

Specialized eye shading model with layered cornea refraction, iris parallax depth, sclera subsurface
scattering, and limbal ring darkening. The cornea is a separate transparent layer refracting the view
into the iris detail underneath. Caustic highlights are approximated analytically.

- **Requirements:** R-2.8.4
- **Dependencies:** F-2.4.9
- **Platform notes:** Mobile: single-layer eye (no cornea refraction or parallax);
  flat iris texture with limbal ring. Switch: parallax iris without cornea refraction.
  Desktop: full layered eye model. High-end: full model with sclera SSS and caustic
  highlights.

### F-2.8.5 Cloth Rendering

Fabric shading model simulating a fuzz layer across cloth surfaces. Light interacts with the fiber
microstructure differently than smooth surfaces, producing a soft energy-conserving wrap that
replaces standard specular with a fabric-appropriate lobe. Driven by cloth-specific material
parameters (fuzz color, scatter width).

- **Requirements:** R-2.8.5
- **Dependencies:** F-2.4.9
- **Platform notes:** Mobile: simplified wrap lighting (no fuzz layer); single diffuse
  lobe. Switch: fuzz layer on hero characters only. Desktop: full cloth shading model.
  High-end: full model with subsurface transmission for thin fabrics.

### F-2.8.6 Skin Rendering (Subsurface Profiles)

High-fidelity screen-space subsurface scattering optimized for skin using diffusion profiles.
Per-material scatter radius, scatter color, and transmission distance control how light diffuses
beneath the surface. Supports the Burley normalized diffusion model for physically accurate results.

- **Requirements:** R-2.8.6
- **Dependencies:** F-2.4.4
- **Platform notes:** Mobile: preintegrated skin LUT only (no screen-space diffusion
  blur). Switch: screen-space SSS at half resolution with 3 samples. Desktop: full
  Burley diffusion model at native resolution. High-end: full model with RT
  transmission for backlit skin.

### F-2.8.7 Compute Software Rasterized Hair

Compute shader software rasterizer for sub-pixel hair strands that bypasses hardware rasterization.
Strands are classified by projected pixel area: large segments use hardware rasterization while thin
strands use froxel-based line segment registration with anti-aliased 2D line drawing. Front-to-back
froxel rasterization with early termination when tiles become sufficiently opaque provides correct
transparency ordering.

- **Requirements:** R-2.8.7
- **Dependencies:** F-2.8.1
- **Platform notes:** Mobile: disabled (strand rendering disabled). Switch: disabled.
  Desktop: enabled alongside strand rendering for sub-pixel strands. High-end: full
  compute rasterizer with high tile resolution and anti-aliased line drawing.

### F-2.8.8 Peach Fuzz (Vellus Hair)

Shader rendering visible vellus hair on character faces and skin surfaces. A fine-grain fuzzy layer
is rendered as a screen-space effect driven by a fuzz direction map, producing the subtle
light-catching peach fuzz visible in close-up character rendering without requiring individual strand
geometry.

- **Requirements:** R-2.8.8
- **Dependencies:** F-2.8.6
- **Platform notes:** Mobile: disabled; subtle effect not visible at mobile resolutions.
  Switch: disabled. Desktop: enabled for hero characters in close-up views only.
  High-end: enabled for all skin-shaded characters within configurable screen-size
  threshold.

### F-2.8.9 Biometric Skin Model

Physically-based skin shading driven by melanin concentration and blood distribution maps rather than
explicit subsurface color. The model accurately displays any skin tone with correct subsurface
scattering by parameterizing the biological pigment layers, producing natural variation across body
regions.

- **Requirements:** R-2.8.9
- **Dependencies:** F-2.8.6
- **Platform notes:** Mobile: melanin/blood maps baked to diffuse color at import; no
  runtime pigment evaluation. Switch: simplified 2-layer pigment model. Desktop: full
  multi-layer biometric model. High-end: full model with per-region body variation.
