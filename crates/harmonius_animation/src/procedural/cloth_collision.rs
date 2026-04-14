//! Cloth vs analytic collision proxies.

use crate::math::Vec3;

/// Infinite capsule (Y axis) with radius `r`, centerline x=z=0.
pub fn push_out_of_capsule(p: Vec3, r: f32) -> Vec3 {
    let d = (p.x * p.x + p.z * p.z).sqrt();
    if d >= r {
        return p;
    }
    if d < 1e-8 {
        return Vec3::new(r, p.y, 0.0);
    }
    let s = r / d;
    Vec3::new(p.x * s, p.y, p.z * s)
}

/// Triangle hull: pushes `p` to the positive side of the triangle plane if it lies "below".
pub fn push_out_of_triangle(p: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
    let ab = b - a;
    let ac = c - a;
    let mut n = ab.cross(ac);
    let nl = n.length();
    if nl < 1e-8 {
        return p;
    }
    n = n * (1.0 / nl);
    let ap = p - a;
    let dist = ap.dot(n);
    if dist < 0.0 {
        p - n * (dist - 1e-4)
    } else {
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_9_5_5_1_cloth_capsule() {
        let r = 0.2;
        let p = Vec3::new(0.05, -0.1, 0.05);
        let q = push_out_of_capsule(p, r);
        let d = (q.x * q.x + q.z * q.z).sqrt();
        assert!(d + 1e-4 >= r * 0.99);

        let a = Vec3::new(0.0, 0.0, 0.0);
        let b = Vec3::new(1.0, 0.0, 0.0);
        let c = Vec3::new(0.0, 0.0, 1.0);
        let p2 = Vec3::new(0.2, -0.05, 0.2);
        let q2 = push_out_of_triangle(p2, a, b, c);
        let ab = b - a;
        let ac = c - a;
        let n = ab.cross(ac).normalized();
        let dist = (q2 - a).dot(n);
        assert!(dist >= -1e-3);
    }
}
