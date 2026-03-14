# 9.1 — Skeletal Animation

## GPU Skinning

### F-9.1.1 GPU Compute Skinning

Transforms vertices by weighted bone matrices on the GPU using compute shaders. Supports both
linear blend skinning and dual-quaternion skinning to eliminate candy-wrapping artifacts at
twist joints. Bone palettes are uploaded as world-space matrices each frame.

- **Requirements:** R-9.1.1
- **Dependencies:** None
- **Platform notes:** None

## Animation Clips (GPU Keyframe Evaluation)

### F-9.1.2 GPU Keyframe Evaluation

Evaluates animation curves entirely on the GPU using Hermite interpolation. Per-joint
translation, rotation, and scale tracks are sampled at arbitrary time offsets in a single
compute dispatch. Supports looped, clamped, and ping-pong playback modes.

- **Requirements:** R-9.1.2
- **Dependencies:** F-9.1.1
- **Platform notes:** None

## Animation Blending

### F-9.1.3 Animation Blending (Linear and Cubic)

Blends multiple animation clips using linear interpolation or cubic Hermite curves to produce
smooth transitions between poses. Blend weights are computed CPU-side by the state machine and
uploaded as blend descriptors. Supports blending up to 8 simultaneous clips per skeleton.

- **Requirements:** R-9.1.3
- **Dependencies:** F-9.1.2
- **Platform notes:** None

## Animation Layers

### F-9.1.4 Animation Layers and Additive Blending

Applies animations on separate layers with per-bone masks, enabling upper-body combat
animations overlaid on lower-body locomotion. Additive layers encode pose deltas relative to a
reference pose, allowing hit reactions, breathing, and weapon sway to composite on top of any
base animation without authoring per-combination clips.

- **Requirements:** R-9.1.4
- **Dependencies:** F-9.1.3
- **Platform notes:** None

## Instanced Skeletal Animation

### F-9.1.5 Instanced Skeletal Animation

Batch-evaluates animation for thousands of skeleton instances in a single compute dispatch
using an arena buffer. Enables MMO-scale crowds, armies, and city NPCs without per-instance
CPU overhead. Instances sharing the same animation clip are grouped for optimal GPU occupancy.

- **Requirements:** R-9.1.5
- **Dependencies:** F-9.1.1, F-9.1.2
- **Platform notes:** None

## Root Motion

### F-9.1.6 Root Motion Extraction

Extracts translation and rotation deltas from the root bone of animation clips and applies
them to the character's world transform via the physics system. Separates authored locomotion
from gameplay movement, enabling animation-driven traversal for dodges, vaults, and mount
animations while maintaining capsule-based collision.

- **Requirements:** R-9.1.6
- **Dependencies:** F-9.1.2
- **Platform notes:** None

## Animation Compression

### F-9.1.7 Animation Compression

Compresses animation clip data using variable-rate quantization per track: rotation tracks use
smallest-three quaternion encoding, translation and scale tracks use range-reduced fixed-point
values. Achieves 10:1+ compression ratios on typical humanoid clips, critical for streaming
thousands of unique emote, combat, and mount animations in an MMO world.

- **Requirements:** R-9.1.7
- **Dependencies:** F-9.1.2
- **Platform notes:** None
