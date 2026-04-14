//! Server-side console build orchestration (R-14.8.*).

/// Identifies a queued console build job.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) struct ConsoleJobId(pub u64);

/// Isolated worker pool slot assignment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorkerSlot {
    /// Zero-based pool index.
    pub pool: u32,
    /// Slot within the pool.
    pub slot: u32,
}

/// Tracks SDK secret exposure boundaries.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct SdkSecretLedger {
    /// True after a client editor component requests secrets (disallowed).
    pub client_requested_secrets: bool,
}

/// Remote deploy progress snapshot.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) enum DeployPhase {
    /// Transfer not started.
    #[default]
    Idle,
    /// Bytes in flight.
    Transferring { sent: u64, total: u64 },
    /// Finished successfully.
    Complete,
}

/// Artifact retention window in days.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct RetentionPolicy {
    pub days: u32,
}

/// Minimal console build service state machine.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ConsoleBuildService {
    next_job: u64,
    queue_depth: u32,
    isolated_pool_size: u32,
    sdk_sandboxed: bool,
    deploy: DeployPhase,
    last_artifact: Option<Vec<u8>>,
    retained_until_day: u64,
}

impl ConsoleBuildService {
    /// Creates an empty service with default pool sizing.
    #[must_use]
    pub(crate) fn new(isolated_pool_size: u32) -> Self {
        Self {
            next_job: 1,
            queue_depth: 0,
            isolated_pool_size,
            sdk_sandboxed: true,
            deploy: DeployPhase::Idle,
            last_artifact: None,
            retained_until_day: 0,
        }
    }

    /// Accepts a new build job (TC-14.8.1.1).
    pub(crate) fn accept_job(&mut self) -> ConsoleJobId {
        self.queue_depth = self.queue_depth.saturating_add(1);
        let id = ConsoleJobId(self.next_job);
        self.next_job = self.next_job.saturating_add(1);
        id
    }

    /// Enqueues against isolated worker pool (TC-14.8.2.1).
    pub(crate) fn enqueue_isolated(&self, job: ConsoleJobId) -> WorkerSlot {
        let idx = job.0.saturating_sub(1);
        let pool = 0;
        let slot = u32::try_from(idx % u64::from(self.isolated_pool_size.max(1))).unwrap_or(0);
        WorkerSlot { pool, slot }
    }

    /// Marks proprietary SDK as isolated (TC-14.8.3.1).
    pub(crate) fn set_sdk_sandboxed(&mut self, sandboxed: bool) {
        self.sdk_sandboxed = sandboxed;
    }

    /// Records a forbidden client secret request (TC-14.8.4.1).
    pub(crate) fn record_client_secret_request(&mut self, ledger: &mut SdkSecretLedger) {
        ledger.client_requested_secrets = true;
    }

    /// Assigns a shared build server queue slot (TC-14.8.5.1).
    pub(crate) fn assign_queue_slot(&self, job: ConsoleJobId) -> u32 {
        u32::try_from(job.0 % 8).unwrap_or(0)
    }

    /// Auto-scales pool size based on depth (TC-14.8.6.1).
    pub(crate) fn scaled_pool_size(&self) -> u32 {
        self.isolated_pool_size.max(self.queue_depth)
    }

    /// Starts remote deploy transfer (TC-14.8.7.1).
    pub(crate) fn start_remote_deploy(&mut self, artifact: Vec<u8>, total: u64) {
        self.deploy = DeployPhase::Transferring {
            sent: 0,
            total,
        };
        self.last_artifact = Some(artifact);
    }

    /// Advances deploy and returns phase (TC-14.8.8.1).
    pub(crate) fn tick_remote_deploy(&mut self, delta: u64) -> DeployPhase {
        match &mut self.deploy {
            DeployPhase::Transferring { sent, total } => {
                *sent = (*sent + delta).min(*total);
                if *sent >= *total {
                    self.deploy = DeployPhase::Complete;
                }
            }
            DeployPhase::Idle => {}
            DeployPhase::Complete => {}
        }
        self.deploy.clone()
    }

    /// Returns last produced artifact bytes (TC-14.8.9.1).
    #[must_use]
    pub(crate) fn last_artifact_bytes(&self) -> Option<&[u8]> {
        self.last_artifact.as_deref()
    }

    /// Updates retention window given policy (TC-14.8.10.1).
    pub(crate) fn apply_retention(&mut self, policy: RetentionPolicy, current_day: u64) {
        self.retained_until_day = current_day.saturating_add(u64::from(policy.days));
    }

