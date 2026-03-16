# User Stories — 10.6 Accessibility

## US-10.6.1 Navigate UI With a Screen Reader

**As a** player (P-23) who is blind or has low vision, **I want** the widget tree exposed to
platform accessibility APIs so screen readers announce widget labels, roles, states, and values,
**so that** I can navigate all menus and interact with the game using assistive technology.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Every interactive widget has a programmatic name and role | F-10.6.1 | R-10.6.1 |
| Navigation events and focus changes announced in real time | F-10.6.1 | R-10.6.1 |
| Live region updates (chat, combat log, notifications) announced | F-10.6.1 | R-10.6.1 |

## US-10.6.2 Implement Platform Accessibility Bridges

**As an** engine developer (P-26), **I want** to implement native accessibility bridges for
NSAccessibility (macOS), UI Automation (Windows), and AT-SPI (Linux), **so that** the widget tree
integrates with each platform's screen reader infrastructure.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| macOS bridge via NSAccessibility | F-10.6.1 | R-10.6.1 |
| Windows bridge via UI Automation | F-10.6.1 | R-10.6.1 |
| Linux bridge via AT-SPI | F-10.6.1 | R-10.6.1 |

## US-10.6.3 Verify Screen Reader Announces All Interactive Elements

**As a** QA engineer (P-19), **I want** to verify that every interactive widget is announced
correctly by VoiceOver (macOS), Narrator (Windows), and Orca (Linux), **so that** no UI element is
unreachable via screen reader navigation.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| All buttons, sliders, checkboxes announced with label and role | F-10.6.1 | R-10.6.1 |
| State changes (checked/unchecked, enabled/disabled) announced | F-10.6.1 | R-10.6.1 |
| No interactive element missing from accessibility tree | F-10.6.1 | R-10.6.1 |

## US-10.6.4 Read Subtitles for Dialogue and Cinematics

**As a** player (P-23) who is deaf or hard of hearing, **I want** configurable subtitles for
dialogue and voice-over with closed captions for non-speech audio including directional indicators,
**so that** I understand all audio information visually.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Subtitles with configurable font size, color, background opacity | F-10.6.2 | R-10.6.2 |
| Speaker identification labels | F-10.6.2 | R-10.6.2 |
| Closed captions for non-speech audio with directional indicators | F-10.6.2 | R-10.6.2 |
| Subtitle positioning avoids obscuring gameplay elements | F-10.6.2 | R-10.6.2 |

## US-10.6.5 Customize Subtitle Appearance and Timing

**As a** designer (P-5), **I want** to configure subtitle and caption presentation including timing,
speed, line count, and positioning, **so that** subtitles integrate well with the game's visual
design while remaining readable.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Timing, speed, and line count configurable | F-10.6.2 | R-10.6.2 |
| Positioning avoids important gameplay elements | F-10.6.2 | R-10.6.2 |
| System caption preferences read from iOS/Android APIs | F-10.6.2 | R-10.6.2 |

## US-10.6.6 Verify Subtitle Timing Syncs With Audio

**As a** QA engineer (P-19), **I want** to verify that subtitles and captions sync correctly with
audio playback across all supported platforms, **so that** no subtitle appears early, late, or out
of sequence.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Subtitle timing within 100ms of audio playback | F-10.6.2 | R-10.6.2 |
| Caption sequence matches audio event order | F-10.6.2 | R-10.6.2 |
| Subtitle speed adjustable without desync | F-10.6.2 | R-10.6.2 |

## US-10.6.7 Play With Color Vision Deficiency Using Corrective Modes

**As a** player (P-23) with color vision deficiency, **I want** simulation and correction modes for
protanopia, deuteranopia, and tritanopia with alternative visual cues for color-coded information,
**so that** I can perceive gameplay-critical information like item rarity and team colors.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Post-process filters for all three CVD types | F-10.6.3 | R-10.6.3 |
| Alternative cues (patterns, shapes, icons, labels) for color-coded info | F-10.6.3 | R-10.6.3 |
| Colorblind modes previewable in settings | F-10.6.3 | R-10.6.3 |

## US-10.6.8 Create Alternative Visual Cues for Color-Coded Elements

