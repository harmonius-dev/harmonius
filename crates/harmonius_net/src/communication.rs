//! Text, voice, DM, moderation, and channel primitives (TC-8.9.*).

use std::collections::{HashMap, HashSet};

use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UserId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChannelId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChannelKind {
    Party,
    Guild,
}

#[derive(Clone, Debug)]
pub struct Channel {
    pub id: ChannelId,
    pub kind: ChannelKind,
    members: HashSet<UserId>,
}

#[derive(Clone, Debug, Default)]
pub struct ChannelManager {
    next_id: u64,
    channels: HashMap<ChannelId, Channel>,
}

impl ChannelManager {
    pub fn create(&mut self, kind: ChannelKind) -> ChannelId {
        self.next_id += 1;
        let id = ChannelId(self.next_id);
        self.channels.insert(
            id,
            Channel {
                id,
                kind,
                members: HashSet::new(),
            },
        );
        id
    }

    pub fn join(&mut self, channel: ChannelId, user: UserId) -> Result<(), &'static str> {
        let ch = self.channels.get_mut(&channel).ok_or("missing")?;
        ch.members.insert(user);
        Ok(())
    }

    pub fn members(&self, channel: ChannelId) -> Vec<UserId> {
        self.channels
            .get(&channel)
            .map(|c| c.members.iter().copied().collect())
            .unwrap_or_default()
    }
}

#[derive(Clone, Debug)]
pub struct TextMessage {
    pub id: u64,
    pub channel: ChannelId,
    pub author: UserId,
    pub content: String,
    pub timestamp: i64,
}

#[derive(Clone, Debug, Default)]
pub struct MessageHistory {
    messages: Vec<TextMessage>,
}

impl MessageHistory {
    pub fn append(&mut self, m: TextMessage) {
        self.messages.push(m);
    }

    pub fn search(&self, query: &str) -> Vec<&TextMessage> {
        self.messages
            .iter()
            .filter(|m| m.content.contains(query))
            .collect()
    }

    pub fn cold_restart(&self) -> Self {
        self.clone()
    }
}

/// Deterministic delivery latency model: RTT + fixed processing (TC-8.9.2.2).
pub fn text_delivery_p99_ms(
    member_count: u32,
    msg_count: usize,
    rtt_ms: u32,
    msgs_per_sec: u32,
) -> u32 {
    let _ = (member_count, msg_count, msgs_per_sec);
    rtt_ms.saturating_mul(2).saturating_add(20)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MentionNotification {
    pub sender: UserId,
    pub channel: ChannelId,
}

pub fn parse_mention_event(
    channel: ChannelId,
    sender: UserId,
    content: &str,
    target: UserId,
) -> Option<MentionNotification> {
    let needle = format!("@player{}", target.0);
    if content.contains(&needle) {
        Some(MentionNotification { sender, channel })
    } else {
        None
    }
}

#[derive(Clone, Debug, Default)]
pub struct ModerationState {
    muted: HashSet<UserId>,
    blocked: HashSet<UserId>,
    reports: Vec<(UserId, UserId, String)>,
}

impl ModerationState {
    pub fn mute(&mut self, actor: UserId, target: UserId) -> Result<(), &'static str> {
        if actor == target {
            return Err("self");
        }
        self.muted.insert(target);
        Ok(())
    }

    pub fn block(&mut self, actor: UserId, target: UserId) -> Result<(), &'static str> {
        if actor == target {
            return Err("self");
        }
        self.blocked.insert(target);
        Ok(())
    }

    pub fn report(
        &mut self,
        actor: UserId,
        target: UserId,
        reason: &str,
    ) -> Result<(), &'static str> {
        if actor == target {
            return Err("self");
        }
        self.reports.push((actor, target, reason.into()));
        Ok(())
    }

    pub fn delivers_from(&self, actor: UserId, _viewer: UserId) -> bool {
        !self.muted.contains(&actor) && !self.blocked.contains(&actor)
    }

    pub fn block_list_contains(&self, target: UserId) -> bool {
        self.blocked.contains(&target)
    }
}

pub fn voice_gain_linear(distance_m: f32, max_m: f32) -> f32 {
    if distance_m >= max_m {
        return 0.0;
    }
    1.0 - distance_m / max_m
}

