# Animation

How characters and objects move: skeletal poses, blended states, procedural motion, and physical
secondary motion.

## Topics

- [skeletal-and-states](./skeletal-and-states.md) — joint hierarchies, animation clips, blending,
  and state machines.
- [procedural-and-physical](./procedural-and-physical.md) — IK, look-at, springs, cloth, and hair.
- [matching-and-blending](./matching-and-blending.md) — motion matching, morph targets, and
  pose synthesis from motion libraries.
- [character-and-first-person](./character-and-first-person.md) — viewmodel animation, weapon
  poses, and first-person specific layers.

## Key takeaways

- State machines organize animations hierarchically (locomotion → walk/run/jump) with transitions
  gated by conditions, simplifying authoring and behavior reuse.
- Additive blending layers procedural offsets (recoil, flinch) or directional motions (strafing)
  on top of base animations, enabling dynamic variations from limited clips.
- IK solvers (FABRIK, analytic) position end effectors (hands, feet) by solving joint rotations,
  enabling natural interaction without unique animations per target position.
- Motion matching queries a pose database for closest match, enabling seamless transitions between
  distinct animation styles without explicit state machine paths.
- Ragdoll mode transitions physics-driven simulation, enabling limp bodies and impact-responsive
  limbs; partial ragdoll keeps torso animated while freeing limbs.

## Integration risks

- State machine transition conditions (speed > 0 → walk) must account for frame timing variations;
  hysteresis (speed 0.5 to enter, -0.5 to exit) prevents oscillation. See [skeletal-and-states.md](./skeletal-and-states.md)
  for hysteresis implementation.
- IK solver iteration count (solver passes) affects accuracy vs cost; low iterations cause
  foot placement inaccuracy or visible jitter. See [procedural-and-physical.md](./procedural-and-physical.md)
  for per-limb iteration tuning.
- Motion matching pose database must cover diverse states (all speeds, directions); sparse database
  causes visible popping between states. See [matching-and-blending.md](./matching-and-blending.md)
  for database coverage requirements.
- Ragdoll initialization must set bone velocities matching animation to prevent sudden limb
  snapping. See [procedural-and-physical.md](./procedural-and-physical.md) for smooth transition.
- First-person arm mesh position relative to camera affects gun sight placement; misalignment
  causes aiming drift. See [character-and-first-person.md](./character-and-first-person.md)
  for offset tuning.
