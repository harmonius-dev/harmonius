//! Software skinning fake for headless failure-mode tests (TC-IR-5.3.F5).

use glam::Mat4;

/// Emitted when the skinning dispatch path fails before producing deformed matrices.
pub const WARN_SKINNING_DISPATCH_FAILED: &str = "GPU skinning dispatch failed; using bind pose";

/// Outcome of a single skinning dispatch attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SkinningDispatchStatus {
    /// GPU / compute path produced deformed matrices.
    Ok,
    /// Dispatch failed; policy selects bind pose.
    Failed,
}

/// Headless stand-in for the GPU skinning dispatch seam.
#[derive(Debug)]
pub struct SoftwareSkinningFake {
    fail_next_dispatch: bool,
}

impl SoftwareSkinningFake {
    /// Creates a fake that succeeds until [`Self::set_fail_next_dispatch`].
    #[must_use]
    pub const fn new() -> Self {
        Self {
            fail_next_dispatch: false,
        }
    }

    /// When set, the next [`Self::dispatch`] returns `Failed` once.
    pub fn set_fail_next_dispatch(&mut self, fail: bool) {
        self.fail_next_dispatch = fail;
    }

    /// Simulates submitting skinning work to the GPU queue.
    pub fn dispatch(&mut self) -> SkinningDispatchStatus {
        if self.fail_next_dispatch {
            self.fail_next_dispatch = false;
            SkinningDispatchStatus::Failed
        } else {
            SkinningDispatchStatus::Ok
        }
    }

    /// Resolves the matrix used for viewport display after a dispatch attempt.
    #[must_use]
    pub fn resolve_world_matrix(
        status: SkinningDispatchStatus,
        bind_pose: Mat4,
        animated: Mat4,
    ) -> Mat4 {
        match status {
            SkinningDispatchStatus::Ok => animated,
            SkinningDispatchStatus::Failed => bind_pose,
        }
    }

    /// Runs one dispatch and returns the resolved matrix plus warning text for failures.
    pub fn skin_world_matrix(
        &mut self,
        bind_pose: Mat4,
        animated: Mat4,
    ) -> (Mat4, Vec<&'static str>) {
        let status = self.dispatch();
        let matrix = Self::resolve_world_matrix(status, bind_pose, animated);
        let warnings = match status {
            SkinningDispatchStatus::Ok => Vec::new(),
            SkinningDispatchStatus::Failed => vec![WARN_SKINNING_DISPATCH_FAILED],
        };
        (matrix, warnings)
    }
}

impl Default for SoftwareSkinningFake {
    fn default() -> Self {
        Self::new()
    }
}
