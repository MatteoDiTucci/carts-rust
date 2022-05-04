use crate::{Cart, Grid, Movement};
use crate::domain::error::InvalidMovementError;
use crate::domain::grid::{ADD_CART_TO_GRID, IS_MOVEMENT_FORBIDDEN};

pub type Mission = (Cart, Vec<Movement>);
pub type MoveCart = fn(Mission, &Grid) -> Result<Grid, InvalidMovementError>;

pub const MOVE_CART: MoveCart = {
    |(cart, movements), grid| {
        let mut moved_cart = cart;

        for movement in movements {
            if IS_MOVEMENT_FORBIDDEN(&grid, &moved_cart, &movement) {
                return Err(
                    InvalidMovementError {
                        movement,
                        cart: moved_cart,
                    }
                );
            }
            moved_cart = movement.apply(&moved_cart);
        }

        Ok(
            ADD_CART_TO_GRID(moved_cart, grid)
        )
    }
};