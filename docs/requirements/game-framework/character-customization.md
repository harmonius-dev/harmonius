# R-13.8 — Character Customization Requirements

## Facial Morphing

| ID       | Derived From                                                         |
|----------|----------------------------------------------------------------------|
| R-13.8.1 | [F-13.8.1](../../features/game-framework/character-customization.md) |
| R-13.8.2 | [F-13.8.2](../../features/game-framework/character-customization.md) |

1. **R-13.8.1** — The engine **SHALL** provide per-region morph target controls (eyes, nose, mouth,
   jaw, cheeks, ears, forehead) with 3D coordinate sliders and rotation, supporting symmetry
   enforcement with optional per-marker symmetry breaking and additive blending across multiple
   adjustments.
   - **Rationale:** Per-region parametric controls enable fine-grained facial customization while
     additive blending ensures adjustments compose without overwriting each other.
   - **Verification:** Unit test: set morph targets on two overlapping facial regions and verify the
     resulting vertex positions reflect additive composition. Toggle symmetry breaking on a marker
     and verify left/right sides can diverge independently.
2. **R-13.8.2** — The engine **SHALL** interpolate between multiple preset face configurations using
   per-preset weight sliders (stored as morph target weight vectors), provide a library of base
   templates, and support save/load/share of custom presets as serialized data.
   - **Rationale:** Preset blending lowers the skill floor for character creation by providing
     curated starting points that players refine rather than build from scratch.
   - **Verification:** Unit test: blend two presets at 50/50 weights and verify the resulting morph
     vector equals the arithmetic mean. Save a custom preset, reload it, and assert all morph
     weights are restored within floating-point tolerance.

## Body Proportions

| ID       | Derived From                                                         |
|----------|----------------------------------------------------------------------|
| R-13.8.3 | [F-13.8.3](../../features/game-framework/character-customization.md) |
| R-13.8.4 | [F-13.8.4](../../features/game-framework/character-customization.md) |

1. **R-13.8.3** — The engine **SHALL** provide continuous sliders for body proportions (height,
   chest, waist, hips, shoulder width, arm length, leg length, musculature, body fat) blending
   between archetypal morph targets with skeleton scale bone adjustments, enforcing measurement
   constraints for plausible proportions.
   - **Rationale:** Combined morph target and skeleton scaling produces natural-looking body
     variation while constraints prevent implausible proportions.
   - **Verification:** Unit test: set extreme slider values and verify mesh vertex positions and
     bone scales stay within defined plausibility bounds. Verify intermediate slider values produce
     smooth interpolation without mesh artifacts.
2. **R-13.8.4** — The engine **SHALL** propagate body shape changes to equipped clothing and armor
   meshes via matching morph targets or a runtime conform system, with rigid armor pieces attaching
   to bones and skipping deformation, preventing clipping across all body types.
   - **Rationale:** Equipment must follow body shape to prevent visible clipping, which breaks
     visual quality and player immersion across diverse character builds.
   - **Verification:** Visual test: apply extreme body shapes with each equipment type and verify no
     mesh penetration. Unit test: verify rigid armor pieces maintain bone-relative transforms and do
     not receive morph deformation.

## Skin and Appearance

| ID       | Derived From                                                         |
|----------|----------------------------------------------------------------------|
| R-13.8.5 | [F-13.8.5](../../features/game-framework/character-customization.md) |
| R-13.8.6 | [F-13.8.6](../../features/game-framework/character-customization.md) |
| R-13.8.7 | [F-13.8.7](../../features/game-framework/character-customization.md) |

1. **R-13.8.5** — The engine **SHALL** provide full-spectrum skin tone selection with
   physically-based subsurface scattering per tone and layered detail maps (wrinkles, freckles,
   stubble, pores, roughness, vascular detail) composing additively as compact parameter sets
   without unique per-character textures.
   - **Rationale:** Parameter-driven skin materials avoid the memory cost of unique textures per
     character while enabling high visual fidelity through physically-based shading.
   - **Verification:** Unit test: verify that two characters with identical skin parameters produce
     the same material output. Visual test: render skin tones across the full spectrum and verify
     subsurface scattering adapts correctly per tone.
2. **R-13.8.6** — The engine **SHALL** composite decal-based appearance layers (foundation, blush,
   lipstick, eyeliner, eyeshadow, face paint, tattoos, scars) at runtime with per-layer color,
   opacity, blend mode, and mask texture, supporting UV-space projection for arbitrary placement
   without modifying base textures.
   - **Rationale:** Runtime compositing avoids baking unique textures per character, saving memory
     and enabling real-time appearance changes in the character creator.
   - **Verification:** Unit test: apply 3 overlapping layers and verify compositing order and blend
     modes produce correct pixel values. Verify UV-space projection places a tattoo at the specified
     coordinates on the mesh.
