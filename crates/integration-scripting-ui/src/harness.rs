//! Headless harness wiring UI events, scripting handlers, bindings, and dialogue CH-9.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::types::{
    ArgValue, AssetId, BindingSource, BindingUpdateMode, DataBinding, DialogueChoice, Entity,
    FallbackCounters, FormattedString, ScriptValue, SetWidgetState, VisibilityEval, WidgetEvent,
    WidgetEventKind, WidgetStateKind,
};

/// Scripting-side handler invoked for each `WidgetEvent` during Phase 3 simulation.
pub type WidgetEventHandler = dyn FnMut(&WidgetEvent, &mut ScriptingUiHarness);

/// Resolved binding payload before widget-type coercion.
#[derive(Clone, Debug, Eq, PartialEq)]
enum RawBinding {
    /// Displayable UTF-8 payload.
    Text(String),
    /// Scalar that cannot populate a text widget without coercion (FM-2).
    IncompatibleU32(u32),
}

/// Per-widget snapshot consumed by layout tests.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WidgetSnapshot {
    /// Resolved label text.
    pub text: String,
    /// Layout visibility.
    pub visible: bool,
    /// Interaction enabled (inverse of `InteractionState.disabled`).
    pub enabled: bool,
    /// Slider/scalar model value.
    pub value: f64,
    /// Progress bar fraction.
    pub progress: f32,
    /// Image asset id.
    pub image: AssetId,
    /// When true, `Clicked` events are suppressed (IR-4.5.6).
    pub interaction_disabled: bool,
}

/// Fake ECS + channels for CI tests.
#[derive(Debug)]
pub struct ScriptingUiHarness {
    /// Widgets that still exist in the ECS view.
    pub alive: BTreeSet<Entity>,
    /// Widget state keyed by entity.
    pub widgets: BTreeMap<Entity, WidgetSnapshot>,
    /// Monotonically increasing entity id generator.
    next_entity: u64,
    /// Current simulation frame / tick counter.
    pub frame_index: u64,
    /// `Health::current` keyed by entity for component bindings.
    pub health: BTreeMap<Entity, u32>,
    /// `(row_id, field)` -> string cell for data table bindings.
    pub data_table: BTreeMap<(String, String), String>,
    /// Events UI emitted for scripting (Phase 3 input).
    pub widget_events: VecDeque<WidgetEvent>,
    /// State writes scripting produced for UI (Phase 7 input).
    pub set_widget_state: VecDeque<SetWidgetState>,
    /// Fallback counters FM-1 … FM-7.
    pub counters: FallbackCounters,
    /// CH-9 bounded queue + overflow staging for FM-3.
    dialogue_main: VecDeque<DialogueChoice>,
    dialogue_overflow: VecDeque<DialogueChoice>,
    /// Capacity of `dialogue_main` (design CH-9 cap = 16).
    pub dialogue_cap: usize,
    /// Active dialogue entities (closed dialogues reject new choices).
    pub active_dialogues: BTreeSet<Entity>,
    /// Last source fingerprint for `OnChange` mode keyed by widget entity + path.
    on_change_fp: BTreeMap<(Entity, String), String>,
    /// Last rendered text for `OnChange` skips.
    on_change_last_text: BTreeMap<(Entity, String), String>,
    /// Handlers disabled after panic (FM-4).
    disabled_handlers: BTreeSet<usize>,
    /// Increments whenever a binding fully evaluates its source (not an OnChange skip).
    pub binding_eval_calls: u64,
    /// Increments when `OnChange` short-circuits because the fingerprint matched.
    pub binding_eval_skips: u64,
}

impl ScriptingUiHarness {
    fn data_table_lookup(table: &BTreeMap<(String, String), String>, path: &str) -> String {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.len() < 2 {
            return String::new();
        }
        let field = parts[parts.len() - 1].to_string();
        let row = parts[..parts.len() - 1].join(".");
        table.get(&(row, field)).cloned().unwrap_or_default()
    }

