# Environment Setup

Use the setup script to provision host tools and initialize the repo.

## Quick Start

```bash
./scripts/setup_environment.sh
./scripts/dev.sh bootstrap macos
```

Preview package manager commands without changing the host:

```bash
./scripts/setup_environment.sh --dry-run
```

## Supported Hosts

| Host | Package manager | Graphics stack | Build entrypoint |
| --- | --- | --- | --- |
| macOS | Homebrew | Metal | `scripts/dev.sh` |
| Debian/Ubuntu | apt | Vulkan + X11 | `scripts/dev.sh` |
| Fedora | dnf | Vulkan + X11 | `scripts/dev.sh` |
| Arch | pacman | Vulkan + X11 | `scripts/dev.sh` |

## What The Script Does

1. Detects the host operating system and package manager.
2. Installs Git, Swift support tools, `pkg-config`, `jq`, and certificates.
3. Installs XcodeGen and `swift-format` on macOS.
4. Installs Linux graphics prerequisites for Vulkan and X11 development.
5. Initializes git submodules recursively.
6. Runs `scripts/dev.sh bootstrap macos` on macOS.
7. Generates `Harmonius.xcodeproj` with XcodeGen on macOS.

## vcpkg Triplets

`scripts/dev.sh` installs third-party C/C++ dependencies through vcpkg:

- `arm64-osx` for macOS.
- `arm64-ios` for iOS devices.
- `arm64-ios-simulator` for iOS simulator.
- `x64-linux` for Linux.

The Bash workflow generates pkg-config shims under `build/pkgconfig/<triplet>/`.
SwiftPM consumes those shims through system-library targets.

## Appium Tools

Install Appium and the required drivers with:

```bash
./scripts/dev.sh appium-bootstrap
```

The workflow uses Appium `mac2` for macOS and `xcuitest` for iOS targets.
All UI test bodies are Swift tests that use `swift-webdriver`.

## Notes

- SwiftPM owns package builds, shader artifacts, and package tests.
- XcodeGen owns Apple bundles, resources, signing, schemes, and archives.
- Bash is the public interface for VS Code, CI, and LLM agents.
- vcpkg ports may use their own internal build systems while installing deps.
