use geometry::{Dimensions, Position, self};
use models::{Block, Tetromino, TetShape};

#[derive(Debug)]
pub struct Playfield {
    pub size: Dimensions,
    pub live_tetromino: Option<Tetromino>,
    pub lines: Vec<Vec<Option<Block>>>,
}

impl Playfield {
    pub fn new(size:Dimensions) -> Playfield {
        Playfield {
            size: size,
            live_tetromino: None,
            lines: Playfield::create_space(size),
        }
    }

    pub fn create_space(size: Dimensions) -> Vec<Vec<Option<Block>>> {
        vec![ vec![None;size.x as usize]; size.y as usize]
    }

    pub fn new_tetromino(&mut self, shape:TetShape) -> geometry::Result {
        let tetromino = Tetromino::new(shape, Position::new(4,19));
        if self.has_room_for(&tetromino) {
            self.live_tetromino = Some(tetromino);
            Ok(())
        } else {
            Err(geometry::CauseOfFailure::CantFit)
        }
    }

    pub fn lock_tetromino(&mut self) {
        if let Some(tetromino) = self.live_tetromino.take() {
            let loc = tetromino.location;
            for &(pos, block) in tetromino.blocks.iter() {
                self.lines[(loc+pos).y as usize][(loc+pos).x as usize] = Some(block);
            }
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
        self.lines.append(&mut Playfield::create_space(Dimensions{x:width, y:lines_removed}));
        lines_removed
    }

    pub fn has_room_for(&self, tetromino: &Tetromino) -> bool {

        let positions = tetromino.blocks.iter()
            .map(|&(pos, _block)| pos + tetromino.location)
            .collect::<Vec<Position>>();

        for position in positions.clone() {
            if position.x < 0
               || position.x >= self.size.x
               || position.y < 0
               || position.y >= self.size.y
            {
                   return false;
            }
        }

        for (y, line) in self.lines.iter().enumerate() {
            for (x, block) in line.iter().enumerate() {
                if block.is_some() {
                    for position in positions.clone() {
                        if position == Position::new(x as i16, y as i16) {
                            return false;
                        }
                    }
                }
            }
        }

        return true;
    }

    pub fn checked_trans_rot<F>(&mut self, closure: F) -> geometry::Result 
        where F: FnOnce(&mut Tetromino)
    {
        if let Some(tetromino) = self.live_tetromino.take() {
            let mut new_tetromino = tetromino.clone();

            closure(&mut new_tetromino);

            if self.has_room_for(&new_tetromino) {
                self.live_tetromino = Some(new_tetromino);
                Ok(())
            } else {
                self.live_tetromino = Some(tetromino);
                Err(geometry::CauseOfFailure::CantFit)
            }
        } else {
            Err(geometry::CauseOfFailure::NoObject)
        }

    }

    pub fn drop_block(&mut self) {
        while Ok(()) == self.checked_trans_rot(Tetromino::move_down) {}
    }
}