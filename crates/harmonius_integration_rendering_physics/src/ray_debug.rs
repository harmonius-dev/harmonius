//! Ray and shape cast debug lines (IR-3.4.4).

use crate::buffer::DebugDrawBuffer;
use crate::types::{DebugLine, LinearColor};
use glam::Vec3;

/// Ray cast result for visualization.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RayCastResult {
    pub origin: Vec3,
    pub direction: Vec3,
    pub max_distance: f32,
    pub hit: bool,
    pub hit_distance: f32,
}

/// Shape cast result (capsule sweep) for visualization.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShapeCastResult {
    pub start_center: Vec3,
    pub end_center: Vec3,
    pub radius: f32,
    pub hit: bool,
    pub miss_color: LinearColor,
    pub hit_color: LinearColor,
}

/// Ray line: green to hit, red to max distance on miss.
pub fn ray_debug_lines(
    buf: &mut DebugDrawBuffer,
    max_lines: u32,
    rays: &[RayCastResult],
    hit_color: LinearColor,
    miss_color: LinearColor,
) {
    for r in rays {
        let dir_len = r.direction.length();
        if dir_len < 1e-12 || !r.max_distance.is_finite() || r.max_distance <= 0.0 {
            continue;
        }
        let d = r.direction / dir_len;
        let end = if r.hit {
            r.origin + d * r.hit_distance
        } else {
            r.origin + d * r.max_distance
        };
        let color = if r.hit { hit_color } else { miss_color };
        buf.push_line(
            max_lines,
            DebugLine {
                start: r.origin,
                end,
                color,
            },
        );
    }
}

/// Capsule sweep: start circle, end circle, two tangent connectors.
pub fn shape_cast_debug_lines(
    buf: &mut DebugDrawBuffer,
    max_lines: u32,
    casts: &[ShapeCastResult],
    ring_segments: usize,
) {
    let seg = ring_segments.max(8);
    for c in casts {
        let color = if c.hit { c.hit_color } else { c.miss_color };
        for center in [c.start_center, c.end_center] {
            for i in 0..seg {
                let t0 = (i as f32 / seg as f32) * std::f32::consts::TAU;
                let t1 = ((i + 1) as f32 / seg as f32) * std::f32::consts::TAU;
                let s = center + Vec3::new(c.radius * t0.cos(), 0.0, c.radius * t0.sin());
                let e = center + Vec3::new(c.radius * t1.cos(), 0.0, c.radius * t1.sin());
                buf.push_line(
                    max_lines,
                    DebugLine {
                        start: s,
                        end: e,
                        color,
                    },
                );
            }
        }
        let delta = c.end_center - c.start_center;
        if delta.length_squared() < 1e-12 {
            continue;
        }
        let side = delta.normalize().cross(Vec3::Y);
        let orth = if side.length_squared() < 1e-8 {
            Vec3::X
        } else {
            side.normalize()
        };
        let off = orth * c.radius;
        buf.push_line(
            max_lines,
            DebugLine {
                start: c.start_center + off,
                end: c.end_center + off,
                color,
            },
        );
        buf.push_line(
            max_lines,
            DebugLine {
                start: c.start_center - off,
                end: c.end_center - off,
                color,
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_3_4_4_1_raycast_hit_green() {
        let mut buf = DebugDrawBuffer::new(32);
        let green = LinearColor::new(0.0, 1.0, 0.0, 1.0);
        let red = LinearColor::new(1.0, 0.0, 0.0, 1.0);
        let r = RayCastResult {
            origin: Vec3::ZERO,
            direction: Vec3::X,
            max_distance: 10.0,
            hit: true,
            hit_distance: 5.0,
        };
        ray_debug_lines(&mut buf, 1000, &[r], green, red);
        assert_eq!(buf.lines.len(), 1);
        assert!((buf.lines[0].end - Vec3::new(5.0, 0.0, 0.0)).length() < 1e-3);
        assert!(buf.lines[0].color.g > 0.5);
    }

    #[test]
    fn tc_ir_3_4_4_2_raycast_miss_red() {
        let mut buf = DebugDrawBuffer::new(32);
        let green = LinearColor::new(0.0, 1.0, 0.0, 1.0);
        let red = LinearColor::new(1.0, 0.0, 0.0, 1.0);
        let r = RayCastResult {
            origin: Vec3::ZERO,
            direction: Vec3::X,
            max_distance: 3.0,
            hit: false,
            hit_distance: 0.0,
        };
        ray_debug_lines(&mut buf, 1000, &[r], green, red);
        assert!((buf.lines[0].end - Vec3::new(3.0, 0.0, 0.0)).length() < 1e-3);
        assert!(buf.lines[0].color.r > 0.5);
    }

    #[test]
    fn tc_ir_3_4_4_3_shapecast_swept_volume() {
        let mut buf = DebugDrawBuffer::new(512);
        let c = ShapeCastResult {
            start_center: Vec3::ZERO,
            end_center: Vec3::new(5.0, 0.0, 0.0),
            radius: 0.5,
            hit: false,
            miss_color: LinearColor::new(1.0, 0.0, 0.0, 1.0),
            hit_color: LinearColor::new(0.0, 1.0, 0.0, 1.0),
        };
        shape_cast_debug_lines(&mut buf, 1000, &[c], 16);
        assert!(buf.lines.len() >= 16 * 2 + 2);
    }

    #[test]
    fn tc_ir_3_4_n7_raycast_zero_length_skipped() {
        let mut buf = DebugDrawBuffer::new(32);
        let r = RayCastResult {
            origin: Vec3::ZERO,
            direction: Vec3::ZERO,
            max_distance: 5.0,
            hit: false,
            hit_distance: 0.0,
        };
        ray_debug_lines(
            &mut buf,
            1000,
            &[r],
            LinearColor::new(0.0, 1.0, 0.0, 1.0),
            LinearColor::new(1.0, 0.0, 0.0, 1.0),
        );
        assert!(buf.lines.is_empty());
    }

    #[test]
    fn tc_ir_3_4_n8_shapecast_no_hit_red() {
        let mut buf = DebugDrawBuffer::new(512);
        let c = ShapeCastResult {
            start_center: Vec3::ZERO,
            end_center: Vec3::new(5.0, 0.0, 0.0),
            radius: 0.5,
            hit: false,
            miss_color: LinearColor::new(1.0, 0.0, 0.0, 1.0),
            hit_color: LinearColor::new(0.0, 1.0, 0.0, 1.0),
        };
        shape_cast_debug_lines(&mut buf, 1000, &[c], 8);
        assert!(buf.lines.iter().all(|l| l.color.r > 0.5));
    }
}
