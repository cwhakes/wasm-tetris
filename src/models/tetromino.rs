use rand::Rng;

use geometry::{self, Position};
use models::{Block, Playfield};

#[derive(Debug, Clone)]
pub struct Tetromino {
    pub location: Position,
    ///4 blocks because TETRis
    pub blocks: [(Position, Block); 4],
}

pub enum TetShape {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

impl Tetromino {
    pub fn rotate_sunwise<T:Rng>(&mut self) {
        for &mut (pos, _block) in self.blocks.iter_mut() {
            pos.rotate_sunwise();
        };
    }

    pub fn rotate_widdershins(&mut self) {
        for &mut (pos, _block) in self.blocks.iter_mut() {
            pos.rotate_widdershins();
        }
    }

    pub fn move_left(&mut self) {
        self.location.x -= 1;
    }

    pub fn move_right(&mut self) {
        self.location.x += 1;
    }

    pub fn move_down(&mut self) {
        self.location.y -= 1;
    }
}
