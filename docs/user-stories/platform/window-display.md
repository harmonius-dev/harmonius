# User Stories -- 14.1 Window & Display

## US-14.1.1 Create and Manage the Game Window on Every Platform

**As a** player (P-23), **I want** the game to create, resize, minimize, maximize, and restore
windows with consistent behavior on Windows, macOS, and Linux, **so that** basic window management
works as I expect regardless of my operating system.

## US-14.1.2 Switch Between Fullscreen and Windowed Without Issues

**As a** player (P-23), **I want** to switch between exclusive fullscreen, borderless fullscreen,
and windowed modes at runtime without losing GPU context or crashing, **so that** I can alt-tab
freely to Discord, browsers, and streaming tools during MMO sessions.

## US-14.1.3 Play on Any Monitor in a Multi-Display Setup

**As a** player (P-23), **I want** the game to enumerate all connected displays with resolution,
refresh rate, and HDR capability, and let me drag the window between monitors, **so that** my
multi-monitor streaming setup works correctly.

## US-14.1.4 See Sharp UI on High-DPI and Fractional-Scaling Displays

**As a** player (P-23), **I want** per-monitor DPI awareness with correct fractional scaling (125%,
150%), **so that** UI text, cursor hot-spots, and hit regions are sharp and correctly aligned on my
4K monitor or laptop display.

## US-14.1.5 Eliminate Tearing and Control Frame Pacing

**As a** player (P-23), **I want** configurable VSync (immediate, FIFO, mailbox, adaptive) and an
independent frame rate cap, **so that** I can choose between lowest latency and smoothest frame
delivery for competitive and casual play.

## US-14.1.6 Experience HDR Visuals on Supported Displays

**As a** player (P-23), **I want** the game to enable HDR output with correct color space and peak
luminance metadata when my display supports it, **so that** I see the full dynamic range of the
rendering pipeline.

## US-14.1.7 Implement Window Management with Platform-Native APIs

**As an** engine developer (P-26), **I want** window creation and lifecycle management through
platform-native APIs (Win32, NSWindow, xcb/Wayland), **so that** each platform behaves correctly
including hot-plug re-enumeration and DPI change events.

## US-14.1.8 Support Auxiliary Windows for Debug and Streaming

**As a** game developer (P-15), **I want** support for auxiliary windows (debug overlays, chat
pop-outs, streaming dashboards) alongside the primary game window, **so that** I can build MMO tools
that run in separate windows.

## US-14.1.9 Implement HDR Swapchain Negotiation per Platform

**As an** engine developer (P-26), **I want** HDR swapchain format and metadata negotiation using
DXGI on Windows, CAMetalLayer with EDR on macOS, and wp_color_management_v1 on Wayland, **so that**
HDR output works correctly on each platform with the appropriate color space and tone mapping
handoff.

## US-14.1.10 Handle Display Hot-Plug Without Crashing

**As an** engine developer (P-26), **I want** display topology changes (monitor connect, disconnect,
resolution change) to trigger re-enumeration and graceful reconfiguration, **so that** plugging in
or removing a monitor mid-session does not crash the game or leave the window in an invalid state.

## US-14.1.11 Verify Window Modes on All Platforms

**As an** engine tester (P-27), **I want** automated tests that cycle through fullscreen,
borderless, and windowed modes on each platform, verifying GPU context is preserved, **so that**
mode switching regressions are caught before release.

## US-14.1.12 Verify Fractional DPI Scaling Produces Correct UI Layout

**As an** engine tester (P-27), **I want** tests that render UI at 100%, 125%, 150%, and 200%
scaling and verify element positions and text sharpness against reference images, **so that**
fractional scaling does not produce blurry text or misaligned hit regions.

## US-14.1.13 Configure Console-Specific Presentation Modes

**As a** game developer (P-15), **I want** console platforms to use their dedicated flip-queue APIs
with platform-specific frame pacing, **so that** console certification presentation requirements are
met.

## US-14.1.14 Verify VSync and Frame Pacing Under Load

**As an** engine tester (P-27), **I want** benchmarks that measure frame time variance under GPU
load with each VSync mode enabled, **so that** I can confirm tearing prevention and frame pacing
behave correctly across all platforms.

## US-14.1.17 Create a GPU Swapchain from the Raw Window Handle

**As an** engine developer (P-26), **I want** to call `window.raw_handle()` and receive a
platform-native handle compatible with `raw-window-handle`, **so that** I can create a Vulkan,
Metal, or DX12 swapchain without platform branching.

## US-14.1.18 Pass Raw Handle to Any GPU Backend

**As a** game developer (P-15), **I want** the raw window handle to work with wgpu, ash, and
metal-rs, **so that** I can choose my GPU backend without modifying windowing code.

## US-14.1.19 Drain Window Events Each Frame Without Blocking

**As an** engine developer (P-26), **I want** to iterate over pending window events via a
non-blocking `EventIterator`, **so that** I process resize, DPI, and focus events each frame without
stalling the render loop.

## US-14.1.20 Await Window Events Asynchronously

**As an** engine developer (P-26), **I want** to `.await` the event channel when no events are
pending, **so that** async systems can wait for window events without busy-polling.

## US-14.1.21 Verify No Events Are Dropped Under Burst Load

**As an** engine tester (P-27), **I want** a stress test that generates 100+ window events in a
single frame and verifies all are received in order with none dropped, **so that** the bounded
channel handles event bursts without loss.

## US-14.1.22 Set DPI Policy Per Window at Creation Time

**As a** game developer (P-15), **I want** to set a `DpiPolicy` (`SystemScaled` or
`ApplicationScaled`) per window in `WindowConfig`, **so that** my game window auto-scales on DPI
change while my debug overlay keeps a fixed pixel layout.

## US-14.1.23 Verify SystemScaled Window Resizes on DPI Change

**As an** engine tester (P-27), **I want** a test that drags a `SystemScaled` window between
monitors with different DPI and verifies it resizes to the OS-suggested rectangle, **so that**
automatic DPI scaling works correctly.

## US-14.1.24 Verify ApplicationScaled Window Does Not Resize

**As an** engine tester (P-27), **I want** a test that triggers a DPI change on an
`ApplicationScaled` window and verifies it emits the `DpiChanged` event but does not resize,
**so that** fixed-layout windows are not disrupted by DPI changes.

## US-14.1.25 Receive All Event Types from a Single Poller

**As an** engine developer (P-26), **I want** a single unified event poller that receives window,
input, and display events, **so that** I avoid race conditions between separate polling loops and
get consistent event ordering.

## US-14.1.26 Translate Platform Events to Cross-Platform Types

**As an** engine developer (P-26), **I want** the event translator to convert platform-native data
(Win32 `LPARAM`, Wayland configure events) into engine event types, **so that** downstream systems
never handle platform-specific data.

## US-14.1.27 Use Logical and Physical Size Types in the API

**As a** game developer (P-15), **I want** all window API surfaces to use explicit `LogicalSize` and
`PhysicalSize` types, **so that** I never confuse logical and physical coordinates and avoid DPI
scaling bugs.

## US-14.1.28 Convert Between Logical and Physical Coordinates

**As a** game developer (P-15), **I want** `LogicalSize.to_physical(scale)` and
`PhysicalSize.to_logical(scale)` conversion methods, **so that** I can convert coordinates without
manual DPI arithmetic and trust the rounding is correct at fractional scale factors.
