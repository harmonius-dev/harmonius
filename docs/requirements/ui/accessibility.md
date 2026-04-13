# R-10.6 — Accessibility Requirements

## Screen Reader

1. **R-10.6.1** — The engine **SHALL** expose the widget tree to platform accessibility APIs
   (NSAccessibility, UI Automation, AT-SPI) so screen readers announce every interactive widget's
   name, role, state, and value, and deliver focus-change and live-region events in real time.
   - **Rationale:** Blind and low-vision players depend on screen readers; missing labels render the
     UI unusable.
   - **Verification:** Enable screen reader on each platform. Tab through every widget type. Assert
     correct name, role, state announced. Trigger live region update. Assert announced without focus
     change.

## Subtitles

2. **R-10.6.2** — The engine **SHALL** display subtitles with configurable font size, color,
   background opacity, speaker ID, positioning, timing, speed, and line count, plus closed captions
   with directional indicators for non-speech audio.
   - **Rationale:** Deaf and hard-of-hearing players rely on subtitles and captions.
   - **Verification:** Play cinematic with dialogue and non-speech audio. Assert subtitles with
     speaker labels and directional indicators. Adjust every configurable property and assert
     display updates.

## Visual Accessibility

3. **R-10.6.3** — The engine **SHALL** provide post-process color-remapping for protanopia,
   deuteranopia, and tritanopia, and supply alternative non-color cues (patterns, shapes, icons) for
   all color-coded gameplay information, with real-time preview in settings.
   - **Rationale:** ~8% of males have color vision deficiency; sole reliance on color excludes these
     players.
   - **Verification:** Enable each mode. Assert filter applied. Verify every color-coded element
     also has a non-color cue. Toggle in settings. Assert preview updates in real time.

4. **R-10.6.4** — The engine **SHALL** provide high-contrast mode with increased borders and removed
   decorative transparency, and continuous UI scaling from 80% to 250% with proportional reflow,
   respecting system DPI and text-size preferences.
   - **Rationale:** Low-vision players need high contrast and enlarged elements; broken layout at
     extreme scales negates the benefit.
   - **Verification:** Enable high-contrast. Assert increased borders. Set scale to 80%, 175%, 250%.
     Assert proportional scaling with no overlap or clip. Change OS DPI. Assert reflected on next
     launch.

## Input Accessibility

5. **R-10.6.5** — The engine **SHALL** allow complete remapping of all bindings including UI
   navigation, support one-handed layouts, mouse-only play, switch-device scanning, and
   hold-to-toggle conversion, with per-character binding profiles.
   - **Rationale:** Motor-impaired players may operate a single switch; without full remapping the
     game is inaccessible.
   - **Verification:** Remap every default binding. Assert all functions work. Navigate full UI with
     switch scanning. Configure hold-to-toggle. Assert activates on single press and deactivates on
     second. Create two character profiles. Assert each loads correctly.

## Text-to-Speech

6. **R-10.6.6** — The engine **SHALL** convert chat and notifications to speech using platform TTS
   engines with per-channel voice, speed, and volume, supporting selective enablement and platform
   STT for outgoing composition.
   - **Rationale:** Blind players cannot read chat; TTS converts text to an accessible auditory
     channel.
   - **Verification:** Enable TTS on party only. Send party and guild messages. Assert only party is
     spoken. Adjust settings. Assert audible changes. Activate STT. Dictate message. Assert it
     appears in input.

## WCAG Compliance

7. **R-10.6.7** — The engine **SHALL** meet WCAG 2.1 AA for all menu and settings UI and AAA for
   critical gameplay information, including 4.5:1 contrast for normal text, 3:1 for large text,
   keyboard operability, meaningful focus order, visible focus indicators, error identification, and
   adjustable timing for time-limited interactions.
   - **Rationale:** WCAG 2.1 is the de facto accessibility standard ensuring perceivable, operable,
     and understandable UI.
   - **Verification:** Run automated contrast checker on all text. Assert ratios. Tab through every
     element. Assert visible focus indicator in logical order. Trigger time-limited interaction.
     Assert timeout is adjustable.

## Reduced Motion

8. **R-10.6.8** — The engine **SHALL** provide reduced motion settings suppressing animations,
   parallax, and camera shake with configurable animation speed scaling.
   - **Rationale:** Players with vestibular conditions experience discomfort from excessive motion;
     reduced motion is a distinct accessibility need from high contrast and scaling.
   - **Verification:** Enable reduced motion. Assert UI animations are suppressed. Assert parallax
     layers scroll at uniform rate. Assert camera shake is disabled. Set animation scale to 0.5.
     Assert animations play at half speed.
