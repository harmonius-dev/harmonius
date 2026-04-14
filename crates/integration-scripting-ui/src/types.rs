//! Shared types for scripting ↔ UI integration contracts.

/// Opaque ECS entity handle used by the headless harness.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Entity(pub u64);

/// Stable localization table key (see `localization-ui` integration).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LocalizedStringId(pub u32);

/// Opaque image asset handle.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AssetId(pub u32);

/// Script-side literal or bound value applied to widget state.
#[derive(Clone, Debug, PartialEq)]
pub enum ScriptValue {
    /// Boolean literal.
    Bool(bool),
    /// Scalar literal (progress, slider value, etc.).
    F64(f64),
    /// Text literal.
    Text(String),
    /// Image asset literal.
    Image(AssetId),
    /// Wrong-type sentinel for binding mismatch tests.
    U32(u32),
}

/// Expression placeholder carried on `SetWidgetState` in the harness.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BoundExpr {
    /// Fake codegen slot id.
    pub codegen_id: u32,
}

/// High-level expression classification (design `ExprKind`).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExprKind {
    /// Literal leaf.
    Literal,
    /// Identifier leaf.
    Identifier,
    /// Function call.
    Call,
    /// Binary operator.
    BinaryOp,
    /// Field access.
    FieldAccess,
    /// Index operator.
    Index,
}

impl BoundExpr {
    /// Builds a literal-bound expression tag for tests.
    #[must_use]
    pub const fn literal() -> Self {
        Self { codegen_id: 0 }
    }
}

/// Widget interaction event emitted by UI toward scripting.
#[derive(Clone, Debug, PartialEq)]
pub struct WidgetEvent {
    /// Hit-tested widget entity.
    pub target: Entity,
    /// Interaction classification.
    pub kind: WidgetEventKind,
    /// Monotonic frame index from `GameTime`.
    pub frame_index: u64,
}

/// Interaction kinds for `WidgetEvent`.
#[derive(Clone, Debug, PartialEq)]
pub enum WidgetEventKind {
    /// Primary click completed.
    Clicked,
    /// Form submit gesture.
    Submitted,
    /// Value edit (slider, text field).
    ValueChanged {
        /// New normalized scalar when applicable.
        new: f64,
    },
    /// Focus gained.
    Focused,
    /// Focus lost.
    Blurred,
    /// Pointer hover.
    Hovered,
    /// Selection index changed.
    SelectedChanged,
}

/// State writes produced by scripting for UI layout.
#[derive(Clone, Debug, PartialEq)]
pub struct SetWidgetState {
    /// Target widget entity.
    pub target: Entity,
    /// Component/state slot to write.
    pub kind: WidgetStateKind,
    /// Bound expression metadata (harness uses `ScriptValue` directly).
    pub value: ScriptValue,
    /// Design trace field for codegen linkage.
    pub bound: BoundExpr,
}

/// Widget-facing state slots referenced by IR-4.5.1.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WidgetStateKind {
    /// Label or body text.
    Text,
    /// Scalar value (slider model).
    Value,
    /// Visibility flag.
    Visible,
    /// Interaction enabled flag.
    Enabled,
    /// Selection index.
    Selected,
    /// Image asset binding.
    ImageAsset,
    /// Normalized progress in `[0, 1]`.
    ProgressFraction,
}

/// Where a `DataBinding` reads from during layout.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BindingSource {
    /// Component field on a bound entity.
    EntityComponent,
    /// Engine resource keyed by stable name.
    Resource,
    /// Data table row field.
    DataTableRow,
    /// Script output slot.
    ScriptOutput,
    /// Blackboard key.
    Blackboard,
}

/// How often a binding re-evaluates.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BindingUpdateMode {
    /// Resolve once at first layout.
    Once,
    /// Resolve every layout frame.
    EveryFrame,
    /// Resolve only when the source value changes.
    OnChange,
}

/// Declarative binding from UI layout to ECS-like data.
#[derive(Clone, Debug, PartialEq)]
pub struct DataBinding {
    /// Storage class for the path.
    pub source: BindingSource,
    /// Stable path string (`player.hp`, `game_time.tick`, ...).
    pub path: String,
    /// Refresh policy.
    pub mode: BindingUpdateMode,
    /// Design-time expression link.
    pub path_expr: BoundExpr,
}

/// Player picked a dialogue option (CH-9 payload shape).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DialogueChoice {
    /// Dialogue root entity.
    pub dialogue: Entity,
    /// Zero-based option index.
    pub choice_index: u32,
    /// Label shown for the option.
    pub label: LocalizedStringId,
}

/// ICU-ish formatted message with positional/named args (subset).
#[derive(Clone, Debug, PartialEq)]
pub struct FormattedString {
    /// Template id resolved later by localization.
    pub id: LocalizedStringId,
    /// Named args (`n` -> value).
    pub args: Vec<(String, ArgValue)>,
}

/// Typed argument bundle entry.
#[derive(Clone, Debug, PartialEq)]
pub enum ArgValue {
    /// Integer used by plural rules.
    I64(i64),
    /// Floating scalar.
    F64(f64),
    /// Inline text fragment.
    Text(String),
}

/// Result of evaluating visibility expressions before layout.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VisibilityEval {
    /// Widget entity evaluated.
    pub widget: Entity,
    /// Layout inclusion flag.
    pub visible: bool,
    /// Interaction gating flag.
    pub enabled: bool,
}

/// Counters for documented fallback modes FM-1 … FM-7.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FallbackCounters {
    /// FM-1: event target despawned.
    pub fm1: u32,
    /// FM-2: binding type mismatch.
    pub fm2: u32,
    /// FM-3: CH-9 backpressure.
    pub fm3: u32,
    /// FM-4: script handler panic.
    pub fm4: u32,
    /// FM-5: missing visibility dependency.
    pub fm5: u32,
    /// FM-6: binding source missing.
    pub fm6: u32,
    /// FM-7: orphan dialogue choice.
    pub fm7: u32,
}
