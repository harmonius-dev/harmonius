//! Integration tests mapped from `docs/design/integration/scripting-ui-test-cases.md`.

use std::cell::Cell;
use std::rc::Rc;
use std::time::Instant;

use integration_scripting_ui::{
    ArgValue, AssetId, BindingSource, BindingUpdateMode, BoundExpr, DataBinding, DialogueChoice,
    Entity, FormattedString, LocalizedStringId, ScriptValue, ScriptingUiHarness, SetWidgetState,
    WidgetEventHandler, WidgetEventKind, WidgetStateKind,
};

fn set_cmd(target: Entity, kind: WidgetStateKind, value: ScriptValue) -> SetWidgetState {
    SetWidgetState {
        target,
        kind,
        value,
        bound: BoundExpr::literal(),
    }
}

// --- IR-4.5.1 SetWidgetState -------------------------------------------------

#[test]
fn tc_ir_4_5_1_1_script_sets_widget_text() {
    let mut h = ScriptingUiHarness::new();
    let w = h.spawn_widget();
    h.set_widget_state.push_back(set_cmd(
        w,
        WidgetStateKind::Text,
        ScriptValue::Text("Hi".into()),
    ));
    h.apply_set_widget_states();
    assert_eq!(h.widgets[&w].text, "Hi");
}

#[test]
fn tc_ir_4_5_1_2_script_sets_widget_visibility() {
    let mut h = ScriptingUiHarness::new();
    let w = h.spawn_widget();
    h.set_widget_state
        .push_back(set_cmd(w, WidgetStateKind::Visible, ScriptValue::Bool(false)));
    h.apply_set_widget_states();
    assert!(!h.widgets[&w].visible);
}

#[test]
fn tc_ir_4_5_1_3_script_sets_progress_fraction() {
    let mut h = ScriptingUiHarness::new();
    let w = h.spawn_widget();
    h.set_widget_state.push_back(set_cmd(
        w,
        WidgetStateKind::ProgressFraction,
        ScriptValue::F64(0.42),
    ));
    h.apply_set_widget_states();
    assert!((f64::from(h.widgets[&w].progress) - 0.42).abs() < 1e-5);
}

#[test]
fn tc_ir_4_5_1_4_script_sets_image_asset() {
    let mut h = ScriptingUiHarness::new();
    let w = h.spawn_widget();
    h.set_widget_state.push_back(set_cmd(
        w,
        WidgetStateKind::ImageAsset,
        ScriptValue::Image(AssetId(7)),
    ));
    h.apply_set_widget_states();
    assert_eq!(h.widgets[&w].image, AssetId(7));
}

#[test]
fn tc_ir_4_5_1_n1_script_panic_caught_fm4() {
    let mut h = ScriptingUiHarness::new();
    let w = h.spawn_widget();
    h.emit_click(w);
    let mut handlers: Vec<Box<WidgetEventHandler>> = vec![Box::new(|_, _| {
        panic!("simulated script panic");
    })];
    h.drain_widget_events_to_script(&mut handlers);
    assert_eq!(h.counters.fm4, 1);
    h.emit_click(w);
    h.drain_widget_events_to_script(&mut handlers);
    assert_eq!(h.counters.fm4, 1);
}

// --- IR-4.5.2 WidgetEvent -----------------------------------------------------

#[test]
fn tc_ir_4_5_2_1_click_fires_widget_event_clicked() {
    let mut h = ScriptingUiHarness::new();
    let w = h.spawn_widget();
    h.emit_click(w);
    let e = h.widget_events.pop_front().expect("event");
    assert_eq!(e.target, w);
    assert_eq!(e.kind, WidgetEventKind::Clicked);
}

