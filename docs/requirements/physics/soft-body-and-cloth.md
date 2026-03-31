# R-4.7 -- Soft Body & Cloth Requirements

1. **R-4.7.1** -- The engine **SHALL** solve soft body constraints via XPBD querying ClothSimulation
   ECS components with GPU buffer handles and per-entity compliance parameters.
   - **Rationale:** XPBD within the ECS schedule avoids a separate physics world for cloth.
   - **Verification:** Simulate a cloth. Assert constraint resolution. Assert stiffness varies with
     compliance.

2. **R-4.7.2** -- The engine **SHALL** support cloth instances attached to skeleton bones via
   ClothAttachment, with active instance count capped per platform tier.
   - **Rationale:** Skeleton attachment enables capes and banners following animated characters.
   - **Verification:** Attach cloth to a skeleton. Assert cloth follows bone movement. Assert mobile
     caps at 2 instances.

3. **R-4.7.3** -- The engine **SHALL** support cloth self-collision via a SelfCollisionEnabled
   marker component, disabled by default on mobile.
   - **Rationale:** Self-collision prevents cloth from passing through itself but is expensive on
     mobile.
   - **Verification:** Enable self-collision. Assert folding cloth does not penetrate itself. Assert
     mobile default is disabled.

4. **R-4.7.4** -- The engine **SHALL** provide two-way rigid body coupling where cloth generates
   contact constraints against nearby colliders and writes reaction forces to rigid body entities.
   - **Rationale:** Two-way coupling makes cloth-object interaction physically convincing.
   - **Verification:** Drop cloth on a small rigid body. Assert the body moves. Assert cloth drapes
     correctly.

5. **R-4.7.5** -- The engine **SHALL** generate a shared 3D wind field texture from WindSource ECS
   entities, sampled by cloth, hair, foliage, and particles.
   - **Rationale:** A unified wind field ensures all wind consumers respond consistently.
   - **Verification:** Place a wind source. Assert cloth and foliage both deflect in the same
     direction.

6. **R-4.7.6** -- The engine **SHALL** support cloth tearing when constraint strain exceeds a
   configurable threshold, splitting topology and spawning new cloth entities.
   - **Rationale:** Destructible sails and battle-damaged banners require runtime topology changes.
   - **Verification:** Apply excessive strain. Assert the cloth splits. Assert torn edges generate
     proper boundary normals.

7. **R-4.7.7** -- The engine **SHALL** reduce cloth simulation fidelity (particle count, iterations,
   update frequency) with distance via a ClothLod component, replacing extreme-distance cloth with
   an animation fallback at zero simulation cost.
   - **Rationale:** Only nearby cloth justifies full simulation cost.
   - **Verification:** Place cloth at increasing distances. Assert LOD tiers reduce simulation.
     Assert animation fallback at maximum distance.
