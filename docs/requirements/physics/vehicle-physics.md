# R-4.5 -- Vehicle Physics Requirements

1. **R-4.5.1** -- The engine **SHALL** simulate wheel suspension with configurable stiffness,
   damping, and travel limits via ray-cast ground detection per wheel.
   - **Rationale:** Suspension determines ride quality and vehicle stability over terrain.
   - **Verification:** Drive over bumps. Assert suspension compresses and rebounds within configured
     limits.

2. **R-4.5.2** -- The engine **SHALL** compute tire forces using Pacejka Magic Formula coefficients
   with surface-type friction curves, falling back to linear friction on mobile.
   - **Rationale:** Pacejka produces realistic tire grip; mobile needs a simpler model for
     performance.
   - **Verification:** Drive on ice vs asphalt. Assert different grip levels. Assert mobile uses
     linear model.

3. **R-4.5.3** -- The engine **SHALL** simulate drivetrains with engine torque curves, gear ratios,
   differential types, and configurable drive layout (FWD, RWD, AWD).
   - **Rationale:** Drivetrain simulation produces authentic acceleration and handling
     characteristics.
   - **Verification:** Configure FWD and RWD. Assert front wheels receive torque in FWD. Assert rear
     in RWD.

4. **R-4.5.4** -- The engine **SHALL** support anti-roll bars and stability control as optional
   components disableable per entity.
   - **Rationale:** Stability assists prevent rollover while allowing arcade-style handling when
     disabled.
   - **Verification:** Enable anti-roll bars. Assert reduced body roll in corners. Disable. Assert
     increased roll.

5. **R-4.5.5** -- The engine **SHALL** simulate tracked vehicles with differential track steering
   and configurable ground contact sampling.
   - **Rationale:** Tanks and construction vehicles steer by varying track speeds, not wheel angles.
   - **Verification:** Apply differential track input. Assert the vehicle pivots. Assert ground
     casts scale per platform.

6. **R-4.5.6** -- The engine **SHALL** simulate hover vehicles with configurable repulsor height,
   force falloff, and tilt stabilization.
   - **Rationale:** Hover vehicles are a distinct vehicle class requiring repulsion-based physics.
   - **Verification:** Fly a hover vehicle over terrain edges. Assert it stays level. Assert
     repulsion force varies with height.

7. **R-4.5.7** -- The engine **SHALL** replicate vehicle state via the ECS state replication system
   with client prediction and server snapshot reconciliation.
   - **Rationale:** Multiplayer vehicles require synchronized state without custom serialization.
   - **Verification:** Drive in multiplayer. Assert client prediction. Apply a server correction.
     Assert smooth reconciliation.
