use crate::Number;

mod display;
mod maths;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vector {
    pub x: Number,
    pub y: Number,
}

impl Vector {
    pub const ZERO: Vector = Vector { x: 0., y: 0. };
}
