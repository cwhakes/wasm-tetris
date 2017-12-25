use rand::Rng;

use geometry::Dimensions;

#[derive(Debug)]
pub struct Playfield<T: Rng> {
    rng: T,
    pub size: Dimensions,
    pub live_tetromino: Option<Tetromino>,
    pub lines: Vec<Vec<Option<Block>>>,
}

impl<T: Rng> Playfield<T> {
    pub fn new(rng:T, size:Dimensions) -> Playfield<T> {
        Playfield {
            rng: rng,
            size: size,
            live_tetromino: None,
            lines: vec![ vec![None;size.x as usize ]; size.y as usize],
        }
    }
}