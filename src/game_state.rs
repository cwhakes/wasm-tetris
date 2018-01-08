use geometry::Dimensions;
use models::Playfield;

/// The data structure that contains the state of the game
pub struct GameState {
    /// The world contains everything that needs to be drawn
    pub playfield: Playfield,
    /// The current score of the player
    pub score: i16,
}

impl GameState {
    /// Returns a new `GameState` containing a `World` of the given `Size`
    pub fn new(size: Dimensions) -> GameState {
        GameState {
            playfield: Playfield::new(size),
            score: 0,
        }
    }

    pub fn end_game(&mut self) {
        self.reset();
    }

    /// Reset our game-state
    pub fn reset(&mut self) {

        //clear the field and delete the currently falling piece
        self.playfield.lines.clear();
        self.playfield.lines.append(&mut Playfield::create_space(self.playfield.size));
        self.playfield.live_tetromino = None;

        // Reset score
        self.score = 0;
    }
}