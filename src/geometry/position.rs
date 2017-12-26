use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

impl Position {
    pub fn new(x:i16, y:i16) -> Position {
        Position {
            x: x,
            y: y,
        }
    }

    pub fn rotate_sunwise(&mut self) {
        let temp = self.y;
        self.y = -self.x;
        self.x = temp;
    }
    
    pub fn rotate_widdershins(&mut self) {
        let temp = self.x;
        self.x = -self.y;
        self.y = temp;
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}