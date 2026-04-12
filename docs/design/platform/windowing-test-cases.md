# Platform Windowing Test Cases

Companion test cases for [windowing.md](windowing.md).

## Unit Tests

### TC-14.1.1.1 Window Config Default

| # | Requirement |
|---|-------------|
| 1 | R-14.1.1    |

1. **#1** — `WindowConfig::default()`
   - **Expected:** Size=1280x720, mode=Windowed, present=Fifo, HDR disabled, DPI=SystemScaled

### TC-14.1.4.1 Logical to Physical 100

| # | Requirement |
|---|-------------|
| 1 | R-14.1.4    |

1. **#1** — `LogicalSize(1280, 720).to_physical(1.0)`
   - **Expected:** `PhysicalSize(1280, 720)`

### TC-14.1.4.2 Logical to Physical 125

| # | Requirement |
|---|-------------|
| 1 | R-14.1.4    |

1. **#1** — `LogicalSize(1280, 720).to_physical(1.25)`
   - **Expected:** `PhysicalSize(1600, 900)`

### TC-14.1.4.3 Logical to Physical 150

| # | Requirement |
|---|-------------|
| 1 | R-14.1.4    |

1. **#1** — `LogicalSize(1280, 720).to_physical(1.5)`
   - **Expected:** `PhysicalSize(1920, 1080)`

### TC-14.1.4.4 Logical to Physical 200

| # | Requirement |
|---|-------------|
| 1 | R-14.1.4    |

1. **#1** — `LogicalSize(1280, 720).to_physical(2.0)`
   - **Expected:** `PhysicalSize(2560, 1440)`

### TC-14.1.6.1 HDR Config Disabled

| # | Requirement |
|---|-------------|
| 1 | R-14.1.6    |

1. **#1** — `HdrConfig::disabled()`
   - **Expected:** enabled=false, color_space=Srgb, peak_luminance_nits=80.0

### TC-14.1.6.2 HDR Config Platform Default Windows

| # | Requirement |
|---|-------------|
| 1 | R-14.1.6    |

1. **#1** — `HdrConfig::platform_default(1000.0)` on Windows
   - **Expected:** color_space=ScRgb, peak=1000.0

### TC-14.1.6.3 HDR Config Platform Default macOS

| # | Requirement |
|---|-------------|
| 1 | R-14.1.6    |

1. **#1** — `HdrConfig::platform_default(1000.0)` on macOS
   - **Expected:** color_space=ExtendedLinearSrgb, peak=1000.0

### TC-14.1.6.4 HDR Config Platform Default Linux

| # | Requirement |
|---|-------------|
| 1 | R-14.1.6    |

1. **#1** — `HdrConfig::platform_default(1000.0)` on Linux
   - **Expected:** color_space=Bt2020Pq, peak=1000.0

### TC-14.1.3.1 Display ID Equality

| # | Requirement |
|---|-------------|
| 1 | R-14.1.3    |
| 2 | R-14.1.3    |

1. **#1** — `DisplayId(1) == DisplayId(1)`
   - **Expected:** true
2. **#2** — `DisplayId(1) == DisplayId(2)`
   - **Expected:** false

### TC-14.1.5.1 Present Mode Variants

| # | Requirement |
|---|-------------|
| 1 | R-14.1.5    |

1. **#1** — Enumerate `PresentMode` variants
   - **Expected:** Exactly 3: Immediate, Fifo, Mailbox

### TC-14.1.5.2 Frame Rate Cap Variants

| # | Requirement |
|---|-------------|
| 1 | R-14.1.5    |

1. **#1** — `FrameRateCap::Capped(30)` vs `FrameRateCap::Uncapped`
   - **Expected:** Distinct values, not equal

### TC-14.1.9.1 Event Channel Auto Size Capacity

| # | Requirement |
|---|-------------|
| 1 | R-14.1.9    |
| 2 | R-14.1.9    |

1. **#1** — Push events at slow rate (10/s) for 1 s into bounded channel
   - **Expected:** Channel capacity remains at minimum baseline (e.g., 64), no resize
2. **#2** — Push burst of 10,000 events in 16 ms (rapid resize storm)
   - **Expected:** Auto-size grows capacity to absorb burst without dropping events; later decays
     back toward baseline when rate falls

## Integration Tests

### TC-14.1.1.I1 Window Lifecycle

| # | Requirement |
|---|-------------|
| 1 | R-14.1.1    |
| 2 | R-14.1.1    |
| 3 | R-14.1.1    |
| 4 | R-14.1.1    |
| 5 | R-14.1.1    |

1. **#1** — Create window, resize to 1920x1080
   - **Expected:** `Resized` event, dimensions = 1920x1080
2. **#2** — Minimize window
   - **Expected:** `Minimized` event
3. **#3** — Maximize window
   - **Expected:** `Maximized` event
4. **#4** — Restore window
   - **Expected:** `Restored` event
5. **#5** — Destroy window
   - **Expected:** Window destroyed, no crash

### TC-14.1.1.I2 Auxiliary Window

| # | Requirement |
|---|-------------|
| 1 | R-14.1.1    |
| 2 | R-14.1.1    |

1. **#1** — Create primary + auxiliary window, resize auxiliary
   - **Expected:** Primary unaffected by auxiliary resize
2. **#2** — Destroy auxiliary first
   - **Expected:** Primary continues operating

### TC-14.1.2.I1 Fullscreen Cycle No Device Loss

| # | Requirement |
|---|-------------|
| 1 | R-14.1.2    |
| 2 | R-14.1.2    |

