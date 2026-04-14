//! Composite asset partial re-import.

/// One clip inside a composite.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Clip {
    /// Clip index.
    pub index: u32,
    /// Payload bytes.
    pub data: Vec<u8>,
}

/// Composite with many clips.
#[derive(Clone, Debug, Default)]
pub struct CompositeAsset {
    /// Animation clips.
    pub clips: Vec<Clip>,
}

/// Which clip was edited on disk.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SubassetEdit {
    /// Clip index.
    pub clip_index: u32,
}

/// Run partial reimport; returns number of processor invocations (should be 1).
pub fn partial_subasset_reimport(comp: &mut CompositeAsset, edit: SubassetEdit) -> u32 {
    let i = edit.clip_index as usize;
    if let Some(c) = comp.clips.get_mut(i) {
        c.data.push(1);
        1
    } else {
        0
    }
}
