//! GPU pass breadcrumbs for crash triage (`R-14.4.6`).

use std::sync::Mutex;

/// Opaque GPU device handle (stub until rendering wires a device).
#[derive(Debug, Default)]
pub struct GpuDevice;

impl GpuDevice {
    /// Constructs a stub device for unit tests.
    pub const fn new() -> Self {
        Self
    }
}

/// Identifies a render pass in breadcrumb buffers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PassId(pub u32);

/// Tracks nested GPU pass markers for crash reports.
pub struct GpuBreadcrumbs {
    inner: Mutex<BreadcrumbState>,
}

#[derive(Debug)]
struct BreadcrumbState {
    stack: Vec<PassId>,
    completed: Vec<PassId>,
    last_completed: Option<PassId>,
}

impl GpuBreadcrumbs {
    /// Creates breadcrumbs bound to `device` (currently unused on stub hosts).
    pub fn new(_device: &GpuDevice) -> Result<Self, super::CrashError> {
        Ok(Self {
            inner: Mutex::new(BreadcrumbState {
                stack: Vec::new(),
                completed: Vec::new(),
                last_completed: None,
            }),
        })
    }

    /// Marks the beginning of `pass_id`.
    pub fn begin_pass(&self, pass_id: PassId) {
        let mut g = self.inner.lock().expect("breadcrumb mutex poisoned");
        g.stack.push(pass_id);
    }

    /// Marks completion of `pass_id`.
    pub fn end_pass(&self, pass_id: PassId) {
        let mut g = self.inner.lock().expect("breadcrumb mutex poisoned");
        if g.stack.last() == Some(&pass_id) {
            g.stack.pop();
        }
        g.completed.push(pass_id);
        g.last_completed = Some(pass_id);
    }

    /// Returns the last fully completed pass.
    pub fn read_last_completed(&self) -> Option<PassId> {
        self.inner
            .lock()
            .expect("breadcrumb mutex poisoned")
            .last_completed
    }

    /// Serializes completed pass ids for crash bundles.
    pub fn serialize_for_crash_report(&self) -> Vec<u8> {
        let g = self.inner.lock().expect("breadcrumb mutex poisoned");
        let mut out = Vec::new();
        for p in &g.completed {
            out.extend_from_slice(&p.0.to_le_bytes());
        }
        out
    }
}
