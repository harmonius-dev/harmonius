# 14.2 — OS Integration

## Clipboard and Dialogs

### F-14.2.1 Clipboard Access

Read and write plain text and image data to the system clipboard. Players use clipboard for
pasting chat messages, sharing coordinates, copying build URLs, and exchanging guild recruitment
text. Image clipboard support enables screenshot sharing without saving to disk.

- **Requirements:** R-14.2.1
- **Dependencies:** None
- **Platform notes:** Windows uses `OpenClipboard`/`SetClipboardData` with COM wrappers;
  macOS uses `NSPasteboard` via Objective-C++ cxx wrappers; Linux uses X11 selections
  (`XCB_ATOM_CLIPBOARD`) or the `wl_data_device` protocol on Wayland. Clipboard operations
  must run asynchronously on Wayland since the compositor mediates data transfer.

### F-14.2.2 Native File Dialogs

Open file-picker and folder-picker dialogs for importing add-ons, selecting screenshot export
paths, and loading user-provided assets. Dialogs run on a separate thread to avoid blocking the
game loop and support file-type filters.

- **Requirements:** R-14.2.2
- **Dependencies:** None
- **Platform notes:** Windows uses `IFileOpenDialog`/`IFileSaveDialog` COM interfaces; macOS
  uses `NSOpenPanel`/`NSSavePanel` via Objective-C++ wrappers; Linux uses the portal D-Bus
  interface (`org.freedesktop.portal.FileChooser`) for Flatpak/Snap compatibility, falling
  back to `zenity` or `kdialog`.

## System Interaction

### F-14.2.3 System Notifications and Tray Icons

Display toast notifications for background events (guild invite received, auction sold, queue
pop) when the game is minimized or the player is tabbed out. Optionally provide a system tray
icon with a context menu for quick status checks.

- **Requirements:** R-14.2.3
- **Dependencies:** F-14.1.1
- **Platform notes:** Windows uses `Shell_NotifyIcon` and `ToastNotificationManager` (WinRT);
  macOS uses `NSUserNotificationCenter` or `UNUserNotificationCenter` via Objective-C++
  wrappers; Linux uses `org.freedesktop.Notifications` D-Bus interface. Console platforms do
  not support OS-level notifications — use in-game UI instead.

### F-14.2.4 Drag and Drop

Accept files and data dragged from the OS desktop onto the game window. Useful for importing
add-on archives, dropping screenshots into chat, or loading custom UI themes. The engine
validates MIME types and file extensions before accepting the drop.

- **Requirements:** R-14.2.4
- **Dependencies:** F-14.1.1
- **Platform notes:** Windows uses `IDropTarget` COM interface registered via
  `RegisterDragDrop`; macOS uses `NSDraggingDestination` protocol on `NSWindow` via
  Objective-C++ wrappers; Linux X11 uses XDND protocol, Wayland uses `wl_data_device`
  drag-and-drop events.

## Text Input

### F-14.2.5 Platform Keyboard Layouts and Dead Keys

Query the active keyboard layout and correctly interpret dead-key sequences (accented characters,
compose sequences). Chat input in an MMO must work correctly for all Latin, Cyrillic, and other
script layouts without requiring players to switch to a US layout.

- **Requirements:** R-14.2.5
- **Dependencies:** None
- **Platform notes:** Windows uses `GetKeyboardLayout` and `ToUnicodeEx` for dead-key
  resolution; macOS uses `TISCopyCurrentKeyboardInputSource` and `UCKeyTranslate`; Linux uses
  `xkbcommon` for both X11 and Wayland layout handling. Layout-change events must be monitored
  to update the key-to-character mapping at runtime.

### F-14.2.6 Input Method Editor (IME) for CJK

Integrate with the OS input method editor to support Chinese, Japanese, and Korean text entry in
chat, naming, and search fields. The engine provides a candidate window position hint and handles
composition, commit, and candidate-list events.

- **Requirements:** R-14.2.6
- **Dependencies:** F-14.2.5, F-14.1.1
- **Platform notes:** Windows uses `ImmGetContext` / TSF (`ITfThreadMgr`); macOS uses
  `NSTextInputClient` protocol via Objective-C++ wrappers; Linux uses `IBus` or `Fcitx` via
  their respective D-Bus or direct C APIs. The candidate window must track the text cursor
  position in screen coordinates, updating as the game window moves or resizes.
