use game_state::GameState;
use models::Tetromino;

const UPDATE_PERIOD: f64 = 0.250;

pub struct Controller {
    current_time: f64,
    last_update: f64,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            current_time: 0.0,
            last_update: 0.0,
        }
    }

    pub fn update(&mut self, dt:f64, state: &mut GameState) {
        self.current_time += dt;
        if self.current_time >= self.last_update + UPDATE_PERIOD {
            self.last_update = self.current_time;
            if state.playfield.live_tetromino.is_none() {
                state.playfield.new_tetromino();
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