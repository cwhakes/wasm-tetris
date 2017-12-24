#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate pcg_rand;

use std::os::raw::{c_double, c_int};
use std::sync::Mutex;

use pcg_rand::Pcg32Basic;
use rand::SeedableRng;

lazy_static! {
    static ref DATA: Mutex<GameData> = Mutex::new(new_game_data(80.0, 220.0));
}

struct GameData {
    state: GameState,
    //actions: Actions,
    //time_controller: TimeController<Pcg32Basic>
}

fn new_game_data(width: f64, height: f64) -> GameData {
    GameData {
        state: GameState::new(Size::new(width, height)),
        //actions: Actions::default(),
        //time_controller: TimeController::new(Pcg32Basic::from_seed([42, 42]))
    }
}

extern "C" {
    fn clear_screen();
    fn draw_block(_: c_double, _: c_double);
    fn draw_score(_: c_double);
}
/*
#[no_mangle]
pub extern "C" fn resize(width: c_double, height: c_double) {
    *DATA.lock().unwrap() = new_game_data(width, height);
}
*/

pub unsafe extern "C" fn draw() {
    let data = &mut DATA.lock().unwrap();
    let playfield = &data.state.playfield;

    clear_screen();
    for (y, line) in &world.lines.iter().enumerate() {
        for (x, block) in &line.iter().enumerate() {
            if block.filled() {
                draw_block(x * 8, y * 8);
            }
            
        }
    }

    let loc = &world.live_tetromino.location;
    for block in &world.live_tetromino.blocks {
        draw_block(block.x + loc.x, block.y + loc.y);
    }

    draw_player(world.player.x(), world.player.y(), world.player.direction());
    draw_score(data.state.score as f64);
}
