//! Typed console variables (`ConVar`) and registry APIs.

use crate::primitives::{SmallVec, SortedVecMap};

// -------- ConVarName ------------------------------------------------------

/// Stable name for a ConVar backed by a compile-time literal.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ConVarName {
    /// FNV-1a 64-bit hash of [`Self::literal`], computed at compile time.
    pub stable_hash: u64,
    /// Dotted name literal (for example `"r.debug.wireframe"`).
    pub literal: &'static str,
}

impl ConVarName {
    /// Builds a [`ConVarName`] from a `'static` literal.
    pub const fn new(literal: &'static str) -> Self {
        Self {
            stable_hash: fnv1a_const(literal),
            literal,
        }
    }
}

impl Ord for ConVarName {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.stable_hash
            .cmp(&other.stable_hash)
            .then_with(|| self.literal.cmp(other.literal))
    }
}

impl PartialOrd for ConVarName {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const fn fnv1a_const(s: &str) -> u64 {
    let bytes = s.as_bytes();
    let mut hash: u64 = 0xcbf29ce484222325;
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(0x100000001b3);
        i += 1;
    }
    hash
}

// -------- ConVarValue -----------------------------------------------------

/// Supported ConVar payloads.
#[derive(Clone, Debug, PartialEq)]
pub enum ConVarValue {
    /// Boolean toggle.
    Bool(bool),
    /// Signed 32-bit integer.
    Int(i32),
    /// Single-precision float.
    Float(f32),
    /// Small set of string literals resolved at registration time.
    StaticStr(&'static str),
}

// -------- ConVarScope -----------------------------------------------------

/// Declares how widely a value is shared.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ConVarScope {
    /// One value shared across every world instance.
    Global,
    /// Each world owns an isolated copy.
    PerWorld,
    /// Mutations require an enabled cheat session.
    Cheat,
}

// -------- ConVarFlags -----------------------------------------------------

bitflags::bitflags! {
    /// Bit flags controlling persistence, replication, and mutation policy.
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub struct ConVarFlags: u32 {
        /// Only mutable when session cheats are enabled.
        const CHEAT = 1 << 0;
        /// Persisted to the `.cfg` file.
        const ARCHIVE = 1 << 1;
        /// Replicated to networked clients.
        const REPLICATED = 1 << 2;
        /// Read-only at runtime (registration only).
        const READ_ONLY = 1 << 3;
        /// Current value survives middleman `.dylib` reload.
        const HOT_RELOADABLE = 1 << 4;
    }
}

// -------- ConVarTag -------------------------------------------------------

/// Optional grouping label for editor tooling.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ConVarTag {
    /// Rendering subsystem.
    Render,
    /// Physics subsystem.
    Physics,
    /// Audio subsystem.
    Audio,
    /// Networking subsystem.
    Network,
    /// Debug visualization.
    Debug,
    /// Profiling toggles.
    Profiler,
    /// Logging knobs.
    Log,
}

// -------- ConVarEntry -----------------------------------------------------

/// Registered metadata plus current and default values.
#[derive(Clone, Debug)]
pub struct ConVarEntry {
    /// Stable name.
    pub name: ConVarName,
    /// Active value.
    pub value: ConVarValue,
    /// Registration default.
    pub default: ConVarValue,
    /// Sharing semantics.
    pub scope: ConVarScope,
    /// Policy flags.
    pub flags: ConVarFlags,
    /// Tooling tags.
    pub tags: SmallVec<ConVarTag, 4>,
}

// -------- ConVarRegistry --------------------------------------------------

/// Stores all console variables for one world or the global table.
#[derive(Clone, Debug)]
pub struct ConVarRegistry {
    entries: SortedVecMap<ConVarName, ConVarEntry>,
    events: SmallVec<ConVarChanged, 16>,
    cheats_enabled: bool,
}

impl ConVarRegistry {
    /// Builds an empty registry with cheats disabled.
    pub fn new() -> Self {
        Self {
            entries: SortedVecMap::new(),
            events: SmallVec::new(),
            cheats_enabled: false,
        }
    }

    /// Registers a new entry. Duplicate names return [`ConVarError::AlreadyRegistered`].
    pub fn register(&mut self, entry: ConVarEntry) -> Result<(), ConVarError> {
        if self.entries.get(&entry.name).is_some() {
            return Err(ConVarError::AlreadyRegistered { name: entry.name });
        }
        self.entries.insert(entry.name, entry);
        Ok(())
    }

    /// Returns a clone of the stored value when present.
    pub fn get(&self, name: ConVarName) -> Option<ConVarValue> {
        self.entries.get(&name).map(|e| e.value.clone())
    }

    /// Reads a boolean when the stored type matches.
    pub fn get_bool(&self, name: ConVarName) -> Option<bool> {
        match self.get(name)? {
            ConVarValue::Bool(b) => Some(b),
            _ => None,
        }
    }

    /// Reads an integer when the stored type matches.
    pub fn get_int(&self, name: ConVarName) -> Option<i32> {
        match self.get(name)? {
            ConVarValue::Int(i) => Some(i),
            _ => None,
        }
    }

