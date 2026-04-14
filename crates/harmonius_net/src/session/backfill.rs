//! Mid-match backfill (TC-8.5.18.1).

use super::types::{BackfillRequest, Glicko2Rating, PlayerId};

#[derive(Clone, Debug)]
pub struct TeamMatch {
    pub team_a: Vec<(PlayerId, Glicko2Rating)>,
    pub team_b: Vec<(PlayerId, Glicko2Rating)>,
}

impl TeamMatch {
    pub fn team_avg_rating(team: &[(PlayerId, Glicko2Rating)]) -> f64 {
        let n = team.len().max(1) as f64;
        team.iter().map(|(_, g)| g.rating).sum::<f64>() / n
    }

    pub fn request_backfill(
        &self,
        leaver: PlayerId,
        queue: &[(PlayerId, Glicko2Rating)],
        window: f64,
        _wait_ticks: u32,
    ) -> Option<(BackfillRequest, PlayerId)> {
        let remaining: Vec<_> = self
            .team_a
            .iter()
            .chain(self.team_b.iter())
            .filter(|(p, _)| *p != leaver)
            .cloned()
            .collect();
        let avg = Self::team_avg_rating(&remaining);
        for (cand, r) in queue {
            if (r.rating - avg).abs() <= window {
                let req = BackfillRequest {
                    match_id: 1,
                    slot: 4,
                    rating_window: window,
                };
                return Some((req, *cand));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn g(r: f64) -> Glicko2Rating {
        Glicko2Rating {
            rating: r,
            deviation: 50.0,
            volatility: 0.06,
        }
    }

    /// TC-8.5.18.1
    #[test]
    fn test_backfill_replacement() {
        let team_a: Vec<_> = (0..5).map(|i| (PlayerId(i), g(1500.0))).collect();
        let team_b: Vec<_> = (5..10).map(|i| (PlayerId(i), g(1520.0))).collect();
        let m = TeamMatch { team_a, team_b };
        let leaver = PlayerId(0);
        let queue = vec![(PlayerId(100), g(1510.0))];
        let remaining: Vec<_> = m
            .team_a
            .iter()
            .chain(m.team_b.iter())
            .filter(|(p, _)| *p != leaver)
            .cloned()
            .collect();
        let avg = TeamMatch::team_avg_rating(&remaining);
        let (req, cand) = m
            .request_backfill(leaver, &queue, 50.0, 10)
            .expect("backfill");
        assert_eq!(cand.0, 100);
        assert!((queue[0].1.rating - avg).abs() <= req.rating_window);
    }
}
