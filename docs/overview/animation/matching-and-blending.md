# Matching and Blending

Motion matching, phase alignment, and advanced blending for seamless transitions.

## What it covers

- Motion matching: querying a motion database for similar poses.
- Pose databases: storing animation frames for lookup.
- Feature extraction: joint velocities, direction, phase for distance metrics.
- Lerp vs motion-matched transitions: choosing when to motion match.
- Phase-aligned blending: synchronizing clips to matching phases.
- Slope blending: matching surface angles for foot placement.
- Directional blending: choosing animations by character direction.
- Weight-driven locomotion: blending walk, strafe, back-pedal by movement direction.
- Parkour transitions: blending between distinct animation styles (running, vaulting, sliding).
- Smooth transitions across animation styles without glitchy motion.

## Concepts

### Motion Matching Foundations

Motion matching queries a database of animation frames (poses) to find the best match for current
state. Rather than maintaining complex state machines, motion matching finds the closest pose in
the database and blends to it. The database stores poses from multiple animations; during playback,
the system continuously queries for the best match. This produces smooth transitions between any
animation styles.

### Feature-Based Distance Metrics

Pose similarity uses features: joint positions, velocities, phase (foot phase in gait cycle),
direction heading. Distance metrics weight each feature; similar joints and matching phase produce
low distance. The system queries for poses within a distance threshold; if no perfect match exists,
it blends toward the closest pose over a transition duration.

### Phase and Slope Alignment

Phase-aligned blending synchronizes clips to the same phase (e.g., both at midstride). Walking
animations repeat cyclically; phase tracks position in the cycle. Aligning phases before blending
prevents foot sliding or stuttering. Slope blending matches surface angles: if the character walks
uphill, choose an animation leaning forward; downhill chooses a backward lean. Height slope
(vertical movement) and direction slope (horizontal angle) drive selection.

### Directional Blending

Locomotion blends four directions: forward walk, backward walk, left strafe, right strafe. A
movement direction vector selects which animations to blend. Moving forward picks forward walk;
moving at 45° angle blends forward and left animations at 50–50.

### Complex Transitions

Parkour transitions blend between running, vaulting, sliding, and climbing. Rather than fixed state
machine paths, motion matching finds the best frame in the database and blends to it, enabling
dynamic transitions. A character at the ledge near a vault opportunity can smoothly transition to
vaulting without explicit vault state.

## How it fits

- See [skeletal-and-states.md](./skeletal-and-states.md) for animation clip storage and playback.
- See [character-and-first-person.md](./character-and-first-person.md) for character-specific
  animation blending.
- Integrates with [../game-framework/camera-and-controls.md](../game-framework/camera-and-controls.md)
  for locomotion input.
