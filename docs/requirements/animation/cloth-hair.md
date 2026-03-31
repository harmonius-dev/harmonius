# R-9.5 -- Cloth and Hair Simulation Requirements

## Cloth Simulation

1. **R-9.5.1** -- The engine **SHALL** run position-based dynamics cloth simulation on GPU compute
   with distance, bending, and self-collision constraints, driven by wind, skeletal animation, and
   bone-attached collision capsules.
   - **Rationale:** GPU cloth simulation provides realistic garment movement without CPU overhead,
     with fallback for lower platforms.
   - **Verification:** Simulate a cape on a running character. Verify distance and bending
     constraints prevent stretching. Verify collision capsules prevent cloth passing through limbs.
     Verify mobile uses baked animation fallback.

## Hair Systems

1. **R-9.5.2** -- The engine **SHALL** simulate strand-based hair using guide curves with stretch,
   bend, and collision constraints, driving interpolated render strands via skinning weights.
   - **Rationale:** Strand-based hair provides the highest fidelity for hero characters on desktop
     where simulation cost is justified.
   - **Verification:** Simulate 128 guide strands on a character. Verify collision with skeletal
     capsules. Verify wind and gravity response. Verify strand hair is desktop-only.

2. **R-9.5.3** -- The engine **SHALL** render card-based hair with alpha-tested or alpha-blended
   transparency and anisotropic specular shading, driven by simple spring physics or baked
   animation.
   - **Rationale:** Card-based hair provides a performant alternative for NPCs and lower platforms
     while maintaining visual quality at medium distances.
   - **Verification:** Render card hair on mobile and verify correct transparency and specular.
     Verify spring physics responds to movement. Verify card count scales per platform tier.

3. **R-9.5.4** -- The engine **SHALL** transition hair between strand, cluster, card, and shell
   representations based on camera distance with temporal blending to avoid popping.
   - **Rationale:** Hair LOD is critical for scenes with hundreds of visible characters across
     varying distances.
   - **Verification:** Move the camera from close to far and verify smooth LOD transitions. Verify
     no visible pop at tier boundaries.

## Cloth-Body and Wind

1. **R-9.5.5** -- The engine **SHALL** resolve cloth-body collisions using capsule and convex hull
   proxies attached to skeleton bones, with friction and sticking contacts.
   - **Rationale:** Cloth-body collision prevents garments from passing through limbs during fast
     movement and animation.
   - **Verification:** Run a fast arm-swing animation. Verify cloth does not penetrate the arm
     capsule. Verify collision proxy count scales per platform tier.

2. **R-9.5.6** -- The engine **SHALL** apply directional and turbulent wind forces to both
   strand-based and card-based hair from the shared wind field texture.
   - **Rationale:** Wind-responsive hair maintains visual coherence with all other simulated
     elements in the scene.
   - **Verification:** Place a wind source and verify strand hair responds with per-particle drag.
     Verify card hair responds with simplified spring displacement.
