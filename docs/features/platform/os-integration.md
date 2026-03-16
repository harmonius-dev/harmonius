# 14.2 — OS Integration

## Clipboard and Dialogs

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|-------------|----------------|
| F-14.2.1 | Clipboard Access | Read and write plain text and image data to the system clipboard. Players use clipboard for pasting chat messages, sharing coordinates, copying build URLs, and exchanging guild recruitment text. Image clipboard support enables screenshot sharing without saving to disk. | R-14.2.1 | None | Windows uses `OpenClipboard`/`SetClipboardData` with COM wrappers; macOS uses `NSPasteboard` via Objective-C++ cxx wrappers; Linux uses X11 selections (`XCB_ATOM_CLIPBOARD`) or the `wl_data_device` protocol on Wayland. Clipboard operations must run asynchronously on Wayland since the compositor mediates data transfer. |
| F-14.2.2 | Native File Dialogs | Open file-picker and folder-picker dialogs for importing add-ons, selecting screenshot export paths, and loading user-provided assets. Dialogs run on a separate thread to avoid blocking the game loop and support file-type filters. | R-14.2.2 | None | Windows uses `IFileOpenDialog`/`IFileSaveDialog` COM interfaces; macOS uses `NSOpenPanel`/`NSSavePanel` via Objective-C++ wrappers; Linux uses the portal D-Bus interface (`org.freedesktop.portal.FileChooser`) for Flatpak/Snap compatibility, falling back to `zenity` or `kdialog`. |

## System Interaction

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|-------------|----------------|
| F-14.2.3 | System Notifications and Tray Icons | Display toast notifications for background events (guild invite received, auction sold, queue pop) when the game is minimized or the player is tabbed out. Optionally provide a system tray icon with a context menu for quick status checks. | R-14.2.3 | F-14.1.1 | Windows uses `Shell_NotifyIcon` and `ToastNotificationManager` (WinRT); macOS uses `NSUserNotificationCenter` or `UNUserNotificationCenter` via Objective-C++ wrappers; Linux uses `org.freedesktop.Notifications` D-Bus interface. Console platforms do not support OS-level notifications — use in-game UI instead. |
| F-14.2.4 | Drag and Drop | Accept files and data dragged from the OS desktop onto the game window. Useful for importing add-on archives, dropping screenshots into chat, or loading custom UI themes. The engine validates MIME types and file extensions before accepting the drop. | R-14.2.4 | F-14.1.1 | Windows uses `IDropTarget` COM interface registered via `RegisterDragDrop`; macOS uses `NSDraggingDestination` protocol on `NSWindow` via Objective-C++ wrappers; Linux X11 uses XDND protocol, Wayland uses `wl_data_device` drag-and-drop events. |

## Text Input

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|-------------|----------------|
| F-14.2.5 | Platform Keyboard Layouts and Dead Keys | Query the active keyboard layout and correctly interpret dead-key sequences (accented characters, compose sequences). Chat input in an MMO must work correctly for all Latin, Cyrillic, and other script layouts without requiring players to switch to a US layout. | R-14.2.5 | None | Windows uses `GetKeyboardLayout` and `ToUnicodeEx` for dead-key resolution; macOS uses `TISCopyCurrentKeyboardInputSource` and `UCKeyTranslate`; Linux uses `xkbcommon` for both X11 and Wayland layout handling. Layout-change events must be monitored to update the key-to-character mapping at runtime. |
| F-14.2.6 | Input Method Editor (IME) for CJK | Integrate with the OS input method editor to support Chinese, Japanese, and Korean text entry in chat, naming, and search fields. The engine provides a candidate window position hint and handles composition, commit, and candidate-list events. | R-14.2.6 | F-14.2.5, F-14.1.1 | Windows uses `ImmGetContext` / TSF (`ITfThreadMgr`); macOS uses `NSTextInputClient` protocol via Objective-C++ wrappers; Linux uses `IBus` or `Fcitx` via their respective D-Bus or direct C APIs. The candidate window must track the text cursor position in screen coordinates, updating as the game window moves or resizes. |

## Async API Surface

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|-------------|----------------|
| F-14.2.7 | Uniform Async/Await API for Clipboard and Dialogs | All clipboard read/write and file dialog operations expose a uniform `async fn` API regardless of platform. On Wayland, where the compositor mediates data transfer asynchronously, the async boundary maps directly to the protocol. On Windows and macOS, synchronous platform calls are wrapped in async by dispatching to a background thread, so callers always use `.await` without caring which platform requires true async. This eliminates platform-conditional calling conventions in gameplay and editor code. | R-14.2.7 | F-14.2.1, F-14.2.2 | Windows clipboard calls (`OpenClipboard`, `SetClipboardData`) and dialog calls (`IFileOpenDialog`) are dispatched to a background thread and wrapped in a `Future`. macOS `NSPasteboard` and `NSOpenPanel` calls are dispatched via GCD and wrapped in a `Future`. Linux Wayland data transfer is natively async; X11 selection calls are wrapped like Windows. |
