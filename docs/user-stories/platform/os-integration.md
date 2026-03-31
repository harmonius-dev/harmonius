# User Stories -- 14.2 OS Integration

| ID         | Persona                 |
|------------|-------------------------|
| US-14.2.1  | player (P-23)           |
| US-14.2.2  | player (P-23)           |
| US-14.2.3  | player (P-23)           |
| US-14.2.4  | player (P-23)           |
| US-14.2.5  | player (P-23)           |
| US-14.2.6  | player (P-23)           |
| US-14.2.7  | game developer (P-15)   |
| US-14.2.8  | game developer (P-15)   |
| US-14.2.9  | game developer (P-15)   |
| US-14.2.10 | game developer (P-15)   |
| US-14.2.11 | designer (P-5)          |
| US-14.2.12 | engine developer (P-26) |
| US-14.2.13 | engine developer (P-26) |
| US-14.2.14 | engine developer (P-26) |
| US-14.2.15 | engine tester (P-27)    |
| US-14.2.16 | engine tester (P-27)    |
| US-14.2.17 | engine tester (P-27)    |

## Clipboard and Dialogs

1. **US-14.2.1** — **As a** player (P-23), **I want** to copy and paste text from within the game to
   the system clipboard, **so that** I can share chat messages and coordinates with external apps.
2. **US-14.2.2** — **As a** player (P-23), **I want** to copy a screenshot to the system clipboard
   without saving to disk first, **so that** I can paste it directly into chat or social media.
3. **US-14.2.3** — **As a** game developer (P-15), **I want** native file-picker and folder-picker
   dialogs with file-type filters that run without blocking the game loop, **so that** users can
   select files for import or export seamlessly.

## System Interaction

4. **US-14.2.4** — **As a** player (P-23), **I want** toast notifications for guild invites, auction
   sales, and queue pops when the game is minimized, **so that** I do not miss time-sensitive events
   while multitasking.
5. **US-14.2.5** — **As a** player (P-23), **I want** to drag files from my desktop onto the game
   window to import add-on archives or custom themes, **so that** I can bring content in using a
   familiar OS interaction.
6. **US-14.2.6** — **As a** player (P-23), **I want** the game to reject dragged files with
   unsupported types before accepting the drop, **so that** I get clear feedback instead of an error
   after the fact.

## Text Input

7. **US-14.2.7** — **As a** player (P-23), **I want** my keyboard layout and dead-key sequences
   handled correctly for accented characters, **so that** I can type in my language without
   switching to an English layout.
8. **US-14.2.8** — **As a** player (P-23), **I want** IME support for Chinese, Japanese, and Korean
   input, **so that** I can type in CJK characters in chat, naming, and search fields.

## Game Developer -- API

9. **US-14.2.9** — **As a** game developer (P-15), **I want** clipboard read/write and file dialog
   operations to expose `async fn` methods I can `.await`, **so that** I write one calling
   convention regardless of the underlying platform.
10. **US-14.2.10** — **As a** game developer (P-15), **I want** consoles that lack OS-level
    notifications to fall back to in-game UI notifications, **so that** the notification system
    works consistently across PC and console without platform branching.

## Designer -- Configuration

11. **US-14.2.11** — **As a** designer (P-5), **I want** to configure which game events trigger OS
    notifications and their urgency levels, **so that** players are alerted for important events but
    not spammed by low-priority updates.

## Engine Developer -- Platform Backends

12. **US-14.2.12** — **As an** engine developer (P-26), **I want** clipboard access through
    platform-native APIs (Win32 COM, NSPasteboard, X11/Wayland), with Wayland's async data transfer
    handled natively, **so that** clipboard operations work correctly on all platforms.
13. **US-14.2.13** — **As an** engine developer (P-26), **I want** IME integration using TSF on
    Windows, NSTextInputClient on macOS, and IBus/Fcitx on Linux, with the candidate window tracking
    the text cursor, **so that** CJK input works in all text fields.
14. **US-14.2.14** — **As an** engine developer (P-26), **I want** synchronous clipboard and dialog
    calls on Windows and macOS dispatched to a background thread and wrapped in a `Future`,
    **so that** the public API is uniformly async on every platform.

## Engine Tester -- Validation

15. **US-14.2.15** — **As an** engine tester (P-27), **I want** tests that verify dead-key
    sequences, compose sequences, and layout switching produce correct characters for Latin,
    Cyrillic, and CJK scripts, **so that** text input regressions are caught for all supported
    locales.
16. **US-14.2.16** — **As an** engine tester (P-27), **I want** tests that verify the IME candidate
    window repositions correctly when the game window moves or resizes, **so that** CJK text input
    remains usable in all window configurations.
17. **US-14.2.17** — **As an** engine tester (P-27), **I want** tests that verify toast
    notifications display correctly using platform-native APIs on each OS, **so that** notification
    delivery regressions are caught before release.
