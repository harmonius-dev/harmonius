//! Integration tests mapped from `docs/design/integration/localization-ui-test-cases.md`.

use integration_localization_ui::{
    icon_button_layout, layout_line, shape_line, split_mixed_runs, ArgValue, AssetId, FontChain,
    FontChainResolver, Gender, ImeEvent, ImeEventKind, LayoutLocaleBridge, LocaleChangeChannel,
    LocaleChangeEvent, LocalizedStringId, LocalizationTable, MessageTemplate, ResolveArgs,
    ResolvedText, ScriptTag, TextDirection, TextInputState, WidgetForest, WidgetShell,
};

fn fr_locale() -> integration_localization_ui::LocaleId {
    integration_localization_ui::LocaleId::from_ascii("fr")
}

fn en_locale() -> integration_localization_ui::LocaleId {
    integration_localization_ui::LocaleId::from_ascii("en")
}

fn de_locale() -> integration_localization_ui::LocaleId {
    integration_localization_ui::LocaleId::from_ascii("de")
}

fn ar_locale() -> integration_localization_ui::LocaleId {
    integration_localization_ui::LocaleId::from_ascii("ar")
}

#[test]
fn tc_ir_4_4_1_1_resolved_text_matches_locale_entry() {
    let mut table = LocalizationTable::new(en_locale());
    table.insert_template(
        fr_locale(),
        LocalizedStringId(7),
        MessageTemplate {
            pattern: "Bonjour".into(),
            direction: TextDirection::Ltr,
            script: ScriptTag::LATN,
        },
    );
    let resolved = table.resolve(LocalizedStringId(7), fr_locale(), &ResolveArgs::new());
    let resolver = FontChainResolver;
    assert_eq!(
        resolved,
        ResolvedText {
            display: "Bonjour".into(),
            direction: TextDirection::Ltr,
            script: ScriptTag::LATN,
            fonts: resolver.resolve(ScriptTag::LATN),
        }
    );
}

#[test]
fn tc_ir_4_4_1_2_fallback_locale_on_missing_key() {
    let mut table = LocalizationTable::new(en_locale());
    table.insert_template(
        en_locale(),
        LocalizedStringId(7),
        MessageTemplate {
            pattern: "Hello".into(),
            direction: TextDirection::Ltr,
            script: ScriptTag::LATN,
        },
    );
    let resolved = table.resolve(LocalizedStringId(7), de_locale(), &ResolveArgs::new());
    assert_eq!(resolved.display, "Hello");
    assert_eq!(table.counters.fm1, 1);
}

#[test]
fn tc_ir_4_4_1_3_missing_in_both_locales() {
    let mut table = LocalizationTable::new(en_locale());
    let resolved = table.resolve(LocalizedStringId(42), en_locale(), &ResolveArgs::new());
    assert_eq!(resolved.display, "[missing:42]");
    assert_eq!(table.counters.fm2, 1);
}

