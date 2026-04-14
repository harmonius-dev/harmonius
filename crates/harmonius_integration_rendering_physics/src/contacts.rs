//! Contact manifold debug lines (IR-3.4.2).

use crate::buffer::DebugDrawBuffer;
use crate::types::{DebugLine, LinearColor};
use glam::Vec3;

/// One contact point in a manifold.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContactPoint {
    pub position: Vec3,
}

/// Contact manifold between two bodies.
#[derive(Clone, Debug, PartialEq)]
pub struct ContactManifold {
    pub normal: Vec3,
    pub penetration: f32,
    pub points: Vec<ContactPoint>,
}

/// Appends normal arrows and contact dots (small cross segments) into `buf`.
pub fn contact_debug_lines(
    buf: &mut DebugDrawBuffer,
    max_lines: u32,
    manifolds: &[ContactManifold],
    normal_scale: f32,
    dot_radius: f32,
    arrow_color: LinearColor,
    dot_color: LinearColor,
) {
    for m in manifolds {
        let n = m.normal.normalize_or_zero();
        if n.length_squared() < 1e-12 {
            continue;
        }
        let arrow_len = m.penetration.max(1e-4) * normal_scale;
        for p in &m.points {
            let base = p.position;
            let tip = base + n * arrow_len;
            buf.push_line(
                max_lines,
                DebugLine {
                    start: base,
                    end: tip,
                    color: arrow_color,
                },
            );
            let r = dot_radius.max(1e-4);
            for off in [
                Vec3::new(r, 0.0, 0.0),
                Vec3::new(0.0, r, 0.0),
                Vec3::new(0.0, 0.0, r),
            ] {
                buf.push_line(
                    max_lines,
                    DebugLine {
                        start: base - off,
                        end: base + off,
                        color: dot_color,
                    },
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_3_4_2_1_contact_normal_arrow_up() {
        let mut buf = DebugDrawBuffer::new(64);
        let m = ContactManifold {
            normal: Vec3::Y,
            penetration: 0.2,
            points: vec![ContactPoint {
                position: Vec3::ZERO,
            }],
        };
        contact_debug_lines(
            &mut buf,
            1000,
            &[m],
            1.0,
            0.05,
            LinearColor::new(1.0, 0.0, 0.0, 1.0),
            LinearColor::new(0.0, 1.0, 0.0, 1.0),
        );
        let arrow = buf.lines.iter().find(|l| l.color.r > 0.9).expect("arrow");
        assert!(arrow.end.y > arrow.start.y);
    }

    #[test]
    fn tc_ir_3_4_2_2_penetration_scales_arrow() {
        let mut a = DebugDrawBuffer::new(64);
        let mut b = DebugDrawBuffer::new(64);
        let m_small = ContactManifold {
            normal: Vec3::Y,
            penetration: 0.1,
            points: vec![ContactPoint {
                position: Vec3::ZERO,
            }],
        };
        let m_big = ContactManifold {
            normal: Vec3::Y,
            penetration: 0.5,
            points: vec![ContactPoint {
                position: Vec3::ZERO,
            }],
        };
        contact_debug_lines(
            &mut a,
            1000,
            &[m_small],
            1.0,
            0.05,
            LinearColor::new(1.0, 1.0, 1.0, 1.0),
            LinearColor::new(0.0, 1.0, 0.0, 1.0),
        );
        contact_debug_lines(
            &mut b,
            1000,
            &[m_big],
            1.0,
            0.05,
            LinearColor::new(1.0, 1.0, 1.0, 1.0),
            LinearColor::new(0.0, 1.0, 0.0, 1.0),
        );
        let la = (a.lines[0].end - a.lines[0].start).length();
        let lb = (b.lines[0].end - b.lines[0].start).length();
        assert!(lb > la);
    }

    #[test]
    fn tc_ir_3_4_2_3_contact_point_dots_four_points() {
        let mut buf = DebugDrawBuffer::new(256);
        let pts: Vec<_> = (0..4)
            .map(|i| ContactPoint {
                position: Vec3::new(i as f32, 0.0, 0.0),
            })
            .collect();
        let m = ContactManifold {
            normal: Vec3::Y,
            penetration: 0.1,
            points: pts,
        };
        contact_debug_lines(
            &mut buf,
            1000,
            &[m],
            1.0,
            0.05,
            LinearColor::new(1.0, 0.0, 0.0, 1.0),
            LinearColor::new(0.0, 0.0, 1.0, 1.0),
        );
        let dots: Vec<_> = buf.lines.iter().filter(|l| l.color.b > 0.9).collect();
        assert_eq!(dots.len(), 4 * 3);
    }
}
