# Harmonius

Harmonius is a modern cross-platform game engine for building real-time interactive 2D, 3D, and XR
games and high performance applications. Harmonius is written in Rust with thin C wrappers to
OS-level APIs.

Harmonius supports desktop applications on MacOS, iOS, Linux, Android, Nintendo Switch, Xbox an
Windows. Harmonius's graphics backend can use Metal 4, Direct3D 12, or Vulkan 1.4, and requires mesh
shaders and ray tracing as minimum requirements. It supports both hybrid ray tracing and full path
tracing. Harmonius scales from mobile devices and portable consoles to high-end desktop PCs for the
most ambitious projects. Harmonius does not support the traditional rendering pipeline, low end
devices, or any legacy graphics API backends. Harmonius does not support targeting the web.

## Architecture

See [docs/architecture.md](docs/architecture.md) for the full engine architecture with Mermaid
diagrams covering all 15 subsystems, frame data flow, platform abstraction, and design wave
structure.

## Supported Platforms

| OS | Graphics API | Async I/O |
|----|-------------|-----------|
| macOS | Metal 4 | GCD / Dispatch IO |
| Windows | Direct3D 12 | IOCP |
| Linux | Vulkan 1.4 | io_uring |
| iOS | Metal 4 | GCD / Dispatch IO |
| Android | Vulkan 1.4 | io_uring |
| Nintendo Switch | Platform SDK | Platform SDK |
| Xbox | Direct3D 12 | Platform SDK |

## Documentation

| Directory | Contents |
|-----------|----------|
| [docs/design/](docs/design/) | 87 design documents across 15 domains |
| [docs/features/](docs/features/) | 1,381 feature definitions |
| [docs/requirements/](docs/requirements/) | 1,171 requirements |
| [docs/user-stories/](docs/user-stories/) | 5,859 user stories |
| [docs/standards/](docs/standards/) | Language coding standards |
| [docs/architecture.md](docs/architecture.md) | Engine architecture overview |