1. **#1** — Windowed -> BorderlessFullscreen -> ExclusiveFullscreen -> Windowed
   - **Expected:** No GPU device loss at any transition
2. **#2** — Check backbuffer resolution in fullscreen
   - **Expected:** Matches display resolution

### TC-14.1.2.I2 Borderless Alt Tab

| # | Requirement |
|---|-------------|
| 1 | R-14.1.2    |
| 2 | R-14.1.2    |

1. **#1** — In borderless fullscreen, simulate Alt+Tab
   - **Expected:** Window loses focus without mode change
2. **#2** — Return to app
   - **Expected:** Window regains focus

### TC-14.1.3.I1 Display Enumeration

| # | Requirement |
|---|-------------|
| 1 | R-14.1.3    |

1. **#1** — Enumerate displays on multi-monitor system
   - **Expected:** Each reports valid resolution > 0x0, refresh > 0, at least one primary

### TC-14.1.3.I2 Display Hot Plug

| # | Requirement |
|---|-------------|
| 1 | R-14.1.3    |

1. **#1** — Simulate display connect/disconnect
   - **Expected:** Re-enumeration fires within one frame, list updates correctly

### TC-14.1.4.I1 DPI Change Multi Monitor

| # | Requirement |
|---|-------------|
| 1 | R-14.1.4    |
| 2 | R-14.1.4    |

1. **#1** — Drag window from 96 DPI to 192 DPI monitor
   - **Expected:** `DpiChanged` event with correct old/new scale factors
2. **#2** — Check `current_dpi()` after move
   - **Expected:** Updated within one frame

### TC-14.1.4.I2 Fractional DPI Scaling

| # | Requirement |
|---|-------------|
| 1 | R-14.1.4    |

1. **#1** — Render UI at 125%, 150%, 200% scale
   - **Expected:** Text at native resolution (no bilinear blur), hit regions within 1 px

### TC-14.1.5.I1 VSync FIFO Frame Pacing

| # | Requirement |
|---|-------------|
| 1 | R-14.1.5    |

1. **#1** — PresentMode::Fifo, render 300 frames
   - **Expected:** Frame intervals match VSync period within 0.5 ms

### TC-14.1.5.I2 VSync Mailbox No Tearing

| # | Requirement |
|---|-------------|
| 1 | R-14.1.5    |

1. **#1** — PresentMode::Mailbox, render 300 frames
   - **Expected:** No tearing artifacts (scanline detection)

### TC-14.1.5.I3 Frame Rate Cap 30 FPS

| # | Requirement |
|---|-------------|
| 1 | R-14.1.5    |

1. **#1** — `FrameRateCap::Capped(30)` on 60 Hz display, 300 frames
   - **Expected:** Frame intervals = 33.3 ms within 1 ms tolerance

### TC-14.1.6.I1 HDR Swapchain Format

| # | Requirement |
|---|-------------|
| 1 | R-14.1.6    |

1. **#1** — Enable HDR on HDR-capable display
   - **Expected:** Swapchain reports HDR format (FP16/scRGB or extended linear)

### TC-14.1.6.I2 HDR Above 1 Not Clipped

| # | Requirement |
|---|-------------|
| 1 | R-14.1.6    |

1. **#1** — Render test pattern with values > 1.0, HDR enabled
   - **Expected:** Output values not clipped to 1.0

### TC-14.1.6.I3 HDR Unsupported Display

| # | Requirement |
|---|-------------|
| 1 | R-14.1.6    |

1. **#1** — Call `enable_hdr()` on SDR-only display
   - **Expected:** Returns `HdrError::DisplayNotHdrCapable`

<!-- THIN: design section lacks detail -->
### TC-14.1.2.I3 US Fullscreen Mode Switching

| # | Requirement |
|---|-------------|
| 1 | US-14.1.2  |

1. **#1** — User toggles between Windowed, BorderlessFullscreen, ExclusiveFullscreen via API
   - **Expected:** Each transition succeeds, swapchain reconfigured, window state matches request
2. **#2** — Drag window to second monitor, switch to fullscreen
   - **Expected:** Fullscreen claims target monitor, no device loss

<!-- THIN: design section lacks detail -->
### TC-14.1.7.I1 US Window Creation And Event Throughput

| # | Requirement |
|---|-------------|
| 1 | US-14.1.7  |

1. **#1** — Create native window via `Window::new(WindowConfig::default())`
   - **Expected:** Window appears, latency < 50 ms wall time
2. **#2** — Generate 10,000 input events in one frame, drain channel
   - **Expected:** All 10,000 events delivered without drop within frame budget

<!-- THIN: design section lacks detail -->
### TC-14.1.8.I1 US Always On Top Overlay Window

| # | Requirement |
|---|-------------|
| 1 | US-14.1.8  |

1. **#1** — Create auxiliary always-on-top transparent window via
   `WindowConfig { always_on_top: true, transparent: true, .. }`
   - **Expected:** Window stays above other apps, transparent regions click-through
2. **#2** — Render an FPS counter into the overlay
   - **Expected:** Visible above the primary window without stealing focus

<!-- THIN: design section lacks detail -->
### TC-14.1.13.I1 US Console Presentation Path

| # | Requirement |
|---|-------------|
| 1 | US-14.1.13 |

1. **#1** — On a console build target, initialize platform presentation controller
   - **Expected:** Platform-specific flip queue path selected, swapchain valid
2. **#2** — Submit a frame, query present completion
   - **Expected:** Present succeeds within VSync budget, no validation errors

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
