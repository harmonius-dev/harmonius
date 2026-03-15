# User Stories — 6.2 Input Actions & Mapping

## US-6.2.1.1 Define Boolean Input Actions
**As a** designer (P-5), **I want to** define boolean actions (pressed/not pressed), **so
that** simple triggers like "Jump" and "Dodge" work.

## US-6.2.1.2 Define Axis 1D Actions
**As a** designer (P-5), **I want to** define axis 1D actions (scalar float), **so that**
throttle and zoom inputs produce continuous values.

## US-6.2.1.3 Define Axis 2D Actions
**As a** designer (P-5), **I want to** define axis 2D actions (vector2), **so that** movement
and camera look produce directional input.

## US-6.2.1.4 Implement Typed Action System
**As an** engine developer (P-26), **I want to** implement strongly typed actions decoupling
gameplay from device inputs, **so that** the same action fires from any device.

## US-6.2.1.5 Use Same Controls on Keyboard and Gamepad
**As a** player (P-23), **I want** the same game actions to work on keyboard, gamepad, and
touch, **so that** I can use any device.

## US-6.2.1.6 Define Axis 3D Actions
**As an** engine developer (P-26), **I want** axis 3D actions (vector3), **so that** VR motion
and spatial input produce 3-axis values.

## US-6.2.1.7 Use Touch Virtual Joystick on Mobile
**As a** player (P-23), **I want** touch virtual joystick as the default axis2D source on
mobile, **so that** movement works on touchscreens.

## US-6.2.1.8 Verify Action Types Fire Correctly
**As an** engine tester (P-27), **I want to** verify boolean, axis 1D, 2D, and 3D action
types fire correctly from all input sources, **so that** typed actions work.

## US-6.2.1.9 Configure Default Bindings Per Platform
**As a** designer (P-5), **I want** default action bindings per platform (touch, gamepad,
keyboard), **so that** each platform has appropriate defaults.

## US-6.2.1.10 Test Actions from Multiple Device Types
**As a** QA tester (P-19), **I want to** test each action from keyboard, gamepad, and touch,
**so that** all input paths are validated.

## US-6.2.1.11 Fire Same Action from Any Device
**As an** engine developer (P-26), **I want** the same "Dodge Roll" action to fire from
keyboard, gamepad button, or touch gesture, **so that** input is device-independent.

## US-6.2.1.12 Experience Consistent Controls Across Devices
**As a** player (P-23), **I want** controls to feel consistent regardless of input device,
**so that** switching devices is seamless.

## US-6.2.2.1 Create Named Mapping Contexts
**As a** designer (P-5), **I want to** create named mapping contexts (OnFoot, Mounted, UIMenu),
**so that** input bindings are grouped by mode.

## US-6.2.2.2 Activate Contexts on Priority Stack
**As an** engine developer (P-26), **I want** contexts activated on a priority-ordered stack,
**so that** higher-priority contexts consume inputs first.

## US-6.2.2.3 Switch Controls When Mounting
**As a** player (P-23), **I want** controls to change when mounting a vehicle or horse, **so
that** each mode has appropriate bindings.

## US-6.2.2.4 Implement Context Stack Management
**As an** engine developer (P-26), **I want** context activation, deactivation, and priority
management, **so that** modal overlays work correctly.

## US-6.2.2.5 Ship Touch Contexts for Mobile
**As a** designer (P-5), **I want** touch-specific contexts shipped for mobile, **so that**
touch UI has appropriate bindings.

## US-6.2.2.6 Verify Context Priority Ordering
**As an** engine tester (P-27), **I want to** verify higher-priority contexts consume inputs
before lower ones, **so that** modal overlays work.

## US-6.2.2.7 Stack Inventory Context Over Movement
**As a** player (P-23), **I want** the inventory context to capture Escape before combat,
**so that** closing inventory works correctly.

## US-6.2.2.8 Test Context Switching at Runtime
**As an** engine tester (P-27), **I want to** test rapid context switching, **so that**
activation and deactivation are robust.

## US-6.2.2.9 Configure Contexts in Visual Editor
**As a** designer (P-5), **I want to** configure mapping contexts in the visual editor, **so
that** input modes are visual.

