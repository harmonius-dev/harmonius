# R-9.6 -- First-Person Animation Requirements

## Camera and Weapon

1. **R-9.6.1** -- The engine **SHALL** provide a first-person camera system with head-bob
   synchronized to locomotion, landing impact proportional to fall distance, procedural lean/peek,
   slope tilt, and a separate viewmodel FOV.
   - **Rationale:** Spring-damper camera effects create physical immersion; separate viewmodel FOV
     prevents weapon distortion at wide world FOVs.
   - **Verification:** Walk and verify head-bob frequency matches locomotion speed. Fall and verify
     impact magnitude scales with distance. Set world FOV to 120 and verify weapon is not distorted.

2. **R-9.6.2** -- The engine **SHALL** apply weapon viewmodel sway, locomotion bob, and sprint tilt
   via spring physics with per-weapon data-driven parameters.
   - **Rationale:** Per-weapon spring data enables distinct weapon feel during movement without code
     changes.
   - **Verification:** Compare two weapons and verify distinct sway and bob characteristics. Verify
     sprint tilt activates at sprint speed and deactivates on stop.

3. **R-9.6.3** -- The engine **SHALL** apply non-repetitive recoil from pattern data via spring
   forces with recovery, smooth ADS transitions, and render-to-texture scope magnification.
   - **Rationale:** Pattern-based recoil creates learnable weapon mastery; ADS transitions and scope
     rendering are core FPS mechanics.
   - **Verification:** Fire a weapon and verify recoil follows the pattern data. Verify ADS
     interpolates smoothly between hip and sight positions. Verify scope RTT shows magnified scene.

4. **R-9.6.4** -- The engine **SHALL** support weapon equip/holster sequences, inspection
   animations, and dual wielding with independent sway, bob, fire, and reload per hand.
   - **Rationale:** Weapon handling animations add physical presence; dual wielding doubles
     expressiveness for action-oriented games.
   - **Verification:** Equip a weapon and verify the draw animation plays. Dual wield and verify
     independent fire and reload per hand. Verify alternating and simultaneous fire modes.
