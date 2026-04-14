//! Shared value types for telemetry.

use serde::{Deserialize, Serialize};

/// Stable identifier for an event schema.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SchemaId(pub u32);

/// Telemetry consent scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Scope {
    /// Engine performance, crashes, frame timings.
    Engine,
    /// Gameplay-facing analytics events.
    GameLogic,
}

/// PII classification for schema fields.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PiiClass {
    /// No personal data.
    None,
    /// Personal identifiers or stable hardware fingerprints.
    Personal,
    /// Sensitive classes are rejected by schema validation.
    Sensitive,
}

/// Opaque anonymous identifier for a local install.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AnonId(pub [u8; 16]);

impl AnonId {
    /// Pseudorandom identifier for first-run flows (not cryptographic).
    pub fn random() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        use std::time::{SystemTime, UNIX_EPOCH};

        static COUNTER: AtomicU64 = AtomicU64::new(1);
        let mut state = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64 ^ COUNTER.fetch_add(1, Ordering::Relaxed))
            .unwrap_or_else(|_| COUNTER.fetch_add(1, Ordering::Relaxed));

        let mut bytes = [0_u8; 16];
        for chunk in bytes.chunks_mut(8) {
            // xorshift64*
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            chunk.copy_from_slice(&state.to_le_bytes());
        }
        Self(bytes)
    }

    /// Deterministic id for tests.
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }
}
