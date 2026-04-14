//! Clipboard staging (`R-14.2.1`).

use std::sync::Mutex;

use super::error::OsError;

/// Pixel buffer for clipboard images.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageData {
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
    /// RGBA8 pixels row-major.
    pub pixels: Vec<u8>,
}

/// ECS-facing clipboard cache (read-only resource snapshot).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ClipboardState {
    /// Last published text payload.
    pub text: Option<String>,
    /// Last published image payload.
    pub image: Option<ImageData>,
}

/// Main-thread clipboard facade with frame publish semantics.
pub struct Clipboard {
    pending: Mutex<ClipboardState>,
    published: Mutex<ClipboardState>,
}

impl Clipboard {
    /// Creates an empty clipboard bridge.
    pub fn new() -> Self {
        Self {
            pending: Mutex::new(ClipboardState::default()),
            published: Mutex::new(ClipboardState::default()),
        }
    }

    /// Queues UTF-8 text for the next publish.
    pub fn set_text(&self, text: &str) -> Result<(), OsError> {
        let mut p = self.pending.lock().expect("clipboard mutex poisoned");
        p.text = Some(text.into());
        Ok(())
    }

    /// Queues an RGBA image for the next publish.
    pub fn set_image(&self, image: &ImageData) -> Result<(), OsError> {
        let mut p = self.pending.lock().expect("clipboard mutex poisoned");
        p.image = Some(image.clone());
        Ok(())
    }

    /// Publishes pending clipboard contents (call once per frame from main).
    pub fn publish_frame(&self) {
        let pending = self.pending.lock().expect("clipboard mutex poisoned").clone();
        *self.published.lock().expect("clipboard mutex poisoned") = pending;
    }

    /// Reads the published ECS snapshot.
    pub fn published(&self) -> ClipboardState {
        self.published
            .lock()
            .expect("clipboard mutex poisoned")
            .clone()
    }
}

impl Default for Clipboard {
    fn default() -> Self {
        Self::new()
    }
}
