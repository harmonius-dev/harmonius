//! Deterministic physics preview (play / pause / step / stop).

use std::collections::HashMap;

use crate::math::Vec3;
use crate::model::Entity;

/// Dynamic body state integrated under gravity (TC-IR-5.4.3.x).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PreviewBody {
    /// World translation.
    pub position: Vec3,
    /// World linear velocity.
    pub velocity: Vec3,
}

/// Fixed-timestep sandbox for editor play mode.
#[derive(Debug)]
pub struct PhysicsPreview {
    dt: f32,
    gravity: Vec3,
    bodies: HashMap<Entity, PreviewBody>,
    snapshot: Option<HashMap<Entity, PreviewBody>>,
    paused: bool,
}

impl PhysicsPreview {
    /// Builds preview with default timestep `1.0 / 60.0`.
    #[must_use]
    pub fn new(gravity: Vec3) -> Self {
        Self {
            dt: 1.0 / 60.0,
            gravity,
            bodies: HashMap::new(),
            snapshot: None,
            paused: false,
        }
    }

    /// Registers a dynamic body at rest.
    pub fn register_body(&mut self, entity: Entity, position: Vec3) {
        self.bodies.insert(
            entity,
            PreviewBody {
                position,
                velocity: Vec3::zero(),
            },
        );
    }

    /// Starts play: captures poses, clears pause.
    pub fn play(&mut self) {
        self.snapshot = Some(self.bodies.clone());
        self.paused = false;
    }

    /// Pauses integration until `step_once` or `play`.
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Integrates exactly one fixed substep when paused (TC-IR-5.4.3.2).
    pub fn step_once(&mut self) {
        self.integrate_tick();
    }

    /// Advances all dynamic bodies by one fixed tick when running.
    pub fn tick(&mut self) {
        if !self.paused {
            self.integrate_tick();
        }
    }

    fn integrate_tick(&mut self) {
        let g = self.gravity;
        let dt = self.dt;
        for b in self.bodies.values_mut() {
            b.velocity.x += g.x * dt;
            b.velocity.y += g.y * dt;
            b.velocity.z += g.z * dt;
            b.position.x += b.velocity.x * dt;
            b.position.y += b.velocity.y * dt;
            b.position.z += b.velocity.z * dt;
        }
    }

    /// Restores the pre-play snapshot (TC-IR-5.4.3.3).
    pub fn stop(&mut self) {
        if let Some(snap) = self.snapshot.take() {
            self.bodies = snap;
        }
        self.paused = false;
    }

    /// Reads body position after simulation.
    #[must_use]
    pub fn position(&self, entity: Entity) -> Option<Vec3> {
        self.bodies.get(&entity).map(|b| b.position)
    }
}
