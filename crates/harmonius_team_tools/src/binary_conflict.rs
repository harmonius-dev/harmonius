//! Binary conflict parse, diff, and resolve (**TC-15.10.3.3**–**TC-15.10.3.10**, R-15.10.3).
//!
//! Known fixtures begin with [`KNOWN_MAGIC`]; unknown payloads fail parse so callers can fall back
//! to manual binary merge.

use std::collections::BTreeMap;

/// Magic header for Harmonius structural fixture blobs handled by [`BinaryConflictResolver`].
pub const KNOWN_MAGIC: &[u8] = b"HNV1\n";

/// Failure modes for binary conflict resolution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MergeError {
    /// Payload does not carry a supported Harmonius fixture header.
    UnknownAssetType,
    /// UTF-8 decode failed for a fixture body.
    InvalidUtf8,
    /// Malformed `path=value` line inside the fixture body.
    InvalidLine,
    /// `resolve` pick count does not match [`BinaryConflictResolver::diff`] output length.
    PickCountMismatch {
        /// Number of conflicts from [`BinaryConflictResolver::diff`].
        expected: usize,
        /// Picks supplied to [`BinaryConflictResolver::resolve`].
        got: usize,
    },
}

/// Unified view of base, ours, and theirs property values (see design `PropertyTrees`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PropertyTrees {
    /// Sorted unique paths with paired values from each side.
    pub rows: Vec<PropertyRow>,
}

/// One logical property across the three merge inputs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PropertyRow {
    /// Dotted property path (supports nested keys for diff reporting).
    pub path: String,
    /// Value on the merge base revision.
    pub base_value: String,
    /// Value on our branch.
    pub ours_value: String,
    /// Value on their branch.
    pub theirs_value: String,
}

/// One property the UI must present for resolution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PropertyConflict {
    /// Full dotted path (nested segments use `.`).
    pub path: String,
    /// Base revision value.
    pub base_value: String,
    /// Our branch value.
    pub ours_value: String,
    /// Their branch value.
    pub theirs_value: String,
}

/// User resolution choice for one [`PropertyConflict`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConflictPick {
    /// Keep our value.
    Ours,
    /// Keep their value.
    Theirs,
    /// Supply an explicit merged scalar.
    Manual(String),
}

/// Parses Harmonius fixture blobs and applies structural diff / resolve passes.
#[derive(Clone, Copy, Debug, Default)]
pub struct BinaryConflictResolver;

impl BinaryConflictResolver {
    /// New resolver instance.
    pub fn new() -> Self {
        Self
    }

    /// Parse three fixture blobs into a unified [`PropertyTrees`] view.
    pub fn parse(
        &self,
        base: &[u8],
        ours: &[u8],
        theirs: &[u8],
    ) -> Result<PropertyTrees, MergeError> {
        let b = strip_magic(base)?;
        let o = strip_magic(ours)?;
        let t = strip_magic(theirs)?;
        let mb = parse_body(b)?;
        let mo = parse_body(o)?;
        let mt = parse_body(t)?;
        Ok(build_trees(&mb, &mo, &mt))
    }

    /// Compute per-property conflicts where ours and theirs disagree.
    pub fn diff(&self, trees: &PropertyTrees) -> Vec<PropertyConflict> {
        let mut out = Vec::new();
        for row in &trees.rows {
            if row.ours_value != row.theirs_value {
                out.push(PropertyConflict {
                    base_value: row.base_value.clone(),
                    ours_value: row.ours_value.clone(),
                    path: row.path.clone(),
                    theirs_value: row.theirs_value.clone(),
                });
            }
        }
        out
    }

    /// Apply ordered picks for every [`Self::diff`] row and emit a merged fixture blob.
    pub fn resolve(
        &self,
        trees: &PropertyTrees,
        picks: &[ConflictPick],
    ) -> Result<Vec<u8>, MergeError> {
        let conflicts = self.diff(trees);
        if picks.len() != conflicts.len() {
            return Err(MergeError::PickCountMismatch {
                expected: conflicts.len(),
                got: picks.len(),
            });
        }

        let mut merged: BTreeMap<String, String> = BTreeMap::new();
        for row in &trees.rows {
            merged.insert(row.path.clone(), auto_merge_value(row));
        }

        for (c, pick) in conflicts.iter().zip(picks.iter()) {
            let value = match pick {
                ConflictPick::Ours => c.ours_value.clone(),
                ConflictPick::Theirs => c.theirs_value.clone(),
                ConflictPick::Manual(v) => v.clone(),
            };
            merged.insert(c.path.clone(), value);
        }

        Ok(wrap_magic(&serialize_body(&merged)))
    }
}

