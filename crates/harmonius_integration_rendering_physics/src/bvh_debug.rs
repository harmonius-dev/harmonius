//! BVH AABB debug lines (IR-3.4.3).

use crate::buffer::DebugDrawBuffer;
use crate::types::{DebugLine, LinearColor};
use glam::Vec3;

/// One BVH node AABB for visualization (physics-private tree mirrored here).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BvhNode {
    pub min: Vec3,
    pub max: Vec3,
    pub depth: u32,
    pub is_leaf: bool,
}

fn aabb_wireframe_lines(min: Vec3, max: Vec3, color: LinearColor) -> Vec<DebugLine> {
    let corners = [
        Vec3::new(min.x, min.y, min.z),
        Vec3::new(max.x, min.y, min.z),
        Vec3::new(max.x, max.y, min.z),
        Vec3::new(min.x, max.y, min.z),
        Vec3::new(min.x, min.y, max.z),
        Vec3::new(max.x, min.y, max.z),
        Vec3::new(max.x, max.y, max.z),
        Vec3::new(min.x, max.y, max.z),
    ];
    let edges = [
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0),
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4),
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ];
    edges
        .iter()
        .map(|&(a, b)| DebugLine {
            start: corners[a],
            end: corners[b],
            color,
        })
        .collect()
}

/// Emits wireframe boxes for each node respecting `bvh_max_depth` and colors.
pub fn bvh_debug_lines(
    buf: &mut DebugDrawBuffer,
    max_lines: u32,
    nodes: &[BvhNode],
    bvh_max_depth: u32,
    leaf_color: LinearColor,
    internal_color: LinearColor,
) {
    for node in nodes {
        if node.depth >= bvh_max_depth {
            continue;
        }
        let color = if node.is_leaf {
            leaf_color
        } else {
            internal_color
        };
        for ln in aabb_wireframe_lines(node.min, node.max, color) {
            buf.push_line(max_lines, ln);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn box_node(min: Vec3, max: Vec3, depth: u32, leaf: bool) -> BvhNode {
        BvhNode {
            min,
            max,
            depth,
            is_leaf: leaf,
        }
    }

    #[test]
    fn tc_ir_3_4_3_1_bvh_leaf_green() {
        let mut buf = DebugDrawBuffer::new(4096);
        let green = LinearColor::new(0.0, 1.0, 0.0, 1.0);
        let yellow = LinearColor::new(1.0, 1.0, 0.0, 1.0);
        let mut nodes = Vec::new();
        for i in 0..10 {
            let o = i as f32 * 2.0;
            nodes.push(box_node(
                Vec3::new(o, 0.0, 0.0),
                Vec3::new(o + 1.0, 1.0, 1.0),
                0,
                true,
            ));
        }
        bvh_debug_lines(&mut buf, 100_000, &nodes, 8, green, yellow);
        let green_lines = buf
            .lines
            .iter()
            .filter(|l| l.color.g > 0.9 && l.color.r < 0.5)
            .count();
        assert_eq!(green_lines, 10 * 12);
    }

    #[test]
    fn tc_ir_3_4_3_2_bvh_depth_filter() {
        let mut buf = DebugDrawBuffer::new(4096);
        let green = LinearColor::new(0.0, 1.0, 0.0, 1.0);
        let yellow = LinearColor::new(1.0, 1.0, 0.0, 1.0);
        let nodes = vec![
            box_node(Vec3::ZERO, Vec3::ONE * 4.0, 0, false),
            box_node(Vec3::ZERO, Vec3::ONE * 2.0, 1, false),
            box_node(Vec3::ZERO, Vec3::ONE, 2, true),
        ];
        bvh_debug_lines(&mut buf, 100_000, &nodes, 2, green, yellow);
        assert_eq!(buf.lines.len(), 24);
    }

    #[test]
    fn tc_ir_3_4_3_3_internal_yellow() {
        let mut buf = DebugDrawBuffer::new(4096);
        let green = LinearColor::new(0.0, 1.0, 0.0, 1.0);
        let yellow = LinearColor::new(1.0, 1.0, 0.0, 1.0);
        let nodes = vec![
            box_node(Vec3::ZERO, Vec3::ONE * 4.0, 0, false),
            box_node(Vec3::ZERO, Vec3::ONE * 2.0, 1, false),
            box_node(Vec3::ZERO, Vec3::ONE, 2, true),
        ];
        bvh_debug_lines(&mut buf, 100_000, &nodes, 8, green, yellow);
        let yellow_lines = buf
            .lines
            .iter()
            .filter(|l| l.color.r > 0.9 && l.color.g > 0.9)
            .count();
        assert_eq!(yellow_lines, 2 * 12);
    }

    #[test]
    fn tc_ir_3_4_n6_empty_bvh() {
        let mut buf = DebugDrawBuffer::new(64);
        bvh_debug_lines(
            &mut buf,
            1000,
            &[],
            8,
            LinearColor::new(0.0, 1.0, 0.0, 1.0),
            LinearColor::new(1.0, 1.0, 0.0, 1.0),
        );
        assert!(buf.lines.is_empty());
    }
}