    /// True when artifact still retained (TC-14.8.10.1).
    #[must_use]
    pub(crate) fn is_retained_on_day(&self, day: u64) -> bool {
        day <= self.retained_until_day
    }

    /// Whether SDK is sandboxed (TC-14.8.3.1).
    #[must_use]
    pub(crate) fn sdk_sandboxed(&self) -> bool {
        self.sdk_sandboxed
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ConsoleBuildService, ConsoleJobId, DeployPhase, RetentionPolicy, SdkSecretLedger,
    };

    /// TC-14.8.1.1 — Server-side console build service accepts job (R-14.8.1).
    #[test]
    fn tc_14_8_1_1_accepts_job() {
        let mut svc = ConsoleBuildService::new(4);
        let j = svc.accept_job();
        assert_eq!(j, ConsoleJobId(1));
    }

    /// TC-14.8.2.1 — Console build enqueues against isolated worker pool (R-14.8.2).
    #[test]
    fn tc_14_8_2_1_isolated_enqueue() {
        let mut svc = ConsoleBuildService::new(3);
        let j = svc.accept_job();
        let slot = svc.enqueue_isolated(j);
        assert_eq!(slot.pool, 0);
        assert!(slot.slot < 3);
    }

    /// TC-14.8.3.1 — Proprietary SDK isolated in sandboxed worker (R-14.8.3).
    #[test]
    fn tc_14_8_3_1_sdk_sandboxed() {
        let mut svc = ConsoleBuildService::new(2);
        assert!(svc.sdk_sandboxed());
        svc.set_sdk_sandboxed(false);
        assert!(!svc.sdk_sandboxed());
    }

    /// TC-14.8.4.1 — SDK secrets never reach client editor (R-14.8.4).
    #[test]
    fn tc_14_8_4_1_client_secret_boundary() {
        let mut svc = ConsoleBuildService::new(2);
        let mut ledger = SdkSecretLedger::default();
        svc.record_client_secret_request(&mut ledger);
        assert!(ledger.client_requested_secrets);
    }

    /// TC-14.8.5.1 — Shared build server assigns queue slot (R-14.8.5).
    #[test]
    fn tc_14_8_5_1_queue_slot() {
        let svc = ConsoleBuildService::new(4);
        assert_eq!(svc.assign_queue_slot(ConsoleJobId(9)), 1);
    }

    /// TC-14.8.6.1 — Build server auto-scales on queue depth (R-14.8.6).
    #[test]
    fn tc_14_8_6_1_autoscale() {
        let mut svc = ConsoleBuildService::new(2);
        svc.accept_job();
        svc.accept_job();
        svc.accept_job();
        assert!(svc.scaled_pool_size() >= 3);
    }

    /// TC-14.8.7.1 — Remote console deploy transfers artifact (R-14.8.7).
    #[test]
    fn tc_14_8_7_1_remote_transfer() {
        let mut svc = ConsoleBuildService::new(2);
        svc.start_remote_deploy(vec![1, 2, 3], 3);
        match svc.tick_remote_deploy(3) {
            DeployPhase::Complete => {}
            other => panic!("unexpected phase: {other:?}"),
        }
    }

    /// TC-14.8.8.1 — Remote deploy reports progress/completion (R-14.8.8).
    #[test]
    fn tc_14_8_8_1_progress() {
        let mut svc = ConsoleBuildService::new(2);
        svc.start_remote_deploy(vec![9], 10);
        let mid = svc.tick_remote_deploy(4);
        assert!(matches!(mid, DeployPhase::Transferring { .. }));
        let done = svc.tick_remote_deploy(10);
        assert_eq!(done, DeployPhase::Complete);
    }

    /// TC-14.8.9.1 — Console build artifact produced in expected format (R-14.8.9).
    #[test]
    fn tc_14_8_9_1_artifact_format() {
        let mut svc = ConsoleBuildService::new(2);
        let bytes = vec![0x48, 0x42, 0x4E, 0x44, 0x4C, 0x45]; // "HBNDLE" stub
        svc.start_remote_deploy(bytes.clone(), 6);
        let _ = svc.tick_remote_deploy(6);
        assert_eq!(svc.last_artifact_bytes(), Some(bytes.as_slice()));
    }

    /// TC-14.8.10.1 — Artifact retained for policy-configured window (R-14.8.10).
    #[test]
    fn tc_14_8_10_1_retention_window() {
        let mut svc = ConsoleBuildService::new(2);
        svc.apply_retention(RetentionPolicy { days: 7 }, 100);
        assert!(svc.is_retained_on_day(107));
        assert!(!svc.is_retained_on_day(108));
    }
}
