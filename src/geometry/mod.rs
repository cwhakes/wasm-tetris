#[derive(Debug)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

impl Position {
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

#[derive(Debug)]
pub struct Dimensions {
    pub x: i16,
    pub y: i16,
}