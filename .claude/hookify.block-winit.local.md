---
name: block-winit
enabled: true
event: file
pattern: \bwinit\b
action: block
---

# winit is forbidden in this project

Build custom platform-native windowing instead. Each platform uses its own native API directly
(AppKit/UIKit, Win32, X11/Wayland).
