use geometry::Position;
use models::Block;

#[derive(Debug)]
pub struct Tetromino {
    pub position: Position,
    pub blocks: Vec<(Position, Block)>,
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
    pub fn rotate_sunwise(&mut self) {
        for (pos, _block) in self.blocks {
            pos.rotate_sunwise();
        }
    }

    pub fn rotate_widdershins(&mut self) {
        for (pos, _block) in self.blocks {
            pos.rotate_widdershins();
        }
    }

    pub fn move_left(&mut self) {
        self.position.x -= 1;
    }

    pub fn move_right(&mut self) {
        self.position.x += 1;
    }

    pub fn move_down(&mut self) {
        self.position.y -= 1;
    }
}