fn strip_magic(data: &[u8]) -> Result<&str, MergeError> {
    if !data.starts_with(KNOWN_MAGIC) {
        return Err(MergeError::UnknownAssetType);
    }
    std::str::from_utf8(&data[KNOWN_MAGIC.len()..]).map_err(|_| MergeError::InvalidUtf8)
}

fn wrap_magic(body: &[u8]) -> Vec<u8> {
    let mut out = KNOWN_MAGIC.to_vec();
    out.extend_from_slice(body);
    out
}

fn parse_body(text: &str) -> Result<BTreeMap<String, String>, MergeError> {
    let mut map = BTreeMap::new();
    for line in text.lines() {
        if line.is_empty() {
            continue;
        }
        let (k, v) = line.split_once('=').ok_or(MergeError::InvalidLine)?;
        if k.is_empty() {
            return Err(MergeError::InvalidLine);
        }
        map.insert(k.to_string(), v.to_string());
    }
    Ok(map)
}

fn serialize_body(map: &BTreeMap<String, String>) -> Vec<u8> {
    let mut s = String::new();
    for (k, v) in map {
        if !s.is_empty() {
            s.push('\n');
        }
        s.push_str(k);
        s.push('=');
        s.push_str(v);
    }
    s.into_bytes()
}

fn build_trees(
    base: &BTreeMap<String, String>,
    ours: &BTreeMap<String, String>,
    theirs: &BTreeMap<String, String>,
) -> PropertyTrees {
    let mut keys: Vec<String> = Vec::new();
    for k in base.keys() {
        keys.push(k.clone());
    }
    for k in ours.keys() {
        if !base.contains_key(k) {
            keys.push(k.clone());
        }
    }
    for k in theirs.keys() {
        if !base.contains_key(k) && !ours.contains_key(k) {
            keys.push(k.clone());
        }
    }
    keys.sort();
    keys.dedup();

    let mut rows = Vec::new();
    for path in keys {
        let base_value = base.get(&path).cloned().unwrap_or_default();
        let ours_value = ours.get(&path).cloned().unwrap_or_default();
        let theirs_value = theirs.get(&path).cloned().unwrap_or_default();
        rows.push(PropertyRow {
            base_value,
            ours_value,
            path,
            theirs_value,
        });
    }
    PropertyTrees { rows }
}

