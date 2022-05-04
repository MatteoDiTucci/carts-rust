use crate::{Cart, Movement};

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

type AddCartToGrid = fn(Cart, &Grid) -> Grid;

pub const ADD_CART_TO_GRID: AddCartToGrid = {
    |cart, grid| {
        let mut result = grid.carts.clone();
        result.push(cart);

        Grid {
            boundaries: grid.boundaries.clone(),
            carts: result,
        }
    }
};

pub const IS_MOVEMENT_FORBIDDEN: fn(&Grid, &Cart, &Movement) -> bool =
    |grid, cart, movement| {
        let moved_cart = movement.apply(&cart);

        moved_cart.coordinate.x > grid.boundaries.width
            || moved_cart.coordinate.x < 0
            || moved_cart.coordinate.y > grid.boundaries.height
            || moved_cart.coordinate.y < 0
    };