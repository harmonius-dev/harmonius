//! Tamper-evident audit log using BLAKE3 over `prev_hash || entry_bytes` (design hash chain).

use blake3::Hasher;

/// One append-only audit record with explicit predecessor digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditEntry {
    /// Digest of the prior chain link (`[0; 32]` for the first entry).
    pub prev_hash: [u8; 32],
    /// Serialized event bytes included in the chain hash.
    pub body: Vec<u8>,
}

/// Append-only audit log with rolling chain tail.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AuditLog {
    entries: Vec<AuditEntry>,
    /// BLAKE3 digest after the last appended entry (`[0; 32]` when empty).
    tail_hash: [u8; 32],
}

impl AuditLog {
    /// Empty log whose implicit predecessor is the all-zero genesis digest.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            tail_hash: [0u8; 32],
        }
    }

    /// Appends `body` and advances the internal chain tail.
    pub fn append(&mut self, body: Vec<u8>) {
        let prev_hash = self.tail_hash;
        self.tail_hash = chain_digest(&prev_hash, &body);
        self.entries.push(AuditEntry { prev_hash, body });
    }

    /// Ordered entries for inspection (TC-15.7.6.1).
    pub fn entries(&self) -> &[AuditEntry] {
        &self.entries
    }

    /// Returns true when every link matches `BLAKE3(prev || body)`.
    pub fn validate_chain(&self) -> bool {
        let mut expect_prev = [0u8; 32];
        for entry in &self.entries {
            if entry.prev_hash != expect_prev {
                return false;
            }
            expect_prev = chain_digest(&entry.prev_hash, &entry.body);
        }
        expect_prev == self.tail_hash
    }

    /// Mutates entry `index` body bytes to simulate tampering (tests only).
    #[cfg(test)]
    pub fn tamper_body(&mut self, index: usize, byte: u8) {
        if let Some(e) = self.entries.get_mut(index) {
            if e.body.is_empty() {
                e.body.push(byte);
            } else {
                e.body[0] ^= byte;
            }
        }
    }
}

/// BLAKE3 digest of `prev_hash` concatenated with `body`.
pub fn chain_digest(prev_hash: &[u8; 32], body: &[u8]) -> [u8; 32] {
    let mut hasher = Hasher::new();
    hasher.update(prev_hash);
    hasher.update(body);
    *hasher.finalize().as_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-15.7.6.1 — sequential appends chain `prev_hash` to prior digest.
    #[test]
    fn tc_15_7_6_1_audit_hash_chain_append() {
        let mut log = AuditLog::new();
        log.append(b"e0".to_vec());
        log.append(b"e1".to_vec());
        log.append(b"e2".to_vec());
        let entries = log.entries();
        assert_eq!(entries[0].prev_hash, [0u8; 32]);
        assert_eq!(entries[1].prev_hash, chain_digest(&[0u8; 32], b"e0"));
        assert_eq!(entries[2].prev_hash, chain_digest(&entries[1].prev_hash, b"e1"));
        assert!(log.validate_chain());
    }

    /// TC-15.7.6.2 — tampering breaks validation at the edited entry.
    #[test]
    fn tc_15_7_6_2_audit_chain_validation_tamper() {
        let mut log = AuditLog::new();
        for i in 0u8..5 {
            log.append(vec![i]);
        }
        assert!(log.validate_chain());
        log.tamper_body(2, 0xFF);
        assert!(!log.validate_chain());
    }
}
