# R-11.4 — Weather & Environmental FX Requirements

## R-11.4.1 Rain Particle System and Screen Droplets

The engine **SHALL** render multi-layered rain combining GPU particle streaks for world-space
rainfall, screen-space droplet simulation with gravity flow, merge, and evaporation behavior, and
ripple normal maps on wet surfaces, with particle density scaling by weather intensity and camera
sky exposure.

- **Derived from:** [F-11.4.1](../../features/vfx/weather-environmental.md)
- **Rationale:** Layered rain rendering provides convincing precipitation from distant streaks to
  near-camera droplets, essential for immersive weather transitions.
- **Verification:** Enable rain at full intensity and verify visible world-space particle streaks,
  screen-space droplets flowing downward, and ripple normals on surfaces; reduce weather intensity
  and confirm particle density decreases proportionally; move the camera under shelter and verify
  screen droplets cease while world-space rain continues outside; stop the storm and confirm screen
  droplets evaporate over time.

## R-11.4.2 Rain Puddles and Wet Surfaces

The engine **SHALL** form dynamic puddles driven by heightfield accumulation and terrain concavity,
modifying surface roughness toward mirror-smooth, darkening albedo, adding animated ripple normals
during rainfall, and applying material-driven wet responses that drain over time after rain stops.

- **Derived from:** [F-11.4.2](../../features/vfx/weather-environmental.md)
- **Rationale:** Puddles and wet surfaces ground rainfall in the environment by showing
  material-appropriate moisture accumulation and drainage.
- **Verification:** Enable rain over varied terrain and verify puddles form in concave areas;
  confirm stone surfaces darken, metal surfaces increase reflectivity, and dirt transitions to mud;
  verify animated ripple normals appear in puddles during active rain; stop rain and confirm puddle
  depth drains over time with surfaces returning to dry state.

## R-11.4.3 Snow Accumulation and Interaction

The engine **SHALL** render vertex-displacement snow that accumulates on upward-facing surfaces over
time using a world-space height texture updated by weather state, with character and vehicle
movement carving deformation trails via depth stamps that reveal the underlying surface and fade
under continued snowfall.

- **Derived from:** [F-11.4.3](../../features/vfx/weather-environmental.md)
- **Rationale:** Interactive snow accumulation creates a living environment that responds to both
  weather progression and player activity.
- **Verification:** Enable snowfall and verify snow visibly accumulates on upward-facing surfaces
  over time; walk a character through accumulated snow and confirm deformation trails appear
  revealing the underlying material; continue snowfall and verify deformation trails gradually fill
  in; confirm non-upward-facing surfaces such as walls remain snow-free.

## R-11.4.4 Fog Volumes

The engine **SHALL** render localized volumetric fog regions defined by oriented boxes or convex
hulls with per-volume density, color, height falloff, and animation parameters, injecting density
into the global volumetric fog froxel grid with temporal reprojection.

- **Derived from:** [F-11.4.4](../../features/vfx/weather-environmental.md)
- **Rationale:** Localized fog volumes enable area-specific atmospheric effects such as swamp haze
  and dungeon mist without altering the global atmosphere.
- **Verification:** Place a box fog volume in a scene and verify fog density is confined to the
  volume bounds; adjust density, color, and height falloff and confirm each parameter affects the
  volume independently; verify the fog volume injects into the froxel grid and participates in
  temporal reprojection without ghosting; confirm areas outside the volume remain unaffected.

## R-11.4.5 Lightning

The engine **SHALL** render procedural branching lightning bolts using L-system subdivision with
randomized branching angle and segment length, emitting a burst of light into the scene's lighting
system with single-frame illumination and exponential decay, and supporting multiple simultaneous
bolts with distance-delayed audio cues.

- **Derived from:** [F-11.4.5](../../features/vfx/weather-environmental.md)
- **Rationale:** Lightning provides dramatic visual punctuation during storms and requires
  synchronized lighting and audio for believable atmospheric presentation.
- **Verification:** Trigger a lightning bolt and verify a visible branching geometry with randomized
  structure; confirm the scene is illuminated for a single frame with exponential decay; trigger
  multiple simultaneous bolts and verify independent rendering; verify thunder audio cue delay
  increases with distance from the bolt origin.

## R-11.4.6 Wind Visualization and Dust Storms

The engine **SHALL** render GPU particle-driven wind visualization of leaves, dust, and debris with
velocity sampled from the shared wind vector volume, and simulate dust storms by injecting
scattering density into atmospheric fog and tinting the sky toward the storm color to reduce
visibility.

- **Derived from:** [F-11.4.6](../../features/vfx/weather-environmental.md)
- **Rationale:** Wind-driven particles and dust storms make the wind field visually tangible and
  create dramatic weather transitions across open-world zones.
- **Verification:** Enable wind with particle visualization and verify leaves and debris move
  coherently with vegetation sway direction; trigger a dust storm and confirm visibility decreases
  as scattering density increases in atmospheric fog; verify sky color tints toward the configured
  storm color; disable the wind field and confirm particle motion ceases.

## R-11.4.7 Underwater Caustics and Depth Fog

The engine **SHALL** render underwater environments with animated caustic light patterns projected
onto submerged geometry, exponential depth fog with wavelength-dependent absorption, particle-driven
bubble streams, refraction distortion at the water surface boundary, and screen-space god rays from
the surface.

- **Derived from:** [F-11.4.7](../../features/vfx/weather-environmental.md)
- **Rationale:** Underwater rendering requires specialized optical effects to simulate light
  behavior through water, including caustics, absorption, and refraction.
- **Verification:** Submerge the camera and verify animated caustic patterns appear on submerged
  geometry; confirm distant objects fade with exponential fog where red attenuates before blue;
  verify bubble particle streams are visible; cross the water surface boundary and confirm
  refraction distortion at the interface; verify screen-space god rays emanate from the surface
  direction.
