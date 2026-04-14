//! Small 2D / 3D wave function collapse solvers for design acceptance tests.

/// Adjacency compatibility: `compat[a][b]` allows `b` east or south of `a`.
pub type CompatTable = [[bool; 5]; 5];

/// Fully permissive 5-tile adjacency.
pub fn compat_five_tile_mesh() -> CompatTable {
    [[true; 5]; 5]
}

/// Ring adjacency: each tile only neighbors its ring neighbors.
pub fn compat_five_tile_ring() -> CompatTable {
    let mut t = [[false; 5]; 5];
    for (a, row) in t.iter_mut().enumerate() {
        for (b, cell) in row.iter_mut().enumerate() {
            let d = (a as i32 - b as i32).abs();
            *cell = d == 1 || d == 4;
        }
    }
    t
}

fn idx2(w: usize, x: usize, y: usize) -> usize {
    y * w + x
}

fn ok_local(
    w: usize,
    h: usize,
    grid: &[Option<u8>],
    x: usize,
    y: usize,
    val: u8,
    compat: &CompatTable,
) -> bool {
    if x > 0 {
        if let Some(left) = grid[idx2(w, x - 1, y)] {
            if !compat[left as usize][val as usize] {
                return false;
            }
        }
    }
    if y > 0 {
        if let Some(up) = grid[idx2(w, x, y - 1)] {
            if !compat[up as usize][val as usize] {
                return false;
            }
        }
    }
    if x + 1 < w {
        if let Some(right) = grid[idx2(w, x + 1, y)] {
            if !compat[val as usize][right as usize] {
                return false;
            }
        }
    }
    if y + 1 < h {
        if let Some(down) = grid[idx2(w, x, y + 1)] {
            if !compat[val as usize][down as usize] {
                return false;
            }
        }
    }
    true
}

/// Deterministic LCG for shuffling visit order.
#[derive(Clone, Copy)]
struct Lcg(u64);

impl Lcg {
    fn new(seed: u64) -> Self {
        Self(if seed == 0 { 0xC0FFEE } else { seed })
    }

    fn next_u32(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
        (self.0 >> 32) as u32
    }
}

fn dfs_fill(
    w: usize,
    h: usize,
    compat: &CompatTable,
    grid: &mut [Option<u8>],
    order: &[(usize, usize)],
    oi: usize,
) -> bool {
    if oi == order.len() {
        return true;
    }
    let (x, y) = order[oi];
    let i = idx2(w, x, y);
    if grid[i].is_some() {
        return dfs_fill(w, h, compat, grid, order, oi + 1);
    }
    for t in 0u8..5 {
        if ok_local(w, h, grid, x, y, t, compat) {
            grid[i] = Some(t);
            if dfs_fill(w, h, compat, grid, order, oi + 1) {
                return true;
            }
            grid[i] = None;
        }
    }
    false
}

/// Solves a `w × h` grid with tiles `0..5` and optional pinned cell.
pub fn solve_wfc_2d(
    w: usize,
    h: usize,
    compat: &CompatTable,
    seed: u64,
    pin: Option<(usize, usize, u8)>,
) -> Option<Vec<u8>> {
    let mut grid = vec![None; w * h];
    if let Some((px, py, pv)) = pin {
        grid[idx2(w, px, py)] = Some(pv);
    }
    let mut rng = Lcg::new(seed);
    let mut order: Vec<(usize, usize)> = (0..h).flat_map(|y| (0..w).map(move |x| (x, y))).collect();
    for i in (1..order.len()).rev() {
        let j = (rng.next_u32() as usize) % (i + 1);
        order.swap(i, j);
    }
    if dfs_fill(w, h, compat, &mut grid, &order, 0) {
        Some(grid.into_iter().map(|c| c.expect("complete")).collect())
    } else {
        None
    }
}

fn idx3(w: usize, d: usize, x: usize, y: usize, z: usize) -> usize {
    (z * d + y) * w + x
}

