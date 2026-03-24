# 14.1 — Window & Display

## Window Management

| ID       | Feature                                    | Requirements |
|----------|--------------------------------------------|--------------|
| F-14.1.1 | Window Creation and Lifecycle              | R-14.1.1     |
| F-14.1.2 | Fullscreen, Borderless, and Windowed Modes | R-14.1.2     |

1. **F-14.1.1** — Create, resize, minimize, maximize, restore, and destroy native windows with
   consistent behavior across platforms. The engine maintains a primary game window and supports
   additional auxiliary windows for debug overlays, chat pop-outs, and streaming dashboards common
   in MMO workflows.
   - **Platform:** Windows uses `CreateWindowEx` with COM wrappers via C ABI; macOS uses `NSWindow`
     via Swift C ABI wrappers; Linux uses `xcb_create_window` (X11) or
     `wl_compositor_create_surface` (Wayland) via standard C FFI.
2. **F-14.1.2** — Switch between exclusive fullscreen, borderless fullscreen (desktop composition),
   and windowed modes at runtime without losing GPU device context. Borderless fullscreen is the
   default for MMO players who frequently alt-tab to browsers, Discord, or streaming tools.
   - **Deps:** F-14.1.1
   - **Platform:** Windows exclusive fullscreen uses `IDXGISwapChain::SetFullscreenState`;
     borderless sets `WS_POPUP` with monitor-sized geometry. macOS uses `NSWindow.toggleFullScreen`
     or `NSBorderlessWindowMask`. Linux Wayland uses `xdg_toplevel_set_fullscreen`; X11 uses
     `_NET_WM_STATE_FULLSCREEN`.

## Multi-Monitor

| ID       | Feature                                       | Requirements |
|----------|-----------------------------------------------|--------------|
| F-14.1.3 | Display Enumeration and Multi-Monitor Support | R-14.1.3     |
| F-14.1.4 | DPI Awareness and Display Scaling             | R-14.1.4     |

1. **F-14.1.3** — Enumerate connected displays, report resolution, refresh rate, color depth, HDR
   capability, and physical position. Players can drag the game window between monitors or launch on
   a specific display. Streaming setups with 2-3 monitors require correct display identification.
   - **Deps:** F-14.1.1
   - **Platform:** Windows uses `EnumDisplayMonitors` and `DXGI` output enumeration; macOS uses
     `NSScreen.screens` and `CGDisplayCopyDisplayMode`; Linux uses `xrandr` (X11) or `wl_output`
     events (Wayland). Display topology changes (hot-plug) must trigger re-enumeration.
2. **F-14.1.4** — Detect per-monitor DPI and respond to DPI changes when the window moves between
   monitors. UI elements, cursor hot-spots, and render resolution scale correctly. Fractional
   scaling (125%, 150%) must not produce blurry text or misaligned UI hit regions.
   - **Deps:** F-14.1.1, F-14.1.3
   - **Platform:** Windows uses `SetProcessDpiAwarenessContext(PER_MONITOR_AWARE_V2)` and handles
     `WM_DPICHANGED`; macOS uses `NSWindow.backingScaleFactor` (always integer 1x or 2x); Linux
     reads `Xft.dpi` (X11) or `wl_output.scale` (Wayland). Fractional scaling on Linux Wayland
     requires `wp_fractional_scale_v1`.

## Presentation

| ID       | Feature                             | Requirements |
|----------|-------------------------------------|--------------|
| F-14.1.5 | VSync and Refresh Rate Management   | R-14.1.5     |
| F-14.1.6 | HDR Output and Tone Mapping Handoff | R-14.1.6     |

1. **F-14.1.5** — Support multiple presentation modes: immediate (no VSync), FIFO (VSync on),
   mailbox (triple buffered), and adaptive VSync. Allow the player to cap frame rate independently
   of the display refresh rate. Correct VSync behavior prevents tearing during MMO raids where
   consistent frame pacing is critical.
   - **Deps:** F-14.1.1
   - **Platform:** Vulkan exposes `VK_PRESENT_MODE_*` directly. Metal uses
     `CAMetalLayer.displaySyncEnabled` for VSync and manual frame pacing for mailbox-like behavior.
     Console platforms have dedicated flip-queue APIs with platform-specific NDA documentation.
