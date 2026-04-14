//! Boolean condition expressions with registry-backed leaves.
use crate::shared::{Entity, World};

/// Opaque identifier for a codegen'd condition leaf.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ConditionId(pub u32);

/// Read-only evaluation context passed to condition functions.
#[derive(Clone, Copy, Debug)]
pub struct ConditionContext<'w> {
    /// ECS world reference (stub in this crate).
    pub world: &'w World,
    /// Entity being evaluated.
    pub entity: Entity,
}

/// Function pointer type for condition evaluation.
pub type ConditionCheckFn = fn(&ConditionContext<'_>) -> bool;

/// Boolean expression tree evaluated against a [`ConditionRegistry`].
#[derive(Clone, Debug, PartialEq)]
pub enum ConditionExpr {
    /// Logical conjunction.
    And(Vec<ConditionExpr>),
    /// Logical disjunction.
    Or(Vec<ConditionExpr>),
    /// Logical negation.
    Not(Box<ConditionExpr>),
    /// Leaf referencing a registered function.
    Leaf(ConditionId),
}

impl ConditionExpr {
    /// Evaluates the tree using registry-backed leaves.
    pub fn evaluate(&self, ctx: &ConditionContext<'_>, registry: &ConditionRegistry) -> bool {
        match self {
            ConditionExpr::And(children) => {
                children.iter().all(|child| child.evaluate(ctx, registry))
            }
            ConditionExpr::Or(children) => {
                children.iter().any(|child| child.evaluate(ctx, registry))
            }
            ConditionExpr::Not(child) => !child.evaluate(ctx, registry),
            ConditionExpr::Leaf(id) => registry
                .get(*id)
                .map(|function| function(ctx))
                .unwrap_or(false),
        }
    }
}

/// Registry mapping [`ConditionId`] values to codegen'd checks.
#[derive(Clone, Debug, Default)]
pub struct ConditionRegistry {
    checks: Vec<Option<ConditionCheckFn>>,
}

impl ConditionRegistry {
    /// Registers a function for `id`, growing the backing store as needed.
    pub fn register(&mut self, id: ConditionId, check: ConditionCheckFn) {
        let index = id.0 as usize;
        if self.checks.len() <= index {
            self.checks.resize(index + 1, None);
        }
        self.checks[index] = Some(check);
    }

    /// Looks up a registered function.
    #[must_use]
    pub fn get(&self, id: ConditionId) -> Option<ConditionCheckFn> {
        self.checks.get(id.0 as usize).copied().flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condition_and_or_not() {
        let mut registry = ConditionRegistry::default();
        registry.register(ConditionId(1), |_ctx| true);
        registry.register(ConditionId(2), |_ctx| false);
        registry.register(ConditionId(3), |_ctx| false);

        let expr = ConditionExpr::And(vec![
            ConditionExpr::Leaf(ConditionId(1)),
            ConditionExpr::Or(vec![
                ConditionExpr::Leaf(ConditionId(2)),
                ConditionExpr::Not(Box::new(ConditionExpr::Leaf(ConditionId(3)))),
            ]),
        ]);

        let ctx = ConditionContext {
            world: &World,
            entity: Entity(1),
        };

        assert!(expr.evaluate(&ctx, &registry));
    }

    #[test]
    fn test_condition_leaf_dispatch() {
        use std::sync::atomic::{AtomicU32, Ordering};

        static CALLS: AtomicU32 = AtomicU32::new(0);

        fn counted(_ctx: &ConditionContext<'_>) -> bool {
            CALLS.fetch_add(1, Ordering::Relaxed);
            true
        }

        CALLS.store(0, Ordering::Relaxed);
        let mut registry = ConditionRegistry::default();
        registry.register(ConditionId(7), counted);

        let expr = ConditionExpr::Leaf(ConditionId(7));
        let ctx = ConditionContext {
            world: &World,
            entity: Entity(2),
        };

        assert!(expr.evaluate(&ctx, &registry));
        assert_eq!(CALLS.load(Ordering::Relaxed), 1);
    }
}
