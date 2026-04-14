//! Headless dedicated server surface (TC-8.5.8.*).

use super::types::{HeadlessConfig, SubsystemTag};

#[derive(Clone, Debug)]
pub struct HeadlessServer {
    cfg: HeadlessConfig,
    players: u32,
    inited: bool,
}

impl HeadlessServer {
    pub fn new(cfg: HeadlessConfig) -> Self {
        Self {
            cfg,
            players: 0,
            inited: false,
        }
    }

    pub fn init(&mut self) -> Result<(), &'static str> {
        self.inited = true;
        Ok(())
    }

    pub fn subsystems(&self) -> Vec<SubsystemTag> {
        vec![SubsystemTag::Simulation, SubsystemTag::NetworkStack]
    }

    pub fn health_json(&self) -> String {
        format!(
            r#"{{"status":"ok","tick_hz":{},"players":{}}}"#,
            self.cfg.tick_hz, self.players
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.5.8.1
    #[test]
    fn test_headless_no_gpu_init() {
        let mut hs = HeadlessServer::new(HeadlessConfig {
            tick_hz: 30,
            max_players: 64,
        });
        assert!(hs.init().is_ok());
        let tags = hs.subsystems();
        assert!(!tags.contains(&SubsystemTag::Renderer));
        assert!(!tags.contains(&SubsystemTag::WindowSystem));
        assert!(!tags.contains(&SubsystemTag::AudioMixer));
        assert!(!tags.contains(&SubsystemTag::InputDevice));
    }

    /// TC-8.5.8.2
    #[test]
    fn test_headless_health_endpoint() {
        let hs = HeadlessServer::new(HeadlessConfig {
            tick_hz: 30,
            max_players: 64,
        });
        let body = hs.health_json();
        assert!(body.contains("\"tick_hz\":30"));
        assert!(body.contains("\"status\":\"ok\""));
        assert!(body.contains("\"players\":"));
    }
}
