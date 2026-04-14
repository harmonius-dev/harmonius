//! Minimal XLIFF 1.2 encode/decode for authoring round-trips.

use std::collections::HashMap;

use super::string_table::{StringEntry, StringKey, StringTableModel, TransUnitState};

/// Counts applied during merge imports.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct XliffMergeStats {
    /// Rows updated from existing keys.
    pub updated: u32,
    /// Rows inserted as new keys.
    pub inserted: u32,
    /// Rows removed (unused in current tests).
    pub lost: u32,
}

/// Serializes the full table into a deterministic XLIFF fragment.
pub fn encode_xliff(model: &StringTableModel) -> String {
    let mut rows: Vec<_> = model
        .rows()
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    rows.sort_by(|a, b| a.0.0.cmp(&b.0.0));
    let mut out = String::from("<?xml version=\"1.0\"?><xliff><file><body>");
    for (key, entry) in rows {
        let id = xml_escape(&key.0);
        let src = xml_escape(&entry.source);
        let tgt = entry
            .translations
            .get("fr")
            .map(String::as_str)
            .unwrap_or("");
        let trg = xml_escape(tgt);
        out.push_str("<trans-unit id=\"");
        out.push_str(&id);
        out.push_str("\"><source>");
        out.push_str(&src);
        out.push_str("</source><target>");
        out.push_str(&trg);
        out.push_str("</target></trans-unit>");
    }
    out.push_str("</body></file></xliff>");
    out
}

/// Parses XLIFF produced by `encode_xliff` back into a model.
pub fn decode_xliff(data: &str) -> StringTableModel {
    let mut model = StringTableModel::new();
    for (id, source, target) in parse_units(data) {
        let mut translations = HashMap::new();
        if !target.is_empty() {
            translations.insert("fr".to_string(), target);
        }
        let state = if translations.is_empty() {
            TransUnitState::Initial
        } else {
            TransUnitState::Translated
        };
        model.insert_entry(
            StringKey(id),
            StringEntry {
                source,
                translations,
                state,
                locked: false,
            },
        );
    }
    model
}

/// Merges translated units from `fragment` into `base`, counting updates vs inserts.
pub fn merge_xliff(base: &mut StringTableModel, fragment: &str) -> XliffMergeStats {
    let mut updated = 0u32;
    let mut inserted = 0u32;
    for (id, source, target) in parse_units(fragment) {
        let key = StringKey(id.clone());
        if base.rows().contains_key(&key) {
            if let Some(entry) = base.rows_mut().get_mut(&key) {
                if !target.is_empty() {
                    entry.translations.insert("fr".to_string(), target);
                    entry.state = TransUnitState::Translated;
                    updated += 1;
                }
            }
        } else {
            let mut translations = HashMap::new();
            if !target.is_empty() {
                translations.insert("fr".to_string(), target);
            }
            base.insert_entry(
                key,
                StringEntry {
                    source,
                    translations,
                    state: TransUnitState::Translated,
                    locked: false,
                },
            );
            inserted += 1;
        }
    }
    XliffMergeStats {
        updated,
        inserted,
        lost: 0,
    }
}

/// Sets `TransUnitState::Initial` when the imported source disagrees with the row.
pub fn reimport_reset_on_source_drift(base: &mut StringTableModel, fragment: &str) {
    for (id, source, _target) in parse_units(fragment) {
        let key = StringKey(id);
        if let Some(entry) = base.rows_mut().get_mut(&key) {
            if entry.source != source {
                entry.state = TransUnitState::Initial;
            }
        }
    }
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\"', "&quot;")
}

fn parse_units(data: &str) -> Vec<(String, String, String)> {
    let mut out = Vec::new();
    for segment in data.split("<trans-unit") {
        if !segment.contains("id=\"") {
            continue;
        }
        let id = extract_between(segment, "id=\"", "\"");
        let source = extract_between(segment, "<source>", "</source>");
        let target = extract_between(segment, "<target>", "</target>");
        out.push((xml_unescape(&id), xml_unescape(&source), xml_unescape(&target)));
    }
    out
}

fn extract_between(hay: &str, start: &str, end: &str) -> String {
    let i = hay.find(start).map(|i| i + start.len()).unwrap_or(0);
    let rest = &hay[i..];
    let j = rest.find(end).unwrap_or(0);
    rest[..j].to_string()
}

fn xml_unescape(s: &str) -> String {
    s.replace("&quot;", "\"")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

#[cfg(test)]
mod tests {
    use super::super::string_table::{StringEntry, StringKey, StringTableModel, TransUnitState};
    use super::*;
    use std::collections::HashMap;

    fn sample_table(n: u32) -> StringTableModel {
        let mut model = StringTableModel::new();
        for i in 0..n {
            let mut tr = HashMap::new();
            tr.insert("fr".to_string(), format!("t{i}"));
            model.insert_entry(
                StringKey(format!("k{i}")),
                StringEntry {
                    source: format!("s{i}"),
                    translations: tr,
                    state: TransUnitState::Translated,
                    locked: false,
                },
            );
        }
        model
    }

    /// TC-15.13.3.1 — XLIFF encode/decode round-trip preserves rows.
    #[test]
    fn tc_15_13_3_1_xliff_encode_round_trip() {
        let original = sample_table(10);
        let xml = encode_xliff(&original);
        let decoded = decode_xliff(&xml);
        assert_eq!(decoded.rows().len(), 10);
        for i in 0..10 {
            let key = StringKey(format!("k{i}"));
            assert_eq!(decoded.get(&key), original.get(&key));
        }
    }

    /// TC-15.13.3.2 — merge counts updates vs inserts without losing rows.
    #[test]
    fn tc_15_13_3_2_xliff_import_merge() {
        let mut base = StringTableModel::new();
        for id in ["k1", "k2", "k3", "k4"] {
            base.insert_entry(
                StringKey(id.to_string()),
                StringEntry {
                    source: format!("src_{id}"),
                    translations: HashMap::new(),
                    state: TransUnitState::Initial,
                    locked: false,
                },
            );
        }
        let fragment = "<?xml version=\"1.0\"?><xliff><file><body>\
            <trans-unit id=\"k1\"><source>src_k1</source><target>fr1</target></trans-unit>\
            <trans-unit id=\"k2\"><source>src_k2</source><target>fr2</target></trans-unit>\
            <trans-unit id=\"k3\"><source>src_k3</source><target>fr3</target></trans-unit>\
            <trans-unit id=\"k_new\"><source>new</source><target>frn</target></trans-unit>\
            </body></file></xliff>";
        let stats = merge_xliff(&mut base, fragment);
        assert_eq!(stats, XliffMergeStats {
            updated: 3,
            inserted: 1,
            lost: 0,
        });
        assert!(base.rows().contains_key(&StringKey("k4".to_string())));
    }

    /// TC-15.13.3.3 — divergent source text after export resets workflow state.
    #[test]
    fn tc_15_13_3_3_source_dirty_detection() {
        let mut model = StringTableModel::new();
        model.insert_entry(
            StringKey("ui.ok".to_string()),
            StringEntry {
                source: "OK".to_string(),
                translations: HashMap::new(),
                state: TransUnitState::Translated,
                locked: false,
            },
        );
        let xml = encode_xliff(&model);
        model
            .rows_mut()
            .get_mut(&StringKey("ui.ok".to_string()))
            .expect("row")
            .source = "OK!".to_string();
        reimport_reset_on_source_drift(&mut model, &xml);
        let st = model
            .get(&StringKey("ui.ok".to_string()))
            .expect("entry")
            .state;
        assert_eq!(st, TransUnitState::Initial);
    }
}