#[test]
fn tc_ir_4_5_2_2_script_handler_runs_on_click_event() {
    let mut h = ScriptingUiHarness::new();
    let w = h.spawn_widget();
    let fired = Rc::new(Cell::new(false));
    let fired2 = fired.clone();
    h.emit_click(w);
    let mut handlers: Vec<Box<WidgetEventHandler>> = vec![Box::new(move |evt, harness| {
        if evt.target == w {
            harness.set_widget_state.push_back(set_cmd(
                w,
                WidgetStateKind::Text,
                ScriptValue::Text("Done".into()),
            ));
            fired2.set(true);
        }
    })];
    h.drain_widget_events_to_script(&mut handlers);
    assert!(fired.get());
    h.apply_set_widget_states();
    assert_eq!(h.widgets[&w].text, "Done");
}

#[test]
fn tc_ir_4_5_2_3_value_changed_carries_new_value() {
    let mut h = ScriptingUiHarness::new();
    let w = h.spawn_widget();
    h.emit_widget_event(w, WidgetEventKind::ValueChanged { new: 0.75 });
    let e = h.widget_events.pop_front().expect("event");
    assert_eq!(e.kind, WidgetEventKind::ValueChanged { new: 0.75 });
}

#[test]
fn tc_ir_4_5_2_n1_event_to_despawned_widget_fm1() {
    let mut h = ScriptingUiHarness::new();
    let w = h.spawn_widget();
    h.emit_click(w);
    h.despawn(w);
    let mut handlers: Vec<Box<WidgetEventHandler>> =
        vec![Box::new(|_, _| unreachable!("handler should not run"))];
    h.drain_widget_events_to_script(&mut handlers);
    assert_eq!(h.counters.fm1, 1);
}

// --- IR-4.5.3 DataBinding -----------------------------------------------------

#[test]
fn tc_ir_4_5_3_1_component_binding_resolves_at_layout() {
    let mut h = ScriptingUiHarness::new();
    let w = h.spawn_widget();
    h.health.insert(w, 55);
    let b = DataBinding {
        source: BindingSource::EntityComponent,
        path: "player.hp".into(),
        mode: BindingUpdateMode::EveryFrame,
        path_expr: BoundExpr::literal(),
    };
    let t = h.resolve_binding_text(w, &b, true);
    assert_eq!(t, "55");
}

#[test]
fn tc_ir_4_5_3_2_resource_binding_resolves_at_layout() {
    let mut h = ScriptingUiHarness::new();
    let w = h.spawn_widget();
    h.frame_index = 99;
    let b = DataBinding {
        source: BindingSource::Resource,
        path: "game_time.tick".into(),
        mode: BindingUpdateMode::EveryFrame,
        path_expr: BoundExpr::literal(),
    };
    let t = h.resolve_binding_text(w, &b, true);
    assert_eq!(t, "99");
}

#[test]
fn tc_ir_4_5_3_3_data_table_binding_resolves_at_layout() {
    let mut h = ScriptingUiHarness::new();
    let w = h.spawn_widget();
    h.data_table.insert(
        ("weapon.sword".into(), "damage".into()),
        "12".into(),
    );
    let b = DataBinding {
        source: BindingSource::DataTableRow,
        path: "weapon.sword.damage".into(),
        mode: BindingUpdateMode::EveryFrame,
        path_expr: BoundExpr::literal(),
    };
    let t = h.resolve_binding_text(w, &b, true);
    assert_eq!(t, "12");
}

#[test]
fn tc_ir_4_5_3_4_on_change_mode_skips_unchanged() {
    let mut h = ScriptingUiHarness::new();
    let w = h.spawn_widget();
    h.health.insert(w, 10);
    let b = DataBinding {
        source: BindingSource::EntityComponent,
        path: "player.hp".into(),
        mode: BindingUpdateMode::OnChange,
        path_expr: BoundExpr::literal(),
    };
    assert_eq!(h.resolve_binding_text(w, &b, true), "10");
    for _ in 0..59 {
        h.tick_frame();
        assert_eq!(h.resolve_binding_text(w, &b, true), "10");
    }
    assert_eq!(h.binding_eval_calls, 1);
    assert_eq!(h.binding_eval_skips, 59);
}

