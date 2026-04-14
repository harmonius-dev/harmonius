//! Drag-and-drop filtering (`R-14.2.4`).

use std::collections::VecDeque;
use std::sync::Mutex;

use super::error::OsError;

/// MIME / extension filter for drag operations.
#[derive(Clone, Debug)]
pub struct MimeFilter {
    /// Accepted MIME types.
    pub mime_types: Vec<&'static str>,
    /// Accepted file extensions (without dot).
    pub extensions: Vec<&'static str>,
}

/// Drag surface response.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DragResponse {
    /// Accept the drag.
    Accept,
    /// Reject the drag.
    Reject,
}

/// Simulated drag events for tests.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DragEvent {
    /// Cursor entered drop target.
    Enter {
        /// MIME types offered by the drag source.
        mime_types: Vec<String>,
    },
}

/// Registers filters and replays synthetic drags.
pub struct DragDrop {
    filter: Mutex<Option<MimeFilter>>,
    events: Mutex<VecDeque<DragEvent>>,
}

impl DragDrop {
    /// Creates an empty drag/drop hub.
    pub fn new() -> Self {
        Self {
            filter: Mutex::new(None),
            events: Mutex::new(VecDeque::new()),
        }
    }

    /// Installs an acceptance filter.
    pub fn register_filter(&self, filter: MimeFilter) -> Result<(), OsError> {
        *self.filter.lock().expect("drag mutex poisoned") = Some(filter);
        Ok(())
    }

    /// Simulates a drag enter with MIME list (test hook).
    pub fn simulate_enter(&self, mime_types: &[&str]) -> Result<(), OsError> {
        let f = self.filter.lock().expect("drag mutex poisoned").clone();
        let Some(filter) = f else {
            return Ok(());
        };
        let offered: Vec<String> = mime_types.iter().map(|s| (*s).into()).collect();
        let ok = offered.iter().any(|m| filter.mime_types.contains(&m.as_str()));
        if ok {
            self.events
                .lock()
                .expect("drag mutex poisoned")
                .push_back(DragEvent::Enter {
                    mime_types: offered,
                });
        }
        Ok(())
    }

    /// Pops the next filtered drag event.
    pub fn poll_event(&self) -> Option<DragEvent> {
        self.events.lock().expect("drag mutex poisoned").pop_front()
    }

    /// Returns whether the simulated drag should be rejected outright.
    pub fn respond(&self, mime_types: &[&str]) -> DragResponse {
        let f = self.filter.lock().expect("drag mutex poisoned").clone();
        let Some(filter) = f else {
            return DragResponse::Accept;
        };
        let ok = mime_types.iter().any(|m| filter.mime_types.contains(m));
        if ok {
            DragResponse::Accept
        } else {
            DragResponse::Reject
        }
    }
}

impl Default for DragDrop {
    fn default() -> Self {
        Self::new()
    }
}
