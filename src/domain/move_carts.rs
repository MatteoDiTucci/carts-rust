use crate::domain::error::InvalidMovementError;
use crate::{Cart, Grid, Movement};

pub type Mission = (Cart, Vec<Movement>);
pub type MoveCart = fn(Mission, &Grid) -> Result<Grid, InvalidMovementError>;

pub const MOVE_CART: MoveCart = {
    |(cart, movements), grid| {
        let mut moved_cart: Cart = cart;

        for movement in movements {
            if IS_OUTSIDE_GRID(&grid, &movement.apply(&moved_cart)) {
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

const IS_OUTSIDE_GRID: fn(&Grid, &Cart) -> bool =
    |grid, cart| {
        cart.coordinate.x > grid.boundaries.width
            || cart.coordinate.x < 0
            || cart.coordinate.y > grid.boundaries.height
            || cart.coordinate.y < 0
    };

type AddCartToGrid = fn(Cart, &Grid) -> Grid;

const ADD_CART_TO_GRID: AddCartToGrid = {
    |cart, grid| {
        let mut result = grid.carts.clone();
        result.push(cart);

        Grid {
            boundaries: grid.boundaries.clone(),
            carts: result,
        }
    }
};