# User Stories — 10.1 Widget Framework

## US-10.1.1 Build UI Layouts in the Visual Editor Without Code

**As a** designer (P-5), **I want** to compose UI screens using a declarative widget tree
authored entirely in the visual editor with template composition and slot injection, **so that**
I can build complex MMO interfaces without editing text files or writing code.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Widget tree authored as declarative asset files in visual editor | F-10.1.1 | R-10.1.1 |
| Binary UI asset format with templates and named slots | F-10.1.2 | R-10.1.2 |
| Template composition — define once, instantiate with different bindings | F-10.1.2 | R-10.1.2 |
| No text editing required for any UI authoring task | F-10.1.2 | R-X.9.5 |

## US-10.1.2 Verify Widget Tree Diffs Apply Minimal Mutations

**As an** engine tester (P-27), **I want** to verify that the declarative tree diffing engine
applies only insert, remove, update, and reorder mutations without full rebuilds, **so that**
I can confirm the framework meets its O(n) diffing performance target for keyed lists.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Diffing produces minimal mutations for property changes | F-10.1.12 | R-10.1.12 |
| Keyed list reconciliation runs in O(n) | F-10.1.12 | R-10.1.12 |
| Unchanged subtrees skip layout recalculation | F-10.1.12 | R-10.1.12 |
| 60fps UI updates with hundreds of bound widgets changing | F-10.1.1, F-10.1.12 | R-10.1.1, R-10.1.12 |

## US-10.1.3 Develop the Retained Widget Tree with Efficient Diffing

**As an** engine developer (P-26), **I want** to implement a retained widget tree with
automatic minimal diffing using a keyed reconciliation algorithm, **so that** the framework
achieves retained-mode performance with declarative authoring simplicity.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Retained tree updated via diff, not full rebuild | F-10.1.1 | R-10.1.1 |
| Property mutations patch in place | F-10.1.12 | R-10.1.12 |
| Insertions and deletions splice the tree | F-10.1.12 | R-10.1.12 |
| Reordered children moved without destroy/recreate | F-10.1.12 | R-10.1.12 |

## US-10.1.4 Recycle Widgets in Virtualized Lists Without Allocation Churn

**As a** developer (P-15), **I want** widget instances pooled and recycled for virtualized
list views and inventory grids, **so that** frequently rebuilt UI elements avoid allocation
churn even when hundreds of items scroll in and out of view each second.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Widgets return to pool on dismiss or scroll-out | F-10.1.3 | R-10.1.3 |
| Pooled widgets rebound to new data without reallocation | F-10.1.3 | R-10.1.3 |
| Mobile enforces 200 active widget budget, desktop 500 | F-10.1.3 | R-10.1.3 |

## US-10.1.5 Stress-Test Widget Pooling Under Rapid Scroll

**As an** engine tester (P-27), **I want** to stress-test widget pooling by scrolling through
thousands of items at maximum speed, **so that** I can verify zero allocation churn and
confirm the active widget budget is respected on all platforms.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| No heap allocations during rapid scroll of 10,000-item list | F-10.1.3 | R-10.1.3 |
| Active widget count stays within platform budget | F-10.1.3 | R-10.1.3 |
| Pool correctly rebinds data on widget reuse | F-10.1.3 | R-10.1.3 |

## US-10.1.6 Arrange Widgets Using Flexbox and Grid Layout

**As a** designer (P-5), **I want** CSS-like flexbox for one-dimensional flows and grid
layout for two-dimensional arrangements, **so that** I can build toolbars, action bars,
inventory grids, and talent trees with automatic positioning and alignment.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Flexbox layout with gap, alignment, justification, wrapping | F-10.1.4 | R-10.1.4 |
| Grid layout with min/max size constraints | F-10.1.4 | R-10.1.4 |
| Both layout modes support responsive reflow | F-10.1.4 | R-10.1.4 |

## US-10.1.7 Anchor HUD Elements to Screen Edges and Other Widgets

**As a** designer (P-5), **I want** anchor-based and constraint-based layout for HUD
elements that maintain fixed positions relative to screen edges or other widgets, **so that**
my layouts adapt correctly across resolutions without manual repositioning.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Anchors define edges with pixel or percentage offsets | F-10.1.5 | R-10.1.5 |
| Constraints express inter-widget relationships | F-10.1.5 | R-10.1.5 |
| Layouts adapt to different screen resolutions | F-10.1.5 | R-10.1.5 |

## US-10.1.8 Verify Layout Correctness Across Resolutions

