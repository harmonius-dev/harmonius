# User Stories — 3.4 Water

## Stories

| ID       | Persona      | Features | Requirements |
|----------|--------------|----------|--------------|
| US-3.4.1 | player       |          |              |
| US-3.4.2 | player       |          |              |
| US-3.4.3 | player       |          |              |
| US-3.4.4 | player       |          |              |
| US-3.4.5 | player       |          |              |
| US-3.4.6 | world artist |          |              |
| US-3.4.7 | player       |          |              |

1. **US-3.4.1** — I want ocean waves computed via FFT with multiple spectral cascades producing
   displacement, normal, and fold maps
   - **Acceptance:** the ocean looks physically plausible from large swells down to capillary
     ripples
2. **US-3.4.2** — I want water to fade smoothly into terrain at shorelines with depth-based opacity,
   reduced wave amplitude, and animated foam
   - **Acceptance:** beaches and rocky shores look natural
3. **US-3.4.3** — I want the renderer to switch to underwater mode with depth fog, Beer-Lambert
   absorption, refracted surface view, and volumetric god rays
   - **Acceptance:** underwater zones feel immersive
4. **US-3.4.4** — I want refracted light patterns projected onto the seabed and underwater surfaces
   - **Acceptance:** shallow water and underwater environments have visual depth and richness
5. **US-3.4.5** — I want water surfaces with Fresnel-weighted reflections (SSR, cubemap, optional
   planar) and normal-map-driven refraction distortion
   - **Acceptance:** water looks physically correct from all viewing angles
6. **US-3.4.6** — I want to paint flow maps on river splines that drive UV animation of normal and
   foam textures
   - **Acceptance:** rivers visually flow in the correct direction and connect seamlessly with the
     ocean
7. **US-3.4.7** — I want foam generated from wave folding, shoreline depth, flow turbulence, and
   object wakes with time-based decay
   - **Acceptance:** whitecaps, surf, and wake foam appear consistently across all water bodies
