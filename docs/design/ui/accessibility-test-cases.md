# Accessibility Test Cases

Companion test cases for [accessibility.md](accessibility.md).

## Unit Tests

### TC-10.6.1.1 Accessible Node Creation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Widget with role=Button, label="Submit" | AccessibleNode has role=Button, label="Submit" | R-10.6.1 |

### TC-10.6.1.2 Tree Sync Add Remove

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add widget with AccessibleRole to widget tree | AccessibilityTree gains new node, marked dirty | R-10.6.1 |
| 2 | Remove widget from widget tree | Node removed from accessibility tree, marked dirty | R-10.6.1 |

### TC-10.6.1.3 Live Region Announce

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `announce(entity, "HP low!", Assertive)` | Node marked dirty with LiveRegionMode::Assertive | R-10.6.1 |

### TC-10.6.1.4 Focus Tab Order

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5 interactive widgets, press Tab 5 times | Focus visits all 5 widgets in order | R-10.6.1 |

### TC-10.6.2.1 Subtitle Timing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | SubtitleEntry start=1.0, end=3.0, advance to t=2.0 | Subtitle visible | R-10.6.2 |
| 2 | Advance to t=3.5 | Subtitle removed from visible list | R-10.6.2 |

### TC-10.6.2.2 Caption Direction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | CaptionEntry with direction=Left | Caption displays left directional indicator | R-10.6.2 |

### TC-10.6.2.3 Subtitle Settings

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Change font_size from 16 to 24 | Visible subtitles render at font_size=24 | R-10.6.2 |

### TC-10.6.3.1 Protan Matrix

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pure red (1.0, 0.0, 0.0) through protanopia matrix | Output is distinguishable non-red hue | R-10.6.3 |

### TC-10.6.3.2 Deutan Matrix

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pure green (0.0, 1.0, 0.0) through deuteranopia matrix | Output is distinguishable non-green hue | R-10.6.3 |

### TC-10.6.3.3 Tritan Matrix

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pure blue (0.0, 0.0, 1.0) through tritanopia matrix | Output is distinguishable non-blue hue | R-10.6.3 |

### TC-10.6.3.4 Alternative Cues

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Color-coded element with AlternativeCue attached | Non-color cue (pattern or shape_icon) present | R-10.6.3 |

### TC-10.6.7.1 Contrast Ratio AA

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `ContrastChecker::check(WHITE, BLACK)` | ratio=21.0, passes_aa_normal=true, passes_aaa_normal=true | R-10.6.7 |

### TC-10.6.7.2 Contrast Ratio Fail

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `ContrastChecker::check(Color(0.7,0.7,0.7), WHITE)` | ratio < 4.5, passes_aa_normal=false | R-10.6.7 |

### TC-10.6.4.1 High Contrast Borders

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enable HighContrastSettings, border_width=3.0 | All interactive widget borders >= 3.0 px | R-10.6.4 |

### TC-10.6.4.2 High Contrast No Transparency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enable HighContrastSettings with remove_decorative=true | All decorative transparency replaced with solid fills | R-10.6.4 |

### TC-10.6.4.3 UI Scale 80

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set user_scale=0.8 | All widgets scale to 80%, no layout overflow | R-10.6.4 |

### TC-10.6.4.4 UI Scale 250

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set user_scale=2.5 | All widgets scale to 250%, no content overflow | R-10.6.4 |

### TC-10.6.5.1 Rebind All Actions

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Rebind "Jump" from Space to Enter | "Jump" triggers on Enter press, not Space | R-10.6.5 |

### TC-10.6.5.2 Hold Toggle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enable hold_toggle for "Sprint", press once | Sprint activates and stays active | R-10.6.5 |
| 2 | Press again | Sprint deactivates | R-10.6.5 |

### TC-10.6.5.3 Scanning Navigation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enable scanning with interval=500ms, 5 widgets | Scan visits all 5 interactive elements in sequence | R-10.6.5 |

### TC-10.6.5.4 Per Character Profile

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Save profile for character_id=1 with custom bindings, load for character_id=2 | Each profile loads correct independent bindings | R-10.6.5 |

### TC-10.6.6.1 TTS Channel Filter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | TTS enabled for Party channel only, Guild message arrives | No speech output for Guild message | R-10.6.6 |
| 2 | Party message arrives | Speech output generated | R-10.6.6 |

### TC-10.6.6.2 TTS Volume Control

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set Party channel volume=0.5, speak message | TTS output at 50% volume | R-10.6.6 |

### TC-10.6.7.3 Reduced Motion No Shake

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enable ReducedMotionSettings, trigger camera shake | CameraShakeOffset remains zero | R-10.6.7 |

### TC-10.6.7.4 Focus Indicator Visible

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Tab to each focusable widget | Focus indicator renders on every focused widget | R-10.6.7 |

## Integration Tests

### TC-10.6.1.I1 VoiceOver macOS

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Navigate all widget types with VoiceOver on macOS | All roles announced correctly | R-10.6.1 |

### TC-10.6.1.I2 Narrator Windows

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Navigate all widget types with Narrator on Windows | All roles announced correctly | R-10.6.1 |

### TC-10.6.1.I3 Orca Linux

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Navigate all widget types with Orca on Linux | All roles announced correctly | R-10.6.1 |

### TC-10.6.2.I1 Subtitle Audio Sync

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Play audio clip with subtitle track | Subtitles sync within 100 ms of audio playback | R-10.6.2 |

### TC-10.6.3.I1 Colorblind Preview

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Toggle CVD mode in settings | Real-time preview updates immediately | R-10.6.3 |

### TC-10.6.4.I1 DPI Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Query DPI on each platform | Correct system DPI scale returned | R-10.6.4 |

### TC-10.6.5.I1 Switch Device Full UI

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Single-button scanning device | Reaches every UI element | R-10.6.5 |

### TC-10.6.6.I1 TTS Platform Voices

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enumerate TTS voices on each platform | Returns non-empty list of native voices | R-10.6.6 |

### TC-10.6.7.I1 WCAG All Screens

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Automated WCAG AA audit on all menu/settings screens | All screens pass contrast and keyboard operability checks | R-10.6.7 |

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
