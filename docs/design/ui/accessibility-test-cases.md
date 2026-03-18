# Accessibility Test Cases

Companion test cases for [accessibility.md](accessibility.md).

## Unit Tests

### TC-10.6.1.1 Accessible Node Creation

| # | Requirement |
|---|-------------|
| 1 | R-10.6.1    |

1. **#1** — Widget with role=Button, label="Submit"
   - **Expected:** AccessibleNode has role=Button, label="Submit"

### TC-10.6.1.2 Tree Sync Add Remove

| # | Requirement |
|---|-------------|
| 1 | R-10.6.1    |
| 2 | R-10.6.1    |

1. **#1** — Add widget with AccessibleRole to widget tree
   - **Expected:** AccessibilityTree gains new node, marked dirty
2. **#2** — Remove widget from widget tree
   - **Expected:** Node removed from accessibility tree, marked dirty

### TC-10.6.1.3 Live Region Announce

| # | Requirement |
|---|-------------|
| 1 | R-10.6.1    |

1. **#1** — `announce(entity, "HP low!", Assertive)`
   - **Expected:** Node marked dirty with LiveRegionMode::Assertive

### TC-10.6.1.4 Focus Tab Order

| # | Requirement |
|---|-------------|
| 1 | R-10.6.1    |

1. **#1** — 5 interactive widgets, press Tab 5 times
   - **Expected:** Focus visits all 5 widgets in order

### TC-10.6.2.1 Subtitle Timing

| # | Requirement |
|---|-------------|
| 1 | R-10.6.2    |
| 2 | R-10.6.2    |

1. **#1** — SubtitleEntry start=1.0, end=3.0, advance to t=2.0
   - **Expected:** Subtitle visible
2. **#2** — Advance to t=3.5
   - **Expected:** Subtitle removed from visible list

### TC-10.6.2.2 Caption Direction

| # | Requirement |
|---|-------------|
| 1 | R-10.6.2    |

1. **#1** — CaptionEntry with direction=Left
   - **Expected:** Caption displays left directional indicator

### TC-10.6.2.3 Subtitle Settings

| # | Requirement |
|---|-------------|
| 1 | R-10.6.2    |

1. **#1** — Change font_size from 16 to 24
   - **Expected:** Visible subtitles render at font_size=24

### TC-10.6.3.1 Protan Matrix

| # | Requirement |
|---|-------------|
| 1 | R-10.6.3    |

1. **#1** — Pure red (1.0, 0.0, 0.0) through protanopia matrix
   - **Expected:** Output is distinguishable non-red hue

### TC-10.6.3.2 Deutan Matrix

| # | Requirement |
|---|-------------|
| 1 | R-10.6.3    |

1. **#1** — Pure green (0.0, 1.0, 0.0) through deuteranopia matrix
   - **Expected:** Output is distinguishable non-green hue

### TC-10.6.3.3 Tritan Matrix

| # | Requirement |
|---|-------------|
| 1 | R-10.6.3    |

1. **#1** — Pure blue (0.0, 0.0, 1.0) through tritanopia matrix
   - **Expected:** Output is distinguishable non-blue hue

### TC-10.6.3.4 Alternative Cues

| # | Requirement |
|---|-------------|
| 1 | R-10.6.3    |

1. **#1** — Color-coded element with AlternativeCue attached
   - **Expected:** Non-color cue (pattern or shape_icon) present

### TC-10.6.7.1 Contrast Ratio AA

| # | Requirement |
|---|-------------|
| 1 | R-10.6.7    |

1. **#1** — `ContrastChecker::check(WHITE, BLACK)`
   - **Expected:** ratio=21.0, passes_aa_normal=true, passes_aaa_normal=true

### TC-10.6.7.2 Contrast Ratio Fail

| # | Requirement |
|---|-------------|
| 1 | R-10.6.7    |

1. **#1** — `ContrastChecker::check(Color(0.7,0.7,0.7), WHITE)`
   - **Expected:** ratio < 4.5, passes_aa_normal=false

### TC-10.6.4.1 High Contrast Borders

| # | Requirement |
|---|-------------|
| 1 | R-10.6.4    |

1. **#1** — Enable HighContrastSettings, border_width=3.0
   - **Expected:** All interactive widget borders >= 3.0 px

### TC-10.6.4.2 High Contrast No Transparency

| # | Requirement |
|---|-------------|
| 1 | R-10.6.4    |

1. **#1** — Enable HighContrastSettings with remove_decorative=true
   - **Expected:** All decorative transparency replaced with solid fills

### TC-10.6.4.3 UI Scale 80

| # | Requirement |
|---|-------------|
| 1 | R-10.6.4    |

1. **#1** — Set user_scale=0.8
   - **Expected:** All widgets scale to 80%, no layout overflow

### TC-10.6.4.4 UI Scale 250

| # | Requirement |
|---|-------------|
| 1 | R-10.6.4    |

1. **#1** — Set user_scale=2.5
   - **Expected:** All widgets scale to 250%, no content overflow

### TC-10.6.5.1 Rebind All Actions

| # | Requirement |
|---|-------------|
| 1 | R-10.6.5    |

