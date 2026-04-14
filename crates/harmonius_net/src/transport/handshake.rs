//! Application-layer authentication after QUIC (Phase 2 in the design).
//!
//! Anti-replay keeps the last [`MAX_NONCES`] successful nonces. Older nonces can be reused once
//! evicted; production code should pair this with TLS 1.3 inside QUIC and short session lifetimes.

use std::collections::{HashSet, VecDeque};

use super::NetworkError;

/// Upper bound on stored nonces after successful auth (memory cap).
pub const MAX_NONCES: usize = 8192;

/// Session credential presented by the client.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionToken(pub String);

/// Client hello carrying token, nonce, and wire protocol version.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Handshake {
    /// Session token bytes interpreted as UTF-8 in tests.
    pub token: SessionToken,
    /// Unique per-connection client nonce (replay window key).
    pub client_nonce: u64,
    /// Application wire version.
    pub version: u32,
}

/// Server-side handshake processing with a bounded replay window.
#[derive(Debug, Default)]
pub struct HandshakeServer {
    nonce_order: VecDeque<u64>,
    seen_nonces: HashSet<u64>,
}

impl HandshakeServer {
    /// Creates an empty anti-replay table.
    pub fn new() -> Self {
        Self::default()
    }

    /// Validates a handshake. `valid_token` must match `token.0` for success.
    ///
    /// On success records `client_nonce` and returns `Ok(player_id)`.
    /// Replayed nonces return [`NetworkError::ReplayDetected`].
    pub fn accept(&mut self, msg: &Handshake, valid_token: &str) -> Result<u32, NetworkError> {
        if self.seen_nonces.contains(&msg.client_nonce) {
            return Err(NetworkError::ReplayDetected);
        }
        if msg.token.0.as_str() != valid_token {
            return Err(NetworkError::AuthRejected {
                reason: "invalid token".into(),
            });
        }
        if msg.version == 0 {
            return Err(NetworkError::AuthRejected {
                reason: "unsupported version".into(),
            });
        }
        while self.nonce_order.len() >= MAX_NONCES {
            if let Some(old) = self.nonce_order.pop_front() {
                self.seen_nonces.remove(&old);
            }
        }
        self.nonce_order.push_back(msg.client_nonce);
        self.seen_nonces.insert(msg.client_nonce);
        Ok(1)
    }
}

/// Alias used by tests mirroring the design doc wording.
pub type HandshakeError = NetworkError;

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.1.1.1 — valid token authenticates and returns a player id.
    #[test]
    fn test_handshake_authenticates() {
        let mut srv = HandshakeServer::new();
        let hs = Handshake {
            token: SessionToken("valid".into()),
            client_nonce: 42,
            version: 1,
        };
        let player = srv.accept(&hs, "valid").expect("auth");
        assert_eq!(player, 1);
    }

    /// TC-8.1.1.2 — verbatim replay of the same nonce is rejected.
    #[test]
    fn test_handshake_replay_rejected() {
        let mut srv = HandshakeServer::new();
        let hs = Handshake {
            token: SessionToken("valid".into()),
            client_nonce: 7,
            version: 1,
        };
        srv.accept(&hs, "valid").unwrap();
        let err = srv.accept(&hs, "valid").unwrap_err();
        assert_eq!(err, NetworkError::ReplayDetected);
    }
}
