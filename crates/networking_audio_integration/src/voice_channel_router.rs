//! Flat-bucket voice channel routing without `HashMap` on the hot path.

use crate::connection::ConnectionId;
use crate::voice_packet::VoiceChannelId;

/// Subscriber entry for routing tables.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ConnectionSlot(pub ConnectionId);

/// Lookup table for voice channel routing.
#[derive(Debug, Default)]
pub struct VoiceChannelRouter {
    buckets: [Vec<(u32, Vec<ConnectionSlot>)>; 4],
}

impl VoiceChannelRouter {
    /// Returns the subscriber list for `channel`, or an empty slice if unknown.
    #[must_use]
    pub fn lookup(&self, channel: VoiceChannelId) -> &[ConnectionSlot] {
        match channel {
            VoiceChannelId::Proximity => self.buckets[0]
                .iter()
                .find(|(k, _)| *k == 0)
                .map(|(_, slots)| slots.as_slice())
                .unwrap_or(&[]),
            VoiceChannelId::Party(id) => Self::lookup_inner(&self.buckets[1], id),
            VoiceChannelId::Raid(id) => Self::lookup_inner(&self.buckets[2], id),
            VoiceChannelId::Custom(id) => Self::lookup_inner(&self.buckets[3], id),
        }
    }

    fn lookup_inner(bucket: &[(u32, Vec<ConnectionSlot>)], id: u32) -> &[ConnectionSlot] {
        match bucket.binary_search_by_key(&id, |(k, _)| *k) {
            Ok(i) => bucket[i].1.as_slice(),
            Err(_) => &[],
        }
    }

    /// Inserts `slot` for `channel`, preserving sort order on the inner id key.
    pub fn insert(&mut self, channel: VoiceChannelId, slot: ConnectionSlot) {
        if channel == VoiceChannelId::Proximity {
            let bucket = &mut self.buckets[0];
            if let Some((_, slots)) = bucket.iter_mut().find(|(k, _)| *k == 0) {
                if !slots.contains(&slot) {
                    slots.push(slot);
                }
            } else {
                bucket.push((0, vec![slot]));
            }
            return;
        }
        let (bucket_idx, inner) = Self::key(channel);
        let bucket = &mut self.buckets[bucket_idx];
        match bucket.binary_search_by_key(&inner, |(k, _)| *k) {
            Ok(i) => {
                if !bucket[i].1.contains(&slot) {
                    bucket[i].1.push(slot);
                }
            }
            Err(pos) => bucket.insert(pos, (inner, vec![slot])),
        }
    }

    /// Removes `slot` from `channel`, preserving sort order.
    pub fn remove(&mut self, channel: VoiceChannelId, slot: ConnectionSlot) {
        if channel == VoiceChannelId::Proximity {
            let bucket = &mut self.buckets[0];
            if let Some((_, slots)) = bucket.iter_mut().find(|(k, _)| *k == 0) {
                slots.retain(|s| *s != slot);
                if slots.is_empty() {
                    bucket.retain(|(k, _)| *k != 0);
                }
            }
            return;
        }
        let (bucket_idx, inner) = Self::key(channel);
        let bucket = &mut self.buckets[bucket_idx];
        if let Ok(i) = bucket.binary_search_by_key(&inner, |(k, _)| *k) {
            bucket[i].1.retain(|s| *s != slot);
            if bucket[i].1.is_empty() {
                bucket.remove(i);
            }
        }
    }

    fn key(channel: VoiceChannelId) -> (usize, u32) {
        match channel {
            VoiceChannelId::Proximity => (0, 0),
            VoiceChannelId::Party(id) => (1, id),
            VoiceChannelId::Raid(id) => (2, id),
            VoiceChannelId::Custom(id) => (3, id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-IR-4.3.1.5 — many lookups on many channels stay panic-free (no `HashMap` on path).
    #[test]
    fn tc_ir_4_3_1_5_router_hot_path_lookups() {
        let mut r = VoiceChannelRouter::default();
        for i in 0..64 {
            r.insert(
                VoiceChannelId::Party(i),
                ConnectionSlot(ConnectionId(i as u16)),
            );
            r.insert(
                VoiceChannelId::Raid(i),
                ConnectionSlot(ConnectionId((i + 100) as u16)),
            );
        }
        r.insert(VoiceChannelId::Proximity, ConnectionSlot(ConnectionId(9)));
        for _ in 0..1000 {
            let _ = r.lookup(VoiceChannelId::Party(33));
            let _ = r.lookup(VoiceChannelId::Raid(12));
            let _ = r.lookup(VoiceChannelId::Proximity);
        }
        assert_eq!(r.lookup(VoiceChannelId::Party(33)).len(), 1);
    }
}