    /// Builds an empty harness with CH-9 capacity 16.
    #[must_use]
    pub fn new() -> Self {
        Self {
            alive: BTreeSet::new(),
            widgets: BTreeMap::new(),
            next_entity: 1,
            frame_index: 0,
            health: BTreeMap::new(),
            data_table: BTreeMap::new(),
            widget_events: VecDeque::new(),
            set_widget_state: VecDeque::new(),
            counters: FallbackCounters::default(),
            dialogue_main: VecDeque::new(),
            dialogue_overflow: VecDeque::new(),
            dialogue_cap: 16,
            active_dialogues: BTreeSet::new(),
            on_change_fp: BTreeMap::new(),
            on_change_last_text: BTreeMap::new(),
            disabled_handlers: BTreeSet::new(),
            binding_eval_calls: 0,
            binding_eval_skips: 0,
        }
    }

    /// Allocates a new entity id and marks it alive with default widget snapshot.
    pub fn spawn_widget(&mut self) -> Entity {
        let e = Entity(self.next_entity);
        self.next_entity += 1;
        self.alive.insert(e);
        self.widgets.insert(
            e,
            WidgetSnapshot {
                text: String::new(),
                visible: true,
                enabled: true,
                value: 0.0,
                progress: 0.0,
                image: AssetId(0),
                interaction_disabled: false,
            },
        );
        e
    }

    /// Removes an entity from the live set (despawn).
    pub fn despawn(&mut self, entity: Entity) {
        self.alive.remove(&entity);
        self.widgets.remove(&entity);
        self.health.remove(&entity);
    }

    /// Marks a dialogue entity as active (accepts choices).
    pub fn open_dialogue(&mut self, dialogue: Entity) {
        self.active_dialogues.insert(dialogue);
    }

    /// Marks dialogue finished (orphan choices increment FM-7).
    pub fn close_dialogue(&mut self, dialogue: Entity) {
        self.active_dialogues.remove(&dialogue);
    }

    /// Drains scripting → UI writes into widget snapshots.
    pub fn apply_set_widget_states(&mut self) {
        while let Some(cmd) = self.set_widget_state.pop_front() {
            if !self.alive.contains(&cmd.target) {
                continue;
            }
            let Some(w) = self.widgets.get_mut(&cmd.target) else {
                continue;
            };
            match cmd.kind {
                WidgetStateKind::Text => {
                    if let ScriptValue::Text(t) = cmd.value {
                        w.text = t;
                    }
                }
                WidgetStateKind::Visible => {
                    if let ScriptValue::Bool(v) = cmd.value {
                        w.visible = v;
                    }
                }
                WidgetStateKind::Enabled => {
                    if let ScriptValue::Bool(v) = cmd.value {
                        w.enabled = v;
                    }
                }
                WidgetStateKind::Value => {
                    if let ScriptValue::F64(v) = cmd.value {
                        w.value = v;
                    }
                }
                WidgetStateKind::ProgressFraction => {
                    if let ScriptValue::F64(v) = cmd.value {
                        w.progress = v as f32;
                    }
                }
                WidgetStateKind::ImageAsset => {
                    if let ScriptValue::Image(a) = cmd.value {
                        w.image = a;
                    }
                }
                WidgetStateKind::Selected => {}
            }
            w.interaction_disabled = !w.enabled;
        }
    }

    /// Emits a click event if the widget is alive, visible, and not interaction-disabled.
    pub fn emit_click(&mut self, target: Entity) {
        self.emit_widget_event(target, WidgetEventKind::Clicked);
    }