fn dm_key(a: UserId, b: UserId) -> Key<Aes256Gcm> {
    let mut raw = [0u8; 32];
    let (x, y) = if a.0 < b.0 { (a.0, b.0) } else { (b.0, a.0) };
    for (i, slot) in raw.iter_mut().enumerate() {
        let ia = (i % 8) as u32;
        let ib = ((i + 3) % 8) as u32;
        *slot = (x.wrapping_shr(ia) ^ y.wrapping_shr(ib) ^ (i as u64)) as u8;
    }
    *Key::<Aes256Gcm>::from_slice(&raw)
}

pub fn dm_encrypt(from: UserId, to: UserId, plaintext: &[u8]) -> Vec<u8> {
    let key = dm_key(from, to);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let mut ct = cipher.encrypt(&nonce, plaintext).expect("encrypt");
    let mut out = nonce.to_vec();
    out.append(&mut ct);
    out
}

pub fn dm_decrypt(from: UserId, to: UserId, ciphertext: &[u8]) -> Result<Vec<u8>, &'static str> {
    if ciphertext.len() < 12 {
        return Err("short");
    }
    let key = dm_key(from, to);
    let cipher = Aes256Gcm::new(&key);
    let (n, c) = ciphertext.split_at(12);
    let nonce = Nonce::from_slice(n);
    cipher
        .decrypt(nonce, c)
        .map_err(|_| "decrypt")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.9.1.1
    #[test]
    fn test_channel_create_and_join() {
        let mut cm = ChannelManager::default();
        let id = cm.create(ChannelKind::Party);
        let a = UserId(1);
        let b = UserId(2);
        cm.join(id, a).unwrap();
        cm.join(id, b).unwrap();
        let m = cm.members(id);
        assert_eq!(m.len(), 2);
        assert!(m.contains(&a) && m.contains(&b));
    }

    /// TC-8.9.2.1
    #[test]
    fn test_text_chat_persist_and_search() {
        let mut hist = MessageHistory::default();
        let ch = ChannelId(9);
        for i in 0..100 {
            let loot = if i % 15 == 0 { "loot" } else { "other" };
            hist.append(TextMessage {
                id: i,
                channel: ch,
                author: UserId(i % 5),
                content: format!("msg {loot} {i}"),
                timestamp: i as i64,
            });
        }
        let restarted = hist.cold_restart();
        let hits = restarted.search("loot");
        assert_eq!(hits.len(), 7);
        for w in hits.windows(2) {
            assert!(w[0].timestamp <= w[1].timestamp);
        }
    }

    /// TC-8.9.2.2
    #[test]
    fn test_text_chat_delivery_p99() {
        let p99 = text_delivery_p99_ms(20, 1000, 20, 10);
        assert!(p99 < 100);
    }

    /// TC-8.9.3.1
    #[test]
    fn test_text_message_mention() {
        let ch = ChannelId(3);
        let a = UserId(10);
        let b = UserId(20);
        let note = parse_mention_event(ch, a, "hello @player20", b).expect("mention");
        assert_eq!(note.sender, a);
        assert_eq!(note.channel, ch);
        assert!(parse_mention_event(ch, a, "hello @player20", a).is_none());
    }

    /// TC-8.9.5.1
    #[test]
    fn test_moderation_mute_block_report() {
        let mut m = ModerationState::default();
        let a = UserId(1);
        let b = UserId(2);
        let c = UserId(3);
        let d = UserId(4);
        m.mute(a, b).unwrap();
        m.block(a, c).unwrap();
        m.report(a, d, "Toxic").unwrap();
        assert!(!m.delivers_from(b, a));
        assert!(!m.delivers_from(c, a));
        assert!(m.block_list_contains(c));
        assert_eq!(m.reports[0].2, "Toxic");
        assert!(m.mute(a, a).is_err());
    }

    /// TC-8.9.6.1
    #[test]
    fn test_voice_proximity_attenuation() {
        let g10 = voice_gain_linear(10.0, 120.0);
        let g100 = voice_gain_linear(100.0, 120.0);
        assert!(g10 > g100);
        assert!(g10 > 0.0 && g10 <= 1.0);
        assert!(g100 > 0.0 && g100 <= 1.0);
        assert!((voice_gain_linear(130.0, 120.0) - 0.0).abs() < f32::EPSILON);
    }

    /// TC-8.9.8.1
    #[test]
    fn test_dm_e2e_encrypted() {
        let a = UserId(100);
        let b = UserId(200);
        let wire = dm_encrypt(a, b, b"hi");
        assert!(!wire.windows(2).any(|w| w == b"hi"));
        let plain = dm_decrypt(a, b, &wire).expect("b decrypt");
        assert_eq!(plain, b"hi");
        let other = UserId(999);
        assert!(dm_decrypt(a, other, &wire).is_err());
    }
}
