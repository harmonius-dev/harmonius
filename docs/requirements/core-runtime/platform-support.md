# Platform Support

## R-1.2.1 macOS via Metal

macOS SHALL be supported via Metal 4 on Apple Silicon (M1+). This is the initial development target.

## R-1.2.2 Windows via Direct3D 12

Windows SHALL be supported via Direct3D 12 with the Agility SDK for access to the latest GPU
features independent of OS version.

## R-1.2.3 Windows and SteamOS via Vulkan

Windows and SteamOS SHALL be supported via Vulkan 1.4 with required extensions. SteamOS support
enables the Steam Deck and Linux desktop gaming.

## R-1.2.4 Platform-Native IO

Each platform SHALL use its native high-performance async IO path. C++ standard library file IO
(e.g. `std::fstream`, `std::ifstream`, `std::ofstream`, `std::async` file operations) SHALL NOT be
used. The following platform-native IO mechanisms SHALL be used:
- macOS: `dispatch_io` (Grand Central Dispatch async IO)
- Windows (D3D12): DirectStorage
- Windows (Vulkan): I/O completion ports (IOCP)
- Linux/SteamOS: `io_uring`

## R-1.2.5 Desktop Only

Only desktop platforms (macOS, Windows, Linux/SteamOS) SHALL be targeted. This keeps the hardware
baseline high enough to require mesh shaders and bindless resources universally.
