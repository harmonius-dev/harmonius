//! Motion matching pose search (CPU brute force for tests).

/// Squared weighted L2 distance between feature vectors.
pub fn feature_distance(a: &[f32], b: &[f32], w: &[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .zip(w.iter())
        .map(|((x, y), wv)| wv * (x - y) * (x - y))
        .sum()
}

/// Brute-force best pose index for trajectory + pose features concatenated.
pub fn best_pose_index(database: &[Vec<f32>], query: &[f32], weights: &[f32]) -> usize {
    let mut best = 0usize;
    let mut best_d = f32::MAX;
    for (i, row) in database.iter().enumerate() {
        let d = feature_distance(row, query, weights);
        if d < best_d {
            best_d = d;
            best = i;
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_9_3_6_1_motion_matching() {
        let mut db: Vec<Vec<f32>> = Vec::new();
        for i in 0..1000 {
            let mut v = vec![0.0_f32; 12];
            v[0] = i as f32 * 0.001;
            v[1] = (i as f32 * 0.003).sin();
            db.push(v);
        }
        let mut query = vec![0.0_f32; 12];
        query[0] = 0.37;
        query[1] = 0.11;
        let weights = vec![1.0_f32; 12];
        let idx = best_pose_index(&db, &query, &weights);
        let mut best = 0usize;
        let mut bd = f32::MAX;
        for (i, row) in db.iter().enumerate() {
            let d = feature_distance(row, &query, &weights);
            if d < bd {
                bd = d;
                best = i;
            }
        }
        assert_eq!(idx, best);

        let w2 = vec![1.0, 1.0, 1.0, 1.0, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5];
        let idx2 = best_pose_index(&db, &query, &w2);
        let mut best2 = 0usize;
        let mut bd2 = f32::MAX;
        for (i, row) in db.iter().enumerate() {
            let d = feature_distance(row, &query, &w2);
            if d < bd2 {
                bd2 = d;
                best2 = i;
            }
        }
        assert_eq!(idx2, best2);
        assert!((bd2 - feature_distance(&db[idx2], &query, &w2)).abs() < 1e-4);
    }
}
