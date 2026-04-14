//! GOAP world state, actions, planner, cache, and agent helpers.
//!
//! [`plan_dijkstra`] runs uniform-cost (Dijkstra) search. That is A* with a zero heuristic, so it
//! stays optimal for the additive edge costs used here; a future admissible heuristic only changes
//! the open-set ordering.

use std::collections::HashMap;

/// Fixed-size boolean + integer world encoding from the design doc.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WorldState {
    /// Packed boolean facts.
    pub bits: u128,
    /// Up to eight integer slots.
    pub ints: [i16; 8],
}

impl WorldState {
    /// Constructs a state.
    pub const fn new(bits: u128, ints: [i16; 8]) -> Self {
        Self { bits, ints }
    }

    /// Tests goal bits (masked) and optional minimum integer constraints.
    pub fn satisfies(
        &self,
        goal_bits_mask: u128,
        goal_bits_values: u128,
        int_mins: &[Option<i16>; 8],
    ) -> bool {
        let masked = self.bits & goal_bits_mask;
        let expected = goal_bits_values & goal_bits_mask;
        if masked != expected {
            return false;
        }
        for (i, mn) in int_mins.iter().enumerate() {
            if let Some(mn) = *mn {
                if self.ints[i] < mn {
                    return false;
                }
            }
        }
        true
    }

    /// Hamming-style distance on masked bits plus unsatisfied int slots.
    pub fn goal_distance_heuristic(
        &self,
        goal_bits_mask: u128,
        goal_bits_values: u128,
        int_mins: &[Option<i16>; 8],
    ) -> u32 {
        let mut h = 0u32;
        let diff = (self.bits ^ goal_bits_values) & goal_bits_mask;
        h += diff.count_ones();
        for (i, mn) in int_mins.iter().enumerate() {
            if let Some(mn) = *mn {
                if self.ints[i] < mn {
                    h += 1;
                }
            }
        }
        h
    }
}

/// Bit/int patch applied to a [`WorldState`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorldStateDelta {
    /// Bits that must match before applying (precondition).
    pub pre_mask: u128,
    /// Required values under `pre_mask`.
    pub pre_values: u128,
    /// Bits to set to 1.
    pub set_bits: u128,
    /// Bits to clear (applied after sets).
    pub clear_bits: u128,
    /// Integer adds per slot (applied if slot touched).
    pub int_add: [i16; 8],
    /// Which int slots receive `int_add`.
    pub int_touched: u8,
}

impl WorldStateDelta {
    /// Identity delta (always applicable, no effect).
    pub const fn noop() -> Self {
        Self {
            pre_mask: 0,
            pre_values: 0,
            set_bits: 0,
            clear_bits: 0,
            int_add: [0; 8],
            int_touched: 0,
        }
    }

    /// Preconditions hold on `s`.
    pub fn pre_hold(&self, s: WorldState) -> bool {
        (s.bits & self.pre_mask) == (self.pre_values & self.pre_mask)
    }

    /// Applies effects to produce a new [`WorldState`].
    pub fn apply(self, s: WorldState) -> WorldState {
        let mut bits = s.bits;
        bits |= self.set_bits;
        bits &= !self.clear_bits;
        let mut ints = s.ints;
        for (i, v) in ints.iter_mut().enumerate() {
            if (self.int_touched & (1 << i)) != 0 {
                *v = v.saturating_add(self.int_add[i]);
            }
        }
        WorldState { bits, ints }
    }
}

/// Planner action definition.
#[derive(Clone, Debug, PartialEq)]
pub struct GoapAction {
    /// Stable id (index into registry).
    pub id: usize,
    /// Human-readable label.
    pub name: &'static str,
    /// Preconditions + effects.
    pub delta: WorldStateDelta,
    /// Positive step cost.
    pub cost: f32,
}

