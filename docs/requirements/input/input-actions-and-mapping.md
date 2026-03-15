# R-6.2 — Input Actions & Mapping User Stories

## F-6.2.1 Typed Input Actions

## US-6.2.1.1 Define Typed Actions

**As a** designer (P-5), **I want to** define input actions as boolean, axis 1D, axis 2D, or
axis 3D types in the editor, **so that** gameplay actions are device-independent.

## US-6.2.1.2 Verify Same Action from Multiple Devices

**As an** engine tester (P-27), **I want to** bind a boolean action to keyboard, gamepad, and
touch and verify all produce the same value, **so that** device independence is confirmed.

## US-6.2.1.3 Verify Type Mismatch Error

**As an** engine tester (P-27), **I want to** bind an axis 2D binding to a boolean action and
verify a load-time error, **so that** type safety is enforced.

## US-6.2.1.4 Implement Typed Action System

**As an** engine developer (P-26), **I want to** implement the strongly typed action system
with boolean, axis 1D, 2D, and 3D variants, **so that** gameplay logic is decoupled from
device bindings.

## US-6.2.1.5 Play with Any Input Device

**As a** player (P-23), **I want** the same gameplay actions to work from keyboard, gamepad,
or touch without configuration, **so that** any input device works out of the box.

---

## F-6.2.2 Input Mapping Contexts with Priority Stacking

## US-6.2.2.1 Create Mapping Contexts in Editor

**As a** designer (P-5), **I want to** create named mapping contexts (OnFoot, Mounted, UIMenu)
with input-to-action bindings in the editor, **so that** each gameplay mode has its own
control scheme.

## US-6.2.2.2 Configure Context Priority in Editor

**As a** designer (P-5), **I want to** set context priority ordering in the editor, **so that**
higher-priority contexts consume inputs first.

## US-6.2.2.3 Verify Priority-Based Input Consumption

**As an** engine tester (P-27), **I want to** push Combat and UIMenu contexts where both bind
Escape, activate UIMenu at higher priority, and assert only UIMenu fires, **so that** priority
consumption is correct.

## US-6.2.2.4 Verify Input Pass-Through

**As an** engine tester (P-27), **I want to** verify that inputs not bound in the top context
pass through to lower contexts, **so that** non-consumed inputs reach the correct handler.

## US-6.2.2.5 Implement Context Stack

**As an** engine developer (P-26), **I want to** implement the priority-ordered context stack
where higher-priority contexts consume matching inputs first, **so that** modal control
schemes are supported.

## US-6.2.2.6 Open Inventory Without Triggering Combat Actions

**As a** player (P-23), **I want** opening the inventory to prevent combat actions from
triggering, **so that** UI and gameplay controls do not conflict.

---

## F-6.2.3 Input Value Modifiers

## US-6.2.3.1 Configure Dead Zones in Editor

**As a** designer (P-5), **I want to** set axial and radial dead zones per stick binding in
the editor, **so that** stick drift is eliminated.

## US-6.2.3.2 Configure Response Curves in Editor

**As a** designer (P-5), **I want to** set response curves (linear, exponential, S-curve) per
input binding, **so that** stick feel is tuned per use case.

## US-6.2.3.3 Configure Sensitivity Multiplier

**As a** designer (P-5), **I want to** set a scalar sensitivity multiplier per binding,
**so that** camera and movement sensitivity is adjustable.

## US-6.2.3.4 Verify Radial Dead Zone

**As an** engine tester (P-27), **I want to** apply a 0.15 radial dead zone to an axis 2D
input of magnitude 0.10 and assert output is (0, 0), and at 0.50 verify remapped output,
**so that** dead zone math is correct.

## US-6.2.3.5 Verify Modifier Chain Composition

**As an** engine tester (P-27), **I want to** chain dead zone, exponential curve, and scalar
2.0 and verify the output matches mathematical composition, **so that** chain ordering is
correct.

## US-6.2.3.6 Implement Modifier Chain Pipeline

**As an** engine developer (P-26), **I want to** implement the composable modifier pipeline
with dead zone, response curve, swizzle, negate, and scalar modifiers, **so that** raw inputs
are processed before reaching actions.

## US-6.2.3.7 Test Modifier Presets on Different Controllers

**As a** QA tester (P-19), **I want to** verify dead zone presets work correctly on Xbox,
DualSense, and Switch Pro controllers, **so that** stick feel is good on all gamepads.

## US-6.2.3.8 Adjust Sensitivity in Settings

**As a** player (P-23), **I want to** adjust camera sensitivity and dead zones in the
settings menu, **so that** controls match my preference.

---

## F-6.2.4 Input Trigger Conditions

