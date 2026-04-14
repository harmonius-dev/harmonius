//! Competitive abandon escalation (TC-8.5.19.1).

use std::collections::VecDeque;
use std::time::Duration;

use super::types::{AbandonPenalty, PlayerId};

#[derive(Clone, Debug, Default)]
pub struct AbandonPolicy {
    window: Duration,
}

impl AbandonPolicy {
    pub fn new(window: Duration) -> Self {
        Self { window }
    }

    /// Records an abandon at `now_secs` and returns newly emitted penalties for this event.
    pub fn record_abandon(
        &self,
        history: &mut VecDeque<u64>,
        now_secs: u64,
        _player: PlayerId,
    ) -> Vec<AbandonPenalty> {
        while let Some(&t) = history.front() {
            if now_secs.saturating_sub(t) > self.window.as_secs() {
                history.pop_front();
            } else {
                break;
            }
        }
        history.push_back(now_secs);
        let n = history.len();
        match n {
            1 => vec![AbandonPenalty::Cooldown(Duration::from_secs(15 * 60))],
            2 => vec![AbandonPenalty::RatingPenalty(-50)],
            3 => vec![AbandonPenalty::TempBan(Duration::from_secs(24 * 3600))],
            _ => vec![AbandonPenalty::TempBan(Duration::from_secs(7 * 24 * 3600))],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.5.19.1
    #[test]
    fn test_abandon_escalation() {
        let pol = AbandonPolicy::new(Duration::from_secs(24 * 3600));
        let mut hist = VecDeque::new();
        let p = PlayerId(1);
        let mut penalties = Vec::new();
        penalties.extend(pol.record_abandon(&mut hist, 100, p));
        penalties.extend(pol.record_abandon(&mut hist, 200, p));
        penalties.extend(pol.record_abandon(&mut hist, 300, p));
        assert_eq!(
            penalties[0],
            AbandonPenalty::Cooldown(Duration::from_secs(15 * 60))
        );
        assert_eq!(penalties[1], AbandonPenalty::RatingPenalty(-50));
        assert_eq!(
            penalties[2],
            AbandonPenalty::TempBan(Duration::from_secs(24 * 3600))
        );
        let fourth = pol.record_abandon(&mut hist, 400, p);
        assert_eq!(
            fourth[0],
            AbandonPenalty::TempBan(Duration::from_secs(7 * 24 * 3600))
        );
    }
}
