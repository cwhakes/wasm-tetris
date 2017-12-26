use rand::Rng;

use geometry::Dimensions;
use models::{Block, Tetromino};

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
            lines: vec![ vec![None;size.x as usize]; size.y as usize],
        }
    }

    ///Checks for and removes filled lines then returns number of lines eliminated
    pub fn check_and_remove_filled(&mut self) -> i16 {
        let width = self.size.x;
        let height = self.size.y;

        self.lines.retain(|line| {
            width > line.iter().filter(|block| block.is_some()).count() as i16
        });

        let lines_removed = height - self.lines.len() as i16;
        self.lines.append(&mut vec![ vec![None; width as usize ]; lines_removed as usize]);
        lines_removed
    }
}