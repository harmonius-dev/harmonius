//! Dedicated server cluster pool (TC-8.5.4.1).

use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr};

use super::types::{ClusterPool, InstanceId, InstanceStatus, RegistryEntry, SessionSpec};

#[derive(Clone, Debug, Default)]
pub struct ClusterManager {
    registry: HashMap<InstanceId, RegistryEntry>,
    next_id: u64,
}

impl ClusterManager {
    pub fn spawn_instance(
        &mut self,
        pool: &mut ClusterPool,
        _spec: &SessionSpec,
    ) -> InstanceId {
        if pool.warm == 0 {
            panic!("empty pool");
        }
        pool.warm -= 1;
        self.next_id += 1;
        let id = InstanceId(self.next_id);
        let addr = SocketAddr::new(
            std::net::IpAddr::V4(Ipv4Addr::new(10, 0, 0, 2)),
            7777 + self.next_id as u16,
        );
        self.registry.insert(
            id,
            RegistryEntry {
                id,
                addr,
                status: InstanceStatus::Ready,
            },
        );
        id
    }

    pub fn registry_get(&self, id: InstanceId) -> Option<&RegistryEntry> {
        self.registry.get(&id)
    }

    pub fn shutdown_instance(&mut self, id: InstanceId) {
        self.registry.remove(&id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.5.4.1
    #[test]
    fn test_dedicated_server_cluster_spawn() {
        let mut cm = ClusterManager::default();
        let mut pool = ClusterPool {
            warm: 4,
            target_region: "us-east".into(),
        };
        let spec = SessionSpec {
            map: "arena_01".into(),
        };
        let id = cm.spawn_instance(&mut pool, &spec);
        assert_eq!(pool.warm, 3);
        let ent = cm.registry_get(id).expect("registry");
        assert_eq!(ent.status, InstanceStatus::Ready);
        cm.shutdown_instance(id);
        assert!(cm.registry_get(id).is_none());
    }
}
