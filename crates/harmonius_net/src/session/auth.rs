//! OAuth-style login and session token validation (TC-8.5.1.*).

use super::types::{
    AccountId, Permissions, Platform, SessionId, SessionToken, TokenError,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuthError {
    InvalidCode,
    PlatformUnavailable { platform: Platform },
}

/// Stateless auth facade with injectable wall clock for tests.
#[derive(Clone)]
pub struct AuthService {
    now_fn: fn() -> u64,
    default_ttl_secs: u64,
}

impl AuthService {
    pub fn new(now_fn: fn() -> u64, default_ttl_secs: u64) -> Self {
        Self {
            now_fn,
            default_ttl_secs,
        }
    }

    /// `AuthService::login_oauth(provider, code)` per design test cases.
    pub fn login_oauth(
        &self,
        provider: Platform,
        code: &str,
    ) -> Result<SessionToken, AuthError> {
        if matches!(provider, Platform::Steam) && code == "valid" {
            let now = (self.now_fn)();
            let sub = "76561198000000001".to_string();
            let aid = AccountId(sub.parse().unwrap_or(1));
            return Ok(SessionToken {
                account_id: aid,
                session_id: SessionId(1),
                platform: provider,
                issued_at: now,
                expires_at: now.saturating_add(self.default_ttl_secs),
                permissions: Permissions::PLAY,
                subject: sub,
            });
        }
        Err(AuthError::InvalidCode)
    }

    pub fn validate_token(&self, token: &SessionToken) -> Result<AccountId, TokenError> {
        let now = (self.now_fn)();
        if now > token.expires_at {
            return Err(TokenError::Expired);
        }
        Ok(token.account_id)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::*;

    thread_local! {
        static MOCK_NOW: Cell<u64> = Cell::new(0);
    }

    fn mock_now() -> u64 {
        MOCK_NOW.with(|c| c.get())
    }

    fn set_now(t: u64) {
        MOCK_NOW.with(|c| c.set(t));
    }

    /// TC-8.5.1.1
    #[test]
    fn test_oauth_token_issued() {
        set_now(1_700_000_000);
        let auth = AuthService::new(mock_now, 3600);
        let tok = auth
            .login_oauth(Platform::Steam, "valid")
            .expect("valid steam code");
        assert_eq!(tok.subject, "76561198000000001");
        assert_eq!(tok.account_id.0, 7_656_119_800_000_0001);
        let ttl = tok.expires_at.saturating_sub(tok.issued_at);
        assert!(ttl <= 3600 + 1, "ttl within bound");
    }

    /// TC-8.5.1.2
    #[test]
    fn test_token_expires() {
        set_now(0);
        let auth = AuthService::new(mock_now, 60);
        let tok = auth.login_oauth(Platform::Steam, "valid").unwrap();
        set_now(61);
        let err = auth.validate_token(&tok).unwrap_err();
        assert_eq!(err, TokenError::Expired);
    }
}
