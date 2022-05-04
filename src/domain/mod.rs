pub mod cart;
pub mod grid;
pub mod movement;
pub mod error;

#[cfg(test)]
mod functional_tests {
    use crate::{Dimensions, Cart, Coordinate, MOVE, Grid, LEFT, MOVE_CART, NORTH};
    use crate::domain::cart::Direction::{SOUTH};
    use crate::domain::error::InvalidMovementError;

    #[test]
    fn move_carts() {
        let mission =
                (Cart {
                    coordinate: Coordinate { x: 1, y: 2 },
                    direction: NORTH,
                },
                 vec![
                     LEFT, MOVE, LEFT, MOVE, LEFT, MOVE, LEFT, MOVE, MOVE,
                 ]);
        let grid = Grid { boundaries: Dimensions { width: 5, height: 5 }, carts: vec![] };

        let result = MOVE_CART(mission, &grid);


        assert_eq!(Cart { coordinate: Coordinate { x: 1, y: 3 }, direction: NORTH }, result.as_ref().unwrap().carts[0]);
    }

    #[test]
    fn move_carts_with_illegal_movement() {
        let cart = Cart {
            coordinate: Coordinate { x: 0, y: 0 },
            direction: SOUTH,
        };
        let mission =
            (cart.clone(),
             vec![
                 MOVE, MOVE,
             ]);
        let grid = Grid { boundaries: Dimensions { width: 5, height: 5 }, carts: vec![] };

        let result = MOVE_CART(mission, &grid);

        assert_eq!(InvalidMovementError { movement: MOVE, cart }, result.unwrap_err());
    }
}