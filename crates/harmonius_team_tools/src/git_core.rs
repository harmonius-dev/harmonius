//! Embedded Git operations via libgit2 (**TC-15.10.1.2**, **TC-15.10.1.3**, R-15.10.1).

use std::path::{Path, PathBuf};

use git2::{
    Commit, Cred, CredentialType, PushOptions, RemoteCallbacks, Repository, Signature, Status,
    StatusOptions,
};

use crate::git_status::{FileStatus, StatusKind, VcError};

/// Opaque Git commit identity (20-byte SHA-1 object id).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CommitId(pub [u8; 20]);

/// Embedded repository handle backed by libgit2.
pub struct GitCore {
    repo: Repository,
}

impl std::fmt::Debug for GitCore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GitCore").finish_non_exhaustive()
    }
}

impl GitCore {
    /// Open an existing repository at `path`.
    pub fn open(path: &Path) -> Result<Self, VcError> {
        let repo = Repository::open(path)?;
        Ok(Self { repo })
    }

    /// Working tree and index status snapshot.
    pub fn status(&self) -> Result<Vec<FileStatus>, VcError> {
        let mut opts = StatusOptions::new();
        opts.include_untracked(true).recurse_untracked_dirs(true);
        let statuses = self.repo.statuses(Some(&mut opts))?;
        let mut out = Vec::with_capacity(statuses.len());
        for i in 0..statuses.len() {
            let entry = statuses.get(i).ok_or_else(|| VcError::Git {
                message: "status entry index out of range".to_string(),
            })?;
            let flags = entry.status();
            let path = PathBuf::from(entry.path().ok_or(VcError::InvalidPath)?);
            out.push(FileStatus {
                index_status: map_index_column(flags),
                is_lfs: false,
                path,
                worktree_status: map_worktree_column(flags),
            });
        }
        out.sort_by(|a, b| a.path.cmp(&b.path));
        Ok(out)
    }

    /// Stage the given paths (repository-relative).
    pub fn stage(&self, paths: &[&Path]) -> Result<(), VcError> {
        let mut index = self.repo.index()?;
        for path in paths {
            index.add_path(path)?;
        }
        index.write()?;
        Ok(())
    }

    /// Create a commit from the current index tree.
    pub fn commit(&self, message: &str) -> Result<CommitId, VcError> {
        let mut index = self.repo.index()?;
        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;
        let sig = Signature::now("Harmonius Team Tools", "devnull@harmonius.invalid")?;

        let oid = if let Some(parent) = head_parent(&self.repo)? {
            self.repo
                .commit(Some("HEAD"), &sig, &sig, message, &tree, &[&parent])?
        } else {
            self.repo
                .commit(Some("HEAD"), &sig, &sig, message, &tree, &[])?
        };

        let bytes: [u8; 20] = oid.as_bytes().try_into().map_err(|_| VcError::Git {
            message: "commit oid is not 20 bytes".to_string(),
        })?;
        Ok(CommitId(bytes))
    }

    /// Push `branch` to `remote` using the default credential helpers where applicable.
    ///
    /// Local integration tests use a `file://` bare remote and therefore do not require
    /// interactive credential prompts.
    pub fn push(&self, remote: &str, branch: &str) -> Result<(), VcError> {
        let mut remote = self.repo.find_remote(remote)?;
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, _username_from_url, allowed_types| {
            if allowed_types.contains(CredentialType::USER_PASS_PLAINTEXT) {
                Cred::userpass_plaintext("git", "UNUSED")
            } else {
                Cred::default()
            }
        });
        let mut opts = PushOptions::new();
        opts.remote_callbacks(callbacks);
        let refspec = format!("refs/heads/{branch}:refs/heads/{branch}");
        remote.push(&[&refspec], Some(&mut opts))?;
        Ok(())
    }
}

fn head_parent(repo: &Repository) -> Result<Option<Commit<'_>>, VcError> {
    let head = match repo.head() {
        Ok(h) => h,
        Err(e) if is_unborn_head(&e) => return Ok(None),
        Err(e) => return Err(VcError::from(e)),
    };
    if !head.is_branch() {
        return Ok(None);
    }
    let oid = head.target().ok_or_else(|| VcError::Git {
        message: "HEAD has no target oid".to_string(),
    })?;
    Ok(Some(repo.find_commit(oid)?))
}

