use crate::{Cart, Movement};

#[derive(Debug, PartialEq)]
pub struct InvalidMove {
    pub movement: Movement,
    pub cart: Cart
}