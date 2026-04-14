//! Sprite batching and draw-order sorting.

use crate::types::{BlendMode, LayerZ};

/// One instanced draw batch sharing atlas and blend state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpriteBatch {
    /// Atlas identifier shared by all instances in this batch.
    pub atlas: u32,
    /// Blend mode for the batch.
    pub blend: BlendMode,
    /// Entity indices grouped into this batch.
    pub entities: Vec<u32>,
}

/// Group sprites for instancing by atlas only (`TC-10.5.1.1`).
#[must_use]
pub fn sprite_batch_by_atlas(sprites: &[(u32, u32)]) -> Vec<SpriteBatch> {
    let mut buckets: Vec<(u32, Vec<u32>)> = Vec::new();
    for &(entity, atlas) in sprites {
        if let Some((_, v)) = buckets.iter_mut().find(|(a, _)| *a == atlas) {
            v.push(entity);
        } else {
            buckets.push((atlas, vec![entity]));
        }
    }
    buckets.sort_by_key(|(a, _)| *a);
    buckets
        .into_iter()
        .map(|(atlas, entities)| SpriteBatch {
            atlas,
            blend: BlendMode::AlphaBlend,
            entities,
        })
        .collect()
}

/// Group sprites by `(atlas, blend)` pair (`TC-10.5.1.2`).
#[must_use]
pub fn sprite_batch_by_atlas_and_blend(sprites: &[(u32, u32, BlendMode)]) -> Vec<SpriteBatch> {
    let mut keys: Vec<(u32, BlendMode)> = Vec::new();
    let mut map: Vec<Vec<u32>> = Vec::new();
    for &(entity, atlas, blend) in sprites {
        let key = (atlas, blend);
        if let Some(i) = keys.iter().position(|k| *k == key) {
            map[i].push(entity);
        } else {
            keys.push(key);
            map.push(vec![entity]);
        }
    }
    let mut order: Vec<usize> = (0..keys.len()).collect();
    order.sort_by_key(|&i| keys[i]);
    order
        .into_iter()
        .map(|i| SpriteBatch {
            atlas: keys[i].0,
            blend: keys[i].1,
            entities: map[i].clone(),
        })
        .collect()
}

/// Stable sort for back-to-front draw order: ascending `sort_layer`, then ascending `z_order`.
#[must_use]
pub fn sort_sprite_draw_order(layers: &[LayerZ]) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..layers.len()).collect();
    idx.sort_by(|&a, &b| {
        layers[a]
            .sort_layer
            .cmp(&layers[b].sort_layer)
            .then_with(|| layers[a].z_order.total_cmp(&layers[b].z_order))
    });
    idx
}
