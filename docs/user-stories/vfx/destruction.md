# User Stories -- 11.5 Destruction VFX

## Debris and Fragments

| ID          | Persona               | Features | Requirements |
|-------------|-----------------------|----------|--------------|
| US-11.5.1.1 | Player (P-23)         | F-11.5.1 | R-11.5.1     |
| US-11.5.1.2 | Effects artist (P-12) | F-11.5.1 | R-11.5.1     |
| US-11.5.1.3 | Engine tester (P-27)  | F-11.5.1 | R-11.5.1     |
| US-11.5.2.1 | Player (P-23)         | F-11.5.2 | R-11.5.2     |
| US-11.5.2.2 | Engine tester (P-27)  | F-11.5.2 | R-11.5.2     |
| US-11.5.3.1 | Player (P-23)         | F-11.5.3 | R-11.5.3     |
| US-11.5.3.2 | Effects artist (P-12) | F-11.5.3 | R-11.5.3     |
| US-11.5.3.3 | Engine tester (P-27)  | F-11.5.3 | R-11.5.3     |

1. **US-11.5.1.1** — I want debris mesh fragments to emit from destruction events with
   physics-driven trajectories and material inherited from the source object, so that destroying a
   stone wall produces stone fragments rather than generic grey chunks.
   - **Acceptance:** Fragments spawn with matching material and velocity; material matches source
     object
2. **US-11.5.1.2** — I want per-object debris tables controlling fragment type, count, velocity
   cone, and material, so that each destructible object produces debris appropriate to its material
   and size.
   - **Acceptance:** Debris table drives fragment type, count, velocity cone, and material per
     object
3. **US-11.5.1.3** — I want to trigger 20 simultaneous destruction events and verify that the global
   debris budget (32 on mobile, 256 on desktop) caps concurrent fragments, so that frame time stays
   bounded during large battles.
   - **Acceptance:** Total concurrent fragments never exceeds budget cap; mobile: 32, desktop: 256
4. **US-11.5.2.1** — I want dust clouds colored by the destroyed material (grey for stone, brown for
   wood) that expand on impact and transition to drifting smoke plumes, so that destruction kick-up
   looks physically convincing.
   - **Acceptance:** Dust particle color matches material; fast expansion transitioning to slow
     drift
5. **US-11.5.2.2** — I want to destroy a structure in windy conditions and verify that smoke plumes
   drift with the wind field, so that destruction smoke is coherent with the weather system. Mobile
   skips wind interaction.
   - **Acceptance:** Smoke plume drift aligns with wind vector within 15 degrees; mobile skips wind
     interaction
6. **US-11.5.3.1** — I want bright spark bursts from metallic impacts with gravity, bounce
   collision, and rapid color fade from white-hot to dark, so that hitting metal surfaces produces
   convincing spark showers.
   - **Acceptance:** Spark particles apply gravity and bounce on collision; color interpolates from
     white to orange to dark
7. **US-11.5.3.2** — I want slow drifting ember particles that rise from burning debris with
   flickering emissive intensity and light emission, so that fire-damaged areas glow with residual
   heat.
   - **Acceptance:** Ember particles drift upward; flickering emissive intensity; light
     contributions in clustered light buffer
8. **US-11.5.3.3** — I want to verify that particle-emitted lights for sparks and embers are
   disabled on mobile and spark count is reduced to 25% of desktop, so that destruction VFX fit
   within mobile compute and lighting budgets.
   - **Acceptance:** Mobile: particle lights disabled, spark count at 25% of desktop

## Surface Damage

| ID          | Persona               | Features | Requirements |
|-------------|-----------------------|----------|--------------|
| US-11.5.4.1 | Player (P-23)         | F-11.5.4 | R-11.5.4     |
| US-11.5.4.2 | Effects artist (P-12) | F-11.5.4 | R-11.5.4     |
| US-11.5.4.3 | Engine tester (P-27)  | F-11.5.4 | R-11.5.4     |
| US-11.5.5.1 | Player (P-23)         | F-11.5.5 | R-11.5.5     |
| US-11.5.5.2 | Engine tester (P-27)  | F-11.5.5 | R-11.5.5     |

1. **US-11.5.4.1** — I want animated crack decals that spread from impact points over time, with
   growth speed scaling with accumulated damage, so that I get visual warning before full structural
   collapse during siege events.
   - **Acceptance:** Crack decals appear at impact point and grow outward; growth speed increases
     with accumulated damage; crack atlas matches surface material
2. **US-11.5.4.2** — I want pre-authored directional crack atlases selected by surface material
   (stone, concrete, wood), so that crack patterns look appropriate for each material type.
   - **Acceptance:** Crack atlas selected per surface material; distinct patterns for stone,
     concrete, wood
3. **US-11.5.4.3** — I want to verify that mobile uses static crack decals with no animated growth
   to avoid per-frame decal updates, so that crack rendering is cheap on mobile.
   - **Acceptance:** Mobile: static crack decals, no animated growth
4. **US-11.5.5.1** — I want long-lived burn decals that darken albedo, increase roughness, and
   flatten normals at fire and explosion sites, so that the battlefield shows lasting evidence of
   explosive combat.
   - **Acceptance:** Scorch decal modifies albedo, roughness, and normal channels; persists above
     transient combat decals; fades over configured world-time duration
5. **US-11.5.5.2** — I want to verify that mobile scorch marks modify albedo only (no roughness or
   normal changes) with shorter persistence, so that scorch decals reduce G-buffer writes on mobile.
   - **Acceptance:** Mobile: albedo-only modification, shorter persistence

