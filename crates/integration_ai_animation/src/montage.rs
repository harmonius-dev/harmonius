use core::marker::PhantomData;

use crate::StringId;

/// Marker type for montage asset definitions resolved through the content pipeline.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MontageDef;

/// Untyped asset handle for montage definitions.
#[derive(Debug, Eq, PartialEq)]
pub struct AssetHandle<T> {
    /// Stable asset identifier; `0` denotes missing / invalid for tests (FM-3).
    pub id: u32,
    _marker: PhantomData<fn() -> T>,
}

impl<T> Copy for AssetHandle<T> {}

impl<T> Clone for AssetHandle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> AssetHandle<T> {
    /// Constructs a handle with the given numeric id.
    #[must_use]
    pub const fn new(id: u32) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }

    /// Returns `true` when the handle cannot reference loaded content.
    #[must_use]
    pub const fn is_invalid(self) -> bool {
        self.id == 0
    }
}

/// High-level montage playback state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MontageState {
    /// Montage is actively blending in or playing.
    Playing,
    /// Montage finished and should be removed from the entity.
    Finished,
}

/// Runtime montage instance data carried on [`ActiveMontage`].
#[derive(Clone, Debug, PartialEq)]
pub struct MontageInstance {
    /// Asset reference; invalid handles fail load rules (FM-3).
    pub montage: AssetHandle<MontageDef>,
    /// Total montage length in seconds (test fixtures).
    pub duration_sec: f32,
    /// Seconds elapsed in the active section.
    pub elapsed_sec: f32,
    /// Blend weight applied when layering the montage over base locomotion.
    pub blend_weight: f32,
    /// Playback lifecycle marker for removal rules.
    pub state: MontageState,
    /// Notify names that fired this frame (test harness, no string intern table).
    pub notifies_fired_this_frame: Vec<String>,
}

impl MontageInstance {
    /// Advances playback by `dt` seconds, marking completion when duration elapses.
    pub fn tick(&mut self, dt: f32) {
        if self.state == MontageState::Finished {
            return;
        }
        self.elapsed_sec += dt;
        if self.elapsed_sec >= self.duration_sec {
            self.state = MontageState::Finished;
        }
    }
}

/// Component inserted by AI to request a one-shot montage (IR-1.1.3).
#[derive(Clone, Debug, PartialEq)]
pub struct ActiveMontage {
    /// Active playback instance.
    pub instance: MontageInstance,
}

impl ActiveMontage {
    /// Records that a named notify fired on the current frame (test / stub path).
    pub fn fire_notify_for_test(&mut self, name: impl Into<String>) {
        self.instance
            .notifies_fired_this_frame
            .push(name.into());
    }

    /// Clears per-frame notify bookkeeping after animation evaluation.
    pub fn clear_frame_notifies(&mut self) {
        self.instance.notifies_fired_this_frame.clear();
    }
}

/// Builds a stable [`StringId`] for notify comparisons.
#[must_use]
pub fn notify_id(name: &str) -> StringId {
    StringId::from(name)
}
