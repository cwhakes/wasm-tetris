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
    for (y, line) in playfield.lines.iter().enumerate() {
        for (x, block) in line.iter().enumerate() {
            if block.is_some() {
                draw_block(x as f64 * 8.0, y as f64 * 8.0);
            }
            
        }
    }

    if let &Some(tetromino) = &playfield.live_tetromino {
        let loc = tetromino.location;
        for &(pos, _block) in tetromino.blocks.iter() {
            draw_block((loc.x + pos.x) as f64 * 8.0, (loc.y + pos.y) as f64 * 8.0);
        }
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
    if let Some(tetromino) = data.state.playfield.live_tetromino {
        let _ = tetromino.rotate_widdershins();
    }
}

#[no_mangle]
pub extern "C" fn rotate_sunwise() {
    let data = &mut DATA.lock().unwrap();
    let _ = data.state.playfield.rotate_live_sw();
}

#[no_mangle]
pub extern "C" fn move_left() {
    let data = &mut DATA.lock().unwrap();
    if let Some(tetromino) = data.state.playfield.live_tetromino {
        let _ = tetromino.move_left();
    }
}

#[no_mangle]
pub extern "C" fn move_right() {
    let data = &mut DATA.lock().unwrap();
    if let Some(tetromino) = data.state.playfield.live_tetromino {
        let _ = tetromino.move_right();
    }
}

#[no_mangle]
pub extern "C" fn drop_block() {
    let data = &mut DATA.lock().unwrap();
    data.state.playfield.drop_block();
    //We just let the dropped tetromino lie until the controller's next cycle 
}