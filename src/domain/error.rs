use crate::{Cart, Movement};

#[derive(Debug, PartialEq)]
pub struct InvalidMovementError {
    pub movement: Movement,
    pub cart: Cart
}