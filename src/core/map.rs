use tcod::colors::*;
pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 45;

pub const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
pub const COLOR_DARK_GROUPD: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};
#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
}

impl Tile {
    pub fn new(blocked: bool, block_sight: bool) -> Self {
        Self {
            blocked,
            block_sight,
        }
    }
    pub fn empty() -> Self {
        Self {
            blocked: false,
            block_sight: false,
        }
    }
    pub fn wall() -> Self {
        Self {
            blocked: true,
            block_sight: true,
        }
    }
}
