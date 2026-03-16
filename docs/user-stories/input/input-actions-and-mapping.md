# User Stories — 6.2 Input Actions & Mapping

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-6.2.1.1 | designer (P-5) | define boolean actions (pressed/not pressed) | simple triggers like "Jump" and "Dodge" work |  |  |
| US-6.2.1.2 | designer (P-5) | define axis 1D actions (scalar float) | throttle and zoom inputs produce continuous values |  |  |
| US-6.2.1.3 | designer (P-5) | define axis 2D actions (vector2) | movement and camera look produce directional input |  |  |
| US-6.2.1.4 | engine developer (P-26) | implement strongly typed actions decoupling gameplay from device inputs | the same action fires from any device |  |  |
| US-6.2.1.5 | player (P-23) | the same game actions to work on keyboard, gamepad, and touch | I can use any device |  |  |
| US-6.2.1.6 | engine developer (P-26) | axis 3D actions (vector3) | VR motion and spatial input produce 3-axis values |  |  |
| US-6.2.1.7 | player (P-23) | touch virtual joystick as the default axis2D source on mobile | movement works on touchscreens |  |  |
| US-6.2.1.8 | engine tester (P-27) | verify boolean, axis 1D, 2D, and 3D action types fire correctly from all input sources | typed actions work |  |  |
| US-6.2.1.9 | designer (P-5) | default action bindings per platform (touch, gamepad, keyboard) | each platform has appropriate defaults |  |  |
| US-6.2.1.10 | QA tester (P-19) | test each action from keyboard, gamepad, and touch | all input paths are validated |  |  |
| US-6.2.1.11 | engine developer (P-26) | the same "Dodge Roll" action to fire from keyboard, gamepad button, or touch gesture | input is device-independent |  |  |
| US-6.2.1.12 | player (P-23) | controls to feel consistent regardless of input device | switching devices is seamless |  |  |
| US-6.2.2.1 | designer (P-5) | create named mapping contexts (OnFoot, Mounted, UIMenu) | input bindings are grouped by mode |  |  |
| US-6.2.2.2 | engine developer (P-26) | contexts activated on a priority-ordered stack | higher-priority contexts consume inputs first |  |  |
| US-6.2.2.3 | player (P-23) | controls to change when mounting a vehicle or horse | each mode has appropriate bindings |  |  |
| US-6.2.2.4 | engine developer (P-26) | context activation, deactivation, and priority management | modal overlays work correctly |  |  |
| US-6.2.2.5 | designer (P-5) | touch-specific contexts shipped for mobile | touch UI has appropriate bindings |  |  |
| US-6.2.2.6 | engine tester (P-27) | verify higher-priority contexts consume inputs before lower ones | modal overlays work |  |  |
| US-6.2.2.7 | player (P-23) | the inventory context to capture Escape before combat | closing inventory works correctly |  |  |
| US-6.2.2.8 | engine tester (P-27) | test rapid context switching | activation and deactivation are robust |  |  |
| US-6.2.2.9 | designer (P-5) | configure mapping contexts in the visual editor | input modes are visual |  |  |
| US-6.2.2.10 | QA tester (P-19) | verify each platform ships with appropriate default contexts | out-of-box experience is good |  |  |
| US-6.2.2.11 | engine developer (P-26) | UI contexts layered over movement contexts | menus consume inputs without affecting gameplay |  |  |
| US-6.2.2.12 | player (P-23) | controls to automatically adapt when entering menus, vehicles, or combat | the right bindings are active |  |  |
| US-6.2.3.1 | designer (P-5) | axial and radial dead zone modifiers on stick input | stick drift is eliminated |  |  |
| US-6.2.3.2 | designer (P-5) | linear, exponential, and S-curve response modifiers | stick feel is tunable per action |  |  |
| US-6.2.3.3 | engine developer (P-26) | configurable modifier chains applied to raw input before actions | processing is composable |  |  |
| US-6.2.3.4 | player (P-23) | per-player sensitivity options | I can customize stick feel to my preference |  |  |
| US-6.2.3.5 | engine developer (P-26) | swizzle (remap axes) and negate (invert) modifiers | axis remapping is flexible |  |  |
| US-6.2.3.6 | engine developer (P-26) | scalar sensitivity multiplier modifier | input speed is tunable |  |  |
| US-6.2.3.7 | designer (P-5) | dead zone defaults varying by controller type (Xbox vs DualSense) | each controller feels right out of box |  |  |
| US-6.2.3.8 | engine tester (P-27) | verify modifiers apply in the correct order | dead zone before response curve is maintained |  |  |
| US-6.2.3.9 | player (P-23) | camera control to feel smooth with no jitter | stick input is comfortable |  |  |
| US-6.2.3.10 | designer (P-5) | response curve presets for camera aim, vehicle steering, and menu scrolling | common cases are handled |  |  |
| US-6.2.3.11 | engine tester (P-27) | test modifier chains with extreme input values | edge cases produce valid output |  |  |
| US-6.2.3.12 | QA tester (P-19) | verify modifier pipeline cost is negligible | input processing does not affect frame rate |  |  |
| US-6.2.4.1 | designer (P-5) | a "pressed" trigger condition firing on the frame a key is pressed | instant-cast abilities work |  |  |
| US-6.2.4.2 | designer (P-5) | a "hold" trigger condition firing after sustained duration | channeled abilities work |  |  |
| US-6.2.4.3 | designer (P-5) | a "tap" trigger (press and release within threshold) | quick actions are distinguished from holds |  |  |
| US-6.2.4.4 | engine developer (P-26) | chord triggers (multiple simultaneous inputs) | Shift+1 extended action bar pages work |  |  |
| US-6.2.4.5 | engine developer (P-26) | combo triggers (ordered sequence within time window) | ability chains work |  |  |
| US-6.2.4.6 | player (P-23) | holding a button to channel abilities | sustained actions feel natural |  |  |
| US-6.2.4.7 | engine developer (P-26) | pulse triggers repeating on interval while held | auto-fire mechanics work |  |  |
| US-6.2.4.8 | designer (P-5) | longer tap/hold thresholds on mobile | finger imprecision is accommodated |  |  |
| US-6.2.4.9 | engine tester (P-27) | verify chord triggers cap at 2 simultaneous on touch | mobile limitations are respected |  |  |
| US-6.2.4.10 | engine developer (P-26) | "released" triggers firing on key-up | bow release and charge attacks work |  |  |
| US-6.2.4.11 | engine tester (P-27) | test all trigger types (pressed, released, hold, tap, pulse, chord, combo) | all conditions work |  |  |
| US-6.2.4.12 | player (P-23) | combo inputs to chain abilities | skilled input produces powerful moves |  |  |
| US-6.2.5.1 | player (P-23) | rebind any action to any compatible input at runtime | controls are fully customizable |  |  |
| US-6.2.5.2 | engine developer (P-26) | binding conflict detection within overlapping mapping contexts | duplicate bindings are flagged |  |  |
| US-6.2.5.3 | engine developer (P-26) | conflict resolution options (swap, unbind, cancel) presented to the player | conflicts are resolved cleanly |  |  |
| US-6.2.5.4 | engine developer (P-26) | rebindings serialized and restored on session start | customizations persist |  |  |
| US-6.2.5.5 | player (P-23) | customize dozens of action bar bindings | my hotbar layout matches my preferences |  |  |
| US-6.2.5.6 | engine developer (P-26) | rebinding to respect platform reserved keys (PS button, Guide button) | reserved inputs are not stolen |  |  |
| US-6.2.5.7 | engine tester (P-27) | verify conflict detection works across overlapping contexts | only real conflicts are flagged |  |  |
| US-6.2.5.8 | designer (P-5) | platform-specific default bindings | Xbox and PlayStation use correct button labels |  |  |
| US-6.2.5.9 | engine tester (P-27) | verify rebindings persist across game sessions | customizations are not lost |  |  |
| US-6.2.5.10 | player (P-23) | reset bindings to defaults | I can undo customizations |  |  |
| US-6.2.5.11 | engine developer (P-26) | conflict detection to account for context priority | non-overlapping contexts do not trigger false positives |  |  |
| US-6.2.5.12 | QA tester (P-19) | test rebinding with modifier keys (Shift, Ctrl, Alt) | modifier combinations are supported |  |  |
| US-6.2.6.1 | engine developer (P-26) | input actions automatically resolved to platform-specific button icons | UI prompts show correct glyphs |  |  |
| US-6.2.6.2 | engine developer (P-26) | displayed glyphs to update instantly when the active device changes | prompts always match the current input |  |  |
| US-6.2.6.3 | player (P-23) | button prompts showing my controller's icons (Xbox A, PlayStation Cross) | UI guidance is correct |  |  |
| US-6.2.6.4 | engine developer (P-26) | each action mapped to the bound input, then to the platform icon atlas | glyph resolution is two-step |  |  |
| US-6.2.6.5 | designer (P-5) | glyph atlases as swappable assets | custom controller icon packs are possible |  |  |
| US-6.2.6.6 | engine tester (P-27) | verify glyph resolution for Xbox, PlayStation, Switch, and keyboard | all controller types show correct icons |  |  |
| US-6.2.6.7 | engine developer (P-26) | UI widgets to bind glyph images reactively to the active device | updates are automatic |  |  |
| US-6.2.6.8 | player (P-23) | keyboard key names shown when using keyboard | prompts match my input device |  |  |
| US-6.2.6.9 | engine tester (P-27) | test switching between keyboard and gamepad and verify glyph updates | transitions are instant |  |  |
| US-6.2.6.10 | QA tester (P-19) | verify correct platform-native button icons at all times | console certification requirements are met |  |  |
| US-6.2.6.11 | designer (P-5) | icon pack selectable in settings | players choose their preferred glyph style |  |  |
| US-6.2.6.12 | player (P-23) | VR controller diagrams in prompts when using VR | VR controls are clearly communicated |  |  |
| US-6.2.7.1 | engine developer (P-26) | all input events recorded to a binary stream with timestamps | input replay is possible |  |  |
| US-6.2.7.2 | engine developer (P-26) | recorded input replayed deterministically | game state can be reproduced |  |  |
| US-6.2.7.3 | engine developer (P-26) | playback speed control (slow-mo, fast-forward, frame-step) | debugging uses replay analysis |  |  |
| US-6.2.7.4 | QA tester (P-19) | attach input recordings to bug reports | issues are reproducible |  |  |
| US-6.2.7.5 | engine tester (P-27) | recorded input sequences for automated testing | regression tests replay real input |  |  |
| US-6.2.7.6 | designer (P-5) | ghost playback of recorded inputs for tutorials | players see example movements |  |  |
| US-6.2.7.7 | engine developer (P-26) | recordings stored as lightweight assets referencing the mapping context | files are small and portable |  |  |
| US-6.2.7.8 | engine developer (P-26) | recordings at the action level (not raw device) | replays are cross-platform |  |  |
| US-6.2.7.9 | engine tester (P-27) | verify mobile uses compressed recordings | storage is minimized |  |  |
| US-6.2.7.10 | engine tester (P-27) | test recording under full input load | no events are dropped |  |  |
| US-6.2.7.11 | player (P-23) | tutorial ghost characters showing recorded inputs | I learn by watching |  |  |
| US-6.2.7.12 | QA tester (P-19) | verify recording playback produces identical game state | determinism is confirmed |  |  |
| US-6.2.8.1 | designer (P-5) | combo definitions as visual graph assets in the editor | complex combos are authored visually |  |  |
| US-6.2.8.2 | engine developer (P-26) | directional motion sequence recognition (quarter-circle, dragon punch, charge, 360) | fighting-game combos work |  |  |
| US-6.2.8.3 | player (P-23) | execute complex directional combo chains | skilled input produces powerful attacks |  |  |
| US-6.2.8.4 | engine developer (P-26) | inputs buffered during ability active frames and executed on cancel windows | responsive combos work |  |  |
| US-6.2.8.5 | engine developer (P-26) | combo counter tracking chain length | UI can display combo count and damage scaling |  |  |
| US-6.2.8.6 | engine developer (P-26) | failed combos (wrong input or timeout) to reset to root state | recovery is clean |  |  |
| US-6.2.8.7 | engine developer (P-26) | stick, D-pad, and WASD normalized to the same cardinal/diagonal inputs | combos work on any device |  |  |
| US-6.2.8.8 | designer (P-5) | wider leniency windows on touch (150ms vs 100ms) | finger imprecision is accommodated |  |  |
| US-6.2.8.9 | player (P-23) | swipe directions mapped to combo inputs on touch | directional combos work on mobile |  |  |
| US-6.2.8.10 | engine tester (P-27) | test all branches in a combo tree | every valid path produces correct results |  |  |
| US-6.2.8.11 | designer (P-5) | timing windows configurable per combo tree node | each step has appropriate leniency |  |  |
| US-6.2.8.12 | QA tester (P-19) | verify combo counter increments correctly and resets on failure | display is accurate |  |  |
| US-6.2.9.1 | engine developer (P-26) | the next input buffered during current ability active frames | queued actions execute on completion |  |  |
| US-6.2.9.2 | designer (P-5) | cancel windows defined per ability animation as frame ranges | specific categories can interrupt at specific times |  |  |
| US-6.2.9.3 | player (P-23) | queue my next attack during recovery frames | combat chains feel responsive |  |  |
| US-6.2.9.4 | engine developer (P-26) | priority rules resolving conflicts (dodge over attack, attack over movement) | the most important action executes |  |  |
| US-6.2.9.5 | engine developer (P-26) | buffer system integrated with ability activation | abilities declare cancel categories and cancel-by categories |  |  |
| US-6.2.9.6 | designer (P-5) | buffer duration configurable (100-200ms) | responsiveness is tunable |  |  |
| US-6.2.9.7 | designer (P-5) | longer default buffer duration on mobile (200ms vs 100ms) | touch latency is compensated |  |  |
| US-6.2.9.8 | engine tester (P-27) | test input buffering under rapid button presses | only the most recent valid input executes |  |  |
| US-6.2.9.9 | player (P-23) | cancel attack recovery into a dodge | defensive actions feel responsive |  |  |
| US-6.2.9.10 | engine tester (P-27) | verify cancel windows fire at correct animation frames | timing is accurate |  |  |
| US-6.2.9.11 | QA tester (P-19) | test buffered dodge vs attack priority resolution | dodge always wins over attack |  |  |
| US-6.2.9.12 | player (P-23) | combat to feel responsive without requiring frame-perfect timing | action gameplay is accessible |  |  |
| US-6.2.10.1 | engine developer (P-26) | low-pass filter smoothing on analog stick axes | worn controller jitter is reduced |  |  |
| US-6.2.10.2 | engine developer (P-26) | acceleration curves increasing sensitivity with velocity | slow movements are precise and fast movements cover ground |  |  |
| US-6.2.10.3 | engine developer (P-26) | target magnetism pulling crosshair toward nearest valid target | gamepad aim assist works |  |  |
| US-6.2.10.4 | engine developer (P-26) | friction reducing sensitivity when crosshair passes over a target | target tracking is easier on gamepad |  |  |
| US-6.2.10.5 | player (P-23) | aim assist helping me target accurately on gamepad | controller aiming is competitive |  |  |
| US-6.2.10.6 | designer (P-5) | aim assist strength configurable per weapon type and game mode | balance is tunable |  |  |
| US-6.2.10.7 | designer (P-5) | aim assist disabled in competitive PvP | fair play is ensured |  |  |
| US-6.2.10.8 | engine developer (P-26) | aim assist disabled by default on PC with mouse | mouse users have raw input |  |  |
| US-6.2.10.9 | engine developer (P-26) | snap aim assist on ADS activation | aiming down sights locks to nearest target |  |  |
| US-6.2.10.10 | designer (P-5) | smoothing time constant configurable | latency vs jitter reduction is tunable |  |  |
| US-6.2.10.11 | engine tester (P-27) | test aim assist using spatial queries for target detection | magnetism finds valid targets |  |  |
| US-6.2.10.12 | QA tester (P-19) | verify smoothing, acceleration, and aim assist are individually toggleable per action | composability works |  |  |
| US-6.2.11.1 | player (P-23) | D-pad and stick to navigate UI widgets | I can use menus without a mouse |  |  |
| US-6.2.11.2 | engine developer (P-26) | directional focus navigation moving focus between widgets via D-pad/stick | spatial navigation works |  |  |
| US-6.2.11.3 | engine developer (P-26) | a virtual cursor mode where stick controls a software cursor | complex UIs (maps, skill trees) are navigable |  |  |
| US-6.2.11.4 | engine developer (P-26) | A/Cross mapped to confirm and B/Circle to back | standard controller conventions are followed |  |  |
| US-6.2.11.5 | player (P-23) | shoulder buttons to switch tabs | tabbed UI panels are quick to navigate |  |  |
| US-6.2.11.6 | engine developer (P-26) | mouse-hover behaviors (tooltips, highlights) to activate on focus when gamepad is active | controller users see tooltips |  |  |
| US-6.2.11.7 | engine developer (P-26) | seamless switching between gamepad and mouse mid-interaction | mixed input works |  |  |
| US-6.2.11.8 | engine tester (P-27) | verify full UI navigability via controller | console certification is met |  |  |
| US-6.2.11.9 | player (P-23) | stick angle to select items in radial menus | quick-select wheels work on gamepad |  |  |
| US-6.2.11.10 | engine developer (P-26) | context-sensitive button prompts updating dynamically as focus changes | prompts match the focused widget |  |  |
| US-6.2.11.11 | player (P-23) | D-pad to scroll through scroll views when focused | long lists are navigable |  |  |
| US-6.2.11.12 | QA tester (P-19) | test scroll views, sliders, dropdowns, and radial menus with controller input | all widget types are navigable |  |  |