/// Ordered plan slice.
#[derive(Clone, Debug, PartialEq)]
pub struct Plan {
    /// Action indices to execute in order.
    pub steps: Vec<usize>,
    /// Sum of action costs.
    pub total_cost: f32,
    /// Runtime cursor.
    pub current_step: usize,
}

impl Plan {
    /// Current action under the cursor.
    pub fn current_action<'a>(&self, reg: &'a GoapActionRegistry) -> Option<&'a GoapAction> {
        self.steps
            .get(self.current_step)
            .and_then(|&i| reg.actions.get(i))
    }

    /// Preconditions for the active step hold on `state`.
    pub fn current_step_pre_hold(&self, reg: &GoapActionRegistry, state: WorldState) -> bool {
        self.current_action(reg)
            .map(|a| a.delta.pre_hold(state))
            .unwrap_or(true)
    }
}

/// Owns the action table and a monotonic version for cache busting.
#[derive(Clone, Debug, PartialEq)]
pub struct GoapActionRegistry {
    /// Actions.
    pub actions: Vec<GoapAction>,
    /// Bumped to invalidate [`PlanCache`].
    pub version: u64,
}

impl GoapActionRegistry {
    /// New registry with version 1.
    pub fn new(actions: Vec<GoapAction>) -> Self {
        Self {
            actions,
            version: 1,
        }
    }

    /// Increments version (invalidates caches).
    pub fn bump_version(&mut self) {
        self.version = self.version.wrapping_add(1);
    }
}

/// Goal description using the same encoding as [`WorldState::satisfies`].
#[derive(Clone, Debug, PartialEq)]
pub struct GoapGoal {
    /// Goal id for cache keys.
    pub id: u32,
    pub bit_mask: u128,
    pub bit_values: u128,
    pub int_mins: [Option<i16>; 8],
    /// Designer priority (larger = more important).
    pub priority: f32,
    /// Cached satisfaction flag from last check.
    pub is_satisfied: bool,
}

impl GoapGoal {
    /// Updates [`Self::is_satisfied`] from `state`.
    pub fn refresh(&mut self, state: WorldState) {
        self.is_satisfied = state.satisfies(self.bit_mask, self.bit_values, &self.int_mins);
    }
}

/// Cache key: goal + hashed world.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PlanCacheKey {
    /// Goal id.
    pub goal_id: u32,
    /// Hash of relevant world fields.
    pub state_hash: u64,
}

/// Plan cache with registry version guard.
#[derive(Clone, Debug, PartialEq)]
pub struct PlanCache {
    /// Stored plans.
    pub entries: HashMap<PlanCacheKey, Plan>,
    /// Mirrors last seen registry version.
    pub registry_version: u64,
}

impl Default for PlanCache {
    fn default() -> Self {
        Self::new()
    }
}

impl PlanCache {
    /// Empty cache.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            registry_version: 0,
        }
    }

    /// Drops all entries when `registry.version` changes.
    pub fn sync_version(&mut self, registry: &GoapActionRegistry) {
        if self.registry_version != registry.version {
            self.entries.clear();
            self.registry_version = registry.version;
        }
    }

    /// Clears everything (explicit invalidation).
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

/// Per-agent GOAP state for tests.
#[derive(Clone, Debug, PartialEq)]
pub struct GoapAgent {
    /// Current world snapshot.
    pub current_state: WorldState,
    /// Prioritized goals.
    pub goals: Vec<GoapGoal>,
    /// Active plan if any.
    pub active_plan: Option<Plan>,
    /// Active goal id.
    pub active_goal: Option<u32>,
    /// Seconds until another replan is allowed.
    pub replan_cooldown_remaining: f32,
}

impl GoapAgent {
    /// New agent with empty goals.
    pub fn new(initial: WorldState) -> Self {
        Self {
            current_state: initial,
            goals: Vec::new(),
            active_plan: None,
            active_goal: None,
            replan_cooldown_remaining: 0.0,
        }
    }