    /// Reads a float when the stored type matches.
    pub fn get_float(&self, name: ConVarName) -> Option<f32> {
        match self.get(name)? {
            ConVarValue::Float(f) => Some(f),
            _ => None,
        }
    }

    /// Reads a static string when the stored type matches.
    pub fn get_str(&self, name: ConVarName) -> Option<&'static str> {
        match self.get(name)? {
            ConVarValue::StaticStr(s) => Some(s),
            _ => None,
        }
    }

    /// Mutates the current value with full policy checks.
    pub fn set(&mut self, name: ConVarName, value: ConVarValue) -> Result<(), ConVarError> {
        let entry = self
            .entries
            .get_mut(&name)
            .ok_or(ConVarError::NotFound { name })?;
        if entry.flags.contains(ConVarFlags::READ_ONLY) {
            return Err(ConVarError::ReadOnly { name });
        }
        let cheat_guarded =
            entry.scope == ConVarScope::Cheat || entry.flags.contains(ConVarFlags::CHEAT);
        if cheat_guarded && !self.cheats_enabled {
            return Err(ConVarError::CheatBlocked { name });
        }
        if !same_variant(&entry.value, &value) {
            return Err(ConVarError::TypeMismatch { name });
        }
        if entry.value == value {
            return Ok(());
        }
        let old_value = entry.value.clone();
        entry.value = value.clone();
        self.events.push(ConVarChanged {
            name,
            old_value,
            new_value: value,
        });
        Ok(())
    }

    /// Restores [`ConVarEntry::default`] for `name`.
    pub fn reset(&mut self, name: ConVarName) -> Result<(), ConVarError> {
        let entry = self
            .entries
            .get_mut(&name)
            .ok_or(ConVarError::NotFound { name })?;
        if entry.flags.contains(ConVarFlags::READ_ONLY) {
            return Err(ConVarError::ReadOnly { name });
        }
        let cheat_guarded =
            entry.scope == ConVarScope::Cheat || entry.flags.contains(ConVarFlags::CHEAT);
        if cheat_guarded && !self.cheats_enabled {
            return Err(ConVarError::CheatBlocked { name });
        }
        let target = entry.default.clone();
        if entry.value == target {
            return Ok(());
        }
        let old_value = entry.value.clone();
        entry.value = target.clone();
        self.events.push(ConVarChanged {
            name,
            old_value,
            new_value: target,
        });
        Ok(())
    }

    /// Removes queued change records and returns them to the caller.
    pub fn drain_events(&mut self) -> SmallVec<ConVarChanged, 16> {
        let mut out = SmallVec::new();
        for ev in self.events.drain_all() {
            out.push(ev);
        }
        out
    }

    /// Copies hot-reloadable values from `old` into `self`, emitting change records.
    pub fn survive_reload(&mut self, old: ConVarRegistry) {
        for (name, entry) in self.entries.iter_mut() {
            if !entry.flags.contains(ConVarFlags::HOT_RELOADABLE) {
                continue;
            }
            let Some(old_entry) = old.entries.get(name) else {
                continue;
            };
            if !same_variant(&entry.value, &old_entry.value) {
                continue;
            }
            if old_entry.value == entry.value {
                continue;
            }
            let old_value = entry.value.clone();
            entry.value = old_entry.value.clone();
            self.events.push(ConVarChanged {
                name: *name,
                old_value,
                new_value: entry.value.clone(),
            });
        }
    }

    /// Enables cheat-scoped mutations for this session.
    pub fn enable_cheats(&mut self) {
        self.cheats_enabled = true;
    }

    /// Returns whether cheats are enabled for this registry.
    pub fn cheats_enabled(&self) -> bool {
        self.cheats_enabled
    }

    /// Borrows a registered entry when present.
    pub fn get_entry(&self, name: ConVarName) -> Option<&ConVarEntry> {
        self.entries.get(&name)
    }
}

impl Default for ConVarRegistry {
    fn default() -> Self {
        Self::new()
    }
}

fn same_variant(lhs: &ConVarValue, rhs: &ConVarValue) -> bool {
    matches!(
        (lhs, rhs),
        (ConVarValue::Bool(_), ConVarValue::Bool(_))
            | (ConVarValue::Int(_), ConVarValue::Int(_))
            | (ConVarValue::Float(_), ConVarValue::Float(_))
            | (ConVarValue::StaticStr(_), ConVarValue::StaticStr(_))
    )
}

// -------- ConVarChanged Event --------------------------------------------

/// Fired after a successful mutation or hot-reload copy.
#[derive(Clone, Debug, PartialEq)]
pub struct ConVarChanged {
    /// Affected name.
    pub name: ConVarName,
    /// Previous value.
    pub old_value: ConVarValue,
    /// New value.
    pub new_value: ConVarValue,
}

// -------- ConVarError -----------------------------------------------------

