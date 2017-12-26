use rand::Rng;
use pcg_rand::Pcg32Basic;
use rand::SeedableRng;

use geometry::Dimensions;
use models::Playfield;

/// The data structure that contains the state of the game
pub struct GameState {
    /// The world contains everything that needs to be drawn
    pub playfield: Playfield<Pcg32Basic>,
    /// The current score of the player
    pub score: i32,
}

impl GameState {
    /// Returns a new `GameState` containing a `World` of the given `Size`
    pub fn new(size: Dimensions) -> GameState {
        let mut rng = Pcg32Basic::from_seed([42, 42]);
        GameState {
            playfield: Playfield::new(rng, size),
            score: 0,
        }
    }

    /// Reset our game-state
    pub fn reset(&mut self) {
        let mut rng = Pcg32Basic::from_seed([42, 42]);

        //clear the field and delete the currently falling piece
        self.playfield.lines.clear();
        self.playfield.live_tetromino = None;

        // Reset score
        self.score = 0;
    }
}