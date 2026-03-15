# User Stories -- 11.5 Destruction VFX

## US-11.5.1.1 See Debris Fly When Structures Break Apart

**As a** player (P-23), **I want** debris mesh fragments to emit from destruction events with
physics-driven trajectories and material inherited from the source object, **so that**
destroying a stone wall produces stone fragments rather than generic grey chunks.

## US-11.5.1.2 Author Debris Tables Per Destructible Object

**As a** effects artist (P-12), **I want** per-object debris tables controlling fragment type,
count, velocity cone, and material, **so that** each destructible object produces debris
appropriate to its material and size.

## US-11.5.1.3 Validate Global Debris Budget During Siege Battles

**As an** engine tester (P-27), **I want** to trigger 20 simultaneous destruction events and
verify that the global debris budget (32 on mobile, 256 on desktop) caps concurrent fragments,
**so that** frame time stays bounded during large battles.

## US-11.5.2.1 See Material-Colored Dust Clouds at Destruction Sites

**As a** player (P-23), **I want** dust clouds colored by the destroyed material (grey for
stone, brown for wood) that expand on impact and transition to drifting smoke plumes, **so
that** destruction kick-up looks physically convincing.

## US-11.5.2.2 Validate Smoke Plume Wind Interaction

**As an** engine tester (P-27), **I want** to destroy a structure in windy conditions and verify
that smoke plumes drift with the wind field, **so that** destruction smoke is coherent with the
weather system. Mobile skips wind interaction.

## US-11.5.3.1 See White-Hot Sparks From Metal Impacts

**As a** player (P-23), **I want** bright spark bursts from metallic impacts with gravity,
bounce collision, and rapid color fade from white-hot to dark, **so that** hitting metal
surfaces produces convincing spark showers.

## US-11.5.3.2 Author Ember Effects for Burning Wreckage

**As a** effects artist (P-12), **I want** slow drifting ember particles that rise from burning
debris with flickering emissive intensity and light emission, **so that** fire-damaged areas
glow with residual heat.

## US-11.5.3.3 Validate Spark and Ember Light Emission on Mobile

**As an** engine tester (P-27), **I want** to verify that particle-emitted lights for sparks
and embers are disabled on mobile and spark count is reduced to 25% of desktop, **so that**
destruction VFX fit within mobile compute and lighting budgets.

## US-11.5.4.1 Watch Cracks Spread Across Damaged Walls Before Collapse

**As a** player (P-23), **I want** animated crack decals that spread from impact points over
time, with growth speed scaling with accumulated damage, **so that** I get visual warning before
full structural collapse during siege events.

## US-11.5.4.2 Author Crack Atlas Variants Per Surface Material

**As a** effects artist (P-12), **I want** pre-authored directional crack atlases selected by
surface material (stone, concrete, wood), **so that** crack patterns look appropriate for each
material type.

## US-11.5.4.3 Validate Static Crack Decals on Mobile

**As an** engine tester (P-27), **I want** to verify that mobile uses static crack decals with
no animated growth to avoid per-frame decal updates, **so that** crack rendering is cheap on
mobile.

## US-11.5.5.1 See Persistent Scorch Marks at Explosion Sites

**As a** player (P-23), **I want** long-lived burn decals that darken albedo, increase
roughness, and flatten normals at fire and explosion sites, **so that** the battlefield shows
lasting evidence of explosive combat.

## US-11.5.5.2 Validate Scorch Mark Channel Modification on Mobile

**As an** engine tester (P-27), **I want** to verify that mobile scorch marks modify albedo only
(no roughness or normal changes) with shorter persistence, **so that** scorch decals reduce
G-buffer writes on mobile.

## US-11.5.6.1 See Expanding Shockwave Distortion From Explosions

**As a** player (P-23), **I want** expanding spherical distortion waves that propagate from
detonation points, displacing the color buffer radially with ground-contact dust rings, **so
that** explosions have dramatic visual impact beyond the fireball.

## US-11.5.6.2 Validate Shockwave Rendering on Mobile

**As an** engine tester (P-27), **I want** to verify that mobile skips screen-space distortion
and uses camera shake plus dust particles only, with concurrent shockwaves capped at 1, **so
that** explosion effects stay within mobile GPU budgets.

## US-11.5.7.1 Watch Fire Spread Across Wooden Structures

**As a** player (P-23), **I want** fire to propagate across flammable surfaces along material
connections at wind-influenced rates, with flame particles, emissive overlays, and light
emission, **so that** setting a wooden tower on fire produces realistic spreading flames.

## US-11.5.7.2 Author Fire Propagation Maps Per Destructible Structure

**As a** effects artist (P-12), **I want** per-texel propagation maps tracking burn state
across surfaces, **so that** fire spreads along defined flammable paths rather than randomly.

## US-11.5.7.3 Validate Fire Propagation Resolution on Mobile

**As an** engine tester (P-27), **I want** to verify that mobile uses quarter texel density
propagation maps with no wind-influenced spread and caps fire light sources to 2, **so that**
fire propagation cost scales per platform.
