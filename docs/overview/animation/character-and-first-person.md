# Character and First-Person

Character animation rigs, first-person view models, and combined animation systems.

## What it covers

- Third-person character rigs: full-body skeletons with animation.
- First-person arms and weapons: separate skeletal meshes for first-person perspective.
- Character-specific animation variants: male, female, tall, short rigs.
- Head look-at: procedural head rotation toward a target (player, camera).
- Facial animation: blend shape targets for expressions and mouth shapes.
- Lip-sync: synchronizing mouth shapes to dialogue audio.
- Hands and finger animation: articulated hands with IK-driven grasping.
- Weapon-specific animations: unique hold and firing animations per weapon.
- Emotes and animations: character performed expressions, dances, gestures.
- Locomotion layering: combining directional movement with upper-body actions.

## Concepts

### Character Rigs and Variants

Character rigs define skeleton structure and animation. A humanoid rig has spine, arms, legs, head
with standard bone names. Game characters vary in body shape: tall, short, stocky, lean. The engine
supports multiple rig variants; different body types share animation clips via retargeting. IK
solvers adjust limbs to fit different proportions without re-recording animations.

### First-Person Arm Models

First-person view renders a separate arm skeleton from the first-person perspective. Third-person
animation drives movement; the first-person arm mesh mirrors hand positions and rotations. First-person
weapons attach to hand bones. When third-person animation plays reload, first-person arms perform
matching reload motion.

### Head Look-At and Facial Animation

Procedural head look-at rotates the head toward a target (player looking at NPC, NPC looking at
player). Head bones rotate to point a forward vector at the target. Neck and spine also contribute,
creating natural-looking gaze. Facial animation uses blend shapes (morph targets): neutral face is
the base, and blend shapes add smiles, frowns, surprise. Blending multiple shapes produces complex
expressions.

### Lip-Sync

Lip-sync extracts phoneme timings from dialogue audio. Each phoneme (sound) maps to viseme (mouth
shape). The audio engine sends viseme events timed to dialogue; the character controller blends mouth
shapes (blend shapes) to match. This creates the illusion of synchronized speech.

### Hands and Grasping

Hand rigs articulate all fingers. IK solvers position hands to grasp objects: given a grab target
(sword, button), hand IK reaches and curls fingers around it. Grasp targets define approach vector,
grasp point, and finger curling degree. Hand animations layer procedurally over grasp IK, adding
detail (finger wiggle, thumb hook) on top of procedural grasp.

### Locomotion and Upper-Body Layering

Locomotion animations handle leg movement; upper-body actions (aiming, reloading, attacking) layer
on top. Additive blending combines base locomotion with upper-body offset. A character can walk
forward and aim sideways simultaneously, with legs going forward and upper body twisted. Layering
enables thousands of animation combinations without explicit keyframing every one.

## How it fits

- See [skeletal-and-states.md](./skeletal-and-states.md) for skeleton and animation clip
  foundations.
- See [procedural-and-physical.md](./procedural-and-physical.md) for IK solvers and procedural
  motion.
- See [../audio/music-and-voice.md](../audio/music-and-voice.md) for lip-sync timing and voice
  playback.
