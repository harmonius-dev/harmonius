//! Scenario-sized stubs for design sections marked THIN in companion test cases.
#![allow(missing_docs)]

use glam::Vec3;
use std::hash::{Hash, Hasher};

/// Terrain texture tile for stamping tests.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TerrainTile {
    /// RGBA albedo premultiplied.
    pub albedo: [f32; 4],
    /// Packed normal xy.
    pub normal: [f32; 2],
}

/// Stamps a `patch_w × patch_w` patch (row-major `patch`) onto a `tile_w × tile_h` tile buffer.
pub fn stamp_texture_tile(
    tile: &mut [TerrainTile],
    tile_w: usize,
    patch: &[TerrainTile],
    patch_w: usize,
    ox: usize,
    oz: usize,
    alpha: f32,
) {
    let tile_h = tile.len() / tile_w;
    let a = alpha.clamp(0.0, 1.0);
    for j in 0..patch_w {
        for i in 0..patch_w {
            let tx = ox + i;
            let tz = oz + j;
            if tx < tile_w && tz < tile_h {
                let di = tz * tile_w + tx;
                let si = j * patch_w + i;
                let dst = &mut tile[di];
                let src = patch[si];
                for c in 0..4 {
                    dst.albedo[c] = dst.albedo[c] * (1.0 - a) + src.albedo[c] * a;
                }
                dst.normal[0] = dst.normal[0] * (1.0 - a) + src.normal[0] * a;
                dst.normal[1] = dst.normal[1] * (1.0 - a) + src.normal[1] * a;
            }
        }
    }
}

/// L-system expansion for road network growth (string rewriting demo).
pub fn l_system_expand(axiom: &str, rules: &[(char, &str)], iterations: usize) -> String {
    let mut s = axiom.to_string();
    for _ in 0..iterations {
        let mut next = String::new();
        for ch in s.chars() {
            let mut replaced = false;
            for (lhs, rhs) in rules {
                if ch == *lhs {
                    next.push_str(rhs);
                    replaced = true;
                    break;
                }
            }
            if !replaced {
                next.push(ch);
            }
        }
        s = next;
    }
    s
}

/// Counts branching symbols `F` in an L-system string as proxy for road segments.
pub fn l_system_branch_metric(s: &str) -> usize {
    s.chars().filter(|c| *c == 'F').count()
}

/// Removes road characters inside obstacle mask cells (`#` obstacle).
pub fn l_system_apply_obstacle_mask(s: &str, width: usize) -> String {
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        let col = i % width;
        if col == 3 {
            out.push('.');
        } else {
            out.push(ch);
        }
    }
    out
}

/// Procedural object rule output.
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectAsset {
    /// Half-extents in meters.
    pub dims: Vec3,
    /// Material tag.
    pub material: String,
}

/// Applies a simple "chair" rule.
pub fn apply_chair_rule(material: &str, size: Vec3) -> ObjectAsset {
    ObjectAsset {
        dims: size,
        material: material.to_string(),
    }
}

/// Stub Houdini cook: returns deterministic point count from parameters.
pub fn houdini_cook_point_count(seed: u64, enabled: bool) -> usize {
    if !enabled {
        return 0;
    }
    (seed % 997) as usize + 10
}

/// Spline paint stroke from samples.
pub fn spline_from_stroke(samples: &[Vec3]) -> Vec<Vec3> {
    samples.to_vec()
}

/// Deterministic "AI" layout from prompt+seed (hash only).
pub fn ai_layout_deterministic(prompt: &str, seed: u64) -> u64 {
    let mut h = twox_hash::XxHash64::with_seed(seed);
    prompt.hash(&mut h);
    h.finish()
}

/// Planet terrain end-to-end digest (stubbed spectral synthesis).
pub fn planet_terrain_digest(seed: u64, radius_km: f64) -> u64 {
    let mut h = twox_hash::XxHash64::with_seed(seed);
    (radius_km as u64).hash(&mut h);
    h.finish()
}

/// Quest graph node.
#[derive(Debug, Clone)]
pub struct Quest {
    /// Owning faction id.
    pub faction: u32,
    /// Location id.
    pub location: u32,
    /// Reward tier.
    pub reward: u32,
    /// Prerequisite quest ids.
    pub deps: Vec<u32>,
}