3. **R-13.8.7** — The engine **SHALL** support per-eye customization of iris color (including
   heterochromia), iris pattern, pupil size, sclera color and vein visibility, limbal ring, cornea
   clarity and refraction, and eye wetness via a layered eye material (sclera, iris, cornea).
   - **Rationale:** Eyes are a primary focal point in character close-ups; layered material controls
     enable photorealistic and stylized eye rendering.
   - **Verification:** Unit test: set left and right eye to different iris colors and verify each
     eye material instance reflects the correct value. Visual test: render the eye material and
     verify cornea refraction responds to clarity parameter changes.

## Hair

| ID       | Derived From                                                         |
|----------|----------------------------------------------------------------------|
| R-13.8.8 | [F-13.8.8](../../features/game-framework/character-customization.md) |

1. **R-13.8.8** — The engine **SHALL** support swappable hairstyle, facial hair, eyebrow, and
   eyelash assets with per-groom controls for color (primary, highlight, ombre gradient), length,
   curl pattern, and density, transitioning between strand-based rendering at high LOD and
   card-based at lower LODs.
   - **Rationale:** Swappable groom assets with parametric controls provide wide visual variety
     while LOD transitions maintain performance at distance.
   - **Verification:** Unit test: swap hairstyle assets and verify the new groom renders with
     correct color and curl parameters. Verify LOD transition from strand to card rendering occurs
     at the configured distance threshold without visible popping.

## Modular Equipment

| ID        | Derived From                                                          |
|-----------|-----------------------------------------------------------------------|
| R-13.8.9  | [F-13.8.9](../../features/game-framework/character-customization.md)  |
| R-13.8.10 | [F-13.8.10](../../features/game-framework/character-customization.md) |
| R-13.8.11 | [F-13.8.11](../../features/game-framework/character-customization.md) |

1. **R-13.8.9** — The engine **SHALL** assemble characters from interchangeable mesh parts (head,
   torso, arms, legs, feet, hands, and sub-parts) sharing a common skeleton, combined at runtime via
   master-pose component sharing or mesh merging, with per-part material and color customization.
   - **Rationale:** Modular mesh parts enable combinatorial equipment variety without authoring
     unique full-body meshes for every combination.
   - **Verification:** Unit test: assemble a character from 6 mesh parts and verify all parts share
     skeleton transforms and animate in sync. Swap one part and verify the replacement integrates
     without seam artifacts at part boundaries.
2. **R-13.8.10** — The engine **SHALL** attach equipment entities to named skeleton bone sockets
   (head, back, shoulders, elbows, hips, knees, hands) inheriting bone transforms, supporting both
   rigid and skinned attachments, and hiding body mesh regions under opaque equipment to prevent
   clipping and save GPU cost.
   - **Rationale:** Bone-socket attachments ensure equipment follows character animation, while body
     region hiding eliminates overdraw and clipping artifacts.
   - **Verification:** Unit test: attach a rigid weapon to the hand socket and verify its transform
     matches the hand bone each frame. Equip opaque chest armor and verify the underlying torso mesh
     region is culled from rendering.
3. **R-13.8.11** — The engine **SHALL** separate visual appearance from gameplay stats, maintaining
   an account-wide wardrobe of unlocked appearances with outfit slot loadouts and a per-slot dye
   system that preserves color customization across outfit changes.
   - **Rationale:** Transmog decouples visual identity from gear optimization, increasing player
     engagement and cosmetic value without affecting gameplay balance.
   - **Verification:** Unit test: equip stat gear and apply a different appearance override; verify
     stats come from the equipped item and visuals come from the override. Save and load an outfit
     loadout and verify all appearance and dye values are restored.

## Race and Species

| ID        | Derived From                                                          |
|-----------|-----------------------------------------------------------------------|
| R-13.8.12 | [F-13.8.12](../../features/game-framework/character-customization.md) |

1. **R-13.8.12** — The engine **SHALL** support multiple base mesh sets for distinct races or
   species, each defining its own skeleton, morph target set, and compatible equipment catalog, with
   animation retargeting across races with compatible skeletons and race-specific morph sliders
   extending base customization.
   - **Rationale:** Multiple base meshes enable visually distinct playable races while animation
     retargeting maximizes animation asset reuse across races.
   - **Verification:** Unit test: spawn two characters of different races and verify each uses its
     own skeleton and morph set. Play a shared animation on both via retargeting and verify correct
     playback. Attempt to equip a race-incompatible item and verify rejection.

