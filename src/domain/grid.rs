use crate::{Cart, Movement, Race};
use crate::domain::cart::MOVE_CART;
use crate::domain::error::ForbiddenMovementError;

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

type RunRaceOnGrid = fn(Race, &Grid) -> Result<Grid, ForbiddenMovementError>;

pub const RUN_RACE_ON_GRID: RunRaceOnGrid = {
    |(cart, movements), grid| {
        let mut moved_cart = cart;

        for movement in movements {
            if IS_MOVEMENT_FORBIDDEN(&grid, &moved_cart, &movement) {
                return Err(
                    ForbiddenMovementError {
                        movement,
                        cart: moved_cart,
                    }
                );
            }
            moved_cart = MOVE_CART(&moved_cart, &movement);
        }

        Ok(
            ADD_CART_TO_GRID(moved_cart, grid)
        )
    }
};

const ADD_CART_TO_GRID: fn(Cart, &Grid) -> Grid = {
    |cart, grid| {
        let mut result = grid.carts.clone();
        result.push(cart);

        Grid {
            boundaries: grid.boundaries.clone(),
            carts: result,
        }
    }
};

const IS_MOVEMENT_FORBIDDEN: fn(&Grid, &Cart, &Movement) -> bool =
    |grid, cart, movement| {
        let moved_cart = MOVE_CART(&cart, movement);

        moved_cart.coordinate.x > grid.boundaries.width
            || moved_cart.coordinate.x < 0
            || moved_cart.coordinate.y > grid.boundaries.height
            || moved_cart.coordinate.y < 0
    };