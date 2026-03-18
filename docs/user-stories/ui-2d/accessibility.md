# User Stories — 10.6 Accessibility

## Screen Reader and Assistive Technology

| ID        | Persona                 | Features | Requirements |
|-----------|-------------------------|----------|--------------|
| US-10.6.1 | Player (P-23)           | F-10.6.1 | R-10.6.1     |
| US-10.6.2 | Engine developer (P-26) | F-10.6.1 | R-10.6.1     |
| US-10.6.3 | QA engineer (P-19)      | F-10.6.1 | R-10.6.1     |

1. **US-10.6.1** — I want the widget tree exposed to platform accessibility APIs so screen readers
   announce widget labels, roles, states, and values, so that I can navigate all menus and interact
   with the game using assistive technology.
   - **Acceptance:** Every interactive widget has a programmatic name and role; navigation events
     and focus changes announced in real time; live region updates (chat, combat log, notifications)
     announced
2. **US-10.6.2** — I want to implement native accessibility bridges for NSAccessibility (macOS), UI
   Automation (Windows), and AT-SPI (Linux), so that the widget tree integrates with each platform's
   screen reader infrastructure.
   - **Acceptance:** macOS bridge via NSAccessibility; Windows bridge via UI Automation; Linux
     bridge via AT-SPI
3. **US-10.6.3** — I want to verify that every interactive widget is announced correctly by
   VoiceOver (macOS), Narrator (Windows), and Orca (Linux), so that no UI element is unreachable via
   screen reader navigation.
   - **Acceptance:** All buttons, sliders, checkboxes announced with label and role; state changes
     (checked/unchecked, enabled/disabled) announced; no interactive element missing from
     accessibility tree

## Subtitle and Caption System

| ID        | Persona            | Features | Requirements |
|-----------|--------------------|----------|--------------|
| US-10.6.4 | Player (P-23)      | F-10.6.2 | R-10.6.2     |
| US-10.6.5 | Designer (P-5)     | F-10.6.2 | R-10.6.2     |
| US-10.6.6 | QA engineer (P-19) | F-10.6.2 | R-10.6.2     |

1. **US-10.6.4** — I want configurable subtitles for dialogue and voice-over with closed captions
   for non-speech audio including directional indicators, so that I understand all audio information
   visually.
   - **Acceptance:** Subtitles with configurable font size, color, background opacity; speaker
     identification labels; closed captions for non-speech audio with directional indicators;
     subtitle positioning avoids obscuring gameplay elements
2. **US-10.6.5** — I want to configure subtitle and caption presentation including timing, speed,
   line count, and positioning, so that subtitles integrate well with the game's visual design while
   remaining readable.
   - **Acceptance:** Timing, speed, and line count configurable; positioning avoids important
     gameplay elements; system caption preferences read from iOS/Android APIs
3. **US-10.6.6** — I want to verify that subtitles and captions sync correctly with audio playback
   across all supported platforms, so that no subtitle appears early, late, or out of sequence.
   - **Acceptance:** Subtitle timing within 100ms of audio playback; caption sequence matches audio
     event order; subtitle speed adjustable without desync

## Visual Accessibility

| ID         | Persona                 | Features | Requirements |
|------------|-------------------------|----------|--------------|
| US-10.6.7  | Player (P-23)           | F-10.6.3 | R-10.6.3     |
| US-10.6.8  | Technical artist (P-13) | F-10.6.3 | R-10.6.3     |
| US-10.6.9  | QA engineer (P-19)      | F-10.6.3 | R-10.6.3     |
| US-10.6.10 | Player (P-23)           | F-10.6.4 | R-10.6.4     |
| US-10.6.11 | Engine developer (P-26) | F-10.6.4 | R-10.6.4     |
| US-10.6.12 | Engine tester (P-27)    | F-10.6.4 | R-10.6.4     |

1. **US-10.6.7** — I want simulation and correction modes for protanopia, deuteranopia, and
   tritanopia with alternative visual cues for color-coded information, so that I can perceive
   gameplay-critical information like item rarity and team colors.
   - **Acceptance:** Post-process filters for all three CVD types; alternative cues (patterns,
     shapes, icons, labels) for color-coded info; colorblind modes previewable in settings
2. **US-10.6.8** — I want to define alternative visual cues (patterns, shapes, icons) for each
   color-coded gameplay element, so that item rarity, team colors, and threat indicators are
   distinguishable without relying solely on color.
   - **Acceptance:** Pattern overlays for item rarity borders; shape-based indicators for team
     colors; icon-based threat level indicators
3. **US-10.6.9** — I want to verify that each colorblind mode provides sufficient visual
   differentiation for all gameplay-critical color-coded information, so that no player with CVD is
   at a disadvantage.
   - **Acceptance:** All color-coded elements distinguishable in each CVD mode; alternative cues
     present for every color-critical indicator; preview mode accurately simulates each CVD type
4. **US-10.6.10** — I want to scale the entire UI from 80% to 250% with all widgets, text, and icons
   scaling proportionally and high-contrast mode for stark color pairs, so that I can read and
   interact with the UI comfortably at any display size.
   - **Acceptance:** Continuous UI scale from 80% to 250%; all widgets, text, and icons scale
     proportionally; high-contrast mode increases borders, removes transparency; layout reflows
     correctly at extreme scales