**As a** technical artist (P-13), **I want** to define alternative visual cues (patterns, shapes,
icons) for each color-coded gameplay element, **so that** item rarity, team colors, and threat
indicators are distinguishable without relying solely on color.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Pattern overlays for item rarity borders | F-10.6.3 | R-10.6.3 |
| Shape-based indicators for team colors | F-10.6.3 | R-10.6.3 |
| Icon-based threat level indicators | F-10.6.3 | R-10.6.3 |

## US-10.6.9 Verify Colorblind Modes Provide Sufficient Contrast

**As a** QA engineer (P-19), **I want** to verify that each colorblind mode provides sufficient
visual differentiation for all gameplay-critical color-coded information, **so that** no player with
CVD is at a disadvantage.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| All color-coded elements distinguishable in each CVD mode | F-10.6.3 | R-10.6.3 |
| Alternative cues present for every color-critical indicator | F-10.6.3 | R-10.6.3 |
| Preview mode accurately simulates each CVD type | F-10.6.3 | R-10.6.3 |

## US-10.6.10 Scale the UI From 80% to 250% Without Breaking Layout

**As a** player (P-23), **I want** to scale the entire UI from 80% to 250% with all widgets, text,
and icons scaling proportionally and high-contrast mode for stark color pairs, **so that** I can
read and interact with the UI comfortably at any display size.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Continuous UI scale from 80% to 250% | F-10.6.4 | R-10.6.4 |
| All widgets, text, and icons scale proportionally | F-10.6.4 | R-10.6.4 |
| High-contrast mode increases borders, removes transparency | F-10.6.4 | R-10.6.4 |
| Layout reflows correctly at extreme scales | F-10.6.4 | R-10.6.4 |

## US-10.6.11 Implement System DPI and Text Scale Integration

**As an** engine developer (P-26), **I want** to implement integration with system-level DPI and
text scale settings on Windows, macOS, and Linux, **so that** the UI automatically respects the
player's OS accessibility preferences.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Windows DPI via SetProcessDpiAwareness | F-10.6.4 | R-10.6.4 |
| macOS scale via backingScaleFactor | F-10.6.4 | R-10.6.4 |
| Linux DPI via Xft.dpi / Wayland scale | F-10.6.4 | R-10.6.4 |

## US-10.6.12 Verify Layout Reflow at Extreme Scale Factors

**As an** engine tester (P-27), **I want** to verify that all UI screens layout correctly at 80%,
150%, 200%, and 250% scale, **so that** no widget overflows, clips, or becomes unreachable at any
scale factor.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| No overflow or clipping at 80% scale | F-10.6.4 | R-10.6.4 |
| No overflow or clipping at 250% scale | F-10.6.4 | R-10.6.4 |
| All interactive elements remain reachable at every scale | F-10.6.4 | R-10.6.4 |

## US-10.6.13 Remap All Inputs for One-Handed or Switch-Device Play

**As a** player (P-23) with motor disabilities, **I want** complete remapping of all keyboard,
mouse, and gamepad bindings including one-handed layouts, mouse-only play, and switch-device
scanning navigation, **so that** I can play the game with my available input capabilities.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| All bindings remappable including UI navigation | F-10.6.5 | R-10.6.5 |
| One-handed keyboard layouts supported | F-10.6.5 | R-10.6.5 |
| Mouse-only play with click-to-move and virtual buttons | F-10.6.5 | R-10.6.5 |
| Hold-to-toggle conversion for sustained key presses | F-10.6.5 | R-10.6.5 |
| Per-character control schemes for MMO alts | F-10.6.5 | R-10.6.5 |

## US-10.6.14 Configure Input Accessibility for Multiple Characters

**As a** developer (P-15), **I want** input remapping stored per-character, **so that** MMO players
with multiple characters can use different control schemes for each alt without reconfiguring on
every login.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Input mappings saved per character profile | F-10.6.5 | R-10.6.5 |
| Character switch loads correct mapping automatically | F-10.6.5 | R-10.6.5 |
| All mapping changes persist across sessions | F-10.6.5 | R-10.6.5 |

