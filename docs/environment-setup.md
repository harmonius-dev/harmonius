# Environment Setup

Use the setup script to provision prerequisites and initialize the project.

## Quick start

```bash
./scripts/setup_environment.sh
```

Preview the package manager commands without changing the host:

```bash
./scripts/setup_environment.sh --dry-run
```

## Supported hosts

| Host | Package manager | Graphics stack | Project generation |
| --- | --- | --- | --- |
| macOS | Homebrew | Metal | XcodeGen |
| Debian/Ubuntu | apt | Vulkan + X11 | CMake-ready deps |
| Fedora | dnf | Vulkan + X11 | CMake-ready deps |
| Arch | pacman | Vulkan + X11 | CMake-ready deps |

## What the script does

1. Detects the host operating system and package manager.
2. Installs common build tools: Git, CMake, Ninja, and compiler packages.
3. Installs Linux graphics prerequisites for Vulkan and X11 development.
4. Initializes git submodules recursively.
5. Generates `Harmonius.xcodeproj` with `xcodegen` on macOS.

## Linux Vulkan and X11 packages

The Linux path installs development headers and runtime tools for:

- Vulkan loader, headers, tools, validation layers, and Mesa Vulkan drivers.
- X11, XCB, RandR, Xcursor, Xi, Xinerama, Xrender, Xfixes, and XKB common headers.
- `pkg-config`, compiler toolchains, CMake, Ninja, Git, and certificates.

## Notes

- Harmonius app builds and tests with Xcode on macOS today.
- Linux setup prepares a Vulkan and X11 development environment for native Linux work.
- Harmonius requires CMake 4.0 or newer. Some distro repositories may ship older CMake packages.
