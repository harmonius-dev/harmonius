//! UI-facing combat log types shared with `docs/design/integration/event-logs-ui.md`.

use harmonius_event_logs::{Entity, EventLogQuery, EventTypeId, LogEventMetadata};
use rkyv::{Archive, Deserialize, Serialize};
use smallvec::SmallVec;
use smol_str::SmolStr;

/// Opaque localization table key carried on each [`CombatLogLine`].
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Hash, Archive, Serialize, Deserialize,
)]
pub struct LocalizedStringId(pub u32);

/// Locale discriminator for deterministic string resolution in tests.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct LocaleId(pub u32);

/// Combat event category. Fully enumerated so UI style classes and filter predicates are
/// exhaustive.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Hash,
    Archive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum CombatEventKind {
    /// Direct damage.
    Damage = 0,
    /// Healing applied.
    Healing = 1,
    /// Status effect applied.
    StatusApplied = 2,
    /// Status effect removed.
    StatusRemoved = 3,
    /// Entity death.
    Death = 4,
    /// Attack missed.
    Miss = 5,
    /// Attack dodged.
    Dodge = 6,
    /// Attack blocked.
    Block = 7,
}

/// Structured combat payload stored inside [`harmonius_event_logs::EventLog`].
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Archive, Serialize, Deserialize,
)]
pub struct CombatEvent {
    /// High-level category for filtering and templates.
    pub kind: CombatEventKind,
    /// Signed magnitude (damage positive, heal positive magnitude).
    pub value: i32,
    /// Attacker or healer.
    pub source: Entity,
    /// Primary target.
    pub target: Entity,
}

impl LogEventMetadata for CombatEvent {
    fn event_type_id(&self) -> EventTypeId {
        EventTypeId(self.kind as u32)
    }
}

/// Typed argument slot for a [`CombatLogLine`] template.
#[derive(
    Clone, Debug, PartialEq, Archive, Serialize, Deserialize,
)]
pub enum CombatLogArg {
    /// Display name resolved from an entity's name table at paint time.
    Entity(Entity),
    /// Integer magnitude (damage, heal amount).
    Int(i32),
    /// Fractional value (percentages, multipliers).
    Float(f32),
    /// Pre-localized ability or status name.
    Name(LocalizedStringId),
}

/// Structured combat log line produced from a [`harmonius_event_logs::DecayingEntry`].
#[derive(
    Clone, Debug, PartialEq, Archive, Serialize, Deserialize,
)]
pub struct CombatLogLine {
    /// Localization key for the format string.
    pub template: LocalizedStringId,
    /// Event category for style class routing.
    pub kind: CombatEventKind,
    /// Typed argument slots for the template.
    pub args: Vec<CombatLogArg>,
    /// Display opacity based on entry accuracy.
    pub opacity: f32,
    /// Game tick for sorting.
    pub timestamp: u64,
}

/// Data binding that connects an [`harmonius_event_logs::EventLog`] to a combat log scroll view.
#[derive(
    Clone, Debug, PartialEq, Archive, Serialize, Deserialize,
)]
pub struct CombatLogBinding {
    /// Entity whose [`harmonius_event_logs::EventLog`] to display.
    pub log_entity: Entity,
    /// Active filter query.
    pub query: EventLogQuery,
    /// Maximum entries to display.
    pub max_display: u16,
    /// Max entries processed per frame to throttle high-rate updates.
    pub max_per_frame: u16,
}

/// ECS-style notification that a new row exists (display-only integration surface).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LogEntryAdded {
    /// Owning log entity.
    pub log_entity: Entity,
    /// Index of the appended entry (`EntryId` payload in the simulation crate).
    pub entry_index: u32,
}

/// World-space floating combat text widget state (simulation of Phase-8 spawn payload).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FloatingCombatText {
    /// World-space anchor.
    pub world_pos: [f32; 3],
    /// Total lifetime in seconds.
    pub duration: f32,
    /// Accumulated lifetime in seconds.
    pub elapsed: f32,
}

impl FloatingCombatText {
    /// Advances elapsed time and reports whether the widget should despawn.
    pub fn tick(&mut self, dt: f32) -> bool {
        self.elapsed += dt;
        self.elapsed >= self.duration
    }
}

/// Minimal transform sample for FCT positioning tests.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    /// Translation used as FCT anchor.
    pub translation: [f32; 3],
}

/// Minimal data-binding bag mirroring the UI framework name from the integration design.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DataBindingComponent {
    /// Last computed [`CombatLogLine`] rows for paint-time consumption.
    pub computed: SmallVec<[CombatLogLine; 8]>,
}

/// Single painted row inside a log [`ScrollView`](crate::scroll::ScrollView).
#[derive(Clone, Debug, PartialEq)]
pub struct RichText {
    /// Resolved UTF-8 payload for deterministic tests.
    pub display: SmolStr,
    /// Per-line opacity from accuracy styling.
    pub opacity: f32,
}