fn auto_merge_value(row: &PropertyRow) -> String {
    if row.ours_value == row.theirs_value {
        return row.ours_value.clone();
    }
    if row.ours_value == row.base_value {
        return row.theirs_value.clone();
    }
    if row.theirs_value == row.base_value {
        return row.ours_value.clone();
    }
    // Unresolved until picks overwrite; placeholder uses base for serialization stability.
    row.base_value.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(body: &str) -> Vec<u8> {
        let mut v = KNOWN_MAGIC.to_vec();
        v.extend_from_slice(body.as_bytes());
        v
    }

    /// **TC-15.10.3.3** — Binary conflict parse deserializes base/ours/theirs.
    #[test]
    fn tc_15_10_3_3_parse_round_trip() {
        let body = "a=1\nb=2";
        let base = fixture(body);
        let ours = fixture("a=1\nb=3");
        let theirs = fixture("a=1\nb=3");
        let trees = BinaryConflictResolver::new()
            .parse(&base, &ours, &theirs)
            .expect("parse");
        assert_eq!(trees.rows.len(), 2);
        assert_eq!(trees.rows[0].path, "a");
        assert_eq!(trees.rows[1].path, "b");
    }

    /// **TC-15.10.3.4** — Binary conflict diff reports per-property changes.
    #[test]
    fn tc_15_10_3_4_diff_reports_changes() {
        let base = fixture("x=0\ny=1");
        let ours = fixture("x=0\ny=2");
        let theirs = fixture("x=0\ny=3");
        let trees = BinaryConflictResolver::new()
            .parse(&base, &ours, &theirs)
            .expect("parse");
        let conflicts = BinaryConflictResolver::new().diff(&trees);
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].path, "y");
    }

    /// **TC-15.10.3.5** — Binary conflict resolve with ours pick keeps ours value.
    #[test]
    fn tc_15_10_3_5_resolve_ours() {
        let base = fixture("k=1");
        let ours = fixture("k=2");
        let theirs = fixture("k=3");
        let trees = BinaryConflictResolver::new()
            .parse(&base, &ours, &theirs)
            .expect("parse");
        let conflicts = BinaryConflictResolver::new().diff(&trees);
        assert_eq!(conflicts.len(), 1);
        let out = BinaryConflictResolver::new()
            .resolve(&trees, &[ConflictPick::Ours])
            .expect("resolve");
        let text = String::from_utf8(out).expect("utf8");
        assert!(text.contains("k=2"));
    }

    /// **TC-15.10.3.6** — Binary conflict resolve with theirs pick keeps theirs value.
    #[test]
    fn tc_15_10_3_6_resolve_theirs() {
        let base = fixture("k=1");
        let ours = fixture("k=2");
        let theirs = fixture("k=3");
        let trees = BinaryConflictResolver::new()
            .parse(&base, &ours, &theirs)
            .expect("parse");
        let out = BinaryConflictResolver::new()
            .resolve(&trees, &[ConflictPick::Theirs])
            .expect("resolve");
        let text = String::from_utf8(out).expect("utf8");
        assert!(text.contains("k=3"));
    }

    /// **TC-15.10.3.7** — Binary conflict resolve with Manual pick uses edited value.
    #[test]
    fn tc_15_10_3_7_resolve_manual() {
        let base = fixture("k=1");
        let ours = fixture("k=2");
        let theirs = fixture("k=3");
        let trees = BinaryConflictResolver::new()
            .parse(&base, &ours, &theirs)
            .expect("parse");
        let out = BinaryConflictResolver::new()
            .resolve(&trees, &[ConflictPick::Manual("9".into())])
            .expect("resolve");
        let text = String::from_utf8(out).expect("utf8");
        assert!(text.contains("k=9"));
    }

    /// **TC-15.10.3.8** — Binary conflict diff on nested properties reports full path.
    #[test]
    fn tc_15_10_3_8_nested_property_paths() {
        let base = fixture("entity.position.x=0");
        let ours = fixture("entity.position.x=1");
        let theirs = fixture("entity.position.x=2");
        let trees = BinaryConflictResolver::new()
            .parse(&base, &ours, &theirs)
            .expect("parse");
        let conflicts = BinaryConflictResolver::new().diff(&trees);
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].path, "entity.position.x");
    }

    /// **TC-15.10.3.9** — Binary conflict resolve with mixed picks across properties.
    #[test]
    fn tc_15_10_3_9_mixed_picks() {
        let base = fixture("a=0\nb=0");
        let ours = fixture("a=1\nb=2");
        let theirs = fixture("a=3\nb=4");
        let trees = BinaryConflictResolver::new()
            .parse(&base, &ours, &theirs)
            .expect("parse");
        let conflicts = BinaryConflictResolver::new().diff(&trees);
        assert_eq!(conflicts.len(), 2);
        let out = BinaryConflictResolver::new()
            .resolve(&trees, &[ConflictPick::Ours, ConflictPick::Theirs])
            .expect("resolve");
        let text = String::from_utf8(out).expect("utf8");
        assert!(text.contains("a=1"));
        assert!(text.contains("b=4"));
    }

    /// **TC-15.10.3.10** — Binary conflict parse fails gracefully on unknown type.
    #[test]
    fn tc_15_10_3_10_unknown_type_rejected() {
        let bad = b"XXXX\na=1";
        let ours = fixture("a=1");
        let theirs = fixture("a=1");
        let err = BinaryConflictResolver::new()
            .parse(bad, &ours, &theirs)
            .expect_err("unknown");
        assert_eq!(err, MergeError::UnknownAssetType);
    }
}
