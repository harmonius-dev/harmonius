//! Toast and tray helpers (`R-14.2.3`).

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

use super::error::OsError;

/// Notification urgency hint.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Urgency {
    /// Low priority toast.
    Low,
    /// Standard toast.
    Normal,
    /// Critical alert.
    Critical,
}

/// Tray icon handle returned to callers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TrayIconHandle(pub u32);

static NEXT_TRAY: AtomicU32 = AtomicU32::new(1);

/// Stub notifications surface (records calls for tests).
#[derive(Debug, Default)]
pub struct Notifications {
    tray: Mutex<Vec<(TrayIconHandle, String)>>,
}

impl Notifications {
    /// Creates an empty notification hub.
    pub fn new() -> Self {
        Self::default()
    }

    /// Shows a toast-style notification (stub always succeeds).
    pub fn show_notification(
        &self,
        title: &str,
        body: &str,
        urgency: Urgency,
        icon: Option<&str>,
    ) -> Result<(), OsError> {
        let _ = (title, body, urgency, icon);
        Ok(())
    }

    /// Creates a tray icon entry tagged with `title`.
    pub fn create_tray_icon(
        &self,
        title: &str,
        menu_items: &[&str],
    ) -> Result<TrayIconHandle, OsError> {
        let id = TrayIconHandle(NEXT_TRAY.fetch_add(1, Ordering::Relaxed));
        let mut g = self.tray.lock().expect("tray mutex poisoned");
        let label = format!("{}:{:?}", title, menu_items);
        g.push((id, label));
        Ok(id)
    }

    /// Removes a tray icon previously returned.
    pub fn remove_tray_icon(&self, handle: TrayIconHandle) -> Result<(), OsError> {
        let mut g = self.tray.lock().expect("tray mutex poisoned");
        g.retain(|(h, _)| *h != handle);
        Ok(())
    }
}