## Performance

| ID        | Derived From                                                          |
|-----------|-----------------------------------------------------------------------|
| R-13.8.13 | [F-13.8.13](../../features/game-framework/character-customization.md) |
| R-13.8.14 | [F-13.8.14](../../features/game-framework/character-customization.md) |

1. **R-13.8.13** — The engine **SHALL** apply multi-level LOD to customized characters (full morph
   and strand hair at close range, simplified mesh and card hair at medium, instanced low-poly or
   impostor billboards at extreme range), culling facial animation and detail layers beyond
   configurable distances, and supporting GPU-driven instanced rendering of hundreds of unique
   characters.
   - **Rationale:** Aggressive LOD and instancing are essential to render large crowds of uniquely
     customized characters within frame budget.
   - **Verification:** Benchmark: render 500 unique characters and verify frame time stays below 16
     ms at 1080p on target hardware. Verify facial detail layers are culled beyond the configured
     distance threshold.
2. **R-13.8.14** — The engine **SHALL** combine a character's modular mesh parts into a single
   merged skinned mesh at runtime to reduce draw calls, running the merge asynchronously and caching
   results per unique part combination, with nearby characters keeping separate parts for equipment
   hot-swapping.
   - **Rationale:** Merging N mesh parts into 1 draw call dramatically reduces CPU overhead for
     distant characters where equipment changes are infrequent.
   - **Verification:** Unit test: merge 6 mesh parts and verify the resulting mesh produces 1 draw
     call with correct vertex data. Verify the merge runs asynchronously without blocking the main
     thread. Verify cache hit on a repeated part combination.

## Serialization

| ID        | Derived From                                                          |
|-----------|-----------------------------------------------------------------------|
| R-13.8.15 | [F-13.8.15](../../features/game-framework/character-customization.md) |

1. **R-13.8.15** — The engine **SHALL** serialize all customization parameters (morph weights,
   colors, part selections, layer settings, equipment) into a compact version-tagged binary or JSON
   format with forward-compatible migration logic, supporting save/load to local storage, cloud
   save, and player-to-player sharing via export/import.
   - **Rationale:** Version-tagged serialization ensures old character saves remain loadable as the
     customization system evolves, while compact format minimizes storage and transfer cost.
   - **Verification:** Unit test: serialize a fully customized character, deserialize it, and assert
     all parameters match. Serialize with version N, add a new parameter in version N+1, and verify
     migration fills the new parameter with its default value.

## Non-Functional Requirements

| ID         | Derived From                 |
|------------|------------------------------|
| R-13.8.NF1 | F-13.8.1, F-13.8.3, F-13.8.8 |
| R-13.8.NF2 | F-13.8.1, F-13.8.3           |
| R-13.8.NF3 | F-13.8.15                    |

1. **R-13.8.NF1** — The engine **SHALL** load the character creation screen (all base meshes, morph
   targets, presets, hair assets, and material systems) within 3 seconds from menu selection on
   target hardware.
   - **Rationale:** Long character creator load times frustrate new players during their first
     interaction with the game and increase bounce rates during first-time setup.
   - **Verification:** From the main menu, select "Create Character" and measure time until the
     fully interactive character creator is displayed. Verify load time is under 3 seconds on target
     hardware across 10 consecutive loads.
2. **R-13.8.NF2** — The engine **SHALL** update the character mesh in response to morph target
   slider changes within 1 frame (16.67 ms at 60 fps), providing real-time visual feedback during
   customization.
   - **Rationale:** Real-time slider response is fundamental to an interactive character creator;
     any visible lag between slider movement and mesh deformation breaks the creative flow.
   - **Verification:** Move a facial morph slider continuously for 5 seconds. Measure the time from
     slider value change to mesh vertex update on each frame. Verify all updates complete within the
     same frame as the input.
3. **R-13.8.NF3** — The engine **SHALL** serialize a fully customized character appearance (all
   morph weights, colors, part selections, layers, equipment) into no more than 16 KB, enabling
   efficient network transmission for player-to-player sharing and multiplayer replication.
   - **Rationale:** Character appearance data is replicated to every nearby player in multiplayer;
     large payloads increase bandwidth consumption and initial replication latency.
   - **Verification:** Fully customize a character with all available options. Serialize the
     appearance data and verify the output size is under 16 KB.
