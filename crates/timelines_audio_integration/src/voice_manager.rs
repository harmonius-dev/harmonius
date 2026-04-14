//! Fixed-size voice allocator with deterministic stealing.

use crate::types::{VoiceId, VoicePriority};

/// Simple fixed-pool voice allocator used on the game thread.
#[derive(Debug)]
pub struct VoiceManager {
    slots: Vec<Option<(VoiceId, VoicePriority)>>,
    next_id: u32,
}

impl VoiceManager {
    /// Builds a manager with `max_voices` real slots.
    pub fn new(max_voices: usize) -> Self {
        Self {
            slots: vec![None; max_voices],
            next_id: 1,
        }
    }

    /// Allocates a voice, optionally stealing the lowest-priority active voice.
    pub fn allocate(&mut self, priority: VoicePriority) -> Option<VoiceId> {
        if let Some(index) = self.slots.iter().position(|s| s.is_none()) {
            let id = VoiceId(self.next_id);
            self.next_id = self.next_id.saturating_add(1).max(1);
            self.slots[index] = Some((id, priority));
            return Some(id);
        }
        let steal_index = self
            .slots
            .iter()
            .enumerate()
            .filter_map(|(i, s)| s.as_ref().map(|(_, p)| (i, *p)))
            .min_by_key(|(_, p)| *p)
            .map(|(i, _)| i)?;
        let (_, old_pri) = self.slots[steal_index].expect("slot");
        if priority >= old_pri {
            let id = VoiceId(self.next_id);
            self.next_id = self.next_id.saturating_add(1).max(1);
            self.slots[steal_index] = Some((id, priority));
            Some(id)
        } else {
            None
        }
    }

    /// Releases a previously allocated voice id.
    pub fn release(&mut self, voice: VoiceId) {
        for slot in &mut self.slots {
            if let Some((id, _)) = slot {
                if *id == voice {
                    *slot = None;
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-IR-4.7.2.3 — allocation failure when no steal is possible.
    #[test]
    fn tc_ir_4_7_2_3_voice_allocation_failure() {
        let mut vm = VoiceManager::new(1);
        assert!(vm.allocate(VoicePriority::Critical).is_some());
        assert!(vm.allocate(VoicePriority::Normal).is_none());
    }

    /// TC-IR-4.7.N.6 — critical request steals the lowest-priority busy voice.
    #[test]
    fn tc_ir_4_7_n_6_voice_steal_fallback() {
        let mut vm = VoiceManager::new(2);
        let a = vm.allocate(VoicePriority::Ambient).expect("a");
        let b = vm.allocate(VoicePriority::Normal).expect("b");
        assert_ne!(a, b);
        let c = vm
            .allocate(VoicePriority::Critical)
            .expect("steals Ambient");
        assert_ne!(c, a);
        assert_ne!(c, b);
    }
}
