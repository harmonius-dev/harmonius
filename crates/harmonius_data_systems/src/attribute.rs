//! Schema-driven attribute sets with memoized evaluation.
use crate::shared::StatAggregator;
use smallvec::SmallVec;
use smol_str::SmolStr;

/// Identifier for an attribute schema.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AttributeSchemaId(pub u32);

/// Identifier for a single attribute field.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AttributeDefId(pub u32);

/// Storage kind for an attribute field.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AttributeValueType {
    /// 32-bit floating point scalar.
    F32,
    /// 32-bit signed integer scalar (stored as f32 for the hot path).
    I32,
    /// Boolean flag stored as 0/1 in the scalar lane.
    Bool,
    /// Enumerated value referenced by `type_id`.
    Enum {
        /// Editor-defined discriminant table id.
        type_id: u32,
    },
}

/// Immutable attribute field definition.
#[derive(Clone, Debug, PartialEq)]
pub struct AttributeDefinition {
    /// Field identifier.
    pub id: AttributeDefId,
    /// Human readable name used by tooling and saves.
    pub name: SmolStr,
    /// Logical storage type.
    pub value_type: AttributeValueType,
    /// Minimum allowed scalar (for `F32` lanes).
    pub min_value: f32,
    /// Maximum allowed scalar (for `F32` lanes).
    pub max_value: f32,
    /// Default scalar used when constructing a set.
    pub default_value: f32,
    /// Whether the field participates in replication.
    pub replicate: bool,
}

/// Schema describing an ordered attribute collection.
#[derive(Clone, Debug, PartialEq)]
pub struct AttributeSchema {
    /// Schema identifier.
    pub id: AttributeSchemaId,
    /// Display name for editors.
    pub name: SmolStr,
    /// Ordered field definitions.
    pub attributes: Vec<AttributeDefinition>,
}

impl AttributeSchema {
    /// Looks up a definition by id.
    #[must_use]
    pub fn get(&self, id: AttributeDefId) -> Option<&AttributeDefinition> {
        self.attributes.iter().find(|field| field.id == id)
    }

    /// Resolves the positional index for a definition id.
    #[must_use]
    pub fn index_of(&self, id: AttributeDefId) -> Option<usize> {
        self.attributes.iter().position(|field| field.id == id)
    }

    /// Finds a field by its stable name.
    #[must_use]
    pub fn find_by_name(&self, name: &str) -> Option<&AttributeDefinition> {
        self.attributes.iter().find(|field| field.name == name)
    }
}

/// Runtime attribute scalar with layered modifiers.
#[derive(Clone, Debug, PartialEq)]
pub struct AttributeValue {
    /// Authoritative base scalar before modifiers.
    pub base: f32,
    /// Cached evaluated scalar for memoized reads.
    current_cached: f32,
    /// Modifier stack layered on the scalar lane.
    pub modifier_stack: StatAggregator,
    /// Whether `current_cached` matches `modifier_stack` + `base`.
    cache_valid: bool,
}

impl AttributeValue {
    /// Constructs a value with an empty modifier stack.
    pub fn new(base: f32) -> Self {
        Self {
            base,
            current_cached: base,
            modifier_stack: StatAggregator::new(),
            cache_valid: false,
        }
    }

    /// Updates the base scalar and invalidates memoized reads.
    pub fn set_base(&mut self, value: f32) {
        self.base = value;
        self.cache_valid = false;
    }

    /// Returns the evaluated scalar for `definition`, optionally counting aggregation passes.
    pub fn read_scalar(
        &mut self,
        definition: &AttributeDefinition,
        audit: Option<&mut u32>,
    ) -> f32 {
        if self.cache_valid {
            return self.current_cached;
        }

        let evaluated = self.modifier_stack.evaluate_tracked(
            self.base,
            definition.min_value,
            definition.max_value,
            audit,
        );
        self.current_cached = evaluated;
        self.cache_valid = true;
        evaluated
    }

