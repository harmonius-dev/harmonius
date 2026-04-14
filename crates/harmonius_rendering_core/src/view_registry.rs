//! Registered render views with per-view projection matrices (R-2.10.3).

use glam::Mat4;

/// Stable numeric id for a render view (main, shadow, reflection, UI, …).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ViewId(pub u32);

/// One registered view and its clip projection.
#[derive(Clone, Debug, PartialEq)]
pub struct ViewRegistration {
    /// View identity chosen by the engine.
    pub id: ViewId,
    /// Column-major projection from view space to clip space.
    pub projection: Mat4,
}

/// Append-only registry used during extract / prepare.
#[derive(Clone, Debug, Default)]
pub struct ViewRegistry {
    views: Vec<ViewRegistration>,
}

impl ViewRegistry {
    /// Empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self { views: Vec::new() }
    }

    /// Registers `view` in registration order.
    pub fn register(&mut self, view: ViewRegistration) {
        self.views.push(view);
    }

    /// Registered views, oldest first.
    #[must_use]
    pub fn views(&self) -> &[ViewRegistration] {
        &self.views
    }

    /// Number of registered views.
    #[must_use]
    pub fn len(&self) -> usize {
        self.views.len()
    }

    /// `true` when no views are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.views.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::{ViewId, ViewRegistration, ViewRegistry};
    use glam::Mat4;

    /// TC-2.10.3.1 — four views (main, shadow, reflection, minimap) retain distinct projections.
    #[test]
    fn test_view_register_4_views() {
        let mut reg = ViewRegistry::new();
        let p_main = Mat4::IDENTITY;
        let p_shadow = Mat4::from_cols_array(&[1.0; 16]);
        let p_refl = Mat4::from_cols_array(&[2.0; 16]);
        let p_mini = Mat4::from_cols_array(&[3.0; 16]);
        reg.register(ViewRegistration {
            id: ViewId(0),
            projection: p_main,
        });
        reg.register(ViewRegistration {
            id: ViewId(1),
            projection: p_shadow,
        });
        reg.register(ViewRegistration {
            id: ViewId(2),
            projection: p_refl,
        });
        reg.register(ViewRegistration {
            id: ViewId(3),
            projection: p_mini,
        });
        assert_eq!(reg.len(), 4);
        let v = reg.views();
        assert_eq!(v[0].projection, p_main);
        assert_eq!(v[1].projection, p_shadow);
        assert_eq!(v[2].projection, p_refl);
        assert_eq!(v[3].projection, p_mini);
    }
}
