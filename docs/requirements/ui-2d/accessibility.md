# R-10.6 — Accessibility Requirements

## R-10.6.1 Screen Reader Support

The engine **SHALL** expose the widget tree to platform accessibility APIs (NSAccessibility on
macOS, UI Automation on Windows, AT-SPI on Linux) so that screen readers can announce every
interactive widget's programmatic name, role (button, checkbox, slider, list item, text input),
state, and value, and shall deliver focus-change, navigation, and live-region update events
(chat messages, combat log entries, notification toasts) to assistive technology in real time.

- **Derived from:** [F-10.6.1](../../features/ui-2d/accessibility.md)
- **Rationale:** Blind and low-vision players depend on screen readers to perceive and interact
  with the UI; missing labels or silent state changes render the application unusable.
- **Verification:** Enable a screen reader on each supported platform, tab through every
  interactive widget type, and confirm the reader announces the correct name, role, and state;
  trigger a live-region update and confirm it is announced without manual focus change.

## R-10.6.2 Subtitle and Caption System

The engine **SHALL** display subtitles for dialogue, voice-over, and cinematics with
player-configurable font size, color, background opacity, speaker identification, positioning,
timing, speed, and line count, and shall display closed captions for non-speech audio
(footsteps, explosions, environmental sounds) with directional indicators.

- **Derived from:** [F-10.6.2](../../features/ui-2d/accessibility.md)
- **Rationale:** Deaf and hard-of-hearing players rely on subtitles and captions to receive
  auditory information; configurability ensures readability across display sizes and preferences.
- **Verification:** Play a cinematic containing dialogue and non-speech audio; confirm subtitles
  render with correct speaker labels and that captions include directional indicators; adjust
  every configurable property and verify the display updates accordingly.

## R-10.6.3 Colorblind Modes

The engine **SHALL** provide post-process color-remapping filters for protanopia, deuteranopia,
and tritanopia, and shall supply alternative non-color visual cues (patterns, shapes, icons, or
labels) for all gameplay-critical color-coded information including item rarity, team colors,
threat indicators, and resource node types, with a real-time preview in the settings UI.

- **Derived from:** [F-10.6.3](../../features/ui-2d/accessibility.md)
- **Rationale:** Approximately 8% of males have some form of color vision deficiency; relying
  solely on color to convey information excludes these players from critical gameplay feedback.
- **Verification:** Enable each colorblind mode and confirm the post-process filter is applied;
  verify that every color-coded gameplay element also carries a distinguishable non-color cue;
  toggle the mode in settings and confirm the preview updates in real time.

## R-10.6.4 High Contrast and Scalable UI

The engine **SHALL** provide a high-contrast mode that increases border widths, applies stark
foreground/background color pairs, and removes decorative transparency and gradients, and shall
support continuous UI scaling from 80% to 250% where all widgets, text, and icons scale
proportionally with correct layout reflow, respecting system-level DPI and text-size preferences
on Windows, macOS, and Linux.

- **Derived from:** [F-10.6.4](../../features/ui-2d/accessibility.md)
- **Rationale:** Low-vision players need high contrast and enlarged UI elements to perceive
  interface content; broken layout at extreme scales negates the benefit of scaling.
- **Verification:** Enable high-contrast mode and confirm increased border widths and removal of
  decorative transparency; set UI scale to 80%, 100%, 175%, and 250% and verify all widgets
  scale proportionally with no overlapping or clipped elements; change the OS text-size or DPI
  setting and confirm the engine reflects it on next launch.

## R-10.6.5 Input Remapping for Accessibility

The engine **SHALL** allow complete remapping of all keyboard, mouse, and gamepad bindings
including UI navigation, support one-handed keyboard layouts, mouse-only play with click-to-move
and on-screen virtual buttons, switch-device single-button scanning navigation, and hold-to-toggle
conversion for sustained key presses, with per-character binding profiles.

- **Derived from:** [F-10.6.5](../../features/ui-2d/accessibility.md)
- **Rationale:** Motor-impaired players may operate a single switch device or one-handed layout;
  without full remapping and alternative input modes the game is physically inaccessible.
- **Verification:** Remap every default binding to a different key and confirm all functions
  remain operational; navigate the full UI using only a single switch device in scanning mode;
  configure hold-to-toggle for a sustained-press ability and confirm it activates on a single
  press and deactivates on a second press; create two character profiles with different bindings
  and confirm each loads correctly.

## R-10.6.6 Text-to-Speech for Chat

The engine **SHALL** convert incoming chat messages and system notifications to speech using
platform TTS engines (AVSpeechSynthesizer on macOS, SAPI/OneCore on Windows, Speech Dispatcher
on Linux) with per-channel configurable voice, speed, and volume, allow selective TTS enablement
per chat channel, and support platform speech-to-text for outgoing message composition where
available.

- **Derived from:** [F-10.6.6](../../features/ui-2d/accessibility.md)
- **Rationale:** Blind players cannot read chat text; TTS converts textual communication to an
  accessible auditory channel, and STT allows them to compose messages without a visual editor.
- **Verification:** Enable TTS on the party channel only, send a message in party and guild
  channels, and confirm only the party message is spoken; adjust voice, speed, and volume settings
  and confirm audible changes; activate STT, dictate a message, and confirm it appears in the
  chat input field.

## R-10.6.7 WCAG Compliance Targets

The engine **SHALL** meet WCAG 2.1 AA compliance for all menu and settings UI and AAA compliance
for critical gameplay information (health, objectives, chat), including minimum contrast ratios
of 4.5:1 for normal text and 3:1 for large text, keyboard operability of all interactive
elements, meaningful focus order, visible focus indicators, error identification, and adjustable
timing for all time-limited UI interactions (ready checks, loot rolls, trade confirmations).

- **Derived from:** [F-10.6.7](../../features/ui-2d/accessibility.md)
- **Rationale:** WCAG 2.1 is the de facto accessibility standard; meeting AA/AAA targets ensures
  the UI is perceivable, operable, and understandable for the widest range of players.
- **Verification:** Run an automated contrast-ratio checker against every text element and confirm
  4.5:1 for normal text and 3:1 for large text; tab through every interactive element and confirm
  a visible focus indicator follows a logical order; trigger every time-limited interaction and
  confirm the timeout is adjustable or extendable.
