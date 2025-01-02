use tcod::console::blit;
use tcod::Console;

use super::map::*;
use super::tcod_extend::*;

type Map = Vec<Vec<Tile>>;
pub struct Game {
    pub map: Map,
}

impl Game {
    pub fn new() -> Self {
        Self { map: make_map() }
    }
}

pub fn make_map() -> Map {
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    map[30][22] = Tile::wall();
    map[50][22] = Tile::wall();
    map
}

pub fn render_all(tcod: &mut Tcod, game: &Game, players: &[Player]) {
    for player in players {
        player.draw(&mut tcod.con);
    }
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = game.map[x as usize][y as usize].block_sight;
            if wall {
                tcod.con
                    .set_char_background(x, y, COLOR_DARK_WALL, tcod::BackgroundFlag::Set);
            } else {
                tcod.con
                    .set_char_background(x, y, COLOR_DARK_GROUPD, tcod::BackgroundFlag::Set);
            }
        }
    }
    blit(
        &tcod.con,
        (0, 0),
        (MAP_WIDTH, MAP_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0,
    );
}
