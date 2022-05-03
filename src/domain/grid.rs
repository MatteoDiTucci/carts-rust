use crate::{Movement, Cart};

#[derive(Clone)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone)]
pub struct Grid {
    pub boundaries: Dimensions,
    pub carts: Vec<Cart>,
}

pub type Mission = (Cart, Vec<Movement>);
pub type MoveCarts = fn(Vec<Mission>, &Grid) -> Option<Grid>;

pub const MOVE_CARTS: MoveCarts = {
    |missions, grid| {
        let mut result = grid.clone();

        for mission in missions {
            if let Some(new_grid) = MOVE_CART(mission, &result) {
                result = new_grid;
            } else {
                return None
            }
        }
        Some(result)
    }
};

type MoveCart = fn(Mission, &Grid) -> Option<Grid>;
const MOVE_CART: MoveCart = {
    |(cart, movements), grid| {
        let mut moved_cart: Cart = cart;

        for movement in movements {
            moved_cart = movement.apply(&moved_cart);

            if IS_OUTSIDE_GRID(&grid, &moved_cart) {
                return None;
            }
        }

        Some(ADD_CART_TO_GRID(moved_cart, grid))
    }
};

const IS_OUTSIDE_GRID: fn(&Grid, &Cart) -> bool =
    |grid, cart| {
             cart.coordinate.x > grid.boundaries.width
            && cart.coordinate.y > grid.boundaries.height
    };

type AddCartToGrid = fn(Cart, &Grid) -> Grid;

const ADD_CART_TO_GRID: AddCartToGrid = {
    |cart, grid| {
        let mut result = grid.carts.clone();
        result.push(cart);

        Grid {
            boundaries: grid.boundaries.clone(),
            carts: result
        }
    }
};