## Explosive and Fire Effects

| ID          | Persona               | Features | Requirements |
|-------------|-----------------------|----------|--------------|
| US-11.5.6.1 | Player (P-23)         | F-11.5.6 | R-11.5.6     |
| US-11.5.6.2 | Engine tester (P-27)  | F-11.5.6 | R-11.5.6     |
| US-11.5.7.1 | Player (P-23)         | F-11.5.7 | R-11.5.7     |
| US-11.5.7.2 | Effects artist (P-12) | F-11.5.7 | R-11.5.7     |
| US-11.5.7.3 | Engine tester (P-27)  | F-11.5.7 | R-11.5.7     |

1. **US-11.5.6.1** — I want expanding spherical distortion waves that propagate from detonation
   points, displacing the color buffer radially with ground-contact dust rings, so that explosions
   have dramatic visual impact beyond the fireball.
   - **Acceptance:** Radial refraction ring expands outward; dust particles spawn at ground plane;
     camera shake decreases with distance
2. **US-11.5.6.2** — I want to verify that mobile skips screen-space distortion and uses camera
   shake plus dust particles only, with concurrent shockwaves capped at 1, so that explosion effects
   stay within mobile GPU budgets.
   - **Acceptance:** Mobile: no screen-space distortion, camera shake and dust only, max 1
     concurrent shockwave
3. **US-11.5.7.1** — I want fire to propagate across flammable surfaces along material connections
   at wind-influenced rates, with flame particles, emissive overlays, and light emission, so that
   setting a wooden tower on fire produces realistic spreading flames.
   - **Acceptance:** Fire spreads across flammable material; does not spread onto non-flammable
     surfaces; wind increases spread rate in downwind direction
4. **US-11.5.7.2** — I want per-texel propagation maps tracking burn state across surfaces, so that
   fire spreads along defined flammable paths rather than randomly.
   - **Acceptance:** Propagation map tracks burn state; fire follows flammable material connections
5. **US-11.5.7.3** — I want to verify that mobile uses quarter texel density propagation maps with
   no wind-influenced spread and caps fire light sources to 2, so that fire propagation cost scales
   per platform.
   - **Acceptance:** Mobile: quarter texel density, no wind spread, max 2 fire light sources

## Voxel and SDF Integration

| ID           | Persona               | Features  | Requirements |
|--------------|-----------------------|-----------|--------------|
| US-11.5.8.1  | Effects artist (P-12) | F-11.5.8  | R-11.5.8     |
| US-11.5.8.2  | Effects artist (P-12) | F-11.5.8  | R-11.5.8     |
| US-11.5.9.1  | Effects artist (P-12) | F-11.5.9  | R-11.5.9     |
| US-11.5.9.2  | Effects artist (P-12) | F-11.5.9  | R-11.5.9     |
| US-11.5.10.1 | Effects artist (P-12) | F-11.5.10 | R-11.5.10    |
| US-11.5.10.2 | Effects artist (P-12) | F-11.5.10 | R-11.5.10    |
| US-11.5.11.1 | Designer (P-5)        | F-11.5.11 | R-11.5.11    |
| US-11.5.11.2 | Designer (P-5)        | F-11.5.11 | R-11.5.11    |

1. **US-11.5.8.1** — I want voxel edit operations to emit particles whose color and texture match
   the voxel material ID (brown dust for dirt, grey chips for stone, translucent shards for ice), so
   that terrain excavation produces visually appropriate debris.
   - **Acceptance:** Particle colors match material lookup table; distinct particles per material
     type
2. **US-11.5.8.2** — I want larger voxel excavations to produce proportionally more particles
   distributed across the SDF edit boundary surface, so that big blasts feel more impactful than
   small digs.
   - **Acceptance:** Particle count increases proportionally with edit volume; distributed across
     edit boundary
3. **US-11.5.9.1** — I want GPU particles to collide with SDF voxel volumes in world-space using
   sphere-traced queries, so that particles bounce off deformed terrain, carved caves, and tunnels
   without requiring mesh collision regeneration.
   - **Acceptance:** Particles bounce off cave ceiling and walls; collision works with newly carved
     surfaces on next frame
4. **US-11.5.9.2** — I want SDF collision for world-space voxel volumes and depth-buffer collision
   for screen-space geometry to work together without double-collision artifacts, so that particles
   interact correctly with all scene geometry.
   - **Acceptance:** SDF and depth-buffer collision coexist; no double-collision artifacts
5. **US-11.5.10.1** — I want destroying a wooden object to produce splinter fragment meshes, brown
   dust trails, and sawdust particles matching the wood material preset, so that each material type
   has a distinct destruction character.
   - **Acceptance:** Wood destruction produces splinters, brown dust, sawdust; matches wood material
     preset
6. **US-11.5.10.2** — I want destroying a metal object to produce metallic shards, bright sparks,
   and a secondary ringing audio cue matching the metal material preset, so that metal destruction
   looks and sounds different from stone or wood.
   - **Acceptance:** Metal destruction produces shards, sparks, ringing audio; visually distinct
     from other materials
7. **US-11.5.11.1** — I want destruction audio and VFX to trigger within the same frame via a
   unified event bus, so that the visual and auditory impact of destruction are perfectly
   synchronized.
   - **Acceptance:** Audio and VFX fire within same frame; no perceptible delay between audio and
     visual
8. **US-11.5.11.2** — I want destruction events with more fragments to produce layered audio with
   more concurrent voices, so that large-scale destruction sounds more complex and impactful than
   small breakages.
   - **Acceptance:** Audio voice count scales with fragment count; large destructions sound more
     complex
