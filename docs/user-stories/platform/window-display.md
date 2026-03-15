# User Stories -- 14.1 Window & Display

## US-14.1.1 Create and Manage the Game Window on Every Platform

**As a** player (P-23), **I want** the game to create, resize, minimize, maximize, and
restore windows with consistent behavior on Windows, macOS, and Linux, **so that** basic
window management works as I expect regardless of my operating system.

## US-14.1.2 Switch Between Fullscreen and Windowed Without Issues

**As a** player (P-23), **I want** to switch between exclusive fullscreen, borderless
fullscreen, and windowed modes at runtime without losing GPU context or crashing, **so
that** I can alt-tab freely to Discord, browsers, and streaming tools during MMO sessions.

## US-14.1.3 Play on Any Monitor in a Multi-Display Setup

**As a** player (P-23), **I want** the game to enumerate all connected displays with
resolution, refresh rate, and HDR capability, and let me drag the window between monitors,
**so that** my multi-monitor streaming setup works correctly.

## US-14.1.4 See Sharp UI on High-DPI and Fractional-Scaling Displays

**As a** player (P-23), **I want** per-monitor DPI awareness with correct fractional
scaling (125%, 150%), **so that** UI text, cursor hot-spots, and hit regions are sharp and
correctly aligned on my 4K monitor or laptop display.

## US-14.1.5 Eliminate Tearing and Control Frame Pacing

**As a** player (P-23), **I want** configurable VSync (immediate, FIFO, mailbox, adaptive)
and an independent frame rate cap, **so that** I can choose between lowest latency and
smoothest frame delivery for competitive and casual play.

## US-14.1.6 Experience HDR Visuals on Supported Displays

**As a** player (P-23), **I want** the game to enable HDR output with correct color space
and peak luminance metadata when my display supports it, **so that** I see the full dynamic
range of the rendering pipeline.

## US-14.1.7 Implement Window Management with Platform-Native APIs

**As an** engine developer (P-26), **I want** window creation and lifecycle management
through platform-native APIs (Win32, NSWindow, xcb/Wayland), **so that** each platform
behaves correctly including hot-plug re-enumeration and DPI change events.

## US-14.1.8 Support Auxiliary Windows for Debug and Streaming

**As a** game developer (P-15), **I want** support for auxiliary windows (debug overlays,
chat pop-outs, streaming dashboards) alongside the primary game window, **so that** I can
build MMO tools that run in separate windows.

## US-14.1.9 Implement HDR Swapchain Negotiation per Platform

**As an** engine developer (P-26), **I want** HDR swapchain format and metadata negotiation
using DXGI on Windows, CAMetalLayer with EDR on macOS, and wp_color_management_v1 on
Wayland, **so that** HDR output works correctly on each platform with the appropriate color
space and tone mapping handoff.

## US-14.1.10 Handle Display Hot-Plug Without Crashing

**As an** engine developer (P-26), **I want** display topology changes (monitor connect,
disconnect, resolution change) to trigger re-enumeration and graceful reconfiguration,
**so that** plugging in or removing a monitor mid-session does not crash the game or leave
the window in an invalid state.

## US-14.1.11 Verify Window Modes on All Platforms

**As an** engine tester (P-27), **I want** automated tests that cycle through fullscreen,
borderless, and windowed modes on each platform, verifying GPU context is preserved, **so
that** mode switching regressions are caught before release.

## US-14.1.12 Verify Fractional DPI Scaling Produces Correct UI Layout

**As an** engine tester (P-27), **I want** tests that render UI at 100%, 125%, 150%, and
200% scaling and verify element positions and text sharpness against reference images, **so
that** fractional scaling does not produce blurry text or misaligned hit regions.

## US-14.1.13 Configure Console-Specific Presentation Modes

**As a** game developer (P-15), **I want** console platforms to use their dedicated
flip-queue APIs with platform-specific frame pacing, **so that** console certification
presentation requirements are met.

## US-14.1.14 Verify VSync and Frame Pacing Under Load

**As an** engine tester (P-27), **I want** benchmarks that measure frame time variance under
GPU load with each VSync mode enabled, **so that** I can confirm tearing prevention and
frame pacing behave correctly across all platforms.
