# Platform Windowing Test Cases

Companion test cases for [windowing.md](windowing.md).

## Unit Tests

### TC-14.1.1.1 Window Config Default

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `WindowConfig::default()` | Size=1280x720, mode=Windowed, present=Fifo, HDR disabled, DPI=SystemScaled | R-14.1.1 |

### TC-14.1.4.1 Logical to Physical 100

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `LogicalSize(1280, 720).to_physical(1.0)` | `PhysicalSize(1280, 720)` | R-14.1.4 |

### TC-14.1.4.2 Logical to Physical 125

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `LogicalSize(1280, 720).to_physical(1.25)` | `PhysicalSize(1600, 900)` | R-14.1.4 |

### TC-14.1.4.3 Logical to Physical 150

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `LogicalSize(1280, 720).to_physical(1.5)` | `PhysicalSize(1920, 1080)` | R-14.1.4 |

### TC-14.1.4.4 Logical to Physical 200

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `LogicalSize(1280, 720).to_physical(2.0)` | `PhysicalSize(2560, 1440)` | R-14.1.4 |

### TC-14.1.6.1 HDR Config Disabled

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `HdrConfig::disabled()` | enabled=false, color_space=Srgb, peak_luminance_nits=80.0 | R-14.1.6 |

### TC-14.1.6.2 HDR Config Platform Default Windows

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `HdrConfig::platform_default(1000.0)` on Windows | color_space=ScRgb, peak=1000.0 | R-14.1.6 |

### TC-14.1.6.3 HDR Config Platform Default macOS

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `HdrConfig::platform_default(1000.0)` on macOS | color_space=ExtendedLinearSrgb, peak=1000.0 | R-14.1.6 |

### TC-14.1.6.4 HDR Config Platform Default Linux

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `HdrConfig::platform_default(1000.0)` on Linux | color_space=Bt2020Pq, peak=1000.0 | R-14.1.6 |

### TC-14.1.3.1 Display ID Equality

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `DisplayId(1) == DisplayId(1)` | true | R-14.1.3 |
| 2 | `DisplayId(1) == DisplayId(2)` | false | R-14.1.3 |

### TC-14.1.5.1 Present Mode Variants

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enumerate `PresentMode` variants | Exactly 3: Immediate, Fifo, Mailbox | R-14.1.5 |

### TC-14.1.5.2 Frame Rate Cap Variants

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `FrameRateCap::Capped(30)` vs `FrameRateCap::Uncapped` | Distinct values, not equal | R-14.1.5 |

## Integration Tests

### TC-14.1.1.I1 Window Lifecycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create window, resize to 1920x1080 | `Resized` event, dimensions = 1920x1080 | R-14.1.1 |
| 2 | Minimize window | `Minimized` event | R-14.1.1 |
| 3 | Maximize window | `Maximized` event | R-14.1.1 |
| 4 | Restore window | `Restored` event | R-14.1.1 |
| 5 | Destroy window | Window destroyed, no crash | R-14.1.1 |

### TC-14.1.1.I2 Auxiliary Window

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create primary + auxiliary window, resize auxiliary | Primary unaffected by auxiliary resize | R-14.1.1 |
| 2 | Destroy auxiliary first | Primary continues operating | R-14.1.1 |

### TC-14.1.2.I1 Fullscreen Cycle No Device Loss

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Windowed -> BorderlessFullscreen -> ExclusiveFullscreen -> Windowed | No GPU device loss at any transition | R-14.1.2 |
| 2 | Check backbuffer resolution in fullscreen | Matches display resolution | R-14.1.2 |

### TC-14.1.2.I2 Borderless Alt Tab

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | In borderless fullscreen, simulate Alt+Tab | Window loses focus without mode change | R-14.1.2 |
| 2 | Return to app | Window regains focus | R-14.1.2 |

### TC-14.1.3.I1 Display Enumeration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enumerate displays on multi-monitor system | Each reports valid resolution > 0x0, refresh > 0, at least one primary | R-14.1.3 |

### TC-14.1.3.I2 Display Hot Plug

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Simulate display connect/disconnect | Re-enumeration fires within one frame, list updates correctly | R-14.1.3 |

### TC-14.1.4.I1 DPI Change Multi Monitor

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Drag window from 96 DPI to 192 DPI monitor | `DpiChanged` event with correct old/new scale factors | R-14.1.4 |
| 2 | Check `current_dpi()` after move | Updated within one frame | R-14.1.4 |

### TC-14.1.4.I2 Fractional DPI Scaling

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Render UI at 125%, 150%, 200% scale | Text at native resolution (no bilinear blur), hit regions within 1 px | R-14.1.4 |

### TC-14.1.5.I1 VSync FIFO Frame Pacing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | PresentMode::Fifo, render 300 frames | Frame intervals match VSync period within 0.5 ms | R-14.1.5 |

### TC-14.1.5.I2 VSync Mailbox No Tearing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | PresentMode::Mailbox, render 300 frames | No tearing artifacts (scanline detection) | R-14.1.5 |

### TC-14.1.5.I3 Frame Rate Cap 30 FPS

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `FrameRateCap::Capped(30)` on 60 Hz display, 300 frames | Frame intervals = 33.3 ms within 1 ms tolerance | R-14.1.5 |

### TC-14.1.6.I1 HDR Swapchain Format

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enable HDR on HDR-capable display | Swapchain reports HDR format (FP16/scRGB or extended linear) | R-14.1.6 |

### TC-14.1.6.I2 HDR Above 1 Not Clipped

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Render test pattern with values > 1.0, HDR enabled | Output values not clipped to 1.0 | R-14.1.6 |

### TC-14.1.6.I3 HDR Unsupported Display

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Call `enable_hdr()` on SDR-only display | Returns `HdrError::DisplayNotHdrCapable` | R-14.1.6 |

## Benchmarks

### TC-14.1.1.B1 Window Creation Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Create native window from config | Creation time | < 50 ms | R-14.1.1 |

### TC-14.1.2.B1 Fullscreen Transition Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Windowed to borderless fullscreen | Transition time | < 2 VSync intervals | R-14.1.2 |

### TC-14.1.4.B1 DPI Change Response Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Window dragged to new-DPI monitor | DPI update latency | < 1 frame | R-14.1.4 |

### TC-14.1.1.B2 Event Polling Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Synthetic event flood | Events processed per frame | > 10,000 | R-14.1.1 |

### TC-14.1.3.B1 Display Re-Enumeration Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Display hot-plug event | Re-enumeration time | < 1 frame | R-14.1.3 |