#[test]
fn tc_ir_4_4_2_1_rtl_alignment_places_run_at_right_edge() {
    let layout = layout_line("مرحبا", TextDirection::Rtl, 200.0);
    let adv = layout.runs[0].advance;
    assert!((layout.runs[0].start_x - (200.0 - adv)).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_4_4_2_2_mixed_bidi_runs_split() {
    let runs = split_mixed_runs("Hello עברית");
    assert_eq!(runs.len(), 2);
    assert_eq!(runs[0].0, TextDirection::Ltr);
    assert_eq!(runs[1].0, TextDirection::Rtl);
}

#[test]
fn tc_ir_4_4_2_3_icon_order_flips_in_rtl() {
    let (icon, label) = icon_button_layout(TextDirection::Rtl, 0.0, 100.0, 8.0);
    assert_eq!(icon, 100.0);
    assert_eq!(label, 8.0);
}

#[test]
fn tc_ir_4_4_3_1_script_selects_font_chain() {
    let chain = FontChainResolver.resolve(ScriptTag::HANI);
    assert_eq!(chain.fonts.first().copied(), Some(AssetId(10)));
}

#[test]
fn tc_ir_4_4_3_2_missing_glyph_falls_through_chain() {
    let chain = FontChain::new(vec![AssetId(1), AssetId(2)]);
    let support = [('x', AssetId(2))];
    let (run, fm3) = shape_line("x", &chain, &support);
    assert_eq!(run.glyphs[0].font, AssetId(2));
    assert_eq!(fm3, 0);
}

#[test]
fn tc_ir_4_4_4_1_ime_start_shows_composition() {
    let mut input = TextInputState::new();
    let mut fm4 = 0;
    input.apply_ime(
        &ImeEvent {
            kind: ImeEventKind::Start,
            text: "か".into(),
            selection_start: 0,
            selection_end: 0,
        },
        true,
        &mut fm4,
    );
    assert_eq!(input.composition.as_deref(), Some("か"));
    assert!(input.composition_underline);
    assert_eq!(fm4, 0);
}

#[test]
fn tc_ir_4_4_4_2_ime_update_replaces_composition() {
    let mut input = TextInputState::new();
    let mut fm4 = 0;
    input.apply_ime(
        &ImeEvent {
            kind: ImeEventKind::Start,
            text: "か".into(),
            selection_start: 0,
            selection_end: 0,
        },
        true,
        &mut fm4,
    );
    input.apply_ime(
        &ImeEvent {
            kind: ImeEventKind::Update,
            text: "かん".into(),
            selection_start: 0,
            selection_end: 0,
        },
        true,
        &mut fm4,
    );
    assert_eq!(input.composition.as_deref(), Some("かん"));
}

#[test]
fn tc_ir_4_4_4_3_ime_commit_appends_text() {
    let mut input = TextInputState::new();
    let mut fm4 = 0;
    input.apply_ime(
        &ImeEvent {
            kind: ImeEventKind::Start,
            text: "か".into(),
            selection_start: 0,
            selection_end: 0,
        },
        true,
        &mut fm4,
    );
    input.apply_ime(
        &ImeEvent {
            kind: ImeEventKind::Commit,
            text: "感".into(),
            selection_start: 0,
            selection_end: 0,
        },
        true,
        &mut fm4,
    );
    assert_eq!(input.text, "感");
    assert!(input.composition.is_none());
}

#[test]
fn tc_ir_4_4_4_4_ime_cancel_clears_composition() {
    let mut input = TextInputState::new();
    let mut fm4 = 0;
    input.apply_ime(
        &ImeEvent {
            kind: ImeEventKind::Start,
            text: "か".into(),
            selection_start: 0,
            selection_end: 0,
        },
        true,
        &mut fm4,
    );
    input.apply_ime(
        &ImeEvent {
            kind: ImeEventKind::Cancel,
            text: String::new(),
            selection_start: 0,
            selection_end: 0,
        },
        true,
        &mut fm4,
    );
    assert!(input.composition.is_none());
    assert_eq!(input.text, "");
}

#[test]
fn tc_ir_4_4_5_1_locale_change_marks_widgets_dirty() {
    let mut forest = WidgetForest {
        widgets: vec![
            WidgetShell {
                localized: Some(LocalizedStringId(1)),
                layout_dirty: false,
            },
            WidgetShell {
                localized: None,
                layout_dirty: false,
            },
        ],
    };
    forest.on_locale_change();
    assert!(forest.widgets[0].layout_dirty);
    assert!(!forest.widgets[1].layout_dirty);
}

#[test]
fn tc_ir_4_4_5_2_locale_change_reselects_font_chain_for_arabic_script() {
    let mut table = LocalizationTable::new(en_locale());
    table.insert_template(
        en_locale(),
        LocalizedStringId(1),
        MessageTemplate {
            pattern: "Hi".into(),
            direction: TextDirection::Ltr,
            script: ScriptTag::LATN,
        },
    );
    table.insert_template(
        ar_locale(),
        LocalizedStringId(1),
        MessageTemplate {
            pattern: "مرحبا".into(),
            direction: TextDirection::Rtl,
            script: ScriptTag::ARAB,
        },
    );
    let en_res = table.resolve(LocalizedStringId(1), en_locale(), &ResolveArgs::new());
    let ar_res = table.resolve(LocalizedStringId(1), ar_locale(), &ResolveArgs::new());
    assert_eq!(en_res.fonts.fonts.first().copied(), Some(AssetId(1)));
    assert_eq!(ar_res.fonts.fonts.first().copied(), Some(AssetId(20)));
}

#[test]
fn tc_ir_4_4_5_3_locale_change_propagates_over_channel() {
    let mut ch = LocaleChangeChannel::new(16);
    let event = LocaleChangeEvent {
        previous: en_locale(),
        next: fr_locale(),
    };
    ch.send(event);
    let drained = ch.drain();
    assert_eq!(
        drained,
        vec![event],
        "CH-29 delivers LocaleChangeEvent for UI and audio consumers"
    );
}

#[test]
fn tc_ir_4_4_6_1_plural_arg_selects_form() {
    let mut table = LocalizationTable::new(en_locale());
    table.insert_template(
        en_locale(),
        LocalizedStringId(9),
        MessageTemplate {
            pattern: "{n, plural, one{# item} other{# items}}".into(),
            direction: TextDirection::Ltr,
            script: ScriptTag::LATN,
        },
    );
    let mut args = ResolveArgs::new();
    args.insert("n", ArgValue::Int(1));
    let resolved = table.resolve(LocalizedStringId(9), en_locale(), &args);
    assert_eq!(resolved.display, "1 item");
}

#[test]
fn tc_ir_4_4_6_2_gender_arg_selects_form() {
    let mut table = LocalizationTable::new(en_locale());
    table.insert_template(
        en_locale(),
        LocalizedStringId(10),
        MessageTemplate {
            pattern: "{g, gender, male{he} female{she} other{they}}".into(),
            direction: TextDirection::Ltr,
            script: ScriptTag::LATN,
        },
    );
    let mut args = ResolveArgs::new();
    args.insert("g", ArgValue::Gender(Gender::Female));
    let resolved = table.resolve(LocalizedStringId(10), en_locale(), &args);
    assert_eq!(resolved.display, "she");
}

#[test]
fn tc_ir_4_4_6_3_numeric_arg_formatted_per_locale() {
    let mut table = LocalizationTable::new(en_locale());
    table.insert_template(
        fr_locale(),
        LocalizedStringId(11),
        MessageTemplate {
            pattern: "{n, number}".into(),
            direction: TextDirection::Ltr,
            script: ScriptTag::LATN,
        },
    );
    let mut args = ResolveArgs::new();
    args.insert("n", ArgValue::Float(1234.5));
    let resolved = table.resolve(LocalizedStringId(11), fr_locale(), &args);
    assert_eq!(resolved.display, "1 234,5");
}

#[test]
fn tc_ir_4_4_4_n1_commit_after_focus_lost() {
    let mut input = TextInputState::new();
    let mut fm4 = 0;
    input.apply_ime(
        &ImeEvent {
            kind: ImeEventKind::Start,
            text: "か".into(),
            selection_start: 0,
            selection_end: 0,
        },
        true,
        &mut fm4,
    );
    input.apply_ime(
        &ImeEvent {
            kind: ImeEventKind::Commit,
            text: "感".into(),
            selection_start: 0,
            selection_end: 0,
        },
        false,
        &mut fm4,
    );
    assert_eq!(input.text, "");
    assert_eq!(fm4, 1);
}

#[test]
fn tc_ir_4_4_5_n1_locale_change_mid_layout_defers() {
    let mut counters = integration_localization_ui::FallbackCounters::default();
    let mut bridge = LayoutLocaleBridge::new(en_locale());
    bridge.begin_layout();
    bridge.request_locale(fr_locale(), &mut counters);
    assert_eq!(bridge.current(), en_locale());
    assert_eq!(counters.fm7, 1);
    bridge.end_layout();
    assert_eq!(bridge.current(), fr_locale());
}

#[test]
fn tc_ir_4_4_6_n1_missing_format_arg() {
    let mut table = LocalizationTable::new(en_locale());
    table.insert_template(
        en_locale(),
        LocalizedStringId(12),
        MessageTemplate {
            pattern: "Hello {name}".into(),
            direction: TextDirection::Ltr,
            script: ScriptTag::LATN,
        },
    );
    let resolved = table.resolve(LocalizedStringId(12), en_locale(), &ResolveArgs::new());
    assert_eq!(resolved.display, "Hello {missing:name}");
    assert_eq!(table.counters.fm6, 1);
}

#[test]
fn tc_ir_4_4_3_n1_empty_font_chain_triggers_notdef() {
    let chain = FontChain::empty();
    let support: [(char, AssetId); 0] = [];
    let (_run, fm3) = shape_line("a", &chain, &support);
    assert_eq!(fm3, 1);
}
