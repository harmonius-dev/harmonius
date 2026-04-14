//! Entitlement and subscription checks (`R-14.5.6`).

use std::collections::HashSet;
use std::sync::Mutex;

/// Verifies DLC / subscription ownership (stubbed catalog).
#[derive(Debug, Default)]
pub struct EntitlementService {
    owned: Mutex<HashSet<String>>,
    subs: Mutex<HashSet<String>>,
}

impl EntitlementService {
    /// Creates an empty entitlement catalog.
    pub fn new() -> Self {
        Self::default()
    }

    /// Refreshes entitlement bits from a platform sync (test hook).
    pub fn refresh(&self, owned: &[&str], subs: &[&str]) {
        let mut o = self.owned.lock().expect("ent mutex poisoned");
        let mut s = self.subs.lock().expect("ent mutex poisoned");
        o.clear();
        s.clear();
        for x in owned {
            o.insert((*x).into());
        }
        for x in subs {
            s.insert((*x).into());
        }
    }

    /// Returns whether `sku` is owned outright.
    pub fn is_owned(&self, sku: &str) -> bool {
        self.owned.lock().expect("ent mutex poisoned").contains(sku)
    }

    /// Returns whether `sku` subscription is active.
    pub fn is_subscription_active(&self, sku: &str) -> bool {
        self.subs.lock().expect("ent mutex poisoned").contains(sku)
    }
}
