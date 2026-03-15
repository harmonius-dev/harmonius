# R-14.2 — OS Integration Requirements

## Clipboard and Dialogs

### R-14.2.1 Clipboard Access

The engine **SHALL** read and write plain text and image data to the system clipboard on Windows,
macOS, and Linux (X11 and Wayland), with clipboard operations executing asynchronously so they
never block the game loop.

- **Derived from:** [F-14.2.1](../../features/platform/os-integration.md)
- **Rationale:** Players rely on the clipboard for pasting chat messages, sharing coordinates, and
  exchanging screenshots; blocking clipboard operations would cause frame hitches during gameplay.
- **Verification:** Integration test: on each platform, write a known UTF-8 string to the clipboard
  via the engine API and read it back; assert the round-tripped string is identical. Write an RGBA
  image to the clipboard, read it back, and assert pixel data matches. Measure the game loop frame
  time during clipboard operations and assert no frame exceeds the target frame budget.

### R-14.2.2 Native File Dialogs

The engine **SHALL** open native file-picker and folder-picker dialogs that support file-type
filters, executing on a separate thread so the game loop continues rendering while the dialog is
open.

- **Derived from:** [F-14.2.2](../../features/platform/os-integration.md)
- **Rationale:** File dialogs must not freeze the game window; the render loop and network
  processing must continue while the player browses the filesystem.
- **Verification:** Integration test: on each platform, open a file dialog with a filter for ".png"
  files; assert only .png files are shown. While the dialog is open, assert the game loop continues
  to render frames at the target frame rate (within 10% tolerance). Select a file and assert the
  returned path is valid and accessible.

## System Interaction

### R-14.2.3 System Notifications and Tray Icons

The engine **SHALL** display OS toast notifications for background events when the game window is
minimized or unfocused, and **SHALL** optionally provide a system tray icon with a context menu,
on platforms that support these features.

- **Derived from:** [F-14.2.3](../../features/platform/os-integration.md)
- **Rationale:** Players tabbed out of the game need to be alerted to time-sensitive events such as
  queue pops or auction results without requiring the game window to be visible.
- **Verification:** Integration test: minimize the game window, trigger a notification, and assert
  the OS notification API was invoked with the correct title and body text. On Windows, macOS, and
  Linux, verify the notification is visible in the system notification center. Create a tray icon
  with a context menu and assert menu item selection emits the correct engine event.

### R-14.2.4 Drag and Drop

The engine **SHALL** accept files and data dragged from the OS desktop onto the game window,
validating MIME types and file extensions before accepting the drop, on Windows, macOS, and
Linux (X11 and Wayland).

- **Derived from:** [F-14.2.4](../../features/platform/os-integration.md)
- **Rationale:** Drag-and-drop provides a natural workflow for importing add-ons and assets without
  navigating a file dialog, reducing friction for common content operations.
- **Verification:** Integration test: on each platform, drag a file with an accepted extension onto
  the game window and assert the drop event contains the correct file path. Drag a file with a
  rejected extension and assert the drop is rejected. Assert no frame time spike exceeds 2 ms during
  drag hover and drop handling.

## Text Input

### R-14.2.5 Platform Keyboard Layouts and Dead Keys

The engine **SHALL** query the active keyboard layout, correctly interpret dead-key sequences for
accented and composed characters, and update the key-to-character mapping within one frame of a
layout-change event.

- **Derived from:** [F-14.2.5](../../features/platform/os-integration.md)
- **Rationale:** Players using non-US keyboard layouts must be able to type correctly in chat and
  search fields without switching layouts; dead-key sequences are essential for Latin-extended and
  Cyrillic scripts.
- **Verification:** Integration test: on each platform, set the layout to French AZERTY and type a
  dead-key sequence (e.g., `^` then `e` to produce `ê`); assert the engine emits `ê` as the
  composed character. Switch the layout at runtime and assert the key-to-character mapping updates
  within one frame.

### R-14.2.6 Input Method Editor (IME) for CJK

The engine **SHALL** integrate with the OS input method editor for Chinese, Japanese, and Korean
text entry, providing a candidate window position hint in screen coordinates and handling
composition, commit, and candidate-list events.

- **Derived from:** [F-14.2.6](../../features/platform/os-integration.md)
- **Rationale:** CJK text entry requires IME integration; without it, players cannot type in their
  native language in chat, naming, or search fields.
- **Verification:** Integration test: on each platform with a CJK IME active, type a composition
  sequence and assert the engine receives composition update events with the correct intermediate
  text. Commit the composition and assert the final text matches the expected character. Move and
  resize the game window and assert the candidate window position updates to track the text cursor.
