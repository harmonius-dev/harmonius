# R-14.2 — OS Integration Requirements

## Clipboard and Dialogs

1. **R-14.2.1** — The engine **SHALL** read and write plain text and image data to the system
   clipboard on Windows, macOS, and Linux (X11 and Wayland), with operations executing
   asynchronously so they never block the game loop.
   - **Rationale:** Players rely on the clipboard for pasting chat messages, sharing coordinates,
     and exchanging screenshots; blocking operations would cause frame hitches.
   - **Verification:** Integration test per platform: write a UTF-8 string, read it back, assert
     round-trip equality. Write an RGBA image, read back, assert pixel match. Measure frame time
     during operations and assert no frame exceeds the target budget.

2. **R-14.2.2** — The engine **SHALL** open native file-picker and folder-picker dialogs with
   file-type filters, executing on a separate thread so the game loop continues rendering while the
   dialog is open.
   - **Rationale:** File dialogs must not freeze the game window; rendering and networking must
     continue while the player browses the filesystem.
   - **Verification:** Integration test per platform: open a file dialog filtered to ".png". Assert
     only .png files shown. Assert game loop continues rendering during dialog (within 10 %
     tolerance).

## System Interaction

3. **R-14.2.3** — The engine **SHALL** display OS toast notifications for background events when the
   game is minimized or unfocused, and **SHALL** optionally provide a system tray icon with a
   context menu on supported platforms.
   - **Rationale:** Players tabbed out need alerts for time-sensitive events without the game window
     being visible.
   - **Verification:** Integration test: minimize game, trigger notification, assert OS notification
     API invoked with correct title and body. Create tray icon with context menu; assert menu
     selection emits the correct engine event.

4. **R-14.2.4** — The engine **SHALL** accept files and data dragged from the OS desktop onto the
   game window, validating MIME types and file extensions before accepting the drop, on Windows,
   macOS, and Linux (X11 and Wayland).
   - **Rationale:** Drag-and-drop provides a natural workflow for importing add-ons and assets.
   - **Verification:** Integration test per platform: drag an accepted file type onto the window and
     assert the drop event contains the correct path. Drag a rejected type and assert rejection.
     Assert no frame spike exceeds 2 ms during drop handling.

## Text Input

5. **R-14.2.5** — The engine **SHALL** query the active keyboard layout, correctly interpret
   dead-key sequences for accented and composed characters, and update the key-to-character mapping
   within one frame of a layout-change event.
   - **Rationale:** Players using non-US layouts must type correctly without switching layouts; dead
     keys are essential for Latin-extended and Cyrillic.
   - **Verification:** Integration test per platform: set French AZERTY layout, type `^` then `e`,
     assert engine emits `e-hat`. Switch layout at runtime and assert mapping updates within one
     frame.

6. **R-14.2.6** — The engine **SHALL** integrate with the OS input method editor for CJK text entry,
   providing a candidate window position hint in screen coordinates and handling composition,
   commit, and candidate-list events.
   - **Rationale:** CJK text entry requires IME integration; without it, players cannot type in
     their native language.
   - **Verification:** Integration test per platform with CJK IME active: type a composition
     sequence, assert engine receives composition events. Commit and assert final text matches. Move
     window and assert candidate window tracks the text cursor.

## Async API Surface

7. **R-14.2.7** — The engine **SHALL** expose all clipboard read/write and file dialog operations as
   `async fn` methods returning `Future`s, providing a uniform calling convention across all
   platforms so callers always use `.await`.
   - **Rationale:** A uniform async API eliminates platform-conditional calling conventions. Callers
     need not know whether the underlying OS call is synchronous or asynchronous.
   - **Verification:** Unit test: call `Clipboard::read_text()` and `FileDialog::open_file()` from
     an async context on each platform; verify they compile and resolve without blocking.
     Integration test: invoke clipboard write followed by read using `.await` on all platforms and
     verify round-trip correctness.

## Error Handling

8. **R-14.2.8** — The engine **SHALL** return structured errors from clipboard, dialog,
   notification, and drag-and-drop operations, including platform-specific error codes mapped to
   engine error types, so callers can distinguish transient failures (clipboard locked) from
   permanent failures (unsupported operation).
   - **Rationale:** Structured errors enable callers to implement retry logic for transient failures
     and graceful degradation for unsupported operations.
   - **Verification:** Unit test: simulate a clipboard lock and assert the returned error is
     `ClipboardError::Locked`. Call an unsupported operation on a platform that lacks it and assert
     `ClipboardError::Unsupported`.

## Console Fallback

9. **R-14.2.9** — On platforms that lack OS-level notification or clipboard APIs (consoles), the
   engine **SHALL** fall back to in-game UI equivalents, so that gameplay code does not need
   platform-specific branching.
   - **Rationale:** Console platforms do not expose system-level clipboard or notification APIs. A
     unified fallback keeps gameplay code portable.
   - **Verification:** Integration test on console: trigger a notification and assert the in-game UI
     displays it. Call clipboard read and assert the in-game clipboard provides the data.
