//! Priority scheduling under a byte budget.

/// Priority class for replication.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PriorityClass {
    /// Low priority bulk state.
    Low = 0,
    /// High priority gameplay-critical.
    High = 1,
}

/// Entity replication demand.
#[derive(Clone, Debug)]
pub struct EntityRep {
    /// Id.
    pub id: u32,
    /// Priority.
    pub class: PriorityClass,
    /// Bytes if scheduled this tick.
    pub bytes: u32,
}

/// Greedy schedule: all high first until budget, then remaining budget to low in stable id order.
pub fn schedule_replication(entities: &[EntityRep], budget: u32) -> (Vec<u32>, u32) {
    let mut high: Vec<_> = entities
        .iter()
        .filter(|e| e.class == PriorityClass::High)
        .collect();
    let mut low: Vec<_> = entities
        .iter()
        .filter(|e| e.class == PriorityClass::Low)
        .collect();
    high.sort_by_key(|e| e.id);
    low.sort_by_key(|e| e.id);
    let mut used = 0u32;
    let mut out = Vec::new();
    for e in high {
        if used + e.bytes <= budget {
            used += e.bytes;
            out.push(e.id);
        }
    }
    for e in low {
        if used + e.bytes <= budget {
            used += e.bytes;
            out.push(e.id);
        }
    }
    (out, used)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.2.5.1 — high-priority entities always win under a tiny budget.
    #[test]
    fn test_priority_under_budget() {
        let mut ents = Vec::new();
        for i in 0..10 {
            ents.push(EntityRep {
                id: i,
                class: PriorityClass::High,
                bytes: 100,
            });
        }
        for i in 10..510 {
            ents.push(EntityRep {
                id: i,
                class: PriorityClass::Low,
                bytes: 400,
            });
        }
        let budget = 50_000;
        let (ids, used) = schedule_replication(&ents, budget);
        assert!(used <= budget);
        for hi in 0..10 {
            assert!(ids.contains(&hi), "missing high {hi}");
        }
    }
}