/// Registry mutation failures.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConVarError {
    /// Unknown name.
    NotFound {
        /// Missing name.
        name: ConVarName,
    },
    /// Value variant does not match the registered entry.
    TypeMismatch {
        /// Entry name.
        name: ConVarName,
    },
    /// Entry cannot be changed at runtime.
    ReadOnly {
        /// Entry name.
        name: ConVarName,
    },
    /// Cheat policy blocked the change.
    CheatBlocked {
        /// Entry name.
        name: ConVarName,
    },
    /// Reserved for future clamped numeric ranges.
    OutOfRange {
        /// Entry name.
        name: ConVarName,
    },
    /// Duplicate registration attempt.
    AlreadyRegistered {
        /// Conflicting name.
        name: ConVarName,
    },
}

// -------- ECS integration -------------------------------------------------

/// Resource alias used by ECS integrations.
pub type ConVarRegistryResource = ConVarRegistry;

/// Borrowing handle used by synchronous ECS-style dispatch tests.
pub struct ResMut<'a, T>(pub &'a mut T);

impl<T> std::ops::Deref for ResMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<T> std::ops::DerefMut for ResMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

/// Borrowing read handle.
pub struct Res<'a, T>(pub &'a T);

impl<T> std::ops::Deref for Res<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

/// Collects drained change records into a caller-owned buffer.
pub struct EventWriter<'a, T>(pub &'a mut Vec<T>);

impl<T> EventWriter<'_, T> {
    /// Appends one event.
    pub fn send(&mut self, event: T) {
        self.0.push(event);
    }
}

/// Drains queued [`ConVarChanged`] records and forwards clones to `writer`.
pub fn convar_dispatch_system(
    mut registry: ResMut<'_, ConVarRegistryResource>,
    mut writer: EventWriter<'_, ConVarChanged>,
) {
    let drained = registry.drain_events();
    for ev in drained.iter() {
        writer.send(ev.clone());
    }
}

// -------- Text .cfg format ------------------------------------------------

/// Summary produced by [`parse_cfg`].
#[derive(Debug)]
pub struct CfgReport {
    /// Successfully applied lines.
    pub applied: u32,
    /// Keys that were not registered.
    pub unknown: SmallVec<String, 8>,
    /// Keys whose textual value did not match the registered type.
    pub type_mismatch: SmallVec<String, 8>,
}

/// Serializes every [`ConVarFlags::ARCHIVE`] entry as `key = value` lines.
pub fn serialize_cfg(reg: &ConVarRegistry) -> String {
    let mut out = String::new();
    out.push_str("# Harmonius user.cfg\n");
    for (_name, entry) in reg.entries.iter() {
        if !entry.flags.contains(ConVarFlags::ARCHIVE) {
            continue;
        }
        out.push_str(entry.name.literal);
        out.push_str(" = ");
        out.push_str(&format_value(&entry.value));
        out.push('\n');
    }
    out
}

fn format_value(value: &ConVarValue) -> String {
    match value {
        ConVarValue::Bool(b) => {
            if *b {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
        ConVarValue::Int(i) => i.to_string(),
        ConVarValue::Float(f) => f.to_string(),
        ConVarValue::StaticStr(s) => format!("\"{s}\""),
    }
}

/// Parses `key = value` lines and applies them through [`ConVarRegistry::set`].
pub fn parse_cfg(reg: &mut ConVarRegistry, contents: &str) -> CfgReport {
    let mut report = CfgReport {
        applied: 0,
        unknown: SmallVec::new(),
        type_mismatch: SmallVec::new(),
    };
    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value_text)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let value_text = value_text.trim();
        let Some(name) = find_name_by_literal(reg, key) else {
            report.unknown.push(key.to_string());
            continue;
        };
        let Some(parsed) = parse_value_for(reg, name, value_text) else {
            report.type_mismatch.push(key.to_string());
            continue;
        };
        if reg.set(name, parsed).is_ok() {
            report.applied += 1;
        }
    }
    report
}

fn find_name_by_literal(reg: &ConVarRegistry, key: &str) -> Option<ConVarName> {
    for (name, _entry) in reg.entries.iter() {
        if name.literal == key {
            return Some(*name);
        }
    }
    None
}