#[test]
fn tc_ir_4_5_3_n1_binding_type_mismatch_fm2() {
    let mut h = ScriptingUiHarness::new();
    let w = h.spawn_widget();
    h.health.insert(w, 42);
    let b = DataBinding {
        source: BindingSource::EntityComponent,
        path: "binding.u32_only".into(),
        mode: BindingUpdateMode::EveryFrame,
        path_expr: BoundExpr::literal(),
    };
    let t = h.resolve_binding_text(w, &b, true);
    assert_eq!(t, "<err>");
    assert_eq!(h.counters.fm2, 1);
}

#[test]
fn tc_ir_4_5_3_n2_binding_source_not_found_fm6() {
    let mut h = ScriptingUiHarness::new();
    let w = h.spawn_widget();
    let b = DataBinding {
        source: BindingSource::Resource,
        path: "no.such.resource".into(),
        mode: BindingUpdateMode::Once,
        path_expr: BoundExpr::literal(),
    };
    let t = h.resolve_binding_text(w, &b, true);
    assert_eq!(t, "<missing>");
    assert_eq!(h.counters.fm6, 1);
}

// --- IR-4.5.4 DialogueChoice --------------------------------------------------

#[test]
fn tc_ir_4_5_4_1_dialogue_choice_flows_to_quest() {
    let mut h = ScriptingUiHarness::new();
    let dlg = h.spawn_widget();
    h.open_dialogue(dlg);
    let choice = DialogueChoice {
        dialogue: dlg,
        choice_index: 2,
        label: LocalizedStringId(1),
    };
    h.enqueue_dialogue_choice(choice.clone());
    assert_eq!(h.drain_one_dialogue_choice(), Some(choice));
}

#[test]
fn tc_ir_4_5_4_2_choice_index_encoded_zero_based() {
    let mut h = ScriptingUiHarness::new();
    let dlg = h.spawn_widget();
    h.open_dialogue(dlg);
    h.enqueue_dialogue_choice(DialogueChoice {
        dialogue: dlg,
        choice_index: 2,
        label: LocalizedStringId(0),
    });
    let got = h.drain_one_dialogue_choice().expect("choice");
    assert_eq!(got.choice_index, 2);
}

#[test]
fn tc_ir_4_5_4_n1_ch9_full_backpressure_fm3() {
    let mut h = ScriptingUiHarness::new();
    let dlg = h.spawn_widget();
    h.open_dialogue(dlg);
    for i in 0..32 {
        h.enqueue_dialogue_choice(DialogueChoice {
            dialogue: dlg,
            choice_index: i,
            label: LocalizedStringId(i),
        });
    }
    assert!(h.counters.fm3 >= 1);
    let mut drained = 0u32;
    while h.drain_one_dialogue_choice().is_some() {
        drained += 1;
    }
    assert_eq!(drained, 32);
}

#[test]
fn tc_ir_4_5_4_n2_orphan_dialogue_choice_fm7() {
    let mut h = ScriptingUiHarness::new();
    let dlg = h.spawn_widget();
    h.open_dialogue(dlg);
    h.close_dialogue(dlg);
    h.enqueue_dialogue_choice(DialogueChoice {
        dialogue: dlg,
        choice_index: 0,
        label: LocalizedStringId(0),
    });
    assert_eq!(h.counters.fm7, 1);
}

// --- IR-4.5.5 FormattedString -------------------------------------------------

#[test]
fn tc_ir_4_5_5_1_formatted_string_args_pass_through() {
    let fs = FormattedString {
        id: LocalizedStringId(9),
        args: vec![("n".into(), ArgValue::I64(3))],
    };
    assert_eq!(ScriptingUiHarness::resolve_formatted_plural_items(&fs), "3 items");
}

// --- IR-4.5.6 Visibility --------------------------------------------------------

