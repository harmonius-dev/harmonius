# Camera and Controls

Camera systems, control schemes, and player input handling.

## What it covers

- Third-person camera: orbiting behind character.
- First-person camera: weapon sight and head tracking.
- Dynamic camera: adjusting distance and angle based on environment.
- Camera constraints: avoiding obstacles and terrain.
- Look-ahead and prediction: camera leading character movement.
- Controller vibration: haptic feedback on camera impact.
- Control schemes: mouse/keyboard, gamepad, motion controls.
- Aim assist: helping players target enemies.
- Sprint and movement: speed modulation via input.
- Look sensitivity and acceleration: customizable input feel.

## Concepts

### Third-Person Camera

Third-person camera orbits around the character at a fixed distance and height offset. Input (mouse
movement, right stick) rotates the camera around character. The camera looks slightly downward,
showing character and surrounding area. Collision detection prevents camera clipping into walls; if
wall blocks direct path, camera zooms in or shifts aside.

### First-Person Camera

First-person camera is positioned at character head, looking forward. Input rotates the camera
(look around). Weapon meshes render in front of camera (first-person arms). No character mesh
renders (player never sees self in first-person). Look-down shows weapon sights or hand models.

### Dynamic Adjustment

Camera adjusts based on gameplay: zooming in for tight spaces (narrow corridors), zooming out for
open areas. Look-ahead adjusts camera position ahead of character direction during sprint, showing
ground ahead. Vertical adjustment based on terrain: climbing slope tips camera down to see where
walking; descending slopes tip camera up.

### Control Schemes

Keyboard/mouse: movement keys (WASD) move character; mouse movement aims camera. Gamepad: left
stick moves character; right stick aims camera. Motion controls: device tilt controls camera
direction. Players customize sensitivity (how far mouse movement rotates camera) and acceleration
(lag before reaching max rotation speed).

### Aiming and Feedback

Aim assist helps target enemies: slight aim correction when crosshair is near enemy. Sprint input
increases move speed. Ability input triggers skills. Input latency is minimized: input buffer
stores recent inputs for frame-perfect replay. Controller vibration provides feedback: landing
impact, firing weapon, hitting enemy.

## How it fits

- See [save-and-persistence.md](./save-and-persistence.md) for saving camera settings and
  preferences.
- See [../input/devices-and-actions.md](../input/devices-and-actions.md) for input mapping and
  customization.
- See [../rendering/pipeline.md](../rendering/pipeline.md) for view setup and multi-view
  rendering.
