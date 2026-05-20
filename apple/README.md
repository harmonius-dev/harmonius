# Harmonius macOS App Bundle

Packages the Rust `harmonius_app` binary into a `.app` bundle so AppKit windowing works.

## Prerequisites

- Xcode command-line tools
- [XcodeGen](https://github.com/yonaskolb/XcodeGen)
- Rust stable
- Vulkan SDK or Homebrew `molten-vk` (MoltenVK ICD + dylib)

## Build and run

```bash
cd apple
xcodegen generate
xcodebuild -project Harmonius.xcodeproj -scheme Harmonius -configuration Debug build
open build/Debug/Harmonius.app
```

## Linux

```bash
cargo run -p harmonius_app --release
```

Requires X11 and a Vulkan loader.
