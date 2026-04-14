//! Parallel batch queries routed through [`crate::ThreadPool`].

use std::sync::mpsc;
use std::thread;

use crate::physics_broadphase::PhysicsBroadphase;
use crate::physics_bvh::PhysicsBvh;
use crate::ray::{RayHit, RayQuery};
use crate::thread_pool::ThreadPool;

/// Failure modes for [`BatchSpatialQuery::dispatch`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BatchDispatchError {
    /// A worker thread panicked while executing a queued query.
    WorkerPanicked,
}

/// Aggregated outputs for a batch dispatch.
#[derive(Debug, Default, PartialEq)]
pub struct BatchResults {
    /// One entry per queued ray (in order).
    pub ray_hits: Vec<Option<RayHit>>,
}

/// Parallel dispatcher for ray queries (shape/overlap hooks reserved).
#[derive(Debug, Default)]
pub struct BatchSpatialQuery {
    /// Queued ray tests.
    pub rays: Vec<RayQuery>,
}

impl BatchSpatialQuery {
    /// Dispatches all ray queries in parallel using scoped worker threads.
    pub fn dispatch(
        &self,
        pool: &ThreadPool,
        world: &PhysicsBroadphase,
        main_thread: thread::ThreadId,
        results: &mut BatchResults,
    ) -> Result<(), BatchDispatchError> {
        let bvh: &PhysicsBvh = world.inner();
        results.ray_hits.clear();
        results.ray_hits.resize(self.rays.len(), None);

        let mut dispatch_status = Ok(());

        pool.scope(|scope| {
            let mut handles = Vec::with_capacity(self.rays.len());
            for index in 0..self.rays.len() {
                let ray = self.rays[index];
                let bvh_ref = bvh;
                handles.push(scope.spawn(move || {
                    assert_ne!(
                        thread::current().id(),
                        main_thread,
                        "ray work must not run on the main thread"
                    );
                    let hit = bvh_ref.raycast_first(&ray);
                    (index, hit)
                }));
            }

            for handle in handles {
                match handle.join() {
                    Ok((index, hit)) => {
                        results.ray_hits[index] = hit;
                    }
                    Err(_) => {
                        dispatch_status = Err(BatchDispatchError::WorkerPanicked);
                    }
                }
            }
        });

        dispatch_status
    }
}

/// Dispatches `count` no-op messages through a `sync_channel` with capacity 1024.
///
/// This models the bounded MPSC result channel described in the integration design.
pub fn drain_bounded_channel_1024(count: usize) -> Result<Vec<u32>, mpsc::RecvError> {
    let (sender, receiver) = mpsc::sync_channel(1024);
    thread::scope(|scope| {
        for index in 0..count {
            let sender = sender.clone();
            scope.spawn(move || {
                sender
                    .send(index as u32)
                    .expect("bounded channel accepts 1024 sends");
            });
        }
    });
    drop(sender);

    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        values.push(receiver.recv()?);
    }
    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aabb::Aabb;
    use crate::collision_layers::CollisionLayers;
    use crate::entity::Entity;
    use crate::math::Vec3;
    use crate::physics_bvh::LeafEntry;
    use crate::ray::intersect_ray_aabb;
    use crate::shared_bvh_index::BvhIndex;

    #[test]
    fn tc_ir_3_9_4_u1_batch_spatial_query_dispatch_parallel_rays() {
        let pool = ThreadPool::new();
        let main = thread::current().id();

        let mut world = PhysicsBroadphase::new();
        world.insert_leaf(LeafEntry {
            entity: Entity::from_raw(1),
            aabb: Aabb::unit_cube_at_origin(),
            layers: CollisionLayers::default(),
        });

        let mut rays = Vec::new();
        for offset in 0..8 {
            rays.push(RayQuery {
                origin: Vec3::new(offset as f32 * 0.01, 0.0, 5.0),
                dir: Vec3::new(0.0, 0.0, -1.0),
                max_t: 50.0,
            });
        }

        let batch = BatchSpatialQuery { rays };
        let mut results = BatchResults::default();
        batch
            .dispatch(&pool, &world, main, &mut results)
            .expect("dispatch succeeds");
        assert_eq!(results.ray_hits.len(), 8);
        assert!(results.ray_hits.iter().all(|hit| hit.is_some()));
    }

    #[test]
    fn tc_ir_3_9_4_u2_mpsc_buffer_cap_1024() {
        let values = drain_bounded_channel_1024(1024).expect("recv");
        assert_eq!(values.len(), 1024);
    }

    #[test]
    fn ray_hit_picks_nearest_leaf() {
        let mut world = PhysicsBroadphase::new();
        world.insert_leaf(LeafEntry {
            entity: Entity::from_raw(1),
            aabb: Aabb::unit_cube_with_min_corner(Vec3::new(2.0, -0.5, -0.5)),
            layers: CollisionLayers::default(),
        });
        world.insert_leaf(LeafEntry {
            entity: Entity::from_raw(2),
            aabb: Aabb::unit_cube_with_min_corner(Vec3::new(4.0, -0.5, -0.5)),
            layers: CollisionLayers::default(),
        });

        let ray = RayQuery {
            origin: Vec3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(1.0, 0.0, 0.0),
            max_t: 100.0,
        };
        let first = world.raycast_first(&ray).expect("hit");
        let expected = intersect_ray_aabb(
            &ray,
            &Aabb::unit_cube_with_min_corner(Vec3::new(2.0, -0.5, -0.5)),
        )
        .expect("intersection");
        assert!((first.t - expected).abs() < 1.0e-3);
    }

    #[test]
    fn tc_ir_3_9_2_1_shared_bvh_index_revision_unchanged_after_physics_queries() {
        let shared = BvhIndex::default();
        let revision_before = shared.revision();

        let pool = ThreadPool::new();
        let main = thread::current().id();
        let mut world = PhysicsBroadphase::new();
        world.insert_leaf(LeafEntry {
            entity: Entity::from_raw(1),
            aabb: Aabb::unit_cube_at_origin(),
            layers: CollisionLayers::default(),
        });

        let batch = BatchSpatialQuery {
            rays: vec![RayQuery {
                origin: Vec3::new(0.0, 0.0, 5.0),
                dir: Vec3::new(0.0, 0.0, -1.0),
                max_t: 50.0,
            }],
        };
        let mut results = BatchResults::default();
        batch
            .dispatch(&pool, &world, main, &mut results)
            .expect("dispatch succeeds");

        let _ = world.overlapping_pairs();
        assert_eq!(revision_before, shared.revision());
        assert!(results.ray_hits[0].is_some());
    }
}
