# User Stories — 3.4 Water

## US-3.4.1 Simulate Realistic Ocean Waves

**As a** player, **I want** ocean waves computed via FFT with multiple spectral cascades producing
displacement, normal, and fold maps, **so that** the ocean looks physically plausible from large
swells down to capillary ripples.

## US-3.4.2 Blend Water at Shorelines

**As a** player, **I want** water to fade smoothly into terrain at shorelines with depth-based
opacity, reduced wave amplitude, and animated foam, **so that** beaches and rocky shores look
natural.

## US-3.4.3 Explore Underwater Environments

**As a** player, **I want** the renderer to switch to underwater mode with depth fog, Beer-Lambert
absorption, refracted surface view, and volumetric god rays, **so that** underwater zones feel
immersive.

## US-3.4.4 See Caustic Light Underwater

**As a** player, **I want** refracted light patterns projected onto the seabed and underwater
surfaces, **so that** shallow water and underwater environments have visual depth and richness.

## US-3.4.5 See Reflections and Refraction on Water

**As a** player, **I want** water surfaces with Fresnel-weighted reflections (SSR, cubemap, optional
planar) and normal-map-driven refraction distortion, **so that** water looks physically correct from
all viewing angles.

## US-3.4.6 Create Rivers with Directional Flow

**As a** world artist, **I want** to paint flow maps on river splines that drive UV animation of
normal and foam textures, **so that** rivers visually flow in the correct direction and connect
seamlessly with the ocean.

## US-3.4.7 Generate Dynamic Foam

**As a** player, **I want** foam generated from wave folding, shoreline depth, flow turbulence, and
object wakes with time-based decay, **so that** whitecaps, surf, and wake foam appear consistently
across all water bodies.