    /// Forces recomputation on the next read.
    pub fn invalidate_cache(&mut self) {
        self.cache_valid = false;
    }

    /// Recomputes the scalar and reports whether the published value changed.
    pub fn evaluate(&mut self, definition: &AttributeDefinition) -> bool {
        let before = if self.cache_valid {
            self.current_cached
        } else {
            self.modifier_stack
                .evaluate(self.base, definition.min_value, definition.max_value)
        };
        self.cache_valid = false;
        let after = self.read_scalar(definition, None);
        (after - before).abs() > f32::EPSILON
    }
}

/// ECS-facing attribute collection for one schema.
#[derive(Clone, Debug, PartialEq)]
pub struct AttributeSet {
    /// Owning schema identifier.
    pub schema_id: AttributeSchemaId,
    /// Values aligned with `AttributeSchema::attributes` order.
    pub values: Vec<AttributeValue>,
}

impl AttributeSet {
    /// Builds a set from a schema using default values.
    pub fn from_schema(schema: &AttributeSchema) -> Self {
        let values = schema
            .attributes
            .iter()
            .map(|field| AttributeValue::new(field.default_value))
            .collect();
        Self {
            schema_id: schema.id,
            values,
        }
    }

    /// Borrows a field value by id when the schema matches ordering.
    #[must_use]
    pub fn get_by_id<'a>(
        &'a self,
        id: AttributeDefId,
        schema: &AttributeSchema,
    ) -> Option<&'a AttributeValue> {
        let index = schema.index_of(id)?;
        self.values.get(index)
    }

    /// Mutably borrows a field value by id.
    pub fn get_by_id_mut(
        &mut self,
        id: AttributeDefId,
        schema: &AttributeSchema,
    ) -> Option<&mut AttributeValue> {
        let index = schema.index_of(id)?;
        self.values.get_mut(index)
    }

    /// Reads a scalar field by editor name.
    pub fn read_named_scalar(
        &mut self,
        schema: &AttributeSchema,
        name: &str,
        audit: Option<&mut u32>,
    ) -> Option<f32> {
        let field = schema.find_by_name(name)?;
        let index = schema.index_of(field.id)?;
        let value = self.values.get_mut(index)?;
        Some(value.read_scalar(field, audit))
    }

    /// Writes a scalar field by editor name with clamping.
    pub fn write_named_scalar(
        &mut self,
        schema: &AttributeSchema,
        name: &str,
        scalar: f32,
    ) -> Option<()> {
        let field = schema.find_by_name(name)?;
        let index = schema.index_of(field.id)?;
        let value = self.values.get_mut(index)?;
        let clamped = scalar.clamp(field.min_value, field.max_value);
        value.set_base(clamped);
        Some(())
    }

    /// Evaluates every field, returning indices whose outputs changed.
    pub fn evaluate_all(&mut self, schema: &AttributeSchema) -> SmallVec<[usize; 8]> {
        let mut changed = SmallVec::new();
        for (index, field) in schema.attributes.iter().enumerate() {
            if let Some(value) = self.values.get_mut(index) {
                if value.evaluate(field) {
                    changed.push(index);
                }
            }
        }
        changed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_schema() -> AttributeSchema {
        AttributeSchema {
            id: AttributeSchemaId(1),
            name: SmolStr::new("character"),
            attributes: vec![
                AttributeDefinition {
                    id: AttributeDefId(1),
                    name: SmolStr::new("strength"),
                    value_type: AttributeValueType::F32,
                    min_value: 0.0,
                    max_value: 100.0,
                    default_value: 10.0,
                    replicate: true,
                },
                AttributeDefinition {
                    id: AttributeDefId(2),
                    name: SmolStr::new("agility"),
                    value_type: AttributeValueType::F32,
                    min_value: 0.0,
                    max_value: 100.0,
                    default_value: 8.0,
                    replicate: true,
                },
                AttributeDefinition {
                    id: AttributeDefId(3),
                    name: SmolStr::new("intellect"),
                    value_type: AttributeValueType::F32,
                    min_value: 0.0,
                    max_value: 100.0,
                    default_value: 5.0,
                    replicate: true,
                },
            ],
        }
    }

    #[test]
    fn test_attribute_schema_typed_def() {
        let schema = AttributeSchema {
            id: AttributeSchemaId(2),
            name: SmolStr::new("typed"),
            attributes: vec![
                AttributeDefinition {
                    id: AttributeDefId(1),
                    name: SmolStr::new("speed"),
                    value_type: AttributeValueType::F32,
                    min_value: 0.0,
                    max_value: 10.0,
                    default_value: 1.0,
                    replicate: false,
                },
                AttributeDefinition {
                    id: AttributeDefId(2),
                    name: SmolStr::new("level"),
                    value_type: AttributeValueType::I32,
                    min_value: 0.0,
                    max_value: 99.0,
                    default_value: 1.0,
                    replicate: false,
                },
                AttributeDefinition {
                    id: AttributeDefId(3),
                    name: SmolStr::new("alive"),
                    value_type: AttributeValueType::Bool,
                    min_value: 0.0,
                    max_value: 1.0,
                    default_value: 1.0,
                    replicate: false,
                },
                AttributeDefinition {
                    id: AttributeDefId(4),
                    name: SmolStr::new("role"),
                    value_type: AttributeValueType::Enum { type_id: 42 },
                    min_value: 0.0,
                    max_value: 10.0,
                    default_value: 0.0,
                    replicate: false,
                },
            ],
        };

        let speed = schema.find_by_name("speed").expect("speed");
        assert_eq!(speed.value_type, AttributeValueType::F32);
        let level = schema.find_by_name("level").expect("level");
        assert_eq!(level.value_type, AttributeValueType::I32);
        let alive = schema.find_by_name("alive").expect("alive");
        assert_eq!(alive.value_type, AttributeValueType::Bool);
        let role = schema.find_by_name("role").expect("role");
        assert_eq!(role.value_type, AttributeValueType::Enum { type_id: 42 });
    }

    #[test]
    fn test_attribute_set_defaults() {
        let schema = AttributeSchema {
            id: AttributeSchemaId(3),
            name: SmolStr::new("defaults"),
            attributes: vec![
                AttributeDefinition {
                    id: AttributeDefId(1),
                    name: SmolStr::new("speed"),
                    value_type: AttributeValueType::F32,
                    min_value: 0.0,
                    max_value: 20.0,
                    default_value: 5.0,
                    replicate: false,
                },
                AttributeDefinition {
                    id: AttributeDefId(2),
                    name: SmolStr::new("level"),
                    value_type: AttributeValueType::I32,
                    min_value: 0.0,
                    max_value: 99.0,
                    default_value: 1.0,
                    replicate: false,
                },
                AttributeDefinition {
                    id: AttributeDefId(3),
                    name: SmolStr::new("alive"),
                    value_type: AttributeValueType::Bool,
                    min_value: 0.0,
                    max_value: 1.0,
                    default_value: 1.0,
                    replicate: false,
                },
                AttributeDefinition {
                    id: AttributeDefId(4),
                    name: SmolStr::new("role"),
                    value_type: AttributeValueType::Enum { type_id: 1 },
                    min_value: 0.0,
                    max_value: 10.0,
                    default_value: 0.0,
                    replicate: false,
                },
            ],
        };

        let mut set = AttributeSet::from_schema(&schema);
        assert_eq!(
            set.read_named_scalar(&schema, "speed", None)
                .expect("speed"),
            5.0
        );
        assert_eq!(
            set.read_named_scalar(&schema, "level", None)
                .expect("level"),
            1.0
        );
        assert_eq!(
            set.read_named_scalar(&schema, "alive", None)
                .expect("alive"),
            1.0
        );
        assert_eq!(
            set.read_named_scalar(&schema, "role", None).expect("role"),
            0.0
        );
    }

    #[test]
    fn test_attribute_min_max_clamp() {
        let schema = AttributeSchema {
            id: AttributeSchemaId(4),
            name: SmolStr::new("bounds"),
            attributes: vec![AttributeDefinition {
                id: AttributeDefId(1),
                name: SmolStr::new("hp"),
                value_type: AttributeValueType::F32,
                min_value: 0.0,
                max_value: 10.0,
                default_value: 5.0,
                replicate: false,
            }],
        };

        let mut set = AttributeSet::from_schema(&schema);
        set.write_named_scalar(&schema, "hp", 15.0).expect("write");
        assert_eq!(
            set.read_named_scalar(&schema, "hp", None).expect("hp"),
            10.0
        );
        set.write_named_scalar(&schema, "hp", -5.0).expect("write");
        assert_eq!(set.read_named_scalar(&schema, "hp", None).expect("hp"), 0.0);
    }

    #[test]
    fn test_attribute_memoized_read() {
        use crate::shared::{Entity, ModOp, StatModifier};

        let schema = AttributeSchema {
            id: AttributeSchemaId(5),
            name: SmolStr::new("memo"),
            attributes: vec![AttributeDefinition {
                id: AttributeDefId(1),
                name: SmolStr::new("hp"),
                value_type: AttributeValueType::F32,
                min_value: 0.0,
                max_value: 200.0,
                default_value: 100.0,
                replicate: false,
            }],
        };

        let mut set = AttributeSet::from_schema(&schema);
        let hp = set
            .get_by_id_mut(AttributeDefId(1), &schema)
            .expect("hp field");
        for index in 0..4 {
            hp.modifier_stack.add(StatModifier {
                op: ModOp::Flat,
                value: 1.0,
                source: Entity(index),
                priority: 0,
            });
        }

        let mut calls = 0u32;
        let first = set
            .read_named_scalar(&schema, "hp", Some(&mut calls))
            .expect("hp");
        let second = set
            .read_named_scalar(&schema, "hp", Some(&mut calls))
            .expect("hp");
        assert_eq!(first, second);
        assert_eq!(calls, 1);
    }

    #[test]
    fn test_attribute_invalidate_on_set() {
        use crate::shared::{Entity, ModOp, StatModifier};

        let schema = AttributeSchema {
            id: AttributeSchemaId(6),
            name: SmolStr::new("invalidate"),
            attributes: vec![AttributeDefinition {
                id: AttributeDefId(1),
                name: SmolStr::new("hp"),
                value_type: AttributeValueType::F32,
                min_value: 0.0,
                max_value: 200.0,
                default_value: 40.0,
                replicate: false,
            }],
        };

        let mut set = AttributeSet::from_schema(&schema);
        let hp = set
            .get_by_id_mut(AttributeDefId(1), &schema)
            .expect("hp field");
        hp.modifier_stack.add(StatModifier {
            op: ModOp::Flat,
            value: 2.0,
            source: Entity(1),
            priority: 0,
        });

        let mut calls = 0u32;
        let _ = set
            .read_named_scalar(&schema, "hp", Some(&mut calls))
            .expect("hp");
        set.write_named_scalar(&schema, "hp", 50.0).expect("write");
        let _ = set
            .read_named_scalar(&schema, "hp", Some(&mut calls))
            .expect("hp");
        assert_eq!(calls, 2);
    }

    #[test]
    fn test_character_stats_from_set() {
        let schema = sample_schema();
        let mut set = AttributeSet::from_schema(&schema);
        assert_eq!(
            set.read_named_scalar(&schema, "strength", None)
                .expect("strength"),
            10.0
        );
        assert_eq!(
            set.read_named_scalar(&schema, "agility", None)
                .expect("agility"),
            8.0
        );
        assert_eq!(
            set.read_named_scalar(&schema, "intellect", None)
                .expect("intellect"),
            5.0
        );
    }
}
