//! Snapshot interpolation helpers.

/// Linear interpolation between snapshots `(t_ms, value)`.
pub fn interpolate_snapshots(times_ms: &[f32], values: &[f32], t_ms: f32) -> f32 {
    if times_ms.is_empty() {
        return 0.0;
    }
    if t_ms <= times_ms[0] {
        return values[0];
    }
    if t_ms >= times_ms[times_ms.len() - 1] {
        return values[values.len() - 1];
    }
    for i in 0..times_ms.len() - 1 {
        if t_ms >= times_ms[i] && t_ms <= times_ms[i + 1] {
            let u = (t_ms - times_ms[i]) / (times_ms[i + 1] - times_ms[i]).max(1e-6);
            return values[i] + u * (values[i + 1] - values[i]);
        }
    }
    values[values.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.4.3.1 — interpolated samples stay monotonic between snapshots.
    #[test]
    fn test_interpolation_smooth_motion() {
        let t = [0f32, 50.0, 100.0];
        let p = [0.0, 5.0, 10.0];
        let mut last = 0.0f32;
        for i in 0..144 {
            let tt = (i as f32 / 143.0) * 100.0;
            let v = interpolate_snapshots(&t, &p, tt);
            assert!(v + 1e-3 >= last, "non-monotonic at i={i} v={v} last={last}");
            last = v;
        }
    }
}
