//! Deterministic stepping and peer state hashing (R-16.5.4).

use bevy_ecs::prelude::*;

use crate::components::{
    AbilityState, AttributeSet, Container, CraftingState, DirectedGraphInstance, Effect,
    InventoryState, Meter, QuestState, ScheduleState, StealthState,
};
use crate::phase::FramePhase;

/// Seeded RNG for deterministic recipes (design: `(tick, recipe_id)` seed).
#[derive(Clone, Copy, Debug)]
pub struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    /// Build RNG from tick and stable recipe id.
    pub fn from_tick(tick: u64, recipe_id: u64) -> Self {
        let mut s = tick ^ recipe_id.rotate_left(17);
        if s == 0 {
            s = 0x9e3779b97f4a7c15;
        }
        Self { state: s }
    }

    /// Next pseudo-random `u32`.
    pub fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        (self.state >> 32) as u32
    }
}

fn mix(mut h: u64, x: u64) -> u64 {
    h ^= x.wrapping_add(0x9e3779b97f4a7c15);
    h = h.rotate_left(27);
    h.wrapping_mul(0x85ebca6b)
}

/// Per-frame inputs replicated across peers.
#[derive(Clone, Debug, Default)]
pub struct FrameInput {
    /// Monotonic tick.
    pub tick: u64,
    /// Quest kill presses this frame.
    pub quest_kills: u8,
    /// Ability cast requested.
    pub ability_cast: bool,
    /// Inventory equip slot toggle.
    pub inventory_equip: bool,
    /// Craft attempt.
    pub craft_attempt: bool,
    /// Player moved through noisy cell.
    pub stealth_move: bool,
    /// Schedule timeline tick advance amount.
    pub schedule_ticks: u64,
}

/// Advance composed state for one frame following [`FramePhase::ORDER`].
pub fn step_frame(world: &mut World, input: &FrameInput) {
    for phase in FramePhase::ORDER {
        match phase {
            FramePhase::EventLogIngest => {}
            FramePhase::TimelineTick => {
                let mut q = world.query::<&mut ScheduleState>();
                for mut s in q.iter_mut(world) {
                    s.elapsed = s.elapsed.saturating_add(input.schedule_ticks);
                    if s.elapsed >= s.fire_at {
                        s.fired = true;
                    }
                }
            }
            FramePhase::SpatialAwareness => {}
            FramePhase::GridPropagation => {}
            FramePhase::ContainerMutation => {
                if input.craft_attempt {
                    let mut q = world.query::<&mut CraftingState>();
                    for mut c in q.iter_mut(world) {
                        if c.herbs >= 2 && c.vials >= 1 {
                            c.herbs -= 2;
                            c.vials -= 1;
                            c.potions += 1;
                        }
                    }
                }
            }
            FramePhase::DirectedGraphAdvance => {
                if input.quest_kills > 0 {
                    let mut qq = world.query::<&mut QuestState>();
                    for mut q in qq.iter_mut(world) {
                        let inc = input.quest_kills as u32;
                        q.active_objective =
                            (q.active_objective + inc).min(q.objectives.saturating_sub(1));
                    }
                    let mut gq = world.query::<&mut DirectedGraphInstance>();
                    for mut g in gq.iter_mut(world) {
                        let inc = input.quest_kills as u32;
                        g.current_node = (g.current_node + inc).min(g.nodes.saturating_sub(1));
                    }
                }
            }
            FramePhase::AttributeAggregation => {
                if input.inventory_equip {
                    let mut q = world.query::<&mut InventoryState>();
                    for mut inv in q.iter_mut(world) {
                        inv.attack += 5.0;
                    }
                }
            }
            FramePhase::EffectTick => {
                if input.ability_cast {
                    let mut q = world.query::<&mut AbilityState>();
                    for mut a in q.iter_mut(world) {
                        a.mana = (a.mana - 20.0).max(0.0);
                    }
                }
                if input.stealth_move {
                    let mut q = world.query::<&mut StealthState>();
                    for mut s in q.iter_mut(world) {
                        s.detected = true;
                    }
                }
            }
            FramePhase::EventLogCommit => {}
        }
    }
}

/// Deterministic hash of gameplay-relevant components for all entities.
///
/// Order-independent: hashes each entity's contribution then sorts before mixing so different
/// spawn indices across peers still compare equal for identical simulation state.
pub fn peer_state_hash(world: &mut World) -> u64 {
    let entities: Vec<Entity> = {
        let mut q = world.query::<Entity>();
        q.iter(world).collect()
    };
    let mut chunks: Vec<u64> = Vec::new();
    for id in entities {
        let mut h = 0u64;
        if let Some(q) = world.entity(id).get::<QuestState>() {
            h = mix(h, 1);
            h = mix(h, u64::from(q.active_objective));
            h = mix(h, u64::from(q.objectives));
        }
        if let Some(g) = world.entity(id).get::<DirectedGraphInstance>() {
            h = mix(h, 2);
            h = mix(h, u64::from(g.current_node));
            h = mix(h, u64::from(g.nodes));
        }
        if let Some(a) = world.entity(id).get::<AbilityState>() {
            h = mix(h, 3);
            h = mix(h, a.mana.to_bits() as u64);
        }
        if let Some(i) = world.entity(id).get::<InventoryState>() {
            h = mix(h, 4);
            h = mix(h, i.attack.to_bits() as u64);
        }
        if let Some(c) = world.entity(id).get::<CraftingState>() {
            h = mix(h, 5);
            h = mix(h, u64::from(c.herbs));
            h = mix(h, u64::from(c.vials));
            h = mix(h, u64::from(c.potions));
        }
        if let Some(s) = world.entity(id).get::<StealthState>() {
            h = mix(h, 6);
            h = mix(h, u64::from(s.detected as u8));
        }
        if let Some(s) = world.entity(id).get::<ScheduleState>() {
            h = mix(h, 7);
            h = mix(h, s.elapsed);
            h = mix(h, u64::from(s.fired as u8));
        }
        if let Some(m) = world.entity(id).get::<Meter>() {
            h = mix(h, 8);
            h = mix(h, m.value.to_bits() as u64);
        }
        if let Some(a) = world.entity(id).get::<AttributeSet>() {
            h = mix(h, 9);
            for (k, v) in &a.values {
                for b in k.as_bytes() {
                    h = mix(h, u64::from(*b));
                }
                h = mix(h, v.to_bits() as u64);
            }
        }
        if let Some(c) = world.entity(id).get::<Container>() {
            h = mix(h, 10);
            h = mix(h, u64::from(c.capacity));
            for slot in &c.slots {
                h = mix(h, slot.map(|i| i.0).unwrap_or(0));
            }
        }
        if let Some(eff) = world.entity(id).get::<Effect>() {
            h = mix(h, 11);
            for b in eff.name.as_bytes() {
                h = mix(h, u64::from(*b));
            }
            h = mix(h, eff.magnitude.to_bits() as u64);
        }
        chunks.push(h);
    }
    chunks.sort_unstable();
    let mut out = 0u64;
    for cell in chunks {
        out = mix(out, cell);
    }
    out
}
