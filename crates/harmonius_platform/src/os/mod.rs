//! OS integration facades (`R-14.2`).

mod clipboard;
mod drag_drop;
mod error;
mod file_dialog;
mod ime;
mod keyboard;
mod notifications;

pub use clipboard::{Clipboard, ClipboardState, ImageData};
pub use drag_drop::{DragDrop, DragEvent, DragResponse, MimeFilter};
pub use error::OsError;
pub use file_dialog::FileDialog;
pub use ime::{Ime, ImeEvent, ImePosition};
pub use keyboard::{DeadKeyResult, Keyboard, KeyboardLayout};
pub use notifications::{Notifications, TrayIconHandle, Urgency};
