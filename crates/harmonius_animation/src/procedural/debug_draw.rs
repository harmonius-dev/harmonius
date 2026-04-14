//! Debug draw aggregation for locomotion / IK visualization.

use crate::math::Vec3;

/// One debug primitive.
#[derive(Clone, Debug, PartialEq)]
pub enum DebugPrimitive {
    /// Foot placement target marker.
    FootTarget(Vec3),
    /// IK joint axes (origin + tangent).
    IkJoint {
        /// Joint origin.
        origin: Vec3,
        /// Tangent direction.
        tangent: Vec3,
    },
}

/// Per-frame debug draw list.
#[derive(Clone, Debug, Default)]
pub struct DebugDraw {
    items: Vec<DebugPrimitive>,
}

impl DebugDraw {
    /// Clears all primitives.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Adds a primitive.
    pub fn push(&mut self, p: DebugPrimitive) {
        self.items.push(p);
    }

    /// Returns slice of primitives.
    pub fn items(&self) -> &[DebugPrimitive] {
        &self.items
    }

    /// Counts foot markers.
    pub fn foot_marker_count(&self) -> usize {
        self.items
            .iter()
            .filter(|p| matches!(p, DebugPrimitive::FootTarget(_)))
            .count()
    }
}

/// Per-entity overlay toggle.
#[derive(Clone, Debug, Default)]
pub struct EntityDebug {
    flags: Vec<bool>,
}

impl EntityDebug {
    /// Sets overlay for entity index.
    pub fn set(&mut self, entity: usize, on: bool) {
        if self.flags.len() <= entity {
            self.flags.resize(entity + 1, false);
        }
        self.flags[entity] = on;
    }

    /// Returns whether overlays are enabled.
    pub fn enabled(&self, entity: usize) -> bool {
        self.flags.get(entity).copied().unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_9_3_11_1_foot_targets() {
        let mut d = DebugDraw::default();
        for _ in 0..30 {
            d.clear();
            d.push(DebugPrimitive::FootTarget(Vec3::new(0.0, 0.0, 0.0)));
        }
        assert_eq!(d.foot_marker_count(), 1);
        d.clear();
        assert_eq!(d.foot_marker_count(), 0);
    }

    #[test]
    fn tc_9_3_11_2_ik_axes() {
        let mut d = DebugDraw::default();
        d.push(DebugPrimitive::IkJoint {
            origin: Vec3::ZERO,
            tangent: Vec3::new(1.0, 0.0, 0.0),
        });
        assert!(matches!(d.items()[0], DebugPrimitive::IkJoint { .. }));
    }

    #[test]
    fn tc_9_3_11_3_per_entity() {
        let mut e = EntityDebug::default();
        e.set(0, false);
        e.set(1, true);
        assert!(!e.enabled(0));
        assert!(e.enabled(1));
    }
}
