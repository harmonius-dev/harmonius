//! Inactivity deviation growth (TC-8.5.20.1).

use super::types::Glicko2Rating;

#[derive(Clone, Copy, Debug)]
pub struct PlayerRating {
    pub glicko: Glicko2Rating,
    pub last_active_secs: u64,
}

pub fn effective_rd(player: &PlayerRating, now_secs: u64, inactive_secs: u64) -> f64 {
    let idle = now_secs.saturating_sub(player.last_active_secs);
    if idle < inactive_secs {
        return player.glicko.deviation;
    }
    let steps = idle / inactive_secs;
    player.glicko.deviation + steps as f64 * 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.5.20.1
    #[test]
    fn test_rating_decay_inactivity() {
        let pr = PlayerRating {
            glicko: Glicko2Rating {
                rating: 1800.0,
                deviation: 60.0,
                volatility: 0.06,
            },
            last_active_secs: 0,
        };
        let now = 30 * 24 * 3600;
        let rd = effective_rd(&pr, now, 30 * 24 * 3600);
        assert!(rd > 60.0);
        assert!((pr.glicko.rating - 1800.0).abs() < f64::EPSILON);
    }
}
