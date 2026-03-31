# 13.8 — Character Customization

## Facial Morphing

| ID       | Feature                          |
|----------|----------------------------------|
| F-13.8.1 | Parametric Facial Feature System |
| F-13.8.2 | Preset Blending and Templates    |

1. **F-13.8.1** — Per-region morph target controls for eyes, nose, mouth, jaw, cheeks, ears, and
   forehead. Each region exposes 3D coordinate sliders (height, width, depth) plus rotation,
   combined with fine-grained sculpt markers for vertex-level displacement. Symmetry is enforced by
   default with an option to break symmetry per marker. Morph targets are blended additively so
   multiple adjustments compose without interference.
   - **Deps:** F-9.2.1 (Blend Shapes), F-1.1.1 (ECS)
2. **F-13.8.2** — Select multiple preset face configurations and interpolate between them with
   per-preset weight sliders. Presets are stored as morph target weight vectors — blending produces
   a weighted average. A library of base templates (by ethnicity, age range, body type) provides
   starting points. Players can save, load, and share custom presets as serialized data.
   - **Deps:** F-13.8.1

## Body Proportions

| ID       | Feature                             |
|----------|-------------------------------------|
| F-13.8.3 | Parametric Body Shape System        |
| F-13.8.4 | Body Morph Propagation to Equipment |

1. **F-13.8.3** — Continuous sliders for height, chest, waist, hips, shoulder width, arm length, leg
   length, musculature, and body fat. Body shapes are defined by blending between archetypal morph
   targets (slim, heavy, muscular, tall, short). Skeleton scale bones adjust proportions while mesh
   morph targets refine silhouette. Measurement constraints ensure plausible proportions.
   - **Deps:** F-9.2.1 (Blend Shapes), F-9.1.1 (Skeletal Animation)
2. **F-13.8.4** — Body shape changes automatically propagate to equipped clothing and armor meshes.
   Equipment meshes are authored with matching morph targets or use a runtime conform system that
   deforms equipment vertices to follow the underlying body surface. Rigid armor pieces (plate,
   helmets) attach to bones and skip deformation. Prevents clipping across all body types.
   - **Deps:** F-13.8.3, F-13.8.9

## Skin and Appearance

| ID       | Feature                            |
|----------|------------------------------------|
| F-13.8.5 | Skin Material System               |
| F-13.8.6 | Makeup and Face Paint Layer System |
| F-13.8.7 | Eye Customization                  |

1. **F-13.8.5** — Full-spectrum skin tone selection with physically-based subsurface scattering
   adjusted per tone. Layered detail maps control wrinkles/aging, freckles (density, saturation),
   stubble, pores, roughness (oily vs matte), and vascular detail (broken capillaries). Detail
   layers compose additively and are stored as compact parameter sets, not unique textures per
   character.
   - **Deps:** F-2.4.1 (Materials)
2. **F-13.8.6** — Decal-based appearance layers for foundation, blush, lipstick, eyeliner,
   eyeshadow, face paint, tattoos, and scars. Each layer has color, opacity, blend mode, and a mask
   texture. Layers are composited at runtime into the character's material without modifying base
   textures. Supports arbitrary placement via UV-space projection for tattoos and body markings.
   - **Deps:** F-13.8.5
3. **F-13.8.7** — Per-eye iris color (supporting heterochromia), iris pattern texture, pupil size,
   sclera color and vein visibility, limbal ring darkness, cornea clarity and refraction, and eye
   wetness. Eyes use a layered material (sclera → iris → cornea) with separate controls per layer.
   Presets provide common combinations as starting points.
   - **Deps:** F-2.4.1 (Materials)

## Hair

| ID       | Feature                   |
|----------|---------------------------|
| F-13.8.8 | Hair Customization System |

1. **F-13.8.8** — Selectable hairstyles, facial hair, eyebrows, and eyelashes as swappable groom or
   mesh assets. Per-groom controls for color (primary, highlight, ombre gradient), length, curl
   pattern, and density. Hair color optionally propagates to eyebrows and eyelashes with
   per-component override. Strand-based rendering at high LOD transitions to card-based at lower
   LODs per the hair LOD system (F-9.5.4).
   - **Deps:** F-9.5.2 (Strand Hair), F-9.5.3 (Card Hair), F-9.5.4 (Hair LOD)

