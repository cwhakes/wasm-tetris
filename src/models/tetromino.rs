use geometry::Position;
use models::Block;

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
    pub fn new(shape:TetShape, location:Position) -> Tetromino {
        let mut new_tetromino = match shape {
            TetShape::I => Tetromino::from_positions( ( 0,-1), (0,0), ( 0,1), ( 0,2) ),
            TetShape::O => Tetromino::from_positions( ( 0, 0), (1,0), ( 1,1), ( 0,1) ),
            TetShape::T => Tetromino::from_positions( ( 0,-1), (0,0), ( 0,1), ( 1,0) ),
            TetShape::J => Tetromino::from_positions( (-1, 0), (0,0), ( 0,1), ( 0,2) ),
            TetShape::L => Tetromino::from_positions( ( 1, 0), (0,0), ( 0,1), ( 0,2) ),
            TetShape::S => Tetromino::from_positions( ( 0,-1), (0,0), ( 1,0), ( 1,1) ),
            TetShape::Z => Tetromino::from_positions( ( 0,-1), (0,0), (-1,0), (-1,1) ),
        };
        new_tetromino.location = location;
        new_tetromino
    }

    fn from_positions(a:(i16, i16), b:(i16, i16), c:(i16, i16), d:(i16, i16)) -> Tetromino {
        Tetromino {
            location: Position::new(0, 0),
            blocks: [
                (Position::new(a.0, a.1), Block{}),
                (Position::new(b.0, b.1), Block{}),
                (Position::new(c.0, c.1), Block{}),
                (Position::new(d.0, d.1), Block{}),
            ]
        }
    }
}

impl Tetromino {
    pub fn rotate_sunwise(&mut self) {
        for &mut (mut pos, _block) in self.blocks.iter_mut() {
            pos.rotate_sunwise();
        };
    }

    pub fn rotate_widdershins(&mut self) {
        for &mut (mut pos, _block) in self.blocks.iter_mut() {
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