**As a** QA engineer (P-19), **I want** to verify that flexbox, grid, anchor, and constraint
layouts produce correct results across all supported resolutions and aspect ratios, **so that**
no widget overflows, clips incorrectly, or overlaps at any resolution.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Layouts correct at 720p, 1080p, 1440p, 4K, ultrawide | F-10.1.4, F-10.1.5 | R-10.1.4, R-10.1.5 |
| No widget overflow or unintended clipping at any resolution | F-10.1.4, F-10.1.5 | R-10.1.4, R-10.1.5 |
| Anchor offsets maintain proportional spacing | F-10.1.5 | R-10.1.5 |

## US-10.1.9 Theme the UI With Cascading Styles and Runtime Swapping

**As a** designer (P-5), **I want** a CSS-like cascading style system with external theme
files that can be swapped at runtime, **so that** I can support light/dark modes,
faction-specific skins, and seasonal event themes without duplicating widget trees.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Styles cascade by widget type, ID, class, and state | F-10.1.6 | R-10.1.6 |
| Theme files swapped at runtime without rebuild | F-10.1.6 | R-10.1.6 |
| Selectors match hover, pressed, focused, disabled states | F-10.1.6 | R-10.1.6 |

## US-10.1.10 Create Faction-Themed UI Skins

**As an** artist (P-8), **I want** to create distinct visual themes per faction by defining
style overrides for colors, fonts, borders, and backgrounds, **so that** each faction's UI
feels unique while sharing the same widget structure.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Multiple theme files coexist and swap at runtime | F-10.1.6 | R-10.1.6 |
| Style overrides apply colors, fonts, borders, shadows | F-10.1.6 | R-10.1.6 |
| Same widget tree renders differently per theme | F-10.1.6 | R-10.1.6 |

## US-10.1.11 Bind UI Widgets to Live Game State Reactively

**As a** developer (P-15), **I want** reactive data bindings that automatically update UI
when game state changes, supporting one-way, two-way, and computed values, **so that**
player stats, inventory, quest progress, and buff durations reflect in real time without
manual polling.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| One-way bindings update view when model changes | F-10.1.7 | R-10.1.7 |
| Two-way bindings sync model and view bidirectionally | F-10.1.7 | R-10.1.7 |
| Computed/derived values update automatically | F-10.1.7 | R-10.1.7 |

## US-10.1.12 See Real-Time Stat Updates Without Manual Refresh

**As a** player (P-23), **I want** health, mana, experience, and quest progress to update
on screen immediately when they change, **so that** I always see accurate information
without needing to close and reopen panels.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Stats update within same frame as underlying data change | F-10.1.7 | R-10.1.7 |
| No manual refresh or panel reopen required | F-10.1.7 | R-10.1.7 |

## US-10.1.13 Navigate UI Panels With Keyboard and Gamepad

**As a** player (P-23), **I want** keyboard tab order, D-pad directional navigation, focus
groups, and focus trapping for modal dialogs, **so that** I can navigate complex MMO UI
panels, cycle action bar slots, and switch between open panels without a mouse.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Tab order traverses all interactive widgets | F-10.1.8 | R-10.1.8 |
| D-pad and arrow keys navigate directionally | F-10.1.8 | R-10.1.8 |
| Modal dialogs trap focus within their bounds | F-10.1.8 | R-10.1.8 |
| Context preserved when switching between open panels | F-10.1.8 | R-10.1.8 |

## US-10.1.14 Test Focus Traversal Across Nested Menu Hierarchies

**As a** QA engineer (P-19), **I want** to verify that focus traversal works correctly
across nested menus, modal dialogs, and multiple open panels, **so that** no interactive
element is unreachable and focus never gets stuck in a dead end.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| All interactive elements reachable via keyboard/gamepad | F-10.1.8 | R-10.1.8 |
| Focus never becomes trapped outside modal boundaries | F-10.1.8 | R-10.1.8 |
| Nested menu navigation returns to parent correctly | F-10.1.8 | R-10.1.8 |

## US-10.1.15 Switch Locale and See UI Re-Layout Automatically

**As a** player (P-23), **I want** to switch the game's language at runtime and have all
text, images, and layout directions update automatically, **so that** I can play in my
preferred language including right-to-left languages like Arabic.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| All user-visible text switches by locale at runtime | F-10.1.9 | R-10.1.9 |
| Widgets re-layout when text changes length | F-10.1.9 | R-10.1.9 |
| RTL layout mirroring for Arabic and Hebrew | F-10.1.9 | R-10.1.9 |
| Pluralization, gendered text, number/date formatting | F-10.1.9 | R-10.1.9 |

## US-10.1.16 Verify Localization Across All Supported Languages

**As a** QA engineer (P-19), **I want** to verify that every UI screen renders correctly
in all supported languages, including RTL and CJK, **so that** no text overflows, clips,
or breaks layout in any locale.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| No text overflow in any locale's longest strings | F-10.1.9 | R-10.1.9 |
| RTL mirroring correct for all interactive controls | F-10.1.9 | R-10.1.9 |
| CJK text shapes and renders correctly | F-10.1.9 | R-10.1.9 |