#[test]
fn tc_ir_4_5_6_1_visible_expr_false_hides_widget() {
    let mut h = ScriptingUiHarness::new();
    let w = h.spawn_widget();
    let ev = h.visibility_eval(w, |_| Ok(false), |_| Ok(true));
    h.apply_visibility(ev);
    assert!(!h.widgets[&w].visible);
}

#[test]
fn tc_ir_4_5_6_2_enabled_expr_false_disables_widget() {
    let mut h = ScriptingUiHarness::new();
    let w = h.spawn_widget();
    let ev = h.visibility_eval(w, |_| Ok(true), |_| Ok(false));
    h.apply_visibility(ev);
    assert!(h.widgets[&w].interaction_disabled);
    h.emit_click(w);
    assert!(h.widget_events.is_empty());
}

#[test]
fn tc_ir_4_5_6_3_hidden_widgets_skipped_in_layout_count() {
    let mut h = ScriptingUiHarness::new();
    let mut ids = Vec::new();
    for _ in 0..20 {
        ids.push(h.spawn_widget());
    }
    for w in ids.iter().take(10) {
        let ev = h.visibility_eval(*w, |_| Ok(false), |_| Ok(true));
        h.apply_visibility(ev);
    }
    assert_eq!(h.layout_visible_count(), 10);
}

#[test]
fn tc_ir_4_5_6_n1_missing_expr_dep_default_visible_fm5() {
    let mut h = ScriptingUiHarness::new();
    let w = h.spawn_widget();
    let ev = h.visibility_eval(w, |_| Err(()), |_| Ok(true));
    assert!(ev.visible);
    assert_eq!(h.counters.fm5, 1);
}

// --- Benchmark-style smoke (generous ceilings for CI variance) ---------------

#[test]
fn tc_ir_4_5_1_b1_apply_50_set_widget_state_under_budget() {
    let mut h = ScriptingUiHarness::new();
    let w = h.spawn_widget();
    let start = Instant::now();
    for _ in 0..50 {
        h.set_widget_state.push_back(set_cmd(
            w,
            WidgetStateKind::Text,
            ScriptValue::Text("x".into()),
        ));
    }
    h.apply_set_widget_states();
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs_f64() < 0.05,
        "50 applies took {:?}",
        elapsed
    );
}

#[test]
fn tc_ir_4_5_3_b1_resolve_500_bindings_under_budget() {
    let mut h = ScriptingUiHarness::new();
    let w = h.spawn_widget();
    h.health.insert(w, 7);
    let b = DataBinding {
        source: BindingSource::EntityComponent,
        path: "player.hp".into(),
        mode: BindingUpdateMode::EveryFrame,
        path_expr: BoundExpr::literal(),
    };
    let start = Instant::now();
    for _ in 0..500 {
        let _ = h.resolve_binding_text(w, &b, true);
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs_f64() < 0.1,
        "500 resolves took {:?}",
        elapsed
    );
}

#[test]
fn tc_ir_4_5_6_b1_visibility_eval_500_widgets_under_budget() {
    let mut h = ScriptingUiHarness::new();
    let mut ids = Vec::new();
    for _ in 0..500 {
        ids.push(h.spawn_widget());
    }
    let start = Instant::now();
    for w in &ids {
        let ev = h.visibility_eval(*w, |_| Ok(true), |_| Ok(true));
        h.apply_visibility(ev);
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs_f64() < 0.05,
        "500 visibility evals took {:?}",
        elapsed
    );
}

#[test]
fn tc_ir_4_5_2_b1_dispatch_100_widget_events_under_budget() {
    let mut h = ScriptingUiHarness::new();
    let w = h.spawn_widget();
    let mut handlers: Vec<Box<WidgetEventHandler>> = vec![Box::new(|_, _| {})];
    let start = Instant::now();
    for _ in 0..100 {
        h.emit_click(w);
        h.drain_widget_events_to_script(&mut handlers);
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs_f64() < 0.05,
        "100 click dispatch cycles took {:?}",
        elapsed
    );
}