5. **US-10.6.11** — I want to implement integration with system-level DPI and text scale settings on
   Windows, macOS, and Linux, so that the UI automatically respects the player's OS accessibility
   preferences.
   - **Acceptance:** Windows DPI via SetProcessDpiAwareness; macOS scale via backingScaleFactor;
     Linux DPI via Xft.dpi / Wayland scale
6. **US-10.6.12** — I want to verify that all UI screens layout correctly at 80%, 150%, 200%, and
   250% scale, so that no widget overflows, clips, or becomes unreachable at any scale factor.
   - **Acceptance:** No overflow or clipping at 80% scale; no overflow or clipping at 250% scale;
     all interactive elements remain reachable at every scale

## Input Accessibility

| ID         | Persona            | Features | Requirements |
|------------|--------------------|----------|--------------|
| US-10.6.13 | Player (P-23)      | F-10.6.5 | R-10.6.5     |
| US-10.6.14 | Developer (P-15)   | F-10.6.5 | R-10.6.5     |
| US-10.6.15 | QA engineer (P-19) | F-10.6.5 | R-10.6.5     |

1. **US-10.6.13** — I want complete remapping of all keyboard, mouse, and gamepad bindings including
   one-handed layouts, mouse-only play, and switch-device scanning navigation, so that I can play
   the game with my available input capabilities.
   - **Acceptance:** All bindings remappable including UI navigation; one-handed keyboard layouts
     supported; mouse-only play with click-to-move and virtual buttons; hold-to-toggle conversion
     for sustained key presses; per-character control schemes for MMO alts
2. **US-10.6.14** — I want input remapping stored per-character, so that MMO players with multiple
   characters can use different control schemes for each alt without reconfiguring on every login.
   - **Acceptance:** Input mappings saved per character profile; character switch loads correct
     mapping automatically; all mapping changes persist across sessions
3. **US-10.6.15** — I want to verify that single-button scanning navigation reaches every
   interactive UI element in the correct order, so that players using switch devices can access all
   game functionality.
   - **Acceptance:** Scanning visits every interactive element; scan order follows logical
     reading/navigation order; no element unreachable via switch input

## Text-to-Speech

| ID         | Persona                 | Features | Requirements |
|------------|-------------------------|----------|--------------|
| US-10.6.16 | Player (P-23)           | F-10.6.6 | R-10.6.6     |
| US-10.6.17 | Engine developer (P-26) | F-10.6.6 | R-10.6.6     |
| US-10.6.18 | QA engineer (P-19)      | F-10.6.6 | R-10.6.6     |

1. **US-10.6.16** — I want incoming chat messages and notifications converted to speech with
   configurable voice, speed, and volume per channel, so that I can follow conversations without
   reading the screen.
   - **Acceptance:** TTS for incoming chat messages; per-channel enable/disable (party, guild,
     whisper); configurable voice, speed, and volume; speech-to-text for outgoing message
     composition
2. **US-10.6.17** — I want to implement text-to-speech using AVSpeechSynthesizer (macOS),
   SAPI/OneCore (Windows), and Speech Dispatcher (Linux), so that chat TTS uses native platform
   voices with low-latency output.
   - **Acceptance:** macOS TTS via AVSpeechSynthesizer; Windows TTS via SAPI/OneCore; Linux TTS via
     Speech Dispatcher
3. **US-10.6.18** — I want to verify that per-channel TTS enable/disable and volume controls work
   correctly, so that players can selectively listen to the channels they care about without
   unwanted speech output.
   - **Acceptance:** Enabling TTS on party channel speaks only party messages; volume control per
     channel adjusts TTS volume independently; disabling a channel stops TTS immediately for that
     channel

## Standards Compliance

| ID         | Persona                 | Features | Requirements |
|------------|-------------------------|----------|--------------|
| US-10.6.19 | Developer (P-15)        | F-10.6.7 | R-10.6.7     |
| US-10.6.20 | QA engineer (P-19)      | F-10.6.7 | R-10.6.7     |
| US-10.6.21 | Technical artist (P-13) | F-10.6.7 | R-10.6.7     |

1. **US-10.6.19** — I want all menu and settings UI to meet WCAG 2.1 AA compliance with AAA targets
   for critical gameplay information, so that the game is accessible to the widest possible player
   base.
   - **Acceptance:** 4.5:1 contrast ratio for normal text; 3:1 contrast ratio for large text; all
     interactive elements keyboard-operable; visible focus indicators on all focused elements;
     timing adjustability for time-limited interactions
2. **US-10.6.20** — I want to audit contrast ratios across all color themes and high-contrast mode
   to verify WCAG compliance, so that no text or interactive element falls below the required
   contrast ratio in any theme.
   - **Acceptance:** Normal text meets 4.5:1 in all themes; large text meets 3:1 in all themes;
     focus indicators visible in all themes; high-contrast mode exceeds AA thresholds
3. **US-10.6.21** — I want to define accessible default styles for every widget type that meet WCAG
   contrast requirements while maintaining visual quality, so that accessibility is built into the
   design system from the start.
   - **Acceptance:** Default styles meet WCAG AA contrast ratios; focus indicators visible without
     clashing with design; error identification clear and color-independent