2. **F-14.1.6** — Enable HDR output when the display and OS support it, selecting the appropriate
   color space (scRGB on Windows, EDR on macOS) and signaling peak luminance to the compositor. The
   rendering pipeline hands off a linear HDR buffer; this feature handles swapchain format and
   metadata negotiation.
   - **Deps:** F-14.1.1, F-14.1.3
   - **Platform:** Windows uses `DXGI_COLOR_SPACE_RGB_FULL_G2084_NONE_P2020` and
     `DXGI_HDR_METADATA_HDR10`; macOS uses `CAMetalLayer` with `CGColorSpace` set to extended linear
     sRGB and EDR headroom queries. Linux Wayland HDR is experimental via `wp_color_management_v1`.
     Console platforms require platform-specific HDR calibration UIs.

## GPU Integration

| ID       | Feature                                      | Requirements |
|----------|----------------------------------------------|--------------|
| F-14.1.8 | Raw Window Handle for GPU Swapchain Creation | R-14.1.8     |

1. **F-14.1.8** — Expose a `RawWindowHandle` enum from each window that provides the platform-native
   handle (HWND on Windows, NSView on macOS, xcb window or Wayland surface on Linux) required by GPU
   backends to create swapchains. The handle is compatible with the `raw-window-handle` crate
   ecosystem so that Vulkan, Metal, and DX12 backends create surfaces without platform-specific
   branching.
   - **Deps:** F-14.1.1
   - **Platform:** Windows returns `hwnd` and `hinstance` pointers; macOS returns an `ns_view`
     pointer; Linux X11 returns an xcb window ID and connection pointer; Linux Wayland returns
     surface and display pointers. The handle is valid for the lifetime of the window.

## Event Delivery

| ID        | Feature                               | Requirements |
|-----------|---------------------------------------|--------------|
| F-14.1.9  | Bounded Channel Event Delivery        | R-14.1.9     |
| F-14.1.10 | Per-Window DPI Policy                 | R-14.1.10    |
| F-14.1.11 | Unified Event Polling and Translation | R-14.1.11    |
| F-14.1.12 | Logical and Physical Coordinate Types | R-14.1.12    |

1. **F-14.1.9** — Deliver window events through a bounded async channel that consumers poll via a
   non-blocking `EventIterator` or `.await`. Each event is delivered exactly once and consumed in
   order. The channel decouples the OS event loop from engine frame processing, preventing event
   loss under load.
   - **Deps:** F-14.1.1
   - **Platform:** Each platform backend's native event poller (Win32 message loop, `NSApplication`
     run loop, xcb/Wayland event dispatch) writes events into the bounded channel. Engine systems
     drain the channel each frame via `Window::events()`.
2. **F-14.1.10** — Support a per-window DPI policy that controls how each window responds to DPI
   changes. `SystemScaled` lets the OS resize the window to its suggested rectangle (default for
   game windows). `ApplicationScaled` keeps the window size fixed in physical pixels so the
   application handles scaling internally (useful for auxiliary debug windows with fixed layouts).
   - **Deps:** F-14.1.4
   - **Platform:** The DPI policy is set in `WindowConfig` at creation time. On DPI change events,
     `SystemScaled` windows automatically resize to the OS-suggested rectangle; `ApplicationScaled`
     windows emit the DPI event but do not resize.
3. **F-14.1.11** — Run a single event poller that receives all OS events (window, input, display) in
   a unified stream and translates platform-native event types into engine event types. The event
   translator converts native data (e.g., Win32 `LPARAM` fields, Wayland configure events) into
   cross-platform types and routes them to the appropriate bounded channel.
   - **Deps:** F-14.1.1, F-14.1.9
   - **Platform:** The poller is driven by each platform's native event loop (Win32
     `GetMessage`/`PeekMessage`, `NSApplication` run loop, xcb/Wayland event dispatch). Native key
     codes are mapped to engine-agnostic scan codes. Cursor positions are provided in both logical
     and physical coordinates. Focus change events are forwarded so the input system can suppress
     input when the window is unfocused.
4. **F-14.1.12** — Provide explicit `LogicalSize`, `PhysicalSize`, `Point`, and `Rect` coordinate
   types with conversion methods that account for the DPI scale factor. All public API surfaces
   accept and return these types so that callers never perform manual DPI arithmetic. Conversions
   round correctly to avoid off-by-one pixel errors at fractional scale factors.
   - **Deps:** F-14.1.4
   - **Platform:** `LogicalSize.to_physical(scale_factor)` and
     `PhysicalSize.to_logical(scale_factor)` handle all conversions. All window size, position, and
     event payloads use these types.