1. **#1** — Rebind "Jump" from Space to Enter
   - **Expected:** "Jump" triggers on Enter press, not Space

### TC-10.6.5.2 Hold Toggle

| # | Requirement |
|---|-------------|
| 1 | R-10.6.5    |
| 2 | R-10.6.5    |

1. **#1** — Enable hold_toggle for "Sprint", press once
   - **Expected:** Sprint activates and stays active
2. **#2** — Press again
   - **Expected:** Sprint deactivates

### TC-10.6.5.3 Scanning Navigation

| # | Requirement |
|---|-------------|
| 1 | R-10.6.5    |

1. **#1** — Enable scanning with interval=500ms, 5 widgets
   - **Expected:** Scan visits all 5 interactive elements in sequence

### TC-10.6.5.4 Per Character Profile

| # | Requirement |
|---|-------------|
| 1 | R-10.6.5    |

1. **#1** — Save profile for character_id=1 with custom bindings, load for character_id=2
   - **Expected:** Each profile loads correct independent bindings

### TC-10.6.6.1 TTS Channel Filter

| # | Requirement |
|---|-------------|
| 1 | R-10.6.6    |
| 2 | R-10.6.6    |

1. **#1** — TTS enabled for Party channel only, Guild message arrives
   - **Expected:** No speech output for Guild message
2. **#2** — Party message arrives
   - **Expected:** Speech output generated

### TC-10.6.6.2 TTS Volume Control

| # | Requirement |
|---|-------------|
| 1 | R-10.6.6    |

1. **#1** — Set Party channel volume=0.5, speak message
   - **Expected:** TTS output at 50% volume

### TC-10.6.7.3 Reduced Motion No Shake

| # | Requirement |
|---|-------------|
| 1 | R-10.6.7    |

1. **#1** — Enable ReducedMotionSettings, trigger camera shake
   - **Expected:** CameraShakeOffset remains zero

### TC-10.6.7.4 Focus Indicator Visible

| # | Requirement |
|---|-------------|
| 1 | R-10.6.7    |

1. **#1** — Tab to each focusable widget
   - **Expected:** Focus indicator renders on every focused widget

## Integration Tests

### TC-10.6.1.I1 VoiceOver macOS

| # | Requirement |
|---|-------------|
| 1 | R-10.6.1    |

1. **#1** — Navigate all widget types with VoiceOver on macOS
   - **Expected:** All roles announced correctly

### TC-10.6.1.I2 Narrator Windows

| # | Requirement |
|---|-------------|
| 1 | R-10.6.1    |

1. **#1** — Navigate all widget types with Narrator on Windows
   - **Expected:** All roles announced correctly

### TC-10.6.1.I3 Orca Linux

| # | Requirement |
|---|-------------|
| 1 | R-10.6.1    |

1. **#1** — Navigate all widget types with Orca on Linux
   - **Expected:** All roles announced correctly

### TC-10.6.2.I1 Subtitle Audio Sync

| # | Requirement |
|---|-------------|
| 1 | R-10.6.2    |

1. **#1** — Play audio clip with subtitle track
   - **Expected:** Subtitles sync within 100 ms of audio playback

### TC-10.6.3.I1 Colorblind Preview

| # | Requirement |
|---|-------------|
| 1 | R-10.6.3    |

1. **#1** — Toggle CVD mode in settings
   - **Expected:** Real-time preview updates immediately

### TC-10.6.4.I1 DPI Detection

| # | Requirement |
|---|-------------|
| 1 | R-10.6.4    |

1. **#1** — Query DPI on each platform
   - **Expected:** Correct system DPI scale returned

### TC-10.6.5.I1 Switch Device Full UI

| # | Requirement |
|---|-------------|
| 1 | R-10.6.5    |

1. **#1** — Single-button scanning device
   - **Expected:** Reaches every UI element

### TC-10.6.6.I1 TTS Platform Voices

| # | Requirement |
|---|-------------|
| 1 | R-10.6.6    |

1. **#1** — Enumerate TTS voices on each platform
   - **Expected:** Returns non-empty list of native voices

### TC-10.6.7.I1 WCAG All Screens

| # | Requirement |
|---|-------------|
| 1 | R-10.6.7    |

1. **#1** — Automated WCAG AA audit on all menu/settings screens
   - **Expected:** All screens pass contrast and keyboard operability checks

## Benchmarks

### TC-10.6.1.B1 Accessibility Tree Sync

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Sync 200-widget tree | Sync time | < 0.5 ms | R-10.6.1 |

### TC-10.6.1.B2 Platform Bridge Push

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Push 50 dirty nodes to platform | Push time | < 1 ms | R-10.6.1 |

### TC-10.6.7.B1 Contrast Ratio Check

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Check 1000 color pairs | Total time | < 0.1 ms | R-10.6.7 |

### TC-10.6.3.B1 Colorblind Post-Process Pass

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Apply CVD matrix at 1080p | GPU time | < 0.3 ms | R-10.6.3 |

### TC-10.6.6.B1 TTS Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Message received to speech start | Latency | < 200 ms | R-10.6.6 |
