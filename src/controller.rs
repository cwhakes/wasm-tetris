use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use game_state::GameState;
use models::{Tetromino, TetShape};

const UPDATE_PERIOD: f64 = 2.50; //time per step in seconds (gets divided by 10+score)

pub struct Controller {
    current_time: f64,
    last_update: f64,
    rng: ChaCha8Rng,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            current_time: 0.0,
            last_update: 0.0,
            rng: ChaCha8Rng::from_seed(Default::default())
        }
    }

    pub fn update(&mut self, dt:f64, state: &mut GameState) {
        self.current_time += dt;
        if self.current_time >= self.last_update + (UPDATE_PERIOD / (10 + state.score) as f64) {
            self.last_update = self.current_time;
            if state.playfield.live_tetromino.is_none() {
                let shape = TetShape::random(&mut self.rng);
                if state.playfield.new_tetromino(shape).is_err() {
                    state.end_game();
                }
            } else {
                if state.playfield.checked_trans_rot(Tetromino::move_down).is_err() {
                    state.playfield.lock_tetromino();
                    let score_up = state.playfield.check_and_remove_filled();
                    state.score += score_up;
                }
            }
        }
    }
}