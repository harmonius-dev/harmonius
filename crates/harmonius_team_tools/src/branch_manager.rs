//! Branch-per-feature workflow helpers (**TC-15.10.4.1**, R-15.10.4).

use std::path::Path;

use git2::build::CheckoutBuilder;
use git2::Repository;

use crate::git_status::VcError;

/// High-level branch operations on an open repository (see design `BranchManager`).
pub struct BranchManager {
    repo: Repository,
}

impl BranchManager {
    /// Open an existing repository for branch operations.
    pub fn open(path: &Path) -> Result<Self, VcError> {
        Ok(Self {
            repo: Repository::open(path)?,
        })
    }

    /// Create `name` at the tip of local branch `from` (for example `main`).
    pub fn create_branch(&self, name: &str, from: &str) -> Result<(), VcError> {
        let from_ref = format!("refs/heads/{from}");
        let commit = self
            .repo
            .find_reference(&from_ref)?
            .peel_to_commit()?;
        self.repo.branch(name, &commit, false)?;
        Ok(())
    }

    /// Check out local branch `name` and move `HEAD`.
    pub fn switch_branch(&self, name: &str) -> Result<(), VcError> {
        let refname = format!("refs/heads/{name}");
        let commit = self
            .repo
            .find_reference(&refname)?
            .peel_to_commit()?;
        let tree = commit.tree()?;
        let mut checkout = CheckoutBuilder::default();
        self.repo
            .checkout_tree(tree.as_object(), Some(&mut checkout))?;
        self.repo.set_head(&refname)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::process::Command;

    use tempfile::TempDir;

    use super::*;

    fn init_repo_with_commit(dir: &Path) -> String {
        let st = Command::new("git")
            .args(["init", "-q"])
            .current_dir(dir)
            .status()
            .expect("git init");
        assert!(st.success());
        let st = Command::new("git")
            .args(["config", "user.email", "tc@harmonius.invalid"])
            .current_dir(dir)
            .status()
            .expect("config");
        assert!(st.success());
        let st = Command::new("git")
            .args(["config", "user.name", "TC"])
            .current_dir(dir)
            .status()
            .expect("config");
        assert!(st.success());
        fs::write(dir.join("README.md"), "# root\n").expect("write");
        let st = Command::new("git")
            .args(["add", "README.md"])
            .current_dir(dir)
            .status()
            .expect("add");
        assert!(st.success());
        let st = Command::new("git")
            .args(["commit", "-q", "-m", "init"])
            .current_dir(dir)
            .status()
            .expect("commit");
        assert!(st.success());
        let out = Command::new("git")
            .args(["branch", "--show-current"])
            .current_dir(dir)
            .output()
            .expect("branch");
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    }

    /// **TC-15.10.4.1** — Branch create and switch updates HEAD correctly.
    #[test]
    fn tc_15_10_4_1_branch_create_and_switch() {
        let dir = TempDir::new().expect("tempdir");
        let default_branch = init_repo_with_commit(dir.path());
        let mgr = BranchManager::open(dir.path()).expect("open");
        mgr.create_branch("feature", &default_branch)
            .expect("create branch");
        mgr.switch_branch("feature").expect("switch");

        let repo = Repository::open(dir.path()).expect("re-open");
        let head = repo.head().expect("head");
        assert_eq!(head.shorthand().expect("name"), "feature");
    }
}