## US-6.2.4.1 Configure Trigger Types in Editor

**As a** designer (P-5), **I want to** set trigger conditions (pressed, released, hold, tap,
pulse, chord, combo) on actions in the editor, **so that** each ability has the right
activation pattern.

## US-6.2.4.2 Verify Pressed Trigger Fires Once

**As an** engine tester (P-27), **I want to** verify pressed fires on the key-down frame only,
**so that** single-frame activation is correct.

## US-6.2.4.3 Verify Hold Trigger Fires After Duration

**As an** engine tester (P-27), **I want to** verify hold fires after exactly the configured
duration, **so that** hold timing is accurate.

## US-6.2.4.4 Verify Tap Trigger Requires Quick Release

**As an** engine tester (P-27), **I want to** verify tap fires only if release occurs within
the threshold, **so that** tap vs hold disambiguation works.

## US-6.2.4.5 Verify Chord Trigger Requires Simultaneous Input

**As an** engine tester (P-27), **I want to** verify chord fires only when all required inputs
are active simultaneously, **so that** modifier key combinations work.

## US-6.2.4.6 Verify Combo Trigger Requires Ordered Sequence

**As an** engine tester (P-27), **I want to** verify combo fires only when inputs arrive in
order within the time window, **so that** ability chains work.

## US-6.2.4.7 Implement Trigger Condition System

**As an** engine developer (P-26), **I want to** implement all trigger condition types
(pressed, released, hold, tap, pulse, chord, combo), **so that** diverse activation patterns
are supported.

## US-6.2.4.8 Use Hold for Channeled Abilities

**As a** player (P-23), **I want** holding a button to channel an ability and releasing to
stop, **so that** channeled spells and abilities feel intuitive.

---

## F-6.2.5 Runtime Key Rebinding with Conflict Detection

## US-6.2.5.1 Rebind Keys in Settings UI

**As a** designer (P-5), **I want** the settings UI to provide a rebinding interface where
players can remap any action, **so that** input customization is available.

## US-6.2.5.2 Verify Conflict Detection

**As an** engine tester (P-27), **I want to** rebind two actions to the same key and verify a
conflict dialog appears with swap/unbind options, **so that** conflicts are surfaced.

## US-6.2.5.3 Verify Rebinding Persistence

**As an** engine tester (P-27), **I want to** rebind keys, restart the session, and verify
rebindings are restored, **so that** persistence works.

## US-6.2.5.4 Verify Platform-Reserved Key Rejection

**As an** engine tester (P-27), **I want to** attempt to rebind to the PS button and verify
it is rejected, **so that** platform-reserved keys are protected.

## US-6.2.5.5 Implement Rebinding System

**As an** engine developer (P-26), **I want to** implement runtime rebinding with conflict
detection, swap/unbind resolution, and persistent storage, **so that** players can customize
controls.

## US-6.2.5.6 Test Rebinding Across All Platforms

**As a** QA tester (P-19), **I want to** test rebinding on all platforms and verify platform-
specific reserved keys are handled, **so that** rebinding is safe everywhere.

## US-6.2.5.7 Customize Controls to My Preference

**As a** player (P-23), **I want to** rebind any action to any key or button with conflict
warnings, **so that** controls match my preferences and accessibility needs.

---

## F-6.2.6 Platform-Aware Button Glyph Resolution

## US-6.2.6.1 Configure Glyph Atlases in Editor

**As a** designer (P-5), **I want to** set up platform-specific button glyph atlases (Xbox,
PlayStation, Switch, keyboard) in the editor, **so that** button icons are correct per
platform.

## US-6.2.6.2 Verify Instant Glyph Update on Device Switch

**As an** engine tester (P-27), **I want to** switch from keyboard to Xbox gamepad and verify
displayed glyphs update to Xbox A button within one frame, **so that** glyph switching is
instant.

## US-6.2.6.3 Verify Cross-Controller Glyph Accuracy

**As an** engine tester (P-27), **I want to** switch to DualSense and verify Cross icon, then
to keyboard and verify the key label, **so that** every controller shows correct icons.

## US-6.2.6.4 Implement Glyph Resolution System

**As an** engine developer (P-26), **I want to** implement the glyph resolver that maps actions
to bindings to platform icons, updating reactively on device change, **so that** button prompts
are always correct.

## US-6.2.6.5 Test Glyph Accuracy for Console Certification

**As a** QA tester (P-19), **I want to** verify correct platform-native button icons display
at all times on each console, **so that** certification requirements are met.

## US-6.2.6.6 See Correct Button Icons for My Controller

**As a** player (P-23), **I want** button prompts to show the correct icons for my current
controller, **so that** I know which buttons to press.

