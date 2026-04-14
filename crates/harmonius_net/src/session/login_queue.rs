//! Login queue with VIP ordering (TC-8.5.7.*).

use super::types::{PlayerId, QueuePosition, QueuePriority};

#[derive(Clone, Debug)]
struct Entry {
    player: PlayerId,
}

#[derive(Clone, Debug)]
pub struct LoginQueue {
    pub capacity: u32,
    pub current: u32,
    entries: Vec<Entry>,
}

impl LoginQueue {
    pub fn new(capacity: u32, current: u32) -> Self {
        Self {
            capacity,
            current,
            entries: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, player: PlayerId, tier: QueuePriority) -> QueuePosition {
        let pos = if matches!(tier, QueuePriority::Admin | QueuePriority::Founder) {
            self.entries.insert(0, Entry { player });
            1
        } else {
            self.entries.push(Entry { player });
            self.entries.len() as u32
        };
        QueuePosition {
            player_id: player,
            position: pos,
            eta_secs: Some(pos.saturating_mul(5)),
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn position_of(&self, player: PlayerId) -> u32 {
        self.entries
            .iter()
            .position(|e| e.player == player)
            .map(|i| (i + 1) as u32)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.5.7.1
    #[test]
    fn test_login_queue_position() {
        let mut q = LoginQueue::new(100, 100);
        let p = PlayerId(99);
        let pos = q.enqueue(p, QueuePriority::Standard);
        assert_eq!(pos.position, 1);
        assert_eq!(q.len(), 1);
    }

    /// TC-8.5.7.2
    #[test]
    fn test_login_queue_vip_priority() {
        let mut q = LoginQueue::new(10, 10);
        q.enqueue(PlayerId(1), QueuePriority::Standard);
        q.enqueue(PlayerId(2), QueuePriority::Standard);
        q.enqueue(PlayerId(3), QueuePriority::Standard);
        let vip = PlayerId(4);
        q.enqueue(vip, QueuePriority::Founder);
        assert_eq!(q.position_of(vip), 1);
        assert_eq!(q.position_of(PlayerId(1)), 2);
    }
}
