//! Binary payload writer for archived events.

/// Growable byte buffer used by [`crate::Event::archive`].
#[derive(Debug, Default, Clone)]
pub struct BlobWriter {
    bytes: Vec<u8>,
}

impl BlobWriter {
    /// Pre-allocate capacity for typical small telemetry payloads.
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(cap),
        }
    }

    /// Append raw bytes.
    pub fn write_all(&mut self, data: &[u8]) {
        self.bytes.extend_from_slice(data);
    }

    /// Finish writing and return the frozen blob.
    pub fn finish(self) -> Vec<u8> {
        self.bytes
    }
}
