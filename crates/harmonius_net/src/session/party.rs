//! Party roles and ready checks (TC-8.5.3.*).

use std::collections::HashMap;

use super::types::{PartyRole, PlayerId, ReadyCheckResult};

#[derive(Clone, Debug)]
pub struct PartyComponent {
    leader: PlayerId,
    roles: HashMap<PlayerId, PartyRole>,
    ready: HashMap<PlayerId, bool>,
}

impl PartyComponent {
    pub fn new(leader: PlayerId) -> Self {
        let mut roles = HashMap::new();
        roles.insert(leader, PartyRole::Unassigned);
        Self {
            leader,
            roles,
            ready: HashMap::new(),
        }
    }

    pub fn add_member(&mut self, p: PlayerId) {
        self.roles.entry(p).or_insert(PartyRole::Unassigned);
    }

    pub fn assign_role(&mut self, actor: PlayerId, member: PlayerId, role: PartyRole) -> bool {
        if actor != self.leader {
            return false;
        }
        if matches!(role, PartyRole::Unassigned) {
            self.roles.insert(member, role);
            return true;
        }
        for (pid, r) in &self.roles {
            if *pid != member && *r == role {
                return false;
            }
        }
        self.roles.insert(member, role);
        true
    }

    pub fn role_of(&self, member: PlayerId) -> Option<PartyRole> {
        self.roles.get(&member).copied()
    }

    pub fn start_ready_check(&mut self, members: &[PlayerId]) {
        self.ready.clear();
        for m in members {
            self.ready.insert(*m, false);
        }
    }

    pub fn set_ready(&mut self, member: PlayerId, ready: bool) {
        if let Some(v) = self.ready.get_mut(&member) {
            *v = ready;
        }
    }

    /// When all tracked members are ready, returns [`ReadyCheckResult::AllReady`].
    pub fn finish_ready_if_all(&self) -> ReadyCheckResult {
        if self.ready.is_empty() {
            return ReadyCheckResult::Cancelled;
        }
        if self.ready.values().all(|&v| v) {
            ReadyCheckResult::AllReady
        } else {
            ReadyCheckResult::Timeout {
                not_ready: self.ready.values().filter(|&&v| !v).count() as u32,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.5.3.1
    #[test]
    fn test_party_role_assignment() {
        let leader = PlayerId(1);
        let a = PlayerId(2);
        let b = PlayerId(3);
        let mut party = PartyComponent::new(leader);
        party.add_member(a);
        party.add_member(b);
        assert!(party.assign_role(leader, a, PartyRole::Tank));
        assert!(party.assign_role(leader, b, PartyRole::Healer));
        assert_eq!(party.role_of(a), Some(PartyRole::Tank));
        assert_eq!(party.role_of(b), Some(PartyRole::Healer));
        assert!(!party.assign_role(leader, b, PartyRole::Tank));
    }

    /// TC-8.5.3.2
    #[test]
    fn test_party_ready_check() {
        let leader = PlayerId(1);
        let mut party = PartyComponent::new(leader);
        let members: Vec<PlayerId> = (1..=5).map(PlayerId).collect();
        for m in &members {
            party.add_member(*m);
        }
        party.start_ready_check(&members);
        for m in &members {
            party.set_ready(*m, true);
        }
        assert_eq!(party.finish_ready_if_all(), ReadyCheckResult::AllReady);
    }
}