fn is_unborn_head(err: &git2::Error) -> bool {
    err.code() == git2::ErrorCode::UnbornBranch
        || err.code() == git2::ErrorCode::NotFound
        || err.message().contains("reference is not a branch")
}

fn map_index_column(s: Status) -> StatusKind {
    if s.is_conflicted() {
        return StatusKind::Unmerged;
    }
    if s.is_ignored() {
        return StatusKind::Ignored;
    }
    if s.is_wt_new() && !index_any(s) {
        return StatusKind::Untracked;
    }
    if s.is_index_new() {
        StatusKind::Added
    } else if s.is_index_modified() || s.is_index_typechange() {
        StatusKind::Modified
    } else if s.is_index_deleted() {
        StatusKind::Deleted
    } else if s.is_index_renamed() {
        StatusKind::Renamed
    } else {
        StatusKind::Unmodified
    }
}

fn map_worktree_column(s: Status) -> StatusKind {
    if s.is_conflicted() {
        return StatusKind::Unmerged;
    }
    if s.is_ignored() {
        return StatusKind::Ignored;
    }
    if s.is_wt_new() && !index_any(s) {
        return StatusKind::Untracked;
    }
    if s.is_wt_modified() || s.is_wt_typechange() {
        StatusKind::Modified
    } else if s.is_wt_deleted() {
        StatusKind::Deleted
    } else if s.is_wt_renamed() {
        StatusKind::Renamed
    } else {
        StatusKind::Unmodified
    }
}

fn index_any(s: Status) -> bool {
    s.is_index_new()
        || s.is_index_modified()
        || s.is_index_deleted()
        || s.is_index_renamed()
        || s.is_index_typechange()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::process::Command;

    use tempfile::TempDir;

    use super::*;

    fn init_repo(dir: &Path) {
        let status = Command::new("git")
            .args(["init", "-q"])
            .current_dir(dir)
            .status()
            .expect("git init");
        assert!(status.success());
        let status = Command::new("git")
            .args(["config", "user.email", "tc@harmonius.invalid"])
            .current_dir(dir)
            .status()
            .expect("git config");
        assert!(status.success());
        let status = Command::new("git")
            .args(["config", "user.name", "TC"])
            .current_dir(dir)
            .status()
            .expect("git config");
        assert!(status.success());
    }

    /// **TC-15.10.1.2** — Stage and commit produce a valid commit object via libgit2.
    #[test]
    fn tc_15_10_1_2_stage_and_commit_round_trip() {
        let dir = TempDir::new().expect("tempdir");
        init_repo(dir.path());

        fs::write(dir.path().join("tracked.txt"), "hello").expect("write");

        let core = GitCore::open(dir.path()).expect("open");
        core.stage(&[Path::new("tracked.txt")]).expect("stage");
        let id = core.commit("tc: first commit").expect("commit");

        assert_ne!(id.0, [0u8; 20]);

        let repo = Repository::open(dir.path()).expect("re-open");
        let head = repo.head().expect("head").peel_to_commit().expect("commit");
        assert_eq!(head.id().as_bytes(), &id.0);
    }

    /// **TC-15.10.1.3** — Push updates a bare remote using libgit2 (no interactive credentials).
    #[test]
    fn tc_15_10_1_3_push_to_bare_remote() {
        let bare_root = TempDir::new().expect("bare tempdir");
        let work_root = TempDir::new().expect("work tempdir");

        let status = Command::new("git")
            .args(["init", "--bare", "-q"])
            .current_dir(bare_root.path())
            .status()
            .expect("git init --bare");
        assert!(status.success());

        let status = Command::new("git")
            .args(["clone", "-q"])
            .arg(bare_root.path())
            .arg(work_root.path())
            .status()
            .expect("git clone");
        assert!(status.success());

        fs::write(work_root.path().join("readme.md"), "# demo\n").expect("write");

        let core = GitCore::open(work_root.path()).expect("open");
        core.stage(&[Path::new("readme.md")]).expect("stage");
        core.commit("tc: commit for push").expect("commit");

        let repo = Repository::open(work_root.path()).expect("re-open for branch");
        let branch = repo
            .head()
            .expect("head")
            .shorthand()
            .expect("branch name")
            .to_string();

        core.push("origin", &branch).expect("push");

        let bare_repo = Repository::open_bare(bare_root.path()).expect("open bare");
        bare_repo
            .find_branch(&branch, git2::BranchType::Local)
            .expect("remote branch exists on bare");
    }
}
