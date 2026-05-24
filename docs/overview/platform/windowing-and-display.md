# Windowing and Display

Creating windows, handling resolution and refresh rates, and presenting frames.

## What it covers

- Window lifecycle: create, resize, fullscreen/borderless/windowed modes, hot-switching without GPU
  context loss.
- Multi-monitor support: enumeration, DPI-aware scaling, hot-plug detection.
- Presentation modes: immediate, FIFO, mailbox, adaptive for frame pacing and tear-free rendering.
- HDR output negotiation with color space and peak-luminance metadata.
- Event channel: window events (resize, focus, input) delivered in-order without drops.
- DPI handling: logical vs physical coordinates, fractional scaling support.
- Cursor management: icon, visibility, confinement, warping for relative motion.
- Async frame presentation coordinated with the render thread.

## Concepts

### Window Model

A window is a top-level surface with a native handle, size, and position. Windows transition between
fullscreen (exclusive or borderless) and windowed modes without losing the GPU render context.
Multi-monitor systems report per-monitor DPI, refresh rate, color depth, and HDR capability; on
hot-plug, the engine re-enumerates and updates UI layouts within one frame.

### Presentation and Pacing

Presentation modes control frame delivery: immediate (no sync, potential tearing), FIFO (queue
frames), mailbox (drop old frames), adaptive (sync when possible, tear if latency spikes). HDR
output requires explicit negotiation of color space (rec2020, display-p3) and peak luminance. Frame
pacing is coordinated with the OS and GPU to maintain the target refresh rate.

### DPI and Scaling

Text and hit regions scale with system DPI. The engine exposes both logical (DPI-agnostic) and
physical (pixel-exact) coordinates. Fractional scaling (125%, 150%) is handled per-monitor with
proper rounding to avoid sub-pixel rendering artifacts.

## How it fits

- See [threading-and-async](./threading-and-async.md) for render-thread presentation.
- See [crash-and-telemetry](./crash-and-telemetry.md) for frame-pacing performance metrics.
- Integrates with [rendering](../rendering/pipeline.md) for swapchain creation.
