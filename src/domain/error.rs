use crate::{Cart, Movement};

#[derive(Debug, PartialEq)]
pub struct ForbiddenMovementError {
    pub movement: Movement,
    pub cart: Cart
}