## US-6.2.2.10 Verify Platform Default Contexts
**As a** QA tester (P-19), **I want to** verify each platform ships with appropriate default
contexts, **so that** out-of-box experience is good.

## US-6.2.2.11 Layer UI Context Over Movement
**As an** engine developer (P-26), **I want** UI contexts layered over movement contexts,
**so that** menus consume inputs without affecting gameplay.

## US-6.2.2.12 Switch Contexts Based on Game State
**As a** player (P-23), **I want** controls to automatically adapt when entering menus,
vehicles, or combat, **so that** the right bindings are active.

## US-6.2.3.1 Apply Dead Zone Modifiers
**As a** designer (P-5), **I want** axial and radial dead zone modifiers on stick input, **so
that** stick drift is eliminated.

## US-6.2.3.2 Apply Response Curve Modifiers
**As a** designer (P-5), **I want** linear, exponential, and S-curve response modifiers, **so
that** stick feel is tunable per action.

## US-6.2.3.3 Implement Modifier Chain Pipeline
**As an** engine developer (P-26), **I want** configurable modifier chains applied to raw
input before actions, **so that** processing is composable.

## US-6.2.3.4 Tune Stick Sensitivity in Settings
**As a** player (P-23), **I want** per-player sensitivity options, **so that** I can customize
stick feel to my preference.

## US-6.2.3.5 Apply Swizzle and Negate Modifiers
**As an** engine developer (P-26), **I want** swizzle (remap axes) and negate (invert) modifiers,
**so that** axis remapping is flexible.

## US-6.2.3.6 Apply Scalar Sensitivity Modifier
**As an** engine developer (P-26), **I want** scalar sensitivity multiplier modifier, **so
that** input speed is tunable.

## US-6.2.3.7 Vary Dead Zones by Controller Type
**As a** designer (P-5), **I want** dead zone defaults varying by controller type (Xbox vs
DualSense), **so that** each controller feels right out of box.

## US-6.2.3.8 Verify Modifier Chain Order
**As an** engine tester (P-27), **I want to** verify modifiers apply in the correct order,
**so that** dead zone before response curve is maintained.

## US-6.2.3.9 Experience Smooth Camera Control
**As a** player (P-23), **I want** camera control to feel smooth with no jitter, **so that**
stick input is comfortable.

## US-6.2.3.10 Provide Response Curve Presets
**As a** designer (P-5), **I want** response curve presets for camera aim, vehicle steering,
and menu scrolling, **so that** common cases are handled.

## US-6.2.3.11 Test Modifier Chains with Edge Cases
**As an** engine tester (P-27), **I want to** test modifier chains with extreme input values,
**so that** edge cases produce valid output.

## US-6.2.3.12 Profile Modifier Pipeline Cost
**As a** QA tester (P-19), **I want to** verify modifier pipeline cost is negligible, **so
that** input processing does not affect frame rate.

## US-6.2.4.1 Define Pressed Trigger Condition
**As a** designer (P-5), **I want** a "pressed" trigger condition firing on the frame a key
is pressed, **so that** instant-cast abilities work.

## US-6.2.4.2 Define Hold Trigger Condition
**As a** designer (P-5), **I want** a "hold" trigger condition firing after sustained duration,
**so that** channeled abilities work.

## US-6.2.4.3 Define Tap Trigger Condition
**As a** designer (P-5), **I want** a "tap" trigger (press and release within threshold), **so
that** quick actions are distinguished from holds.

## US-6.2.4.4 Implement Chord Triggers
**As an** engine developer (P-26), **I want** chord triggers (multiple simultaneous inputs),
**so that** Shift+1 extended action bar pages work.

## US-6.2.4.5 Implement Combo Triggers
**As an** engine developer (P-26), **I want** combo triggers (ordered sequence within time
window), **so that** ability chains work.

## US-6.2.4.6 Use Hold for Channeled Abilities
**As a** player (P-23), **I want** holding a button to channel abilities, **so that** sustained
actions feel natural.

## US-6.2.4.7 Implement Pulse Triggers
**As an** engine developer (P-26), **I want** pulse triggers repeating on interval while held,
**so that** auto-fire mechanics work.

## US-6.2.4.8 Use Longer Touch Thresholds on Mobile
**As a** designer (P-5), **I want** longer tap/hold thresholds on mobile, **so that** finger
imprecision is accommodated.

