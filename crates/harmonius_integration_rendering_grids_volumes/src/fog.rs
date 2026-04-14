//! Fog-of-war cell encoding for R8 uploads (IR-3.3.1).

/// Discrete visibility used by tactical fog grids.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FogCellState {
    /// Unexplored / hidden from the player.
    Hidden,
    /// Explored but not currently visible.
    Explored,
    /// In line of sight this frame.
    Visible,
}

/// Maps a fog cell to the normalized R8 byte the design specifies.
#[must_use]
pub fn fog_cell_to_r8(state: FogCellState) -> u8 {
    match state {
        FogCellState::Hidden => 0,
        FogCellState::Explored => 128,
        FogCellState::Visible => 255,
    }
}

#[cfg(test)]
mod tests {
    use super::{fog_cell_to_r8, FogCellState};

    /// TC-IR-3.3.1.2 — three-state R8 values.
    #[test]
    fn fog_three_state_r8_values() {
        assert_eq!(fog_cell_to_r8(FogCellState::Hidden), 0);
        assert_eq!(fog_cell_to_r8(FogCellState::Explored), 128);
        assert_eq!(fog_cell_to_r8(FogCellState::Visible), 255);
    }
}