#[allow(clippy::too_many_arguments)]
fn ok_local_3d(
    w: usize,
    d: usize,
    h: usize,
    grid: &[Option<u8>],
    x: usize,
    y: usize,
    z: usize,
    val: u8,
    compat: &CompatTable,
) -> bool {
    if x > 0 {
        if let Some(other) = grid[idx3(w, d, x - 1, y, z)] {
            if !compat[other as usize][val as usize] {
                return false;
            }
        }
    }
    if y > 0 {
        if let Some(other) = grid[idx3(w, d, x, y - 1, z)] {
            if !compat[other as usize][val as usize] {
                return false;
            }
        }
    }
    if z > 0 {
        if let Some(other) = grid[idx3(w, d, x, y, z - 1)] {
            if !compat[other as usize][val as usize] {
                return false;
            }
        }
    }
    if x + 1 < w {
        if let Some(other) = grid[idx3(w, d, x + 1, y, z)] {
            if !compat[val as usize][other as usize] {
                return false;
            }
        }
    }
    if y + 1 < d {
        if let Some(other) = grid[idx3(w, d, x, y + 1, z)] {
            if !compat[val as usize][other as usize] {
                return false;
            }
        }
    }
    if z + 1 < h {
        if let Some(other) = grid[idx3(w, d, x, y, z + 1)] {
            if !compat[val as usize][other as usize] {
                return false;
            }
        }
    }
    true
}

fn dfs_fill_3d(
    w: usize,
    d: usize,
    h: usize,
    compat: &CompatTable,
    grid: &mut [Option<u8>],
    order: &[(usize, usize, usize)],
    oi: usize,
) -> bool {
    if oi == order.len() {
        return true;
    }
    let (x, y, z) = order[oi];
    let i = idx3(w, d, x, y, z);
    if grid[i].is_some() {
        return dfs_fill_3d(w, d, h, compat, grid, order, oi + 1);
    }
    for t in 0u8..5 {
        if ok_local_3d(w, d, h, grid, x, y, z, t, compat) {
            grid[i] = Some(t);
            if dfs_fill_3d(w, d, h, compat, grid, order, oi + 1) {
                return true;
            }
            grid[i] = None;
        }
    }
    false
}

/// Solves `w × d × h` with the same 5-tile vocabulary.
pub fn solve_wfc_3d(
    w: usize,
    d: usize,
    h: usize,
    compat: &CompatTable,
    seed: u64,
) -> Option<Vec<u8>> {
    let mut rng = Lcg::new(seed.wrapping_add(0x3D));
    let mut order: Vec<(usize, usize, usize)> = (0..h)
        .flat_map(|z| (0..d).flat_map(move |y| (0..w).map(move |x| (x, y, z))))
        .collect();
    for i in (1..order.len()).rev() {
        let j = (rng.next_u32() as usize) % (i + 1);
        order.swap(i, j);
    }
    let mut grid = vec![None; w * d * h];
    if dfs_fill_3d(w, d, h, compat, &mut grid, &order, 0) {
        Some(grid.into_iter().map(|c| c.expect("complete")).collect())
    } else {
        None
    }
}

/// Verifies all local adjacency constraints on a finished 2D grid.
pub fn verify_wfc_2d(grid: &[u8], w: usize, h: usize, compat: &CompatTable) -> bool {
    let g: Vec<Option<u8>> = grid.iter().copied().map(Some).collect();
    for y in 0..h {
        for x in 0..w {
            let v = g[idx2(w, x, y)].expect("tile");
            if !ok_local(w, h, &g, x, y, v, compat) {
                return false;
            }
        }
    }
    true
}

/// Two adjacent `8×8×8` chunks sharing an `8×8` face (`+x` boundary).
pub fn solve_two_chunks_8(compat: &CompatTable, seed: u64) -> Option<(Vec<u8>, Vec<u8>)> {
    let left = solve_wfc_3d(8, 8, 8, compat, seed)?;
    let right = solve_wfc_3d(8, 8, 8, compat, seed ^ 0x51EE)?;
    for y in 0..8usize {
        for z in 0..8usize {
            let a = left[idx3(8, 8, 7, y, z)] as usize;
            let b = right[idx3(8, 8, 0, y, z)] as usize;
            if !compat[a][b] {
                return None;
            }
        }
    }
    Some((left, right))
}
