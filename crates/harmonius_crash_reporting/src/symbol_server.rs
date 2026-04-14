//! In-memory symbol server used by unit tests (`TC-14.4.2.4`).

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::symbols::{SymbolPut, SymbolUploadError};

/// CAS-like symbol store keyed by index path.
#[derive(Clone, Debug, Default)]
pub struct InMemorySymbolServer {
    inner: Arc<Mutex<HashMap<Vec<u8>, Vec<u8>>>>,
}

impl InMemorySymbolServer {
    /// Creates an empty server.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Retrieves bytes previously stored with [`SymbolPut::put`].
    #[must_use]
    pub fn get(&self, index_key: &str) -> Option<Vec<u8>> {
        self.inner.lock().ok()?.get(index_key.as_bytes()).cloned()
    }
}

impl SymbolPut for InMemorySymbolServer {
    fn put(&mut self, index_key: &str, bytes: &[u8]) -> Result<(), SymbolUploadError> {
        self.inner
            .lock()
            .map_err(|_| SymbolUploadError::Io("mutex poisoned".to_owned()))?
            .insert(index_key.as_bytes().to_vec(), bytes.to_vec());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_server_put_and_get() {
        let mut server = InMemorySymbolServer::new();
        let key = "game.pdb/abc/2/game.pdb";
        let bytes = b"symfile".to_vec();
        SymbolPut::put(&mut server, key, &bytes).unwrap();
        assert_eq!(server.get(key), Some(bytes));
    }
}
