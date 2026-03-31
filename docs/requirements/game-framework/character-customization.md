# R-13.8 -- Character Customization Requirements

## Morph Targets

1. **R-13.8.1** -- The engine **SHALL** support per-region morph target controls with 3D coordinate
   sliders, additive blending, and optional symmetry enforcement with per-marker symmetry breaking.
   - **Rationale:** Additive blending ensures multiple adjustments compose without interference.
   - **Verification:** Adjust two overlapping regions and verify the result equals the additive sum.
     Break symmetry on one marker and verify independent control of each side.

2. **R-13.8.2** -- The engine **SHALL** support preset face configurations stored as morph target
   weight vectors with per-preset weight sliders that interpolate via weighted average.
   - **Rationale:** Weight-vector interpolation provides predictable blending between
     artist-authored presets.
   - **Verification:** Blend two presets at 50/50 and verify the resulting weights are the
     mathematical average.

## Body Shape

1. **R-13.8.3** -- The engine **SHALL** provide parametric body shape controls blending between
   archetypal morph targets with skeleton scale bone adjustments and measurement constraints.
   - **Rationale:** Combined mesh and bone scaling produces plausible body variation while
     constraints prevent impossible proportions.
   - **Verification:** Set extreme values and verify constraints clamp to valid ranges.

2. **R-13.8.4** -- The engine **SHALL** propagate body shape changes to equipped mesh parts using
   morph targets or a runtime conform system, with rigid pieces attaching to bones without
   deformation.
   - **Rationale:** Automatic propagation prevents clipping without per-combination authoring.
   - **Verification:** Equip deformable armor on varied body shapes and verify no clipping. Equip
     rigid armor and verify bone attachment.

## Skin, Eyes, and Hair

1. **R-13.8.5** -- The engine **SHALL** provide a layered skin material with full-spectrum tone
   selection, subsurface scattering, and additive detail maps stored as compact parameter sets.
   - **Rationale:** Parameter-based layers avoid per-character textures while enabling rich
     variation.
   - **Verification:** Enable all detail layers and verify additive composition without blending
     artifacts.

2. **R-13.8.6** -- The engine **SHALL** composite decal-based appearance layers at runtime into the
   character material without modifying base textures.
   - **Rationale:** Non-destructive compositing enables unlimited combinations from a fixed set of
     base textures.
   - **Verification:** Apply layers, remove them, and verify the base texture is unchanged.

3. **R-13.8.7** -- The engine **SHALL** render eyes using a layered material (sclera, iris, cornea)
   with per-eye color and per-layer controls.
   - **Rationale:** Layered rendering enables heterochromia and fine-grained customization.
   - **Verification:** Set different iris colors per eye and verify correct independent rendering.

4. **R-13.8.8** -- The engine **SHALL** support swappable hair assets with per-groom controls and
   LOD transitions from strand-based to card-based rendering.
   - **Rationale:** LOD transitions ensure hair quality scales with viewing distance.
   - **Verification:** Walk toward a character and verify smooth LOD transition from card to strand
     hair.

## Modular Equipment

1. **R-13.8.9** -- The engine **SHALL** assemble characters from interchangeable mesh parts sharing
   a common skeleton with runtime master-pose sharing or mesh merging.
   - **Rationale:** Modular assembly enables combinatorial variety from a finite set of authored
     parts.
   - **Verification:** Assemble a character from parts by different artists and verify all animate
     correctly.

2. **R-13.8.10** -- The engine **SHALL** attach equipment to named skeleton bone sockets and hide
   body mesh regions under opaque equipment.
   - **Rationale:** Socket attachment with body hiding prevents clipping and saves GPU cost.
   - **Verification:** Equip opaque armor and verify the underlying body region is hidden.

3. **R-13.8.11** -- The engine **SHALL** separate visual appearance from gameplay stats with an
   account-wide wardrobe, outfit slots, and a per-slot dye system preserving colors across outfit
   changes.
   - **Rationale:** Appearance override enables player expression without gameplay sacrifice.
   - **Verification:** Apply dye, change outfit, change back, verify dye colors are preserved.

## Race, Performance, and Serialization

1. **R-13.8.12** -- The engine **SHALL** support multiple base mesh sets per race with race-specific
   morph sliders and animation retargeting.
   - **Rationale:** Multi-race support enables distinct identities while shared animations reduce
     authoring cost.
   - **Verification:** Play a shared animation on two race skeletons and verify retargeting produces
     correct results.

2. **R-13.8.13** -- The engine **SHALL** provide multi-level LOD for customized characters with
   impostor billboards at extreme range and GPU-driven instanced rendering for crowds.
   - **Rationale:** LOD and instancing render hundreds of unique characters simultaneously.
   - **Verification:** Render 200 unique characters and verify frame time under 16 ms at 1080p.

3. **R-13.8.14** -- The engine **SHALL** merge modular mesh parts into a single skinned mesh at
   distance, cached per unique combination.
   - **Rationale:** Mesh merging reduces draw calls from N parts to 1.
   - **Verification:** Equip different parts and verify the cached merged mesh is correct for each
     combination.

4. **R-13.8.15** -- The engine **SHALL** serialize all customization parameters into a compact
   version-tagged format with forward- compatible migration logic.
   - **Rationale:** Versioned serialization ensures old saves load in new builds.
   - **Verification:** Save in version N, upgrade to N+1, load, and verify new parameters receive
     correct defaults.
