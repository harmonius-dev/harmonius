//! Surface classification for material-pair tables.

/// Number of [`SurfaceType`] variants; keep indexing tables in sync.
pub const SURFACE_TYPE_COUNT: usize = 10;

/// Surface classification for material-pair sound lookup.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum SurfaceType {
    /// Default / unknown surface.
    Default = 0,
    /// Metal surface.
    Metal = 1,
    /// Wood surface.
    Wood = 2,
    /// Stone surface.
    Stone = 3,
    /// Dirt surface.
    Dirt = 4,
    /// Grass surface.
    Grass = 5,
    /// Ice surface.
    Ice = 6,
    /// Rubber surface.
    Rubber = 7,
    /// Water surface.
    Water = 8,
    /// Sand surface.
    Sand = 9,
}

impl SurfaceType {
    /// Converts a material tag to [`SurfaceType::Default`] when unknown.
    pub fn from_tag(tag: u8) -> Self {
        match tag {
            0 => Self::Default,
            1 => Self::Metal,
            2 => Self::Wood,
            3 => Self::Stone,
            4 => Self::Dirt,
            5 => Self::Grass,
            6 => Self::Ice,
            7 => Self::Rubber,
            8 => Self::Water,
            9 => Self::Sand,
            _ => Self::Default,
        }
    }

    /// Returns a zero-based dense index for table lookups.
    pub const fn index(self) -> usize {
        self as usize
    }
}
