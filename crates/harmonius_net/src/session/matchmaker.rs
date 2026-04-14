//! Skill-based matchmaking window logic (TC-8.5.2.*).

use super::types::{Glicko2Rating, MatchOutcome, PlayerId};

#[derive(Clone, Debug, PartialEq)]
pub struct QueuedPlayer {
    pub id: PlayerId,
    pub rating: Glicko2Rating,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Match {
    pub players: [PlayerId; 2],
    pub avg_rating: f64,
}

#[derive(Clone, Debug)]
pub struct Matchmaker {
    queue: Vec<QueuedPlayer>,
    initial_window: f64,
    widen_interval_secs: u64,
    widen_step: f64,
}

impl Matchmaker {
    pub fn new(initial_window: f64, widen_interval_secs: u64, widen_step: f64) -> Self {
        Self {
            queue: Vec::new(),
            initial_window,
            widen_interval_secs,
            widen_step,
        }
    }

    pub fn enqueue(&mut self, p: QueuedPlayer) {
        self.queue.push(p);
    }

    pub fn tick_match(&mut self) -> Option<Match> {
        if self.queue.len() < 2 {
            return None;
        }
        let w = self.initial_window;
        let a = self.queue[0].clone();
        let b = self.queue[1].clone();
        if (a.rating.rating - b.rating.rating).abs() <= w {
            let ids = [a.id, b.id];
            self.queue.retain(|q| q.id != ids[0] && q.id != ids[1]);
            let avg = (a.rating.rating + b.rating.rating) / 2.0;
            return Some(Match {
                players: ids,
                avg_rating: avg,
            });
        }
        None
    }

    /// Effective rating window after `elapsed_secs` with periodic widening.
    pub fn effective_window(&self, elapsed_secs: u64) -> f64 {
        let steps = elapsed_secs / self.widen_interval_secs.max(1);
        self.initial_window + self.widen_step * steps as f64
    }

    pub fn tick_match_with_elapsed(&mut self, elapsed_secs: u64) -> Option<Match> {
        if self.queue.len() < 2 {
            return None;
        }
        let w = self.effective_window(elapsed_secs);
        let a = self.queue[0].clone();
        let b = self.queue[1].clone();
        if (a.rating.rating - b.rating.rating).abs() <= w {
            let ids = [a.id, b.id];
            self.queue.retain(|q| q.id != ids[0] && q.id != ids[1]);
            let avg = (a.rating.rating + b.rating.rating) / 2.0;
            return Some(Match {
                players: ids,
                avg_rating: avg,
            });
        }
        None
    }
}

/// Simplified post-match Glicko-2 style update (TC-8.5.9.1).
pub fn glicko2_update_pair(
    mut a: Glicko2Rating,
    mut b: Glicko2Rating,
    outcome: MatchOutcome,
) -> (Glicko2Rating, Glicko2Rating) {
    let (sa, sb) = match outcome {
        MatchOutcome::Win => (1.0_f64, 0.0_f64),
        MatchOutcome::Loss => (0.0_f64, 1.0_f64),
        MatchOutcome::Draw => (0.5_f64, 0.5_f64),
    };
    let k = 32.0_f64;
    let ea = 1.0 / (1.0 + 10_f64.powf((b.rating - a.rating) / 400.0));
    let eb = 1.0 - ea;
    a.rating += k * (sa - ea);
    b.rating += k * (sb - eb);
    a.deviation = (a.deviation * 0.92_f64).max(30.0);
    b.deviation = (b.deviation * 0.92_f64).max(30.0);
    a.volatility = (a.volatility * 0.99 + 0.0001).min(0.08);
    b.volatility = (b.volatility * 0.99 + 0.0001).min(0.08);
    (a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(id: u64, r: f64) -> QueuedPlayer {
        QueuedPlayer {
            id: PlayerId(id),
            rating: Glicko2Rating {
                rating: r,
                deviation: 200.0,
                volatility: 0.06,
            },
        }
    }

    /// TC-8.5.2.1
    #[test]
    fn test_matchmaker_skill_window() {
        let mut mm = Matchmaker::new(50.0, 10, 25.0);
        mm.enqueue(p(1, 1500.0));
        mm.enqueue(p(2, 1530.0));
        let m = mm.tick_match().expect("match");
        assert_eq!(m.players, [PlayerId(1), PlayerId(2)]);
        assert!((m.avg_rating - 1515.0).abs() < 0.01);
        assert!(mm.queue.is_empty());
    }

    /// TC-8.5.2.2
    #[test]
    fn test_matchmaker_widens_over_time() {
        let mm = Matchmaker::new(25.0, 10, 25.0);
        assert!((mm.effective_window(30) - 100.0).abs() < f64::EPSILON);
    }

    /// TC-8.5.9.1
    #[test]
    fn test_glicko2_recalc() {
        let a = Glicko2Rating {
            rating: 1500.0,
            deviation: 200.0,
            volatility: 0.06,
        };
        let b = Glicko2Rating {
            rating: 1500.0,
            deviation: 200.0,
            volatility: 0.06,
        };
        let (na, nb) = glicko2_update_pair(a, b, MatchOutcome::Win);
        assert!(na.rating > 1500.0);
        assert!(nb.rating < 1500.0);
        assert!(na.deviation < a.deviation);
        assert!(nb.deviation < b.deviation);
    }
}
