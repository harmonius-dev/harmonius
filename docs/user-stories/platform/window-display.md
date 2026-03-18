# User Stories -- 14.1 Window & Display

| ID         | Persona                 | Features | Requirements |
|------------|-------------------------|----------|--------------|
| US-14.1.1  | player (P-23)           |          |              |
| US-14.1.2  | player (P-23)           |          |              |
| US-14.1.3  | player (P-23)           |          |              |
| US-14.1.4  | player (P-23)           |          |              |
| US-14.1.5  | player (P-23)           |          |              |
| US-14.1.6  | player (P-23)           |          |              |
| US-14.1.7  | engine developer (P-26) |          |              |
| US-14.1.8  | game developer (P-15)   |          |              |
| US-14.1.9  | engine developer (P-26) |          |              |
| US-14.1.10 | engine developer (P-26) |          |              |
| US-14.1.11 | engine tester (P-27)    |          |              |
| US-14.1.12 | engine tester (P-27)    |          |              |
| US-14.1.13 | game developer (P-15)   |          |              |
| US-14.1.14 | engine tester (P-27)    |          |              |
| US-14.1.17 | engine developer (P-26) |          |              |
| US-14.1.18 | game developer (P-15)   |          |              |
| US-14.1.19 | engine developer (P-26) |          |              |
| US-14.1.20 | engine developer (P-26) |          |              |
| US-14.1.21 | engine tester (P-27)    |          |              |
| US-14.1.22 | game developer (P-15)   |          |              |
| US-14.1.23 | engine tester (P-27)    |          |              |
| US-14.1.24 | engine tester (P-27)    |          |              |
| US-14.1.25 | engine developer (P-26) |          |              |
| US-14.1.26 | engine developer (P-26) |          |              |
| US-14.1.27 | game developer (P-15)   |          |              |
| US-14.1.28 | game developer (P-15)   |          |              |

1. **US-14.1.1** — the game to create, resize, minimize, maximize, and restore windows with
   consistent behavior on Windows, macOS, and Linux, so that basic window management works as I
   expect regardless of my operating system
2. **US-14.1.2** — to switch between exclusive fullscreen, borderless fullscreen, and windowed modes
   at runtime without losing GPU context or crashing, so that I can alt-tab freely to Discord,
   browsers, and streaming tools during MMO sessions
3. **US-14.1.3** — the game to enumerate all connected displays with resolution, refresh rate, and
   HDR capability, and let me drag the window between monitors, so that my multi-monitor streaming
   setup works correctly
4. **US-14.1.4** — per-monitor DPI awareness with correct fractional scaling (125%, 150%), so that
   UI text, cursor hot-spots, and hit regions are sharp and correctly aligned on my 4K monitor or
   laptop display
5. **US-14.1.5** — configurable VSync (immediate, FIFO, mailbox, adaptive) and an independent frame
   rate cap, so that I can choose between lowest latency and smoothest frame delivery for
   competitive and casual play
6. **US-14.1.6** — the game to enable HDR output with correct color space and peak luminance
   metadata when my display supports it, so that I see the full dynamic range of the rendering
   pipeline
7. **US-14.1.7** — window creation and lifecycle management through platform-native APIs (Win32,
   NSWindow, xcb/Wayland), so that each platform behaves correctly including hot-plug re-enumeration
   and DPI change events
8. **US-14.1.8** — support for auxiliary windows (debug overlays, chat pop-outs, streaming
   dashboards) alongside the primary game window, so that I can build MMO tools that run in separate
   windows
9. **US-14.1.9** — HDR swapchain format and metadata negotiation using DXGI on Windows, CAMetalLayer
   with EDR on macOS, and wp_color_management_v1 on Wayland, so that HDR output works correctly on
   each platform with the appropriate color space and tone mapping handoff
10. **US-14.1.10** — display topology changes (monitor connect, disconnect, resolution change) to
    trigger re-enumeration and graceful reconfiguration, so that plugging in or removing a monitor
    mid-session does not crash the game or leave the window in an invalid state
11. **US-14.1.11** — automated tests that cycle through fullscreen, borderless, and windowed modes
    on each platform, verifying GPU context is preserved, so that mode switching regressions are
    caught before release
12. **US-14.1.12** — tests that render UI at 100%, 125%, 150%, and 200% scaling and verify element
    positions and text sharpness against reference images, so that fractional scaling does not
    produce blurry text or misaligned hit regions
13. **US-14.1.13** — console platforms to use their dedicated flip-queue APIs with platform-specific
    frame pacing, so that console certification presentation requirements are met
14. **US-14.1.14** — benchmarks that measure frame time variance under GPU load with each VSync mode
    enabled, so that I can confirm tearing prevention and frame pacing behave correctly across all
    platforms
15. **US-14.1.17** — to call `window.raw_handle()` and receive a platform-native handle compatible
    with `raw-window-handle`, so that I can create a Vulkan, Metal, or DX12 swapchain without
    platform branching
16. **US-14.1.18** — the raw window handle to work with wgpu, ash, and metal-rs, so that I can
    choose my GPU backend without modifying windowing code
17. **US-14.1.19** — to iterate over pending window events via a non-blocking `EventIterator`, so
    that I process resize, DPI, and focus events each frame without stalling the render loop
18. **US-14.1.20** — to `.await` the event channel when no events are pending, so that async systems
    can wait for window events without busy-polling
19. **US-14.1.21** — a stress test that generates 100+ window events in a single frame and verifies
    all are received in order with none dropped, so that the bounded channel handles event bursts
    without loss
20. **US-14.1.22** — to set a `DpiPolicy` (`SystemScaled` or `ApplicationScaled`) per window in
    `WindowConfig`, so that my game window auto-scales on DPI change while my debug overlay keeps a
    fixed pixel layout
21. **US-14.1.23** — a test that drags a `SystemScaled` window between monitors with different DPI
    and verifies it resizes to the OS-suggested rectangle, so that automatic DPI scaling works
    correctly
22. **US-14.1.24** — a test that triggers a DPI change on an `ApplicationScaled` window and verifies
    it emits the `DpiChanged` event but does not resize, so that fixed-layout windows are not
    disrupted by DPI changes
23. **US-14.1.25** — a single unified event poller that receives window, input, and display events,
    so that I avoid race conditions between separate polling loops and get consistent event ordering
24. **US-14.1.26** — the event translator to convert platform-native data (Win32 `LPARAM`, Wayland
    configure events) into engine event types, so that downstream systems never handle
    platform-specific data
25. **US-14.1.27** — all window API surfaces to use explicit `LogicalSize` and `PhysicalSize` types,
    so that I never confuse logical and physical coordinates and avoid DPI scaling bugs
26. **US-14.1.28** — `LogicalSize.to_physical(scale)` and `PhysicalSize.to_logical(scale)`
    conversion methods, so that I can convert coordinates without manual DPI arithmetic and trust
    the rounding is correct at fractional scale factors
