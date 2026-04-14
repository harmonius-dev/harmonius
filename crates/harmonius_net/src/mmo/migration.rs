//! Player migration between servers (`R-8.7.4`).

use crate::mmo::zone::Transform;

/// Linear velocity snapshot carried across migrations.
#[derive(Clone, Debug, PartialEq)]
pub struct Velocity {
    /// X velocity component.
    pub x: f32,
    /// Y velocity component.
    pub y: f32,
    /// Z velocity component.
    pub z: f32,
}

/// Active timed buff carried with a migrating player.
#[derive(Clone, Debug, PartialEq)]
pub struct Buff {
    /// Stable buff identifier.
    pub id: u32,
    /// Remaining duration in seconds.
    pub remaining: f64,
}

/// Packed entity snapshot used by [`MigrationService`].
#[derive(Clone, Debug, PartialEq)]
pub struct MigrationEntity {
    /// World transform at migration time.
    pub transform: Transform,
    /// Velocity snapshot.
    pub velocity: Velocity,
    /// Active buffs with remaining durations.
    pub buffs: Vec<Buff>,
}

/// Source slot held no entity during [`MigrationService::migrate`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MigrationError;

/// Deterministic migration helper (`TC-8.7.4.1`, `TC-8.7.4.2`).
pub struct MigrationService;

impl MigrationService {
    /// Moves `entity` from `src` to `dst`, clearing `src` on success.
    pub fn migrate(
        src: &mut Option<MigrationEntity>,
        dst: &mut Option<MigrationEntity>,
    ) -> Result<(), MigrationError> {
        let entity = src.take().ok_or(MigrationError)?;
        *dst = Some(entity);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `TC-8.7.4.1` `test_player_migration_state`
    #[test]
    fn test_player_migration_state() {
        let mut src = Some(MigrationEntity {
            transform: Transform {
                x: 100.0,
                y: 5.0,
                z: 200.0,
            },
            velocity: Velocity {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            buffs: Vec::new(),
        });
        let mut dst: Option<MigrationEntity> = None;
        MigrationService::migrate(&mut src, &mut dst).unwrap();
        assert!(src.is_none());
        let entity = dst.as_ref().unwrap();
        assert_eq!(
            entity.transform,
            Transform {
                x: 100.0,
                y: 5.0,
                z: 200.0,
            }
        );
        assert_eq!(
            entity.velocity,
            Velocity {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            }
        );
    }

    /// `TC-8.7.4.2` `test_player_migration_buffs`
    #[test]
    fn test_player_migration_buffs() {
        let buffs = vec![
            Buff {
                id: 1,
                remaining: 5.5,
            },
            Buff {
                id: 2,
                remaining: 12.0,
            },
        ];
        let mut src = Some(MigrationEntity {
            transform: Transform {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            velocity: Velocity {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            buffs: buffs.clone(),
        });
        let mut dst = None;
        MigrationService::migrate(&mut src, &mut dst).unwrap();
        let migrated = dst.as_ref().unwrap();
        assert_eq!(migrated.buffs.len(), buffs.len());
        for (left, right) in migrated.buffs.iter().zip(buffs.iter()) {
            assert_eq!(left.id, right.id);
            assert!((left.remaining - right.remaining).abs() <= 0.05);
        }
    }
}
