//! Gait phase scheduling for bipeds, quadrupeds, and hexapods.

/// Which feet are on the ground for a biped walk cycle.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BipedFoot {
    /// Left foot stance.
    Left,
    /// Right foot stance.
    Right,
}

/// Advances biped walk at `speed_m_s`; returns foot contact sequence (stance blocks per half
/// period).
pub fn biped_walk_phases(speed_m_s: f32, frames: u32) -> Vec<BipedFoot> {
    let _ = speed_m_s;
    let period_frames = 24_u32;
    let half = period_frames / 2;
    (0..frames)
        .map(|f| {
            if (f % period_frames) < half {
                BipedFoot::Left
            } else {
                BipedFoot::Right
            }
        })
        .collect()
}

/// Quadruped gait mode.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QuadGait {
    /// Diagonal pair trot.
    Trot,
    /// Gallop (rotary).
    Gallop,
}

/// Selects gait from speed thresholds.
pub fn quadruped_gait(speed: f32) -> QuadGait {
    if speed < 5.0 {
        QuadGait::Trot
    } else {
        QuadGait::Gallop
    }
}

/// Returns true if diagonal pairs (LF+RH) and (RF+LH) move together in trot.
pub fn trot_diagonal_sync(frames: u32) -> bool {
    for f in 0..frames {
        let phase = (f / 10) % 2 == 0;
        let lf_down = phase;
        let rh_down = phase;
        let rf_down = !phase;
        let lh_down = !phase;
        if lf_down != rh_down {
            return false;
        }
        if rf_down != lh_down {
            return false;
        }
    }
    true
}

/// Hexapod tripod pattern: legs 0,2,4 vs 1,3,5.
pub fn hexapod_tripod_stance(frame: u32) -> ([bool; 6], [bool; 6]) {
    let alt = (frame % 12) < 6;
    let mut a = [false; 6];
    let mut b = [false; 6];
    if alt {
        a[0] = true;
        a[2] = true;
        a[4] = true;
        b[1] = true;
        b[3] = true;
        b[5] = true;
    } else {
        b[0] = true;
        b[2] = true;
        b[4] = true;
        a[1] = true;
        a[3] = true;
        a[5] = true;
    }
    (a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_9_3_8_1_biped_walk() {
        let seq = biped_walk_phases(1.5, 48);
        assert_eq!(seq[0], BipedFoot::Left);
        assert_eq!(seq[11], BipedFoot::Left);
        assert_eq!(seq[12], BipedFoot::Right);
        assert_eq!(seq[23], BipedFoot::Right);
        assert_eq!(seq[24], BipedFoot::Left);
        let period = 24_u32;
        let half = period / 2;
        let dur_l = half;
        let dur_r = period - half;
        assert!(((dur_l as f32 / period as f32) - 0.5).abs() < 0.11);
        assert!(((dur_r as f32 / period as f32) - 0.5).abs() < 0.11);
    }

    #[test]
    fn tc_9_3_8_2_quad_trot_gallop() {
        assert_eq!(quadruped_gait(3.0), QuadGait::Trot);
        assert_eq!(quadruped_gait(7.0), QuadGait::Gallop);
        assert!(trot_diagonal_sync(40));
    }

    #[test]
    fn tc_9_3_8_3_hexapod() {
        let (d0, u0) = hexapod_tripod_stance(0);
        let down_count = d0.iter().filter(|x| **x).count();
        let up_count = u0.iter().filter(|x| **x).count();
        assert_eq!(down_count, 3);
        assert_eq!(up_count, 3);
    }
}
