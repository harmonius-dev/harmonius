//! Screen-space marquee and lasso queries.

use glam::IVec2;
use smallvec::SmallVec;

use crate::types::EntityRef;

/// Rectangle intersection semantics for marquee selection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntersectMode {
    /// Select when bounds are fully enclosed by the marquee.
    Enclose,
    /// Select when bounds intersect the marquee.
    Intersect,
}

/// Axis-aligned integer rectangle in screen space.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScreenRect {
    /// Maximum inclusive corner.
    pub max: IVec2,
    /// Minimum inclusive corner.
    pub min: IVec2,
}

impl ScreenRect {
    /// Returns `true` when `inner` lies fully inside this rectangle.
    pub fn encloses(&self, inner: ScreenRect) -> bool {
        inner.min.x >= self.min.x
            && inner.min.y >= self.min.y
            && inner.max.x <= self.max.x
            && inner.max.y <= self.max.y
    }

    /// Returns `true` when the two rectangles overlap with positive area.
    pub fn intersects(&self, other: ScreenRect) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }
}

/// Performs marquee selection over screen-space bounds.
pub fn marquee_select(
    marquee: ScreenRect,
    mode: IntersectMode,
    items: impl Iterator<Item = (EntityRef, ScreenRect)>,
) -> SmallVec<[EntityRef; 16]> {
    let mut out = SmallVec::new();
    for (entity, bounds) in items {
        let hit = match mode {
            IntersectMode::Intersect => marquee.intersects(bounds),
            IntersectMode::Enclose => marquee.encloses(bounds),
        };
        if hit {
            if let Err(idx) = out.binary_search(&entity) {
                out.insert(idx, entity);
            }
        }
    }
    out
}

/// Point-in-polygon test using winding number (integer coordinates).
pub fn point_in_polygon(point: IVec2, polygon: &[IVec2]) -> bool {
    if polygon.len() < 3 {
        return false;
    }
    let mut winding = 0i32;
    for i in 0..polygon.len() {
        let a = polygon[i];
        let b = polygon[(i + 1) % polygon.len()];
        if a.y <= point.y {
            if b.y > point.y && is_left(a, b, point) > 0 {
                winding += 1;
            }
        } else if b.y <= point.y && is_left(a, b, point) < 0 {
            winding -= 1;
        }
    }
    winding != 0
}

fn is_left(a: IVec2, b: IVec2, p: IVec2) -> i64 {
    (i64::from(b.x) - i64::from(a.x)) * (i64::from(p.y) - i64::from(a.y))
        - (i64::from(p.x) - i64::from(a.x)) * (i64::from(b.y) - i64::from(a.y))
}

/// Selects entities whose integer centroid lies inside `polygon`.
pub fn lasso_select_by_centroid(
    polygon: &[IVec2],
    items: impl Iterator<Item = (EntityRef, IVec2)>,
) -> SmallVec<[EntityRef; 16]> {
    let mut out = SmallVec::new();
    for (entity, centroid) in items {
        if point_in_polygon(centroid, polygon) {
            if let Err(idx) = out.binary_search(&entity) {
                out.insert(idx, entity);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use glam::IVec2;

    use super::*;
    use crate::types::EntityRef;

    fn e(id: u32) -> EntityRef {
        EntityRef(id)
    }

    #[test]
    fn test_marquee_intersect_mode() {
        let marquee = ScreenRect {
            min: IVec2::new(0, 0),
            max: IVec2::new(10, 10),
        };
        let partial = ScreenRect {
            min: IVec2::new(8, 8),
            max: IVec2::new(12, 12),
        };
        let selected = marquee_select(
            marquee,
            IntersectMode::Intersect,
            [(e(1), partial)].into_iter(),
        );
        assert_eq!(selected.as_slice(), &[e(1)]);
    }

    #[test]
    fn test_marquee_enclose_mode() {
        let marquee = ScreenRect {
            min: IVec2::new(0, 0),
            max: IVec2::new(10, 10),
        };
        let partial = ScreenRect {
            min: IVec2::new(8, 8),
            max: IVec2::new(12, 12),
        };
        let inner = ScreenRect {
            min: IVec2::new(2, 2),
            max: IVec2::new(8, 8),
        };
        let selected = marquee_select(
            marquee,
            IntersectMode::Enclose,
            [(e(1), partial), (e(2), inner)].into_iter(),
        );
        assert_eq!(selected.as_slice(), &[e(2)]);
    }

    #[test]
    fn test_lasso_inside_selected() {
        let polygon = [
            IVec2::new(0, 0),
            IVec2::new(10, 0),
            IVec2::new(10, 10),
            IVec2::new(0, 10),
        ];
        let selected = lasso_select_by_centroid(
            &polygon,
            [(e(1), IVec2::new(5, 5)), (e(2), IVec2::new(20, 20))].into_iter(),
        );
        assert_eq!(selected.as_slice(), &[e(1)]);
    }

    #[test]
    fn test_lasso_outside_rejected() {
        let polygon = [
            IVec2::new(0, 0),
            IVec2::new(4, 0),
            IVec2::new(4, 4),
            IVec2::new(0, 4),
        ];
        let selected = lasso_select_by_centroid(&polygon, [(e(3), IVec2::new(20, 20))].into_iter());
        assert!(selected.is_empty());
    }

    #[test]
    fn test_marquee_against_1k_entities() {
        let marquee = ScreenRect {
            min: IVec2::new(0, 0),
            max: IVec2::new(1000, 1000),
        };
        let items = (0..1000).map(|i| {
            let x = i % 100;
            let y = i / 100;
            let origin = IVec2::new(x * 10, y * 10);
            (
                EntityRef(i as u32),
                ScreenRect {
                    min: origin,
                    max: origin + IVec2::new(5, 5),
                },
            )
        });
        let selected = marquee_select(marquee, IntersectMode::Intersect, items);
        assert_eq!(selected.len(), 1000);
    }
}
