# 10.6 — Accessibility

## Screen Reader and Assistive Technology

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|---|---|---|---|---|---|
| F-10.6.1 | Screen Reader Support | Expose the widget tree to platform accessibility APIs so that screen readers can announce widget labels, roles, states, and values. Every interactive widget must have a programmatic name (label or aria-equivalent) and role (button, checkbox, slider, list item, text input). Navigation events, focus changes, and live region updates (chat messages, combat log entries, notification toasts) are announced in real time. Essential for blind and low-vision players. | R-10.6.1 | F-10.1.1, F-10.1.7 | Uses NSAccessibility on macOS, UI Automation on Windows, and AT-SPI on Linux. Each platform requires a native accessibility bridge. |
| F-10.6.2 | Subtitle and Caption System | Display subtitles for dialogue, voice-over, and cinematics with configurable font size, color, background opacity, and speaker identification. Closed captions include non-speech audio descriptions (footsteps, explosions, environmental sounds) with directional indicators. Support subtitle positioning to avoid obscuring important gameplay elements, and allow players to customize timing, speed, and line count. | R-10.6.2 | F-10.2.1, F-10.1.5 | iOS uses UIAccessibility caption APIs; Android uses CaptioningManager for system caption preferences. Mobile defaults to larger subtitle font sizes for readability. |

## Visual Accessibility

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|---|---|---|---|---|---|
| F-10.6.3 | Colorblind Modes | Provide simulation and correction modes for the three major types of color vision deficiency: protanopia, deuteranopia, and tritanopia. Apply color remapping as a post-process filter and also provide alternative visual cues (patterns, shapes, icons, labels) for gameplay-critical color-coded information (item rarity, team colors, threat indicators, resource node types). Colorblind modes must be previewable in settings. | R-10.6.3 | F-10.1.5 | None |
| F-10.6.4 | High Contrast and Scalable UI | Support a high-contrast mode that increases border widths, uses stark foreground/background color pairs, and removes decorative transparency and gradients. UI scale is continuously adjustable from 80% to 250% with all widgets, text, and icons scaling proportionally. Layout must reflow correctly at extreme scales. Support system-level text size and DPI preferences on all platforms. | R-10.6.4 | F-10.1.3, F-10.1.5, F-10.4.2 | Reads system DPI and text scale settings on Windows (SetProcessDpiAwareness), macOS (backingScaleFactor), and Linux (Xft.dpi / Wayland scale). |

## Input Accessibility

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|---|---|---|---|---|---|
| F-10.6.5 | Input Remapping for Accessibility | Allow complete remapping of all keyboard, mouse, and gamepad bindings, including UI navigation controls. Support one-handed keyboard layouts, mouse-only play (click-to-move, on-screen virtual buttons), and switch-device input (single-button scanning navigation). Provide input hold-to-toggle conversion for abilities that normally require sustained key presses. Must be configurable per-character for MMO players with multiple characters using different control schemes. | R-10.6.5 | F-10.1.7 | Switch device support may require platform-specific HID drivers. |
| F-10.6.6 | Text-to-Speech for Chat | Convert incoming chat messages and system notifications to speech using platform text-to-speech engines, with configurable voice, speed, and volume per chat channel. Players can selectively enable TTS for specific channels (party, guild, whisper) while keeping others silent. Outgoing message composition can use speech-to-text where the platform provides it. | R-10.6.6 | F-10.3.8, F-10.6.1 | Uses AVSpeechSynthesizer on macOS, SAPI/OneCore on Windows, and Speech Dispatcher on Linux. |

## Standards Compliance

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|---|---|---|---|---|---|
| F-10.6.7 | WCAG Compliance Targets | Target WCAG 2.1 AA compliance for all menu and settings UI, with AAA targets for critical gameplay information (health, objectives, chat). Compliance areas include minimum contrast ratios (4.5:1 for normal text, 3:1 for large text), keyboard operability of all interactive elements, meaningful focus order, visible focus indicators, error identification, and timing adjustability for time-limited UI interactions (ready checks, loot rolls, trade confirmations). | R-10.6.7 | F-10.6.1, F-10.6.3, F-10.6.4, F-10.6.5 | None |