    /// Picks highest-priority unsatisfied goal.
    pub fn pick_goal(&mut self) -> Option<&mut GoapGoal> {
        let mut best: Option<usize> = None;
        let mut best_p = f32::NEG_INFINITY;
        for (i, g) in self.goals.iter_mut().enumerate() {
            g.refresh(self.current_state);
            if g.is_satisfied {
                continue;
            }
            if g.priority > best_p {
                best_p = g.priority;
                best = Some(i);
            }
        }
        best.map(|i| &mut self.goals[i])
    }

    /// Advances replan cooldown by `dt`.
    pub fn tick_cooldown(&mut self, dt: f32) {
        self.replan_cooldown_remaining = (self.replan_cooldown_remaining - dt).max(0.0);
    }

    /// Arms cooldown after a successful replan trigger.
    pub fn arm_replan_cooldown(&mut self, duration: f32) {
        self.replan_cooldown_remaining = duration;
    }

    /// `true` when a replan is allowed (cooldown exhausted).
    pub fn replan_allowed(&self) -> bool {
        self.replan_cooldown_remaining <= 0.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct StateKey {
    bits: u128,
    ints: [i16; 8],
}

impl From<WorldState> for StateKey {
    fn from(s: WorldState) -> Self {
        Self {
            bits: s.bits,
            ints: s.ints,
        }
    }
}

/// Uniform-cost (Dijkstra) search for a cheapest action sequence to satisfy the goal bits + int
/// mins. Equivalent to A* with a zero heuristic, so plans are optimal for this additive cost model.
pub fn plan_dijkstra(
    start: WorldState,
    goal_bits_mask: u128,
    goal_bits_values: u128,
    int_mins: &[Option<i16>; 8],
    reg: &GoapActionRegistry,
) -> Option<Plan> {
    if start.satisfies(goal_bits_mask, goal_bits_values, int_mins) {
        return Some(Plan {
            steps: Vec::new(),
            total_cost: 0.0,
            current_step: 0,
        });
    }

    let start_key = StateKey::from(start);
    let mut best: HashMap<StateKey, f32> = HashMap::new();
    let mut parent: HashMap<StateKey, (StateKey, usize)> = HashMap::new();
    let mut open: Vec<(f32, WorldState)> = vec![(0.0, start)];

    best.insert(start_key, 0.0);

    while !open.is_empty() {
        let mut best_i = 0usize;
        let mut best_c = open[0].0;
        for (i, &(c, _)) in open.iter().enumerate().skip(1) {
            if c < best_c {
                best_c = c;
                best_i = i;
            }
        }
        let (cost, state) = open.swap_remove(best_i);
        let sk = StateKey::from(state);
        if cost > *best.get(&sk).unwrap_or(&f32::INFINITY) + 1e-3 {
            continue;
        }

        if state.satisfies(goal_bits_mask, goal_bits_values, int_mins) {
            let mut steps = Vec::new();
            let mut ck = sk;
            while ck != start_key {
                let (pk, act) = *parent.get(&ck).expect("parent");
                steps.push(act);
                ck = pk;
            }
            steps.reverse();
            return Some(Plan {
                total_cost: cost,
                steps,
                current_step: 0,
            });
        }

        for (i, act) in reg.actions.iter().enumerate() {
            if !act.delta.pre_hold(state) {
                continue;
            }
            let next = act.delta.apply(state);
            let nk = StateKey::from(next);
            let nc = cost + act.cost;
            let prev = *best.get(&nk).unwrap_or(&f32::INFINITY);
            if nc < prev - 1e-6 {
                best.insert(nk, nc);
                parent.insert(nk, (sk, i));
                open.push((nc, next));
            }
        }
    }

    None
}

/// FNV-1a 64-bit hash for cache keys (deterministic, std-only).
pub fn hash_world(state: WorldState) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    let primes: [u8; 16] = state.bits.to_le_bytes();
    for b in primes {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x100000001b3);
    }
    for v in state.ints {
        for b in v.to_le_bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x100000001b3);
        }
    }
    h
}