## US-6.2.4.9 Limit Chords on Touch
**As an** engine tester (P-27), **I want to** verify chord triggers cap at 2 simultaneous on
touch, **so that** mobile limitations are respected.

## US-6.2.4.10 Define Released Trigger
**As an** engine developer (P-26), **I want** "released" triggers firing on key-up, **so that**
bow release and charge attacks work.

## US-6.2.4.11 Test All Trigger Types
**As an** engine tester (P-27), **I want to** test all trigger types (pressed, released, hold,
tap, pulse, chord, combo), **so that** all conditions work.

## US-6.2.4.12 Execute Combo Ability Chains
**As a** player (P-23), **I want** combo inputs to chain abilities, **so that** skilled input
produces powerful moves.

## US-6.2.5.1 Rebind Actions at Runtime
**As a** player (P-23), **I want to** rebind any action to any compatible input at runtime,
**so that** controls are fully customizable.

## US-6.2.5.2 Detect Binding Conflicts
**As an** engine developer (P-26), **I want** binding conflict detection within overlapping
mapping contexts, **so that** duplicate bindings are flagged.

## US-6.2.5.3 Present Conflict Resolution Options
**As an** engine developer (P-26), **I want** conflict resolution options (swap, unbind, cancel)
presented to the player, **so that** conflicts are resolved cleanly.

## US-6.2.5.4 Serialize Rebindings to Persistent Storage
**As an** engine developer (P-26), **I want** rebindings serialized and restored on session
start, **so that** customizations persist.

## US-6.2.5.5 Customize Dozens of Action Bar Bindings
**As a** player (P-23), **I want to** customize dozens of action bar bindings, **so that** my
hotbar layout matches my preferences.

## US-6.2.5.6 Respect Platform Reserved Keys
**As an** engine developer (P-26), **I want** rebinding to respect platform reserved keys (PS
button, Guide button), **so that** reserved inputs are not stolen.

## US-6.2.5.7 Verify Conflict Detection Across Contexts
**As an** engine tester (P-27), **I want to** verify conflict detection works across
overlapping contexts, **so that** only real conflicts are flagged.

## US-6.2.5.8 Provide Platform-Specific Defaults
**As a** designer (P-5), **I want** platform-specific default bindings, **so that** Xbox and
PlayStation use correct button labels.

## US-6.2.5.9 Test Rebinding Persistence Across Sessions
**As an** engine tester (P-27), **I want to** verify rebindings persist across game sessions,
**so that** customizations are not lost.

## US-6.2.5.10 Reset Bindings to Defaults
**As a** player (P-23), **I want to** reset bindings to defaults, **so that** I can undo
customizations.

## US-6.2.5.11 Avoid False Positives Across Non-Overlapping Contexts
**As an** engine developer (P-26), **I want** conflict detection to account for context
priority, **so that** non-overlapping contexts do not trigger false positives.

## US-6.2.5.12 Test Rebinding with Modifier Keys
**As a** QA tester (P-19), **I want to** test rebinding with modifier keys (Shift, Ctrl, Alt),
**so that** modifier combinations are supported.

## US-6.2.6.1 Resolve Actions to Platform Button Icons
**As an** engine developer (P-26), **I want** input actions automatically resolved to
platform-specific button icons, **so that** UI prompts show correct glyphs.

## US-6.2.6.2 Update Glyphs When Device Changes
**As an** engine developer (P-26), **I want** displayed glyphs to update instantly when the
active device changes, **so that** prompts always match the current input.

## US-6.2.6.3 See Correct Button Icons for My Controller
**As a** player (P-23), **I want** button prompts showing my controller's icons (Xbox A,
PlayStation Cross), **so that** UI guidance is correct.

## US-6.2.6.4 Map Actions to Platform Icon Atlas
**As an** engine developer (P-26), **I want** each action mapped to the bound input, then to
the platform icon atlas, **so that** glyph resolution is two-step.

## US-6.2.6.5 Support Swappable Icon Packs
**As a** designer (P-5), **I want** glyph atlases as swappable assets, **so that** custom
controller icon packs are possible.

## US-6.2.6.6 Verify Glyph Resolution Per Controller Type
**As an** engine tester (P-27), **I want to** verify glyph resolution for Xbox, PlayStation,
Switch, and keyboard, **so that** all controller types show correct icons.

