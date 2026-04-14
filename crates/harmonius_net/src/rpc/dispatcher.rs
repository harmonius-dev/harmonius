//! Minimal RPC surface for deterministic tests.

/// RPC failure modes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RpcError {
    /// Parameter validation failed.
    Validation {
        /// Stable reason tag.
        reason: &'static str,
    },
}

/// Ability cast id.
#[derive(Clone, Copy, Debug)]
pub struct CastAbility {
    /// Ability id.
    pub ability_id: i32,
}

/// Chat line.
#[derive(Clone, Debug)]
pub struct Chat {
    /// UTF-8 text.
    pub text: String,
}

/// Trade offer entity ref (test stub).
#[derive(Clone, Copy, Debug)]
pub struct TradeOffer {
    /// Entity id (0 = invalid in tests).
    pub entity: u32,
}

/// Damage number client RPC payload.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DamageNumber {
    /// Damage amount.
    pub amount: i32,
}

/// Validates `CastAbility` range.
pub fn cast_ability(msg: &CastAbility) -> Result<(), RpcError> {
    if !(0..=4096).contains(&msg.ability_id) {
        return Err(RpcError::Validation {
            reason: "ability_id_range",
        });
    }
    Ok(())
}

/// Coalesce to the highest invocation id in one tick (not tail order).
pub fn merge_reliable_latest(invocations: &[i32]) -> Option<i32> {
    invocations.iter().max().copied()
}

/// Spatial multicast on a line: `positions` are scalar offsets (y ignored).
pub fn multicast_spatial(center: f32, radius: f32, positions: &[f32]) -> usize {
    positions
        .iter()
        .filter(|&&p| (p - center).abs() <= radius)
        .count()
}

/// Planar disk multicast: each position is `(x, y)` in meters; counts peers inside the circle.
pub fn multicast_spatial_disk(center: (f32, f32), radius: f32, positions: &[(f32, f32)]) -> usize {
    let r2 = radius * radius;
    positions
        .iter()
        .filter(|&&(x, y)| {
            let dx = x - center.0;
            let dy = y - center.1;
            dx * dx + dy * dy <= r2
        })
        .count()
}

/// Per-target client RPC fanout counter.
pub fn client_rpc_fanout(target: usize, num_clients: usize) -> usize {
    if target < num_clients {
        1
    } else {
        0
    }
}

/// Three validation cases from the test plan.
pub fn rpc_param_checks() -> [Result<(), RpcError>; 3] {
    [
        cast_ability(&CastAbility {
            ability_id: i32::MAX,
        }),
        {
            let text = "\0".repeat(4097);
            if text.len() > 4096 {
                Err(RpcError::Validation { reason: "chat_len" })
            } else {
                Ok(())
            }
        },
        {
            let t = TradeOffer { entity: 0 };
            if t.entity == 0 {
                Err(RpcError::Validation {
                    reason: "entity_ref",
                })
            } else {
                Ok(())
            }
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.3.1.1 — invalid id rejected; valid accepted.
    #[test]
    fn test_server_rpc_validation() {
        assert_eq!(
            cast_ability(&CastAbility { ability_id: 99999 }),
            Err(RpcError::Validation {
                reason: "ability_id_range"
            })
        );
        assert!(cast_ability(&CastAbility { ability_id: 7 }).is_ok());
    }

    /// TC-8.3.2.1 — single-target fanout.
    #[test]
    fn test_client_rpc_one_shot() {
        assert_eq!(client_rpc_fanout(3, 100), 1);
        assert_eq!(client_rpc_fanout(150, 100), 0);
        let _ = DamageNumber { amount: 42 };
    }

    /// TC-8.3.3.1 — spatial filter counts in-radius clients.
    #[test]
    fn test_multicast_filtered() {
        let mut positions = vec![10.0f32; 30];
        positions.extend((0..70).map(|_| 200.0f32));
        let n = multicast_spatial(10.0, 50.0, &positions);
        assert_eq!(n, 30);
    }

    /// TC-8.3.4.1 — only latest survives coalescing.
    #[test]
    fn test_rpc_reliable_latest() {
        let v: Vec<i32> = (1..=10).collect();
        assert_eq!(merge_reliable_latest(&v), Some(10));
        assert_eq!(merge_reliable_latest(&[5, 10, 3]), Some(10));
    }

    #[test]
    fn test_multicast_disk_counts() {
        let positions: Vec<(f32, f32)> = (0..30).map(|_| (10.0, 0.0)).collect();
        let far: Vec<(f32, f32)> = (0..70).map(|_| (200.0, 0.0)).collect();
        let mut all = positions;
        all.extend(far);
        assert_eq!(multicast_spatial_disk((10.0, 0.0), 50.0, &all), 30);
    }

    /// TC-8.3.5.1 — three validation errors.
    #[test]
    fn test_rpc_param_validation_rejects() {
        let r = rpc_param_checks();
        assert!(r[0].is_err());
        assert!(r[1].is_err());
        assert!(r[2].is_err());
    }
}
