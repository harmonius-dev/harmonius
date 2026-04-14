//! Main-thread-only dynamic library loading surface.

use std::marker::PhantomData;
use std::path::Path;
use std::sync::MutexGuard;

use crate::types::{DylibHandle, FormulaFnSlot};

/// Token proving caller is on the engine main thread (integration stub).
///
/// This type is intentionally `!Send` so worker snapshots cannot smuggle loaders across threads.
#[derive(Debug)]
pub struct MainThreadToken {
    _not_send: PhantomData<MutexGuard<'static, ()>>,
}

impl MainThreadToken {
    /// Creates a token in tests; production code should only call this from the real main thread.
    pub fn new_for_tests() -> Self {
        Self {
            _not_send: PhantomData,
        }
    }
}

/// Dynamic library loader restricted to the main thread.
///
/// ```compile_fail,E0277
/// use harmonius_scripting_data_tables_integration::loader::MainThreadDylibLoader;
/// fn assert_send<T: Send>() {}
/// fn _f() {
///     assert_send::<MainThreadDylibLoader>();
/// }
/// ```
#[derive(Debug)]
pub struct MainThreadDylibLoader {
    _not_send: PhantomData<MutexGuard<'static, ()>>,
}

impl MainThreadDylibLoader {
    /// Builds a loader handle that cannot cross worker threads.
    pub fn new(_token: &MainThreadToken) -> Self {
        Self {
            _not_send: PhantomData,
        }
    }

    /// Attempts to load a `.dylib` from disk (stubbed for CI portability).
    ///
    /// Real builds would call `dlopen` here; tests inject success or failure without I/O.
    pub fn load_stub(
        &mut self,
        _token: &MainThreadToken,
        path: &Path,
        version: u32,
    ) -> Result<DylibHandle, String> {
        let name = path.to_string_lossy();
        if name.contains("CORRUPT") {
            return Err("dlopen failed: corrupt image".to_owned());
        }
        Ok(DylibHandle { version })
    }

    /// Resolves a symbol address for a formula slot (stubbed).
    pub fn resolve_symbol_stub(
        &mut self,
        _token: &MainThreadToken,
        slot: &mut FormulaFnSlot,
        addr: *const (),
    ) {
        slot.fn_addr = addr;
    }
}