## US-6.2.6.7 Bind Glyph Images Reactively to Device
**As an** engine developer (P-26), **I want** UI widgets to bind glyph images reactively to
the active device, **so that** updates are automatic.

## US-6.2.6.8 Show Keyboard Keys When Using Keyboard
**As a** player (P-23), **I want** keyboard key names shown when using keyboard, **so that**
prompts match my input device.

## US-6.2.6.9 Test Device Switch Glyph Update
**As an** engine tester (P-27), **I want to** test switching between keyboard and gamepad and
verify glyph updates, **so that** transitions are instant.

## US-6.2.6.10 Meet Console Certification for Icons
**As a** QA tester (P-19), **I want to** verify correct platform-native button icons at all
times, **so that** console certification requirements are met.

## US-6.2.6.11 Configure Icon Pack in Settings
**As a** designer (P-5), **I want** icon pack selectable in settings, **so that** players
choose their preferred glyph style.

## US-6.2.6.12 See VR Controller Diagrams
**As a** player (P-23), **I want** VR controller diagrams in prompts when using VR, **so
that** VR controls are clearly communicated.

## US-6.2.7.1 Record All Input Events to Stream
**As an** engine developer (P-26), **I want** all input events recorded to a binary stream
with timestamps, **so that** input replay is possible.

## US-6.2.7.2 Replay Recorded Input Deterministically
**As an** engine developer (P-26), **I want** recorded input replayed deterministically, **so
that** game state can be reproduced.

## US-6.2.7.3 Support Speed Control on Playback
**As an** engine developer (P-26), **I want** playback speed control (slow-mo, fast-forward,
frame-step), **so that** debugging uses replay analysis.

## US-6.2.7.4 Attach Input Recordings to Bug Reports
**As a** QA tester (P-19), **I want to** attach input recordings to bug reports, **so that**
issues are reproducible.

## US-6.2.7.5 Use Recordings for Automated Testing
**As an** engine tester (P-27), **I want** recorded input sequences for automated testing,
**so that** regression tests replay real input.

## US-6.2.7.6 Show Ghost Playback for Tutorials
**As a** designer (P-5), **I want** ghost playback of recorded inputs for tutorials, **so
that** players see example movements.

## US-6.2.7.7 Store Recordings as Lightweight Assets
**As an** engine developer (P-26), **I want** recordings stored as lightweight assets referencing
the mapping context, **so that** files are small and portable.

## US-6.2.7.8 Record at Action Level for Cross-Platform
**As an** engine developer (P-26), **I want** recordings at the action level (not raw device),
**so that** replays are cross-platform.

## US-6.2.7.9 Use Compressed Recordings on Mobile
**As an** engine tester (P-27), **I want to** verify mobile uses compressed recordings, **so
that** storage is minimized.

## US-6.2.7.10 Test Recording Under Full Input Load
**As an** engine tester (P-27), **I want to** test recording under full input load, **so that**
no events are dropped.

## US-6.2.7.11 Watch Tutorial Ghost Character
**As a** player (P-23), **I want** tutorial ghost characters showing recorded inputs, **so
that** I learn by watching.

## US-6.2.7.12 Verify Deterministic Playback
**As a** QA tester (P-19), **I want to** verify recording playback produces identical game
state, **so that** determinism is confirmed.

## US-6.2.8.1 Define Combo Trees as Visual Graph Assets
**As a** designer (P-5), **I want** combo definitions as visual graph assets in the editor,
**so that** complex combos are authored visually.

## US-6.2.8.2 Recognize Directional Motion Sequences
**As an** engine developer (P-26), **I want** directional motion sequence recognition
(quarter-circle, dragon punch, charge, 360), **so that** fighting-game combos work.

## US-6.2.8.3 Execute Complex Combo Chains
**As a** player (P-23), **I want to** execute complex directional combo chains, **so that**
skilled input produces powerful attacks.

## US-6.2.8.4 Support Input Buffering During Active Frames
**As an** engine developer (P-26), **I want** inputs buffered during ability active frames and
executed on cancel windows, **so that** responsive combos work.

## US-6.2.8.5 Track Combo Counter for UI Display
**As an** engine developer (P-26), **I want** combo counter tracking chain length, **so that**
UI can display combo count and damage scaling.

