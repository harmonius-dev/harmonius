//! Voice / party bridge selection (`R-14.5.4`).

use std::convert::Infallible;

/// Recognized host platforms for voice routing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VoicePlatform {
    /// PlayStation native party APIs available.
    PlayStation,
    /// Desktop without first-party party stack.
    Desktop,
}

/// Chooses native party audio or Vivox fallback.
#[derive(Debug, Default)]
pub struct VoiceBridge;

impl VoiceBridge {
    /// Creates a bridge instance.
    pub fn new() -> Self {
        Self
    }

    /// Starts voice for `platform` (stub always succeeds).
    pub fn start_voice(
        &self,
        platform: VoicePlatform,
    ) -> Result<&'static str, Infallible> {
        Ok(match platform {
            VoicePlatform::PlayStation => "native",
            VoicePlatform::Desktop => "vivox",
        })
    }
}
