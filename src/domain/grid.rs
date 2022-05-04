use crate::{Cart};

#[derive(Clone, Debug)]
pub struct Dimensions {
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Debug)]
pub struct Grid {
    pub boundaries: Dimensions,
    pub carts: Vec<Cart>,
}