## US-6.2.8.6 Reset to Root on Failed Combo
**As an** engine developer (P-26), **I want** failed combos (wrong input or timeout) to reset
to root state, **so that** recovery is clean.

## US-6.2.8.7 Normalize Directional Inputs Across Devices
**As an** engine developer (P-26), **I want** stick, D-pad, and WASD normalized to the same
cardinal/diagonal inputs, **so that** combos work on any device.

## US-6.2.8.8 Use Wider Leniency Windows on Touch
**As a** designer (P-5), **I want** wider leniency windows on touch (150ms vs 100ms), **so
that** finger imprecision is accommodated.

## US-6.2.8.9 Use Swipe Directions for Touch Combos
**As a** player (P-23), **I want** swipe directions mapped to combo inputs on touch, **so
that** directional combos work on mobile.

## US-6.2.8.10 Test Branching Combo Trees
**As an** engine tester (P-27), **I want to** test all branches in a combo tree, **so that**
every valid path produces correct results.

## US-6.2.8.11 Configure Timing Windows Per Node
**As a** designer (P-5), **I want** timing windows configurable per combo tree node, **so that**
each step has appropriate leniency.

## US-6.2.8.12 Verify Combo Counter Accuracy
**As a** QA tester (P-19), **I want to** verify combo counter increments correctly and resets
on failure, **so that** display is accurate.

## US-6.2.9.1 Buffer Next Input During Active Frames
**As an** engine developer (P-26), **I want** the next input buffered during current ability
active frames, **so that** queued actions execute on completion.

## US-6.2.9.2 Define Cancel Windows Per Ability Animation
**As a** designer (P-5), **I want** cancel windows defined per ability animation as frame
ranges, **so that** specific categories can interrupt at specific times.

## US-6.2.9.3 Queue Next Attack During Recovery
**As a** player (P-23), **I want to** queue my next attack during recovery frames, **so that**
combat chains feel responsive.

## US-6.2.9.4 Resolve Priority Between Buffered Inputs
**As an** engine developer (P-26), **I want** priority rules resolving conflicts (dodge over
attack, attack over movement), **so that** the most important action executes.

## US-6.2.9.5 Integrate with Ability Activation System
**As an** engine developer (P-26), **I want** buffer system integrated with ability activation,
**so that** abilities declare cancel categories and cancel-by categories.

## US-6.2.9.6 Configure Buffer Duration
**As a** designer (P-5), **I want** buffer duration configurable (100-200ms), **so that**
responsiveness is tunable.

## US-6.2.9.7 Use Longer Buffer on Mobile
**As a** designer (P-5), **I want** longer default buffer duration on mobile (200ms vs 100ms),
**so that** touch latency is compensated.

## US-6.2.9.8 Test Buffer Under Rapid Input
**As an** engine tester (P-27), **I want to** test input buffering under rapid button presses,
**so that** only the most recent valid input executes.

## US-6.2.9.9 Cancel Into Dodge During Recovery
**As a** player (P-23), **I want to** cancel attack recovery into a dodge, **so that**
defensive actions feel responsive.

## US-6.2.9.10 Verify Cancel Window Frame Accuracy
**As an** engine tester (P-27), **I want to** verify cancel windows fire at correct animation
frames, **so that** timing is accurate.

## US-6.2.9.11 Test Priority Resolution
**As a** QA tester (P-19), **I want to** test buffered dodge vs attack priority resolution,
**so that** dodge always wins over attack.

## US-6.2.9.12 Experience Responsive Action Combat
**As a** player (P-23), **I want** combat to feel responsive without requiring frame-perfect
timing, **so that** action gameplay is accessible.

## US-6.2.10.1 Apply Input Smoothing Filter
**As an** engine developer (P-26), **I want** low-pass filter smoothing on analog stick axes,
**so that** worn controller jitter is reduced.

## US-6.2.10.2 Apply Mouse Acceleration Curves
**As an** engine developer (P-26), **I want** acceleration curves increasing sensitivity with
velocity, **so that** slow movements are precise and fast movements cover ground.

## US-6.2.10.3 Implement Target Magnetism Aim Assist
**As an** engine developer (P-26), **I want** target magnetism pulling crosshair toward
nearest valid target, **so that** gamepad aim assist works.

