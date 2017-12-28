use game_state::GameState;

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

    }
}