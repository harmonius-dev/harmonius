# User Stories — 3.4 Water

## Stories

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-3.4.1 | player | I want ocean waves computed via FFT with multiple spectral cascades producing displacement, normal, and fold maps | the ocean looks physically plausible from large swells down to capillary ripples |  |  |
| US-3.4.2 | player | I want water to fade smoothly into terrain at shorelines with depth-based opacity, reduced wave amplitude, and animated foam | beaches and rocky shores look natural |  |  |
| US-3.4.3 | player | I want the renderer to switch to underwater mode with depth fog, Beer-Lambert absorption, refracted surface view, and volumetric god rays | underwater zones feel immersive |  |  |
| US-3.4.4 | player | I want refracted light patterns projected onto the seabed and underwater surfaces | shallow water and underwater environments have visual depth and richness |  |  |
| US-3.4.5 | player | I want water surfaces with Fresnel-weighted reflections (SSR, cubemap, optional planar) and normal-map-driven refraction distortion | water looks physically correct from all viewing angles |  |  |
| US-3.4.6 | world artist | I want to paint flow maps on river splines that drive UV animation of normal and foam textures | rivers visually flow in the correct direction and connect seamlessly with the ocean |  |  |
| US-3.4.7 | player | I want foam generated from wave folding, shoreline depth, flow turbulence, and object wakes with time-based decay | whitecaps, surf, and wake foam appear consistently across all water bodies |  |  |
