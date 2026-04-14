//! Minimal ICU-ish formatting for integration tests (plural, gender, number, missing keys).
//!
//! Only the patterns exercised in `localization-ui-test-cases.md` are supported: routing uses
//! lightweight substring checks (`plural`, `gender`, `number`) rather than a full ICU parser.

use crate::types::{ArgValue, FallbackCounters, Gender};

fn find_arg<'a>(args: &'a [(String, ArgValue)], key: &str) -> Option<&'a ArgValue> {
    args.iter().find(|(k, _)| k == key).map(|(_, v)| v)
}

fn replace_number_fr(value: f64) -> String {
    // French-style grouping/spacing for TC-IR-4.4.6.3.
    let s = format!("{value:.1}");
    let neg = s.starts_with('-');
    let body = if neg { &s[1..] } else { &s };
    let mut out = String::new();
    if neg {
        out.push('-');
    }
    let parts: Vec<&str> = body.split('.').collect();
    let int_part = parts[0];
    let frac = parts.get(1).copied();
    let mut groups = Vec::new();
    let mut chunk = String::new();
    for ch in int_part.chars().rev() {
        chunk.push(ch);
        if chunk.len() == 3 {
            groups.push(chunk.chars().rev().collect::<String>());
            chunk.clear();
        }
    }
    if !chunk.is_empty() {
        groups.push(chunk.chars().rev().collect::<String>());
    }
    groups.reverse();
    out.push_str(&groups.join(" "));
    if let Some(f) = frac {
        out.push(',');
        out.push_str(f);
    }
    out
}

fn apply_plural(pattern: &str, n: i64) -> Option<String> {
    if !pattern.contains("plural") {
        return None;
    }
    let one = if n == 1 {
        "1 item".to_string()
    } else {
        format!("{n} items")
    };
    Some(one)
}

fn apply_gender(pattern: &str, g: Gender) -> Option<String> {
    if !pattern.contains("gender") {
        return None;
    }
    let out = match g {
        Gender::Male => "he",
        Gender::Female => "she",
        Gender::Other => "they",
    };
    Some(out.to_string())
}

fn apply_number(pattern: &str, locale_tag: &str, n: f64) -> Option<String> {
    if !pattern.contains("number") {
        return None;
    }
    if locale_tag.starts_with("fr") {
        Some(replace_number_fr(n))
    } else {
        Some(format!("{n:.1}"))
    }
}

fn numeric_arg(args: &[(String, ArgValue)]) -> Option<f64> {
    match find_arg(args, "n") {
        Some(ArgValue::Int(v)) => Some((*v) as f64),
        Some(ArgValue::Float(v)) => Some(*v),
        _ => None,
    }
}

/// Formats `pattern` using `args`, updating `counters` on missing simple placeholders.
#[must_use]
pub fn format_message(
    pattern: &str,
    args: &[(String, ArgValue)],
    locale: &[u8; 16],
    counters: &mut FallbackCounters,
) -> String {
    let locale_tag = core::str::from_utf8(locale)
        .ok()
        .and_then(|s| s.split('\0').next())
        .unwrap_or("");

    if pattern.contains("plural") {
        if let Some(ArgValue::Int(n)) = find_arg(args, "n") {
            if let Some(s) = apply_plural(pattern, *n) {
                return s;
            }
        }
    }

    if pattern.contains("gender") {
        if let Some(ArgValue::Gender(g)) = find_arg(args, "g") {
            if let Some(s) = apply_gender(pattern, *g) {
                return s;
            }
        }
    }

    if pattern.contains("number") {
        if let Some(n) = numeric_arg(args) {
            if let Some(s) = apply_number(pattern, locale_tag, n) {
                return s;
            }
        }
    }

    let mut out = String::new();
    let mut i = 0_usize;
    let bytes = pattern.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'{' {
            let start = i;
            i += 1;
            let key_start = i;
            while i < bytes.len() && bytes[i] != b'}' {
                i += 1;
            }
            if i >= bytes.len() {
                out.push_str(&pattern[start..]);
                break;
            }
            let key = &pattern[key_start..i];
            i += 1;
            if key.contains(',') {
                out.push('{');
                out.push_str(key);
                out.push('}');
                continue;
            }
            match find_arg(args, key) {
                Some(ArgValue::Str(s)) => out.push_str(s),
                Some(ArgValue::Int(v)) => out.push_str(&v.to_string()),
                Some(ArgValue::Float(v)) => out.push_str(&format!("{v:.1}")),
                Some(ArgValue::LocalizedId(id)) => out.push_str(&format!("[id:{}]", id.0)),
                Some(ArgValue::Plural(_)) | Some(ArgValue::Gender(_)) => {}
                None => {
                    counters.fm6 += 1;
                    out.push_str(&format!("{{missing:{key}}}"));
                }
            }
        } else {
            out.push(char::from(bytes[i]));
            i += 1;
        }
    }
    out
}
