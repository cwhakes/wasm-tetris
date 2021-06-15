#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate rand_chacha;

mod controller;
mod game_state;
mod geometry;
mod models;

use std::os::raw::c_double;
use std::sync::Mutex;

use game_state::GameState;
use geometry::Dimensions;

lazy_static! {
    static ref DATA: Mutex<GameData> = Mutex::new(new_game_data());
}

struct GameData {
    state: GameState,
    time_controller: controller::Controller,
}

fn new_game_data() -> GameData {
    GameData {
        state: GameState::new(Dimensions{x:10,y:22}),
        time_controller: controller::Controller::new(),
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
#[no_mangle]
pub unsafe extern "C" fn draw() {
    let data = &mut DATA.lock().unwrap();
    let playfield = &data.state.playfield;

    clear_screen();
    for (y, line) in playfield.lines.iter().enumerate() {
        for (x, block) in line.iter().enumerate() {
            if block.is_some() {
                draw_block(x as f64 * 12.0, y as f64 * 12.0);
            }
            
        }
    }

    if let Some(ref tetromino) = playfield.live_tetromino {
        let loc = tetromino.location;
        for &(pos, _block) in tetromino.blocks.iter() {
            draw_block((loc+pos).x as f64 * 12.0, (loc+pos).y as f64 * 12.0);
        }
    }

    draw_score(data.state.score as f64);
}


#[no_mangle]
pub extern "C" fn update(time: c_double) {
    let data: &mut GameData = &mut DATA.lock().unwrap();
    data.time_controller.update(time, &mut data.state);
}

#[no_mangle]
pub extern "C" fn rotate_widdershins() {
    let data = &mut DATA.lock().unwrap();
    let _ = data.state.playfield.checked_trans_rot(models::Tetromino::rotate_widdershins);
}

#[no_mangle]
pub extern "C" fn rotate_sunwise() {
    let data = &mut DATA.lock().unwrap();
    let _ = data.state.playfield.checked_trans_rot(models::Tetromino::rotate_sunwise);
}

#[no_mangle]
pub extern "C" fn move_left() {
    let data = &mut DATA.lock().unwrap();
    let _ = data.state.playfield.checked_trans_rot(models::Tetromino::move_left);
}

#[no_mangle]
pub extern "C" fn move_right() {
    let data = &mut DATA.lock().unwrap();
    let _ = data.state.playfield.checked_trans_rot(models::Tetromino::move_right);
}

#[no_mangle]
pub extern "C" fn drop_block() {
    let data = &mut DATA.lock().unwrap();
    data.state.playfield.drop_block();
    //We just let the dropped tetromino lie until the controller's next cycle 
}