---

## F-6.2.7 Input Recording and Playback

## US-6.2.7.1 Configure Input Recording in Editor

**As a** designer (P-5), **I want to** enable input recording and set storage preferences in
the editor, **so that** recordings are available for testing.

## US-6.2.7.2 Verify Deterministic Playback

**As an** engine tester (P-27), **I want to** record a 30-second input sequence, play it back,
and verify identical game state, **so that** deterministic replay is confirmed.

## US-6.2.7.3 Verify Speed Control

**As an** engine tester (P-27), **I want to** verify playback at 0.5x and 2.0x speed produces
the same final game state, **so that** speed control is deterministic.

## US-6.2.7.4 Verify Frame Stepping

**As an** engine tester (P-27), **I want to** verify frame-stepping advances exactly one frame
per step, **so that** frame-level debugging works.

## US-6.2.7.5 Implement Input Recording System

**As an** engine developer (P-26), **I want to** implement binary input recording and
deterministic playback with speed control and frame stepping, **so that** automated testing
and bug reproduction are supported.

## US-6.2.7.6 Test Replay for Regression Testing

**As a** QA tester (P-19), **I want to** use recorded input sequences for automated regression
testing, **so that** test coverage is repeatable.

---

## F-6.2.8 Combo Input Trees and Directional Sequences

## US-6.2.8.1 Author Combo Graphs in Editor

**As a** designer (P-5), **I want to** author combo input trees as visual graph assets in the
editor with directional sequences and timing windows, **so that** fighting-game combos are
data-driven.

## US-6.2.8.2 Verify Quarter-Circle Combo Recognition

**As an** engine tester (P-27), **I want to** input a quarter-circle-forward + button within
200ms and assert the combo fires, **so that** directional sequence recognition works.

## US-6.2.8.3 Verify Combo Timeout Reset

**As an** engine tester (P-27), **I want to** input the correct direction but exceed the timing
window and assert the combo resets to root, **so that** timing enforcement is correct.

## US-6.2.8.4 Verify Cross-Device Directional Normalization

**As an** engine tester (P-27), **I want to** verify identical combo behavior with stick,
D-pad, and WASD inputs, **so that** directional normalization works across devices.

## US-6.2.8.5 Implement Combo Recognition System

**As an** engine developer (P-26), **I want to** implement the combo input tree evaluator with
directional motion sequences, branching states, leniency windows, and input buffering,
**so that** fighting-game-grade combos are supported.

## US-6.2.8.6 Execute Combo Moves

**As a** player (P-23), **I want to** execute directional combo moves reliably using stick,
D-pad, or keyboard, **so that** action combat feels responsive.

---

## F-6.2.9 Input Buffering and Ability Cancel Windows

## US-6.2.9.1 Configure Cancel Windows Per Ability

**As a** designer (P-5), **I want to** define cancel window frame ranges and permitted action
categories per ability in the editor, **so that** responsiveness is tuned per move.

## US-6.2.9.2 Configure Buffer Duration

**As a** designer (P-5), **I want to** set the input buffer duration (100-200ms) in the
project settings, **so that** buffer timing is adjustable.

## US-6.2.9.3 Verify Buffered Input Executes at Cancel Window

**As an** engine tester (P-27), **I want to** trigger an ability (500ms, cancel at 300-500ms),
press dodge at 250ms, and assert dodge executes at 300ms, **so that** buffering respects cancel
windows.

## US-6.2.9.4 Verify Priority Resolution

**As an** engine tester (P-27), **I want to** press dodge and attack during the same buffer
window and assert dodge executes (higher priority), **so that** priority rules work.

## US-6.2.9.5 Verify Buffer Expiration

**As an** engine tester (P-27), **I want to** verify no buffered input executes after the
buffer duration expires, **so that** stale inputs are discarded.

## US-6.2.9.6 Implement Input Buffer System

**As an** engine developer (P-26), **I want to** implement the input buffer that stores the
most recent input during active frames and executes it at the next cancel window, with priority
resolution, **so that** responsive action combat is supported.

## US-6.2.9.7 Queue Next Move During Current Ability

**As a** player (P-23), **I want to** press my next attack during the current attack's recovery
and have it execute immediately when possible, **so that** combat flows smoothly.

---

## F-6.2.10 Input Smoothing, Acceleration, and Aim Assist

## US-6.2.10.1 Configure Input Smoothing

**As a** designer (P-5), **I want to** set smoothing time constants per input action in the
editor, **so that** worn controller jitter is eliminated.

## US-6.2.10.2 Configure Acceleration Curves

