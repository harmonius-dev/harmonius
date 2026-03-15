# R-14.1 — Window & Display Requirements

## Window Management

### R-14.1.1 Window Creation and Lifecycle

The engine **SHALL** create, resize, minimize, maximize, restore, and destroy native windows on
Windows, macOS, and Linux (X11 and Wayland) with a consistent cross-platform API, supporting a
primary game window and additional auxiliary windows.

- **Derived from:** [F-14.1.1](../../features/platform/window-display.md)
- **Rationale:** A unified window lifecycle API isolates platform-specific windowing code behind a
  single abstraction, letting gameplay, UI, and debug systems create and manage windows without
  platform branching.
- **Verification:** Integration test: on each platform, create a window, resize it, minimize,
  maximize, restore, and destroy it. Assert each state transition emits the correct lifecycle event
  and the window dimensions match the requested values after resize. Create an auxiliary window and
  verify it operates independently of the primary window.

### R-14.1.2 Fullscreen, Borderless, and Windowed Modes

The engine **SHALL** switch between exclusive fullscreen, borderless fullscreen, and windowed modes
at runtime without losing the GPU device context or requiring a swapchain re-creation that causes
visible flicker.

- **Derived from:** [F-14.1.2](../../features/platform/window-display.md)
- **Rationale:** Borderless fullscreen allows instant alt-tab without mode-switch delays, which is
  critical for players who frequently switch to external tools. Exclusive fullscreen enables lowest
  latency for competitive scenarios.
- **Verification:** Integration test: on each platform, cycle through windowed, borderless
  fullscreen, exclusive fullscreen, and back to windowed. Assert no GPU device loss occurs and the
  rendered frame is visible within two VSync intervals of each transition. Verify the backbuffer
  resolution matches the display resolution in fullscreen modes.

## Multi-Monitor

### R-14.1.3 Display Enumeration and Multi-Monitor Support

The engine **SHALL** enumerate all connected displays reporting resolution, refresh rate, color
depth, HDR capability, and physical position, and **SHALL** re-enumerate within one frame of a
display hot-plug or topology change event.

- **Derived from:** [F-14.1.3](../../features/platform/window-display.md)
- **Rationale:** Correct display enumeration ensures the game launches on the intended monitor and
  that streaming or debug windows can be placed on secondary displays by index or position.
- **Verification:** Integration test: on a multi-monitor system, enumerate displays and assert each
  reports valid resolution, refresh rate, and position. Simulate a hot-plug event (connect/disconnect
  a display) and verify re-enumeration fires within one frame. Assert the game window can be
  programmatically moved to each enumerated display.

### R-14.1.4 DPI Awareness and Display Scaling

The engine **SHALL** detect per-monitor DPI, respond to DPI changes when the window moves between
monitors, and scale UI elements and cursor hit regions correctly at fractional scale factors
(125%, 150%) without producing blurry text or misaligned hit regions.

- **Derived from:** [F-14.1.4](../../features/platform/window-display.md)
- **Rationale:** Modern displays range from 96 to 288+ DPI; UI elements and cursor targeting must
  scale correctly to remain readable and clickable at every scale factor.
- **Verification:** Integration test: on a dual-monitor system with different DPI values, drag the
  window between monitors and assert the reported DPI updates within one frame. Render UI text and
  buttons at 100%, 125%, 150%, and 200% scale; assert text is rendered at native resolution (no
  bilinear blur) and button hit regions match their visual bounds within 1 pixel.

## Presentation

### R-14.1.5 VSync and Refresh Rate Management

The engine **SHALL** support immediate (no VSync), FIFO (VSync on), and mailbox (triple-buffered)
presentation modes, and **SHALL** allow the player to cap the frame rate independently of the
display refresh rate.

- **Derived from:** [F-14.1.5](../../features/platform/window-display.md)
- **Rationale:** Immediate mode minimizes input latency for competitive play, FIFO eliminates
  tearing, and mailbox combines tear-free presentation with reduced latency. An independent frame
  rate cap lets players limit GPU power draw or match a specific target.
- **Verification:** Integration test: for each presentation mode, render 300 frames and measure
  frame pacing. Assert FIFO frames are delivered at VSync intervals (within 0.5 ms tolerance).
  Assert mailbox does not exhibit tearing. Set a 30 FPS frame cap on a 60 Hz display and assert
  frame intervals are 33.3 ms (within 1 ms tolerance).

### R-14.1.6 HDR Output and Tone Mapping Handoff

The engine **SHALL** enable HDR output when the display and OS support it, selecting the correct
color space and peak luminance metadata per platform, and accepting a linear HDR buffer from the
rendering pipeline for swapchain format negotiation.

- **Derived from:** [F-14.1.6](../../features/platform/window-display.md)
- **Rationale:** HDR output requires correct color space signaling and metadata negotiation with the
  OS compositor; incorrect configuration causes washed-out or clipped colors.
- **Verification:** Integration test: on an HDR-capable display, enable HDR output and assert the
  swapchain reports an HDR-compatible format (scRGB on Windows, extended linear sRGB on macOS).
  Render a test pattern with pixel values above 1.0 and verify they are not clipped in a screen
  capture. Verify peak luminance metadata matches the display's reported capability.
