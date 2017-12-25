#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate pcg_rand;

mod game_state;
mod geometry;
mod models;

use std::os::raw::{c_double, c_int};
use std::sync::Mutex;

use pcg_rand::Pcg32Basic;
use rand::SeedableRng;

use game_state::GameState;
use geometry::Dimensions;

lazy_static! {
    static ref DATA: Mutex<GameData> = Mutex::new(new_game_data());
}

struct GameData {
    state: GameState,
    //time_controller: TimeController<Pcg32Basic>
}

fn new_game_data() -> GameData {
    GameData {
        state: GameState::new(Dimensions{x:10,y:22}),
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

//Unsafe because we're calling external draw functions. Safe otherwise.
pub unsafe extern "C" fn draw() {
    let data = &mut DATA.lock().unwrap();
    let playfield = &data.state.playfield;

    clear_screen();
    for (y, line) in &playfield.lines.iter().enumerate() {
        for (x, block) in &line.iter().enumerate() {
            if block.filled() {
                draw_block(x * 8, y * 8);
            }
            
        }
    }

    let loc = &playfield.live_tetromino.location;
    for block in &playfield.live_tetromino.blocks {
        draw_block(block.x + loc.x, block.y + loc.y);
    }

    draw_score(data.state.score as f64);
}


#[no_mangle]
pub extern "C" fn update(time: c_double) {
    let data: &mut GameData = &mut DATA.lock().unwrap();
    data.time_controller.update_seconds(time, &mut data.state);
}

#[no_mangle]
pub extern "C" fn rotate_widdershins() {
    let data = &mut DATA.lock().unwrap();
    data.state.playfield.live_tetromino.rotate_widdershins();
}

#[no_mangle]
pub extern "C" fn rotate_sunwise() {
    let data = &mut DATA.lock().unwrap();
    data.state.playfield.live_tetromino.rotate_sunwise();
}

#[no_mangle]
pub extern "C" fn move_left() {
    let data = &mut DATA.lock().unwrap();
    data.state.playfield.live_tetromino.move_left();
}

#[no_mangle]
pub extern "C" fn move_right() {
    let data = &mut DATA.lock().unwrap();
    data.state.playfield.live_tetromino.move_right();
}

#[no_mangle]
pub extern "C" fn drop_block() {
    let data = &mut DATA.lock().unwrap();
    data.state.playfield.live_tetromino.drop_block();
}