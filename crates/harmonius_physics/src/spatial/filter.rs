//! Query filters for spatial tests.

use std::sync::Arc;

use crate::entity::EntityRef;

/// Bitmask of collision layers on entities and filters.
pub type CollisionLayerMask = u64;

/// Component tag bits used by `with_tags` / `without_tags` filters.
pub type ComponentTagMask = u64;

/// Optional predicate evaluated for each candidate.
pub type PredicateFn = Arc<dyn Fn(&EntityRef) -> bool + Send + Sync>;

/// Unified filter for all spatial queries (design-aligned subset for tests).
#[derive(Clone)]
pub struct QueryFilter {
    /// If non-zero, `(entity_layers & layers) != 0` is required.
    pub layers: CollisionLayerMask,
    pub max_distance: f32,
    pub predicate: Option<PredicateFn>,
    pub with_tags: ComponentTagMask,
    pub without_tags: ComponentTagMask,
}

impl Default for QueryFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl QueryFilter {
    pub fn new() -> Self {
        Self {
            layers: 0,
            max_distance: f32::MAX,
            predicate: None,
            with_tags: 0,
            without_tags: 0,
        }
    }

    pub fn with_layers(mut self, mask: CollisionLayerMask) -> Self {
        self.layers = mask;
        self
    }

    pub fn with_max_distance(mut self, dist: f32) -> Self {
        self.max_distance = dist;
        self
    }

    pub fn with_tags(mut self, mask: ComponentTagMask) -> Self {
        self.with_tags = mask;
        self
    }

    pub fn without_tags(mut self, mask: ComponentTagMask) -> Self {
        self.without_tags = mask;
        self
    }

    pub fn with_predicate(mut self, f: PredicateFn) -> Self {
        self.predicate = Some(f);
        self
    }

    pub fn matches(&self, er: &EntityRef) -> bool {
        if self.layers != 0 && (er.layers & self.layers) == 0 {
            return false;
        }
        if self.with_tags != 0 && (er.tags & self.with_tags) != self.with_tags {
            return false;
        }
        if self.without_tags != 0 && (er.tags & self.without_tags) != 0 {
            return false;
        }
        if let Some(pred) = &self.predicate {
            return pred(er);
        }
        true
    }
}
