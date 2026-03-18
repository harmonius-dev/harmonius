# User Stories -- 14.2 OS Integration

| ID         | Persona                 | Features | Requirements |
|------------|-------------------------|----------|--------------|
| US-14.2.1  | player (P-23)           |          |              |
| US-14.2.2  | player (P-23)           |          |              |
| US-14.2.3  | player (P-23)           |          |              |
| US-14.2.4  | player (P-23)           |          |              |
| US-14.2.5  | game developer (P-15)   |          |              |
| US-14.2.6  | engine developer (P-26) |          |              |
| US-14.2.7  | engine developer (P-26) |          |              |
| US-14.2.8  | engine developer (P-26) |          |              |
| US-14.2.9  | engine tester (P-27)    |          |              |
| US-14.2.10 | engine tester (P-27)    |          |              |
| US-14.2.11 | game developer (P-15)   |          |              |
| US-14.2.12 | engine tester (P-27)    |          |              |
| US-14.2.13 | designer (P-5)          |          |              |
| US-14.2.14 | game developer (P-15)   |          |              |
| US-14.2.15 | engine developer (P-26) |          |              |

1. **US-14.2.1** — to copy and paste text and images to the system clipboard from within the game,
   so that I can share chat messages, coordinates, and screenshots without saving to disk first
2. **US-14.2.2** — to drag files from my desktop onto the game window to import add-on archives,
   screenshots, or custom UI themes, so that I can bring content into the game using a familiar OS
   interaction
3. **US-14.2.3** — toast notifications for guild invites, auction sales, and queue pops when the
   game is minimized, so that I do not miss time-sensitive events while multitasking
4. **US-14.2.4** — correct keyboard layout handling, dead-key sequences, and IME support for
   Chinese, Japanese, and Korean input, so that I can chat, name characters, and search in my
   language without switching to an English layout
5. **US-14.2.5** — native file-picker and folder-picker dialogs that run on a separate thread with
   file-type filters, so that users can select files for import or export without blocking the game
   loop
6. **US-14.2.6** — clipboard access through platform-native APIs (Win32 COM, NSPasteboard,
   X11/Wayland protocols), so that text and image clipboard operations work correctly across all
   platforms including async Wayland data transfer
7. **US-14.2.7** — IME integration using TSF on Windows, NSTextInputClient on macOS, and IBus/Fcitx
   on Linux, with candidate window positioning that tracks the text cursor, so that CJK input works
   correctly in all text fields across all platforms
8. **US-14.2.8** — drag-and-drop using IDropTarget on Windows, NSDraggingDestination on macOS, and
   XDND/wl_data_device on Linux, with MIME type and extension validation, so that only accepted file
   types are imported from drag operations
9. **US-14.2.9** — automated tests that verify dead-key sequences, compose sequences, and layout
   switching produce correct characters for Latin, Cyrillic, and CJK scripts, so that text input
   regressions are caught for all supported locales
10. **US-14.2.10** — tests that verify toast notifications display correctly using WinRT,
    NSUserNotificationCenter, and freedesktop D-Bus on each platform, so that notification delivery
    regressions are caught before release
11. **US-14.2.11** — consoles that lack OS-level notifications to fall back to in-game UI
    notifications, so that the notification system works consistently across PC and console
    platforms without platform-specific code in gameplay systems
12. **US-14.2.12** — tests that verify the IME candidate window repositions correctly when the game
    window moves or resizes, so that CJK text input remains usable in all window configurations
13. **US-14.2.13** — to configure which game events trigger OS notifications and their urgency
    levels, so that players are alerted for important events (queue pop, guild invite) but not
    spammed by low-priority updates
14. **US-14.2.14** — clipboard read/write and file dialog operations to expose `async fn` methods I
    can `.await` on every platform, so that I write one calling convention regardless of whether the
    OS call is synchronous or asynchronous underneath
15. **US-14.2.15** — synchronous clipboard and dialog calls on Windows and macOS dispatched to a
    background thread and wrapped in a `Future`, so that the public API is uniformly async and
    callers never block the game loop even on platforms with synchronous OS APIs
