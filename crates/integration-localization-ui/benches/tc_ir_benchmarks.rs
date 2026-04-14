//! TC-IR-4.4.*.B1 timing checks from `docs/design/integration/localization-ui-test-cases.md`.
//!
//! Run with `cargo bench -p integration-localization-ui --bench tc_ir_benchmarks`.

use integration_localization_ui::{
    ArgValue, FontChainResolver, LocalizedStringId, LocalizationTable, MessageTemplate, ResolveArgs,
    ScriptTag, TextDirection, WidgetForest, WidgetShell,
};
use std::hint::black_box;
use std::thread;
use std::time::{Duration, Instant};

fn en_locale() -> integration_localization_ui::LocaleId {
    integration_localization_ui::LocaleId::from_ascii("en")
}

fn assert_under(label: &'static str, elapsed: Duration, cap: Duration) {
    assert!(
        elapsed <= cap,
        "{label}: took {elapsed:?}, cap {cap:?}"
    );
}

fn bench_resolve_500_strings() {
    let mut table = LocalizationTable::new(en_locale());
    for i in 0..500 {
        table.insert_template(
            en_locale(),
            LocalizedStringId(i),
            MessageTemplate {
                pattern: format!("msg {i}"),
                direction: TextDirection::Ltr,
                script: ScriptTag::LATN,
            },
        );
    }
    let args = ResolveArgs::new();
    thread::yield_now();
    let start = Instant::now();
    for i in 0..500 {
        black_box(table.resolve(LocalizedStringId(i), en_locale(), &args));
    }
    let elapsed = start.elapsed();
    assert_under("TC-IR-4.4.1.B1", elapsed, Duration::from_micros(100));
}

fn bench_font_chain_select_500() {
    let resolver = FontChainResolver;
    thread::yield_now();
    let start = Instant::now();
    for _ in 0..500 {
        black_box(resolver.resolve(ScriptTag::HANI));
    }
    let elapsed = start.elapsed();
    assert_under("TC-IR-4.4.3.B1", elapsed, Duration::from_micros(50));
}

fn bench_locale_switch_500_widgets() {
    let widgets: Vec<_> = (0..500)
        .map(|i| WidgetShell {
            localized: Some(LocalizedStringId(i)),
            layout_dirty: false,
        })
        .collect();
    let mut forest = WidgetForest { widgets };
    thread::yield_now();
    let start = Instant::now();
    forest.on_locale_change();
    black_box(&forest);
    let elapsed = start.elapsed();
    assert_under("TC-IR-4.4.5.B1", elapsed, Duration::from_millis(2));
}

fn bench_format_100_pluralized_strings() {
    let mut table = LocalizationTable::new(en_locale());
    table.insert_template(
        en_locale(),
        LocalizedStringId(1),
        MessageTemplate {
            pattern: "{n, plural, one{# item} other{# items}}".into(),
            direction: TextDirection::Ltr,
            script: ScriptTag::LATN,
        },
    );
    let mut args = ResolveArgs::new();
    args.insert("n", ArgValue::Int(2));
    thread::yield_now();
    let start = Instant::now();
    for _ in 0..100 {
        black_box(table.resolve(LocalizedStringId(1), en_locale(), &args));
    }
    let elapsed = start.elapsed();
    assert_under("TC-IR-4.4.6.B1", elapsed, Duration::from_micros(100));
}

fn main() {
    bench_resolve_500_strings();
    bench_font_chain_select_500();
    bench_locale_switch_500_widgets();
    bench_format_100_pluralized_strings();
    println!("tc_ir_benchmarks: all B1 thresholds passed");
}
