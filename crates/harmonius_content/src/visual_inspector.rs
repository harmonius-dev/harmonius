//! Static field-descriptor → widget mapping for editor UI (no runtime reflection).

/// Widget kinds matching primitive editor controls.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WidgetKind {
    /// Single-line text.
    TextField,
    /// Integer stepper.
    IntegerSpinner,
    /// RGBA color picker.
    ColorPicker,
}

/// One row in the inspector.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InspectorRow {
    /// Field name.
    pub name: String,
    /// Matched widget.
    pub widget: WidgetKind,
}

/// Build inspector rows for a toy asset with `name: String`, `count: u32`, `color: [f32;4]`.
pub fn visual_inspector_fields() -> Vec<InspectorRow> {
    vec![
        InspectorRow {
            name: "name".into(),
            widget: WidgetKind::TextField,
        },
        InspectorRow {
            name: "count".into(),
            widget: WidgetKind::IntegerSpinner,
        },
        InspectorRow {
            name: "color".into(),
            widget: WidgetKind::ColorPicker,
        },
    ]
}