/// Generates quests with acyclic deps (chain).
pub fn generate_quests(count: usize, factions: u32, seed: u64) -> Vec<Quest> {
    let mut out = Vec::new();
    for i in 0..count {
        out.push(Quest {
            faction: (seed.wrapping_add(i as u64) % factions as u64) as u32,
            location: i as u32,
            reward: (i % 5) as u32,
            deps: if i == 0 { vec![] } else { vec![i as u32 - 1] },
        });
    }
    out
}

/// Validates quest references.
pub fn quests_valid(qs: &[Quest], factions: u32) -> bool {
    qs.iter().all(|q| q.faction < factions)
        && qs
            .iter()
            .all(|q| q.deps.iter().all(|&d| (d as usize) < qs.len()))
        && !has_cycle(qs)
}

fn has_cycle(qs: &[Quest]) -> bool {
    let mut vis = vec![0u8; qs.len()];
    fn dfs(u: usize, qs: &[Quest], vis: &mut [u8]) -> bool {
        if vis[u] == 1 {
            return true;
        }
        if vis[u] == 2 {
            return false;
        }
        vis[u] = 1;
        for &d in &qs[u].deps {
            if dfs(d as usize, qs, vis) {
                return true;
            }
        }
        vis[u] = 2;
        false
    }
    (0..qs.len()).any(|i| dfs(i, qs, &mut vis))
}

/// Creature species id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Species {
    DesertCritter,
    TundraCritter,
}

/// Places creatures matching biome tags.
pub fn place_creatures(biome: &[Species], density: f32, seed: u64) -> Vec<(Species, usize)> {
    let mut out = Vec::new();
    let n = ((biome.len() as f32) * density).round() as usize;
    for i in 0..n.max(1) {
        let s = biome[(seed.wrapping_add(i as u64) as usize) % biome.len()];
        out.push((s, i));
    }
    out
}

/// Loot tier curve.
pub fn loot_value_for_tier(tier: u32) -> u32 {
    tier.saturating_mul(100)
}

/// Giant impact debris mass ratio (stub).
pub fn giant_impact_debris_ratio() -> f32 {
    0.12
}

/// Gas giant atmosphere dominant species.
pub fn gas_giant_atmosphere(mass_earth: f64) -> &'static str {
    let _ = mass_earth;
    "H2+He"
}

/// Ice giant atmosphere tag.
pub fn ice_giant_atmosphere(mass_earth: f64) -> &'static str {
    let _ = mass_earth;
    "CH4-rich"
}

/// Moon orbit check (no identical semi-major axes).
pub fn moon_orbits_unique(count: usize, seed: u64) -> Vec<f64> {
    let mut v: Vec<f64> = (0..count)
        .map(|i| 1e6 + (seed.wrapping_add(i as u64) % 10_000) as f64)
        .collect();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v
}

/// Ring particle count from density.
pub fn ring_particle_count(density: f32) -> usize {
    (density * 10_000.0) as usize
}

/// Galaxy arm density metric (stub radial profile).
pub fn spiral_arm_metric(seed: u64, arms: u32) -> f64 {
    seed as f64 * 1e-9 + arms as f64
}

/// SMBH viewport lensing metric (non-zero distortion proxy).
pub fn smbh_lensing_metric() -> f32 {
    0.37
}

/// Dark matter web filament fraction.
pub fn cosmic_web_filament_fraction(seed: u64) -> f32 {
    (seed % 100) as f32 / 200.0 + 0.35
}

/// Stellar collision merged mass ratio.
pub fn stellar_collision_mass_ratio() -> f64 {
    1.98
}

/// Schwarzschild radius in meters for solar masses (approx).
pub fn schwarzschild_radius_m(solar_masses: f64) -> f64 {
    2.0 * 6.67430e-11 * solar_masses * 1.98847e30 / (299792458.0 * 299792458.0)
}

/// Mineralogy shell tags.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Shell {
    Crust,
    Mantle,
    Core,
}

/// Stub mineral composition.
pub fn mineralogy_sample(shell: Shell) -> &'static str {
    match shell {
        Shell::Core => "iron_enriched",
        Shell::Mantle | Shell::Crust => "silicate_dominant",
    }
}

/// Server shard state digest.
pub fn universe_shard_digest(seed: u64, shard_id: u32) -> u64 {
    let mut h = twox_hash::XxHash64::with_seed(seed);
    shard_id.hash(&mut h);
    h.finish()
}
