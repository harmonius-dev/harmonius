//! In-process crash handler wiring (`R-14.4.1`).

use std::fs::OpenOptions;
use std::io;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

use memmap2::{MmapMut, MmapOptions};
use thiserror::Error;
use url::Url;

const SHARED_MAGIC: u32 = 0x314d_5248; // 'HRM1'
const SHARED_FILE_LEN: u64 = 512;

/// Opaque player identifier carried into crash metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UserId(pub u128);

/// Configuration for installing the crash handler.
#[derive(Clone, Debug)]
pub struct CrashConfig {
    /// Directory for local dump files.
    pub dump_dir: PathBuf,
    /// Maximum retained `.mdmp` files (rotation).
    pub rotation_cap: u32,
    /// Path to the `harmonius_crashmon` binary.
    pub monitor_binary: PathBuf,
    /// Upload endpoint for dumps (reserved for monitor-side uploads).
    #[allow(dead_code)]
    pub upload_url: Url,
    /// Whether to scrub PII fields in metadata (reserved for monitor-side redaction).
    #[allow(dead_code)]
    pub anonymize: bool,
    /// Shared memory backing file used for breadcrumbs and lightweight metadata.
    pub shared_state_path: PathBuf,
}

/// Handle for writing crash breadcrumbs into shared memory.
pub struct BreadcrumbHandle<'a> {
    slot: &'a mut [u8],
}

impl BreadcrumbHandle<'_> {
    /// Writes `value` into the shared breadcrumb slot.
    pub fn store(&mut self, value: u32) {
        self.slot.copy_from_slice(&value.to_le_bytes());
    }
}

/// Installed crash handler state.
pub struct CrashHandler {
    child: Child,
    mmap: MmapMut,
}

/// Errors while installing the crash handler.
#[derive(Debug, Error)]
pub enum CrashInstallError {
    /// Underlying IO failure.
    #[error("crash handler install failed: {0}")]
    Io(#[from] io::Error),
    /// Failed to spawn the monitor process.
    #[error("failed to spawn monitor binary: {0}")]
    Spawn(#[source] io::Error),
}

impl CrashHandler {
    /// Installs the handler and starts the monitor process.
    pub fn install(config: CrashConfig) -> Result<Self, CrashInstallError> {
        std::fs::create_dir_all(&config.dump_dir)?;
        if let Some(parent) = config.shared_state_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&config.shared_state_path)?;
        file.set_len(SHARED_FILE_LEN)?;
        // SAFETY: `set_len` above ensures the file is large enough for all fixed offsets used in
        // this module (<= `SHARED_FILE_LEN`).
        let mut mmap = unsafe { MmapOptions::new().map_mut(&file)? };
        mmap[0..4].copy_from_slice(&SHARED_MAGIC.to_le_bytes());

        let child = Command::new(&config.monitor_binary)
            .arg("idle")
            .stdin(Stdio::piped())
            .spawn()
            .map_err(CrashInstallError::Spawn)?;

        Ok(Self { child, mmap })
    }

    /// Returns the OS pid for the monitor child.
    #[must_use]
    pub fn monitor_pid(&self) -> u32 {
        self.child.id()
    }

    /// Returns a handle for writing breadcrumbs.
    pub fn breadcrumb_buffer(&mut self) -> BreadcrumbHandle<'_> {
        BreadcrumbHandle {
            slot: &mut self.mmap[4..8],
        }
    }

    /// Updates the scene name visible to the monitor through shared memory.
    pub fn update_scene(&mut self, name: &str) -> io::Result<()> {
        let bytes = name.as_bytes();
        let len = (bytes.len().min(256)) as u32;
        self.mmap[8..12].copy_from_slice(&len.to_le_bytes());
        self.mmap[12..268].fill(0);
        self.mmap[12..12 + len as usize].copy_from_slice(&bytes[..len as usize]);
        Ok(())
    }

    /// Updates the player id visible to the monitor through shared memory.
    pub fn update_player(&mut self, id: UserId) -> io::Result<()> {
        self.mmap[268..272].copy_from_slice(&1u32.to_le_bytes());
        self.mmap[272..288].copy_from_slice(&id.0.to_le_bytes());
        Ok(())
    }

    /// Stops the monitor process.
    pub fn uninstall(mut self) -> io::Result<()> {
        let _ = self.child.kill();
        let _ = self.child.wait()?;
        Ok(())
    }
}
