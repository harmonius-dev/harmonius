//! Build-server queue stubs (`R-14.8`).

use std::cmp::Ordering;
use std::collections::VecDeque;
use std::sync::Mutex;

/// Build job identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BuildJobId(pub u64);

/// Target console platform for a remote build.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ConsolePlatform {
    /// Sony PlayStation 5.
    Ps5,
    /// Microsoft Xbox Series.
    XboxSeries,
    /// Nintendo Switch.
    Switch,
}

/// Priority hint for scheduling.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuildPriority {
    /// Routine developer iteration.
    Routine,
    /// Certification / submission build.
    Certification,
}

/// Queued build description.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuildJob {
    /// Stable job id.
    pub id: BuildJobId,
    /// Target platform.
    pub platform: ConsolePlatform,
    /// Scheduling priority.
    pub priority: BuildPriority,
    /// Owning team id for isolation checks.
    pub team: String,
}

/// Lifecycle states for a build job.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuildStatus {
    /// Waiting in queue.
    Queued,
    /// Running on a worker.
    Running,
    /// Finished successfully.
    Succeeded,
    /// Access denied for cross-team reads.
    AccessDenied,
    /// Artifact no longer present.
    NotFound,
}

/// In-memory build coordinator for tests.
#[derive(Debug, Default)]
pub struct BuildServer {
    queue: Mutex<VecDeque<BuildJob>>,
    artifacts: Mutex<Vec<(BuildJobId, String, String)>>,
}

impl BuildServer {
    /// Creates an empty coordinator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enqueues `job` respecting priority ordering.
    pub fn enqueue(&self, job: BuildJob) {
        let mut q = self.queue.lock().expect("build mutex poisoned");
        if matches!(job.priority, BuildPriority::Certification) {
            q.push_front(job);
        } else {
            q.push_back(job);
        }
    }

    /// Peeks the next job without dequeuing.
    pub fn peek_next(&self) -> Option<BuildJob> {
        self.queue.lock().expect("build mutex poisoned").front().cloned()
    }

    /// Records a finished artifact URL + signature for `id`.
    pub fn complete_job(&self, id: BuildJobId, url: String, sig: String) {
        self.artifacts
            .lock()
            .expect("build mutex poisoned")
            .push((id, url, sig));
    }

    /// Returns status for `id` (stub lookup).
    pub fn status(&self, id: BuildJobId) -> BuildStatus {
        let arts = self.artifacts.lock().expect("build mutex poisoned");
        if arts.iter().any(|(i, _, _)| *i == id) {
            BuildStatus::Succeeded
        } else {
            BuildStatus::Queued
        }
    }

    /// Fetches artifact metadata for `id` when `team` owns the job.
    pub fn artifact_for_team(
        &self,
        id: BuildJobId,
        team: &str,
        job_team: &str,
    ) -> Result<String, BuildStatus> {
        if team != job_team {
            return Err(BuildStatus::AccessDenied);
        }
        let arts = self.artifacts.lock().expect("build mutex poisoned");
        arts.iter()
            .find(|(i, _, _)| *i == id)
            .map(|(_, u, _)| u.clone())
            .ok_or(BuildStatus::NotFound)
    }
}

/// Sorts jobs so certification builds run before routine builds.
pub fn compare_priority(a: BuildPriority, b: BuildPriority) -> Ordering {
    match (a, b) {
        (BuildPriority::Certification, BuildPriority::Routine) => Ordering::Less,
        (BuildPriority::Routine, BuildPriority::Certification) => Ordering::Greater,
        _ => Ordering::Equal,
    }
}
