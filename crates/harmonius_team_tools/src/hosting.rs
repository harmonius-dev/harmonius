//! Multi-provider Git hosting detection (**TC-15.10.8.1**, R-15.10.8).

/// Recognized hosting backends for editor PR flows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostingKind {
    /// `github.com` remotes.
    GitHub,
    /// `gitlab.com` remotes (self-managed hosts fall under [`HostingKind::Other`] here).
    GitLab,
    /// Any URL that does not match a built-in pattern.
    Other,
}

/// Best-effort provider detection from `git remote` URLs.
pub fn detect_hosting(remote_url: &str) -> HostingKind {
    let lower = remote_url.to_ascii_lowercase();
    if lower.contains("github.com") {
        HostingKind::GitHub
    } else if lower.contains("gitlab.com") {
        HostingKind::GitLab
    } else {
        HostingKind::Other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **TC-15.10.8.1** — Hosting provider detect identifies GitHub/GitLab/etc.
    #[test]
    fn tc_15_10_8_1_detects_common_hosts() {
        assert_eq!(
            detect_hosting("https://github.com/org/repo.git"),
            HostingKind::GitHub
        );
        assert_eq!(
            detect_hosting("git@GitLab.com:group/project.git"),
            HostingKind::GitLab
        );
        assert_eq!(
            detect_hosting("https://git.example.com/org/repo.git"),
            HostingKind::Other
        );
    }
}