    /// Emits a structured widget event from UI toward scripting.
    pub fn emit_widget_event(&mut self, target: Entity, kind: WidgetEventKind) {
        if !self.alive.contains(&target) {
            self.counters.fm1 += 1;
            return;
        }
        let Some(w) = self.widgets.get(&target) else {
            self.counters.fm1 += 1;
            return;
        };
        if !w.visible {
            return;
        }
        if w.interaction_disabled {
            return;
        }
        self.widget_events.push_back(WidgetEvent {
            target,
            kind,
            frame_index: self.frame_index,
        });
    }

    /// Runs registered handlers against pending widget events (Phase 3 simulation slice).
    pub fn drain_widget_events_to_script(&mut self, handlers: &mut [Box<WidgetEventHandler>]) {
        while let Some(evt) = self.widget_events.pop_front() {
            if !self.alive.contains(&evt.target) {
                self.counters.fm1 += 1;
                continue;
            }
            for (idx, h) in handlers.iter_mut().enumerate() {
                if self.disabled_handlers.contains(&idx) {
                    continue;
                }
                let r = catch_unwind(AssertUnwindSafe(|| h(&evt, self)));
                if r.is_err() {
                    self.counters.fm4 += 1;
                    self.disabled_handlers.insert(idx);
                }
            }
        }
    }

    /// Enqueues dialogue choice on CH-9 with overflow accounting (FM-3).
    pub fn enqueue_dialogue_choice(&mut self, choice: DialogueChoice) {
        if !self.active_dialogues.contains(&choice.dialogue) {
            self.counters.fm7 += 1;
            return;
        }
        if self.dialogue_main.len() >= self.dialogue_cap {
            self.counters.fm3 += 1;
            self.dialogue_overflow.push_back(choice);
        } else {
            self.dialogue_main.push_back(choice);
        }
    }

    /// Drains one dialogue message from CH-9 main queue, promoting overflow when space appears.
    pub fn drain_one_dialogue_choice(&mut self) -> Option<DialogueChoice> {
        let c = self.dialogue_main.pop_front()?;
        if !self.dialogue_overflow.is_empty() && self.dialogue_main.len() < self.dialogue_cap {
            if let Some(next) = self.dialogue_overflow.pop_front() {
                self.dialogue_main.push_back(next);
            }
        }
        Some(c)
    }

    /// Evaluates `visible_expr` / `enabled_expr` using harness callbacks.
    pub fn visibility_eval(
        &mut self,
        widget: Entity,
        visible_expr: impl FnOnce(&mut Self) -> Result<bool, ()>,
        enabled_expr: impl FnOnce(&mut Self) -> Result<bool, ()>,
    ) -> VisibilityEval {
        let visible = match visible_expr(self) {
            Ok(v) => v,
            Err(()) => {
                self.counters.fm5 += 1;
                true
            }
        };
        let enabled = enabled_expr(self).unwrap_or(true);
        VisibilityEval {
            widget,
            visible,
            enabled,
        }
    }

    /// Applies visibility eval to widget snapshot (pre-layout).
    pub fn apply_visibility(&mut self, eval: VisibilityEval) {
        if let Some(w) = self.widgets.get_mut(&eval.widget) {
            w.visible = eval.visible;
            w.enabled = eval.enabled;
            w.interaction_disabled = !eval.enabled;
        }
    }

    /// Counts widgets that would participate in layout (visible only).
    #[must_use]
    pub fn layout_visible_count(&self) -> usize {
        self.widgets
            .iter()
            .filter(|(e, w)| self.alive.contains(e) && w.visible)
            .count()
    }

