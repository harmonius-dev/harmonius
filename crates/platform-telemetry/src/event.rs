//! Event trait and tagged archive helpers (R-14.5.6).

use serde::{Deserialize, Serialize};

use crate::blob::BlobWriter;
use crate::types::{PiiClass, Scope};

/// Application-defined telemetry event.
pub trait Event: Sized {
    /// Owning schema numeric id.
    const SCHEMA_ID: u32;
    /// Consent scope for this event type.
    const SCOPE: Scope;
    /// Serialize payload bytes.
    fn archive(&self, out: &mut BlobWriter);
}

/// Wire bundle preserving per-field PII metadata for compliance tooling.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaggedPayload {
    /// Raw field bytes.
    pub data: Vec<u8>,
    /// Parallel PII tags for exported fields.
    pub pii: Vec<(String, PiiClass)>,
}

impl TaggedPayload {
    /// Serialize to an opaque blob stored inside [`crate::ring_buffer::EventRecord::payload`].
    pub fn encode(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    /// Decode from a payload blob.
    pub fn decode(bytes: &[u8]) -> serde_json::Result<Self> {
        serde_json::from_slice(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pii_class_tag_preserved_in_archive() {
        let tagged = TaggedPayload {
            data: vec![1, 2, 3],
            pii: vec![
                ("player_id".to_owned(), PiiClass::Personal),
                ("frame_ms".to_owned(), PiiClass::None),
            ],
        };
        let bytes = tagged.encode().unwrap();
        let round = TaggedPayload::decode(&bytes).unwrap();
        assert_eq!(round, tagged);
    }
}
