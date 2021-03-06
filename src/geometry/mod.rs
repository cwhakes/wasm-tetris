mod position;
mod result;

pub use self::position::Position;
pub use self::result::{Result, CauseOfFailure};

#[derive(Debug, Clone, Copy)]
pub struct Dimensions {
    pub x: i16,
    pub y: i16,
}