    /// Resolves a binding to display text for a widget on `entity`.
    #[must_use]
    pub fn resolve_binding_text(
        &mut self,
        widget: Entity,
        binding: &DataBinding,
        expect_text_widget: bool,
    ) -> String {
        let key = (widget, binding.path.clone());
        if binding.mode == BindingUpdateMode::OnChange {
            let fp = self.binding_fingerprint(widget, binding);
            if self.on_change_fp.get(&key) == Some(&fp) {
                self.binding_eval_skips += 1;
                return self
                    .on_change_last_text
                    .get(&key)
                    .cloned()
                    .unwrap_or_default();
            }
            self.binding_eval_calls += 1;
            self.on_change_fp.insert(key.clone(), fp);
            let raw = self.resolve_binding_raw(widget, binding);
            let text = self.coerce_raw_binding(raw, expect_text_widget);
            self.on_change_last_text.insert(key, text.clone());
            return text;
        }

        self.binding_eval_calls += 1;
        let raw = self.resolve_binding_raw(widget, binding);
        self.coerce_raw_binding(raw, expect_text_widget)
    }

    fn binding_fingerprint(&self, widget: Entity, binding: &DataBinding) -> String {
        match binding.source {
            BindingSource::EntityComponent => {
                if binding.path == "player.hp" {
                    self.health
                        .get(&widget)
                        .map(|v| v.to_string())
                        .unwrap_or_default()
                } else if binding.path == "binding.u32_only" {
                    self.health
                        .get(&widget)
                        .map(|v| format!("u32:{v}"))
                        .unwrap_or_default()
                } else {
                    String::new()
                }
            }
            BindingSource::Resource => {
                if binding.path == "game_time.tick" {
                    self.frame_index.to_string()
                } else {
                    String::new()
                }
            }
            BindingSource::DataTableRow => {
                Self::data_table_lookup(&self.data_table, &binding.path)
            }
            BindingSource::ScriptOutput | BindingSource::Blackboard => String::new(),
        }
    }

    fn resolve_binding_raw(&mut self, widget: Entity, binding: &DataBinding) -> RawBinding {
        match binding.source {
            BindingSource::EntityComponent => {
                if binding.path == "player.hp" {
                    RawBinding::Text(
                        self.health
                            .get(&widget)
                            .map(|v| v.to_string())
                            .unwrap_or_default(),
                    )
                } else if binding.path == "binding.u32_only" {
                    if let Some(v) = self.health.get(&widget).copied() {
                        RawBinding::IncompatibleU32(v)
                    } else {
                        self.counters.fm6 += 1;
                        RawBinding::Text("<missing>".into())
                    }
                } else {
                    RawBinding::Text(String::new())
                }
            }
            BindingSource::Resource => {
                if binding.path == "game_time.tick" {
                    RawBinding::Text(self.frame_index.to_string())
                } else {
                    self.counters.fm6 += 1;
                    RawBinding::Text("<missing>".into())
                }
            }
            BindingSource::DataTableRow => RawBinding::Text(Self::data_table_lookup(
                &self.data_table,
                &binding.path,
            )),
            BindingSource::ScriptOutput | BindingSource::Blackboard => RawBinding::Text(String::new()),
        }
    }

    fn coerce_raw_binding(&mut self, raw: RawBinding, expect_text_widget: bool) -> String {
        match raw {
            RawBinding::Text(s) => s,
            RawBinding::IncompatibleU32(_) => {
                if expect_text_widget {
                    self.counters.fm2 += 1;
                    "<err>".into()
                } else {
                    // Non-text consumers are not exercised in current TC set.
                    "<err>".into()
                }
            }
        }
    }

    /// Advances frame counter (drives resource bindings and event stamps).
    pub fn tick_frame(&mut self) {
        self.frame_index += 1;
    }

    /// Resolves a tiny plural template used by TC-IR-4.5.5.1.
    #[must_use]
    pub fn resolve_formatted_plural_items(formatted: &FormattedString) -> String {
        let n = formatted
            .args
            .iter()
            .find(|(k, _)| k == "n")
            .and_then(|(_, v)| match v {
                ArgValue::I64(i) => Some(*i),
                _ => None,
            })
            .unwrap_or(0);
        if n == 1 {
            "1 item".into()
        } else {
            format!("{n} items")
        }
    }
}

impl Default for ScriptingUiHarness {
    fn default() -> Self {
        Self::new()
    }
}
