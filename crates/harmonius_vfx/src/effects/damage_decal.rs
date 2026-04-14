//! Damage decal variant selection from weapon hits (**TC-11.2.5.1**).

/// Weapon identifiers used by decal tables.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WeaponKind {
    /// Sword slash damage.
    Sword,
}

/// Hit payload used for decal selection.
#[derive(Clone, Copy, Debug)]
pub struct HitEvent {
    /// Weapon used for the strike.
    pub weapon: WeaponKind,
    /// Rotation in degrees applied to the decal quad.
    pub angle_degrees: f32,
    /// Stable hit index for variant hashing.
    pub hit_index: u32,
}

/// Selects atlas variant id and rotation for a hit (**TC-11.2.5.1**).
pub fn select_damage_decal(hit: HitEvent) -> (u32, f32) {
    let variant = 10 + (hit.hit_index % 5);
    (variant, hit.angle_degrees)
}

/// Counts distinct variants used across repeated hits (**TC-11.2.5.1** #2).
pub fn distinct_variant_count(hits: &[HitEvent]) -> usize {
    let mut set = std::collections::HashSet::new();
    for h in hits {
        set.insert(select_damage_decal(*h).0);
    }
    set.len()
}

#[cfg(test)]
mod tests {
    use super::{HitEvent, WeaponKind, distinct_variant_count, select_damage_decal};

    /// **TC-11.2.5.1** — sword hits pick atlas variants and preserve rotation.
    #[test]
    fn tc_11_2_5_1_damage_decal_weapon_variation() {
        let hit = HitEvent {
            weapon: WeaponKind::Sword,
            angle_degrees: 45.0,
            hit_index: 0,
        };
        let (v, rot) = select_damage_decal(hit);
        assert_eq!(rot, 45.0);
        assert!(v >= 10);

        let hits: Vec<_> = (0..10)
            .map(|i| HitEvent {
                weapon: WeaponKind::Sword,
                angle_degrees: 45.0,
                hit_index: i,
            })
            .collect();
        assert!(distinct_variant_count(&hits) >= 3);
    }
}