fn parse_value_for(reg: &ConVarRegistry, name: ConVarName, text: &str) -> Option<ConVarValue> {
    let entry = reg.get_entry(name)?;
    match &entry.value {
        ConVarValue::Bool(_) => {
            let lower = text.to_ascii_lowercase();
            if lower == "true" {
                Some(ConVarValue::Bool(true))
            } else if lower == "false" {
                Some(ConVarValue::Bool(false))
            } else {
                None
            }
        }
        ConVarValue::Int(_) => text.parse::<i32>().ok().map(ConVarValue::Int),
        ConVarValue::Float(_) => text.parse::<f32>().ok().map(ConVarValue::Float),
        ConVarValue::StaticStr(_) => {
            let unquoted = text
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .unwrap_or(text);
            match reg.get_entry(name)?.default {
                ConVarValue::StaticStr(expected) if unquoted == expected => {
                    Some(ConVarValue::StaticStr(expected))
                }
                _ => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const R_DEBUG_WIREFRAME: ConVarName = ConVarName::new("r.debug.wireframe");
    const R_SCALE: ConVarName = ConVarName::new("r.scale");
    const NET_TICK_RATE: ConVarName = ConVarName::new("net.tick_rate");
    const LOG_LEVEL: ConVarName = ConVarName::new("log.level");
    const PHYS_GRAVITY_Y: ConVarName = ConVarName::new("phys.gravity.y");
    const UNKNOWN: ConVarName = ConVarName::new("unknown");

    fn entry_wireframe() -> ConVarEntry {
        ConVarEntry {
            name: R_DEBUG_WIREFRAME,
            value: ConVarValue::Bool(false),
            default: ConVarValue::Bool(false),
            scope: ConVarScope::Global,
            flags: ConVarFlags::ARCHIVE | ConVarFlags::HOT_RELOADABLE,
            tags: SmallVec::new(),
        }
    }

    fn entry_scale() -> ConVarEntry {
        ConVarEntry {
            name: R_SCALE,
            value: ConVarValue::Float(1.0),
            default: ConVarValue::Float(1.0),
            scope: ConVarScope::Global,
            flags: ConVarFlags::ARCHIVE | ConVarFlags::HOT_RELOADABLE,
            tags: SmallVec::new(),
        }
    }

    #[test]
    fn tc_1_16_1_1_convar_entry_construction() {
        let e = entry_wireframe();
        assert_eq!(e.value, e.default);
        assert_eq!(e.scope, ConVarScope::Global);
        assert_eq!(R_DEBUG_WIREFRAME.literal, "r.debug.wireframe");
        assert_eq!(
            R_DEBUG_WIREFRAME.stable_hash,
            fnv1a_const("r.debug.wireframe")
        );
    }

    #[test]
    fn tc_1_16_4_1_registry_register_success() {
        let mut reg = ConVarRegistry::new();
        reg.register(entry_wireframe()).unwrap();
        assert_eq!(reg.get(R_DEBUG_WIREFRAME), Some(ConVarValue::Bool(false)));
    }

    #[test]
    fn tc_1_16_4_2_registry_register_duplicate() {
        let mut reg = ConVarRegistry::new();
        reg.register(entry_scale()).unwrap();
        let err = reg.register(entry_scale()).unwrap_err();
        assert_eq!(err, ConVarError::AlreadyRegistered { name: R_SCALE });
    }

    #[test]
    fn tc_1_16_4_3_registry_get_unknown_name() {
        let reg = ConVarRegistry::new();
        assert_eq!(reg.get(UNKNOWN), None);
    }

    #[test]
    fn tc_1_16_1_2_typed_getters() {
        let mut reg = ConVarRegistry::new();
        reg.register(entry_wireframe()).unwrap();
        reg.register(ConVarEntry {
            name: NET_TICK_RATE,
            value: ConVarValue::Int(60),
            default: ConVarValue::Int(60),
            scope: ConVarScope::Global,
            flags: ConVarFlags::empty(),
            tags: SmallVec::new(),
        })
        .unwrap();
        reg.register(entry_scale()).unwrap();
        reg.register(ConVarEntry {
            name: LOG_LEVEL,
            value: ConVarValue::StaticStr("info"),
            default: ConVarValue::StaticStr("info"),
            scope: ConVarScope::Global,
            flags: ConVarFlags::empty(),
            tags: SmallVec::new(),
        })
        .unwrap();
        assert_eq!(reg.get_bool(R_DEBUG_WIREFRAME), Some(false));
        assert_eq!(reg.get_int(NET_TICK_RATE), Some(60));
        assert_eq!(reg.get_float(R_SCALE), Some(1.0));
        assert_eq!(reg.get_str(LOG_LEVEL), Some("info"));
        assert_eq!(reg.get_int(R_DEBUG_WIREFRAME), None);
    }

    #[test]
    fn tc_1_16_1_3_set_mutates_current_preserves_default() {
        let mut reg = ConVarRegistry::new();
        reg.register(entry_scale()).unwrap();
        reg.set(R_SCALE, ConVarValue::Float(2.0)).unwrap();
        assert_eq!(reg.get_float(R_SCALE), Some(2.0));
        assert_eq!(
            reg.get_entry(R_SCALE).unwrap().default,
            ConVarValue::Float(1.0)
        );
    }

    #[test]
    fn tc_1_16_1_4_reset_reverts_to_default() {
        let mut reg = ConVarRegistry::new();
        reg.register(entry_scale()).unwrap();
        reg.set(R_SCALE, ConVarValue::Float(2.0)).unwrap();
        reg.reset(R_SCALE).unwrap();
        assert_eq!(reg.get_float(R_SCALE), Some(1.0));
    }

    #[test]
    fn tc_1_16_1_5_set_type_mismatch() {
        let mut reg = ConVarRegistry::new();
        reg.register(entry_wireframe()).unwrap();
        let err = reg.set(R_DEBUG_WIREFRAME, ConVarValue::Int(1)).unwrap_err();
        assert_eq!(
            err,
            ConVarError::TypeMismatch {
                name: R_DEBUG_WIREFRAME
            }
        );
        assert_eq!(reg.get_bool(R_DEBUG_WIREFRAME), Some(false));
    }

    #[test]
    fn tc_1_16_3_1_read_only_flag_blocks_set() {
        let mut reg = ConVarRegistry::new();
        reg.register(ConVarEntry {
            name: R_SCALE,
            value: ConVarValue::Float(1.0),
            default: ConVarValue::Float(1.0),
            scope: ConVarScope::Global,
            flags: ConVarFlags::READ_ONLY,
            tags: SmallVec::new(),
        })
        .unwrap();
        let err = reg.set(R_SCALE, ConVarValue::Float(2.0)).unwrap_err();
        assert_eq!(err, ConVarError::ReadOnly { name: R_SCALE });
    }

    #[test]
    fn tc_1_16_3_2_cheat_flag_requires_enable() {
        let mut reg = ConVarRegistry::new();
        reg.register(ConVarEntry {
            name: PHYS_GRAVITY_Y,
            value: ConVarValue::Float(-9.81),
            default: ConVarValue::Float(-9.81),
            scope: ConVarScope::Cheat,
            flags: ConVarFlags::CHEAT,
            tags: SmallVec::new(),
        })
        .unwrap();
        let err = reg
            .set(PHYS_GRAVITY_Y, ConVarValue::Float(-20.0))
            .unwrap_err();
        assert_eq!(
            err,
            ConVarError::CheatBlocked {
                name: PHYS_GRAVITY_Y
            }
        );
        reg.enable_cheats();
        reg.set(PHYS_GRAVITY_Y, ConVarValue::Float(-20.0)).unwrap();
    }

    #[test]
    fn tc_1_16_8_1_set_emits_convar_changed_event() {
        let mut reg = ConVarRegistry::new();
        reg.register(entry_scale()).unwrap();
        reg.set(R_SCALE, ConVarValue::Float(2.0)).unwrap();
        let drained = reg.drain_events();
        assert_eq!(drained.len(), 1);
        let ev = &drained.as_slice()[0];
        assert_eq!(ev.name, R_SCALE);
        assert_eq!(ev.old_value, ConVarValue::Float(1.0));
        assert_eq!(ev.new_value, ConVarValue::Float(2.0));
    }

    #[test]
    fn tc_1_16_8_2_reset_emits_convar_changed_event() {
        let mut reg = ConVarRegistry::new();
        reg.register(entry_scale()).unwrap();
        reg.set(R_SCALE, ConVarValue::Float(2.0)).unwrap();
        reg.reset(R_SCALE).unwrap();
        let drained = reg.drain_events();
        assert_eq!(drained.len(), 2);
        let second = &drained.as_slice()[1];
        assert_eq!(second.old_value, ConVarValue::Float(2.0));
        assert_eq!(second.new_value, ConVarValue::Float(1.0));
    }

    #[test]
    fn tc_1_16_7_1_serialize_cfg_only_archive_flag() {
        let mut reg = ConVarRegistry::new();
        reg.register(ConVarEntry {
            name: R_DEBUG_WIREFRAME,
            value: ConVarValue::Bool(false),
            default: ConVarValue::Bool(false),
            scope: ConVarScope::Global,
            flags: ConVarFlags::ARCHIVE,
            tags: SmallVec::new(),
        })
        .unwrap();
        reg.register(ConVarEntry {
            name: R_SCALE,
            value: ConVarValue::Float(1.0),
            default: ConVarValue::Float(1.0),
            scope: ConVarScope::Global,
            flags: ConVarFlags::empty(),
            tags: SmallVec::new(),
        })
        .unwrap();
        let text = serialize_cfg(&reg);
        assert!(text.contains("r.debug.wireframe"));
        assert!(!text.contains("r.scale"));
    }

    #[test]
    fn tc_1_16_7_2_parse_cfg_apply_values() {
        let mut reg = ConVarRegistry::new();
        reg.register(ConVarEntry {
            name: R_DEBUG_WIREFRAME,
            value: ConVarValue::Bool(false),
            default: ConVarValue::Bool(false),
            scope: ConVarScope::Global,
            flags: ConVarFlags::ARCHIVE,
            tags: SmallVec::new(),
        })
        .unwrap();
        reg.register(entry_scale()).unwrap();
        let report = parse_cfg(&mut reg, "r.debug.wireframe = true\nr.scale = 1.5\n");
        assert_eq!(report.applied, 2);
        assert_eq!(reg.get_bool(R_DEBUG_WIREFRAME), Some(true));
        assert_eq!(reg.get_float(R_SCALE), Some(1.5));
    }

    #[test]
    fn tc_1_16_7_3_parse_cfg_unknown_key() {
        let mut reg = ConVarRegistry::new();
        reg.register(entry_scale()).unwrap();
        let report = parse_cfg(&mut reg, "unknown.key = 1\nr.scale = 1.5\n");
        assert_eq!(report.applied, 1);
        assert_eq!(report.unknown.len(), 1);
    }

    #[test]
    fn tc_1_16_7_4_parse_cfg_type_mismatch() {
        let mut reg = ConVarRegistry::new();
        reg.register(entry_wireframe()).unwrap();
        let report = parse_cfg(&mut reg, "r.debug.wireframe = 5\n");
        assert_eq!(report.type_mismatch.len(), 1);
        assert_eq!(reg.get_bool(R_DEBUG_WIREFRAME), Some(false));
    }

    #[test]
    fn tc_1_16_7_5_parse_cfg_comments_skipped() {
        let mut reg = ConVarRegistry::new();
        reg.register(entry_scale()).unwrap();
        let report = parse_cfg(&mut reg, "# comment\nr.scale = 2.0\n");
        assert_eq!(report.applied, 1);
    }

    #[test]
    fn tc_1_16_5_1_survive_reload_preserves_hot_reloadable() {
        let mut old = ConVarRegistry::new();
        old.register(entry_scale()).unwrap();
        old.set(R_SCALE, ConVarValue::Float(2.0)).unwrap();
        let mut new_reg = ConVarRegistry::new();
        new_reg.register(entry_scale()).unwrap();
        new_reg.survive_reload(old);
        assert_eq!(new_reg.get_float(R_SCALE), Some(2.0));
    }

    #[test]
    fn tc_1_16_5_2_survive_reload_skips_non_hot_reloadable() {
        let mut old = ConVarRegistry::new();
        old.register(ConVarEntry {
            name: PHYS_GRAVITY_Y,
            value: ConVarValue::Float(-9.81),
            default: ConVarValue::Float(-9.81),
            scope: ConVarScope::PerWorld,
            flags: ConVarFlags::CHEAT,
            tags: SmallVec::new(),
        })
        .unwrap();
        old.enable_cheats();
        old.set(PHYS_GRAVITY_Y, ConVarValue::Float(-20.0)).unwrap();

        let mut new_reg = ConVarRegistry::new();
        new_reg
            .register(ConVarEntry {
                name: PHYS_GRAVITY_Y,
                value: ConVarValue::Float(-9.81),
                default: ConVarValue::Float(-9.81),
                scope: ConVarScope::PerWorld,
                flags: ConVarFlags::CHEAT,
                tags: SmallVec::new(),
            })
            .unwrap();
        new_reg.enable_cheats();
        new_reg.survive_reload(old);
        assert_eq!(new_reg.get_float(PHYS_GRAVITY_Y), Some(-9.81));
    }

    #[test]
    fn tc_1_16_5_3_survive_reload_skips_type_mismatch() {
        const KEY: ConVarName = ConVarName::new("hot.mismatch");
        let mut old = ConVarRegistry::new();
        old.register(ConVarEntry {
            name: KEY,
            value: ConVarValue::Int(7),
            default: ConVarValue::Int(0),
            scope: ConVarScope::Global,
            flags: ConVarFlags::HOT_RELOADABLE,
            tags: SmallVec::new(),
        })
        .unwrap();

        let mut new_reg = ConVarRegistry::new();
        new_reg
            .register(ConVarEntry {
                name: KEY,
                value: ConVarValue::Float(1.5),
                default: ConVarValue::Float(1.5),
                scope: ConVarScope::Global,
                flags: ConVarFlags::HOT_RELOADABLE,
                tags: SmallVec::new(),
            })
            .unwrap();
        new_reg.survive_reload(old);
        assert_eq!(new_reg.get_float(KEY), Some(1.5));
    }

    #[test]
    fn tc_1_16_6_1_ecs_dispatch_system_forwards_events() {
        let mut reg = ConVarRegistry::new();
        reg.register(entry_scale()).unwrap();
        reg.set(R_SCALE, ConVarValue::Float(2.0)).unwrap();
        let mut bus = Vec::new();
        convar_dispatch_system(ResMut(&mut reg), EventWriter(&mut bus));
        assert_eq!(bus.len(), 1);
        assert_eq!(bus[0].name, R_SCALE);
    }

    #[test]
    fn tc_1_16_6_2_res_convar_registry_access() {
        let mut reg = ConVarRegistry::new();
        reg.register(entry_scale()).unwrap();
        let before = reg.get_float(R_SCALE);
        reg.set(R_SCALE, ConVarValue::Float(3.0)).unwrap();
        assert_eq!(before, Some(1.0));
        let mut bus = Vec::new();
        convar_dispatch_system(ResMut(&mut reg), EventWriter(&mut bus));
        assert_eq!(bus.len(), 1);
    }

    #[test]
    fn tc_1_16_2_1_per_world_scope_isolated() {
        let mut world_a = ConVarRegistry::new();
        let mut world_b = ConVarRegistry::new();
        let entry = ConVarEntry {
            name: R_SCALE,
            value: ConVarValue::Float(1.0),
            default: ConVarValue::Float(1.0),
            scope: ConVarScope::PerWorld,
            flags: ConVarFlags::empty(),
            tags: SmallVec::new(),
        };
        world_a.register(entry.clone()).unwrap();
        world_b.register(entry).unwrap();
        world_a.set(R_SCALE, ConVarValue::Float(9.0)).unwrap();
        assert_eq!(world_b.get_float(R_SCALE), Some(1.0));
    }

    #[test]
    fn tc_1_16_5_4_survive_reload_round_trip() {
        let mut old = ConVarRegistry::new();
        old.register(ConVarEntry {
            name: R_DEBUG_WIREFRAME,
            value: ConVarValue::Bool(false),
            default: ConVarValue::Bool(false),
            scope: ConVarScope::Global,
            flags: ConVarFlags::HOT_RELOADABLE,
            tags: SmallVec::new(),
        })
        .unwrap();
        old.register(entry_scale()).unwrap();
        old.set(R_DEBUG_WIREFRAME, ConVarValue::Bool(true)).unwrap();
        old.set(R_SCALE, ConVarValue::Float(2.0)).unwrap();

        let mut new_reg = ConVarRegistry::new();
        new_reg
            .register(ConVarEntry {
                name: R_DEBUG_WIREFRAME,
                value: ConVarValue::Bool(false),
                default: ConVarValue::Bool(false),
                scope: ConVarScope::Global,
                flags: ConVarFlags::HOT_RELOADABLE,
                tags: SmallVec::new(),
            })
            .unwrap();
        new_reg.register(entry_scale()).unwrap();
        new_reg.survive_reload(old);
        assert_eq!(new_reg.get_bool(R_DEBUG_WIREFRAME), Some(true));
        assert_eq!(new_reg.get_float(R_SCALE), Some(2.0));
        let events = new_reg.drain_events();
        assert!(!events.is_empty());
    }

    #[test]
    fn tc_1_16_7_6_cfg_file_round_trip_via_io_dispatcher() {
        let mut reg = ConVarRegistry::new();
        reg.register(ConVarEntry {
            name: R_DEBUG_WIREFRAME,
            value: ConVarValue::Bool(false),
            default: ConVarValue::Bool(false),
            scope: ConVarScope::Global,
            flags: ConVarFlags::ARCHIVE,
            tags: SmallVec::new(),
        })
        .unwrap();
        reg.register(ConVarEntry {
            name: NET_TICK_RATE,
            value: ConVarValue::Int(60),
            default: ConVarValue::Int(60),
            scope: ConVarScope::Global,
            flags: ConVarFlags::ARCHIVE,
            tags: SmallVec::new(),
        })
        .unwrap();
        reg.register(ConVarEntry {
            name: R_SCALE,
            value: ConVarValue::Float(1.0),
            default: ConVarValue::Float(1.0),
            scope: ConVarScope::Global,
            flags: ConVarFlags::ARCHIVE,
            tags: SmallVec::new(),
        })
        .unwrap();
        reg.set(R_DEBUG_WIREFRAME, ConVarValue::Bool(true)).unwrap();
        reg.set(NET_TICK_RATE, ConVarValue::Int(120)).unwrap();
        reg.set(R_SCALE, ConVarValue::Float(2.5)).unwrap();
        let written = serialize_cfg(&reg);
        let mut io = MemoryIo::new();
        io.write("save://user.cfg", written.as_bytes());
        let bytes = io.read("save://user.cfg").expect("read back");
        let read_str = std::str::from_utf8(bytes).unwrap();
        assert_eq!(read_str, written);

        let mut reg2 = ConVarRegistry::new();
        reg2.register(ConVarEntry {
            name: R_DEBUG_WIREFRAME,
            value: ConVarValue::Bool(false),
            default: ConVarValue::Bool(false),
            scope: ConVarScope::Global,
            flags: ConVarFlags::ARCHIVE,
            tags: SmallVec::new(),
        })
        .unwrap();
        reg2.register(ConVarEntry {
            name: NET_TICK_RATE,
            value: ConVarValue::Int(60),
            default: ConVarValue::Int(60),
            scope: ConVarScope::Global,
            flags: ConVarFlags::ARCHIVE,
            tags: SmallVec::new(),
        })
        .unwrap();
        reg2.register(ConVarEntry {
            name: R_SCALE,
            value: ConVarValue::Float(1.0),
            default: ConVarValue::Float(1.0),
            scope: ConVarScope::Global,
            flags: ConVarFlags::ARCHIVE,
            tags: SmallVec::new(),
        })
        .unwrap();
        let _ = parse_cfg(&mut reg2, read_str);
        assert_eq!(
            reg2.get_bool(R_DEBUG_WIREFRAME),
            reg.get_bool(R_DEBUG_WIREFRAME)
        );
        assert_eq!(reg2.get_int(NET_TICK_RATE), reg.get_int(NET_TICK_RATE));
        assert_eq!(reg2.get_float(R_SCALE), reg.get_float(R_SCALE));
    }

    struct MemoryIo {
        files: std::collections::HashMap<String, Vec<u8>>,
    }

    impl MemoryIo {
        fn new() -> Self {
            Self {
                files: std::collections::HashMap::new(),
            }
        }

        fn write(&mut self, path: &str, bytes: &[u8]) {
            self.files.insert(path.to_string(), bytes.to_vec());
        }

        fn read(&self, path: &str) -> Option<&[u8]> {
            self.files.get(path).map(|v| v.as_slice())
        }
    }

    #[test]
    fn tc_1_16_4_4_get_cached_lookup_under_100_ns() {
        let mut reg = ConVarRegistry::new();
        reg.register(entry_scale()).unwrap();
        for i in 0..200 {
            let name = format!("bench.key.{i}");
            let leaked: &'static str = Box::leak(name.into_boxed_str());
            reg.register(ConVarEntry {
                name: ConVarName::new(leaked),
                value: ConVarValue::Int(i),
                default: ConVarValue::Int(i),
                scope: ConVarScope::Global,
                flags: ConVarFlags::empty(),
                tags: SmallVec::new(),
            })
            .unwrap();
        }
        let threshold_ns: u128 = if cfg!(debug_assertions) { 500 } else { 100 };
        let mut total = 0u128;
        let samples = if cfg!(debug_assertions) {
            5_000
        } else {
            50_000
        };
        for _ in 0..samples {
            let start = std::time::Instant::now();
            let v = reg.get_float(R_SCALE);
            total += start.elapsed().as_nanos();
            std::hint::black_box(v);
        }
        let avg = total / samples;
        assert!(
            avg < threshold_ns,
            "avg {avg}ns exceeds relaxed budget {threshold_ns}ns"
        );
    }

    #[test]
    fn tc_1_16_4_5_set_throughput() {
        let mut reg = ConVarRegistry::new();
        reg.register(entry_scale()).unwrap();
        for i in 0..200 {
            let name = format!("bench.set.{i}");
            let leaked: &'static str = Box::leak(name.into_boxed_str());
            reg.register(ConVarEntry {
                name: ConVarName::new(leaked),
                value: ConVarValue::Int(i),
                default: ConVarValue::Int(i),
                scope: ConVarScope::Global,
                flags: ConVarFlags::empty(),
                tags: SmallVec::new(),
            })
            .unwrap();
        }
        let threshold_ns: u128 = if cfg!(debug_assertions) { 2_000 } else { 300 };
        let mut total = 0u128;
        let samples = if cfg!(debug_assertions) {
            5_000
        } else {
            50_000
        };
        for i in 0..samples {
            let start = std::time::Instant::now();
            let v = ConVarValue::Float((i as f32) * 0.001);
            let _ = reg.set(R_SCALE, v);
            total += start.elapsed().as_nanos();
        }
        let avg = total / samples;
        assert!(
            avg < threshold_ns,
            "avg {avg}ns exceeds relaxed budget {threshold_ns}ns"
        );
    }

    #[test]
    fn tc_1_16_7_7_parse_cfg_200_entries() {
        let mut reg = ConVarRegistry::new();
        let mut cfg = String::new();
        for i in 0..200 {
            let name = format!("cfg.key.{i}");
            let leaked: &'static str = Box::leak(name.into_boxed_str());
            reg.register(ConVarEntry {
                name: ConVarName::new(leaked),
                value: ConVarValue::Int(0),
                default: ConVarValue::Int(0),
                scope: ConVarScope::Global,
                flags: ConVarFlags::ARCHIVE,
                tags: SmallVec::new(),
            })
            .unwrap();
            cfg.push_str(leaked);
            cfg.push_str(" = ");
            cfg.push_str(&(i % 7).to_string());
            cfg.push('\n');
        }
        let threshold_us: u128 = if cfg!(debug_assertions) { 5_000 } else { 1_000 };
        let start = std::time::Instant::now();
        let report = parse_cfg(&mut reg, &cfg);
        let elapsed_us = start.elapsed().as_micros();
        assert_eq!(report.applied, 200);
        assert!(
            elapsed_us < threshold_us,
            "parse took {elapsed_us}us budget {threshold_us}us"
        );
    }

    #[test]
    fn tc_1_16_7_8_serialize_cfg_200_entries() {
        let mut reg = ConVarRegistry::new();
        for i in 0..200 {
            let name = format!("cfg.out.{i}");
            let leaked: &'static str = Box::leak(name.into_boxed_str());
            reg.register(ConVarEntry {
                name: ConVarName::new(leaked),
                value: ConVarValue::Int(i),
                default: ConVarValue::Int(0),
                scope: ConVarScope::Global,
                flags: ConVarFlags::ARCHIVE,
                tags: SmallVec::new(),
            })
            .unwrap();
        }
        let threshold_us: u128 = if cfg!(debug_assertions) { 5_000 } else { 1_000 };
        let start = std::time::Instant::now();
        let text = serialize_cfg(&reg);
        let elapsed_us = start.elapsed().as_micros();
        assert!(
            elapsed_us < threshold_us,
            "serialize took {elapsed_us}us budget {threshold_us}us"
        );
        assert!(text.len() > 1000);
    }
}