**As a** designer (P-5), **I want to** set acceleration ramp-up time, max multiplier, and
decay rate per input, **so that** camera control feels natural.

## US-6.2.10.3 Configure Aim Assist Parameters

**As a** designer (P-5), **I want to** configure aim assist magnetism, friction, and snap
strengths per weapon type and game mode, **so that** gamepad aiming is competitive.

## US-6.2.10.4 Verify Smoothing Reduces Jitter

**As an** engine tester (P-27), **I want to** apply 50ms smoothing to jittery input and assert
variance is reduced by 80% without exceeding 16ms added latency, **so that** smoothing is
effective without being sluggish.

## US-6.2.10.5 Verify Acceleration Scaling

**As an** engine tester (P-27), **I want to** verify output at max input velocity equals 2x
base sensitivity with acceleration at 2x multiplier, **so that** acceleration scaling is
correct.

## US-6.2.10.6 Verify Aim Assist Target Magnetism

**As an** engine tester (P-27), **I want to** enable magnetism and verify crosshair deflects
toward a valid target and does not deflect when no target is present, **so that** magnetism
is target-aware.

## US-6.2.10.7 Implement Advanced Input Processing

**As an** engine developer (P-26), **I want to** implement smoothing, acceleration, and aim
assist as composable stages in the modifier chain, **so that** advanced processing is
available per-action.

## US-6.2.10.8 Test Aim Assist Across Game Modes

**As a** QA tester (P-19), **I want to** verify aim assist is disabled in competitive PvP and
enabled in PvE, **so that** game mode settings are respected.

## US-6.2.10.9 Aim Accurately with a Gamepad

**As a** player (P-23), **I want** aim assist to help me aim with a gamepad without feeling
unfair, **so that** gamepad aiming is competitive with mouse.

---

## F-6.2.11 Controller-Driven UI Interaction

## US-6.2.11.1 Configure UI Navigation Modes in Editor

**As a** designer (P-5), **I want to** select navigation modes (directional focus, virtual
cursor, action mapping) per UI screen in the editor, **so that** each screen has appropriate
controller navigation.

## US-6.2.11.2 Verify Full UI Navigability via Controller

**As an** engine tester (P-27), **I want to** navigate through all UI screens using only a
gamepad and assert every widget is reachable, **so that** full controller navigability is
confirmed.

## US-6.2.11.3 Verify Virtual Cursor Mode

**As an** engine tester (P-27), **I want to** verify virtual cursor moves proportionally to
stick input, **so that** complex UIs (maps, skill trees) are navigable.

## US-6.2.11.4 Verify Seamless Input Switch

**As an** engine tester (P-27), **I want to** switch from gamepad to mouse mid-interaction and
verify no lost focus state, **so that** mixed input is seamless.

## US-6.2.11.5 Verify Focus-Based Tooltips

**As an** engine tester (P-27), **I want to** verify tooltips appear on focus (not hover) when
gamepad is active, **so that** hover-dependent behaviors adapt to controller input.

## US-6.2.11.6 Implement Controller UI Navigation

**As an** engine developer (P-26), **I want to** implement directional focus, virtual cursor,
and action mapping for controller-driven UI with focus-based tooltip activation, **so that**
full UI navigability is available via controller.

## US-6.2.11.7 Test Console UI Navigability for Certification

**As a** QA tester (P-19), **I want to** verify every UI screen is fully navigable via
controller on each console, **so that** platform certification requirements are met.

## US-6.2.11.8 Navigate All Menus with Controller

**As a** player (P-23), **I want to** navigate all menus and UI screens using only my
controller, **so that** I never need a mouse.

---

## Non-Functional Requirements

### R-6.2.NF1 Action Evaluation Throughput

The engine **SHALL** evaluate all active input actions (modifier chains, trigger conditions,
context priority resolution) within 0.2 ms per frame for up to 128 simultaneously active
actions across all mapping contexts.

- **Derived from:** F-6.2.1, F-6.2.3
- **Rationale:** The action system runs on the main thread every frame. Its cost must be
  negligible relative to the frame budget.
- **Verification:** Benchmark: configure 128 actions across 8 stacked contexts with 4-stage
  modifier chains. Assert total evaluation time is below 0.2 ms.

### R-6.2.NF2 Rebinding Persistence Latency

The engine **SHALL** persist rebinding changes to storage within 100 ms and restore all
rebindings within 50 ms during startup.

- **Derived from:** F-6.2.5
- **Rationale:** Players expect rebinding to survive crashes. Fast restore prevents delay.
- **Verification:** Integration test: rebind 20 actions and measure write time. Assert all
  writes complete within 100 ms. Restart and assert restoration within 50 ms.
