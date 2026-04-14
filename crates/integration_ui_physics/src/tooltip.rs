//! Tooltip hover state machine driven by physics pick hits.

use crate::types::{Entity, LocalizedStringId};

/// Tooltip template attached to an entity (UI component).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TooltipComponent {
    /// Title string id.
    pub title: LocalizedStringId,
    /// Body string id.
    pub body: LocalizedStringId,
    /// Seconds the cursor must rest before the tooltip appears.
    pub hover_delay_s: f32,
}

/// Published draw-list entry once a tooltip becomes visible.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TooltipRenderEntry {
    /// Source entity.
    pub entity: Entity,
    /// Title string id.
    pub title: LocalizedStringId,
    /// Body string id.
    pub body: LocalizedStringId,
}

/// Tracks hover dwell for tooltip gating.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TooltipUiState {
    hover_entity: Option<Entity>,
    elapsed_s: f32,
}

impl TooltipUiState {
    /// Creates an empty hover state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Advances hover timing and optionally emits a tooltip entry.
    pub fn update(
        &mut self,
        dt_s: f32,
        hovered: Option<Entity>,
        tooltip_for: impl Fn(Entity) -> Option<TooltipComponent>,
    ) -> Option<TooltipRenderEntry> {
        let Some(e) = hovered else {
            *self = Self::default();
            return None;
        };
        if self.hover_entity != Some(e) {
            self.hover_entity = Some(e);
            self.elapsed_s = 0.0;
        }
        self.elapsed_s += dt_s;
        let tip = tooltip_for(e)?;
        if self.elapsed_s + 1e-5 >= tip.hover_delay_s {
            Some(TooltipRenderEntry {
                entity: e,
                title: tip.title,
                body: tip.body,
            })
        } else {
            None
        }
    }
}
