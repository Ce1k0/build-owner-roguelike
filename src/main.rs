use tcod::colors::*;
use tcod::console::*;

mod core;
use core::game::render_all;
use core::game::Game;
use core::map::{MAP_HEIGHT, MAP_WIDTH};
use core::tcod_extend::*;
fn main() {
    let mut tcod = Tcod {
        root: Root::initializer()
            .font("arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("rust/libtcod tutorial")
            .init(),
        con: Offscreen::new(MAP_WIDTH, MAP_HEIGHT),
    };
    let game = Game::new();
    let player = Player::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', RED);
    let npc = Player::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', YELLOW);
    let mut players = [player, npc];
    tcod::system::set_fps(LIMIT_FPS);
    while !tcod.root.window_closed() {
        tcod.con.clear();
        render_all(&mut tcod, &game, &players);
        tcod.root.flush();
        let exit = handle_keys(&mut tcod, &mut players[0], &game);
        if exit {
            return;
        }
    }
}
