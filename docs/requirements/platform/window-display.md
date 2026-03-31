# R-14.1 — Window & Display Requirements

## Window Management

1. **R-14.1.1** — The engine **SHALL** create, resize, minimize, maximize, restore, and destroy
   native windows on Windows, macOS, and Linux (X11 and Wayland) with a consistent cross-platform
   API, supporting a primary game window and additional auxiliary windows.
   - **Rationale:** A unified window lifecycle API isolates platform-specific windowing code behind
     a single abstraction, letting all engine systems manage windows without platform branching.
   - **Verification:** Integration test per platform: create a window, resize, minimize, maximize,
     restore, and destroy. Assert each state transition emits the correct lifecycle event and
     dimensions match the requested values. Create an auxiliary window and verify independent
     operation.

2. **R-14.1.2** — The engine **SHALL** switch between exclusive fullscreen, borderless fullscreen,
   and windowed modes at runtime without losing the GPU device context or producing visible flicker.
   - **Rationale:** Borderless fullscreen allows instant alt-tab; exclusive fullscreen enables
     lowest latency for competitive scenarios. Seamless transitions prevent player disruption.
   - **Verification:** Integration test per platform: cycle through windowed, borderless, exclusive
     fullscreen, and back. Assert no GPU device loss and the rendered frame is visible within two
     VSync intervals of each transition.

## Multi-Monitor

3. **R-14.1.3** — The engine **SHALL** enumerate all connected displays reporting resolution,
   refresh rate, color depth, HDR capability, and physical position, and **SHALL** re-enumerate
   within one frame of a display hot-plug event.
   - **Rationale:** Correct display enumeration ensures the game launches on the intended monitor
     and supports multi-monitor streaming setups.
   - **Verification:** Integration test on a multi-monitor system: enumerate and assert valid data
     per display. Simulate hot-plug and verify re-enumeration within one frame.

4. **R-14.1.4** — The engine **SHALL** detect per-monitor DPI, respond to DPI changes when the
   window moves between monitors, and scale UI elements and cursor hit regions correctly at
   fractional scale factors (125 %, 150 %) without blurry text or misaligned hit regions.
   - **Rationale:** Modern displays range from 96 to 288+ DPI; UI and cursor targeting must scale
     correctly at every factor.
   - **Verification:** Integration test: drag window between monitors with different DPI; assert DPI
     updates within one frame. Render text and buttons at 100 %, 125 %, 150 %, 200 %; assert text is
     sharp and hit regions match within 1 pixel.

## Presentation

5. **R-14.1.5** — The engine **SHALL** support immediate (no VSync), FIFO (VSync on), and mailbox
   (triple-buffered) presentation modes, and **SHALL** allow the player to cap the frame rate
   independently of the display refresh rate.
   - **Rationale:** Each mode serves different player preferences: immediate minimizes latency, FIFO
     eliminates tearing, mailbox balances both. An independent cap limits GPU power draw.
   - **Verification:** Integration test per mode: render 300 frames and measure pacing. Assert FIFO
     frames are at VSync intervals (within 0.5 ms). Assert mailbox has no tearing. Assert a 30 FPS
     cap on 60 Hz produces 33.3 ms intervals (within 1 ms).

6. **R-14.1.6** — The engine **SHALL** enable HDR output when the display and OS support it,
   selecting the correct color space (scRGB on Windows, extended linear sRGB on macOS) and signaling
   peak luminance to the compositor.
   - **Rationale:** HDR requires correct color space signaling and metadata; incorrect configuration
     causes washed-out or clipped colors.
   - **Verification:** Integration test on HDR display: enable HDR and assert swapchain reports an
     HDR-compatible format. Render pixels above 1.0 and verify they are not clipped. Verify peak
     luminance metadata matches the display.

## GPU Integration

7. **R-14.1.8** — The engine **SHALL** expose a `RawWindowHandle` from each window that provides the
   platform-native handle required by GPU backends for swapchain creation, layout-compatible with
   the `raw-window-handle` crate. The handle **SHALL** remain valid for the lifetime of the window.
   - **Rationale:** GPU backends require a native handle to create a presentable surface.
     Compatibility with `raw-window-handle` avoids platform branching in GPU code.
   - **Verification:** Integration test per platform: create window, obtain `raw_handle()`, pass to
     GPU backend for swapchain creation. Assert swapchain creation succeeds and a frame can be
     presented. Assert handle validity until window drop.

## Event Delivery

8. **R-14.1.9** — The engine **SHALL** deliver window events through a bounded async channel,
   accessible via a non-blocking `EventIterator`. Each event **SHALL** be delivered exactly once in
   FIFO order. The channel **SHALL NOT** drop events under normal load.
   - **Rationale:** Bounded channels decouple the OS event loop from frame processing. Exactly-once
     delivery prevents duplicate handling.
   - **Verification:** Integration test: generate 100 rapid resize events in one frame. Assert all
     received in order with none dropped. Assert `next()` returns `None` without blocking when the
     channel is empty.

9. **R-14.1.10** — The engine **SHALL** support a per-window `DpiPolicy` configured at creation
   time. `SystemScaled` windows **SHALL** auto-resize on DPI change. `ApplicationScaled` windows
   **SHALL** emit the DPI event but **SHALL NOT** resize.
   - **Rationale:** Game windows benefit from OS-managed scaling; debug windows with fixed layouts
     need manual control.
   - **Verification:** Integration test: create two windows with different policies, simulate DPI
     change. Assert `SystemScaled` resizes, assert `ApplicationScaled` emits event but keeps size.

10. **R-14.1.11** — The engine **SHALL** run a single event poller that receives all OS events and
    translates platform-native types into engine event types before routing to the appropriate
    channel.
    - **Rationale:** A single poller avoids race conditions. Centralized translation ensures
      platform data is decoded once.
    - **Verification:** Integration test: generate simultaneous resize and keyboard events. Assert
      window events and input events arrive on separate channels in correct order.

11. **R-14.1.12** — The engine **SHALL** provide `LogicalSize`, `PhysicalSize`, `Point`, and `Rect`
    types with conversion methods accounting for DPI scale factor. All public window APIs **SHALL**
    use these types. Conversions **SHALL** round correctly at fractional scale factors.
    - **Rationale:** Explicit types prevent confusion between logical and physical pixels,
      eliminating DPI scaling bugs.
    - **Verification:** Unit test: assert `LogicalSize(1280, 720).to_physical(1.25)` equals
      `PhysicalSize(1600, 900)`. Assert round-trip conversion at integer scales produces the
      original value. Assert no off-by-one errors at 1.5x.
