# 14.1 — Window & Display

## Window Management

### F-14.1.1 Window Creation and Lifecycle

Create, resize, minimize, maximize, restore, and destroy native windows with consistent behavior
across platforms. The engine maintains a primary game window and supports additional auxiliary
windows for debug overlays, chat pop-outs, and streaming dashboards common in MMO workflows.

- **Requirements:** R-14.1.1
- **Dependencies:** None
- **Platform notes:** Windows uses `CreateWindowEx` with COM wrappers via cxx; macOS uses
  `NSWindow` via Objective-C++ cxx wrappers; Linux uses `xcb_create_window` (X11) or
  `wl_compositor_create_surface` (Wayland) via standard C FFI.

### F-14.1.2 Fullscreen, Borderless, and Windowed Modes

Switch between exclusive fullscreen, borderless fullscreen (desktop composition), and windowed
modes at runtime without losing GPU device context. Borderless fullscreen is the default for MMO
players who frequently alt-tab to browsers, Discord, or streaming tools.

- **Requirements:** R-14.1.2
- **Dependencies:** F-14.1.1
- **Platform notes:** Windows exclusive fullscreen uses `IDXGISwapChain::SetFullscreenState`;
  borderless sets `WS_POPUP` with monitor-sized geometry. macOS uses
  `NSWindow.toggleFullScreen` or `NSBorderlessWindowMask`. Linux Wayland uses
  `xdg_toplevel_set_fullscreen`; X11 uses `_NET_WM_STATE_FULLSCREEN`.

## Multi-Monitor

### F-14.1.3 Display Enumeration and Multi-Monitor Support

Enumerate connected displays, report resolution, refresh rate, color depth, HDR capability, and
physical position. Players can drag the game window between monitors or launch on a specific
display. Streaming setups with 2-3 monitors require correct display identification.

- **Requirements:** R-14.1.3
- **Dependencies:** F-14.1.1
- **Platform notes:** Windows uses `EnumDisplayMonitors` and `DXGI` output enumeration; macOS
  uses `NSScreen.screens` and `CGDisplayCopyDisplayMode`; Linux uses `xrandr` (X11) or
  `wl_output` events (Wayland). Display topology changes (hot-plug) must trigger re-enumeration.

### F-14.1.4 DPI Awareness and Display Scaling

Detect per-monitor DPI and respond to DPI changes when the window moves between monitors. UI
elements, cursor hot-spots, and render resolution scale correctly. Fractional scaling (125%, 150%)
must not produce blurry text or misaligned UI hit regions.

- **Requirements:** R-14.1.4
- **Dependencies:** F-14.1.1, F-14.1.3
- **Platform notes:** Windows uses `SetProcessDpiAwarenessContext(PER_MONITOR_AWARE_V2)` and
  handles `WM_DPICHANGED`; macOS uses `NSWindow.backingScaleFactor` (always integer 1x or 2x);
  Linux reads `Xft.dpi` (X11) or `wl_output.scale` (Wayland). Fractional scaling on Linux
  Wayland requires `wp_fractional_scale_v1`.

## Presentation

### F-14.1.5 VSync and Refresh Rate Management

Support multiple presentation modes: immediate (no VSync), FIFO (VSync on), mailbox (triple
buffered), and adaptive VSync. Allow the player to cap frame rate independently of the display
refresh rate. Correct VSync behavior prevents tearing during MMO raids where consistent frame
pacing is critical.

- **Requirements:** R-14.1.5
- **Dependencies:** F-14.1.1
- **Platform notes:** Vulkan exposes `VK_PRESENT_MODE_*` directly. Metal uses
  `CAMetalLayer.displaySyncEnabled` for VSync and manual frame pacing for mailbox-like behavior.
  Console platforms have dedicated flip-queue APIs with platform-specific NDA documentation.

### F-14.1.6 HDR Output and Tone Mapping Handoff

Enable HDR output when the display and OS support it, selecting the appropriate color space
(scRGB on Windows, EDR on macOS) and signaling peak luminance to the compositor. The rendering
pipeline hands off a linear HDR buffer; this feature handles swapchain format and metadata
negotiation.

- **Requirements:** R-14.1.6
- **Dependencies:** F-14.1.1, F-14.1.3
- **Platform notes:** Windows uses `DXGI_COLOR_SPACE_RGB_FULL_G2084_NONE_P2020` and
  `DXGI_HDR_METADATA_HDR10`; macOS uses `CAMetalLayer` with `CGColorSpace` set to extended
  linear sRGB and EDR headroom queries. Linux Wayland HDR is experimental via
  `wp_color_management_v1`. Console platforms require platform-specific HDR calibration UIs.
