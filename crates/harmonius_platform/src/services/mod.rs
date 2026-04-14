//! Online platform services (`R-14.5`, `R-14.8` stubs).

mod achievements;
mod certification;
mod cloud;
mod entitlements;
mod leaderboards;
mod presence;
mod voice;

pub use achievements::{
    AchievementError, AchievementId, AchievementProgress, AchievementService, UnlockState,
};
pub use certification::{BuildCertInputs, CertFailure, CertificationReport, CertificationValidator};
pub use cloud::{CloudConflict, CloudError, CloudKey, CloudStorageService};
pub use entitlements::EntitlementService;
pub use leaderboards::{
    LeaderboardError, LeaderboardId, LeaderboardResult, LeaderboardRow, LeaderboardScope,
    LeaderboardService, LeaderboardSort,
};
pub use presence::{PresenceError, PresenceState, RichPresenceService};
pub use voice::{VoiceBridge, VoicePlatform};

/// High-level platform failures shared by services.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformError {
    /// Library not initialized.
    NotInitialized,
    /// SDK-specific failure.
    SdkError {
        /// Host platform discriminator.
        platform: VoicePlatform,
        /// Vendor code.
        code: i32,
    },
    /// Network unavailable for cloud calls.
    NetworkUnavailable,
    /// Auth failure.
    AuthenticationFailed,
    /// Deadline exceeded.
    Timeout,
}

/// Aggregated platform services bundle.
pub struct PlatformServices {
    /// Achievement facade.
    pub achievements: AchievementService,
    /// Leaderboard facade.
    pub leaderboards: LeaderboardService,
    /// Presence facade.
    pub presence: RichPresenceService,
    /// Cloud storage facade.
    pub cloud: CloudStorageService,
    /// Entitlement checks.
    pub entitlements: EntitlementService,
    /// Voice / party bridge.
    pub voice: VoiceBridge,
}

impl PlatformServices {
    /// Initializes default stub services.
    pub fn init() -> Result<Self, PlatformError> {
        Ok(Self {
            achievements: AchievementService::new(),
            leaderboards: LeaderboardService::new(),
            presence: RichPresenceService::new(),
            cloud: CloudStorageService::new(),
            entitlements: EntitlementService::new(),
            voice: VoiceBridge::new(),
        })
    }

    /// Shuts down services (stub).
    pub fn shutdown(&mut self) -> Result<(), PlatformError> {
        Ok(())
    }
}
