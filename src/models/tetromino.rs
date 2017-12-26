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
    pub rotate_sunwise(&mut self) {
        for (&mut pos, &mut _block) in self.blocks {
            pos.rotate_sunwise();
        }
    }
    
    pub rotate_widdershins(&mut self) {
        for (&mut pos, &mut _block) in self.blocks {
            pos.rotate_widdershins();
        }
    }
}