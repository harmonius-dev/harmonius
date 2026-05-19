# User Stories -- 14.1 Window & Display

| ID         | Persona                 |
|------------|-------------------------|
| US-14.1.1  | player (P-23)           |
| US-14.1.2  | player (P-23)           |
| US-14.1.3  | player (P-23)           |
| US-14.1.4  | player (P-23)           |
| US-14.1.5  | player (P-23)           |
| US-14.1.6  | player (P-23)           |
| US-14.1.7  | player (P-23)           |
| US-14.1.8  | game developer (P-15)   |
| US-14.1.9  | game developer (P-15)   |
| US-14.1.10 | game developer (P-15)   |
| US-14.1.11 | game developer (P-15)   |
| US-14.1.12 | game developer (P-15)   |
| US-14.1.13 | game developer (P-15)   |
| US-14.1.14 | engine developer (P-26) |
| US-14.1.15 | engine developer (P-26) |
| US-14.1.16 | engine developer (P-26) |
| US-14.1.17 | engine developer (P-26) |
| US-14.1.18 | engine developer (P-26) |
| US-14.1.19 | engine tester (P-27)    |
| US-14.1.20 | engine tester (P-27)    |
| US-14.1.21 | engine tester (P-27)    |
| US-14.1.22 | engine tester (P-27)    |
| US-14.1.23 | engine tester (P-27)    |

## Window Management

1. **US-14.1.1** — **As a** player (P-23), **I want** the game window to open, resize, minimize,
   maximize, and restore consistently on my OS, **so that** basic window management behaves as I
   expect.
2. **US-14.1.2** — **As a** player (P-23), **I want** to switch between exclusive fullscreen,
   borderless fullscreen, and windowed mode without the game crashing, **so that** I can alt-tab
   freely during play.
3. **US-14.1.3** — **As a** player (P-23), **I want** borderless fullscreen to be the default
   display mode, **so that** switching to other apps is instant with no mode-switch delay.

## Multi-Monitor

4. **US-14.1.4** — **As a** player (P-23), **I want** the game to list all connected monitors with
   their resolution, refresh rate, and HDR status, **so that** I can choose which display to play
   on.
5. **US-14.1.5** — **As a** player (P-23), **I want** to drag the game window between monitors and
   have UI text, cursors, and hit regions scale correctly at fractional DPI (125 %, 150 %),
   **so that** nothing looks blurry or misaligned on my high-DPI display.

## Presentation

6. **US-14.1.6** — **As a** player (P-23), **I want** to pick a VSync mode (off, on,
   triple-buffered, adaptive) and set an independent frame-rate cap, **so that** I can balance
   latency against smoothness.
7. **US-14.1.7** — **As a** player (P-23), **I want** HDR output enabled automatically when my
   display and OS support it, **so that** I see the full dynamic range without manual configuration.

## Game Developer -- Window API

8. **US-14.1.8** — **As a** game developer (P-15), **I want** to open auxiliary windows for debug
   overlays, chat pop-outs, or streaming dashboards alongside the primary game window, **so that** I
   can build multi-window tool workflows.
9. **US-14.1.9** — **As a** game developer (P-15), **I want** to iterate over pending window events
   via a non-blocking `EventIterator` each frame, **so that** resize, DPI, and focus events are
   processed without stalling the render loop.
10. **US-14.1.10** — **As a** game developer (P-15), **I want** to `.await` the event channel when
    no events are pending, **so that** async systems can wait for window events without
    busy-polling.
11. **US-14.1.11** — **As a** game developer (P-15), **I want** to set a `DpiPolicy` per window in
    `WindowConfig` (`SystemScaled` or `ApplicationScaled`), **so that** my game window auto-scales
    while a debug overlay keeps its fixed pixel layout.
12. **US-14.1.12** — **As a** game developer (P-15), **I want** all window API surfaces to use
    explicit `LogicalSize` and `PhysicalSize` types with conversion methods, **so that** I never
    confuse coordinate spaces and avoid DPI scaling bugs.
13. **US-14.1.13** — **As a** game developer (P-15), **I want** console platforms to use their
    dedicated flip-queue APIs, **so that** console certification presentation requirements are met
    automatically.

## Engine Developer -- Platform Backends

14. **US-14.1.14** — **As an** engine developer (P-26), **I want** window creation and lifecycle
    implemented through platform-native APIs (Win32, NSWindow, xcb/Wayland), **so that** each
    backend handles DPI changes and hot-plug re-enumeration correctly.
15. **US-14.1.15** — **As an** engine developer (P-26), **I want** `window.raw_handle()` to return a
    platform-native handle compatible with `raw-window-handle`, **so that** GPU backends create
    swapchains without platform branching.
16. **US-14.1.16** — **As an** engine developer (P-26), **I want** HDR swapchain format and metadata
    negotiation to use DXGI on Windows, Vulkan WSI swapchain with EDR on macOS, and
    `wp_color_management_v1` on Wayland, **so that** HDR output is correct on each platform.
17. **US-14.1.17** — **As an** engine developer (P-26), **I want** a single unified event poller
    that receives all OS events (window, input, display) and translates them into engine event
    types, **so that** there are no race conditions between separate polling loops.
18. **US-14.1.18** — **As an** engine developer (P-26), **I want** display hot-plug events to
    trigger re-enumeration and graceful reconfiguration, **so that** plugging or removing a monitor
    mid-session does not crash the game.

## Engine Tester -- Validation

19. **US-14.1.19** — **As an** engine tester (P-27), **I want** tests that cycle through fullscreen,
    borderless, and windowed modes on each platform verifying GPU context is preserved, **so that**
    mode-switching regressions are caught in CI.
20. **US-14.1.20** — **As an** engine tester (P-27), **I want** tests that render UI at 100 %, 125
    %, 150 %, and 200 % scaling and compare against reference images, **so that** fractional scaling
    does not produce blurry text or misaligned hit regions.
21. **US-14.1.21** — **As an** engine tester (P-27), **I want** a stress test that generates 100+
    window events in a single frame and verifies all are received in order with none dropped,
    **so that** the bounded channel handles event bursts.
22. **US-14.1.22** — **As an** engine tester (P-27), **I want** a test that drags a `SystemScaled`
    window between monitors with different DPI and verifies it resizes to the OS-suggested
    rectangle, **so that** automatic DPI scaling is validated.
23. **US-14.1.23** — **As an** engine tester (P-27), **I want** benchmarks that measure frame time
    variance under GPU load with each VSync mode, **so that** tearing prevention and frame pacing
    are verified across all platforms.
