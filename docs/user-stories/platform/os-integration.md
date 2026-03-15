# User Stories -- 14.2 OS Integration

## US-14.2.1 Paste Chat Messages and Copy Coordinates

**As a** player (P-23), **I want** to copy and paste text and images to the system clipboard
from within the game, **so that** I can share chat messages, coordinates, and screenshots
without saving to disk first.

## US-14.2.2 Import Files via Drag and Drop

**As a** player (P-23), **I want** to drag files from my desktop onto the game window to
import add-on archives, screenshots, or custom UI themes, **so that** I can bring content
into the game using a familiar OS interaction.

## US-14.2.3 Receive Notifications When Tabbed Out

**As a** player (P-23), **I want** toast notifications for guild invites, auction sales,
and queue pops when the game is minimized, **so that** I do not miss time-sensitive events
while multitasking.

## US-14.2.4 Type in My Native Language Including CJK

**As a** player (P-23), **I want** correct keyboard layout handling, dead-key sequences, and
IME support for Chinese, Japanese, and Korean input, **so that** I can chat, name
characters, and search in my language without switching to an English layout.

## US-14.2.5 Open Native File Dialogs Without Freezing the Game

**As a** game developer (P-15), **I want** native file-picker and folder-picker dialogs that
run on a separate thread with file-type filters, **so that** users can select files for
import or export without blocking the game loop.

## US-14.2.6 Implement Clipboard with Platform-Native APIs

**As an** engine developer (P-26), **I want** clipboard access through platform-native APIs
(Win32 COM, NSPasteboard, X11/Wayland protocols), **so that** text and image clipboard
operations work correctly across all platforms including async Wayland data transfer.

## US-14.2.7 Implement IME Integration for Each Platform

**As an** engine developer (P-26), **I want** IME integration using TSF on Windows,
NSTextInputClient on macOS, and IBus/Fcitx on Linux, with candidate window positioning
that tracks the text cursor, **so that** CJK input works correctly in all text fields
across all platforms.

## US-14.2.8 Handle Drag and Drop with MIME Validation

**As an** engine developer (P-26), **I want** drag-and-drop using IDropTarget on Windows,
NSDraggingDestination on macOS, and XDND/wl_data_device on Linux, with MIME type and
extension validation, **so that** only accepted file types are imported from drag
operations.

## US-14.2.9 Test Keyboard Layouts Across All Supported Scripts

**As an** engine tester (P-27), **I want** automated tests that verify dead-key sequences,
compose sequences, and layout switching produce correct characters for Latin, Cyrillic, and
CJK scripts, **so that** text input regressions are caught for all supported locales.

## US-14.2.10 Verify Notifications on All Platforms

**As an** engine tester (P-27), **I want** tests that verify toast notifications display
correctly using WinRT, NSUserNotificationCenter, and freedesktop D-Bus on each platform,
**so that** notification delivery regressions are caught before release.

## US-14.2.11 Use Console-Appropriate Notification Patterns

**As a** game developer (P-15), **I want** consoles that lack OS-level notifications to fall
back to in-game UI notifications, **so that** the notification system works consistently
across PC and console platforms without platform-specific code in gameplay systems.

## US-14.2.12 Verify IME Candidate Window Tracks Cursor During Resize

**As an** engine tester (P-27), **I want** tests that verify the IME candidate window
repositions correctly when the game window moves or resizes, **so that** CJK text input
remains usable in all window configurations.

## US-14.2.13 Design Notification Urgency Levels

**As a** designer (P-5), **I want** to configure which game events trigger OS notifications
and their urgency levels, **so that** players are alerted for important events (queue pop,
guild invite) but not spammed by low-priority updates.
