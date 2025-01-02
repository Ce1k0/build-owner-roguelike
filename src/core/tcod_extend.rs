use super::game::Game;
use tcod::colors::*;
use tcod::console::*;
use tcod::input::KeyCode::*;
use tcod::input::*;

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;
pub const LIMIT_FPS: i32 = 20;

pub struct Tcod {
    pub root: Root,
    pub con: Offscreen,
}

#[derive(Debug)]
pub struct Player {
    pub x: i32,
    pub y: i32,
    char: char,
    color: Color,
}

impl Player {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Self { x, y, char, color }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32, game: &Game) {
        if !game.map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
            self.x += dx;
            self.y += dy;
        }
    }
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

pub fn handle_keys(tcod: &mut Tcod, player: &mut Player, game: &Game) -> bool {
    let key = tcod.root.wait_for_keypress(true);
    match key {
        Key { code: Up, .. } => player.move_by(0, -1, game),
        Key { code: Down, .. } => player.move_by(0, 1, game),
        Key { code: Left, .. } => player.move_by(-1, 0, game),
        Key { code: Right, .. } => player.move_by(1, 0, game),
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true,
        _ => {}
    }
    false
}
