mod position;

pub use self::position::Position;

#[derive(Debug)]
pub struct Dimensions {
    pub x: i16,
    pub y: i16,
}