## US-6.2.10.4 Implement Friction Aim Assist
**As an** engine developer (P-26), **I want** friction reducing sensitivity when crosshair
passes over a target, **so that** target tracking is easier on gamepad.

## US-6.2.10.5 Aim Accurately on Gamepad
**As a** player (P-23), **I want** aim assist helping me target accurately on gamepad, **so
that** controller aiming is competitive.

## US-6.2.10.6 Configure Aim Assist Per Weapon Type
**As a** designer (P-5), **I want** aim assist strength configurable per weapon type and game
mode, **so that** balance is tunable.

## US-6.2.10.7 Disable Aim Assist in Competitive PvP
**As a** designer (P-5), **I want** aim assist disabled in competitive PvP, **so that** fair
play is ensured.

## US-6.2.10.8 Disable Aim Assist for Mouse by Default
**As an** engine developer (P-26), **I want** aim assist disabled by default on PC with mouse,
**so that** mouse users have raw input.

## US-6.2.10.9 Implement ADS Snap Aim Assist
**As an** engine developer (P-26), **I want** snap aim assist on ADS activation, **so that**
aiming down sights locks to nearest target.

## US-6.2.10.10 Configure Smoothing Time Constant
**As a** designer (P-5), **I want** smoothing time constant configurable, **so that** latency
vs jitter reduction is tunable.

## US-6.2.10.11 Test Aim Assist with Spatial Queries
**As an** engine tester (P-27), **I want to** test aim assist using spatial queries for target
detection, **so that** magnetism finds valid targets.

## US-6.2.10.12 Verify Per-Action Toggleability
**As a** QA tester (P-19), **I want to** verify smoothing, acceleration, and aim assist are
individually toggleable per action, **so that** composability works.

## US-6.2.11.1 Navigate UI with D-Pad and Stick
**As a** player (P-23), **I want** D-pad and stick to navigate UI widgets, **so that** I can
use menus without a mouse.

## US-6.2.11.2 Implement Directional Focus Navigation
**As an** engine developer (P-26), **I want** directional focus navigation moving focus between
widgets via D-pad/stick, **so that** spatial navigation works.

## US-6.2.11.3 Implement Virtual Cursor Mode
**As an** engine developer (P-26), **I want** a virtual cursor mode where stick controls a
software cursor, **so that** complex UIs (maps, skill trees) are navigable.

## US-6.2.11.4 Map A/Cross to Confirm and B/Circle to Back
**As an** engine developer (P-26), **I want** A/Cross mapped to confirm and B/Circle to back,
**so that** standard controller conventions are followed.

## US-6.2.11.5 Use Shoulder Buttons for Tab Switching
**As a** player (P-23), **I want** shoulder buttons to switch tabs, **so that** tabbed UI
panels are quick to navigate.

## US-6.2.11.6 Activate Tooltips on Focus Instead of Hover
**As an** engine developer (P-26), **I want** mouse-hover behaviors (tooltips, highlights) to
activate on focus when gamepad is active, **so that** controller users see tooltips.

## US-6.2.11.7 Support Mixed Input Seamlessly
**As an** engine developer (P-26), **I want** seamless switching between gamepad and mouse
mid-interaction, **so that** mixed input works.

## US-6.2.11.8 Verify Full UI Navigability via Controller
**As an** engine tester (P-27), **I want to** verify full UI navigability via controller,
**so that** console certification is met.

## US-6.2.11.9 Use Stick Angle for Radial Menu Selection
**As a** player (P-23), **I want** stick angle to select items in radial menus, **so that**
quick-select wheels work on gamepad.

## US-6.2.11.10 Update Button Prompts on Focus Change
**As an** engine developer (P-26), **I want** context-sensitive button prompts updating
dynamically as focus changes, **so that** prompts match the focused widget.

## US-6.2.11.11 Navigate Scroll Views with D-Pad
**As a** player (P-23), **I want** D-pad to scroll through scroll views when focused, **so
that** long lists are navigable.

## US-6.2.11.12 Test All Widget Patterns with Controller
**As a** QA tester (P-19), **I want to** test scroll views, sliders, dropdowns, and radial
menus with controller input, **so that** all widget types are navigable.
