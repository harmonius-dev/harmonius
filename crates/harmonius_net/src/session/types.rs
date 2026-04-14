//! Session-scoped identifiers and value types for network services.

use std::net::SocketAddr;

/// Stable account identifier from identity provider mapping.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AccountId(pub u64);

/// In-engine session identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SessionId(pub u128);

/// Match or lobby player handle (distinct from account in tests).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PlayerId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Platform {
    Steam,
    PlayStation,
    Xbox,
    AppleGameCenter,
    Google,
    Custom,
}

/// Capability flags carried on a session token.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Permissions(pub u32);

impl Permissions {
    pub const PLAY: Self = Self(1 << 0);
    pub const SPECTATE: Self = Self(1 << 1);
    pub const MODERATE: Self = Self(1 << 2);
    pub const ADMIN: Self = Self(1 << 3);
    pub const VIP_QUEUE: Self = Self(1 << 4);
}

/// Signed session credential returned by [`super::auth::AuthService`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionToken {
    pub account_id: AccountId,
    pub session_id: SessionId,
    pub platform: Platform,
    pub issued_at: u64,
    pub expires_at: u64,
    pub permissions: Permissions,
    pub subject: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenError {
    Expired,
    Invalid,
}

/// Glicko-2-style skill triple (simplified update in this crate).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Glicko2Rating {
    pub rating: f64,
    pub deviation: f64,
    pub volatility: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MatchOutcome {
    Win,
    Loss,
    Draw,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QueuePriority {
    Standard,
    Returning,
    Subscriber,
    Founder,
    Admin,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PartyRole {
    Tank,
    Healer,
    Dps,
    Support,
    Unassigned,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReadyCheckResult {
    AllReady,
    Timeout { not_ready: u32 },
    Cancelled,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

/// ECS-like player snapshot preserved across reconnect grace.
#[derive(Clone, Debug, PartialEq)]
pub struct PlayerSnapshot {
    pub transform: Transform,
    pub health: Health,
    pub buffs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SessionError {
    GraceExpired,
    UnknownPlayer,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionRestored {
    pub player_id: PlayerId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueuePosition {
    pub player_id: PlayerId,
    pub position: u32,
    pub eta_secs: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubsystemTag {
    Renderer,
    WindowSystem,
    AudioMixer,
    InputDevice,
    Simulation,
    NetworkStack,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HeadlessConfig {
    pub tick_hz: u32,
    pub max_players: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InstanceId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstanceStatus {
    Starting,
    Ready,
    Draining,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RegistryEntry {
    pub id: InstanceId,
    pub addr: SocketAddr,
    pub status: InstanceStatus,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClusterPool {
    pub warm: u32,
    pub target_region: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SessionSpec {
    pub map: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BackfillRequest {
    pub match_id: u64,
    pub slot: u8,
    pub rating_window: f64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AbandonPenalty {
    Cooldown(std::time::Duration),
    RatingPenalty(i32),
    TempBan(std::time::Duration),
}
