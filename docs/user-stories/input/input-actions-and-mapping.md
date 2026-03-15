# User Stories -- 6.2 Input Actions & Mapping

## US-6.2.1 Define Device-Independent Actions
**As an** input designer, **I want** to define typed input actions (boolean, axis 1D/2D/3D)
that fire identically from keyboard, gamepad, or touch, **so that** gameplay code never
branches on device type and a single action works across all input methods.

## US-6.2.2 Layer Control Schemes by Game State
**As an** input designer, **I want** named mapping contexts on a priority stack (OnFoot,
Mounted, UIMenu) where higher-priority contexts consume inputs first, **so that** opening
inventory correctly captures Escape without leaking it to combat controls.

## US-6.2.3 Tune Stick Feel Per Controller
**As a** player, **I want** configurable modifier chains (dead zones, response curves,
sensitivity) applied to raw input before it reaches actions, **so that** I can fine-tune
stick feel for my specific controller and personal preference.

## US-6.2.4 Use Taps, Holds, Chords, and Combos
**As an** input designer, **I want** trigger conditions (pressed, hold, tap, pulse, chord,
combo) on each action, **so that** tap triggers instant casts, hold triggers channeled
abilities, Shift+1 accesses extended action bars, and ordered sequences chain abilities.

## US-6.2.5 Rebind Any Key at Runtime
**As a** player, **I want** to rebind any action to any compatible input at runtime with
conflict detection, **so that** I can freely customize my layout across dozens of action
bar slots and have bindings persist across sessions.

## US-6.2.6 See Correct Button Icons Instantly
**As a** player, **I want** all button prompts and tutorial icons to update immediately
when I switch between keyboard, Xbox gamepad, and DualSense, **so that** I always see the
correct platform-native button glyphs for my active input device.

## US-6.2.7 Record and Replay Input for Testing
**As an** accessibility tester, **I want** to record all input events to a binary stream
and replay them deterministically with speed control, **so that** I can automate regression
testing, create tutorial ghosts, and reproduce bugs from input recordings.

## US-6.2.8 Execute Fighting-Game Combos
**As a** player, **I want** the engine to recognize directional motion sequences (quarter-
circle, dragon punch), branching combo trees, and multi-button presses with leniency
windows, **so that** action combat feels responsive with data-driven combo definitions that
work identically across stick, D-pad, and WASD.

## US-6.2.9 Queue Inputs During Ability Animations
**As a** player, **I want** my next input buffered during the current ability's active
frames and executed as soon as a cancel window opens, **so that** I can chain actions
fluidly without frame-perfect timing.

## US-6.2.10 Aim Precisely on Gamepad
**As a** player, **I want** aim assist (magnetism, friction, snap) and input smoothing on
gamepad that are configurable per weapon type and disabled in competitive modes, **so that**
gamepad aiming feels natural without unfair advantage in PvP.

## US-6.2.11 Navigate All UI with Controller
**As an** accessibility tester, **I want** full UI navigation via gamepad using directional
focus, virtual cursor, and context-sensitive action mapping, **so that** every screen is
accessible without a mouse and console certification requirements are met.
