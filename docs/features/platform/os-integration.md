# 14.2 — OS Integration

## Clipboard and Dialogs

| ID       | Feature             | Requirements |
|----------|---------------------|--------------|
| F-14.2.1 | Clipboard Access    | R-14.2.1     |
| F-14.2.2 | Native File Dialogs | R-14.2.2     |

1. **F-14.2.1** — Read and write plain text and image data to the system clipboard. Players use
   clipboard for pasting chat messages, sharing coordinates, copying build URLs, and exchanging
   guild recruitment text. Image clipboard support enables screenshot sharing without saving to
   disk.
   - **Platform:** Windows uses `OpenClipboard`/`SetClipboardData` with COM wrappers; macOS uses
     `NSPasteboard` via Swift swift-bridge bindings; Linux uses X11 selections
     (`XCB_ATOM_CLIPBOARD`) or the `wl_data_device` protocol on Wayland. Clipboard operations must
     run asynchronously on Wayland since the compositor mediates data transfer.
2. **F-14.2.2** — Open file-picker and folder-picker dialogs for importing add-ons, selecting
   screenshot export paths, and loading user-provided assets. Dialogs run on a separate thread to
   avoid blocking the game loop and support file-type filters.
   - **Platform:** Windows uses `IFileOpenDialog`/`IFileSaveDialog` COM interfaces; macOS uses
     `NSOpenPanel`/`NSSavePanel` via Swift swift-bridge bindings; Linux uses the portal D-Bus
     interface (`org.freedesktop.portal.FileChooser`) for Flatpak/Snap compatibility, falling back
     to `zenity` or `kdialog`.

## System Interaction

| ID       | Feature                             | Requirements |
|----------|-------------------------------------|--------------|
| F-14.2.3 | System Notifications and Tray Icons | R-14.2.3     |
| F-14.2.4 | Drag and Drop                       | R-14.2.4     |

1. **F-14.2.3** — Display toast notifications for background events (guild invite received, auction
   sold, queue pop) when the game is minimized or the player is tabbed out. Optionally provide a
   system tray icon with a context menu for quick status checks.
   - **Deps:** F-14.1.1
   - **Platform:** Windows uses `Shell_NotifyIcon` and `ToastNotificationManager` (WinRT); macOS
     uses `NSUserNotificationCenter` or `UNUserNotificationCenter` via Swift swift-bridge bindings;
     Linux uses `org.freedesktop.Notifications` D-Bus interface. Console platforms do not support
     OS-level notifications — use in-game UI instead.
2. **F-14.2.4** — Accept files and data dragged from the OS desktop onto the game window. Useful for
   importing add-on archives, dropping screenshots into chat, or loading custom UI themes. The
   engine validates MIME types and file extensions before accepting the drop.
   - **Deps:** F-14.1.1
   - **Platform:** Windows uses `IDropTarget` COM interface registered via `RegisterDragDrop`; macOS
     uses `NSDraggingDestination` protocol on `NSWindow` via Swift swift-bridge bindings; Linux X11
     uses XDND protocol, Wayland uses `wl_data_device` drag-and-drop events.

## Text Input

| ID       | Feature                                 | Requirements |
|----------|-----------------------------------------|--------------|
| F-14.2.5 | Platform Keyboard Layouts and Dead Keys | R-14.2.5     |
| F-14.2.6 | Input Method Editor (IME) for CJK       | R-14.2.6     |

1. **F-14.2.5** — Query the active keyboard layout and correctly interpret dead-key sequences
   (accented characters, compose sequences). Chat input in an MMO must work correctly for all Latin,
   Cyrillic, and other script layouts without requiring players to switch to a US layout.
   - **Platform:** Windows uses `GetKeyboardLayout` and `ToUnicodeEx` for dead-key resolution; macOS
     uses `TISCopyCurrentKeyboardInputSource` and `UCKeyTranslate`; Linux uses `xkbcommon` for both
     X11 and Wayland layout handling. Layout-change events must be monitored to update the
     key-to-character mapping at runtime.
2. **F-14.2.6** — Integrate with the OS input method editor to support Chinese, Japanese, and Korean
   text entry in chat, naming, and search fields. The engine provides a candidate window position
   hint and handles composition, commit, and candidate-list events.
   - **Deps:** F-14.2.5, F-14.1.1
   - **Platform:** Windows uses `ImmGetContext` / TSF (`ITfThreadMgr`); macOS uses
     `NSTextInputClient` protocol via Swift swift-bridge bindings; Linux uses `IBus` or `Fcitx` via
     their respective D-Bus or direct C APIs. The candidate window must track the text cursor
     position in screen coordinates, updating as the game window moves or resizes.

## Async API Surface

| ID       | Feature                                           | Requirements |
|----------|---------------------------------------------------|--------------|
| F-14.2.7 | Uniform Async/Await API for Clipboard and Dialogs | R-14.2.7     |

1. **F-14.2.7** — All clipboard read/write and file dialog operations expose a uniform `async fn`
   API regardless of platform. On Wayland, where the compositor mediates data transfer
   asynchronously, the async boundary maps directly to the protocol. On Windows and macOS,
   synchronous platform calls are wrapped in async by dispatching to a background thread, so callers
   always use `.await` without caring which platform requires true async. This eliminates
   platform-conditional calling conventions in gameplay and editor code.
   - **Deps:** F-14.2.1, F-14.2.2
   - **Platform:** Windows clipboard calls (`OpenClipboard`, `SetClipboardData`) and dialog calls
     (`IFileOpenDialog`) are dispatched to a background thread and wrapped in a `Future`. macOS
     `NSPasteboard` and `NSOpenPanel` calls are dispatched via GCD and wrapped in a `Future`. Linux
     Wayland data transfer is natively async; X11 selection calls are wrapped like Windows.