## Modular Equipment

| ID        | Feature                                |
|-----------|----------------------------------------|
| F-13.8.9  | Modular Mesh Part System               |
| F-13.8.10 | Equipment Attachment and Socket System |
| F-13.8.11 | Transmog and Appearance Override       |

1. **F-13.8.9** — Characters assembled from interchangeable mesh parts: head, torso, arms, legs,
   feet, hands, and sub-parts (shoulder pads, knee guards, hip attachments). Each slot accepts any
   compatible mesh asset. Parts share a common skeleton and are combined at runtime via master-pose
   component sharing or mesh merging. Per-part material and color customization via shader
   parameters.
   - **Deps:** F-9.1.1 (Skeletal Animation), F-1.1.1 (ECS)
2. **F-13.8.10** — Named attachment points (head, back, shoulders, elbows, hips, knees, hands) bound
   to skeleton bones. Equipment entities attach to sockets and inherit bone transforms. Supports
   rigid attachments (weapons, shields) and skinned attachments (gloves, boots). Body mesh regions
   under opaque equipment are hidden to prevent clipping and save GPU cost.
   - **Deps:** F-13.8.9, F-9.1.1 (Skeletal Animation)
3. **F-13.8.11** — Separate visual appearance from gameplay stats — equip stat-optimal gear while
   displaying a different appearance set. Account-wide wardrobe collects all unlocked appearances.
   Outfit slots save complete appearance loadouts for quick swapping. Dye system allows
   per-equipment- slot color customization with preservation rules across outfit changes.
   - **Deps:** F-13.8.9, F-13.7.2 (Data Tables)

## Race and Species

| ID        | Feature                      |
|-----------|------------------------------|
| F-13.8.12 | Multi-Race Base Mesh Support |

1. **F-13.8.12** — Multiple base mesh sets for distinct races or species (humanoid, elven, bestial,
   mechanical). Each race defines its own skeleton, morph target set, and compatible equipment
   catalog. Animation retargeting (F-9.1.1) enables shared animations across races with compatible
   skeletons. Race-specific morph sliders (ear length for elves, tail shape for beast races) extend
   the base customization.
   - **Deps:** F-13.8.1, F-13.8.3, F-9.1.1 (Skeletal Animation)

## Performance

| ID        | Feature                              |
|-----------|--------------------------------------|
| F-13.8.13 | Character LOD and Crowd Optimization |
| F-13.8.14 | Mesh Merging and Draw Call Reduction |

1. **F-13.8.13** — Multi-level LOD for customized characters: full morph targets and strand hair at
   close range, simplified mesh and card hair at medium range, instanced low-poly or impostor
   billboards at extreme range. Facial animation and detail layers are culled beyond configurable
   distances. GPU-driven instanced rendering supports hundreds of unique characters visible
   simultaneously with per-submesh instancing for variation.
   - **Deps:** F-9.1.6 (Instanced Animation), F-13.8.9
   - **Platform:** Mobile uses aggressive LOD distances — card hair and simplified meshes at shorter
     range, impostor billboards earlier. Fewer simultaneous unique characters rendered (50 vs 200+
     on desktop).
2. **F-13.8.14** — Combine a character's modular mesh parts into a single merged skinned mesh at
   runtime to reduce draw calls from N parts to 1. Merging runs asynchronously and produces a new
   mesh asset cached per unique part combination. Characters in the distance use merged meshes;
   nearby characters keep separate parts for hot-swapping equipment without re-merge.
   - **Deps:** F-13.8.9

## Serialization

| ID        | Feature                            |
|-----------|------------------------------------|
| F-13.8.15 | Character Appearance Serialization |

1. **F-13.8.15** — Serialize all customization parameters (morph weights, colors, part selections,
   layer settings, equipment) into a compact binary or JSON format. Version-tagged for forward
   compatibility — old saves load in new builds with migration logic for added parameters. Supports
   save/load to local storage, cloud save (F-13.3.5), and player-to-player sharing via export/import
   of appearance data files.
   - **Deps:** F-1.4.1 (Serialization), F-13.3.1 (Save System)