## US-10.1.17 Place UI Panels in the 3D World for Diegetic Interfaces

**As a** designer (P-5), **I want** to render the same widget tree as a world-space 3D
panel that receives ray-cast input, **so that** I can create diegetic interfaces like
in-game computer screens, holographic displays, and shop kiosks.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Widget tree renders as textured quad in 3D scene | F-10.1.10 | R-10.1.10 |
| World-space panels receive ray-cast cursor input | F-10.1.10 | R-10.1.10 |
| All layout modes, styling, and data binding work in world space | F-10.1.10 | R-10.1.10 |
| Panel resolution and physical size configurable per instance | F-10.1.10 | R-10.1.10 |

## US-10.1.18 Interact With VR UI Using Controller and Hand Tracking

**As a** player (P-23), **I want** to interact with VR UI panels using laser pointer, direct
touch, gaze-and-dwell, and hand tracking pinch gestures, **so that** I can use menus and
interfaces comfortably in VR without removing my headset.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Laser pointer from hand controllers activates UI elements | F-10.1.11 | R-10.1.11 |
| Direct touch and gaze-and-dwell input modes supported | F-10.1.11 | R-10.1.11 |
| Text auto-scales based on panel distance | F-10.1.11 | R-10.1.11 |
| Comfort settings clamp panel positions | F-10.1.11 | R-10.1.11 |

## US-10.1.19 Test VR Input Modes Across Controller Types

**As a** QA engineer (P-19), **I want** to verify that all VR input modes work correctly
across different headsets and controller types, **so that** no input method fails on any
supported VR platform.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Laser pointer works on all supported VR controllers | F-10.1.11 | R-10.1.11 |
| Hand tracking pinch gestures register accurately | F-10.1.11 | R-10.1.11 |
| Focus system adapts to active input mode | F-10.1.11 | R-10.1.11 |

## US-10.1.20 Animate Widget Properties With Keyframed Curves

**As a** designer (P-5), **I want** to animate widget properties like position, opacity,
color, rotation, and scale using keyframed curves with easing functions, **so that** I can
create polished transition animations, pulsing highlights, and staggered list reveals.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Named animation assets authored in UI visual editor | F-10.1.13 | R-10.1.13 |
| State-change transitions (slide-in, fade-out) | F-10.1.13 | R-10.1.13 |
| Looping animations (pulsing glow, spinning indicator) | F-10.1.13 | R-10.1.13 |
| Interruptible animations blend smoothly when retriggered | F-10.1.13 | R-10.1.13 |
| Staggered list animations with configurable delay | F-10.1.13 | R-10.1.13 |

## US-10.1.21 Implement the Widget Animation System

**As an** engine developer (P-26), **I want** to implement a widget animation system that
operates directly on widget tree properties independent of the game animation system,
**so that** UI animations run at consistent frame rates regardless of game simulation state.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Animations run independently of game animation system | F-10.1.13 | R-10.1.13 |
| Easing functions: linear, ease-in/out, cubic bezier, spring, bounce | F-10.1.13 | R-10.1.13 |
| Animation system operates on widget tree properties directly | F-10.1.13 | R-10.1.13 |

## US-10.1.22 Configure Audio Feedback Per Widget Interaction

**As a** designer (P-5), **I want** automatic audio feedback for widget interactions with
per-widget and per-theme sound overrides played through a dedicated UI mixer bus, **so that**
every button click, hover, scroll, and notification has satisfying audio feedback.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Default sounds for click, hover, focus, scroll, drag | F-10.1.14 | R-10.1.14 |
| Sound slots overridable per-widget and per-theme | F-10.1.14 | R-10.1.14 |
| Sounds play through dedicated UI mixer bus | F-10.1.14 | R-10.1.14 |

## US-10.1.23 Disable or Replace UI Sounds With Haptic Feedback

**As a** player (P-23), **I want** to disable UI audio globally or per sound type and
optionally replace it with haptic feedback, **so that** I can customize my sensory
experience based on my accessibility needs and preferences.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| UI sounds disableable globally and individually per type | F-10.1.14 | R-10.1.14 |
| Haptic feedback replacement on supported platforms | F-10.1.14 | R-10.1.14 |
| Audio feedback respects accessibility settings | F-10.1.14 | R-10.1.14 |

## US-10.1.24 Verify UI Renders Under Budget

**As an** engine tester (P-27), **I want** to benchmark the full HUD rendering to verify
it completes under 2ms GPU time and 50 draw calls, **so that** the UI meets its performance
budget across all target platforms.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Full HUD renders under 2ms GPU time | F-10.1.1 | R-X.12.1 |
| Total draw calls under 50 for complete HUD | F-10.1.1 | R-X.12.1 |
| Performance verified on minimum spec hardware | F-10.1.1 | R-X.12.1 |
