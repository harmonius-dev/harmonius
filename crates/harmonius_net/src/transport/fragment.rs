//! Fragmentation and MTU helpers.
//!
//! Ordered split + concat only. Full `ReassemblyBuffer` (group ids, expiry, duplicate detection,
//! `FragmentTimeout` events) matches the design doc and stays for a follow-up transport slice.

/// Split `payload` into chunks of at most `chunk_payload` bytes (excludes any header the caller adds).
pub fn fragment_payload(payload: &[u8], chunk_payload: usize) -> Vec<Vec<u8>> {
    payload
        .chunks(chunk_payload.max(1))
        .map(|c| c.to_vec())
        .collect()
}

/// Reassemble fragments in order.
pub fn reassemble_fragments(frags: &[Vec<u8>]) -> Vec<u8> {
    frags.iter().flatten().copied().collect()
}

/// Effective MTU after PMTUD timeout (bytes, payload budget in tests).
pub fn effective_mtu(pmtud_blocked: bool) -> u16 {
    if pmtud_blocked {
        1200
    } else {
        1400
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.1.6.1 — 64 KiB over 1400-byte MTU reassembles losslessly.
    #[test]
    fn test_fragment_reassemble_64k() {
        let payload: Vec<u8> = (0..65_536).map(|i| (i % 251) as u8).collect();
        let mtu_payload = 1300usize;
        let frags = fragment_payload(&payload, mtu_payload);
        assert!(frags.len() > 1);
        let got = reassemble_fragments(&frags);
        assert_eq!(got, payload);
    }

    /// TC-8.1.6.2 — blocked PMTUD falls back to 1200-byte effective payload budget.
    #[test]
    fn test_pmtud_fallback_1200() {
        assert_eq!(effective_mtu(true), 1200);
        assert_eq!(effective_mtu(false), 1400);
    }
}
