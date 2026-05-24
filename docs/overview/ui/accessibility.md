# Accessibility

Screen readers, colorblind modes, and inclusive design.

## What it covers

- Screen readers: text-to-speech for UI navigation.
- Color blindness: simulation and alternative palettes (deuteranopia, protanopia, tritanopia).
- Contrast: ensuring text readability against backgrounds.
- Font sizes and readability: adjustable text size.
- Keyboard navigation: full UI and game control via keyboard.
- Captions and subtitles: text representation of audio (dialogue, sound effects).
- Visual indicators: supplementing color with shapes, patterns, icons.
- Remappable controls: customizing key bindings and sensitivity.
- Difficulty options: adjustable challenge and assistance.
- Audio descriptions: narration explaining important visuals.

## Concepts

### Screen Reader Support

Screen readers use accessibility APIs to query UI structure and read text aloud. Widget labels,
button text, and instructions become audio. Navigation reads menu items in order. Developers expose
widget purpose and state via accessibility APIs; screen reader users navigate and interact using
spoken labels.

### Color Blindness Accommodations

Deuteranopia (green-blind), protanopia (red-blind), and tritanopia (blue-blind) affect color
perception. Accessibility modes simulate these conditions and provide alternative color palettes
using distinguishable colors (blues and yellows instead of reds and greens). Game UI avoids
conveying critical information via color alone; shapes, patterns, and icons supplement color.

### Contrast and Readability

Text contrast (ratio of text to background luminance) ensures readability. WCAG standards recommend
4.5:1 for normal text, 3:1 for large text. Font sizes scale with accessibility settings; players
adjust text size to comfortable reading. Anti-aliased fonts and sufficient line spacing aid
readability.

### Keyboard Navigation and Captions

Full keyboard navigation enables playing without mouse or controller. Tab moves focus; arrow keys
navigate menus. Game actions bind to keyboard shortcuts (WASD for movement, Spacebar for jump).
Captions display dialogue and sound effect descriptions: "[gunshot]", "[footsteps approaching]".
Captions aid deaf players and situational audio (muted environments).

### Difficulty and Assistance

Difficulty options adjust challenge: easier reduces enemy health and accuracy; harder increases
both. Assistance options auto-aim, show waypoints, or reduce time pressure. These benefit players
with disabilities (motor control, reaction time) and less-experienced players.

## How it fits

- See [widgets-and-framework.md](./widgets-and-framework.md) for UI accessibility APIs.
- See [hud-and-2d.md](./hud-and-2d.md) for HUD design principles and information presentation.
- Integrates with [../audio/music-and-voice.md](../audio/music-and-voice.md) for caption
  generation.
