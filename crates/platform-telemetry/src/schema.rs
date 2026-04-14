//! Static schema catalog (R-14.5.6).

use crate::error::TelemetryError;
use crate::types::{PiiClass, SchemaId, Scope};

/// Supported primitive field kinds for validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FieldKind {
    /// UTF-8 string payload.
    String,
    /// Unsigned 32-bit integer.
    U32,
    /// 32-bit float.
    F32,
}

/// Field metadata used for export/delete compliance filtering.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldDescriptor {
    /// Column name in the schema.
    pub name: String,
    /// Wire kind for validation.
    pub kind: FieldKind,
    /// PII classification for compliance tooling.
    pub pii: PiiClass,
    /// Whether the field must be present in an archived row.
    pub required: bool,
}

/// Declared telemetry event schema entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventSchema {
    /// Stable numeric id.
    pub id: SchemaId,
    /// Human-readable name.
    pub name: String,
    /// Default consent scope for instances of this event.
    pub scope: Scope,
    /// Ordered field list.
    pub fields: Vec<FieldDescriptor>,
}

/// Immutable catalog of all emitted schemas.
#[derive(Clone, Debug, Default)]
pub struct SchemaCatalog {
    schemas: Vec<EventSchema>,
}

impl SchemaCatalog {
    /// Build a catalog after validating that no field uses [`PiiClass::Sensitive`].
    pub fn from_schemas(schemas: Vec<EventSchema>) -> Result<Self, TelemetryError> {
        for schema in &schemas {
            for field in &schema.fields {
                if field.pii == PiiClass::Sensitive {
                    return Err(TelemetryError::Config(
                        "schema fields may not use PiiClass::Sensitive",
                    ));
                }
            }
        }
        Ok(Self { schemas })
    }

    /// Lookup a schema by id.
    pub fn lookup(&self, id: SchemaId) -> Option<&EventSchema> {
        self.schemas.iter().find(|s| s.id == id)
    }

    /// Iterator for codegen and export tooling.
    pub fn iter(&self) -> impl Iterator<Item = &EventSchema> {
        self.schemas.iter()
    }
}

/// Validate required fields then concatenate tagged payloads for transport.
pub fn archive_event_wired(
    schema: &EventSchema,
    field_payloads: &[(&str, &[u8])],
) -> Result<Vec<u8>, TelemetryError> {
    for field in &schema.fields {
        if field.required {
            let present = field_payloads
                .iter()
                .any(|(name, _)| *name == field.name.as_str());
            if !present {
                return Err(TelemetryError::Config("missing required field"));
            }
        }
    }
    let mut out = Vec::new();
    for (name, bytes) in field_payloads {
        out.extend_from_slice(name.as_bytes());
        out.push(0);
        out.extend_from_slice(bytes);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_catalog_lookup_by_id() {
        let schema = EventSchema {
            id: SchemaId(7),
            name: "FrameTiming".to_owned(),
            scope: Scope::Engine,
            fields: vec![FieldDescriptor {
                name: "frame_ms".to_owned(),
                kind: FieldKind::F32,
                pii: PiiClass::None,
                required: true,
            }],
        };
        let catalog = SchemaCatalog::from_schemas(vec![schema.clone()]).unwrap();
        assert_eq!(catalog.lookup(SchemaId(7)).unwrap().name, "FrameTiming");
    }

    #[test]
    fn test_schema_required_fields_enforced() {
        let schema = EventSchema {
            id: SchemaId(1),
            name: "NeedsName".to_owned(),
            scope: Scope::GameLogic,
            fields: vec![FieldDescriptor {
                name: "title".to_owned(),
                kind: FieldKind::String,
                pii: PiiClass::None,
                required: true,
            }],
        };
        let err = archive_event_wired(&schema, &[]).unwrap_err();
        assert!(matches!(err, TelemetryError::Config(_)));
    }

    #[test]
    fn rejects_sensitive_field_catalog() {
        let schema = EventSchema {
            id: SchemaId(99),
            name: "Bad".into(),
            scope: Scope::Engine,
            fields: vec![FieldDescriptor {
                name: "geo".into(),
                kind: FieldKind::String,
                pii: PiiClass::Sensitive,
                required: false,
            }],
        };
        assert!(SchemaCatalog::from_schemas(vec![schema]).is_err());
    }
}