## US-10.6.15 Verify Switch-Device Navigation Reaches All UI Elements

**As a** QA engineer (P-19), **I want** to verify that single-button scanning navigation reaches
every interactive UI element in the correct order, **so that** players using switch devices can
access all game functionality.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Scanning visits every interactive element | F-10.6.5 | R-10.6.5 |
| Scan order follows logical reading/navigation order | F-10.6.5 | R-10.6.5 |
| No element unreachable via switch input | F-10.6.5 | R-10.6.5 |

## US-10.6.16 Listen to Chat Messages Via Text-to-Speech

**As a** player (P-23) who is blind or has low vision, **I want** incoming chat messages and
notifications converted to speech with configurable voice, speed, and volume per channel,
**so that** I can follow conversations without reading the screen.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| TTS for incoming chat messages | F-10.6.6 | R-10.6.6 |
| Per-channel enable/disable (party, guild, whisper) | F-10.6.6 | R-10.6.6 |
| Configurable voice, speed, and volume | F-10.6.6 | R-10.6.6 |
| Speech-to-text for outgoing message composition | F-10.6.6 | R-10.6.6 |

## US-10.6.17 Implement Platform-Native TTS Integration

**As an** engine developer (P-26), **I want** to implement text-to-speech using AVSpeechSynthesizer
(macOS), SAPI/OneCore (Windows), and Speech Dispatcher (Linux), **so that** chat TTS uses native
platform voices with low-latency output.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| macOS TTS via AVSpeechSynthesizer | F-10.6.6 | R-10.6.6 |
| Windows TTS via SAPI/OneCore | F-10.6.6 | R-10.6.6 |
| Linux TTS via Speech Dispatcher | F-10.6.6 | R-10.6.6 |

## US-10.6.18 Verify TTS Channel Selection and Volume Controls

**As a** QA engineer (P-19), **I want** to verify that per-channel TTS enable/disable and volume
controls work correctly, **so that** players can selectively listen to the channels they care about
without unwanted speech output.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Enabling TTS on party channel speaks only party messages | F-10.6.6 | R-10.6.6 |
| Volume control per channel adjusts TTS volume independently | F-10.6.6 | R-10.6.6 |
| Disabling a channel stops TTS immediately for that channel | F-10.6.6 | R-10.6.6 |

## US-10.6.19 Meet WCAG 2.1 AA Compliance for All Menu UI

**As a** developer (P-15), **I want** all menu and settings UI to meet WCAG 2.1 AA compliance with
AAA targets for critical gameplay information, **so that** the game is accessible to the widest
possible player base.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| 4.5:1 contrast ratio for normal text | F-10.6.7 | R-10.6.7 |
| 3:1 contrast ratio for large text | F-10.6.7 | R-10.6.7 |
| All interactive elements keyboard-operable | F-10.6.7 | R-10.6.7 |
| Visible focus indicators on all focused elements | F-10.6.7 | R-10.6.7 |
| Timing adjustability for time-limited interactions | F-10.6.7 | R-10.6.7 |

## US-10.6.20 Audit Contrast Ratios Across All UI Themes

**As a** QA engineer (P-19), **I want** to audit contrast ratios across all color themes and
high-contrast mode to verify WCAG compliance, **so that** no text or interactive element falls below
the required contrast ratio in any theme.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Normal text meets 4.5:1 in all themes | F-10.6.7 | R-10.6.7 |
| Large text meets 3:1 in all themes | F-10.6.7 | R-10.6.7 |
| Focus indicators visible in all themes | F-10.6.7 | R-10.6.7 |
| High-contrast mode exceeds AA thresholds | F-10.6.7 | R-10.6.7 |

## US-10.6.21 Define Accessible Default Styles for All Widgets

**As a** technical artist (P-13), **I want** to define accessible default styles for every widget
type that meet WCAG contrast requirements while maintaining visual quality, **so that**
accessibility is built into the design system from the start.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Default styles meet WCAG AA contrast ratios | F-10.6.7 | R-10.6.7 |
| Focus indicators visible without clashing with design | F-10.6.7 | R-10.6.7 |
| Error identification clear and color-independent | F-10.6.7 | R-10.6.7 |
