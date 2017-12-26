use geometry::{self, Position};
use models::Block;

#[derive(Debug, Clone, Copy)]
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
    pub fn rotate_sunwise(&mut self) -> geometry::Result {
        for &mut (pos, _block) in self.blocks.iter_mut() {
            pos.rotate_sunwise();
        }
        Ok(())
    }

    pub fn rotate_widdershins(&mut self) -> geometry::Result {
        for &mut (pos, _block) in self.blocks.iter_mut() {
            pos.rotate_widdershins();
        }
        Ok(())
    }

    pub fn move_left(&mut self) -> geometry::Result {
        self.location.x -= 1;
        Ok(())
    }

    pub fn move_right(&mut self) -> geometry::Result {
        self.location.x += 1;
        Ok(())
    }

    pub fn move_down(&mut self) -> geometry::Result {
        self.location.y -= 1;
        Ok(())
    }
}
