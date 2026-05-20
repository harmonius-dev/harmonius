# Custom windowing (no winit)

## Status

Accepted — 2024-12-04 (backfilled 2026-05-20)

## Context

`winit` is the standard Rust windowing library. It abstracts Win32, Cocoa, X11, Wayland,
UIKit, and Android. Most Rust game projects use it. The engine evaluated `winit` and found
three problems:

1. **Event-loop ownership.** `winit` requires its event loop to drive the main thread. The
   engine's main-thread event loop also needs to drain platform-native I/O completions
   (io_uring SQ/CQ pairs, IOCP completion ports, GCD `dispatch_io` queues) and to forward
   timer events. Composing two event loops is fragile.
2. **OS-API surface.** Win32 message hooks for clipboard, IME, raw input, drag-and-drop, and
   power-state callbacks are easier to do directly. `winit` mediates these and exposes a
   subset, requiring escape hatches.
3. **Threading topology.** `winit` mandates a single main thread; the engine's threading
   model agrees but the boundaries between main thread, OS event loop, and platform I/O are
   different from `winit`'s expectations.

## Decision

The engine implements windowing directly:

| Platform | API                                |
|----------|------------------------------------|
| Windows  | Win32 `CreateWindowEx` via `windows-rs` |
| macOS    | `NSWindow` via `objc2-app-kit`     |
| Linux    | X11 via `x11rb`, Wayland via `wayland-client` |
| iOS      | `UIWindow` via `objc2`             |
| Android  | NDK `ANativeWindow` via the `ndk` crate |

The main thread owns the OS event loop directly and integrates platform-native I/O polling in
the same loop. There is no `winit` dependency. The custom layer is small per platform because
the engine consumes only a narrow slice of the windowing API.

## Consequences

- Five platform-specific windowing implementations to maintain. Net cost is positive because
  each is < 1k lines and avoids `winit` escape-hatch maintenance.
- Drag-and-drop, IME, raw input, clipboard, and power-state callbacks are first-class via the
  native APIs.
- iOS `UIApplicationMain` runs `CFRunLoop` on thread 0; the game loop runs on a dedicated
  worker. Touch and accelerometer events are forwarded via channel.
- Game-loop-on-worker model holds across all platforms.

## Alternatives Considered

- **`winit`** — too restrictive on event-loop integration with platform-native I/O.
- **`tao`** (winit fork) — same architectural constraint as winit.
- **SDL** — adds a C dependency and a much wider scope than needed.

## References

- [docs/design/constraints.md](../../design/constraints.md) "Windowing"
- [docs/design/platform/windowing.md](../../design/platform/windowing.md)
